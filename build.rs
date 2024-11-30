use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new()
        .qml_module(QmlModule {
            uri: "kono",
            qml_files: &["src/gui/qml/main.qml"],
            rust_files: &["src/gui/bridge/main.rs"],
            ..Default::default()
        })
        .build();
}
