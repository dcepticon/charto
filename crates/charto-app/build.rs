use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new_qml_module(QmlModule::new("org.charto.app").qml_file("qml/main.qml"))
        // Qt Core is always linked; Gui and Qml come from the qt_full feature of cxx-qt-lib.
        .qt_module("Network")
        .files(["src/backend.rs"])
        .build();
}
