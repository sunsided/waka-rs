//! Loading the `.env` file, which may come from an untrusted directory.
//!
//! The file is picked up from the working directory and its parents, so
//! running `waka-cli` inside an untrusted checkout must not let it decide
//! where credentials go. Loading every variable, as `dotenvy::dotenv` does,
//! would let it set e.g. `HTTPS_PROXY` and `SSL_CERT_FILE` to intercept all
//! requests, or `XDG_CONFIG_HOME` to move the stored credentials. Only an
//! allowlist of credentials and harmless settings is therefore loaded.

/// The variables a `.env` file may set.
const ALLOWED_VARS: &[&str] = &[
    "WAKATIME_API_KEY",
    "WAKATIME_BEARER_TOKEN",
    "WAKATIME_CLIENT_ID",
    "WAKATIME_CLIENT_SECRET",
    "WAKATIME_USER",
    "WAKATIME_TIMEOUT",
];

/// Settings that decide where requests and credentials go. A `.env` file
/// setting one is an error rather than silently ignored, since the user
/// presumably meant it.
const DESTINATION_VARS: &[(&str, &str)] = &[
    ("WAKATIME_BASE_URL", "--base-url"),
    ("WAKATIME_CREDENTIALS_FILE", "--credentials-file"),
    ("WAKATIME_REDIRECT_URI", "--redirect-uri"),
];

/// Loads the allowed variables from the `.env` file, if any, without
/// overriding the environment.
///
/// # Safety
///
/// Modifies the process environment, so it must be called while the process
/// is still single-threaded, i.e. before the async runtime is started.
pub unsafe fn load() -> Result<(), String> {
    // A missing or unreadable file is not an error, matching `dotenvy::dotenv`.
    let Ok(entries) = dotenvy::dotenv_iter() else {
        return Ok(());
    };
    let entries = select(entries.filter_map(Result::ok), |key| {
        std::env::var_os(key).is_some()
    })?;
    for (key, value) in entries {
        // SAFETY: the caller guarantees that no other threads exist yet.
        unsafe { std::env::set_var(key, value) };
    }
    Ok(())
}

/// Picks the entries to set from a `.env` file: allowed variables that are
/// not already set. Fails on [`DESTINATION_VARS`]; ignores everything else.
fn select(
    entries: impl IntoIterator<Item = (String, String)>,
    is_set: impl Fn(&str) -> bool,
) -> Result<Vec<(String, String)>, String> {
    let mut selected = Vec::new();
    for (key, value) in entries {
        if let Some((_, flag)) = DESTINATION_VARS.iter().find(|(var, _)| *var == key) {
            if is_set(&key) {
                // The environment wins over the file anyway.
                continue;
            }
            return Err(format!(
                "the .env file sets {key}, which decides where credentials are sent; a .env \
                 file may come from an untrusted directory, so set {key} in the shell \
                 environment or pass {flag} instead"
            ));
        }
        // Names are compared exactly, so e.g. a lowercase variant, which
        // Windows would treat as the same variable, is never set.
        if ALLOWED_VARS.contains(&key.as_str()) && !is_set(&key) {
            selected.push((key, value));
        }
    }
    Ok(selected)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::{assert, check};

    fn entries(keys: &[&str]) -> Vec<(String, String)> {
        keys.iter()
            .map(|key| (key.to_string(), format!("value of {key}")))
            .collect()
    }

    fn keys(selected: &[(String, String)]) -> Vec<&str> {
        selected.iter().map(|(key, _)| key.as_str()).collect()
    }

    #[test]
    fn loads_allowed_variables() {
        let selected = select(entries(ALLOWED_VARS), |_| false).unwrap();
        check!(keys(&selected) == ALLOWED_VARS);
        check!(selected[0].1 == "value of WAKATIME_API_KEY");
    }

    #[test]
    fn ignores_everything_else() {
        let selected = select(
            entries(&[
                "HTTPS_PROXY",
                "https_proxy",
                "ALL_PROXY",
                "SSL_CERT_FILE",
                "SSL_CERT_DIR",
                "XDG_CONFIG_HOME",
                "HOME",
                "APPDATA",
                "DBUS_SESSION_BUS_ADDRESS",
                "wakatime_base_url",
                "Wakatime_Api_Key",
                "WAKATIME_OAUTH_BASE_URL",
                "DATABASE_URL",
                "WAKATIME_USER",
            ]),
            |_| false,
        )
        .unwrap();
        check!(keys(&selected) == ["WAKATIME_USER"]);
    }

    #[test]
    fn does_not_override_the_environment() {
        let selected = select(entries(&["WAKATIME_API_KEY", "WAKATIME_USER"]), |key| {
            key == "WAKATIME_API_KEY"
        })
        .unwrap();
        check!(keys(&selected) == ["WAKATIME_USER"]);
    }

    #[test]
    fn rejects_destinations() {
        for (var, flag) in DESTINATION_VARS {
            let result = select(entries(&["WAKATIME_API_KEY", var]), |_| false);
            assert!(let Err(message) = result);
            check!(message.contains(var));
            check!(message.contains(flag));
        }
    }

    #[test]
    fn destinations_already_in_the_environment_are_fine() {
        let selected = select(entries(&["WAKATIME_BASE_URL"]), |key| {
            key == "WAKATIME_BASE_URL"
        })
        .unwrap();
        check!(selected.is_empty());
    }
}
