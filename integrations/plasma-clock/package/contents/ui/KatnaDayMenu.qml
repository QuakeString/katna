// SPDX-License-Identifier: GPL-3.0-or-later

pragma ComponentBehavior: Bound

import QtQuick

import org.kde.plasma.workspace.calendar as PlasmaCalendar
import org.kde.plasma.components as PlasmaComponents
import org.kde.plasma.private.digitalclock

// Right-click a day in the month for a menu: add a task for that day, or
// an event in the desktop's calendar app. Only right clicks stop here; left
// clicks reach the month as before.
MouseArea {
    id: area

    required property Item monthView
    required property KatnaTasks tasks

    acceptedButtons: Qt.RightButton

    onClicked: mouse => {
        const day = dayAt(area.monthView, mapToItem(area.monthView, mouse.x, mouse.y));
        if (day === null) {
            return;
        }
        day.clicked(); // picks the day, as a left click does
        menu.day = day.thisDate;
        menu.popup();
    }

    // The month's day cell (its DayDelegate, whose thisDate is the day it
    // shows) under `point` of `item`, or null outside the days view.
    function dayAt(item: Item, point: point): var {
        const child = childUnder(item, point);
        if (child === null) {
            return null;
        }
        if (child.thisDate !== undefined && child.dayModel !== undefined) {
            return child.dateMatchingPrecision === PlasmaCalendar.Calendar.MatchYearMonthAndDay ? child : null;
        }
        return dayAt(child, item.mapToItem(child, point.x, point.y));
    }

    // Item.childAt, but past this area, which lies over the month, and
    // the topmost child by z (childAt goes by order only, so in the month's
    // list it finds the empty highlight under the days).
    function childUnder(item: Item, point: point): var {
        let found = null;
        for (let i = item.children.length - 1; i >= 0; i--) {
            const child = item.children[i];
            if (child !== area && child.visible && (found === null || child.z > found.z)
                && child.contains(item.mapToItem(child, point.x, point.y))) {
                found = child;
            }
        }
        return found;
    }

    PlasmaComponents.Menu {
        id: menu

        property date day: new Date()

        PlasmaComponents.MenuItem {
            icon.name: "list-add"
            text: i18ndc("katna-clock", "@action:inmenu %1 is a short date", "Add a Task for %1",
                menu.day.toLocaleDateString(Qt.locale(), "d MMM"))
            onClicked: area.tasks.focusAdd()
        }

        PlasmaComponents.MenuItem {
            visible: ApplicationIntegration.calendarInstalled
            height: visible ? implicitHeight : 0 // a hidden item keeps its space
            icon.name: "appointment-new"
            text: i18ndc("katna-clock", "@action:inmenu", "Add an Event")
            onClicked: ApplicationIntegration.launchCalendar()
        }
    }
}
