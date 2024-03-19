use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new()
        .qml_module(QmlModule::<&str, &str> {
            uri: "konossieur",
            qml_files: &["src/qml/main.qml"],
            ..Default::default()
        })
        .build();
}