import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.coreaddons
import org.kde.kirigami as Kirigami
import org.kde.kirigamiaddons.delegates as Delegates
import org.kde.kirigamiaddons.formcard as FormCard

Kirigami.ApplicationWindow {
    id: appWindow

    title: i18nc("@title:window", "Kono")
    minimumWidth: Kirigami.Units.gridUnit * 22
    minimumHeight: Kirigami.Units.gridUnit * 20

    pageStack.initialPage: Kirigami.Page {
        Controls.Label {
            anchors.centerIn: parent
            text: i18n("Hello World!")
        }
    }

    globalDrawer: Kirigami.OverlayDrawer {
        id: globalDrawer

        modal: false
        width: Kirigami.Units.gridUnit * 10
        handleVisible: false

        Controls.ButtonGroup {
            buttons: drawerColumn.children
        }

        contentItem: ColumnLayout {
            id: drawerColumn
            spacing: 0

            SidebarEntry {
                text: "Programs"
                icon.name: "applications-games-symbolic"
                checked: true
            }

            SidebarEntry {
                text: "Prefixes"
                icon.name: "system-run-symbolic"
            }

            SidebarEntry {
                text: "Components"
                icon.name: "folder-database-symbolic"
            }

            Item {
                Layout.fillHeight: true
            }
        }
    }

    component SidebarEntry: Delegates.RoundedItemDelegate {
        id: delegate

        padding: Kirigami.Units.largeSpacing
        Layout.fillWidth: true
        checkable: true
        icon.height: 32
        icon.width: 32

        contentItem: ColumnLayout {
            id: root
            spacing: Kirigami.Units.smallSpacing

            Kirigami.Icon {
                id: iconItem

                Layout.alignment: Qt.AlignHCenter
                source: delegate.icon.name
                Layout.preferredHeight: delegate.icon.width
                Layout.preferredWidth: delegate.icon.height
            }

            Controls.Label {
                id: labelItem

                text: delegate.text
                font: delegate.font
                elide: Text.ElideRight
                horizontalAlignment: Text.AlignHCenter
                verticalAlignment: Text.AlignVCenter
                Layout.fillWidth: true
            }
        }
    }
}
