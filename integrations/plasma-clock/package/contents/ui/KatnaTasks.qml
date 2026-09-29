// SPDX-License-Identifier: GPL-3.0-or-later

pragma ComponentBehavior: Bound

import QtQuick
import QtQuick.Layouts

import org.kde.plasma.components as PlasmaComponents
import org.kde.plasma.extras as PlasmaExtras
import org.kde.kirigami as Kirigami

// The Tasks list under the day's events: add a task (due on the day picked
// in the month, when that isn't today) and tick one off. Tasks ticked off
// stay, struck through, until the popup closes, so a wrong tick can be
// taken back.
ColumnLayout {
    id: section

    required property KatnaAgenda agenda
    // The day picked in the month view.
    required property date selectedDate
    required property int paddings
    // Under the day's events the list stays short; alone it fills the column.
    property bool compact: true

    spacing: 0

    readonly property string domain: "katna-clock"
    readonly property date today: new Date()
    readonly property bool todayPicked: sameDay(selectedDate, today)
    // Ticked off while this popup was open.
    property var tickedNow: ({})

    readonly property var shown: agenda.tasks
        .filter(task => !task.done || tickedNow[task.id])
        .sort((a, b) => (a.due || "9999") < (b.due || "9999") ? -1 : (a.due || "9999") > (b.due || "9999") ? 1 : 0)

    Connections {
        target: section.agenda
        function onActiveChanged(): void {
            if (!section.agenda.active) {
                section.tickedNow = ({});
            }
        }
    }

    function tick(id: string, done: bool): void {
        const ticked = Object.assign({}, tickedNow);
        ticked[id] = true;
        tickedNow = ticked;
        agenda.setDone(id, done);
    }

    function sameDay(a: date, b: date): bool {
        return a.getFullYear() === b.getFullYear() && a.getMonth() === b.getMonth() && a.getDate() === b.getDate();
    }

    function isoDay(day: date): string {
        return Qt.formatDate(day, "yyyy-MM-dd");
    }

    // "Yesterday", "Today", "Tomorrow", a weekday within the week ahead,
    // else the day and month (and the year when it isn't this one).
    function dueText(due: string): string {
        const parts = due.split("-").map(Number);
        const day = new Date(parts[0], parts[1] - 1, parts[2]);
        const start = new Date(today.getFullYear(), today.getMonth(), today.getDate());
        const days = Math.round((day - start) / 86400000);
        if (days === -1) {
            return i18ndc(domain, "@label task due date", "Yesterday");
        }
        if (days === 0) {
            return i18ndc(domain, "@label task due date", "Today");
        }
        if (days === 1) {
            return i18ndc(domain, "@label task due date", "Tomorrow");
        }
        if (days > 1 && days < 7) {
            return day.toLocaleDateString(Qt.locale(), "ddd");
        }
        return day.getFullYear() === today.getFullYear()
            ? day.toLocaleDateString(Qt.locale(), "d MMM")
            : day.toLocaleDateString(Qt.locale(), Locale.ShortFormat);
    }

    function dueDays(due: string): int {
        const parts = due.split("-").map(Number);
        const day = new Date(parts[0], parts[1] - 1, parts[2]);
        const start = new Date(today.getFullYear(), today.getMonth(), today.getDate());
        return Math.round((day - start) / 86400000);
    }

    RowLayout {
        Layout.fillWidth: true
        Layout.leftMargin: section.paddings
        Layout.rightMargin: section.paddings
        Layout.topMargin: Kirigami.Units.smallSpacing

        Kirigami.Heading {
            Layout.fillWidth: true
            level: 2
            text: i18ndc(section.domain, "@title heading of the tasks list", "Tasks")
            textFormat: Text.PlainText
            elide: Text.ElideRight
        }
    }

    PlasmaComponents.TextField {
        id: addField

        Layout.fillWidth: true
        Layout.leftMargin: section.paddings
        Layout.rightMargin: section.paddings
        Layout.topMargin: Kirigami.Units.smallSpacing
        Layout.bottomMargin: Kirigami.Units.smallSpacing

        leftPadding: Kirigami.Units.iconSizes.small + Kirigami.Units.largeSpacing
        enabled: section.agenda.reachable
        placeholderText: section.todayPicked
            ? i18ndc(section.domain, "@info:placeholder", "Add a task")
            : i18ndc(section.domain, "@info:placeholder %1 is a short date", "Add a task for %1",
                section.selectedDate.toLocaleDateString(Qt.locale(), "d MMM"))

        Kirigami.Icon {
            anchors.left: parent.left
            anchors.leftMargin: Kirigami.Units.smallSpacing
            anchors.verticalCenter: parent.verticalCenter
            width: Kirigami.Units.iconSizes.small
            height: width
            source: "list-add"
        }

        Keys.onReturnPressed: add()
        Keys.onEnterPressed: add()

        function add(): void {
            const title = text.trim();
            if (title === "") {
                return;
            }
            section.agenda.addTask(title, section.todayPicked ? "" : section.isoDay(section.selectedDate));
            text = "";
        }
    }

    ListView {
        id: taskList

        Layout.fillWidth: true
        Layout.fillHeight: true
        Layout.preferredHeight: contentHeight
        Layout.maximumHeight: section.compact ? Kirigami.Units.gridUnit * 12 : -1
        Layout.minimumHeight: count === 0 ? Kirigami.Units.gridUnit * 3 : 0
        clip: true
        interactive: contentHeight > height
        boundsBehavior: Flickable.StopAtBounds

        model: section.shown

        delegate: PlasmaComponents.ItemDelegate {
            id: row

            required property var modelData

            width: ListView.view.width
            leftPadding: section.paddings
            rightPadding: section.paddings
            hoverEnabled: true
            text: modelData.title
            Accessible.description: modelData.notes

            // The whole row ticks it off: a bigger target than the box.
            onClicked: section.tick(modelData.id, !modelData.done)

            PlasmaComponents.ToolTip {
                text: row.modelData.notes
                visible: text !== "" && row.hovered
            }

            contentItem: RowLayout {
                spacing: Kirigami.Units.largeSpacing

                PlasmaComponents.CheckBox {
                    checked: row.modelData.done
                    Accessible.name: i18ndc(section.domain, "@action:button", "Done")
                    onToggled: section.tick(row.modelData.id, checked)
                }

                PlasmaComponents.Label {
                    Layout.fillWidth: true
                    text: row.modelData.title
                    textFormat: Text.PlainText
                    elide: Text.ElideRight
                    font.strikeout: row.modelData.done
                    opacity: row.modelData.done ? 0.6 : 1
                }

                // The due day as a small pill: red once late, the
                // highlight colour today.
                PlasmaComponents.Label {
                    id: dueLabel

                    readonly property int days: row.modelData.due ? section.dueDays(row.modelData.due) : 0

                    visible: row.modelData.due !== "" && !row.modelData.done
                    text: row.modelData.due ? section.dueText(row.modelData.due) : ""
                    textFormat: Text.PlainText
                    font: Kirigami.Theme.smallFont
                    leftPadding: Kirigami.Units.smallSpacing * 2
                    rightPadding: Kirigami.Units.smallSpacing * 2
                    topPadding: 1
                    bottomPadding: 1
                    color: days < 0 ? Kirigami.Theme.negativeTextColor
                        : days === 0 ? Kirigami.Theme.highlightColor
                        : Kirigami.Theme.textColor

                    background: Rectangle {
                        radius: height / 2
                        color: dueLabel.color
                        opacity: 0.14
                    }
                }
            }
        }

        PlasmaExtras.PlaceholderMessage {
            anchors.centerIn: parent
            width: parent.width - Kirigami.Units.gridUnit * 4
            visible: taskList.count === 0
            iconName: section.agenda.reachable ? "checkmark" : "network-disconnect"
            text: section.agenda.reachable
                ? i18ndc(section.domain, "@info", "No tasks")
                : i18ndc(section.domain, "@info", "Katna isn't answering")
        }
    }
}
