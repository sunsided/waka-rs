//! Output handling: human-readable text by default, JSON with `--json`.

use serde::Serialize;
use std::error::Error;

/// Emits a response either as pretty-printed JSON to stdout (when `json` is
/// set) or through the given human-readable renderer.
pub fn emit<T: Serialize>(
    json: bool,
    value: &T,
    human: impl FnOnce(&T),
) -> Result<(), Box<dyn Error>> {
    if json {
        println!("{}", serde_json::to_string_pretty(value)?);
    } else {
        human(value);
    }
    Ok(())
}
