// SPDX-License-Identifier: GPL-3.0-or-later

pragma ComponentBehavior: Bound

import QtQuick

import org.kde.plasma.workspace.dbus as DBus

// Katna's events and tasks, from katna-daemon's in.invenia.katna.Agenda1
// interface (crates/katna-dbus/src/agenda.rs). Calling it starts the daemon
// through D-Bus activation when it isn't running. Nothing here stores or
// decides anything: the daemon does, and says Changed when to read again.
QtObject {
    id: agenda

    // Read again whenever this turns true (the popup opens).
    property bool active: false
    // The day whose events `events` holds.
    property date day: new Date()

    // [{id, title, due, done, list, notes}], open tasks by due date first.
    property var tasks: []
    // [{id, title, start, end, allDay, location, color, calendar, joinUrl}]
    // with start and end as Dates.
    property var events: []
    // False until the daemon answers once, and while it can't.
    property bool reachable: false

    readonly property string service: "in.invenia.katna.Daemon"
    readonly property string path: "/in/invenia/katna/Daemon/Agenda"
    readonly property string iface: "in.invenia.katna.Agenda1"

    onActiveChanged: if (active) {
        refresh();
    }
    onDayChanged: if (active) {
        readEvents();
    }

    function call(member: string, args: var, signature: string, done: var): void {
        DBus.SessionBus.asyncCall({
            service: agenda.service,
            path: agenda.path,
            iface: agenda.iface,
            member: member,
            arguments: args,
            signature: signature,
        }, reply => {
            agenda.reachable = true;
            if (done) {
                done(reply.value);
            }
        }, reply => {
            agenda.reachable = false;
            console.warn("Katna:", member, reply.error.message);
        });
    }

    function refresh(): void {
        readTasks();
        readEvents();
    }

    function readTasks(): void {
        call("Tasks", [], "", value => {
            agenda.tasks = (value || []).map(task => ({
                id: String(task.id),
                title: String(task.title),
                due: String(task.due || ""),
                done: Boolean(task.done),
                list: String(task.list || ""),
                notes: String(task.notes || ""),
            }));
        });
    }

    function readEvents(): void {
        const from = new Date(day.getFullYear(), day.getMonth(), day.getDate());
        const to = new Date(from.getFullYear(), from.getMonth(), from.getDate() + 1);
        call("Events", [new DBus.int64(from.getTime() / 1000), new DBus.int64(to.getTime() / 1000)], "", value => {
            agenda.events = (value || []).map(event => ({
                id: String(event.id),
                title: String(event.title),
                start: new Date(Number(event.start) * 1000),
                end: new Date(Number(event.end) * 1000),
                allDay: Boolean(event.all_day),
                location: String(event.location || ""),
                color: String(event.color || ""),
                calendar: String(event.calendar || ""),
                joinUrl: String(event.join_url || ""),
            }));
        });
    }

    // Katna's own copy of an event the calendar plugins listed, by its
    // title and start: the plugins' event data carries no ID in Plasma 6.7.
    function eventFor(title: string, start: date): var {
        return events.find(event => event.title === title
            && event.start.getTime() === start.getTime()) ?? null;
    }

    // `due` is YYYY-MM-DD or empty.
    function addTask(title: string, due: string): void {
        call("AddTask", [title, due], "", () => agenda.readTasks());
    }

    function setDone(id: string, done: bool): void {
        // Shown at once; the daemon's Changed reads the list again.
        agenda.tasks = agenda.tasks.map(task => task.id === id ? Object.assign({}, task, { done: done }) : task);
        call("SetTaskDone", [id, done], "", null);
    }

    // Katna came to the front for one of these: the popup closes.
    signal shownInKatna()

    function open(id: string): void {
        call("Open", [id], "", shown => {
            if (shown) {
                agenda.shownInKatna();
            }
        });
    }

    // A new event on `day`, in Katna's small New event window.
    function newEvent(day: date): void {
        call("NewEvent", [Qt.formatDate(day, "yyyy-MM-dd")], "", () => agenda.shownInKatna());
    }

    readonly property DBus.SignalWatcher watcher: DBus.SignalWatcher {
        busType: DBus.BusType.Session
        service: agenda.service
        path: agenda.path
        iface: agenda.iface

        function dbusChanged(): void {
            if (agenda.active) {
                agenda.refresh();
            }
        }
    }
}
