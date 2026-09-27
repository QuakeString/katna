# Katna Mail, Assamese (অসমীয়া).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Language picker (top bar and Settings > General)

language-tooltip = ভাষা: { $language }
language-tooltip-system = ভাষা: { $language }, ছিষ্টেম অনুসৰি
language-search = ভাষা সন্ধান কৰক
language-system-default = ছিষ্টেম ডিফ'ল্ট
language-system-now = এতিয়া { $language }
language-no-match = “{ $query }”ৰ সৈতে মিলা কোনো ভাষা নাই
language-machine = যন্ত্ৰৰ দ্বাৰা অনূদিত। উন্নত কৰাত সহায় কৰক
language-setting = ভাষা
language-setting-detail = মেনু, বুটাম আৰু বাৰ্তাৰ ভাষা, আৰু তাৰিখ আৰু সংখ্যাৰ ফৰ্মেট। ছিষ্টেম ডিফ'ল্টে ডেস্কটপৰ ছেটিং অনুসৰণ কৰে।

## Dates and sizes

ago-just-now = এইমাত্ৰ
ago-minutes = { $count ->
    [one] { $count } মিনিট আগতে
   *[other] { $count } মিনিট আগতে
}
ago-hours = { $count ->
    [one] { $count } ঘণ্টা আগতে
   *[other] { $count } ঘণ্টা আগতে
}
ago-days = { $count ->
    [one] { $count } দিন আগতে
   *[other] { $count } দিন আগতে
}
size-bytes = { $count ->
    [one] { $count } বাইট
   *[other] { $count } বাইট
}
size-kb = { $size } KB
size-mb = { $size } MB
size-gb = { $size } GB
size-tb = { $size } TB

## Top bar

folders-hide = ফ'ল্ডাৰ লুকুৱাওক
folders-show = ফ'ল্ডাৰ দেখুৱাওক
compose = লিখক
search = সন্ধান কৰক
search-mail = মেইল সন্ধান কৰক
search-settings = ছেটিংছ সন্ধান কৰক
search-clear = সন্ধান মচক
search-options-show = সন্ধানৰ বিকল্প দেখুৱাওক
settings = ছেটিংছ
account-add = একাউণ্ট যোগ কৰক

## App rail (and the bottom bar on a phone)

rail-mail = মেইল
rail-calendar = কেলেণ্ডাৰ
rail-contacts = সম্পৰ্কসমূহ
rail-tasks = কাৰ্যসমূহ
rail-notes = টোকাসমূহ
rail-feeds = ফীডসমূহ

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = সোনকালে আহি আছে
app-calendar-promise = আপোনাৰ CalDAV কেলেণ্ডাৰ, আপোনাৰ মেইলত অহা মিটিঙৰ নিমন্ত্ৰণ আৰু ৰিমাইণ্ডাৰ, আপোনাৰ ইনবক্সৰ কাষতে।
app-tasks-promise = CalDAVৰ সৈতে ছিংক হোৱা কৰিবলগীয়া কামৰ তালিকা, আৰু মেইলৰ পৰা বনোৱা কাৰ্য।
app-notes-promise = খৰতকীয়া টোকা, আৰু পিছলৈ কোনো মেইল বা কথোপকথনৰ ওপৰত টোকা।
app-feeds-promise = আপোনাৰ মেইলৰ কাষতে RSS আৰু Atom ফীড পঢ়ক।

## Contacts page

app-contacts-loading = আপোনাৰ মেইলৰ পৰা লোকসকলক গোটোৱা হৈছে…
app-contacts-empty = আপুনি যিসকলৰ সৈতে মেইল আদান-প্ৰদান কৰে, তেওঁলোক ইয়াত দেখা যাব।
app-contacts-count = { $count ->
    [one] আপোনাৰ মেইলৰ পৰা { $count } জন ব্যক্তি, আটাইতকৈ বেছি মেইল আদান-প্ৰদান কৰাসকল প্ৰথমে
   *[other] আপোনাৰ মেইলৰ পৰা { $count } জন লোক, আটাইতকৈ বেছি মেইল আদান-প্ৰদান কৰাসকল প্ৰথমে
}
app-contacts-top = { $count ->
    [one] আপোনাৰ মেইলৰ পৰা শীৰ্ষ { $count } জন ব্যক্তি, আটাইতকৈ বেছি মেইল আদান-প্ৰদান কৰাসকল প্ৰথমে
   *[other] আপোনাৰ মেইলৰ পৰা শীৰ্ষ { $count } জন লোক, আটাইতকৈ বেছি মেইল আদান-প্ৰদান কৰাসকল প্ৰথমে
}
app-contacts-messages = { $count ->
    [one] { $count }টা বাৰ্তা
   *[other] { $count }টা বাৰ্তা
}
app-contacts-last = শেষবাৰ { $date }

## Navigation (the folders pane)

nav-labels = লেবেলসমূহ
nav-folders = ফ'ল্ডাৰসমূহ
nav-label-new = নতুন লেবেল সৃষ্টি কৰক
nav-folder-new = নতুন ফ'ল্ডাৰ সৃষ্টি কৰক
nav-account-unnamed = একাউণ্ট { $number }
nav-tab-new = { $count ->
    [one] { $count }টা নতুন
   *[other] { $count }টা নতুন
}

## Special folders (the user's own folders keep their names)

folder-inbox = ইনবক্স
folder-starred = তৰাচিহ্নিত
folder-drafts = ড্ৰাফ্ট
folder-sent = প্ৰেৰিত
folder-archive = আৰ্কাইভ
folder-spam = স্পাম
folder-trash = ট্ৰেছ
folder-all-mail = সকলো মেইল
folder-scheduled = নিৰ্ধাৰিত

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = নতুন লেবেল
label-folder-new-title = নতুন ফ'ল্ডাৰ
label-prompt = অনুগ্ৰহ কৰি নতুন লেবেলৰ নাম দিয়ক:
label-folder-prompt = অনুগ্ৰহ কৰি নতুন ফ'ল্ডাৰৰ নাম দিয়ক:
label-name-hint = লেবেলৰ নাম
label-folder-name-hint = ফ'ল্ডাৰৰ নাম
label-nest = লেবেলটো ইয়াৰ তলত ৰাখক:
label-folder-nest = ফ'ল্ডাৰটো ইয়াৰ তলত ৰাখক:
label-cancel = বাতিল কৰক
label-create = সৃষ্টি কৰক
label-creating = সৃষ্টি কৰি থকা হৈছে…
label-created = “{ $name }” লেবেল সৃষ্টি কৰা হ'ল।
label-folder-created = “{ $name }” ফ'ল্ডাৰ সৃষ্টি কৰা হ'ল।

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = মুখ্য
tab-promotions = প্ৰচাৰ
tab-social = সামাজিক
tab-updates = আপডেট
tab-forums = ফ'ৰাম
tab-focused = কেন্দ্ৰীভূত
tab-other = অন্যান্য
tab-inbox = ইনবক্স
tab-newsletters = বাতৰি-পত্ৰ
tab-notifications = জাননী
tab-new = { $count }টা নতুন
tab-provider-other = Katnaই সজোৱা

## Mail list: toolbar

list-select = বাছনি কৰক
list-refresh = ৰিফ্ৰেছ কৰক
list-more = অধিক
list-mark-read = পঢ়া বুলি চিহ্নিত কৰক
list-mark-unread = নপঢ়া বুলি চিহ্নিত কৰক
list-move-to = ইয়ালৈ স্থানান্তৰ কৰক
list-archive = আৰ্কাইভ কৰক
list-spam = স্পাম বুলি ৰিপৰ্ট কৰক
list-delete = মচক
list-newer = নতুন
list-older = পুৰণি
list-range = { $total }ৰ { $first }–{ $last }
list-range-about = প্ৰায় { $total }ৰ { $first }–{ $last }
list-results = “{ $query }”ৰ ফলাফল
list-results-corrected = “{ $query }”ৰ ফলাফল দেখুওৱা হৈছে
list-search-instead = ইয়াৰ সলনি “{ $query }” সন্ধান কৰক
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = সকলো
list-pick-none = এটাও নহয়
list-pick-read = পঢ়া
list-pick-unread = নপঢ়া
list-pick-starred = তৰাচিহ্নিত
list-pick-unstarred = তৰাচিহ্নবিহীন

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] সকলো { $count }টা কথোপকথন বাছনি কৰা হৈছে।
       *[other] সকলো { $count }টা কথোপকথন বাছনি কৰা হৈছে।
    }
   *[message] { $count ->
        [one] সকলো { $count }টা বাৰ্তা বাছনি কৰা হৈছে।
       *[other] সকলো { $count }টা বাৰ্তা বাছনি কৰা হৈছে।
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder }ত থকা সকলো { $count }টা কথোপকথন বাছনি কৰা হৈছে।
       *[other] { $folder }ত থকা সকলো { $count }টা কথোপকথন বাছনি কৰা হৈছে।
    }
   *[message] { $count ->
        [one] { $folder }ত থকা সকলো { $count }টা বাৰ্তা বাছনি কৰা হৈছে।
       *[other] { $folder }ত থকা সকলো { $count }টা বাৰ্তা বাছনি কৰা হৈছে।
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] স্ক্ৰীনত থকা সকলো { $count }টা কথোপকথন বাছনি কৰা হৈছে।
       *[other] স্ক্ৰীনত থকা সকলো { $count }টা কথোপকথন বাছনি কৰা হৈছে।
    }
   *[message] { $count ->
        [one] স্ক্ৰীনত থকা সকলো { $count }টা বাৰ্তা বাছনি কৰা হৈছে।
       *[other] স্ক্ৰীনত থকা সকলো { $count }টা বাৰ্তা বাছনি কৰা হৈছে।
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] সকলো { $count }টা কথোপকথন বাছনি কৰক
       *[other] সকলো { $count }টা কথোপকথন বাছনি কৰক
    }
   *[message] { $count ->
        [one] সকলো { $count }টা বাৰ্তা বাছনি কৰক
       *[other] সকলো { $count }টা বাৰ্তা বাছনি কৰক
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder }ত থকা সকলো { $count }টা কথোপকথন বাছনি কৰক
       *[other] { $folder }ত থকা সকলো { $count }টা কথোপকথন বাছনি কৰক
    }
   *[message] { $count ->
        [one] { $folder }ত থকা সকলো { $count }টা বাৰ্তা বাছনি কৰক
       *[other] { $folder }ত থকা সকলো { $count }টা বাৰ্তা বাছনি কৰক
    }
}
list-clear-selection = বাছনি আঁতৰাওক

## Mail list: empty states

list-empty-search = আপোনাৰ সন্ধানৰ সৈতে কোনো বাৰ্তা মিলা নাই।
list-empty-tab = { $tab }ত কোনো মেইল নাই।
list-empty-tab-unknown = এই টেবত কোনো মেইল নাই।
list-empty-folder = { $folder }ত কোনো বাৰ্তা নাই।
list-empty-folder-unknown = এই ফ'ল্ডাৰত কোনো বাৰ্তা নাই।
list-first-sync = আপোনাৰ মেইল অনা হৈছে…
list-first-sync-detail = মেইল অহাৰ লগে লগে ইয়াত দেখা যাব।

## Mail list: lines

row-removed = এই বাৰ্তাটো আঁতৰোৱা হ'ল।
row-starred = তৰাচিহ্নিত
row-not-starred = তৰাচিহ্নিত নহয়
row-important = গুৰুত্বপূৰ্ণ। গুৰুত্বপূৰ্ণ নহয় বুলি চিহ্নিত কৰিবলৈ ক্লিক কৰক।
row-mark-important = গুৰুত্বপূৰ্ণ বুলি চিহ্নিত কৰক
row-pinned = ওপৰত পিন কৰা হৈছে
row-pin = ওপৰত পিন কৰক
row-unpin = আনপিন কৰক

## Mail list: More menu and right-click menu

menu-reply = উত্তৰ দিয়ক
menu-reply-all = সকলোকে উত্তৰ দিয়ক
menu-forward = ফৰৱাৰ্ড কৰক
menu-archive = আৰ্কাইভ কৰক
menu-delete = মচক
menu-spam = স্পাম বুলি ৰিপৰ্ট কৰক
menu-mark-read = পঢ়া বুলি চিহ্নিত কৰক
menu-mark-unread = নপঢ়া বুলি চিহ্নিত কৰক
menu-mark-all-read = সকলোবোৰ পঢ়া বুলি চিহ্নিত কৰক
menu-star = তৰাচিহ্ন যোগ কৰক
menu-unstar = তৰাচিহ্ন আঁতৰাওক
menu-important = গুৰুত্বপূৰ্ণ বুলি চিহ্নিত কৰক
menu-not-important = গুৰুত্বপূৰ্ণ নহয় বুলি চিহ্নিত কৰক
menu-pin = ওপৰত পিন কৰক
menu-unpin = আনপিন কৰক
menu-print-all = সকলো প্ৰিণ্ট কৰক
menu-new-window = নতুন ৱিণ্ড'ত খোলক
menu-move-to = ইয়ালৈ স্থানান্তৰ কৰক
menu-move-to-heading = ইয়ালৈ স্থানান্তৰ কৰক:
menu-find-from = { $name }ৰ পৰা অহা ইমেইল বিচাৰক

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথনটো আৰ্কাইভ কৰা হ'ল।
       *[other] { $count }টা কথোপকথন আৰ্কাইভ কৰা হ'ল।
    }
   *[message] { $count ->
        [one] বাৰ্তাটো আৰ্কাইভ কৰা হ'ল।
       *[other] { $count }টা বাৰ্তা আৰ্কাইভ কৰা হ'ল।
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথনটো ট্ৰেছলৈ স্থানান্তৰ কৰা হ'ল।
       *[other] { $count }টা কথোপকথন ট্ৰেছলৈ স্থানান্তৰ কৰা হ'ল।
    }
   *[message] { $count ->
        [one] বাৰ্তাটো ট্ৰেছলৈ স্থানান্তৰ কৰা হ'ল।
       *[other] { $count }টা বাৰ্তা ট্ৰেছলৈ স্থানান্তৰ কৰা হ'ল।
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথনটো স্থানান্তৰ কৰা হ'ল।
       *[other] { $count }টা কথোপকথন স্থানান্তৰ কৰা হ'ল।
    }
   *[message] { $count ->
        [one] বাৰ্তাটো স্থানান্তৰ কৰা হ'ল।
       *[other] { $count }টা বাৰ্তা স্থানান্তৰ কৰা হ'ল।
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথনটো তৰাচিহ্নিত কৰা হ'ল।
       *[other] { $count }টা কথোপকথন তৰাচিহ্নিত কৰা হ'ল।
    }
   *[message] { $count ->
        [one] বাৰ্তাটো তৰাচিহ্নিত কৰা হ'ল।
       *[other] { $count }টা বাৰ্তা তৰাচিহ্নিত কৰা হ'ল।
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথনটোৰ তৰাচিহ্ন আঁতৰোৱা হ'ল।
       *[other] { $count }টা কথোপকথনৰ তৰাচিহ্ন আঁতৰোৱা হ'ল।
    }
   *[message] { $count ->
        [one] বাৰ্তাটোৰ তৰাচিহ্ন আঁতৰোৱা হ'ল।
       *[other] { $count }টা বাৰ্তাৰ তৰাচিহ্ন আঁতৰোৱা হ'ল।
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথনটো গুৰুত্বপূৰ্ণ বুলি চিহ্নিত কৰা হ'ল।
       *[other] { $count }টা কথোপকথন গুৰুত্বপূৰ্ণ বুলি চিহ্নিত কৰা হ'ল।
    }
   *[message] { $count ->
        [one] বাৰ্তাটো গুৰুত্বপূৰ্ণ বুলি চিহ্নিত কৰা হ'ল।
       *[other] { $count }টা বাৰ্তা গুৰুত্বপূৰ্ণ বুলি চিহ্নিত কৰা হ'ল।
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথনটো গুৰুত্বপূৰ্ণ নহয় বুলি চিহ্নিত কৰা হ'ল।
       *[other] { $count }টা কথোপকথন গুৰুত্বপূৰ্ণ নহয় বুলি চিহ্নিত কৰা হ'ল।
    }
   *[message] { $count ->
        [one] বাৰ্তাটো গুৰুত্বপূৰ্ণ নহয় বুলি চিহ্নিত কৰা হ'ল।
       *[other] { $count }টা বাৰ্তা গুৰুত্বপূৰ্ণ নহয় বুলি চিহ্নিত কৰা হ'ল।
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথনটো ওপৰত পিন কৰা হ'ল।
       *[other] { $count }টা কথোপকথন ওপৰত পিন কৰা হ'ল।
    }
   *[message] { $count ->
        [one] বাৰ্তাটো ওপৰত পিন কৰা হ'ল।
       *[other] { $count }টা বাৰ্তা ওপৰত পিন কৰা হ'ল।
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথনটো আনপিন কৰা হ'ল।
       *[other] { $count }টা কথোপকথন আনপিন কৰা হ'ল।
    }
   *[message] { $count ->
        [one] বাৰ্তাটো আনপিন কৰা হ'ল।
       *[other] { $count }টা বাৰ্তা আনপিন কৰা হ'ল।
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথনটো স্পাম বুলি ৰিপৰ্ট কৰা হ'ল।
       *[other] { $count }টা কথোপকথন স্পাম বুলি ৰিপৰ্ট কৰা হ'ল।
    }
   *[message] { $count ->
        [one] বাৰ্তাটো স্পাম বুলি ৰিপৰ্ট কৰা হ'ল।
       *[other] { $count }টা বাৰ্তা স্পাম বুলি ৰিপৰ্ট কৰা হ'ল।
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথনটো চিৰদিনৰ বাবে মচা হ'ল।
       *[other] { $count }টা কথোপকথন চিৰদিনৰ বাবে মচা হ'ল।
    }
   *[message] { $count ->
        [one] বাৰ্তাটো চিৰদিনৰ বাবে মচা হ'ল।
       *[other] { $count }টা বাৰ্তা চিৰদিনৰ বাবে মচা হ'ল।
    }
}
toast-undone = কাৰ্যটো পূৰ্বাৱস্থালৈ অনা হ'ল।
toast-undo = আনডু কৰক
toast-no-spam-folder = এই একাউণ্টত কোনো স্পাম ফ'ল্ডাৰ নাই।

## Reading pane: toolbar

reader-close = বন্ধ কৰক
reader-back = উভতি যাওক
reader-mark-unread = নপঢ়া বুলি চিহ্নিত কৰক
reader-move-to = ইয়ালৈ স্থানান্তৰ কৰক
reader-more = অধিক
reader-print-all = সকলো প্ৰিণ্ট কৰক
reader-new-window = নতুন ৱিণ্ড'ত
reader-position = { $total }ৰ { $position }
reader-newer = নতুন
reader-older = পুৰণি

## Reading pane: the conversation

reader-removed = এই কথোপকথনটো আঁতৰোৱা হ'ল।
reader-no-subject = (কোনো বিষয় নাই)
reader-collapse-all = সকলো সংকুচিত কৰক
reader-expand-all = সকলো বিস্তাৰ কৰক
reader-unknown-sender = (অজ্ঞাত প্ৰেৰক)
reader-date-ago = { $date } ({ $ago })
reader-me = মই
reader-to = প্ৰাপক: { $names }
reader-starred = তৰাচিহ্নিত
reader-not-starred = তৰাচিহ্নিত নহয়
reader-too-long = বাৰ্তাটো সম্পূৰ্ণকৈ দেখুৱাবলৈ অতি দীঘল।
reader-encrypted-images = এনক্ৰিপ্ট কৰা মেইলত ৱেবৰ পৰা ছবি কেতিয়াও লোড কৰা নহয়।
reader-window-failed = নতুন ৱিণ্ড' খুলিব পৰা নগ'ল।

## Reading pane: message details (opened from "to me")

reader-details-from = প্ৰেৰক:
reader-details-to = প্ৰাপক:
reader-details-cc = cc:
reader-details-date = তাৰিখ:
reader-details-subject = বিষয়:

## Reading pane: downloading a message

reader-downloading = ছাৰ্ভাৰৰ পৰা এই বাৰ্তাটো ডাউনল'ড কৰা হৈছে…
reader-download-failed = এই বাৰ্তাটো ডাউনল'ড কৰিব পৰা নগ'ল।
reader-try-again = পুনৰ চেষ্টা কৰক

## Reply row

reply-reply = উত্তৰ দিয়ক
reply-reply-all = সকলোকে উত্তৰ দিয়ক
reply-forward = ফৰৱাৰ্ড কৰক

## Encrypted and signed mail

security-decrypting = ডিক্ৰিপ্ট কৰা হৈছে…
security-checking = স্বাক্ষৰ পৰীক্ষা কৰা হৈছে…
security-partly-encrypted = এই বাৰ্তাটোৰ কেৱল এটা অংশহে এনক্ৰিপ্ট কৰা। বাকী অংশ সুৰক্ষাৰ বাহিৰত যোগ কৰা হৈছিল আৰু যিকোনো লোকৰ পৰা আহিব পাৰে।
security-partly-signed = এই বাৰ্তাটোৰ কেৱল এটা অংশতহে স্বাক্ষৰ আছে। বাকী অংশ সুৰক্ষাৰ বাহিৰত যোগ কৰা হৈছিল আৰু যিকোনো লোকৰ পৰা আহিব পাৰে।
security-encrypted = এনক্ৰিপ্ট কৰা বাৰ্তা
security-encrypted-smime = এনক্ৰিপ্ট কৰা বাৰ্তা (S/MIME)
security-no-key = এই বাৰ্তাটো ডিক্ৰিপ্ট কৰিব নোৱাৰি: ইয়াক এনে এটা কীৰ বাবে এনক্ৰিপ্ট কৰা হৈছিল যিটো আপোনাৰ ওচৰত নাই।
security-cancelled = ডিক্ৰিপ্ট কৰা বাতিল কৰা হ'ল।
security-damaged = এই বাৰ্তাটো ডিক্ৰিপ্ট কৰিব নোৱাৰি: এনক্ৰিপ্ট কৰা ডেটা ক্ষতিগ্ৰস্ত বা সলনি কৰা হৈছে।
security-decrypt-unavailable = এই বাৰ্তাটো ডিক্ৰিপ্ট কৰিব নোৱাৰি: এনক্ৰিপ্ট কৰা মেইল পঢ়িবলৈ { $tool } ইনষ্টল কৰক।
security-decrypt-failed = এই বাৰ্তাটো ডিক্ৰিপ্ট কৰিব নোৱাৰি: { $reason }
security-unknown-signer = এজন অজ্ঞাত স্বাক্ষৰকাৰী
security-signed-verified = { $signer }ৰ দ্বাৰা স্বাক্ষৰিত · সত্যাপিত
security-signed-not-sender = { $signer }ৰ দ্বাৰা স্বাক্ষৰিত, যিজন প্ৰেৰক নহয়
security-signed-untrusted = { $signer }ৰ দ্বাৰা স্বাক্ষৰিত, এনে এটা কীৰে যিটো আপুনি অবিশ্বাসী বুলি চিহ্নিত কৰিছে
security-signed-unverified = { $signer }ৰ দ্বাৰা স্বাক্ষৰিত · কীটো সত্যাপিত নহয়
security-bad-signature = বেয়া স্বাক্ষৰ: স্বাক্ষৰ কৰাৰ পিছত এই বাৰ্তাটো সলনি কৰা হৈছে, বা স্বাক্ষৰটো জাল।
security-signature-expired = { $signer }ৰ দ্বাৰা স্বাক্ষৰিত · স্বাক্ষৰৰ ম্যাদ উকলিছে
security-key-expired = { $signer }ৰ দ্বাৰা স্বাক্ষৰিত · তাৰ পিছত কীটোৰ ম্যাদ উকলিছে
security-key-revoked = { $signer }ৰ দ্বাৰা এনে এটা কীৰে স্বাক্ষৰিত যিটো প্ৰত্যাহাৰ কৰা হৈছে
security-missing-key = আপোনাৰ ওচৰত নথকা এটা কীৰে স্বাক্ষৰিত, সেয়ে পৰীক্ষা কৰিব নোৱাৰি
security-missing-key-id = আপোনাৰ ওচৰত নথকা এটা কীৰে ({ $key }) স্বাক্ষৰিত, সেয়ে পৰীক্ষা কৰিব নোৱাৰি
security-signature-unavailable = স্বাক্ষৰিত; স্বাক্ষৰ পৰীক্ষা কৰিবলৈ { $tool } ইনষ্টল কৰক
security-signature-error = স্বাক্ষৰ পৰীক্ষা কৰিব পৰা নগ'ল।

## Remote images and pictures

remote-hidden = এই বাৰ্তাটোৰ ছবিসমূহ লুকুৱাই ৰখা হৈছে।
remote-show = ছবি দেখুৱাওক
remote-always-show = এই প্ৰেৰকৰ পৰা সদায় দেখুৱাওক
remote-picture-use = ব্যৱহাৰ কৰক
remote-picture-too-big = 8 MB বা তাতকৈ সৰু ছবি বাছনি কৰক।
remote-picture-type = এটা PNG, JPEG, GIF, WebP বা SVG ছবি বাছনি কৰক।
remote-picture-read-failed = ছবিখন পঢ়িব নোৱাৰি: { $error }
remote-picture-keep-failed = ছবিখন ৰাখিব নোৱাৰি: { $error }
remote-picture-remove-failed = ছবিখন আঁতৰাব নোৱাৰি: { $error }

## Attachments

attachment-count = { $count ->
    [one] এটা সংলগ্নক
   *[other] { $count }টা সংলগ্নক
}
attachment-save = ছেভ কৰক
attachment-save-all = সকলো ছেভ কৰক
attachment-save-all-tooltip = সকলো সংলগ্নক এটা ফ'ল্ডাৰত ছেভ কৰক
attachment-save-here = ইয়াত ছেভ কৰক
attachment-not-downloaded = এই বাৰ্তাটো ডাউনল'ড কৰা হোৱা নাই।
attachment-not-found = বাৰ্তাটোত এই সংলগ্নকটো বিচাৰি পোৱা নগ'ল।
attachment-read-failed = { $name } পঢ়িব পৰা নগ'ল
attachment-numbered = সংলগ্নক { $number }
attachment-saved-all = { $count ->
    [one] { $count }টা ফাইল { $place }ত ছেভ কৰা হ'ল
   *[other] { $count }টা ফাইল { $place }ত ছেভ কৰা হ'ল
}
attachment-saved-some = { $total ->
    [one] { $total }টা ফাইলৰ { $saved }টা { $place }ত ছেভ কৰা হ'ল। { $failed } ছেভ কৰিব পৰা নগ'ল
   *[other] { $total }টা ফাইলৰ { $saved }টা { $place }ত ছেভ কৰা হ'ল। { $failed } ছেভ কৰিব পৰা নগ'ল
}
attachment-saved-to = { $path }ত ছেভ কৰা হ'ল
attachment-save-failed = { $name } ছেভ কৰিব পৰা নগ'ল: { $error }
attachment-open-failed = { $name } খুলিব পৰা নগ'ল: { $error }
attachment-risky = এই ফাইলটোৱে এটা প্ৰগ্ৰাম চলাব পাৰে, সেয়ে Katnaই ইয়াক নোখোলে। ইয়াৰ সলনি ইয়াক ছেভ কৰক।
attachment-encrypted-open = এই ফাইলটো এনক্ৰিপ্ট হৈ আহিছিল। অন্যত খুলিবলৈ ইয়াক ছেভ কৰক।

## Printing

print-failed = প্ৰিণ্ট কৰিব পৰা নগ'ল: { $error }
print-no-font = কোনো ফণ্ট পোৱা নগ'ল
print-opened-as-pdf = তাৰ পৰা প্ৰিণ্ট কৰিবলৈ PDF হিচাপে খোলা হ'ল।
print-not-downloaded = (এতিয়াও ডাউনল'ড কৰা হোৱা নাই।)
print-encrypted = (এনক্ৰিপ্ট কৰা। ইয়াৰ পাঠ প্ৰিণ্ট কৰিবলৈ ইয়াক Katna Mailত খোলক।)
print-to = প্ৰাপক: { $addresses }
print-cc = Cc: { $addresses }
