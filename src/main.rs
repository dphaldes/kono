mod arguments;
mod gui;
mod paths;
mod runner;

use arguments::{Arguments, Command};
use clap::Parser;
use cxx_kde_frameworks::ki18n::{KLocalizedContext, KLocalizedString};
use cxx_qt_lib::{QByteArray, QGuiApplication, QQmlApplicationEngine, QUrl};

fn main() {
    // initialize
    let mut qapplication = QGuiApplication::new();
    let mut qml_engine = QQmlApplicationEngine::new();
    KLocalizedString::set_application_domain(&QByteArray::from("kono"));

    // load config and defaults if present

    // checks paths and create folders if necessary
    if let Err(err) = paths::ensure_kono_paths() {
        println!("Error creating folders!\n{}", err);
        std::process::exit(1);
    };

    // cli
    let args = Arguments::parse();

    if let Some(command) = args.command {
        match command {
            Command::Run { prog: app } => runner::run(app),
        }

        return;
    }

    if let Some(mut engine) = qml_engine.as_mut() {
        KLocalizedContext::initialize_engine(engine.as_mut().as_qqmlengine());
        engine.load(&QUrl::from("qrc:/qt/qml/kono/src/gui/qml/main.qml"));
    }

    qapplication.as_mut().map(|app| app.exec());
}
