// SPDX-License-Identifier: GPL-3.0-or-later

// Katna in GNOME Shell's clock menu: Katna's events join the calendar's
// (GNOME's own Evolution Data Server events stay), and a Tasks card sits
// under the day's events. Everything comes from katna-daemon's
// in.invenia.katna.Agenda1 interface (crates/katna-dbus/src/agenda.rs);
// nothing is stored here.

import Clutter from 'gi://Clutter';
import Gio from 'gi://Gio';
import GLib from 'gi://GLib';
import GObject from 'gi://GObject';
import St from 'gi://St';

import * as Main from 'resource:///org/gnome/shell/ui/main.js';
import * as Calendar from 'resource:///org/gnome/shell/ui/calendar.js';
import {CheckBox} from 'resource:///org/gnome/shell/ui/checkBox.js';
import {Extension, gettext as _, pgettext} from 'resource:///org/gnome/shell/extensions/extension.js';

const AGENDA_XML = `
<node>
  <interface name="in.invenia.katna.Agenda1">
    <method name="Events">
      <arg type="x" direction="in" name="from"/>
      <arg type="x" direction="in" name="to"/>
      <arg type="aa{sv}" direction="out" name="events"/>
    </method>
    <method name="Tasks">
      <arg type="aa{sv}" direction="out" name="tasks"/>
    </method>
    <method name="AddTask">
      <arg type="s" direction="in" name="title"/>
      <arg type="s" direction="in" name="due"/>
      <arg type="s" direction="out" name="id"/>
    </method>
    <method name="SetTaskDone">
      <arg type="s" direction="in" name="id"/>
      <arg type="b" direction="in" name="done"/>
    </method>
    <method name="DeleteTask">
      <arg type="s" direction="in" name="id"/>
    </method>
    <method name="Open">
      <arg type="s" direction="in" name="id"/>
      <arg type="b" direction="out" name="shown"/>
    </method>
    <signal name="Changed"/>
  </interface>
</node>`;

const BUS_NAME = 'in.invenia.katna.Daemon';
const OBJECT_PATH = '/in/invenia/katna/Daemon/Agenda';
const AgendaProxy = Gio.DBusProxy.makeProxyWrapper(AGENDA_XML);

const DAY_MS = 24 * 60 * 60 * 1000;

function startOfDay(date) {
    return new Date(date.getFullYear(), date.getMonth(), date.getDate());
}

function isoDay(date) {
    const pad = n => String(n).padStart(2, '0');
    return `${date.getFullYear()}-${pad(date.getMonth() + 1)}-${pad(date.getDate())}`;
}

function dayOf(iso) {
    const [y, m, d] = iso.split('-').map(Number);
    return new Date(y, m - 1, d);
}

// Katna's events and tasks. Calls start the daemon (D-Bus activation).
class Agenda {
    constructor(onChanged) {
        this._proxy = null;
        this._changedId = 0;
        this._onChanged = onChanged;
        this._cancellable = new Gio.Cancellable();
    }

    async proxy() {
        if (!this._proxy) {
            const proxy = await AgendaProxy.newAsync(
                Gio.DBus.session, BUS_NAME, OBJECT_PATH, this._cancellable,
                Gio.DBusProxyFlags.DO_NOT_LOAD_PROPERTIES);
            this._changedId = proxy.connectSignal('Changed', () => this._onChanged());
            this._proxy = proxy;
        }
        return this._proxy;
    }

    destroy() {
        this._cancellable.cancel();
        if (this._proxy && this._changedId)
            this._proxy.disconnectSignal(this._changedId);
        this._proxy = null;
    }

    async events(begin, end) {
        const proxy = await this.proxy();
        const [items] = await proxy.EventsAsync(Math.floor(begin / 1000), Math.ceil(end / 1000));
        return items.map(item => {
            const event = Object.fromEntries(Object.entries(item).map(([k, v]) => [k, v.deepUnpack()]));
            return {
                id: `katna:${event.id}`,
                date: new Date(Number(event.start) * 1000),
                end: new Date(Number(event.end) * 1000),
                summary: event.title,
            };
        });
    }

    async tasks() {
        const proxy = await this.proxy();
        const [items] = await proxy.TasksAsync();
        return items.map(item => Object.fromEntries(
            Object.entries(item).map(([k, v]) => [k, v.deepUnpack()])));
    }

    async addTask(title, due) {
        const proxy = await this.proxy();
        await proxy.AddTaskAsync(title, due);
    }

    async setDone(id, done) {
        const proxy = await this.proxy();
        await proxy.SetTaskDoneAsync(id, done);
    }
}

// GNOME's own event source (Evolution Data Server) with Katna's events
// added in.
const KatnaEventSource = GObject.registerClass(
class KatnaEventSource extends Calendar.EventSourceBase {
    _init(inner, agenda) {
        super._init();
        this._inner = inner;
        this._agenda = agenda;
        this._events = [];
        this._begin = null;
        this._end = null;
        this._innerIds = [
            inner.connect('changed', () => this.emit('changed')),
            inner.connect('notify::has-calendars', () => this.notify('has-calendars')),
            inner.connect('notify::is-loading', () => this.notify('is-loading')),
        ];
    }

    destroy() {
        this._innerIds.forEach(id => this._inner.disconnect(id));
        this._inner.destroy();
    }

    get isLoading() {
        return this._inner.isLoading;
    }

    get hasCalendars() {
        return this._inner.hasCalendars || this._events.length > 0;
    }

    reload() {
        if (this._begin && this._end)
            this._load(this._begin, this._end);
    }

    async _load(begin, end) {
        try {
            this._events = await this._agenda.events(begin, end);
        } catch (e) {
            this._events = [];
            if (!e.matches?.(Gio.IOErrorEnum, Gio.IOErrorEnum.CANCELLED))
                console.debug(`Katna: no events: ${e.message}`);
        }
        this.notify('has-calendars');
        this.emit('changed');
    }

    requestRange(begin, end) {
        this._inner.requestRange(begin, end);
        if (this._begin?.getTime() !== begin.getTime() || this._end?.getTime() !== end.getTime()) {
            this._begin = begin;
            this._end = end;
            this._load(begin, end);
        }
    }

    _katnaEvents(begin, end) {
        return this._events.filter(e => e.date < end && e.end > begin);
    }

    getEvents(begin, end) {
        return [...this._inner.getEvents(begin, end), ...this._katnaEvents(begin, end)]
            .sort((a, b) => a.date - b.date);
    }

    hasEvents(day) {
        const begin = startOfDay(day);
        const end = new Date(begin.getTime() + DAY_MS);
        return this._inner.hasEvents(day) || this._katnaEvents(begin, end).length > 0;
    }
});

// The Tasks card, styled as GNOME's events card.
const TasksSection = GObject.registerClass(
class TasksSection extends St.BoxLayout {
    _init(agenda) {
        super._init({
            style_class: 'events-button katna-tasks',
            orientation: Clutter.Orientation.VERTICAL,
            x_expand: true,
        });
        this._agenda = agenda;
        this._date = new Date();
        // Ticked off while the menu is open: shown struck through so a
        // wrong tick can be taken back.
        this._tickedNow = new Set();

        const box = new St.BoxLayout({
            style_class: 'events-box',
            orientation: Clutter.Orientation.VERTICAL,
            x_expand: true,
        });
        this.add_child(box);

        this._title = new St.Label({
            style_class: 'events-title',
            text: pgettext('heading of the tasks list', 'Tasks'),
        });
        box.add_child(this._title);

        this._list = new St.BoxLayout({
            style_class: 'events-list',
            orientation: Clutter.Orientation.VERTICAL,
            x_expand: true,
        });
        box.add_child(this._list);

        this._entry = new St.Entry({
            style_class: 'katna-task-entry',
            can_focus: true,
            x_expand: true,
        });
        this._entry.clutter_text.connect('activate', () => this._add());
        box.add_child(this._entry);
        this._updateHint();
    }

    setDate(date) {
        this._date = date;
        this._updateHint();
    }

    menuClosed() {
        this._tickedNow.clear();
    }

    _isToday() {
        return isoDay(this._date) === isoDay(new Date());
    }

    _updateHint() {
        this._entry.hint_text = this._isToday()
            ? pgettext('placeholder', 'Add a task')
            /* Translators: %s is a short date such as "3 Oct" */
            : pgettext('placeholder', 'Add a task for %s').format(
                this._date.toLocaleDateString(undefined, {day: 'numeric', month: 'short'}));
    }

    async _add() {
        const title = this._entry.get_text().trim();
        if (!title)
            return;
        this._entry.set_text('');
        try {
            await this._agenda.addTask(title, this._isToday() ? '' : isoDay(this._date));
        } catch (e) {
            console.warn(`Katna: task not added: ${e.message}`);
            this._entry.set_text(title);
        }
    }

    _dueText(due) {
        const days = Math.round((dayOf(due) - startOfDay(new Date())) / DAY_MS);
        if (days === -1)
            return pgettext('task due date', 'Yesterday');
        if (days === 0)
            return pgettext('task due date', 'Today');
        if (days === 1)
            return pgettext('task due date', 'Tomorrow');
        if (days > 1 && days < 7)
            return dayOf(due).toLocaleDateString(undefined, {weekday: 'short'});
        return dayOf(due).toLocaleDateString(undefined, {day: 'numeric', month: 'short'});
    }

    async reload() {
        let tasks;
        try {
            tasks = await this._agenda.tasks();
        } catch (e) {
            if (!e.matches?.(Gio.IOErrorEnum, Gio.IOErrorEnum.CANCELLED))
                console.debug(`Katna: no tasks: ${e.message}`);
            this.visible = false;
            return;
        }
        this.visible = true;
        this._list.destroy_all_children();
        const shown = tasks.filter(task => !task.done || this._tickedNow.has(task.id));
        for (const task of shown)
            this._list.add_child(this._row(task));
        if (shown.length === 0) {
            this._list.add_child(new St.Label({
                style_class: 'event-placeholder',
                text: _('No tasks'),
            }));
        }
    }

    _row(task) {
        const row = new St.BoxLayout({
            style_class: task.done ? 'katna-task katna-task-done' : 'katna-task',
            x_expand: true,
        });
        const check = new CheckBox(task.title);
        check.x_expand = true;
        check.checked = task.done;
        check.connect('notify::checked', async () => {
            this._tickedNow.add(task.id);
            try {
                await this._agenda.setDone(task.id, check.checked);
            } catch (e) {
                console.warn(`Katna: task not changed: ${e.message}`);
            }
        });
        row.add_child(check);
        if (task.due && !task.done) {
            const late = dayOf(task.due) < startOfDay(new Date());
            row.add_child(new St.Label({
                style_class: late ? 'katna-task-due katna-task-late' : 'katna-task-due',
                text: this._dueText(task.due),
                y_align: Clutter.ActorAlign.CENTER,
            }));
        }
        return row;
    }
});

export default class KatnaClockExtension extends Extension {
    enable() {
        const dateMenu = Main.panel.statusArea.dateMenu;
        this._dateMenu = dateMenu;
        this._agenda = new Agenda(() => this._changed());

        // Katna's events: wrap GNOME's event source in ours.
        const agenda = this._agenda;
        dateMenu._getEventSource = function () {
            return new KatnaEventSource(new Calendar.DBusEventSource(), agenda);
        };
        dateMenu._sessionUpdated();

        // The Tasks card, under the day's events.
        this._tasks = new TasksSection(this._agenda);
        dateMenu._eventsItem.get_parent().insert_child_above(this._tasks, dateMenu._eventsItem);
        this._menuId = dateMenu.menu.connect('open-state-changed', (_menu, open) => {
            if (open)
                this._tasks.reload();
            else
                this._tasks.menuClosed();
        });
        this._dateId = dateMenu._calendar.connect('selected-date-changed', (_calendar, datetime) => {
            this._tasks.setDate(new Date(datetime.to_unix() * 1000));
        });
        this._tasks.reload();
    }

    disable() {
        const dateMenu = this._dateMenu;
        dateMenu.menu.disconnect(this._menuId);
        dateMenu._calendar.disconnect(this._dateId);
        this._tasks.destroy();
        this._tasks = null;

        // Back to the class's own method.
        delete dateMenu._getEventSource;
        dateMenu._sessionUpdated();
        this._agenda.destroy();
        this._agenda = null;
        this._dateMenu = null;
    }

    _changed() {
        if (this._dateMenu?.menu.isOpen)
            this._tasks?.reload();
        this._dateMenu?._eventSource?.reload?.();
    }
}
