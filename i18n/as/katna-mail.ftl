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

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = এই বাৰ্তাটোৰ সংলগ্নকসমূহ পঢ়িবলৈ ইয়াক খোলক।
text-copy = কপি কৰক
text-select-all = সকলো বাছনি কৰক

## Settings page: its tabs

settings-tab-general = সাধাৰণ
settings-tab-inbox = ইনবক্স
settings-tab-accounts = একাউণ্টসমূহ
settings-tab-subscriptions = ছাবস্ক্ৰিপশ্বন
settings-tab-appearance = ৰূপ
settings-tab-shortcuts = শ্বৰ্টকাট
settings-tab-default-apps = ডিফ'ল্ট এপ
settings-tab-folders-rules = ফ'ল্ডাৰ আৰু নিয়ম
settings-tab-compose = লিখা
settings-tab-mcp-server = MCP ছাৰ্ভাৰ
settings-tab-feedback = ব্যৱহাৰকাৰীৰ মতামত
settings-tab-experimental = পৰীক্ষামূলক

## Settings page: tabs still to come

settings-tab-subscriptions-coming = আপুনি পোৱা বাতৰি-পত্ৰ আৰু মেইলিং তালিকাসমূহ চাওক, আৰু এক ক্লিকতে আনছাবস্ক্ৰাইব কৰক।
settings-tab-folders-rules-coming = ফ'ল্ডাৰ আৰু লেবেল সৃষ্টি কৰক, নাম সলনি কৰক, স্থানান্তৰ কৰক আৰু লুকুৱাওক, আৰু কোনবোৰ ছিংক হ'ব বাছনি কৰক। নিয়মসমূহে নতুন মেইল প্ৰেৰক, বিষয় বা শব্দ অনুসৰি নিজে সজায়, লেবেল লগায়, ফৰৱাৰ্ড কৰে বা মচে।
settings-tab-mcp-server-coming = এই কম্পিউটাৰৰ AI সহায়কসমূহক আপোনাৰ অনুমতি সাপেক্ষে আপোনাৰ মেইল সন্ধান কৰিবলৈ, পঢ়িবলৈ আৰু ড্ৰাফ্ট কৰিবলৈ দিয়ক।

## Settings > General

settings-general-conversations = কথোপকথন দৰ্শন
settings-general-conversations-group = একেটা মেইলৰ উত্তৰসমূহ একেলগ কৰক
settings-general-conversations-group-detail = তালিকাত প্ৰতিটো কথোপকথনৰ বাবে এটা শাৰী
settings-general-reading = পঢ়া
settings-general-newest-first = আটাইতকৈ নতুন বাৰ্তা প্ৰথমে
settings-general-newest-first-detail = কথোপকথন ইয়াৰ শেহতীয়া উত্তৰৰে আৰম্ভ হয়
settings-general-full-headers = সম্পূৰ্ণ হেডাৰ দেখুৱাওক
settings-general-full-headers-detail = প্ৰতিটো বাৰ্তাত প্ৰেৰক, প্ৰাপক, cc, তাৰিখ আৰু বিষয় খোলা থাকে
settings-general-full-names = প্ৰাপকৰ সম্পূৰ্ণ নাম
settings-general-full-names-detail = “মোলৈ, Ada”ৰ সলনি “মোলৈ, Ada Lovelace”
settings-general-mark-read = পঢ়া বুলি চিহ্নিত কৰক
settings-general-mark-read-now = খোলাৰ লগে লগে
settings-general-mark-read-1s = 1 ছেকেণ্ড খোলা থকাৰ পিছত
settings-general-mark-read-3s = 3 ছেকেণ্ড খোলা থকাৰ পিছত
settings-general-mark-read-never = কেৱল মই পঢ়া বুলি চিহ্নিত কৰিলেহে
settings-general-reply-button = উত্তৰ বুটাম
settings-general-reply-all = সকলোকে উত্তৰ দিয়ক
settings-general-reply-all-detail = প্ৰতিটো বাৰ্তাৰ কাষৰ উত্তৰ বুটামে কেৱল প্ৰেৰকক নহয়, সকলোকে উত্তৰ দিয়ে
settings-general-remote-images = ৱেবৰ পৰা ছবি
settings-general-remote-images-detail = কোনো বাৰ্তাৰ ছবি লোড কৰিলে ইয়াৰ প্ৰেৰকে জানিব পাৰে যে আপুনি ইয়াক খুলিছে, কেতিয়া, আৰু প্ৰায় ক'ৰ পৰা। অফ থাকিলে, প্ৰতিটো বাৰ্তাই প্ৰথমে সোধে, আৰু আপুনি সদায় কোনো প্ৰেৰকৰ ছবি দেখুৱাব পাৰে।
settings-general-remote-images-always = সদায় ছবি দেখুৱাওক
settings-general-remote-images-always-detail = প্ৰতিটো বাৰ্তাত, কেৱল আপুনি বিশ্বাস কৰা প্ৰেৰকৰ পৰাই নহয়
settings-general-sending = পঠোৱা
settings-general-sending-detail = পঠোৱা বাৰ্তা এটাই কিমান সময় অপেক্ষা কৰে, যাতে ইয়াক ঘূৰাই আনিব পাৰি।
settings-general-offline = অফলাইন মেইল
settings-general-offline-detail = সংযোগ নোহোৱাকৈ পঢ়িবলৈ শেহতীয়া মেইল সম্পূৰ্ণকৈ ডাউনল'ড কৰা হয়। পুৰণি মেইল আপুনি খুলিলে ডাউনল'ড হয়।
settings-general-offline-days = { $count ->
    [one] { $count } দিন
   *[other] { $count } দিন
}
settings-general-offline-years = { $count ->
    [one] { $count } বছৰ
   *[other] { $count } বছৰ
}
settings-general-offline-all = সকলো মেইল
settings-general-offline-note = কম দিন বাছনি কৰিলে ইতিমধ্যে ডাউনল'ড হোৱা মেইল থাকি যায়। ছাৰ্ভাৰত একো সলনি নহয়।
settings-general-notifications = জাননী
settings-general-notifications-detail = ইনবক্সত নতুন মেইলৰ বাবে, Katna Mail বন্ধ থাকিলেও।
settings-general-new-mail = নতুন মেইলৰ বিষয়ে মোক জনাওক
settings-general-new-mail-detail = সকলোকে উত্তৰ দিয়ক, পঢ়া বুলি চিহ্নিত কৰক আৰু আৰ্কাইভ কৰকৰ সৈতে
settings-general-new-mail-sound = শব্দ বজাওক
settings-general-new-mail-sound-detail = ডেস্কটপৰ নতুন মেইলৰ শব্দ
settings-general-desktop = ডেস্কটপ
settings-general-open-at-login = লগইনৰ সময়ত Katna Mail খোলক
settings-general-open-at-login-detail = সেৱা চলি থকালৈকে, লগইনৰ সময়ত মেইল যিকোনো প্ৰকাৰে ছিংক হয়
settings-general-tray = ছিষ্টেম ট্ৰেত Katna দেখুৱাওক
settings-general-tray-detail = নপঢ়াৰ সংখ্যা আৰু এটা মেনুৰ সৈতে
settings-general-unread-badge = টাস্কবাৰ আইকনত নপঢ়াৰ সংখ্যা
settings-general-unread-badge-detail = ইনবক্সৰ কিমান বাৰ্তা পঢ়া হোৱা নাই

## Settings > Inbox

settings-inbox-tabs = ইনবক্স টেব
settings-inbox-tabs-detail = আপোনাৰ মেইল প্ৰদানকাৰীৰ ৱেবছাইটৰ দৰে, ইনবক্সক টেবত সজাওক।
settings-inbox-tabs-show = ইনবক্স টেব দেখুৱাওক
settings-inbox-tabs-show-detail = অফ থাকিলে প্ৰতিটো একাউণ্টৰ বাবে এটা তালিকা দেখুৱায়
settings-inbox-no-accounts = টেব বাছনি কৰিবলৈ এটা একাউণ্ট যোগ কৰক।
settings-inbox-tabs-automatic = স্বয়ংক্ৰিয়: { $tabs } ({ $provider })
settings-inbox-tabs-off = কোনো টেব নাই
settings-inbox-tabs-gmail = মুখ্য, প্ৰচাৰ, সামাজিক, আপডেট, ফ'ৰাম
settings-inbox-tabs-focused = কেন্দ্ৰীভূত আৰু অন্যান্য
settings-inbox-tabs-zoho = ইনবক্স, বাতৰি-পত্ৰ আৰু জাননী
settings-inbox-tabs-shown = দেখুওৱা টেব। আপুনি অফ কৰা টেবৰ মেইল { $tab }ত থাকে।

## Settings > Appearance

settings-appearance-reading-pane = পঢ়া পেন
settings-appearance-reading-pane-detail = খোলা কথোপকথন ক'ত দেখা যায়।
settings-appearance-pane-right = তালিকাৰ সোঁফালে
settings-appearance-pane-none = বিভাজন নাই
settings-appearance-density = ঘনত্ব
settings-appearance-density-default = ডিফ'ল্ট
settings-appearance-density-compact = কম্পেক্ট
settings-appearance-scaling = স্কেলিং
settings-appearance-scaling-detail = ডেস্কটপৰ নিজৰ স্কেলৰ ওপৰত, Katna Mailৰ সকলোবোৰ ডাঙৰ বা সৰু কৰে: পাঠ, আইকন, ব্যৱধান আৰু বিভাজক। আপুনি পঠোৱা মেইলে নিজৰ ফণ্টৰ আকাৰ ৰাখে। অতি সৰু আকাৰত আইকনত ক্লিক কৰাটো কঠিন হ'ব পাৰে।
settings-appearance-theme = থীম
settings-appearance-theme-system = ডেস্কটপৰ দৰেই
settings-appearance-theme-light = পোহৰ
settings-appearance-theme-dark = গাঢ়
settings-appearance-desktop-colors = ডেস্কটপৰ ৰং
settings-appearance-desktop-colors-use = ডেস্কটপৰ ৰং ব্যৱহাৰ কৰক
settings-appearance-desktop-colors-use-detail = ডেস্কটপৰ ৰঙৰ আঁচনি আৰু একচেণ্ট ৰং
settings-appearance-app-names = এপৰ নাম
settings-appearance-app-names-show = এপৰ নাম দেখুৱাওক
settings-appearance-app-names-show-detail = একেবাৰে বাওঁফালৰ এপ আইকনৰ তলত নাম
settings-appearance-sender-pictures = প্ৰেৰকৰ ছবি
settings-appearance-sender-pictures-show = কোম্পানীৰ লোগো দেখুৱাওক
settings-appearance-sender-pictures-show-detail = প্ৰেৰকৰ ড'মেইনৰ দ্বাৰা বিচৰা হয়, কেতিয়াও বাৰ্তাৰ দ্বাৰা নহয়, আৰু এসপ্তাহলৈ ৰখা হয়
settings-appearance-important = গুৰুত্বপূৰ্ণ চিহ্ন
settings-appearance-important-show = গুৰুত্বপূৰ্ণ চিহ্ন দেখুৱাওক
settings-appearance-important-show-detail = তালিকাত প্ৰতিটো বাৰ্তাৰ কাষত
settings-appearance-message-width = বাৰ্তাৰ বহল
settings-appearance-message-width-limit = বাৰ্তাৰ বহল সীমিত কৰক
settings-appearance-message-width-limit-detail = বহল ৱিণ্ড'ত দীঘল শাৰী পঢ়িবলৈ সহজ হয়
settings-appearance-mail-colors = মেইলৰ ৰং
settings-appearance-mail-colors-detail = বেছিভাগ মেইল বগা পৃষ্ঠাৰ বাবে ডিজাইন কৰা হয়। গাঢ় থীমত ইয়াৰ ৰং ভালদৰে পঢ়িব পৰা গাঢ় ৰঙলৈ সলনি কৰা হয়; অফ থাকিলে, ই পোহৰ পৃষ্ঠাত ইয়াৰ প্ৰেৰকৰ ৰং ৰাখে।
settings-appearance-dark-mail = মেইলৰ বাবেও গাঢ় ৰং
settings-appearance-dark-mail-detail = কেৱল থীম গাঢ় হৈ থাকোঁতে
settings-appearance-attachment-previews = সংলগ্নকৰ পূৰ্বদৰ্শন
settings-appearance-attachment-previews-show = সংলগ্নকৰ পূৰ্বদৰ্শন দেখুৱাওক
settings-appearance-attachment-previews-show-detail = প্ৰতিটো ফাইলৰ কাৰ্ডত ইয়াৰ সমলৰ এখন সৰু ছবি

## Settings > Default apps

settings-default-apps-intro = আপুনি সংলগ্নকত ক্লিক কৰিলে সেইবোৰ ক'ত খোল খায়। ভিউৱাৰে সদায় এটা ফাইল আন এপতো খুলিব পাৰে। ডেস্কটপৰ ডিফ'ল্ট এপসমূহ ইয়াৰ নিজৰ ছেটিংছত ছেট কৰা হয়।
settings-default-apps-pdf = PDF ফাইল
settings-default-apps-pdf-detail = পৃষ্ঠা, জুমৰ সৈতে।
settings-default-apps-pictures = ছবি
settings-default-apps-pictures-detail = ফট' (পোন কৰা), PNG, GIF, WebP, BMP, TIFF আৰু SVG।
settings-default-apps-text = পাঠ ফাইল
settings-default-apps-text-detail = সাধাৰণ পাঠ, লগ, ক'ড আৰু অন্য পাঠ।
settings-default-apps-sheets = স্প্ৰেডশ্বীট
settings-default-apps-sheets-detail = Excel (xlsx, xls), OpenDocument (ods) আৰু CSV।
settings-default-apps-documents = নথি
settings-default-apps-documents-detail = Word (docx) আৰু OpenDocument পাঠ (odt)।
settings-default-apps-katna = Katna Mailৰ ভিউৱাৰ
settings-default-apps-system = ডেস্কটপৰ ডিফ'ল্ট এপ
settings-default-apps-ask = প্ৰতিবাৰ কোন এপ সোধক
settings-default-apps-after-saving = ছেভ কৰাৰ পিছত
settings-default-apps-show-folder = ছেভ কৰা ফাইল সেইবোৰৰ ফ'ল্ডাৰত দেখুৱাওক
settings-default-apps-show-folder-detail = ছেভ কৰা সংলগ্নকসমূহ বাছনি কৰা অৱস্থাত ফাইল মেনেজাৰ খোলে

## Settings > Compose

settings-compose-send-from = নতুন বাৰ্তা ইয়াৰ পৰা পঠিয়াওক
settings-compose-send-from-detail = উত্তৰ আৰু ফৰৱাৰ্ড সদায় আপুনি থকা একাউণ্টৰ পৰা যায়।
settings-compose-send-from-current = আপুনি থকা একাউণ্ট
settings-compose-send-on-replies = উত্তৰত পঠিয়াওক
settings-compose-send-on-replies-detail = উত্তৰ বা ফৰৱাৰ্ডত পঠিয়াওকে কি কৰে। পঠিয়াওকৰ কাষৰ মেনুৱে আনটো দিয়ে।
settings-compose-send-plain = পঠিয়াওক
settings-compose-send-archive = পঠিয়াওক আৰু আৰ্কাইভ কৰক
settings-compose-signatures = স্বাক্ষৰ
settings-compose-signatures-detail = আপোনাৰ বাৰ্তাৰ তলত, এটা “--” শাৰীৰ পিছত যোগ কৰা হয়। লিখা ৱিণ্ড'ত আন এটা বাছনি কৰক।
settings-compose-untitled = শিৰোনামহীন
settings-compose-signature-name = নাম, যেনে কাম
settings-compose-signature-first = মোৰ স্বাক্ষৰ
settings-compose-signature-numbered = স্বাক্ষৰ { $number }
settings-compose-signature-delete = মচক
settings-compose-signature-deleted = স্বাক্ষৰ মচা হ'ল
settings-compose-signature-new = নতুন সৃষ্টি কৰক
settings-compose-no-signatures = এতিয়ালৈকে কোনো স্বাক্ষৰ নাই।
settings-compose-no-signature = কোনো স্বাক্ষৰ নাই
settings-compose-for-new-mail = নতুন মেইলৰ বাবে
settings-compose-for-replies = উত্তৰ আৰু ফৰৱাৰ্ডৰ বাবে
settings-compose-for-replies-detail = যি কথোপকথনত আপুনি কোনো বাৰ্তাত স্বাক্ষৰ কৰিছিল, তাত উত্তৰ তাৰ সলনি সেই স্বাক্ষৰৰে আৰম্ভ হয়।
settings-compose-format = ফৰ্মেট
settings-compose-plain-text = সাধাৰণ পাঠত লিখক
settings-compose-plain-text-detail = নতুন মেইল ফৰ্মেটিং নোহোৱাকৈ আৰম্ভ হয়; লিখা ৱিণ্ড'ত সলনি কৰিব পাৰি
settings-compose-spelling = বানান
settings-compose-spell-check = মই লিখি থাকোঁতে বানান পৰীক্ষা কৰক
settings-compose-spell-check-detail = ভুল বানানৰ শব্দৰ তলত ৰেখা টনা হয়, সোঁ-ক্লিকত পৰামৰ্শৰ সৈতে
settings-compose-spell-desktop = ডেস্কটপৰ ভাষা ({ $language })
settings-compose-templates = টেমপ্লেট
settings-compose-templates-detail = আপুনি সঘনাই লিখা মেইল ছেভ কৰক, আৰু তাৰ পৰা নতুন মেইল বা উত্তৰ আৰম্ভ কৰক।

## Settings > Shortcuts

settings-shortcuts-set = শ্বৰ্টকাট ছেট
settings-shortcuts-set-detail = আপুনি জনা এটা মেইল এপৰ কীৰ পৰা আৰম্ভ কৰক। ইয়াত Cmd মানে Ctrl। আপোনাৰ নিজৰ সলনিসমূহ ছেটৰ ওপৰত থাকে, আৰু ডিফ'ল্ট পুনৰুদ্ধাৰ কৰকে ছেটৰ কীলৈ ঘূৰাই নিয়ে।
settings-shortcuts-single = এক-কী শ্বৰ্টকাট
settings-shortcuts-single-detail = ৱেবমেইলৰ দৰে, Ctrl বা Alt নোহোৱা কী: e-য়ে আৰ্কাইভ কৰে, j আৰু k-য়ে স্থানান্তৰ কৰে, /-এ সন্ধান কৰে। এইবোৰ তালিকা আৰু খোলা কথোপকথনত কাম কৰে, টাইপ কৰি থাকোঁতে কেতিয়াও নহয়।
settings-shortcuts-single-use = এক-কী শ্বৰ্টকাট ব্যৱহাৰ কৰক
settings-shortcuts-single-use-detail = Ctrl শ্বৰ্টকাট সদায় কাম কৰে
settings-shortcuts-how = কী সলনি কৰিবলৈ তাত ক্লিক কৰক, বা নতুন যোগ কৰিবলৈ +ত, তাৰ পিছত নতুন কী টিপক। Esc-এ বাতিল কৰে।
settings-shortcuts-restore = ডিফ'ল্ট পুনৰুদ্ধাৰ কৰক
settings-shortcuts-no-key = কোনো কী নাই
settings-shortcuts-press = কী টিপক…
settings-shortcuts-then = { $keys } তাৰ পিছত…
settings-shortcuts-moved = { $keys }-এ এতিয়া “{ $previous }”ৰ সলনি “{ $action }” কৰে।
settings-shortcuts-single-off = এক-কী শ্বৰ্টকাট অফ আছে, সেয়ে সেইবোৰ অন হ'লে এই কীয়ে কাম কৰিব।
settings-shortcuts-restored = প্ৰতিটো শ্বৰ্টকাটে পুনৰ ইয়াৰ ছেটৰ কী পালে।

## Settings search: the line under a result

settings-general-language-summary = এপ, তাৰিখ আৰু সংখ্যাৰ ভাষা
settings-general-reading-summary = আটাইতকৈ নতুন বাৰ্তা প্ৰথমে, সম্পূৰ্ণ হেডাৰ, প্ৰাপকৰ সম্পূৰ্ণ নাম
settings-general-mark-read-summary = খোলা কথোপকথন কেতিয়া পঢ়া বুলি চিহ্নিত হয়: লগে লগে, 1 বা 3 ছেকেণ্ডৰ পিছত, বা নিজে
settings-general-reply-button-summary = প্ৰতিটো বাৰ্তাৰ কাষৰ উত্তৰ বুটামে সকলোকে উত্তৰ দিয়ে
settings-general-remote-images-summary = প্ৰতিটো বাৰ্তাৰ ছবি সদায় দেখুৱাওক
settings-general-sending-summary = পঠোৱা আনডু কৰক: পঠোৱা বাৰ্তা এটাই কিমান সময় অপেক্ষা কৰে, যাতে ইয়াক ঘূৰাই আনিব পাৰি
settings-general-offline-summary = সংযোগ নোহোৱাকৈ পঢ়িবলৈ কিমান দিনৰ শেহতীয়া মেইল সম্পূৰ্ণকৈ ডাউনল'ড হয়
settings-general-notifications-summary = নতুন মেইলৰ জাননী আৰু সেইবোৰৰ শব্দ
settings-general-desktop-summary = লগইনৰ সময়ত Katna Mail খোলক, ছিষ্টেম ট্ৰে আইকন আৰু টাস্কবাৰ আইকনত নপঢ়াৰ সংখ্যা
settings-accounts-accounts-summary = একাউণ্ট যোগ কৰক বা আঁতৰাওক, বা ইয়াৰ ছবি সলনি কৰক
settings-appearance-density-summary = তালিকাত ডিফ'ল্ট বা কম্পেক্ট শাৰী
settings-appearance-scaling-summary = সকলোবোৰ ডাঙৰ বা সৰু কৰক: পাঠ, আইকন, ব্যৱধান আৰু বিভাজক
settings-appearance-theme-summary = ডেস্কটপৰ দৰেই, পোহৰ বা গাঢ়
settings-appearance-sender-pictures-summary = কোম্পানীৰ লোগো, প্ৰেৰকৰ ড'মেইনৰ দ্বাৰা বিচৰা
settings-appearance-important-summary = তালিকাত প্ৰতিটো বাৰ্তাৰ কাষত গুৰুত্বপূৰ্ণ চিহ্ন
settings-appearance-mail-colors-summary = গাঢ় থীমত HTML মেইলৰ বাবে গাঢ় ৰং, বা ইয়াৰ প্ৰেৰকৰ ৰং
settings-appearance-attachment-previews-summary = প্ৰতিটো সংলগ্নকৰ সমলৰ এখন সৰু ছবি
settings-shortcuts-set-summary = Gmail, Inbox by Gmail, Apple Mail, Outlook বা Thunderbirdৰ কীৰ পৰা আৰম্ভ কৰক
settings-shortcuts-single-summary = ৱেবমেইলৰ দৰে, Ctrl বা Alt নোহোৱা কী
settings-default-apps-pdf-summary = PDF সংলগ্নক ক'ত খোল খায়
settings-default-apps-pictures-summary = ফট' আৰু ছবি ক'ত খোল খায়
settings-default-apps-text-summary = সাধাৰণ পাঠ, লগ আৰু ক'ড ক'ত খোল খায়
settings-default-apps-sheets-summary = Excel, OpenDocument আৰু CSV ফাইল ক'ত খোল খায়
settings-default-apps-documents-summary = Word আৰু OpenDocument পাঠ ক'ত খোল খায়
settings-default-apps-after-saving-summary = ছেভ কৰা সংলগ্নক সেইবোৰৰ ফ'ল্ডাৰত দেখুৱাওক
settings-compose-send-from-summary = নতুন মেইল যি একাউণ্টৰ পৰা যায়: আপুনি থকাটো, বা সদায় একেটা
settings-compose-send-on-replies-summary = উত্তৰ আৰু ফৰৱাৰ্ডত পঠিয়াওক, বা পঠিয়াওক আৰু কথোপকথন আৰ্কাইভ কৰক
settings-compose-signatures-summary = আপোনাৰ বাৰ্তাৰ তলত, এটা “--” শাৰীৰ পিছত যোগ কৰা হয়
settings-compose-for-new-mail-summary = নতুন মেইল যি স্বাক্ষৰেৰে আৰম্ভ হয়
settings-compose-for-replies-summary = উত্তৰ আৰু ফৰৱাৰ্ড যি স্বাক্ষৰেৰে আৰম্ভ হয়
settings-compose-format-summary = নতুন মেইল সাধাৰণ পাঠত লিখক
settings-compose-spelling-summary = লিখি থাকোঁতে বানান পৰীক্ষা কৰক, আৰু অভিধানৰ ভাষা
settings-compose-templates-summary = সোনকালে আহিছে: আপুনি সঘনাই লিখা মেইল ছেভ কৰক, আৰু তাৰ পৰা নতুন মেইল বা উত্তৰ আৰম্ভ কৰক
settings-feedback-crash-reports-summary = Katna Mail বা ইয়াৰ নেপথ্য সেৱা ক্ৰেশ্ব হ'লে এই কম্পিউটাৰত ক্ৰেশ্ব ৰিপৰ্ট ছেভ কৰক
settings-feedback-saved-summary = এই কম্পিউটাৰত ছেভ কৰা ক্ৰেশ্ব ৰিপৰ্ট চাওক, কপি কৰক বা মচক
settings-feedback-help-improve-summary = কি ভুল হ'ল সেয়া ঠিক কৰাত সহায় কৰিবলৈ ক্ৰেশ্ব ৰিপৰ্ট পঠিয়াওক; আপুনি অন নকৰালৈকে অফ
settings-experimental-blur-summary = ওপৰৰ বাৰৰ মাজেৰে ডেস্কটপ অস্পষ্টকৈ দেখা যায়, আৰু মেনুবোৰ ঘঁহা কাঁচৰ দৰে দেখা যায়
settings-search-shortcut = কীব'ৰ্ড শ্বৰ্টকাট
settings-search-tab = ছেটিংছ টেব
settings-search-none = “{ $query }”ৰ সৈতে কোনো ছেটিং মিলা নাই।
settings-search-results = “{ $query }”ৰ সৈতে মিলা ছেটিংছ

## Quick settings (the panel that slides in from the right)

quick-title = দ্ৰুত ছেটিংছ
quick-see-all = সকলো ছেটিংছ চাওক
quick-reading-pane = পঢ়া পেন
quick-pane-right = তালিকাৰ সোঁফালে
quick-pane-none = বিভাজন নাই
quick-density = ঘনত্ব
quick-density-default = ডিফ'ল্ট
quick-density-compact = কম্পেক্ট
quick-theme = থীম
quick-theme-system = ডেস্কটপৰ দৰেই
quick-theme-light = পোহৰ
quick-theme-dark = গাঢ়
quick-desktop-colors = ডেস্কটপৰ ৰং
quick-desktop-colors-detail = ডেস্কটপৰ ৰঙৰ আঁচনি আৰু একচেণ্ট ৰং
quick-app-names = এপৰ নাম
quick-app-names-detail = একেবাৰে বাওঁফালৰ এপ আইকনৰ তলত নাম
quick-inbox-tabs = ইনবক্স টেব
quick-inbox-tabs-detail = প্ৰতিটো একাউণ্টৰ মেইল প্ৰদানকাৰীৰ টেব
quick-choose-tabs = টেব বাছনি কৰক
quick-choose-tabs-detail = প্ৰতিটো একাউণ্টৰ বাবে, ছেটিংছত
quick-sending = পঠোৱা
quick-undo-send = পঠোৱা আনডু কৰক
quick-undo-send-off = অফ
quick-undo-send-seconds = { $seconds } ছেকেণ্ড
quick-signatures = স্বাক্ষৰ
quick-signatures-none = এতিয়ালৈকে একো নাই
quick-signatures-one = { $name }, ডিফ'ল্ট হিচাপে ব্যৱহৃত
quick-signatures-many = { $count ->
    [one] { $count }টা স্বাক্ষৰ; ডিফ'ল্ট { $name }
   *[other] { $count }টা স্বাক্ষৰ; ডিফ'ল্ট { $name }
}
quick-signatures-no-default = { $count ->
    [one] { $count }টা, কোনো ডিফ'ল্ট নাই
   *[other] { $count }টা, কোনো ডিফ'ল্ট নাই
}
quick-signature-untitled = শিৰোনামহীন
quick-threading = ইমেইল থ্ৰেডিং
quick-conversation-view = কথোপকথন দৰ্শন
quick-conversation-view-detail = একেটা মেইলৰ উত্তৰসমূহ একেলগ কৰক
quick-help = সহায়
quick-tour = পৰিচিতি ভ্ৰমণ লওক
quick-whats-new = নতুন কি আছে
quick-about = Katnaৰ বিষয়ে

## Settings: opening at login

settings-open-at-login-failed = লগইনৰ সময়ত খোলাটো সলনি কৰিব পৰা নগ'ল: { $error }

## Settings > Appearance > Scaling

scale-letter = অ
scale-percent = { $percent }%
scale-reset = { $percent }%লৈ ঘূৰাই নিয়ক

## Settings > Experimental > Look & Feel

look-intro = এতিয়াও পৰীক্ষা কৰি থকা সুবিধা। এইবোৰ সলনি হ'ব পাৰে বা আঁতৰি যাব পাৰে।
look-heading = ৰূপ আৰু অনুভৱ
look-window-frame = ৱিণ্ড' ফ্ৰেম
look-window-frame-detail = শিৰোনাম বাৰ, ৱিণ্ড' বুটাম, চুক আৰু ছাঁ কোনে আঁকে।
look-frame-native-kde = নেটিভ: KDEৰ ফ্ৰেম, আপোনাৰ Plasma থীমত
look-frame-native = নেটিভ: ডেস্কটপৰ ফ্ৰেম
look-frame-katna = Katna: ওপৰৰ বাৰটো শিৰোনাম বাৰ হৈ পৰে
look-frame-katna-note-named = Katnaই ঘূৰণীয়া চুক আৰু নিজৰ ছাঁ আঁকে। ফ্ৰেমে আৰু { $desktop } থীম অনুসৰণ নকৰে; ৱিণ্ড' নিয়ম তথাপি প্ৰযোজ্য।
look-frame-katna-note = Katnaই ঘূৰণীয়া চুক আৰু নিজৰ ছাঁ আঁকে। ফ্ৰেমে আৰু ডেস্কটপ থীম অনুসৰণ নকৰে; ৱিণ্ড' নিয়ম তথাপি প্ৰযোজ্য।
look-frame-client-side = আপোনাৰ ডেস্কটপে ফ্ৰেমটো প্ৰতিটো এপৰ ওপৰত এৰি দিয়ে, সেয়ে Katnaই ইতিমধ্যে নিজৰ ফ্ৰেম আঁকে।
look-blurred-background = অস্পষ্ট পটভূমি
look-blurred-background-detail = ওপৰৰ বাৰ আৰু ফ'ল্ডাৰৰ মাজেৰে ডেস্কটপ অস্পষ্টকৈ দেখা যায়, আৰু মেনু আৰু পপঅভাৰবোৰ ঘঁহা কাঁচৰ দৰে দেখা যায়।
look-blur = ৱিণ্ড'ৰ পিছফালৰখিনি অস্পষ্ট কৰক
look-blur-detail = মেইল গোটা কাৰ্ডত থাকে, সেয়ে পাঠৰ কনট্ৰাষ্ট অটুট থাকে
look-blur-off-kde = KDEৰ ব্লাৰ প্ৰভাৱ অফ আছে। ছিষ্টেম ছেটিংছ, ৱিণ্ড' পৰিচালনা, ডেস্কটপ প্ৰভাৱত ব্লাৰ অন কৰক, তাৰ পিছত Katna Mail পুনৰ খোলক।
look-blur-none-gnome = GNOMEএ ৱিণ্ড'ৰ পিছফালৰখিনি অস্পষ্ট নকৰে।
look-blur-none-x11 = আপোনাৰ ৱিণ্ড' মেনেজাৰে ৱিণ্ড'ৰ পিছফালৰখিনি অস্পষ্ট নকৰে।
look-blur-none-wayland = আপোনাৰ কম্পজিটৰে ৱিণ্ড'ৰ পিছফালৰখিনি অস্পষ্ট নকৰে।

## Settings > User feedback (crash reports)

feedback-intro-sending = কি ভুল হ'ল সেয়া ঠিক কৰাত সহায় কৰিবলৈ নতুন ক্ৰেশ্ব ৰিপৰ্ট পঠোৱা হয়। আন একোৱেই এই কম্পিউটাৰৰ বাহিৰলৈ নাযায়।
feedback-intro-local = Katnaই ক'লৈকো একো নপঠিয়ায়। ক্ৰেশ্ব ৰিপৰ্ট এই কম্পিউটাৰতে থাকে, যাতে আপুনি সেইবোৰ চাব পাৰে বা কোনো বাগ ৰিপৰ্টৰ সৈতে সংলগ্ন কৰিব পাৰে।
feedback-crash-reports = ক্ৰেশ্ব ৰিপৰ্ট
feedback-crash-reports-detail = Katna Mail বা ইয়াৰ নেপথ্য সেৱা ক্ৰেশ্ব হ'লে লিখা হয়।
feedback-save = এই কম্পিউটাৰত ক্ৰেশ্ব ৰিপৰ্ট ছেভ কৰক
feedback-save-detail = আপোনাৰ হ'ম ফ'ল্ডাৰ, ব্যৱহাৰকাৰী আৰু কম্পিউটাৰৰ নাম আৰু ইমেইল ঠিকনা বাদ দিয়া হয়
feedback-saved = ছেভ কৰা ক্ৰেশ্ব ৰিপৰ্ট
feedback-saved-detail = { $count ->
    [one] আটাইতকৈ নতুন { $count }টা ৰখা হয়।
   *[other] আটাইতকৈ নতুন { $count }টা ৰখা হয়।
}
feedback-help-improve = Katna উন্নত কৰাত সহায় কৰক
feedback-help-improve-detail = আপুনি অন নকৰালৈকে অফ, আৰু আপুনি যিকোনো সময়তে ইয়াত ইয়াক অফ কৰিব পাৰে।
feedback-send = ক্ৰেশ্ব ৰিপৰ্ট পঠিয়াওক
feedback-send-detail = ছেভ কৰা ৰিপৰ্ট, আপুনি ইয়াত যিদৰে চাব পাৰে ঠিক তেনেকৈয়ে, Katnaৰ ক্ৰেশ্ব ট্ৰেকাৰলৈ (Sentry, EUত) যায়। কোনো IP ঠিকনা, বাৰ্তা বা ইমেইল ঠিকনা নহয়
feedback-none-saved = কোনো ক্ৰেশ্ব ৰিপৰ্ট ছেভ কৰা নাই।
feedback-delete-all = সকলো মচক
feedback-app-daemon = নেপথ্য সেৱা
feedback-report-sent = { $date } · পঠোৱা হ'ল
feedback-view = চাওক
feedback-view-tooltip = ৰিপৰ্ট খোলক
feedback-copy-tooltip = বাগ ৰিপৰ্টত পেষ্ট কৰিবলৈ ইয়াক কপি কৰক
feedback-copied = ক্ৰেশ্ব ৰিপৰ্ট কপি কৰা হ'ল।
feedback-deleted-all = ক্ৰেশ্ব ৰিপৰ্ট মচা হ'ল।
feedback-read-failed = ক্ৰেশ্ব ৰিপৰ্ট পঢ়িব পৰা নগ'ল: { $error }
feedback-delete-failed = ক্ৰেশ্ব ৰিপৰ্ট মচিব পৰা নগ'ল: { $error }
feedback-delete-all-failed = ক্ৰেশ্ব ৰিপৰ্টবোৰ মচিব পৰা নগ'ল: { $error }

## Menu bar (the KDE global menu)

desktop-menu-file = _ফাইল
desktop-menu-new-message = _নতুন বাৰ্তা
desktop-menu-quit = _প্ৰস্থান কৰক
desktop-menu-edit = _সম্পাদনা
desktop-menu-undo = _আনডু কৰক
desktop-menu-select-all = _সকলো বাছনি কৰক
desktop-menu-select-none = _একো বাছনি নকৰিব
desktop-menu-find = _বিচাৰক…
desktop-menu-view = _দৰ্শন
desktop-menu-folder-list = _ফ'ল্ডাৰ তালিকা দেখুৱাওক
desktop-menu-refresh = _ৰিফ্ৰেছ কৰক
desktop-menu-go = _যাওক
desktop-menu-inbox = _ইনবক্স
desktop-menu-starred = _তৰাচিহ্নিত
desktop-menu-sent = _প্ৰেৰিত
desktop-menu-drafts = _ড্ৰাফ্ট
desktop-menu-all-mail = _সকলো মেইল
desktop-menu-next = _পৰৱৰ্তী কথোপকথন
desktop-menu-previous = _পূৰ্বৱৰ্তী কথোপকথন
desktop-menu-message = _বাৰ্তা
desktop-menu-open = _খোলক
desktop-menu-reply = _উত্তৰ দিয়ক
desktop-menu-reply-all = _সকলোকে উত্তৰ দিয়ক
desktop-menu-forward = _ফৰৱাৰ্ড কৰক
desktop-menu-archive = _আৰ্কাইভ কৰক
desktop-menu-delete = _মচক
desktop-menu-spam = _স্পাম বুলি ৰিপৰ্ট কৰক
desktop-menu-move-to = _ইয়ালৈ স্থানান্তৰ কৰক…
desktop-menu-mark-read = _পঢ়া বুলি চিহ্নিত কৰক
desktop-menu-mark-unread = _নপঢ়া বুলি চিহ্নিত কৰক
desktop-menu-star = _তৰাচিহ্ন যোগ কৰক
desktop-menu-important = _গুৰুত্বপূৰ্ণ বুলি চিহ্নিত কৰক
desktop-menu-not-important = _গুৰুত্বপূৰ্ণ নহয় বুলি চিহ্নিত কৰক
desktop-menu-settings = _ছেটিংছ
desktop-menu-quick-settings = _দ্ৰুত ছেটিংছ
desktop-menu-configure = _Katna Mail কনফিগাৰ কৰক…
desktop-menu-help = _সহায়
desktop-menu-shortcuts = _কীব'ৰ্ড শ্বৰ্টকাট
desktop-menu-whats-new = _নতুন কি আছে
desktop-menu-about = _Katnaৰ বিষয়ে

## Settings > Keyboard shortcuts: the groups of the list

shortcut-group-moving = নেভিগেশ্বন
shortcut-group-actions = কাৰ্য
shortcut-group-go-to = ইয়ালৈ যাওক
shortcut-group-app = এপ্লিকেশ্বন

## Settings > Keyboard shortcuts: what each shortcut does

shortcut-next = পৰৱৰ্তী কথোপকথন
shortcut-previous = পূৰ্বৱৰ্তী কথোপকথন
shortcut-down = তালিকাত তললৈ যাওক
shortcut-up = তালিকাত ওপৰলৈ যাওক
shortcut-first = তালিকাৰ প্ৰথমটো
shortcut-last = তালিকাৰ শেষটো
shortcut-page-down = তালিকাত এপৃষ্ঠা তললৈ
shortcut-page-up = তালিকাত এপৃষ্ঠা ওপৰলৈ
shortcut-open = কথোপকথন খোলক
shortcut-back = তালিকালৈ উভতি যাওক
shortcut-scroll-down = তললৈ স্ক্ৰ'ল কৰক
shortcut-scroll-up = ওপৰলৈ স্ক্ৰ'ল কৰক
shortcut-scroll-page-down = এপৃষ্ঠা তললৈ স্ক্ৰ'ল কৰক
shortcut-scroll-page-up = এপৃষ্ঠা ওপৰলৈ স্ক্ৰ'ল কৰক
shortcut-compose = লিখক
shortcut-reply = উত্তৰ দিয়ক
shortcut-reply-all = সকলোকে উত্তৰ দিয়ক
shortcut-forward = ফৰৱাৰ্ড কৰক
shortcut-archive = আৰ্কাইভ কৰক
shortcut-delete = মচক
shortcut-spam = স্পাম বুলি ৰিপৰ্ট কৰক
shortcut-move-to = ইয়ালৈ স্থানান্তৰ কৰক
shortcut-mark-read = পঢ়া বুলি চিহ্নিত কৰক
shortcut-mark-unread = নপঢ়া বুলি চিহ্নিত কৰক
shortcut-star = তৰাচিহ্ন যোগ কৰক বা আঁতৰাওক
shortcut-important = গুৰুত্বপূৰ্ণ বুলি চিহ্নিত কৰক
shortcut-not-important = গুৰুত্বপূৰ্ণ নহয় বুলি চিহ্নিত কৰক
shortcut-check = কথোপকথনত টিক দিয়ক
shortcut-select-all = সকলো কথোপকথনত টিক দিয়ক
shortcut-select-none = সকলো কথোপকথনৰ টিক আঁতৰাওক
shortcut-undo = শেষ কাৰ্য আনডু কৰক
shortcut-go-inbox = ইনবক্স
shortcut-go-starred = তৰাচিহ্নিত
shortcut-go-sent = প্ৰেৰিত
shortcut-go-drafts = ড্ৰাফ্ট
shortcut-go-all = সকলো মেইল
shortcut-search = মেইল সন্ধান কৰক
shortcut-navigation = মেনু দেখুৱাওক বা সংকুচিত কৰক
shortcut-quick-settings = দ্ৰুত ছেটিংছ
shortcut-settings = সকলো ছেটিংছ
shortcut-shortcuts = কীব'ৰ্ড শ্বৰ্টকাট
shortcut-reload = নতুন মেইলৰ বাবে পৰীক্ষা কৰক
shortcut-quit = প্ৰস্থান কৰক

## Keys pressed one after another, as a shortcut shows them ("G then I")

shortcut-sequence = { $first } তাৰ পিছত { $second }

## Settings > Accounts

accounts-folder-pane = ফ'ল্ডাৰ পেন
accounts-folder-pane-detail = বাওঁফালৰ পেনে কোনবোৰ একাউণ্টৰ ফ'ল্ডাৰ দেখুৱায়।
accounts-shown-one = এবাৰত এটা একাউণ্ট; একাউণ্ট কাৰ্ডত সলনি কৰক
accounts-shown-all = সকলো একাউণ্ট, এটাৰ পিছত আনটো
accounts-row = একাউণ্টসমূহ
accounts-row-detail = একাউণ্ট আঁতৰালে এই কম্পিউটাৰত থকা ইয়াৰ মেইলৰ Katnaৰ কপি মচা হয়। মেইল ছাৰ্ভাৰত থাকে।
accounts-none = এতিয়ালৈকে কোনো একাউণ্ট নাই।
accounts-kind-imported = আমদানি কৰা
accounts-picture-reset = ডেস্কটপৰ ছবি ব্যৱহাৰ কৰক
accounts-picture-change = ছবি সলনি কৰক
accounts-remove = আঁতৰাওক
accounts-delete-all-row = সকলো ডেটা মচক
accounts-delete-all-row-detail = নতুন ইনষ্টলৰ দৰে, পুনৰ আৰম্ভ কৰক।
accounts-delete-all-about = এই কম্পিউটাৰৰ পৰা প্ৰতিটো একাউণ্ট, সকলো সঞ্চিত মেইল, সম্পৰ্ক আৰু কেলেণ্ডাৰ, সন্ধান ইনডেক্স, আপোনাৰ ছেটিংছ আৰু ছেভ কৰা পাছৱৰ্ড মচে। আপোনাৰ মেইল ছাৰ্ভাৰত একো সলনি নহয়।
accounts-delete-all-open = Katnaৰ সকলো ডেটা মচক

## Settings > Accounts: snackbars after deleting

accounts-removed-local = { $address } Katnaৰ পৰা আঁতৰোৱা হ'ল।
accounts-removed = { $address } Katnaৰ পৰা আঁতৰোৱা হ'ল। ইয়াৰ মেইল এতিয়াও ছাৰ্ভাৰত আছে।
accounts-all-deleted = Katnaৰ সকলো ডেটা এই কম্পিউটাৰৰ পৰা মচা হ'ল।

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = { $address } আঁতৰাব নেকি?
accounts-remove-confirm = একাউণ্ট আঁতৰাওক
accounts-removing = আঁতৰোৱা হৈছে…
accounts-remove-local-mail = { $folders ->
    [0] এই একাউণ্টলৈ আমদানি কৰা সকলো মেইল
    [one] এই একাউণ্টৰ ফ'ল্ডাৰটোলৈ আমদানি কৰা সকলো মেইল
   *[other] এই একাউণ্টৰ { $folders }টা ফ'ল্ডাৰলৈ আমদানি কৰা সকলো মেইল
}
accounts-remove-local-settings = ইয়াৰ Katna ছেটিংছ
accounts-remove-mail = { $folders ->
    [0] Katnaই সঞ্চয় কৰা এই একাউণ্টৰ সকলো মেইল
    [one] Katnaই ইয়াৰ ফ'ল্ডাৰটোত সঞ্চয় কৰা এই একাউণ্টৰ সকলো মেইল
   *[other] Katnaই ইয়াৰ { $folders }টা ফ'ল্ডাৰত সঞ্চয় কৰা এই একাউণ্টৰ সকলো মেইল
}
accounts-remove-outbox = আউটবক্সত অপেক্ষা কৰি থকা ইয়াৰ বাৰ্তা
accounts-remove-settings = ইয়াৰ ছেভ কৰা পাছৱৰ্ড আৰু ইয়াৰ Katna ছেটিংছ
accounts-delete-all-title = Katnaৰ সকলো ডেটা মচিব নেকি?
accounts-delete-all-confirm = সকলো মচক
accounts-deleting = মচা হৈছে…
accounts-delete-all-accounts = প্ৰতিটো একাউণ্ট, আৰু Katnaই সঞ্চয় কৰা সকলো মেইল আৰু সংলগ্নক
accounts-delete-all-contacts = সম্পৰ্ক, কেলেণ্ডাৰ আৰু সন্ধান ইনডেক্স
accounts-delete-all-settings = সকলো ছেটিংছ, স্বাক্ষৰ আৰু কীব'ৰ্ড শ্বৰ্টকাট
accounts-delete-all-passwords = প্ৰতিটো ছেভ কৰা পাছৱৰ্ড
accounts-deleted-heading = এই কম্পিউটাৰৰ পৰা মচা হ'ব:
accounts-cannot-undo = ইয়াক আনডু কৰিব নোৱাৰি।
accounts-server-delete-all = আপোনাৰ মেইল ছাৰ্ভাৰত একো সলনি নহয়: আপোনাৰ মেইল তাতেই থাকে, আৰু পুনৰ একাউণ্ট যোগ কৰিলে সেয়া পুনৰ ডাউনল'ড হয়। ফাইলৰ পৰা আমদানি কৰা মেইল কেৱল Katnaত আছে; ফাইলবোৰ চুই চোৱা নহয়।
accounts-server-local = এই মেইল ফাইলৰ পৰা আমদানি কৰা হৈছিল, সেয়ে একমাত্ৰ কপিটো Katnaৰ হাতত আছে। ই অহা ফাইলবোৰ চুই চোৱা নহয়; ঘূৰাই পাবলৈ সেইবোৰ পুনৰ আমদানি কৰক।
accounts-server-remove = মেইল ছাৰ্ভাৰত একো সলনি নহয়: আপোনাৰ মেইল তাতেই থাকে, আৰু পুনৰ একাউণ্ট যোগ কৰিলে সেয়া পুনৰ ডাউনল'ড হয়।
accounts-confirm-word = মচক
accounts-confirm-placeholder = “{ accounts-confirm-word }” টাইপ কৰক
accounts-confirm-prompt = নিশ্চিত কৰিবলৈ, “{ accounts-confirm-word }” টাইপ কৰক:
accounts-cancel = বাতিল কৰক
