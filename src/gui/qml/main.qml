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
    Component.onCompleted: {
        drawerColumn.children[0].click();
    }

    Component {
        id: aboutPage

        FormCard.AboutPage {
            aboutData: AboutData
        }

    }

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
                text: "Entry 1"
                icon.name: "actor"
                onPressed: {
                    console.log("clicked 1");
                }
            }

            SidebarEntry {
                text: "Entry 2"
                icon.name: "fingerprint"
                onPressed: {
                    console.log("clicked 2");
                }
            }

            SidebarEntry {
                text: "TEST"
                icon.name: "animal"
                onPressed: {
                    pageStack.clear();
                    pageStack.layers.push(aboutPage);
                }
            }

            Item {
                Layout.fillHeight: true
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

                        leftPadding: delegate.mirrored ? (delegate.indicator ? delegate.indicator.width : 0) + delegate.spacing : 0
                        rightPadding: !delegate.mirrored ? (delegate.indicator ? delegate.indicator.width : 0) + delegate.spacing : 0
                        text: delegate.text
                        font: delegate.font
                        color: delegate.enabled ? Kirigami.Theme.textColor : Kirigami.Theme.disabledTextColor
                        elide: Text.ElideRight
                        horizontalAlignment: Text.AlignHCenter
                        verticalAlignment: Text.AlignVCenter
                        Layout.alignment: Qt.AlignLeft
                        Layout.fillWidth: true
                        Accessible.ignored: true
                    }

                }

            }

        }

    }

}
