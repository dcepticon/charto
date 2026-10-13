//! Helpers for the desktop file dialogs: converting the `file://` URIs the
//! freedesktop FileChooser portal returns (RFC 8089, percent-encoded) into
//! paths, suggesting file names and describing a dialog result as JSON for
//! the user interface.

use std::path::{Path, PathBuf};

/// Decodes `%XX` escapes. `None` for a malformed escape.
pub fn percent_decode(s: &str) -> Option<Vec<u8>> {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' {
            let hex = s.get(i + 1..i + 3)?;
            out.push(u8::from_str_radix(hex, 16).ok()?);
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    Some(out)
}

/// `file:///home/me/a%20b.spf` -> `/home/me/a b.spf`. Accepts an empty host
/// or `localhost`; anything else (other hosts, other schemes) gives `None`.
pub fn file_uri_to_path(uri: &str) -> Option<PathBuf> {
    let rest = uri.strip_prefix("file://")?;
    let path_part = if rest.starts_with('/') {
        rest
    } else {
        let slash = rest.find('/')?;
        if !rest[..slash].eq_ignore_ascii_case("localhost") {
            return None;
        }
        &rest[slash..]
    };
    let path_part = path_part.split(['?', '#']).next()?;
    let text = String::from_utf8(percent_decode(path_part)?).ok()?;
    if text.is_empty() {
        None
    } else {
        Some(PathBuf::from(text))
    }
}

/// A command-line argument as a path: plain paths stay as they are, `file://`
/// URIs (what file managers may pass for `%u`) are converted.
pub fn path_from_arg(arg: &str) -> PathBuf {
    if arg.starts_with("file://") {
        if let Some(p) = file_uri_to_path(arg) {
            return p;
        }
    }
    PathBuf::from(arg)
}

/// The folder of `path` if it exists (where a dialog should start).
pub fn existing_folder_of(path: &str) -> Option<PathBuf> {
    Path::new(path)
        .parent()
        .filter(|p| !p.as_os_str().is_empty() && p.is_dir())
        .map(Path::to_path_buf)
}

/// A file name for the save dialog: the stem of `current` (or "pattern")
/// with the given extension.
pub fn suggested_name(current: &str, extension: &str) -> String {
    let stem = Path::new(current)
        .file_stem()
        .and_then(|s| s.to_str())
        .filter(|s| !s.is_empty())
        .unwrap_or("pattern");
    format!("{stem}.{extension}")
}

/// What happened in a file dialog.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DialogOutcome {
    Chosen(PathBuf),
    Cancelled,
    /// No portal (or it failed): the caller should fall back to its own dialog.
    Unavailable(String),
}

/// The result as JSON for the QML side. `seq` makes every result different
/// so that a property change is always signalled.
pub fn outcome_json(seq: u32, purpose: &str, outcome: &DialogOutcome) -> String {
    let (state, path, message) = match outcome {
        DialogOutcome::Chosen(p) => ("chosen", p.to_string_lossy().into_owned(), String::new()),
        DialogOutcome::Cancelled => ("cancelled", String::new(), String::new()),
        DialogOutcome::Unavailable(m) => ("unavailable", String::new(), m.clone()),
    };
    serde_json::json!({
        "seq": seq,
        "purpose": purpose,
        "state": state,
        "path": path,
        "message": message,
    })
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_file_uris() {
        assert_eq!(file_uri_to_path("file:///home/u/a%20b.spf"), Some(PathBuf::from("/home/u/a b.spf")));
        assert_eq!(file_uri_to_path("file:///t/%C3%A4%23.spf"), Some(PathBuf::from("/t/ä#.spf")));
        assert_eq!(file_uri_to_path("file://localhost/tmp/x.spf"), Some(PathBuf::from("/tmp/x.spf")));
        assert_eq!(file_uri_to_path("file:///tmp/x.spf?download=1#top"), Some(PathBuf::from("/tmp/x.spf")));
    }

    #[test]
    fn rejects_what_is_not_a_local_file() {
        assert_eq!(file_uri_to_path("http://example.org/x.spf"), None);
        assert_eq!(file_uri_to_path("file://otherhost/x.spf"), None);
        assert_eq!(file_uri_to_path("/plain/path"), None);
        assert_eq!(file_uri_to_path("file://"), None);
        assert_eq!(file_uri_to_path("file:///bad%zz"), None);
        assert_eq!(file_uri_to_path("file:///bad%2"), None);
        assert_eq!(file_uri_to_path("file:///not-utf8-%ff"), None);
    }

    #[test]
    fn command_line_arguments() {
        assert_eq!(path_from_arg("file:///tmp/a%20b.spf"), PathBuf::from("/tmp/a b.spf"));
        assert_eq!(path_from_arg("/tmp/a.spf"), PathBuf::from("/tmp/a.spf"));
        assert_eq!(path_from_arg("relative/a.spf"), PathBuf::from("relative/a.spf"));
        assert_eq!(path_from_arg("file://otherhost/a.spf"), PathBuf::from("file://otherhost/a.spf"));
    }

    #[test]
    fn suggested_names() {
        assert_eq!(suggested_name("", "spf"), "pattern.spf");
        assert_eq!(suggested_name("/x/My Pattern.spf", "png"), "My Pattern.png");
        assert_eq!(suggested_name("/x/archive.tar.spf", "svg"), "archive.tar.svg");
        assert_eq!(suggested_name("/x/", "spf"), "x.spf");
    }

    #[test]
    fn start_folder_must_exist() {
        let dir = std::env::temp_dir();
        let file = dir.join("charto-dialog-test.spf");
        assert_eq!(existing_folder_of(&file.to_string_lossy()), Some(dir));
        assert_eq!(existing_folder_of(""), None);
        assert_eq!(existing_folder_of("rel.spf"), None);
        assert_eq!(existing_folder_of("/definitely/missing/dir/x.spf"), None);
    }

    #[test]
    fn outcomes_become_json() {
        let v: serde_json::Value =
            serde_json::from_str(&outcome_json(7, "open", &DialogOutcome::Chosen(PathBuf::from("/a/b.spf")))).unwrap();
        assert_eq!(v["seq"], 7);
        assert_eq!(v["purpose"], "open");
        assert_eq!(v["state"], "chosen");
        assert_eq!(v["path"], "/a/b.spf");

        let v: serde_json::Value = serde_json::from_str(&outcome_json(8, "save", &DialogOutcome::Cancelled)).unwrap();
        assert_eq!((v["state"].as_str(), v["path"].as_str()), (Some("cancelled"), Some("")));

        let v: serde_json::Value =
            serde_json::from_str(&outcome_json(9, "exportSvg", &DialogOutcome::Unavailable("no portal".into()))).unwrap();
        assert_eq!(v["state"], "unavailable");
        assert_eq!(v["message"], "no portal");
    }
}
