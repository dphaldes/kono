import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import org.kde.kirigamiaddons.delegates as Delegates

Kirigami.ApplicationWindow {
    // pageStack.initialPage: Kirigami.Page {
    //     Controls.Label {
    //         anchors.centerIn: parent
    //         text: i18n("Hello World!")
    //     }
    // }

    id: app

    title: i18nc("@title:window", "Konossieur")

    globalDrawer: Kirigami.OverlayDrawer {
        id: globalDrawer

        width: Kirigami.Units.gridUnit * 16
        handleVisible: false
        modal: false

        contentItem: ColumnLayout {
            id: drawerColumn
            spacing: 0

            Delegates.RoundedItemDelegate {
                action: Kirigami.Action {
                    checkable: true
                    text: "Entry 1"
                    onTriggered: {
                        console.log("clicked 1")
                    }
                }
                padding: Kirigami.Units.largeSpacing
                Layout.fillWidth: true
            }

            Delegates.RoundedItemDelegate {
                action: Kirigami.Action {
                    checkable: true
                    text: "Entry 1"
                    onTriggered: {
                        console.log("clicked 2")
                    }
                }
                padding: Kirigami.Units.largeSpacing
                Layout.fillWidth: true
            }

            Item {
                Layout.fillHeight: true
            }

        }

        Controls.ButtonGroup {
            buttons: drawerColumn.children
        }

    }

}