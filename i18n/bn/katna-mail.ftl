# Katna Mail, Bengali (বাংলা).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Language picker (top bar and Settings > General)

language-tooltip = ভাষা: { $language }
language-tooltip-system = ভাষা: { $language }, সিস্টেম অনুযায়ী
language-search = ভাষা খুঁজুন
language-system-default = সিস্টেম ডিফল্ট
language-system-now = এখন { $language }
language-no-match = “{ $query }”-এর সাথে মেলে এমন কোনো ভাষা নেই
language-machine = মেশিনে অনুবাদ করা। উন্নত করতে সাহায্য করুন
language-setting = ভাষা
language-setting-detail = মেনু, বোতাম ও মেসেজের ভাষা, এবং তারিখ ও সংখ্যার ফর্ম্যাট। সিস্টেম ডিফল্ট ডেস্কটপের সেটিং অনুসরণ করে।

## Dates and sizes

ago-just-now = এইমাত্র
ago-minutes = { $count ->
    [one] { $count } মিনিট আগে
   *[other] { $count } মিনিট আগে
}
ago-hours = { $count ->
    [one] { $count } ঘণ্টা আগে
   *[other] { $count } ঘণ্টা আগে
}
ago-days = { $count ->
    [one] { $count } দিন আগে
   *[other] { $count } দিন আগে
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

folders-hide = ফোল্ডার লুকান
folders-show = ফোল্ডার দেখান
compose = লিখুন
search = খুঁজুন
search-mail = মেল খুঁজুন
search-settings = সেটিংস খুঁজুন
search-clear = সার্চ মুছুন
search-options-show = সার্চের বিকল্প দেখান
settings = সেটিংস
account-add = অ্যাকাউন্ট যোগ করুন

## App rail (and the bottom bar on a phone)

rail-mail = মেল
rail-calendar = ক্যালেন্ডার
rail-contacts = পরিচিতি
rail-tasks = টাস্ক
rail-notes = নোট
rail-feeds = ফিড

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = শীঘ্রই আসছে
app-calendar-promise = আপনার CalDAV ক্যালেন্ডার, মেলে আসা মিটিংয়ের আমন্ত্রণ আর রিমাইন্ডার, আপনার ইনবক্সের ঠিক পাশেই।
app-tasks-promise = CalDAV-এর সাথে সিঙ্ক হওয়া করণীয় তালিকা, আর মেল থেকে তৈরি টাস্ক।
app-notes-promise = চটজলদি নোট, আর পরে দেখার জন্য কোনো মেল বা কথোপকথনে নোট।
app-feeds-promise = মেলের পাশাপাশি RSS ও Atom ফিড পড়ুন।

## Contacts page

app-contacts-loading = আপনার মেল থেকে লোকজনকে একত্র করা হচ্ছে…
app-contacts-empty = আপনি যাঁদের সাথে মেল আদানপ্রদান করেন, তাঁরা এখানে দেখাবেন।
app-contacts-count = { $count ->
    [one] আপনার মেল থেকে { $count } জন, যাঁর সাথে সবচেয়ে বেশি মেল হয়েছে তিনি আগে
   *[other] আপনার মেল থেকে { $count } জন, যাঁদের সাথে সবচেয়ে বেশি মেল হয়েছে তাঁরা আগে
}
app-contacts-top = { $count ->
    [one] আপনার মেল থেকে শীর্ষ { $count } জন, যাঁর সাথে সবচেয়ে বেশি মেল হয়েছে তিনি আগে
   *[other] আপনার মেল থেকে শীর্ষ { $count } জন, যাঁদের সাথে সবচেয়ে বেশি মেল হয়েছে তাঁরা আগে
}
app-contacts-messages = { $count ->
    [one] { $count }টি মেসেজ
   *[other] { $count }টি মেসেজ
}
app-contacts-last = শেষবার { $date }

## Navigation (the folders pane)

nav-labels = লেবেল
nav-folders = ফোল্ডার
nav-label-new = নতুন লেবেল তৈরি করুন
nav-folder-new = নতুন ফোল্ডার তৈরি করুন
nav-account-unnamed = অ্যাকাউন্ট { $number }
nav-tab-new = { $count ->
    [one] { $count }টি নতুন
   *[other] { $count }টি নতুন
}

## Special folders (the user's own folders keep their names)

folder-inbox = ইনবক্স
folder-starred = তারকাচিহ্নিত
folder-drafts = খসড়া
folder-sent = পাঠানো হয়েছে
folder-archive = আর্কাইভ
folder-spam = স্প্যাম
folder-trash = ট্র্যাশ
folder-all-mail = সব মেল
folder-scheduled = শিডিউল করা

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = নতুন লেবেল
label-folder-new-title = নতুন ফোল্ডার
label-prompt = অনুগ্রহ করে নতুন লেবেলের নাম লিখুন:
label-folder-prompt = অনুগ্রহ করে নতুন ফোল্ডারের নাম লিখুন:
label-name-hint = লেবেলের নাম
label-folder-name-hint = ফোল্ডারের নাম
label-nest = লেবেলটি এর মধ্যে রাখুন:
label-folder-nest = ফোল্ডারটি এর মধ্যে রাখুন:
label-cancel = বাতিল করুন
label-create = তৈরি করুন
label-creating = তৈরি করা হচ্ছে…
label-created = “{ $name }” লেবেল তৈরি করা হয়েছে।
label-folder-created = “{ $name }” ফোল্ডার তৈরি করা হয়েছে।

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = প্রাথমিক
tab-promotions = প্রচার
tab-social = সামাজিক
tab-updates = আপডেট
tab-forums = ফোরাম
tab-focused = ফোকাসড
tab-other = অন্যান্য
tab-inbox = ইনবক্স
tab-newsletters = নিউজলেটার
tab-notifications = বিজ্ঞপ্তি
tab-new = { $count }টি নতুন
tab-provider-other = Katna সাজিয়েছে

## Mail list: toolbar

list-select = বেছে নিন
list-refresh = রিফ্রেশ করুন
list-more = আরও
list-mark-read = পঠিত হিসেবে চিহ্নিত করুন
list-mark-unread = অপঠিত হিসেবে চিহ্নিত করুন
list-move-to = এখানে সরান
list-archive = আর্কাইভ করুন
list-spam = স্প্যাম হিসেবে রিপোর্ট করুন
list-delete = মুছুন
list-newer = নতুন
list-older = পুরনো
list-range = { $total }টির মধ্যে { $first }–{ $last }
list-range-about = প্রায় { $total }টির মধ্যে { $first }–{ $last }
list-results = “{ $query }”-এর ফলাফল
list-results-corrected = “{ $query }”-এর ফলাফল দেখানো হচ্ছে
list-search-instead = এর বদলে “{ $query }” খুঁজুন
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = সব
list-pick-none = কোনোটিই নয়
list-pick-read = পঠিত
list-pick-unread = অপঠিত
list-pick-starred = তারকাচিহ্নিত
list-pick-unstarred = তারকাচিহ্নহীন

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] { $count }টি কথোপকথন বেছে নেওয়া হয়েছে।
       *[other] সবকটি { $count }টি কথোপকথন বেছে নেওয়া হয়েছে।
    }
   *[message] { $count ->
        [one] { $count }টি মেসেজ বেছে নেওয়া হয়েছে।
       *[other] সবকটি { $count }টি মেসেজ বেছে নেওয়া হয়েছে।
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder }-এর { $count }টি কথোপকথন বেছে নেওয়া হয়েছে।
       *[other] { $folder }-এর সবকটি { $count }টি কথোপকথন বেছে নেওয়া হয়েছে।
    }
   *[message] { $count ->
        [one] { $folder }-এর { $count }টি মেসেজ বেছে নেওয়া হয়েছে।
       *[other] { $folder }-এর সবকটি { $count }টি মেসেজ বেছে নেওয়া হয়েছে।
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] স্ক্রিনের { $count }টি কথোপকথন বেছে নেওয়া হয়েছে।
       *[other] স্ক্রিনের সবকটি { $count }টি কথোপকথন বেছে নেওয়া হয়েছে।
    }
   *[message] { $count ->
        [one] স্ক্রিনের { $count }টি মেসেজ বেছে নেওয়া হয়েছে।
       *[other] স্ক্রিনের সবকটি { $count }টি মেসেজ বেছে নেওয়া হয়েছে।
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] { $count }টি কথোপকথন বেছে নিন
       *[other] সবকটি { $count }টি কথোপকথন বেছে নিন
    }
   *[message] { $count ->
        [one] { $count }টি মেসেজ বেছে নিন
       *[other] সবকটি { $count }টি মেসেজ বেছে নিন
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder }-এর { $count }টি কথোপকথন বেছে নিন
       *[other] { $folder }-এর সবকটি { $count }টি কথোপকথন বেছে নিন
    }
   *[message] { $count ->
        [one] { $folder }-এর { $count }টি মেসেজ বেছে নিন
       *[other] { $folder }-এর সবকটি { $count }টি মেসেজ বেছে নিন
    }
}
list-clear-selection = বাছাই মুছুন

## Mail list: empty states

list-empty-search = আপনার সার্চের সাথে কোনো মেসেজ মেলেনি।
list-empty-tab = { $tab }-এ কোনো মেল নেই।
list-empty-tab-unknown = এই ট্যাবে কোনো মেল নেই।
list-empty-folder = { $folder }-এ কোনো মেসেজ নেই।
list-empty-folder-unknown = এই ফোল্ডারে কোনো মেসেজ নেই।
list-first-sync = আপনার মেল আনা হচ্ছে…
list-first-sync-detail = মেল আসার সাথে সাথে এখানে দেখাবে।

## Mail list: lines

row-removed = এই মেসেজটি সরিয়ে দেওয়া হয়েছে।
row-starred = তারকাচিহ্নিত
row-not-starred = তারকাচিহ্নিত নয়
row-important = গুরুত্বপূর্ণ। গুরুত্বপূর্ণ নয় হিসেবে চিহ্নিত করতে ক্লিক করুন।
row-mark-important = গুরুত্বপূর্ণ হিসেবে চিহ্নিত করুন
row-pinned = উপরে পিন করা
row-pin = উপরে পিন করুন
row-unpin = আনপিন করুন

## Mail list: More menu and right-click menu

menu-reply = উত্তর দিন
menu-reply-all = সবাইকে উত্তর দিন
menu-forward = ফরোয়ার্ড করুন
menu-archive = আর্কাইভ করুন
menu-delete = মুছুন
menu-spam = স্প্যাম হিসেবে রিপোর্ট করুন
menu-mark-read = পঠিত হিসেবে চিহ্নিত করুন
menu-mark-unread = অপঠিত হিসেবে চিহ্নিত করুন
menu-mark-all-read = সবগুলি পঠিত হিসেবে চিহ্নিত করুন
menu-star = তারকাচিহ্ন দিন
menu-unstar = তারকাচিহ্ন সরান
menu-important = গুরুত্বপূর্ণ হিসেবে চিহ্নিত করুন
menu-not-important = গুরুত্বপূর্ণ নয় হিসেবে চিহ্নিত করুন
menu-pin = উপরে পিন করুন
menu-unpin = আনপিন করুন
menu-print-all = সব প্রিন্ট করুন
menu-new-window = নতুন উইন্ডোতে খুলুন
menu-move-to = এখানে সরান
menu-move-to-heading = এখানে সরান:
menu-find-from = { $name }-এর পাঠানো ইমেল খুঁজুন

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথন আর্কাইভ করা হয়েছে।
       *[other] { $count }টি কথোপকথন আর্কাইভ করা হয়েছে।
    }
   *[message] { $count ->
        [one] মেসেজ আর্কাইভ করা হয়েছে।
       *[other] { $count }টি মেসেজ আর্কাইভ করা হয়েছে।
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথন ট্র্যাশে সরানো হয়েছে।
       *[other] { $count }টি কথোপকথন ট্র্যাশে সরানো হয়েছে।
    }
   *[message] { $count ->
        [one] মেসেজ ট্র্যাশে সরানো হয়েছে।
       *[other] { $count }টি মেসেজ ট্র্যাশে সরানো হয়েছে।
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথন সরানো হয়েছে।
       *[other] { $count }টি কথোপকথন সরানো হয়েছে।
    }
   *[message] { $count ->
        [one] মেসেজ সরানো হয়েছে।
       *[other] { $count }টি মেসেজ সরানো হয়েছে।
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথনে তারকাচিহ্ন দেওয়া হয়েছে।
       *[other] { $count }টি কথোপকথনে তারকাচিহ্ন দেওয়া হয়েছে।
    }
   *[message] { $count ->
        [one] মেসেজে তারকাচিহ্ন দেওয়া হয়েছে।
       *[other] { $count }টি মেসেজে তারকাচিহ্ন দেওয়া হয়েছে।
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথন থেকে তারকাচিহ্ন সরানো হয়েছে।
       *[other] { $count }টি কথোপকথন থেকে তারকাচিহ্ন সরানো হয়েছে।
    }
   *[message] { $count ->
        [one] মেসেজ থেকে তারকাচিহ্ন সরানো হয়েছে।
       *[other] { $count }টি মেসেজ থেকে তারকাচিহ্ন সরানো হয়েছে।
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথন গুরুত্বপূর্ণ হিসেবে চিহ্নিত করা হয়েছে।
       *[other] { $count }টি কথোপকথন গুরুত্বপূর্ণ হিসেবে চিহ্নিত করা হয়েছে।
    }
   *[message] { $count ->
        [one] মেসেজ গুরুত্বপূর্ণ হিসেবে চিহ্নিত করা হয়েছে।
       *[other] { $count }টি মেসেজ গুরুত্বপূর্ণ হিসেবে চিহ্নিত করা হয়েছে।
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথন গুরুত্বপূর্ণ নয় হিসেবে চিহ্নিত করা হয়েছে।
       *[other] { $count }টি কথোপকথন গুরুত্বপূর্ণ নয় হিসেবে চিহ্নিত করা হয়েছে।
    }
   *[message] { $count ->
        [one] মেসেজ গুরুত্বপূর্ণ নয় হিসেবে চিহ্নিত করা হয়েছে।
       *[other] { $count }টি মেসেজ গুরুত্বপূর্ণ নয় হিসেবে চিহ্নিত করা হয়েছে।
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথন উপরে পিন করা হয়েছে।
       *[other] { $count }টি কথোপকথন উপরে পিন করা হয়েছে।
    }
   *[message] { $count ->
        [one] মেসেজ উপরে পিন করা হয়েছে।
       *[other] { $count }টি মেসেজ উপরে পিন করা হয়েছে।
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথন আনপিন করা হয়েছে।
       *[other] { $count }টি কথোপকথন আনপিন করা হয়েছে।
    }
   *[message] { $count ->
        [one] মেসেজ আনপিন করা হয়েছে।
       *[other] { $count }টি মেসেজ আনপিন করা হয়েছে।
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথন স্প্যাম হিসেবে রিপোর্ট করা হয়েছে।
       *[other] { $count }টি কথোপকথন স্প্যাম হিসেবে রিপোর্ট করা হয়েছে।
    }
   *[message] { $count ->
        [one] মেসেজ স্প্যাম হিসেবে রিপোর্ট করা হয়েছে।
       *[other] { $count }টি মেসেজ স্প্যাম হিসেবে রিপোর্ট করা হয়েছে।
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথন চিরতরে মুছে ফেলা হয়েছে।
       *[other] { $count }টি কথোপকথন চিরতরে মুছে ফেলা হয়েছে।
    }
   *[message] { $count ->
        [one] মেসেজ চিরতরে মুছে ফেলা হয়েছে।
       *[other] { $count }টি মেসেজ চিরতরে মুছে ফেলা হয়েছে।
    }
}
toast-undone = কাজটি পূর্বাবস্থায় ফেরানো হয়েছে।
toast-undo = পূর্বাবস্থায় ফেরান
toast-no-spam-folder = এই অ্যাকাউন্টে কোনো স্প্যাম ফোল্ডার নেই।

## Reading pane: toolbar

reader-close = বন্ধ করুন
reader-back = ফিরে যান
reader-mark-unread = অপঠিত হিসেবে চিহ্নিত করুন
reader-move-to = এখানে সরান
reader-more = আরও
reader-print-all = সব প্রিন্ট করুন
reader-new-window = নতুন উইন্ডোতে
reader-position = { $total }টির মধ্যে { $position }
reader-newer = নতুন
reader-older = পুরনো

## Reading pane: the conversation

reader-removed = এই কথোপকথনটি সরিয়ে দেওয়া হয়েছে।
reader-no-subject = (কোনো বিষয় নেই)
reader-collapse-all = সব সঙ্কুচিত করুন
reader-expand-all = সব প্রসারিত করুন
reader-unknown-sender = (অজানা প্রেরক)
reader-date-ago = { $date } ({ $ago })
reader-me = আমাকে
reader-to = প্রাপক: { $names }
reader-starred = তারকাচিহ্নিত
reader-not-starred = তারকাচিহ্নিত নয়
reader-too-long = মেসেজটি এত বড় যে পুরোটা দেখানো যাচ্ছে না।
reader-encrypted-images = এনক্রিপ্ট করা মেলে ওয়েব থেকে ছবি কখনো লোড করা হয় না।
reader-window-failed = নতুন উইন্ডো খোলা যায়নি।

## Reading pane: message details (opened from "to me")

reader-details-from = প্রেরক:
reader-details-to = প্রাপক:
reader-details-cc = cc:
reader-details-date = তারিখ:
reader-details-subject = বিষয়:

## Reading pane: downloading a message

reader-downloading = সার্ভার থেকে এই মেসেজটি ডাউনলোড করা হচ্ছে…
reader-download-failed = এই মেসেজটি ডাউনলোড করা যায়নি।
reader-try-again = আবার চেষ্টা করুন

## Reply row

reply-reply = উত্তর দিন
reply-reply-all = সবাইকে উত্তর দিন
reply-forward = ফরোয়ার্ড করুন

## Encrypted and signed mail

security-decrypting = ডিক্রিপ্ট করা হচ্ছে…
security-checking = স্বাক্ষর যাচাই করা হচ্ছে…
security-partly-encrypted = এই মেসেজের শুধু একটি অংশ এনক্রিপ্ট করা। বাকি অংশ সুরক্ষার বাইরে যোগ করা হয়েছে এবং যে কেউ পাঠিয়ে থাকতে পারে।
security-partly-signed = এই মেসেজের শুধু একটি অংশে স্বাক্ষর আছে। বাকি অংশ সুরক্ষার বাইরে যোগ করা হয়েছে এবং যে কেউ পাঠিয়ে থাকতে পারে।
security-encrypted = এনক্রিপ্ট করা মেসেজ
security-encrypted-smime = এনক্রিপ্ট করা মেসেজ (S/MIME)
security-no-key = এই মেসেজ ডিক্রিপ্ট করা যাচ্ছে না: এটি এমন একটি কী-এর জন্য এনক্রিপ্ট করা হয়েছে যা আপনার কাছে নেই।
security-cancelled = ডিক্রিপ্ট করা বাতিল করা হয়েছে।
security-damaged = এই মেসেজ ডিক্রিপ্ট করা যাচ্ছে না: এনক্রিপ্ট করা ডেটা ক্ষতিগ্রস্ত বা পরিবর্তিত হয়েছে।
security-decrypt-unavailable = এই মেসেজ ডিক্রিপ্ট করা যাচ্ছে না: এনক্রিপ্ট করা মেল পড়তে { $tool } ইনস্টল করুন।
security-decrypt-failed = এই মেসেজ ডিক্রিপ্ট করা যাচ্ছে না: { $reason }
security-unknown-signer = অজানা স্বাক্ষরকারী
security-signed-verified = স্বাক্ষর করেছেন { $signer } · যাচাই করা
security-signed-not-sender = স্বাক্ষর করেছেন { $signer }, যিনি প্রেরক নন
security-signed-untrusted = স্বাক্ষর করেছেন { $signer }, এমন একটি কী দিয়ে যা আপনি অবিশ্বস্ত হিসেবে চিহ্নিত করেছেন
security-signed-unverified = স্বাক্ষর করেছেন { $signer } · কী যাচাই করা নেই
security-bad-signature = ভুল স্বাক্ষর: স্বাক্ষরের পরে এই মেসেজ বদলানো হয়েছে, অথবা স্বাক্ষরটি জাল।
security-signature-expired = স্বাক্ষর করেছেন { $signer } · স্বাক্ষরের মেয়াদ শেষ হয়ে গেছে
security-key-expired = স্বাক্ষর করেছেন { $signer } · এরপর কী-এর মেয়াদ শেষ হয়ে গেছে
security-key-revoked = স্বাক্ষর করেছেন { $signer }, এমন একটি কী দিয়ে যা প্রত্যাহার করা হয়েছে
security-missing-key = এমন একটি কী দিয়ে স্বাক্ষর করা যা আপনার কাছে নেই, তাই যাচাই করা যাচ্ছে না
security-missing-key-id = এমন একটি কী ({ $key }) দিয়ে স্বাক্ষর করা যা আপনার কাছে নেই, তাই যাচাই করা যাচ্ছে না
security-signature-unavailable = স্বাক্ষরিত; স্বাক্ষর যাচাই করতে { $tool } ইনস্টল করুন
security-signature-error = স্বাক্ষর যাচাই করা যায়নি।

## Remote images and pictures

remote-hidden = এই মেসেজের ছবিগুলি লুকানো আছে।
remote-show = ছবি দেখান
remote-always-show = এই প্রেরকের ছবি সবসময় দেখান
remote-picture-use = ব্যবহার করুন
remote-picture-too-big = 8 MB বা তার কম আকারের ছবি বেছে নিন।
remote-picture-type = PNG, JPEG, GIF, WebP বা SVG ছবি বেছে নিন।
remote-picture-read-failed = ছবিটি পড়া যাচ্ছে না: { $error }
remote-picture-keep-failed = ছবিটি রাখা যাচ্ছে না: { $error }
remote-picture-remove-failed = ছবিটি সরানো যাচ্ছে না: { $error }

## Attachments

attachment-count = { $count ->
    [one] একটি অ্যাটাচমেন্ট
   *[other] { $count }টি অ্যাটাচমেন্ট
}
attachment-save = সেভ করুন
attachment-save-all = সব সেভ করুন
attachment-save-all-tooltip = সব অ্যাটাচমেন্ট একটি ফোল্ডারে সেভ করুন
attachment-save-here = এখানে সেভ করুন
attachment-not-downloaded = এই মেসেজটি ডাউনলোড করা হয়নি।
attachment-not-found = মেসেজে এই অ্যাটাচমেন্টটি পাওয়া যায়নি।
attachment-read-failed = { $name } পড়া যায়নি
attachment-numbered = অ্যাটাচমেন্ট { $number }
attachment-saved-all = { $count ->
    [one] { $place }-এ { $count }টি ফাইল সেভ করা হয়েছে
   *[other] { $place }-এ { $count }টি ফাইল সেভ করা হয়েছে
}
attachment-saved-some = { $total ->
    [one] { $place }-এ { $total }টির মধ্যে { $saved }টি ফাইল সেভ করা হয়েছে। { $failed } সেভ করা যায়নি
   *[other] { $place }-এ { $total }টির মধ্যে { $saved }টি ফাইল সেভ করা হয়েছে। { $failed } সেভ করা যায়নি
}
attachment-saved-to = { $path }-এ সেভ করা হয়েছে
attachment-save-failed = { $name } সেভ করা যায়নি: { $error }
attachment-open-failed = { $name } খোলা যায়নি: { $error }
attachment-risky = এই ফাইলটি কোনো প্রোগ্রাম চালাতে পারে, তাই Katna এটি খোলে না। এর বদলে এটি সেভ করুন।
attachment-encrypted-open = এই ফাইলটি এনক্রিপ্ট করা অবস্থায় এসেছে। অন্য কোথাও খুলতে এটি সেভ করুন।

## Printing

print-failed = প্রিন্ট করা যায়নি: { $error }
print-no-font = কোনো ফন্ট পাওয়া যায়নি
print-opened-as-pdf = PDF হিসেবে খোলা হয়েছে, সেখান থেকে প্রিন্ট করুন।
print-not-downloaded = (এখনও ডাউনলোড করা হয়নি।)
print-encrypted = (এনক্রিপ্ট করা। এর লেখা প্রিন্ট করতে Katna Mail-এ খুলুন।)
print-to = প্রাপক: { $addresses }
print-cc = Cc: { $addresses }

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = অ্যাটাচমেন্টগুলি পড়তে এই মেসেজটি খুলুন।
text-copy = কপি করুন
text-select-all = সব বেছে নিন

## Settings page: its tabs

settings-tab-general = সাধারণ
settings-tab-inbox = ইনবক্স
settings-tab-accounts = অ্যাকাউন্ট
settings-tab-subscriptions = সাবস্ক্রিপশন
settings-tab-appearance = চেহারা
settings-tab-shortcuts = শর্টকাট
settings-tab-default-apps = ডিফল্ট অ্যাপ
settings-tab-folders-rules = ফোল্ডার ও নিয়ম
settings-tab-compose = লিখুন
settings-tab-mcp-server = MCP সার্ভার
settings-tab-feedback = ব্যবহারকারীর মতামত
settings-tab-experimental = পরীক্ষামূলক

## Settings page: tabs still to come

settings-tab-subscriptions-coming = আপনি যেসব নিউজলেটার ও মেলিং লিস্ট পান সেগুলি দেখুন, আর এক ক্লিকে আনসাবস্ক্রাইব করুন।
settings-tab-folders-rules-coming = ফোল্ডার ও লেবেল তৈরি করুন, নাম বদলান, সরান ও লুকান, এবং কোনগুলি সিঙ্ক হবে তা বেছে নিন। নিয়মগুলি প্রেরক, বিষয় বা শব্দ অনুযায়ী নতুন মেল নিজে থেকেই সাজায়, লেবেল দেয়, ফরোয়ার্ড করে বা মুছে দেয়।
settings-tab-mcp-server-coming = এই কম্পিউটারের AI সহকারীদের আপনার অনুমতি নিয়ে আপনার মেল খুঁজতে, পড়তে ও খসড়া লিখতে দিন।

## Settings > General

settings-general-conversations = কথোপকথন ভিউ
settings-general-conversations-group = একই মেলের উত্তরগুলি একসাথে রাখুন
settings-general-conversations-group-detail = তালিকায় প্রতিটি কথোপকথনের জন্য একটি লাইন
settings-general-reading = পড়া
settings-general-newest-first = সবচেয়ে নতুন মেসেজ আগে
settings-general-newest-first-detail = কথোপকথন তার সবচেয়ে নতুন উত্তর দিয়ে শুরু হয়
settings-general-full-headers = সম্পূর্ণ হেডার দেখান
settings-general-full-headers-detail = প্রতিটি মেসেজে প্রেরক, প্রাপক, cc, তারিখ ও বিষয় খোলা থাকে
settings-general-full-names = প্রাপকদের পুরো নাম
settings-general-full-names-detail = “প্রাপক: আমাকে, Ada”-এর বদলে “প্রাপক: আমাকে, Ada Lovelace”
settings-general-mark-read = পঠিত হিসেবে চিহ্নিত করুন
settings-general-mark-read-now = খোলার সাথে সাথেই
settings-general-mark-read-1s = 1 সেকেন্ড খোলা থাকার পরে
settings-general-mark-read-3s = 3 সেকেন্ড খোলা থাকার পরে
settings-general-mark-read-never = শুধু যখন আমি পঠিত হিসেবে চিহ্নিত করি
settings-general-reply-button = উত্তর দেওয়ার বোতাম
settings-general-reply-all = সবাইকে উত্তর দিন
settings-general-reply-all-detail = প্রতিটি মেসেজের পাশের উত্তর বোতাম শুধু প্রেরককে নয়, সবাইকে উত্তর দেয়
settings-general-remote-images = ওয়েব থেকে ছবি
settings-general-remote-images-detail = কোনো মেসেজের ছবি লোড করলে তার প্রেরক জেনে যান যে আপনি সেটি খুলেছেন, কখন খুলেছেন এবং মোটামুটি কোথা থেকে। বন্ধ থাকলে প্রতিটি মেসেজ আগে জিজ্ঞাসা করে, আর আপনি যেকোনো সময় কোনো প্রেরকের ছবি দেখাতে পারেন।
settings-general-remote-images-always = সবসময় ছবি দেখান
settings-general-remote-images-always-detail = প্রতিটি মেসেজে, শুধু বিশ্বস্ত প্রেরকদের মেসেজে নয়
settings-general-sending = পাঠানো
settings-general-sending-detail = পাঠানো মেসেজ কতক্ষণ অপেক্ষা করবে, যাতে সেটি ফিরিয়ে নেওয়া যায়।
settings-general-offline = অফলাইন মেল
settings-general-offline-detail = সাম্প্রতিক মেল পুরোপুরি ডাউনলোড করা হয়, যাতে কানেকশন ছাড়াই পড়া যায়। পুরনো মেল খুললে তখন ডাউনলোড হয়।
settings-general-offline-days = { $count ->
    [one] { $count } দিন
   *[other] { $count } দিন
}
settings-general-offline-years = { $count ->
    [one] { $count } বছর
   *[other] { $count } বছর
}
settings-general-offline-all = সব মেল
settings-general-offline-note = কম দিন বেছে নিলে আগে ডাউনলোড করা মেল থেকে যায়। সার্ভারে কিছুই বদলায় না।
settings-general-notifications = বিজ্ঞপ্তি
settings-general-notifications-detail = ইনবক্সে নতুন মেলের জন্য, Katna Mail বন্ধ থাকলেও।
settings-general-new-mail = নতুন মেলের বিজ্ঞপ্তি দিন
settings-general-new-mail-detail = সবাইকে উত্তর দিন, পঠিত হিসেবে চিহ্নিত করুন এবং আর্কাইভ করুন বোতাম সহ
settings-general-new-mail-sound = শব্দ বাজান
settings-general-new-mail-sound-detail = ডেস্কটপের নতুন মেলের শব্দ
settings-general-desktop = ডেস্কটপ
settings-general-open-at-login = লগ ইন করলে Katna Mail খুলুন
settings-general-open-at-login-detail = পরিষেবা চালু থাকলে লগ ইনের সময় মেল এমনিতেই সিঙ্ক হয়
settings-general-tray = সিস্টেম ট্রে-তে Katna দেখান
settings-general-tray-detail = অপঠিত সংখ্যা ও একটি মেনু সহ
settings-general-unread-badge = টাস্কবারের আইকনে অপঠিত সংখ্যা
settings-general-unread-badge-detail = ইনবক্সের কতগুলি মেসেজ অপঠিত

## Settings > Inbox

settings-inbox-tabs = ইনবক্স ট্যাব
settings-inbox-tabs-detail = ইনবক্সকে ট্যাবে ভাগ করুন, যেমন আপনার মেল প্রদানকারীর ওয়েবসাইট করে।
settings-inbox-tabs-show = ইনবক্স ট্যাব দেখান
settings-inbox-tabs-show-detail = বন্ধ থাকলে প্রতিটি অ্যাকাউন্টের জন্য একটি তালিকা দেখায়
settings-inbox-no-accounts = ট্যাব বেছে নিতে একটি অ্যাকাউন্ট যোগ করুন।
settings-inbox-tabs-automatic = স্বয়ংক্রিয়: { $tabs } ({ $provider })
settings-inbox-tabs-off = কোনো ট্যাব নেই
settings-inbox-tabs-gmail = প্রাথমিক, প্রচার, সামাজিক, আপডেট, ফোরাম
settings-inbox-tabs-focused = ফোকাসড ও অন্যান্য
settings-inbox-tabs-zoho = ইনবক্স, নিউজলেটার ও বিজ্ঞপ্তি
settings-inbox-tabs-shown = দেখানো ট্যাব। যে ট্যাব আপনি বন্ধ করেন, তার মেল { $tab }-এ থাকে।

## Settings > Appearance

settings-appearance-reading-pane = রিডিং প্যান
settings-appearance-reading-pane-detail = খোলা কথোপকথন কোথায় দেখাবে।
settings-appearance-pane-right = তালিকার ডানদিকে
settings-appearance-pane-none = কোনো বিভাজন নেই
settings-appearance-density = ঘনত্ব
settings-appearance-density-default = ডিফল্ট
settings-appearance-density-compact = কমপ্যাক্ট
settings-appearance-scaling = স্কেলিং
settings-appearance-scaling-detail = ডেস্কটপের নিজস্ব স্কেলের উপরে Katna Mail-এর সবকিছু বড় বা ছোট করে: লেখা, আইকন, ফাঁকা জায়গা ও বিভাজক। আপনার পাঠানো মেলের ফন্টের আকার একই থাকে। খুব ছোট আকারে আইকনে ক্লিক করা কঠিন হতে পারে।
settings-appearance-theme = থিম
settings-appearance-theme-system = ডেস্কটপের মতো
settings-appearance-theme-light = লাইট
settings-appearance-theme-dark = ডার্ক
settings-appearance-desktop-colors = ডেস্কটপের রং
settings-appearance-desktop-colors-use = ডেস্কটপের রং ব্যবহার করুন
settings-appearance-desktop-colors-use-detail = ডেস্কটপের কালার স্কিম ও অ্যাকসেন্ট কালার
settings-appearance-app-names = অ্যাপের নাম
settings-appearance-app-names-show = অ্যাপের নাম দেখান
settings-appearance-app-names-show-detail = একেবারে বাঁদিকে অ্যাপ আইকনের নিচে নাম
settings-appearance-sender-pictures = প্রেরকের ছবি
settings-appearance-sender-pictures-show = কোম্পানির লোগো দেখান
settings-appearance-sender-pictures-show-detail = প্রেরকের ডোমেন দিয়ে খোঁজা হয়, কখনো মেসেজ দিয়ে নয়, এবং এক সপ্তাহ রাখা হয়
settings-appearance-important = গুরুত্বপূর্ণ চিহ্ন
settings-appearance-important-show = গুরুত্বপূর্ণ চিহ্ন দেখান
settings-appearance-important-show-detail = তালিকায় প্রতিটি মেসেজের পাশে
settings-appearance-message-width = মেসেজের প্রস্থ
settings-appearance-message-width-limit = মেসেজের প্রস্থ সীমিত করুন
settings-appearance-message-width-limit-detail = চওড়া উইন্ডোতে লম্বা লাইন পড়া সহজ হয়
settings-appearance-mail-colors = মেলের রং
settings-appearance-mail-colors-detail = বেশিরভাগ মেল সাদা পাতার জন্য ডিজাইন করা। ডার্ক থিমে এর রং বদলে এমন গাঢ় রং করা হয় যা সহজে পড়া যায়; বন্ধ থাকলে মেল হালকা পাতায় প্রেরকের রংই রাখে।
settings-appearance-dark-mail = মেলের জন্যও গাঢ় রং
settings-appearance-dark-mail-detail = শুধু যখন থিম ডার্ক থাকে
settings-appearance-attachment-previews = অ্যাটাচমেন্টের প্রিভিউ
settings-appearance-attachment-previews-show = অ্যাটাচমেন্টের প্রিভিউ দেখান
settings-appearance-attachment-previews-show-detail = প্রতিটি ফাইলের কার্ডে তার বিষয়বস্তুর একটি ছোট ছবি

## Settings > Default apps

settings-default-apps-intro = ক্লিক করলে অ্যাটাচমেন্ট কোথায় খুলবে। ভিউয়ার সবসময় ফাইলটি অন্য অ্যাপেও খুলতে পারে। ডেস্কটপের ডিফল্ট অ্যাপগুলি তার নিজের সেটিংসে ঠিক করা হয়।
settings-default-apps-pdf = PDF ফাইল
settings-default-apps-pdf-detail = পাতা, জুম সহ।
settings-default-apps-pictures = ছবি
settings-default-apps-pictures-detail = ফটো (সোজা করে ঘোরানো), PNG, GIF, WebP, BMP, TIFF ও SVG।
settings-default-apps-text = টেক্সট ফাইল
settings-default-apps-text-detail = সাধারণ টেক্সট, লগ, কোড ও অন্যান্য টেক্সট।
settings-default-apps-sheets = স্প্রেডশিট
settings-default-apps-sheets-detail = Excel (xlsx, xls), OpenDocument (ods) ও CSV।
settings-default-apps-documents = ডকুমেন্ট
settings-default-apps-documents-detail = Word (docx) ও OpenDocument টেক্সট (odt)।
settings-default-apps-katna = Katna Mail-এর ভিউয়ার
settings-default-apps-system = ডেস্কটপের ডিফল্ট অ্যাপ
settings-default-apps-ask = প্রতিবার জিজ্ঞাসা করুন কোন অ্যাপ
settings-default-apps-after-saving = সেভ করার পরে
settings-default-apps-show-folder = সেভ করা ফাইল তাদের ফোল্ডারে দেখান
settings-default-apps-show-folder-detail = ফাইল ম্যানেজার খোলে, সেভ করা অ্যাটাচমেন্টগুলি বাছাই করা অবস্থায়

## Settings > Compose

settings-compose-send-from = নতুন মেসেজ যেখান থেকে পাঠানো হবে
settings-compose-send-from-detail = উত্তর ও ফরোয়ার্ড সবসময় আপনি যে অ্যাকাউন্টে আছেন সেখান থেকে যায়।
settings-compose-send-from-current = আপনি যে অ্যাকাউন্টে আছেন
settings-compose-send-on-replies = উত্তরে পাঠান
settings-compose-send-on-replies-detail = উত্তর বা ফরোয়ার্ডে “পাঠান” কী করে। “পাঠান”-এর পাশের মেনুতে অন্যটি পাবেন।
settings-compose-send-plain = পাঠান
settings-compose-send-archive = পাঠান ও আর্কাইভ করুন
settings-compose-signatures = স্বাক্ষর
settings-compose-signatures-detail = আপনার মেসেজের নিচে, একটি “--” লাইনের পরে যোগ করা হয়। লেখার উইন্ডোতে অন্য একটি বেছে নিন।
settings-compose-untitled = শিরোনামহীন
settings-compose-signature-name = নাম, যেমন অফিস
settings-compose-signature-first = আমার স্বাক্ষর
settings-compose-signature-numbered = স্বাক্ষর { $number }
settings-compose-signature-delete = মুছুন
settings-compose-signature-deleted = স্বাক্ষর মুছে ফেলা হয়েছে
settings-compose-signature-new = নতুন তৈরি করুন
settings-compose-no-signatures = এখনও কোনো স্বাক্ষর নেই।
settings-compose-no-signature = কোনো স্বাক্ষর নেই
settings-compose-for-new-mail = নতুন মেলের জন্য
settings-compose-for-replies = উত্তর ও ফরোয়ার্ডের জন্য
settings-compose-for-replies-detail = যে কথোপকথনে আপনি কোনো মেসেজে স্বাক্ষর দিয়েছেন, সেখানে উত্তর সেই স্বাক্ষর দিয়েই শুরু হয়।
settings-compose-format = ফর্ম্যাট
settings-compose-plain-text = সাধারণ টেক্সটে লিখুন
settings-compose-plain-text-detail = নতুন মেল ফর্ম্যাটিং ছাড়া শুরু হয়; লেখার উইন্ডোতে বদলানো যায়
settings-compose-spelling = বানান
settings-compose-spell-check = লেখার সময় বানান যাচাই করুন
settings-compose-spell-check-detail = ভুল বানানের শব্দের নিচে দাগ থাকে, রাইট-ক্লিকে পরামর্শ পাওয়া যায়
settings-compose-spell-desktop = ডেস্কটপের ভাষা ({ $language })
settings-compose-templates = টেমপ্লেট
settings-compose-templates-detail = যে মেল আপনি প্রায়ই লেখেন তা সেভ করুন, আর সেখান থেকে নতুন মেল বা উত্তর শুরু করুন।

## Settings > Shortcuts

settings-shortcuts-set = শর্টকাট সেট
settings-shortcuts-set-detail = আপনার চেনা কোনো মেল অ্যাপের কী দিয়ে শুরু করুন। এখানে Cmd মানে Ctrl। আপনার নিজের পরিবর্তনগুলি সেটের উপরে থেকে যায়, আর “ডিফল্ট ফিরিয়ে আনুন” সেটের কী-তে ফিরে যায়।
settings-shortcuts-single = এক কী-এর শর্টকাট
settings-shortcuts-single-detail = Ctrl বা Alt ছাড়া কী, যেমন ওয়েবমেলে: e আর্কাইভ করে, j ও k সরায়, / খোঁজে। এগুলি তালিকায় ও খোলা কথোপকথনে কাজ করে, টাইপ করার সময় কখনো নয়।
settings-shortcuts-single-use = এক কী-এর শর্টকাট ব্যবহার করুন
settings-shortcuts-single-use-detail = Ctrl শর্টকাট সবসময় কাজ করে
settings-shortcuts-how = বদলাতে একটি কী-তে ক্লিক করুন, বা নতুন যোগ করতে + চাপুন, তারপর নতুন কী চাপুন। Esc চাপলে বাতিল হয়।
settings-shortcuts-restore = ডিফল্ট ফিরিয়ে আনুন
settings-shortcuts-no-key = কোনো কী নেই
settings-shortcuts-press = কী চাপুন…
settings-shortcuts-then = { $keys } তারপর…
settings-shortcuts-moved = { $keys } এখন “{ $previous }”-এর বদলে “{ $action }” করে।
settings-shortcuts-single-off = এক কী-এর শর্টকাট বন্ধ আছে, তাই এগুলি চালু করলে তবেই এই কী কাজ করবে।
settings-shortcuts-restored = প্রতিটি শর্টকাট আবার তার সেটের কী পেয়েছে।

## Settings search: the line under a result

settings-general-language-summary = অ্যাপ, তারিখ ও সংখ্যার ভাষা
settings-general-reading-summary = সবচেয়ে নতুন মেসেজ আগে, সম্পূর্ণ হেডার, প্রাপকদের পুরো নাম
settings-general-mark-read-summary = খোলা কথোপকথন কখন পঠিত হিসেবে চিহ্নিত হবে: সাথে সাথে, 1 বা 3 সেকেন্ড পরে, বা নিজে হাতে
settings-general-reply-button-summary = প্রতিটি মেসেজের পাশের উত্তর বোতাম সবাইকে উত্তর দেয়
settings-general-remote-images-summary = প্রতিটি মেসেজের ছবি সবসময় দেখান
settings-general-sending-summary = পাঠানো পূর্বাবস্থায় ফেরান: পাঠানো মেসেজ কতক্ষণ অপেক্ষা করবে, যাতে সেটি ফিরিয়ে নেওয়া যায়
settings-general-offline-summary = কত দিনের সাম্প্রতিক মেল পুরোপুরি ডাউনলোড হবে, যাতে কানেকশন ছাড়াই পড়া যায়
settings-general-notifications-summary = নতুন মেলের বিজ্ঞপ্তি ও তার শব্দ
settings-general-desktop-summary = লগ ইন করলে Katna Mail খোলা, সিস্টেম ট্রে আইকন ও টাস্কবারের আইকনে অপঠিত সংখ্যা
settings-accounts-accounts-summary = অ্যাকাউন্ট যোগ করুন বা সরান, অথবা তার ছবি বদলান
settings-appearance-density-summary = তালিকায় ডিফল্ট বা কমপ্যাক্ট লাইন
settings-appearance-scaling-summary = সবকিছু বড় বা ছোট করুন: লেখা, আইকন, ফাঁকা জায়গা ও বিভাজক
settings-appearance-theme-summary = ডেস্কটপের মতো, লাইট বা ডার্ক
settings-appearance-sender-pictures-summary = কোম্পানির লোগো, প্রেরকের ডোমেন দিয়ে খোঁজা
settings-appearance-important-summary = তালিকায় প্রতিটি মেসেজের পাশে গুরুত্বপূর্ণ চিহ্ন
settings-appearance-mail-colors-summary = ডার্ক থিমে HTML মেলের জন্য গাঢ় রং, বা প্রেরকের রং
settings-appearance-attachment-previews-summary = প্রতিটি অ্যাটাচমেন্টের বিষয়বস্তুর একটি ছোট ছবি
settings-shortcuts-set-summary = Gmail, Inbox by Gmail, Apple Mail, Outlook বা Thunderbird-এর কী দিয়ে শুরু করুন
settings-shortcuts-single-summary = Ctrl বা Alt ছাড়া কী, যেমন ওয়েবমেলে
settings-default-apps-pdf-summary = PDF অ্যাটাচমেন্ট কোথায় খুলবে
settings-default-apps-pictures-summary = ফটো ও ছবি কোথায় খুলবে
settings-default-apps-text-summary = সাধারণ টেক্সট, লগ ও কোড কোথায় খুলবে
settings-default-apps-sheets-summary = Excel, OpenDocument ও CSV ফাইল কোথায় খুলবে
settings-default-apps-documents-summary = Word ও OpenDocument টেক্সট কোথায় খুলবে
settings-default-apps-after-saving-summary = সেভ করা অ্যাটাচমেন্ট তাদের ফোল্ডারে দেখান
settings-compose-send-from-summary = নতুন মেল কোন অ্যাকাউন্ট থেকে যাবে: আপনি যেটিতে আছেন, বা সবসময় একই অ্যাকাউন্ট
settings-compose-send-on-replies-summary = উত্তর ও ফরোয়ার্ডে পাঠান, অথবা পাঠান ও কথোপকথন আর্কাইভ করুন
settings-compose-signatures-summary = আপনার মেসেজের নিচে, একটি “--” লাইনের পরে যোগ করা হয়
settings-compose-for-new-mail-summary = নতুন মেল যে স্বাক্ষর দিয়ে শুরু হয়
settings-compose-for-replies-summary = উত্তর ও ফরোয়ার্ড যে স্বাক্ষর দিয়ে শুরু হয়
settings-compose-format-summary = নতুন মেল সাধারণ টেক্সটে লিখুন
settings-compose-spelling-summary = লেখার সময় বানান যাচাই, এবং অভিধানের ভাষা
settings-compose-templates-summary = শীঘ্রই আসছে: যে মেল আপনি প্রায়ই লেখেন তা সেভ করুন, আর সেখান থেকে নতুন মেল বা উত্তর শুরু করুন
settings-feedback-crash-reports-summary = Katna Mail বা তার ব্যাকগ্রাউন্ড পরিষেবা ক্র্যাশ করলে এই কম্পিউটারে ক্র্যাশ রিপোর্ট সেভ করুন
settings-feedback-saved-summary = এই কম্পিউটারে সেভ করা ক্র্যাশ রিপোর্ট দেখুন, কপি করুন বা মুছুন
settings-feedback-help-improve-summary = কী ভুল হয়েছে তা ঠিক করতে সাহায্যের জন্য ক্র্যাশ রিপোর্ট পাঠান; আপনি চালু না করলে বন্ধ থাকে
settings-experimental-blur-summary = উপরের বারের ভেতর দিয়ে ডেস্কটপ ঝাপসা হয়ে দেখা যায়, আর মেনুগুলি ঘষা কাচের মতো দেখায়
settings-search-shortcut = কীবোর্ড শর্টকাট
settings-search-tab = সেটিংস ট্যাব
settings-search-none = “{ $query }”-এর সাথে মেলে এমন কোনো সেটিং নেই।
settings-search-results = “{ $query }”-এর সাথে মেলে এমন সেটিংস

## Quick settings (the panel that slides in from the right)

quick-title = দ্রুত সেটিংস
quick-see-all = সব সেটিংস দেখুন
quick-reading-pane = রিডিং প্যান
quick-pane-right = তালিকার ডানদিকে
quick-pane-none = কোনো বিভাজন নেই
quick-density = ঘনত্ব
quick-density-default = ডিফল্ট
quick-density-compact = কমপ্যাক্ট
quick-theme = থিম
quick-theme-system = ডেস্কটপের মতো
quick-theme-light = লাইট
quick-theme-dark = ডার্ক
quick-desktop-colors = ডেস্কটপের রং
quick-desktop-colors-detail = ডেস্কটপের কালার স্কিম ও অ্যাকসেন্ট কালার
quick-app-names = অ্যাপের নাম
quick-app-names-detail = একেবারে বাঁদিকে অ্যাপ আইকনের নিচে নাম
quick-inbox-tabs = ইনবক্স ট্যাব
quick-inbox-tabs-detail = প্রতিটি অ্যাকাউন্টের মেল প্রদানকারীর ট্যাব
quick-choose-tabs = ট্যাব বেছে নিন
quick-choose-tabs-detail = প্রতিটি অ্যাকাউন্টের জন্য, সেটিংসে
quick-sending = পাঠানো
quick-undo-send = পাঠানো পূর্বাবস্থায় ফেরান
quick-undo-send-off = বন্ধ
quick-undo-send-seconds = { $seconds } সেকেন্ড
quick-signatures = স্বাক্ষর
quick-signatures-none = এখনও নেই
quick-signatures-one = { $name }, ডিফল্ট হিসেবে ব্যবহৃত
quick-signatures-many = { $count ->
    [one] { $count }টি স্বাক্ষর; ডিফল্ট { $name }
   *[other] { $count }টি স্বাক্ষর; ডিফল্ট { $name }
}
quick-signatures-no-default = { $count ->
    [one] { $count }টি, কোনোটিই ডিফল্ট নয়
   *[other] { $count }টি, কোনোটিই ডিফল্ট নয়
}
quick-signature-untitled = শিরোনামহীন
quick-threading = ইমেল থ্রেডিং
quick-conversation-view = কথোপকথন ভিউ
quick-conversation-view-detail = একই মেলের উত্তরগুলি একসাথে রাখুন
quick-help = সহায়তা
quick-tour = ঘুরে দেখুন
quick-whats-new = নতুন কী আছে
quick-about = Katna সম্পর্কে

## Settings: opening at login

settings-open-at-login-failed = লগ ইনের সময় খোলার সেটিং বদলানো যায়নি: { $error }

## Settings > Appearance > Scaling

scale-letter = অ
scale-percent = { $percent }%
scale-reset = আবার { $percent }%-এ

## Settings > Experimental > Look & Feel

look-intro = এখনও পরীক্ষা করে দেখা হচ্ছে এমন ফিচার। এগুলি বদলাতে পারে বা সরিয়ে দেওয়া হতে পারে।
look-heading = চেহারা ও অনুভব
look-window-frame = উইন্ডো ফ্রেম
look-window-frame-detail = টাইটেল বার, উইন্ডোর বোতাম, কোণ ও ছায়া কে আঁকে।
look-frame-native-kde = নেটিভ: KDE-র ফ্রেম, আপনার Plasma থিমে
look-frame-native = নেটিভ: ডেস্কটপের ফ্রেম
look-frame-katna = Katna: উপরের বারই টাইটেল বার হয়ে যায়
look-frame-katna-note-named = Katna গোল কোণ ও নিজের ছায়া আঁকে। ফ্রেম আর { $desktop } থিম অনুসরণ করে না; উইন্ডোর নিয়মগুলি তবুও প্রযোজ্য।
look-frame-katna-note = Katna গোল কোণ ও নিজের ছায়া আঁকে। ফ্রেম আর ডেস্কটপ থিম অনুসরণ করে না; উইন্ডোর নিয়মগুলি তবুও প্রযোজ্য।
look-frame-client-side = আপনার ডেস্কটপ ফ্রেম আঁকার ভার প্রতিটি অ্যাপের উপর ছেড়ে দেয়, তাই Katna আগে থেকেই নিজের ফ্রেম আঁকে।
look-blurred-background = ঝাপসা ব্যাকগ্রাউন্ড
look-blurred-background-detail = উপরের বার ও ফোল্ডারগুলির ভেতর দিয়ে ডেস্কটপ ঝাপসা হয়ে দেখা যায়, আর মেনু ও পপওভারগুলি ঘষা কাচের মতো দেখায়।
look-blur = উইন্ডোর পেছনে যা আছে তা ঝাপসা করুন
look-blur-detail = মেল নিরেট কার্ডেই থাকে, তাই লেখার কনট্রাস্ট বজায় থাকে
look-blur-off-kde = KDE-র ঝাপসা (Blur) এফেক্ট বন্ধ আছে। সিস্টেম সেটিংস, উইন্ডো ব্যবস্থাপনা, ডেস্কটপ এফেক্টস-এ ঝাপসা চালু করুন, তারপর Katna Mail আবার খুলুন।
look-blur-none-gnome = GNOME উইন্ডোর পেছনে যা আছে তা ঝাপসা করে না।
look-blur-none-x11 = আপনার উইন্ডো ম্যানেজার উইন্ডোর পেছনে যা আছে তা ঝাপসা করে না।
look-blur-none-wayland = আপনার কম্পোজিটর উইন্ডোর পেছনে যা আছে তা ঝাপসা করে না।

## Settings > User feedback (crash reports)

feedback-intro-sending = কী ভুল হয়েছে তা ঠিক করতে সাহায্যের জন্য নতুন ক্র্যাশ রিপোর্ট পাঠানো হয়। এর বাইরে কিছুই এই কম্পিউটার থেকে বাইরে যায় না।
feedback-intro-local = Katna কোথাও কিছু পাঠায় না। ক্র্যাশ রিপোর্ট এই কম্পিউটারেই থাকে, যাতে আপনি দেখতে পারেন বা কোনো বাগ রিপোর্টে সংযুক্ত করতে পারেন।
feedback-crash-reports = ক্র্যাশ রিপোর্ট
feedback-crash-reports-detail = Katna Mail বা তার ব্যাকগ্রাউন্ড পরিষেবা ক্র্যাশ করলে লেখা হয়।
feedback-save = এই কম্পিউটারে ক্র্যাশ রিপোর্ট সেভ করুন
feedback-save-detail = আপনার হোম ফোল্ডার, ব্যবহারকারী ও কম্পিউটারের নাম এবং ইমেল ঠিকানা বাদ দেওয়া হয়
feedback-saved = সেভ করা ক্র্যাশ রিপোর্ট
feedback-saved-detail = { $count ->
    [one] সবচেয়ে নতুন { $count }টি রাখা হয়।
   *[other] সবচেয়ে নতুন { $count }টি রাখা হয়।
}
feedback-help-improve = Katna উন্নত করতে সাহায্য করুন
feedback-help-improve-detail = আপনি চালু না করলে বন্ধ থাকে, আর আপনি যেকোনো সময় এখানে এটি বন্ধ করতে পারেন।
feedback-send = ক্র্যাশ রিপোর্ট পাঠান
feedback-send-detail = সেভ করা রিপোর্ট, ঠিক যেমন আপনি এখানে দেখতে পান, Katna-র ক্র্যাশ ট্র্যাকারে (Sentry, EU-তে) যায়। কোনো IP ঠিকানা, মেসেজ বা ইমেল ঠিকানা নয়
feedback-none-saved = কোনো ক্র্যাশ রিপোর্ট সেভ করা নেই।
feedback-delete-all = সব মুছুন
feedback-app-daemon = ব্যাকগ্রাউন্ড পরিষেবা
feedback-report-sent = { $date } · পাঠানো হয়েছে
feedback-view = দেখুন
feedback-view-tooltip = রিপোর্টটি খুলুন
feedback-copy-tooltip = বাগ রিপোর্টে পেস্ট করতে এটি কপি করুন
feedback-copied = ক্র্যাশ রিপোর্ট কপি করা হয়েছে।
feedback-deleted-all = ক্র্যাশ রিপোর্টগুলি মুছে ফেলা হয়েছে।
feedback-read-failed = ক্র্যাশ রিপোর্ট পড়া যায়নি: { $error }
feedback-delete-failed = ক্র্যাশ রিপোর্ট মোছা যায়নি: { $error }
feedback-delete-all-failed = ক্র্যাশ রিপোর্টগুলি মোছা যায়নি: { $error }

## Menu bar (the KDE global menu)

desktop-menu-file = _ফাইল
desktop-menu-new-message = _নতুন মেসেজ
desktop-menu-quit = _প্রস্থান
desktop-menu-edit = _সম্পাদনা
desktop-menu-undo = _পূর্বাবস্থায় ফেরান
desktop-menu-select-all = _সব বেছে নিন
desktop-menu-select-none = _কোনোটিই বেছে নেবেন না
desktop-menu-find = _খুঁজুন…
desktop-menu-view = _দেখুন
desktop-menu-folder-list = _ফোল্ডার তালিকা দেখান
desktop-menu-refresh = _রিফ্রেশ করুন
desktop-menu-go = _যান
desktop-menu-inbox = _ইনবক্স
desktop-menu-starred = _তারকাচিহ্নিত
desktop-menu-sent = _পাঠানো হয়েছে
desktop-menu-drafts = _খসড়া
desktop-menu-all-mail = _সব মেল
desktop-menu-next = _পরের কথোপকথন
desktop-menu-previous = _আগের কথোপকথন
desktop-menu-message = _মেসেজ
desktop-menu-open = _খুলুন
desktop-menu-reply = _উত্তর দিন
desktop-menu-reply-all = _সবাইকে উত্তর দিন
desktop-menu-forward = _ফরোয়ার্ড করুন
desktop-menu-archive = _আর্কাইভ করুন
desktop-menu-delete = _মুছুন
desktop-menu-spam = _স্প্যাম হিসেবে রিপোর্ট করুন
desktop-menu-move-to = _এখানে সরান…
desktop-menu-mark-read = _পঠিত হিসেবে চিহ্নিত করুন
desktop-menu-mark-unread = _অপঠিত হিসেবে চিহ্নিত করুন
desktop-menu-star = _তারকাচিহ্ন দিন
desktop-menu-important = _গুরুত্বপূর্ণ হিসেবে চিহ্নিত করুন
desktop-menu-not-important = _গুরুত্বপূর্ণ নয় হিসেবে চিহ্নিত করুন
desktop-menu-settings = _সেটিংস
desktop-menu-quick-settings = _দ্রুত সেটিংস
desktop-menu-configure = _Katna Mail কনফিগার করুন…
desktop-menu-help = _সহায়তা
desktop-menu-shortcuts = _কীবোর্ড শর্টকাট
desktop-menu-whats-new = _নতুন কী আছে
desktop-menu-about = _Katna সম্পর্কে

## Settings > Keyboard shortcuts: the groups of the list

shortcut-group-moving = নেভিগেশন
shortcut-group-actions = কাজ
shortcut-group-go-to = যান
shortcut-group-app = অ্যাপ্লিকেশন

## Settings > Keyboard shortcuts: what each shortcut does

shortcut-next = পরের কথোপকথন
shortcut-previous = আগের কথোপকথন
shortcut-down = তালিকায় নিচে যান
shortcut-up = তালিকায় উপরে যান
shortcut-first = তালিকার প্রথমটি
shortcut-last = তালিকার শেষটি
shortcut-page-down = তালিকায় এক পাতা নিচে
shortcut-page-up = তালিকায় এক পাতা উপরে
shortcut-open = কথোপকথন খুলুন
shortcut-back = তালিকায় ফিরে যান
shortcut-scroll-down = নিচে স্ক্রল করুন
shortcut-scroll-up = উপরে স্ক্রল করুন
shortcut-scroll-page-down = এক পাতা নিচে স্ক্রল করুন
shortcut-scroll-page-up = এক পাতা উপরে স্ক্রল করুন
shortcut-compose = লিখুন
shortcut-reply = উত্তর দিন
shortcut-reply-all = সবাইকে উত্তর দিন
shortcut-forward = ফরোয়ার্ড করুন
shortcut-archive = আর্কাইভ করুন
shortcut-delete = মুছুন
shortcut-spam = স্প্যাম হিসেবে রিপোর্ট করুন
shortcut-move-to = এখানে সরান
shortcut-mark-read = পঠিত হিসেবে চিহ্নিত করুন
shortcut-mark-unread = অপঠিত হিসেবে চিহ্নিত করুন
shortcut-star = তারকাচিহ্ন দিন বা সরান
shortcut-important = গুরুত্বপূর্ণ হিসেবে চিহ্নিত করুন
shortcut-not-important = গুরুত্বপূর্ণ নয় হিসেবে চিহ্নিত করুন
shortcut-check = কথোপকথনে টিক দিন
shortcut-select-all = সব কথোপকথনে টিক দিন
shortcut-select-none = সব কথোপকথন থেকে টিক সরান
shortcut-undo = শেষ কাজটি পূর্বাবস্থায় ফেরান
shortcut-go-inbox = ইনবক্স
shortcut-go-starred = তারকাচিহ্নিত
shortcut-go-sent = পাঠানো হয়েছে
shortcut-go-drafts = খসড়া
shortcut-go-all = সব মেল
shortcut-search = মেল খুঁজুন
shortcut-navigation = মেনু দেখান বা লুকান
shortcut-quick-settings = দ্রুত সেটিংস
shortcut-settings = সব সেটিংস
shortcut-shortcuts = কীবোর্ড শর্টকাট
shortcut-reload = নতুন মেল দেখুন
shortcut-quit = প্রস্থান

## Keys pressed one after another, as a shortcut shows them ("G then I")

shortcut-sequence = { $first } তারপর { $second }

## Settings > Accounts

accounts-folder-pane = ফোল্ডার প্যান
accounts-folder-pane-detail = বাঁদিকের প্যানে কোন অ্যাকাউন্টের ফোল্ডার দেখাবে।
accounts-shown-one = একবারে একটি অ্যাকাউন্ট; অ্যাকাউন্ট কার্ডে বদলান
accounts-shown-all = সব অ্যাকাউন্ট, একটির পর একটি
accounts-row = অ্যাকাউন্ট
accounts-row-detail = কোনো অ্যাকাউন্ট সরালে এই কম্পিউটারে তার মেলের Katna-র কপি মুছে যায়। মেল সার্ভারে থেকে যায়।
accounts-none = এখনও কোনো অ্যাকাউন্ট নেই।
accounts-kind-imported = ইমপোর্ট করা
accounts-picture-reset = ডেস্কটপের ছবি ব্যবহার করুন
accounts-picture-change = ছবি বদলান
accounts-remove = সরান
accounts-delete-all-row = সব ডেটা মুছুন
accounts-delete-all-row-detail = নতুন করে শুরু করুন, নতুন ইনস্টলের মতো।
accounts-delete-all-about = এই কম্পিউটার থেকে প্রতিটি অ্যাকাউন্ট, সব সেভ করা মেল, পরিচিতি ও ক্যালেন্ডার, সার্চ ইনডেক্স, আপনার সেটিংস এবং সেভ করা পাসওয়ার্ড মুছে দেয়। আপনার মেল সার্ভারে কিছুই বদলায় না।
accounts-delete-all-open = Katna-র সব ডেটা মুছুন

## Settings > Accounts: snackbars after deleting

accounts-removed-local = { $address } Katna থেকে সরানো হয়েছে।
accounts-removed = { $address } Katna থেকে সরানো হয়েছে। এর মেল এখনও সার্ভারে আছে।
accounts-all-deleted = Katna-র সব ডেটা এই কম্পিউটার থেকে মুছে ফেলা হয়েছে।

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = { $address } সরাবেন?
accounts-remove-confirm = অ্যাকাউন্ট সরান
accounts-removing = সরানো হচ্ছে…
accounts-remove-local-mail = { $folders ->
    [0] এই অ্যাকাউন্টে ইমপোর্ট করা সব মেল
    [one] এই অ্যাকাউন্টে ইমপোর্ট করা সব মেল, তার ফোল্ডারে
   *[other] এই অ্যাকাউন্টে ইমপোর্ট করা সব মেল, তার { $folders }টি ফোল্ডারে
}
accounts-remove-local-settings = এর Katna সেটিংস
accounts-remove-mail = { $folders ->
    [0] Katna-য় সেভ করা এই অ্যাকাউন্টের সব মেল
    [one] Katna-য় সেভ করা এই অ্যাকাউন্টের সব মেল, তার ফোল্ডারে
   *[other] Katna-য় সেভ করা এই অ্যাকাউন্টের সব মেল, তার { $folders }টি ফোল্ডারে
}
accounts-remove-outbox = আউটবক্সে অপেক্ষারত এর মেসেজগুলি
accounts-remove-settings = এর সেভ করা পাসওয়ার্ড ও Katna সেটিংস
accounts-delete-all-title = Katna-র সব ডেটা মুছবেন?
accounts-delete-all-confirm = সবকিছু মুছুন
accounts-deleting = মোছা হচ্ছে…
accounts-delete-all-accounts = প্রতিটি অ্যাকাউন্ট, এবং Katna-য় সেভ করা সব মেল ও অ্যাটাচমেন্ট
accounts-delete-all-contacts = পরিচিতি, ক্যালেন্ডার ও সার্চ ইনডেক্স
accounts-delete-all-settings = সব সেটিংস, স্বাক্ষর ও কীবোর্ড শর্টকাট
accounts-delete-all-passwords = সেভ করা প্রতিটি পাসওয়ার্ড
accounts-deleted-heading = এই কম্পিউটার থেকে মুছে যাবে:
accounts-cannot-undo = এটি পূর্বাবস্থায় ফেরানো যাবে না।
accounts-server-delete-all = আপনার মেল সার্ভারে কিছুই বদলায় না: আপনার মেল সেখানেই থাকে, আর আবার অ্যাকাউন্ট যোগ করলে আবার ডাউনলোড হয়। ফাইল থেকে ইমপোর্ট করা মেল শুধু Katna-তেই আছে; ফাইলগুলিতে হাত দেওয়া হয় না।
accounts-server-local = এই মেল ফাইল থেকে ইমপোর্ট করা হয়েছিল, তাই এর একমাত্র কপি Katna-র কাছে। যে ফাইলগুলি থেকে এটি এসেছে সেগুলিতে হাত দেওয়া হয় না; ফিরে পেতে সেগুলি আবার ইমপোর্ট করুন।
accounts-server-remove = মেল সার্ভারে কিছুই বদলায় না: আপনার মেল সেখানেই থাকে, আর আবার অ্যাকাউন্টটি যোগ করলে আবার ডাউনলোড হয়।
accounts-confirm-word = মুছুন
accounts-confirm-placeholder = “{ accounts-confirm-word }” লিখুন
accounts-confirm-prompt = নিশ্চিত করতে “{ accounts-confirm-word }” লিখুন:
accounts-cancel = বাতিল করুন
