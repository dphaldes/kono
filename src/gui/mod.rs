mod bridge;
use cxx::UniquePtr;
use cxx_kde_frameworks::ki18n::{KLocalizedContext, KLocalizedString};
use cxx_qt_lib::{QByteArray, QGuiApplication, QQmlApplicationEngine, QUrl};

pub struct KonoGui {
    app: UniquePtr<QGuiApplication>,
    engine: UniquePtr<QQmlApplicationEngine>,
}

impl KonoGui {
    pub fn initialize() -> Self {
        let app = QGuiApplication::new();
        let mut engine = QQmlApplicationEngine::new();

        KLocalizedString::set_application_domain(&QByteArray::from("kono"));
        if let Some(mut engine) = engine.as_mut() {
            KLocalizedContext::initialize_engine(engine.as_mut().as_qqmlengine());
        }

        return KonoGui { app, engine };
    }

    pub fn open_dialog(&mut self) {
        if let Some(engine) = self.engine.as_mut() {
            engine.load(&QUrl::from("qrc:/qt/qml/kono/src/gui/qml/dialog.qml"));
        }
        self.exec();
    }
    
    pub fn open_gui(&mut self) {
        if let Some(engine) = self.engine.as_mut() {
            engine.load(&QUrl::from("qrc:/qt/qml/kono/src/gui/qml/main.qml"));
        }
        self.exec();
    }

    pub fn exec(&mut self) {
        self.app.as_mut().map(|app| app.exec());
    }
}
