//! Open, save and export dialogs through the freedesktop FileChooser portal
//! (`org.freedesktop.portal.FileChooser`), i.e. the file dialog of the user's
//! own desktop (GTK, KDE, ...) with its places, bookmarks, previews and
//! remembered folders, the same in every application.
//!
//! The portal call blocks until the user answers, so it runs on a worker
//! thread (see `backend.rs`). If there is no portal, the QML side falls back
//! to Qt's own dialog.

use std::path::Path;
use std::sync::atomic::{AtomicU32, Ordering};

use ashpd::desktop::file_chooser::{FileFilter, SelectedFiles};
use crate::translations::translate as tr;
use charto_core::fileuri::{
    existing_folder_of, file_uri_to_path, outcome_json, suggested_name, DialogOutcome,
};

static SEQUENCE: AtomicU32 = AtomicU32::new(1);

const PATTERN_MIME: &str = "application/x-charto-pattern";

fn pattern_filter() -> FileFilter {
    FileFilter::new(&tr("Charto patterns (*.spf)"))
        .mimetype(PATTERN_MIME)
        .glob("*.spf")
        .glob("*.SPF")
}

/// The portal could not be reached or refused the request.
fn unavailable(error: ashpd::Error) -> DialogOutcome {
    DialogOutcome::Unavailable(error.to_string())
}

/// The portal was reached but did not return files: the user closed the
/// dialog, or something went wrong inside it.
fn not_answered(error: ashpd::Error) -> DialogOutcome {
    let text = error.to_string();
    if text.to_lowercase().contains("cancel") {
        DialogOutcome::Cancelled
    } else {
        DialogOutcome::Unavailable(text)
    }
}

async fn open_dialog(folder: Option<&Path>) -> Result<SelectedFiles, DialogOutcome> {
    let title = tr("Open pattern");
    let accept_label = tr("Open");
    let mut request = SelectedFiles::open_file()
        .title(title.as_str())
        .accept_label(accept_label.as_str())
        .modal(true)
        .filter(pattern_filter())
        .filter(FileFilter::new(&tr("All files (*)")).glob("*"));
    if let Some(dir) = folder {
        request = request.current_folder(dir).map_err(unavailable)?;
    }
    let pending = request.send().await.map_err(unavailable)?;
    pending.response().map_err(not_answered)
}

async fn save_dialog(
    title: &str,
    accept_label: &str,
    name: &str,
    filter: FileFilter,
    folder: Option<&Path>,
) -> Result<SelectedFiles, DialogOutcome> {
    let mut request = SelectedFiles::save_file()
        .title(title)
        .accept_label(accept_label)
        .modal(true)
        .current_name(name)
        .filter(filter);
    if let Some(dir) = folder {
        request = request.current_folder(dir).map_err(unavailable)?;
    }
    let pending = request.send().await.map_err(unavailable)?;
    pending.response().map_err(not_answered)
}

/// Shows the dialog for `purpose` ("open", "save", "exportPng", "exportSvg").
/// `current` is the path of the open pattern (or empty): the dialog starts in
/// its folder and proposes its name.
async fn choose(purpose: &str, current: &str) -> DialogOutcome {
    let folder = existing_folder_of(current);
    let folder = folder.as_deref();

    let answer = match purpose {
        "open" => open_dialog(folder).await,
        "save" => {
            save_dialog(&tr("Save pattern"), &tr("Save"), &suggested_name(current, "spf"), pattern_filter(), folder).await
        }
        "exportPng" => {
            let filter = FileFilter::new(&tr("PNG images (*.png)")).mimetype("image/png").glob("*.png");
            save_dialog(&tr("Export as PNG image"), &tr("Export"), &suggested_name(current, "png"), filter, folder).await
        }
        "exportSvg" => {
            let filter = FileFilter::new(&tr("SVG drawings (*.svg)")).mimetype("image/svg+xml").glob("*.svg");
            save_dialog(&tr("Export as SVG drawing"), &tr("Export"), &suggested_name(current, "svg"), filter, folder).await
        }
        other => return DialogOutcome::Unavailable(format!("unknown dialog \"{other}\"")),
    };

    match answer {
        Err(outcome) => outcome,
        Ok(files) => match files.uris().first() {
            None => DialogOutcome::Cancelled,
            Some(uri) => match file_uri_to_path(uri.as_str()) {
                Some(path) => DialogOutcome::Chosen(path),
                None => DialogOutcome::Unavailable(format!("{uri} is not a local file")),
            },
        },
    }
}

/// Blocks until the user has answered; returns the result as JSON for QML.
pub fn run(purpose: &str, current: &str) -> String {
    let outcome = pollster::block_on(choose(purpose, current));
    if let DialogOutcome::Unavailable(why) = &outcome {
        // visible when charto is started from a terminal; the QML side then uses Qt's own dialog
        eprintln!("charto: the desktop file dialog is not available ({purpose}): {why}");
    }
    outcome_json(SEQUENCE.fetch_add(1, Ordering::Relaxed), purpose, &outcome)
}
