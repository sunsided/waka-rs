//! Persistent storage of OAuth credentials: the OS keyring, or a file.

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::{Path, PathBuf};
use waka::oauth::{OAuthClient, Token};

/// The keyring service name the credentials are stored under.
const KEYRING_SERVICE: &str = "waka-cli";
/// The keyring entry the credentials are stored under.
const KEYRING_USER: &str = "default";

/// Everything needed to use, refresh and revoke an OAuth token.
#[derive(Clone, Serialize, Deserialize)]
pub struct StoredCredentials {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub token: Token,
}

impl StoredCredentials {
    /// An OAuth client for the app the token was issued to.
    pub fn oauth_client(&self) -> Result<OAuthClient, Box<dyn Error>> {
        Ok(OAuthClient::new(
            &self.client_id,
            &self.client_secret,
            &self.redirect_uri,
        )?)
    }
}

/// Where credentials are stored.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Store {
    /// The OS keyring, falling back to a file if the keyring is unavailable.
    Keyring { fallback: Option<PathBuf> },
    /// A JSON file only readable by the current user.
    File(PathBuf),
}

impl std::fmt::Display for Store {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Store::Keyring { .. } => {
                write!(f, "the OS keyring (service \"{KEYRING_SERVICE}\")")
            }
            Store::File(path) => write!(f, "{}", path.display()),
        }
    }
}

impl Store {
    /// The store to use: the given file, or the keyring with a file in the
    /// user's config directory as fallback.
    pub fn new(credentials_file: Option<&Path>) -> Self {
        match credentials_file {
            Some(path) => Store::File(path.to_path_buf()),
            None => Store::Keyring {
                fallback: dirs::config_dir()
                    .map(|dir| dir.join("waka-cli").join("credentials.json")),
            },
        }
    }

    /// Loads the stored credentials, if any, along with the store they came from.
    pub async fn load(&self) -> Result<Option<(StoredCredentials, Store)>, Box<dyn Error>> {
        match self {
            Store::File(path) => Ok(read_file(path)?.map(|c| (c, self.clone()))),
            Store::Keyring { fallback } => {
                let keyring_error = match blocking(keyring_get).await {
                    Ok(Some(credentials)) => return Ok(Some((credentials, self.clone()))),
                    Ok(None) => None,
                    Err(e) => Some(e),
                };
                if let Some(path) = fallback
                    && let Some(credentials) = read_file(path)?
                {
                    return Ok(Some((credentials, Store::File(path.clone()))));
                }
                // Only worth mentioning if the credentials might be in there.
                if let Some(e) = keyring_error {
                    eprintln!("warning: cannot read the OS keyring: {e}");
                }
                Ok(None)
            }
        }
    }

    /// Saves the credentials, returning the store they ended up in.
    pub async fn save(&self, credentials: &StoredCredentials) -> Result<Store, Box<dyn Error>> {
        let json = serde_json::to_string(credentials)?;
        match self {
            Store::File(path) => {
                write_file(path, &json)?;
                Ok(self.clone())
            }
            Store::Keyring { fallback } => {
                let keyring_json = json.clone();
                match blocking(move || keyring_set(&keyring_json)).await {
                    Ok(()) => {
                        // Don't leave a stale copy behind from an earlier fallback.
                        if let Some(path) = fallback {
                            remove_file(path)?;
                        }
                        Ok(self.clone())
                    }
                    Err(e) => {
                        let Some(path) = fallback else {
                            return Err(e);
                        };
                        eprintln!(
                            "warning: cannot write to the OS keyring ({e}); \
                             storing the credentials in {} instead",
                            path.display()
                        );
                        write_file(path, &json)?;
                        Ok(Store::File(path.clone()))
                    }
                }
            }
        }
    }

    /// Deletes the stored credentials from every location this store uses.
    pub async fn delete(&self) -> Result<(), Box<dyn Error>> {
        match self {
            Store::File(path) => remove_file(path),
            Store::Keyring { fallback } => {
                if let Err(e) = blocking(keyring_delete).await {
                    eprintln!("warning: cannot delete from the OS keyring: {e}");
                }
                match fallback {
                    Some(path) => remove_file(path),
                    None => Ok(()),
                }
            }
        }
    }
}

/// Runs a keyring operation off the async runtime, since the platform
/// stores block, e.g. on D-Bus calls to the Secret Service.
async fn blocking<T, F>(f: F) -> Result<T, Box<dyn Error>>
where
    T: Send + 'static,
    F: FnOnce() -> Result<T, Box<dyn Error + Send + Sync>> + Send + 'static,
{
    tokio::task::spawn_blocking(f)
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e as Box<dyn Error>)
}

fn keyring_entry() -> keyring::Result<keyring::Entry> {
    keyring::Entry::new(KEYRING_SERVICE, KEYRING_USER)
}

fn keyring_get() -> Result<Option<StoredCredentials>, Box<dyn Error + Send + Sync>> {
    match keyring_entry()?.get_password() {
        Ok(json) => Ok(Some(serde_json::from_str(&json)?)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

fn keyring_set(json: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    Ok(keyring_entry()?.set_password(json)?)
}

fn keyring_delete() -> Result<(), Box<dyn Error + Send + Sync>> {
    match keyring_entry()?.delete_credential() {
        Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(e.into()),
    }
}

fn read_file(path: &Path) -> Result<Option<StoredCredentials>, Box<dyn Error>> {
    match std::fs::read_to_string(path) {
        Ok(json) => Ok(Some(serde_json::from_str(&json).map_err(|e| {
            format!("invalid credentials file {}: {e}", path.display())
        })?)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(format!("cannot read {}: {e}", path.display()).into()),
    }
}

/// Writes the file so that only the current user can read it.
fn write_file(path: &Path, contents: &str) -> Result<(), Box<dyn Error>> {
    use std::io::Write;

    if let Some(dir) = path.parent().filter(|dir| !dir.as_os_str().is_empty()) {
        let mut builder = std::fs::DirBuilder::new();
        builder.recursive(true);
        #[cfg(unix)]
        std::os::unix::fs::DirBuilderExt::mode(&mut builder, 0o700);
        builder.create(dir)?;
    }

    let mut options = std::fs::OpenOptions::new();
    options.write(true).create(true).truncate(true);
    #[cfg(unix)]
    std::os::unix::fs::OpenOptionsExt::mode(&mut options, 0o600);
    let mut file = options
        .open(path)
        .map_err(|e| format!("cannot write {}: {e}", path.display()))?;
    // The mode only applies to new files; tighten existing ones too.
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        file.set_permissions(std::fs::Permissions::from_mode(0o600))?;
    }
    file.write_all(contents.as_bytes())?;
    Ok(())
}

fn remove_file(path: &Path) -> Result<(), Box<dyn Error>> {
    match std::fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(format!("cannot delete {}: {e}", path.display()).into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::{assert, check};

    fn credentials() -> StoredCredentials {
        StoredCredentials {
            client_id: "my-client-id".into(),
            client_secret: "my-client-secret".into(),
            redirect_uri: "http://127.0.0.1:8737/callback".into(),
            token: serde_json::from_str(
                r#"{"access_token": "waka_tok_access", "refresh_token": "waka_ref_refresh", "scope": "read_stats"}"#,
            )
            .unwrap(),
        }
    }

    #[tokio::test]
    async fn file_store_round_trips_credentials() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nested").join("credentials.json");
        let store = Store::new(Some(&path));

        check!(store.load().await.unwrap().is_none());

        let saved_to = store.save(&credentials()).await.unwrap();
        check!(saved_to == Store::File(path.clone()));

        assert!(let Some((loaded, loaded_from)) = store.load().await.unwrap());
        check!(loaded_from == store);
        check!(loaded.client_id == "my-client-id");
        check!(loaded.client_secret == "my-client-secret");
        check!(loaded.redirect_uri == "http://127.0.0.1:8737/callback");
        check!(loaded.token.access_token == "waka_tok_access");
        check!(loaded.token.refresh_token.as_deref() == Some("waka_ref_refresh"));

        store.delete().await.unwrap();
        check!(!path.exists());
        check!(store.load().await.unwrap().is_none());
        // Deleting again is not an error.
        store.delete().await.unwrap();
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn file_store_is_private_to_the_user() {
        use std::os::unix::fs::PermissionsExt;

        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("credentials.json");
        // A pre-existing, world-readable file gets tightened.
        std::fs::write(&path, "{}").unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o644)).unwrap();

        Store::new(Some(&path)).save(&credentials()).await.unwrap();

        let mode = std::fs::metadata(&path).unwrap().permissions().mode();
        check!(mode & 0o777 == 0o600);
    }

    #[tokio::test]
    async fn invalid_credentials_file_is_an_error() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("credentials.json");
        std::fs::write(&path, "not json").unwrap();

        let result = Store::new(Some(&path)).load().await;

        assert!(let Err(e) = result);
        check!(e.to_string().starts_with("invalid credentials file"));
    }

    #[test]
    fn default_store_uses_the_keyring() {
        assert!(let Store::Keyring { fallback } = Store::new(None));
        if let Some(fallback) = fallback {
            check!(fallback.ends_with("waka-cli/credentials.json"));
        }
    }
}
