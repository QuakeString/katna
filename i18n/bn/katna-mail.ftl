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
