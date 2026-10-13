/*
    SPDX-FileCopyrightText: 2013 Sebastian Kügler <sebas@kde.org>
    SPDX-FileCopyrightText: 2015 Martin Klapetek <mklapetek@kde.org>
    SPDX-FileCopyrightText: 2021 Carl Schwan <carlschwan@kde.org>
    SPDX-FileCopyrightText: 2023 ivan tkachenko <me@ratijas.tk>

    SPDX-License-Identifier: GPL-2.0-or-later
*/

pragma ComponentBehavior: Bound

import QtQuick
import QtQuick.Layouts

import org.kde.plasma.plasmoid
import org.kde.ksvg as KSvg
import org.kde.plasma.workspace.calendar as PlasmaCalendar
import org.kde.plasma.components as PlasmaComponents
import org.kde.plasma.extras as PlasmaExtras
import org.kde.plasma.private.digitalclock
import org.kde.config as KConfig
import org.kde.kcmutils as KCMUtils
import org.kde.kirigami as Kirigami
import org.kde.plasma.clock

// Top-level layout containing:
// - Leading column with world clock and agenda view
// - Trailing column with current date header and calendar
//
// Trailing column fills exactly half of the popup width, then there's 1
// logical pixel wide separator, and the rest is left for the Leading.
// Representation's header is intentionally zero-sized, because Calendar view
// brings its own header, and there's currently no other way to stack them.
PlasmaExtras.Representation {
    id: calendar

    readonly property var appletInterface: root

    Kirigami.Theme.colorSet: Kirigami.Theme.Window
    Kirigami.Theme.inherit: false

    Layout.minimumWidth: (calendar.showAgenda || calendar.showClocks || calendar.showTasks) ? Kirigami.Units.gridUnit * 45 : Kirigami.Units.gridUnit * 22
    Layout.maximumWidth: Kirigami.Units.gridUnit * 80

    Layout.minimumHeight: Kirigami.Units.gridUnit * 25
    Layout.maximumHeight: Kirigami.Units.gridUnit * 40

    collapseMarginsHint: true

    readonly property int paddings: Kirigami.Units.largeSpacing
    readonly property bool showAgenda: eventPluginsManager.enabledPlugins.length > 0
    readonly property bool showClocks: root.selectedTimeZonesDeduplicatingExplicitLocalTimeZone().length > 1
    // Katna: the Tasks list is always there.
    readonly property bool showTasks: true

    readonly property alias monthView: monthView

    // A time zone as the clock's settings name it: code, city or offset.
    function zoneLabel(zone: string, clock: var): string {
        switch (Plasmoid.configuration.displayTimezoneFormat) {
        case 0: // Code
            return clock.timeZoneCode;
        case 1: // City
            return TimeZonesI18n.i18nCity(clock.timeZone);
        case 2: // Offset from UTC time
            return clock.timeZoneOffset;
        }
        return "";
    }
    // This helps synchronize the header of the agenda and the monthView.
    // We cannot use Kirigami.SizeGroup here because monthView's header is not in a layout.
    readonly property double headerHeight: Math.max(agendaHeader.implicitHeight, monthView.viewHeader.implicitHeight)

    Keys.onDownPressed: event => {
        monthView.Keys.downPressed(event);
    }

    Connections {
        target: root

        function onExpandedChanged() {
            // clear all the selections when the plasmoid is showing/hiding
            monthView.resetToToday();
        }
    }

    // Katna: the day picked in the month is the day Katna's events are read for.
    Binding {
        target: calendar.appletInterface.katna
        property: "day"
        value: monthView.currentDate
    }

    PlasmaCalendar.EventPluginsManager {
        id: eventPluginsManager
        enabledPlugins: Plasmoid.configuration.enabledCalendarPlugins
    }

    // Having this in place helps preserving top margins for Pin and Configure
    // buttons somehow. Actual headers are spread across leading and trailing
    // columns.
    header: Item {}

    // Leading column containing agenda view and time zones
    // ==================================================
    ColumnLayout {
        id: leadingColumn

        visible: calendar.showAgenda || calendar.showClocks || calendar.showTasks

        anchors {
            top: parent.top
            left: parent.left
            right: mainSeparator.left
            bottom: parent.bottom
        }

        spacing: 0

        PlasmaExtras.PlasmoidHeading {
            id: agendaHeader
            Layout.preferredHeight: calendar.headerHeight
            Layout.fillWidth: true
            leftInset: 0
            rightInset: 0

            // Agenda view header
            // -----------------
            contentItem: ColumnLayout {
                spacing: 0

                // Katna: the time zone sits beside the date, where the
                // Time Zones section under the tasks used to be cut off.
                RowLayout {
                    Layout.alignment: Qt.AlignTop
                    Layout.fillWidth: true
                    spacing: Kirigami.Units.smallSpacing

                    Kirigami.Heading {
                        // Match calendar title
                        Layout.leftMargin: calendar.paddings
                        Layout.fillWidth: true

                        text: monthView.currentDate.toLocaleDateString(Qt.locale(), Locale.LongFormat)
                        textFormat: Text.PlainText
                        elide: Text.ElideRight
                    }

                    PlasmaComponents.ToolButton {
                        id: switchTimeZoneButton

                        readonly property bool canSwitch: KConfig.KAuthorized.authorizeControlModule("kcm_clock.desktop")

                        visible: calendar.showClocks || canSwitch
                        Layout.rightMargin: Kirigami.Units.smallSpacing
                        icon.name: "preferences-system-time"
                        text: root.formatTime(root.currentTime, false) + "  " + calendar.zoneLabel(root.currentTimeZone, currentZoneClock)
                        down: pressed || timeZoneMenu.opened

                        Accessible.name: i18nd("plasma_applet_org.kde.plasma.digitalclock", "Time Zones")
                        KeyNavigation.down: addEventButton.visible ? addEventButton : addEventButton.KeyNavigation.down
                        Keys.onRightPressed: event => {
                            monthView.Keys.downPressed(event);
                        }

                        onClicked: timeZoneMenu.popup(switchTimeZoneButton, 0, switchTimeZoneButton.height)

                        Clock {
                            id: currentZoneClock
                            timeZone: root.currentTimeZone
                        }

                        PlasmaComponents.Menu {
                            id: timeZoneMenu

                            Instantiator {
                                model: calendar.showClocks ? root.selectedTimeZonesDeduplicatingExplicitLocalTimeZone() : []
                                onObjectAdded: (index, object) => timeZoneMenu.insertItem(index, object)
                                onObjectRemoved: (index, object) => timeZoneMenu.removeItem(object)

                                delegate: PlasmaComponents.MenuItem {
                                    id: zoneItem

                                    required property string modelData

                                    readonly property bool isCurrent: zoneClock.timeZone == root.currentTimeZone

                                    text: calendar.zoneLabel(modelData, zoneClock) + "    "
                                        + root.formatTime(zoneClock.dateTime, Plasmoid.configuration.showSeconds === 2)
                                        + (isCurrent ? "" : "  " + root.formatOffset(zoneClock.dateTime))
                                    font.bold: isCurrent
                                    // Picking one shows the clock in it, as the
                                    // middle click on the clock does.
                                    onTriggered: {
                                        Plasmoid.configuration.lastSelectedTimezone = modelData;
                                    }

                                    Clock {
                                        id: zoneClock
                                        timeZone: zoneItem.modelData
                                        trackSeconds: Plasmoid.configuration.showSeconds === 2
                                    }
                                }
                            }

                            PlasmaComponents.MenuSeparator {
                                visible: calendar.showClocks && switchTimeZoneButton.canSwitch
                            }

                            PlasmaComponents.MenuItem {
                                visible: switchTimeZoneButton.canSwitch
                                height: visible ? implicitHeight : 0
                                icon.name: "preferences-system-time"
                                text: i18nd("plasma_applet_org.kde.plasma.digitalclock", "Switch…")
                                Accessible.description: i18nd("plasma_applet_org.kde.plasma.digitalclock", "Switch to another time zone")
                                onTriggered: KCMUtils.KCMLauncher.openSystemSettings("kcm_clock")
                            }
                        }
                    }
                }

                PlasmaComponents.Label {
                    visible: monthView.currentDateAuxilliaryText.length > 0

                    Layout.leftMargin: calendar.paddings
                    Layout.rightMargin: calendar.paddings
                    Layout.fillWidth: true

                    font.pixelSize: Kirigami.Theme.smallFont.pixelSize
                    text: monthView.currentDateAuxilliaryText
                    textFormat: Text.PlainText
                }

                RowLayout {
                    spacing: Kirigami.Units.smallSpacing

                    Layout.alignment: Qt.AlignBottom

                    // Heading text
                    Kirigami.Heading {
                        visible: agenda.visible

                        Layout.fillWidth: true
                        Layout.leftMargin: calendar.paddings
                        Layout.rightMargin: calendar.paddings

                        level: 2

                        text: i18nd("plasma_applet_org.kde.plasma.digitalclock", "Events")
                        textFormat: Text.PlainText
                        maximumLineCount: 1
                        elide: Text.ElideRight
                    }
                    PlasmaComponents.ToolButton {
                        id: addEventButton

                        // Katna: new events go to Katna's Calendar, so it shows
                        // without a calendar app (was `&& ApplicationIntegration.calendarInstalled`).
                        visible: agenda.visible
                        text: i18ndc("plasma_applet_org.kde.plasma.digitalclock", "@action:button Add event", "Add…")
                        Layout.rightMargin: Kirigami.Units.smallSpacing
                        icon.name: "list-add"

                        Accessible.description: i18ndc("plasma_applet_org.kde.plasma.digitalclock", "@info:tooltip", "Add a new event")
                        KeyNavigation.down: KeyNavigation.tab
                        KeyNavigation.right: monthView.viewHeader.tabBar

                        // Katna: was ApplicationIntegration.launchCalendar().
                        onClicked: calendar.appletInterface.katna.newEvent(monthView.currentDate)
                        KeyNavigation.tab: calendar.showAgenda && eventsList.count ? eventsList : eventsList.KeyNavigation.down
                    }
                }
            }
        }

        // Agenda view itself
        Item {
            id: agenda
            visible: calendar.showAgenda

            Layout.fillWidth: true
            // Katna: the events take what they need, up to a few, and the
            // Tasks list the rest; with Tasks folded the events take all.
            Layout.fillHeight: katnaTasks.folded
            Layout.preferredHeight: eventsList.count > 0
                ? Math.min(eventsList.contentHeight, Kirigami.Units.gridUnit * 9)
                : noEvents.implicitHeight + Kirigami.Units.largeSpacing * 2
            Layout.minimumHeight: noEvents.implicitHeight + Kirigami.Units.largeSpacing * 2

            function formatDateWithoutYear(date: date): string {
                // Unfortunately Qt overrides ECMA's Date.toLocaleDateString(),
                // which is able to return locale-specific date-and-month-only date
                // formats, with its dumb version that only supports Qt::DateFormat
                // enum subset. So to get a day-and-month-only date format string we
                // must resort to this magic and hope there are no locales that use
                // other separators...
                const format = Qt.locale().dateFormat(Locale.ShortFormat).replace(/[./ ]*Y{2,4}[./ ]*/i, '');
                return Qt.formatDate(date, format);
            }

            function dateEquals(date1: date, date2: date): bool {
                // QDate is exposed to QML as midnight UTC, while MonthView's
                // current date is constructed in the local time zone.
                return date1.getUTCFullYear() === date2.getFullYear()
                    && date1.getUTCMonth() === date2.getMonth()
                    && date1.getUTCDate() === date2.getDate();
            }

            function updateEventsForCurrentDate() {
                eventsList.model = monthView.daysModel.eventsForDate(monthView.currentDate);
            }

            Connections {
                target: monthView

                function onCurrentDateChanged() {
                    agenda.updateEventsForCurrentDate();
                }
            }

            Connections {
                target: monthView.daysModel

                function onAgendaUpdated(updatedDate: date) {
                    if (agenda.dateEquals(updatedDate, monthView.currentDate)) {
                        agenda.updateEventsForCurrentDate();
                    }
                }
            }

            TextMetrics {
                id: dateLabelMetrics

                // Date/time are arbitrary values with all parts being two-digit
                readonly property string timeString: Qt.formatTime(new Date(2000, 12, 12, 12, 12, 12, 12))
                readonly property string dateString: agenda.formatDateWithoutYear(new Date(2000, 12, 12, 12, 12, 12))

                font: Kirigami.Theme.defaultFont
                text: timeString.length > dateString.length ? timeString : dateString
            }

            PlasmaComponents.ScrollView {
                id: eventsView
                anchors.fill: parent

                ListView {
                    id: eventsList

                    focus: false
                    activeFocusOnTab: true
                    highlight: null
                    currentIndex: -1

                    Keys.onRightPressed: event => monthView.Keys.downPressed(event)

                    onCurrentIndexChanged: if (!activeFocus) {
                        currentIndex = -1;
                    }

                    onActiveFocusChanged: if (activeFocus) {
                        currentIndex = 0;
                    } else {
                        currentIndex = -1;
                    }

                    delegate: PlasmaComponents.ItemDelegate {
                        id: eventItem

                        // Crashes if the type is declared as eventData (which is Q_GADGET)
                        required property /*PlasmaCalendar.eventData*/var modelData

                        width: ListView.view.width

                        leftPadding: calendar.paddings

                        text: eventTitle.text
                        hoverEnabled: true
                        // Katna: its own events open in Katna.
                        readonly property var katnaEvent: calendar.appletInterface.katna.eventFor(modelData.title, modelData.startDateTime)
                        onClicked: if (katnaEvent) {
                            calendar.appletInterface.katna.open(katnaEvent.id);
                        }
                        highlighted: ListView.isCurrentItem
                        Accessible.description: modelData.description
                        readonly property bool hasTime: {
                            // Explicitly all-day event
                            if (modelData.isAllDay) {
                                return false;
                            }
                            // Multi-day event which does not start or end today (so
                            // is all-day from today's point of view)
                            if (modelData.startDateTime - monthView.currentDate < 0 &&
                                modelData.endDateTime - monthView.currentDate > 86400000) { // 24hrs in ms
                                return false;
                            }

                            // Non-explicit all-day event
                            const startIsMidnight = modelData.startDateTime.getHours() === 0
                                && modelData.startDateTime.getMinutes() === 0;

                            const endIsMidnight = modelData.endDateTime.getHours() === 0
                                && modelData.endDateTime.getMinutes() === 0;

                            const sameDay = modelData.startDateTime.getDate() === modelData.endDateTime.getDate()
                                && modelData.startDateTime.getDay() === modelData.endDateTime.getDay();

                            return !(startIsMidnight && endIsMidnight && sameDay);
                        }

                        PlasmaComponents.ToolTip {
                            text: eventItem.modelData.description
                            visible: text !== "" && eventItem.hovered
                        }

                        contentItem: GridLayout {
                            id: eventGrid
                            columns: 4 // Katna: 3, and the Join button
                            rows: 2
                            rowSpacing: 0
                            columnSpacing: Kirigami.Units.largeSpacing

                            Rectangle {
                                id: eventColor

                                Layout.row: 0
                                Layout.column: 0
                                Layout.rowSpan: 2
                                Layout.fillHeight: true

                                color: eventItem.modelData.eventColor
                                implicitWidth: 5
                                visible: eventItem.modelData.eventColor !== ""
                            }

                            PlasmaComponents.Label {
                                id: startTimeLabel

                                readonly property bool startsToday: eventItem.modelData.startDateTime - monthView.currentDate >= 0
                                readonly property bool startedYesterdayLessThan12HoursAgo: eventItem.modelData.startDateTime - monthView.currentDate >= -43200000 //12hrs in ms

                                Layout.row: 0
                                Layout.column: 1
                                Layout.minimumWidth: dateLabelMetrics.width

                                text: startsToday || startedYesterdayLessThan12HoursAgo
                                    ? Qt.formatTime(eventItem.modelData.startDateTime)
                                    : agenda.formatDateWithoutYear(eventItem.modelData.startDateTime)
                                textFormat: Text.PlainText
                                horizontalAlignment: Qt.AlignRight
                                visible: eventItem.hasTime
                            }

                            PlasmaComponents.Label {
                                id: endTimeLabel

                                readonly property bool endsToday: eventItem.modelData.endDateTime - monthView.currentDate <= 86400000 // 24hrs in ms
                                readonly property bool endsTomorrowInLessThan12Hours: eventItem.modelData.endDateTime - monthView.currentDate <= 86400000 + 43200000 // 36hrs in ms

                                Layout.row: 1
                                Layout.column: 1
                                Layout.minimumWidth: dateLabelMetrics.width

                                text: endsToday || endsTomorrowInLessThan12Hours
                                    ? Qt.formatTime(eventItem.modelData.endDateTime)
                                    : agenda.formatDateWithoutYear(eventItem.modelData.endDateTime)
                                textFormat: Text.PlainText
                                horizontalAlignment: Qt.AlignRight
                                opacity: 0.75

                                visible: eventItem.hasTime
                            }

                            PlasmaComponents.Label {
                                id: eventTitle

                                Layout.row: 0
                                Layout.column: 2
                                Layout.fillWidth: true

                                elide: Text.ElideRight
                                text: eventItem.modelData.title
                                textFormat: Text.PlainText
                                verticalAlignment: Text.AlignVCenter
                                maximumLineCount: 2
                                wrapMode: Text.Wrap
                            }

                            // Katna: a meeting's video call, one click away.
                            KatnaJoinButton {
                                Layout.row: 0
                                Layout.column: 3
                                Layout.rowSpan: 2
                                Layout.alignment: Qt.AlignVCenter
                                event: eventItem.katnaEvent
                            }
                        }
                    }
                }
            }

            // Katna: a quiet line with a small tick, not Plasma's big
            // placeholder picture.
            RowLayout {
                id: noEvents

                anchors {
                    top: parent.top
                    left: parent.left
                    right: parent.right
                    topMargin: Kirigami.Units.largeSpacing
                    leftMargin: calendar.paddings
                    rightMargin: calendar.paddings
                }
                spacing: Kirigami.Units.smallSpacing
                visible: eventsList.count === 0

                Kirigami.Icon {
                    implicitWidth: Kirigami.Units.iconSizes.small
                    implicitHeight: Kirigami.Units.iconSizes.small
                    source: "checkmark"
                    color: Kirigami.Theme.positiveTextColor
                    isMask: true
                }

                PlasmaComponents.Label {
                    Layout.fillWidth: true
                    opacity: 0.7
                    elide: Text.ElideRight
                    textFormat: Text.PlainText
                    text: monthView.isToday(monthView.currentDate)
                        ? i18nd("plasma_applet_org.kde.plasma.digitalclock", "No events for today")
                        : i18nd("plasma_applet_org.kde.plasma.digitalclock", "No events for this day")
                }
            }
        }

        // Katna: the Tasks list, under the day's events.
        KSvg.SvgItem {
            visible: agenda.visible

            Layout.fillWidth: true
            Layout.preferredHeight: naturalSize.height

            imagePath: "widgets/line"
            elementId: "horizontal-line"
        }

        KatnaTasks {
            id: katnaTasks

            Layout.fillWidth: true
            Layout.fillHeight: !folded || !agenda.visible
            compact: agenda.visible
            folded: agenda.visible && Plasmoid.configuration.katnaTasksFolded
            onFoldToggled: Plasmoid.configuration.katnaTasksFolded = !Plasmoid.configuration.katnaTasksFolded
            agenda: calendar.appletInterface.katna
            selectedDate: monthView.currentDate
            paddings: calendar.paddings
        }
    }

    // Vertical separator line between columns
    // =======================================
    KSvg.SvgItem {
        id: mainSeparator

        anchors {
            top: parent.top
            right: monthViewWrapper.left
            bottom: parent.bottom
            // Stretch all the way to the top of a dialog. This magic comes
            // from PlasmaCore.PlasmaWindow::topPadding and CompactApplet containment.
            topMargin: calendar.parent ? -calendar.parent.y : 0
        }

        width: naturalSize.width
        visible: calendar.showAgenda || calendar.showClocks || calendar.showTasks

        imagePath: "widgets/line"
        elementId: "vertical-line"
    }

    // Trailing column containing calendar
    // ===============================
    FocusScope {
        id: monthViewWrapper

        anchors {
            top: parent.top
            right: parent.right
            bottom: parent.bottom
        }

        // Not anchoring to horizontalCenter to avoid sub-pixel misalignments
        width: (calendar.showAgenda || calendar.showClocks || calendar.showTasks) ? Math.round(parent.width / 2) : parent.width

        onActiveFocusChanged: if (activeFocus) {
            monthViewWrapper.nextItemInFocusChain().forceActiveFocus();
        }

        PlasmaCalendar.MonthView {
            id: monthView
            viewHeader.height: calendar.headerHeight

            anchors {
                fill: parent
                leftMargin: Kirigami.Units.smallSpacing
                rightMargin: Kirigami.Units.smallSpacing
                bottomMargin: Kirigami.Units.smallSpacing
            }

            borderOpacity: 0.25

            eventPluginsManager: eventPluginsManager
            today: root.currentTime
            firstDayOfWeek: Plasmoid.configuration.firstDayOfWeek > -1
                ? Plasmoid.configuration.firstDayOfWeek
                : Qt.locale().firstDayOfWeek
            showWeekNumbers: Plasmoid.configuration.showWeekNumbers

            showDigitalClockHeader: true
            digitalClock: Plasmoid
            eventButton: addEventButton

            KeyNavigation.left: KeyNavigation.tab
            KeyNavigation.tab: addEventButton.visible ? addEventButton : addEventButton.KeyNavigation.down
            Keys.onUpPressed: event => {
                viewHeader.tabBar.currentItem.forceActiveFocus(Qt.BacktabFocusReason);
            }

            // Katna: right-click a day for a menu. Left clicks go through.
            KatnaDayMenu {
                anchors.fill: parent
                monthView: monthView
                tasks: katnaTasks
                agenda: calendar.appletInterface.katna
            }
        }
    }
}
