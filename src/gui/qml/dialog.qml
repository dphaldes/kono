import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as Controls
import org.kde.kirigami as Kirigami

Kirigami.ApplicationWindow {
    id: root
    width: 400
    height: 100

    minimumHeight: height
    maximumHeight: height

    minimumWidth: width
    maximumWidth: width

    ColumnLayout {
        anchors.fill: parent

        Controls.Label {
            text: "Exe is not configured. Would you like to configure it ?"
            padding: 10
            Layout.fillHeight: true
        }

        RowLayout {

            Controls.Button {
                Layout.fillWidth: true
                text: "Configure"
            }

            Controls.Button {
                Layout.fillWidth: true
                text: "Cancel"
                onClicked: Qt.quit()
            }
        }
    }
}
