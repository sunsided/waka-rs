//! Loading the `.env` file, which may come from an untrusted directory.

/// Settings that decide where requests and credentials go. A `.env` file is
/// picked up from the working directory and its parents, so running
/// `waka-cli` inside an untrusted checkout must not let it change these and
/// receive the API key, bearer token, stored OAuth token or client secret.
const DESTINATION_VARS: &[(&str, &str)] = &[
    ("WAKATIME_BASE_URL", "--base-url"),
    ("WAKATIME_CREDENTIALS_FILE", "--credentials-file"),
    ("WAKATIME_REDIRECT_URI", "--redirect-uri"),
];

/// Loads the `.env` file, if any, without overriding the environment.
///
/// Fails if the file would set one of the [`DESTINATION_VARS`]; credentials
/// and other settings are fine.
pub fn load() -> Result<(), String> {
    // A missing or unreadable file is not an error, matching `dotenvy::dotenv`.
    let Ok(entries) = dotenvy::dotenv_iter() else {
        return Ok(());
    };
    // Only variables not already set are taken from the file.
    let keys: Vec<String> = entries
        .filter_map(Result::ok)
        .map(|(key, _)| key)
        .filter(|key| std::env::var_os(key).is_none())
        .collect();
    check(&keys)?;
    dotenvy::dotenv().ok();
    Ok(())
}

/// Rejects destination settings among the keys a `.env` file would set.
fn check<S: AsRef<str>>(keys: &[S]) -> Result<(), String> {
    for key in keys {
        let key = key.as_ref();
        if let Some((_, flag)) = DESTINATION_VARS.iter().find(|(var, _)| *var == key) {
            return Err(format!(
                "the .env file sets {key}, which decides where credentials are sent; a .env \
                 file may come from an untrusted directory, so set {key} in the shell \
                 environment or pass {flag} instead"
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::{assert, check};

    #[test]
    fn credentials_and_settings_are_allowed() {
        check!(
            check(&[
                "WAKATIME_API_KEY",
                "WAKATIME_BEARER_TOKEN",
                "WAKATIME_CLIENT_ID",
                "WAKATIME_CLIENT_SECRET",
                "WAKATIME_USER",
                "WAKATIME_TIMEOUT",
                "UNRELATED",
            ])
            .is_ok()
        );
        check!(check::<&str>(&[]).is_ok());
    }

    #[test]
    fn destinations_are_rejected() {
        for (var, flag) in DESTINATION_VARS {
            let result = check(&["WAKATIME_API_KEY", var]);
            assert!(let Err(message) = result);
            check!(message.contains(var));
            check!(message.contains(flag));
        }
    }
}
