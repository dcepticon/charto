//! Per-user files, placed according to the freedesktop.org XDG Base
//! Directory Specification: settings under `$XDG_CONFIG_HOME/charto`
//! (default `~/.config/charto`), recovery files under
//! `$XDG_STATE_HOME/charto` (default `~/.local/state/charto`).

use std::ffi::OsString;
use std::path::{Path, PathBuf};

pub const MAX_RECENT_FILES: usize = 10;

/// `value` if it is an absolute path (the spec says relative ones are
/// ignored), otherwise `$HOME/<home_rel>`.
pub fn xdg_dir_from(value: Option<OsString>, home: Option<OsString>, home_rel: &str) -> Option<PathBuf> {
    if let Some(v) = value {
        let p = PathBuf::from(v);
        if p.is_absolute() {
            return Some(p);
        }
    }
    let home = PathBuf::from(home?);
    if home.is_absolute() {
        Some(home.join(home_rel))
    } else {
        None
    }
}

fn xdg_dir(var: &str, home_rel: &str) -> Option<PathBuf> {
    xdg_dir_from(std::env::var_os(var), std::env::var_os("HOME"), home_rel)
}

pub fn config_dir() -> Option<PathBuf> {
    xdg_dir("XDG_CONFIG_HOME", ".config").map(|d| d.join("charto"))
}

pub fn state_dir() -> Option<PathBuf> {
    xdg_dir("XDG_STATE_HOME", ".local/state").map(|d| d.join("charto"))
}

/// The most recently used files, newest first, at most [`MAX_RECENT_FILES`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RecentFiles {
    files: Vec<PathBuf>,
}

impl RecentFiles {
    /// One path per line. A missing or unreadable file gives an empty list.
    pub fn load(path: &Path) -> Self {
        let mut list = RecentFiles::default();
        if let Ok(text) = std::fs::read_to_string(path) {
            // the file is stored newest first, so add oldest first
            for line in text.lines().rev().filter(|l| !l.trim().is_empty()) {
                list.add(PathBuf::from(line));
            }
        }
        list
    }

    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let mut text = String::new();
        for f in &self.files {
            text.push_str(&f.to_string_lossy());
            text.push('\n');
        }
        std::fs::write(path, text)
    }

    pub fn add(&mut self, file: PathBuf) {
        self.files.retain(|f| f != &file);
        self.files.insert(0, file);
        self.files.truncate(MAX_RECENT_FILES);
    }

    pub fn clear(&mut self) {
        self.files.clear();
    }

    pub fn files(&self) -> &[PathBuf] {
        &self.files
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn os(s: &str) -> Option<OsString> {
        Some(OsString::from(s))
    }

    #[test]
    fn xdg_rules() {
        assert_eq!(xdg_dir_from(os("/x/cfg"), os("/home/u"), ".config"), Some(PathBuf::from("/x/cfg")));
        // relative values are ignored
        assert_eq!(xdg_dir_from(os("rel/cfg"), os("/home/u"), ".config"), Some(PathBuf::from("/home/u/.config")));
        assert_eq!(xdg_dir_from(os(""), os("/home/u"), ".local/state"), Some(PathBuf::from("/home/u/.local/state")));
        assert_eq!(xdg_dir_from(None, None, ".config"), None);
        assert_eq!(xdg_dir_from(None, os("relative-home"), ".config"), None);
    }

    #[test]
    fn recent_files_are_newest_first_unique_and_capped() {
        let mut r = RecentFiles::default();
        for i in 0..15 {
            r.add(PathBuf::from(format!("/p/{i}.spf")));
        }
        assert_eq!(r.files().len(), MAX_RECENT_FILES);
        assert_eq!(r.files()[0], PathBuf::from("/p/14.spf"));
        r.add(PathBuf::from("/p/10.spf")); // moves to the front, no duplicate
        assert_eq!(r.files()[0], PathBuf::from("/p/10.spf"));
        assert_eq!(r.files().iter().filter(|f| **f == PathBuf::from("/p/10.spf")).count(), 1);
        assert_eq!(r.files().len(), MAX_RECENT_FILES);
        r.clear();
        assert!(r.files().is_empty());
    }

    #[test]
    fn recent_files_roundtrip_on_disk() {
        let dir = std::env::temp_dir().join(format!("charto_cfg_test_{}", std::process::id()));
        let file = dir.join("nested/recent-files.txt");
        let mut r = RecentFiles::default();
        r.add(PathBuf::from("/a/one.spf"));
        r.add(PathBuf::from("/b/two words.spf"));
        r.save(&file).unwrap();
        assert_eq!(RecentFiles::load(&file), r);
        assert!(RecentFiles::load(&dir.join("missing.txt")).files().is_empty());
        let _ = std::fs::remove_dir_all(dir);
    }
}
