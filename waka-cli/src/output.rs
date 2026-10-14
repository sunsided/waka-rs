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

/// Replaces control characters, such as terminal escape sequences, with
/// U+FFFD so that text from remote sources cannot manipulate the terminal.
/// Line breaks are kept only when `keep_newlines` is set.
pub fn sanitize_lines(text: &str, keep_newlines: bool) -> String {
    text.chars()
        .map(|c| {
            if c.is_control() && !(keep_newlines && c == '\n') {
                '\u{fffd}'
            } else {
                c
            }
        })
        .collect()
}

/// Like [`sanitize_lines`], for single-line text.
pub fn sanitize(text: &str) -> String {
    sanitize_lines(text, false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;

    #[test]
    fn sanitize_replaces_control_characters() {
        check!(sanitize("plain text, ünïcode") == "plain text, ünïcode");
        check!(sanitize("a\u{1b}[31mred\u{7}") == "a\u{fffd}[31mred\u{fffd}");
        check!(sanitize("one\ntwo\r\t") == "one\u{fffd}two\u{fffd}\u{fffd}");
        check!(sanitize("\u{9b}31m") == "\u{fffd}31m");
    }

    #[test]
    fn sanitize_lines_keeps_only_newlines() {
        check!(sanitize_lines("one\ntwo\r\n\u{1b}", true) == "one\ntwo\u{fffd}\n\u{fffd}");
    }
}
