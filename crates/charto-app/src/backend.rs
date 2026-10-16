//! The QObject exposed to QML. All logic lives in `charto_core::session`;
//! this file only forwards user actions and publishes a JSON snapshot after
//! each of them.

#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, snapshot)]
        #[qproperty(QString, status)]
        #[qproperty(QString, picked)]
        type ChartBackend = super::ChartBackendRust;

        #[qinvokable]
        #[cxx_name = "paletteJson"]
        fn palette_json(&self) -> QString;

        #[qinvokable]
        #[cxx_name = "newChart"]
        fn new_chart(self: Pin<&mut Self>, rows: i32, cols: i32);

        #[qinvokable]
        #[cxx_name = "openFile"]
        fn open_file(self: Pin<&mut Self>, path: &QString);

        #[qinvokable]
        #[cxx_name = "saveFile"]
        fn save_file(self: Pin<&mut Self>, path: &QString);

        #[qinvokable]
        #[cxx_name = "setActiveSymbol"]
        fn set_active_symbol(self: Pin<&mut Self>, name: &QString);

        #[qinvokable]
        #[cxx_name = "setColor"]
        fn set_color(self: Pin<&mut Self>, hex: &QString);

        #[qinvokable]
        #[cxx_name = "addColor"]
        fn add_color(self: Pin<&mut Self>, hex: &QString);

        #[qinvokable]
        #[cxx_name = "setMode"]
        fn set_mode(self: Pin<&mut Self>, mode: i32);

        #[qinvokable]
        #[cxx_name = "clickCell"]
        fn click_cell(self: Pin<&mut Self>, row: i32, col: i32);

        #[qinvokable]
        #[cxx_name = "selectRect"]
        fn select_rect(self: Pin<&mut Self>, col: i32, row: i32, num_cols: i32, num_rows: i32);

        #[qinvokable]
        #[cxx_name = "selectRow"]
        fn select_row(self: Pin<&mut Self>, row: i32);

        #[qinvokable]
        #[cxx_name = "selectColumn"]
        fn select_column(self: Pin<&mut Self>, col: i32);

        #[qinvokable]
        #[cxx_name = "clearSelection"]
        fn clear_selection(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "applyColor"]
        fn apply_color(self: Pin<&mut Self>);

        #[qinvokable]
        fn undo(self: Pin<&mut Self>);

        #[qinvokable]
        fn redo(self: Pin<&mut Self>);

        #[qinvokable]
        fn copy(self: Pin<&mut Self>);

        /// `col`/`row` of the clicked cell, or -1/-1 to paste into the selection.
        #[qinvokable]
        fn paste(self: Pin<&mut Self>, col: i32, row: i32);

        #[qinvokable]
        #[cxx_name = "insertRows"]
        fn insert_rows(self: Pin<&mut Self>, num: i32, above: bool);

        #[qinvokable]
        #[cxx_name = "deleteRows"]
        fn delete_rows(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "insertColumns"]
        fn insert_columns(self: Pin<&mut Self>, num: i32, left: bool);

        #[qinvokable]
        #[cxx_name = "deleteColumns"]
        fn delete_columns(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "addRowRepeat"]
        fn add_row_repeat(self: Pin<&mut Self>, multiplicity: i32);

        #[qinvokable]
        #[cxx_name = "deleteRowRepeat"]
        fn delete_row_repeat(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "hasRecovery"]
        fn has_recovery(&self, path: &QString) -> bool;

        #[qinvokable]
        #[cxx_name = "openRecovery"]
        fn open_recovery(self: Pin<&mut Self>, path: &QString);

        /// Called by a timer: writes a recovery copy if there are unsaved changes.
        #[qinvokable]
        fn autosave(self: Pin<&mut Self>);

        /// Called when the window closes normally: removes the recovery copy.
        #[qinvokable]
        #[cxx_name = "endSession"]
        fn end_session(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "clearRecentFiles"]
        fn clear_recent_files(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "selectSameColor"]
        fn select_same_color(self: Pin<&mut Self>, row: i32, col: i32);

        #[qinvokable]
        #[cxx_name = "selectSameSymbol"]
        fn select_same_symbol(self: Pin<&mut Self>, row: i32, col: i32);

        #[qinvokable]
        #[cxx_name = "setRowLabel"]
        fn set_row_label(self: Pin<&mut Self>, row: i32, text: &QString);

        #[qinvokable]
        #[cxx_name = "setColumnLabel"]
        fn set_column_label(self: Pin<&mut Self>, col: i32, text: &QString);

        #[qinvokable]
        #[cxx_name = "settingsJson"]
        fn settings_json(&self) -> QString;

        #[qinvokable]
        #[cxx_name = "applySettings"]
        fn apply_settings(self: Pin<&mut Self>, json: &QString);

        #[qinvokable]
        #[cxx_name = "exportSvg"]
        fn export_svg(self: Pin<&mut Self>, path: &QString);

        /// Where QML should save the rendered chart before printing.
        #[qinvokable]
        #[cxx_name = "printTempPath"]
        fn print_temp_path(&self) -> QString;

        /// The text in the user's language (see `translations`), or the text itself.
        #[qinvokable]
        #[cxx_name = "translate"]
        fn translate(&self, text: &QString) -> QString;

        /// The file Charto was started with (`charto pattern.spf`, or a file manager's
        /// double-click), or an empty string.
        #[qinvokable]
        #[cxx_name = "startupFile"]
        fn startup_file(&self) -> QString;

        /// Prints a PNG rendering of the chart through the desktop print portal.
        #[qinvokable]
        #[cxx_name = "printImage"]
        fn print_image(self: Pin<&mut Self>, png_path: &QString, title: &QString);

        #[qinvokable]
        #[cxx_name = "chooseFile"]
        fn choose_file(self: Pin<&mut Self>, purpose: &QString, current: &QString);

        #[qinvokable]
        #[cxx_name = "reportStatus"]
        fn report_status(self: Pin<&mut Self>, message: &QString);
    }

    // lets background threads (printing) report back on the Qt thread
    impl cxx_qt::Threading for ChartBackend {}
}

use core::pin::Pin;
use std::path::{Path, PathBuf};

use cxx_qt::{CxxQtType, Threading};
use cxx_qt_lib::QString;
use charto_core::session::Session;
use charto_core::symbols::SymbolLibrary;

/// Directories searched for symbol libraries; later ones override earlier ones.
fn symbol_roots() -> Vec<PathBuf> {
    let mut roots = Vec::new();
    // development checkout
    roots.push(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/symbols"));
    // installed next to the executable
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            roots.push(dir.join("assets/symbols"));
            roots.push(dir.join("../share/charto/symbols"));
        }
    }
    // the user's own symbols: the original sconcho location first (so custom
    // symbols made with sconcho keep working), then Charto's own, then an
    // explicit override
    if let Some(home) = std::env::var_os("HOME") {
        let home = PathBuf::from(home);
        roots.push(home.join(".sconcho/symbols"));
        roots.push(home.join(".charto/symbols"));
    }
    if let Some(dir) = std::env::var_os("CHARTO_SYMBOLS") {
        roots.push(PathBuf::from(dir));
    }
    roots
}

pub struct ChartBackendRust {
    snapshot: QString,
    status: QString,
    /// JSON answer of the last desktop file dialog (see `filedialog`)
    picked: QString,
    session: Session,
}

impl Default for ChartBackendRust {
    fn default() -> Self {
        let (library, _skipped) = SymbolLibrary::load(&symbol_roots());
        let session = Session::with_dirs(
            library,
            charto_core::config::config_dir(),
            charto_core::config::state_dir(),
        );
        let snapshot = QString::from(session.snapshot_json().as_str());
        ChartBackendRust { snapshot, status: QString::default(), picked: QString::default(), session }
    }
}

impl qobject::ChartBackend {
    /// Publishes the new state and an optional message for the status bar.
    fn finish(mut self: Pin<&mut Self>, message: String) {
        let snapshot = QString::from(self.rust().session.snapshot_json().as_str());
        self.as_mut().set_snapshot(snapshot);
        self.as_mut().set_status(QString::from(message.as_str()));
    }

    fn finish_result(self: Pin<&mut Self>, result: Result<(), String>) {
        self.finish(result.err().unwrap_or_default());
    }

    pub fn palette_json(&self) -> QString {
        QString::from(self.rust().session.palette_json().as_str())
    }

    pub fn new_chart(mut self: Pin<&mut Self>, rows: i32, cols: i32) {
        let r = self.as_mut().rust_mut().session.new_chart(rows, cols);
        self.finish_result(r);
    }

    pub fn open_file(mut self: Pin<&mut Self>, path: &QString) {
        let path = path.to_string();
        let r = self.as_mut().rust_mut().session.open(Path::new(&path));
        let message = match r {
            Ok(Some(warning)) => warning,
            Ok(None) => String::new(),
            Err(e) => e,
        };
        self.finish(message);
    }

    pub fn save_file(mut self: Pin<&mut Self>, path: &QString) {
        let path = path.to_string();
        let r = self.as_mut().rust_mut().session.save(Path::new(&path));
        self.finish_result(r);
    }

    pub fn set_active_symbol(mut self: Pin<&mut Self>, name: &QString) {
        let name = name.to_string();
        let r = self.as_mut().rust_mut().session.set_active_symbol(&name);
        self.finish_result(r);
    }

    pub fn set_color(mut self: Pin<&mut Self>, hex: &QString) {
        let hex = hex.to_string();
        let r = self.as_mut().rust_mut().session.set_color(&hex);
        self.finish_result(r);
    }

    pub fn add_color(mut self: Pin<&mut Self>, hex: &QString) {
        let hex = hex.to_string();
        let r = self.as_mut().rust_mut().session.add_color(&hex);
        self.finish_result(r);
    }

    pub fn set_mode(mut self: Pin<&mut Self>, mode: i32) {
        self.as_mut().rust_mut().session.set_mode(mode);
        self.finish(String::new());
    }

    pub fn click_cell(mut self: Pin<&mut Self>, row: i32, col: i32) {
        self.as_mut().rust_mut().session.click_cell(row, col);
        self.finish(String::new());
    }

    pub fn select_rect(mut self: Pin<&mut Self>, col: i32, row: i32, num_cols: i32, num_rows: i32) {
        self.as_mut().rust_mut().session.select_rect(col, row, num_cols, num_rows);
        self.finish(String::new());
    }

    pub fn select_row(mut self: Pin<&mut Self>, row: i32) {
        self.as_mut().rust_mut().session.select_row(row);
        self.finish(String::new());
    }

    pub fn select_column(mut self: Pin<&mut Self>, col: i32) {
        self.as_mut().rust_mut().session.select_column(col);
        self.finish(String::new());
    }

    pub fn clear_selection(mut self: Pin<&mut Self>) {
        self.as_mut().rust_mut().session.clear_selection();
        self.finish(String::new());
    }

    pub fn apply_color(mut self: Pin<&mut Self>) {
        let r = self.as_mut().rust_mut().session.apply_color();
        self.finish_result(r);
    }

    pub fn undo(mut self: Pin<&mut Self>) {
        self.as_mut().rust_mut().session.undo();
        self.finish(String::new());
    }

    pub fn redo(mut self: Pin<&mut Self>) {
        self.as_mut().rust_mut().session.redo();
        self.finish(String::new());
    }

    pub fn copy(mut self: Pin<&mut Self>) {
        self.as_mut().rust_mut().session.copy();
        self.finish(String::new());
    }

    pub fn paste(mut self: Pin<&mut Self>, col: i32, row: i32) {
        let at = if col < 0 || row < 0 { None } else { Some((col, row)) };
        let r = self.as_mut().rust_mut().session.paste(at);
        self.finish_result(r);
    }

    pub fn insert_rows(mut self: Pin<&mut Self>, num: i32, above: bool) {
        let r = self.as_mut().rust_mut().session.insert_rows(num, above);
        self.finish_result(r);
    }

    pub fn delete_rows(mut self: Pin<&mut Self>) {
        let r = self.as_mut().rust_mut().session.delete_rows();
        self.finish_result(r);
    }

    pub fn insert_columns(mut self: Pin<&mut Self>, num: i32, left: bool) {
        let r = self.as_mut().rust_mut().session.insert_columns(num, left);
        self.finish_result(r);
    }

    pub fn delete_columns(mut self: Pin<&mut Self>) {
        let r = self.as_mut().rust_mut().session.delete_columns();
        self.finish_result(r);
    }

    pub fn add_row_repeat(mut self: Pin<&mut Self>, multiplicity: i32) {
        let r = self.as_mut().rust_mut().session.add_row_repeat(multiplicity);
        self.finish_result(r);
    }

    pub fn delete_row_repeat(mut self: Pin<&mut Self>) {
        let r = self.as_mut().rust_mut().session.delete_row_repeat();
        self.finish_result(r);
    }

    pub fn has_recovery(&self, path: &QString) -> bool {
        let path = path.to_string();
        self.rust().session.has_recovery(Path::new(&path))
    }

    pub fn open_recovery(mut self: Pin<&mut Self>, path: &QString) {
        let path = path.to_string();
        let r = self.as_mut().rust_mut().session.open_recovery(Path::new(&path));
        let message = match r {
            Ok(Some(warning)) => warning,
            Ok(None) => String::new(),
            Err(e) => e,
        };
        self.finish(message);
    }

    /// Only reports a problem; a successful autosave stays silent.
    pub fn autosave(mut self: Pin<&mut Self>) {
        let result = self.as_mut().rust_mut().session.autosave();
        if let Err(error) = result {
            self.finish(error);
        }
    }

    pub fn end_session(mut self: Pin<&mut Self>) {
        self.as_mut().rust_mut().session.clear_recovery();
    }

    pub fn clear_recent_files(mut self: Pin<&mut Self>) {
        self.as_mut().rust_mut().session.clear_recent_files();
        self.finish(String::new());
    }

    pub fn select_same_color(mut self: Pin<&mut Self>, row: i32, col: i32) {
        self.as_mut().rust_mut().session.select_same_color(row, col);
        self.finish(String::new());
    }

    pub fn select_same_symbol(mut self: Pin<&mut Self>, row: i32, col: i32) {
        self.as_mut().rust_mut().session.select_same_symbol(row, col);
        self.finish(String::new());
    }

    pub fn set_row_label(mut self: Pin<&mut Self>, row: i32, text: &QString) {
        let text = text.to_string();
        let r = self.as_mut().rust_mut().session.set_row_label(row, &text);
        self.finish_result(r);
    }

    pub fn set_column_label(mut self: Pin<&mut Self>, col: i32, text: &QString) {
        let text = text.to_string();
        let r = self.as_mut().rust_mut().session.set_column_label(col, &text);
        self.finish_result(r);
    }

    pub fn settings_json(&self) -> QString {
        QString::from(self.rust().session.settings_json().as_str())
    }

    pub fn apply_settings(mut self: Pin<&mut Self>, json: &QString) {
        let json = json.to_string();
        let r = self.as_mut().rust_mut().session.apply_settings_json(&json);
        self.finish_result(r);
    }

    pub fn export_svg(mut self: Pin<&mut Self>, path: &QString) {
        let path = path.to_string();
        let r = self.as_mut().rust_mut().session.export_svg(Path::new(&path));
        let message = match r {
            Ok(()) => format!("Exported {path}"),
            Err(e) => e,
        };
        self.finish(message);
    }

    pub fn translate(&self, text: &QString) -> QString {
        QString::from(crate::translations::translate(&text.to_string()).as_str())
    }

    pub fn startup_file(&self) -> QString {
        // the first argument that is not an option: a path, or a file:// URI from a file manager
        let path = std::env::args()
            .skip(1)
            .find(|arg| !arg.starts_with('-'))
            .map(|arg| charto_core::fileuri::path_from_arg(&arg));
        match path {
            Some(p) => QString::from(&*p.to_string_lossy()),
            None => QString::default(),
        }
    }

    pub fn print_temp_path(&self) -> QString {
        let path = std::env::temp_dir().join(format!("charto-chart-{}.png", std::process::id()));
        QString::from(&*path.to_string_lossy())
    }

    /// The print dialog blocks until the user answers, so printing runs on
    /// its own thread and reports back through the status bar.
    pub fn print_image(mut self: Pin<&mut Self>, png_path: &QString, title: &QString) {
        let path = PathBuf::from(png_path.to_string());
        let title = title.to_string();
        let qt_thread = self.qt_thread();
        self.as_mut().set_status(QString::from("Preparing to print…"));
        std::thread::spawn(move || {
            let message = match crate::print::print_png(&path, &title) {
                Ok(message) => message,
                Err(error) => error,
            };
            let _ = std::fs::remove_file(&path);
            qt_thread
                .queue(move |backend| {
                    backend.set_status(QString::from(message.as_str()));
                })
                .ok();
        });
    }

    /// Shows the desktop's own open/save/export dialog (freedesktop FileChooser
    /// portal). The portal call blocks until the user answers, so it runs on its
    /// own thread and publishes the answer as JSON in `picked`.
    pub fn choose_file(self: Pin<&mut Self>, purpose: &QString, current: &QString) {
        let purpose = purpose.to_string();
        let current = current.to_string();
        let qt_thread = self.qt_thread();
        std::thread::spawn(move || {
            let answer = crate::filedialog::run(&purpose, &current);
            qt_thread
                .queue(move |backend| {
                    backend.set_picked(QString::from(answer.as_str()));
                })
                .ok();
        });
    }

    pub fn report_status(self: Pin<&mut Self>, message: &QString) {
        let message = message.to_string();
        self.finish(message);
    }
}
