# Katna Mail, Assamese (অসমীয়া).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = ফাইল সন্ধান কৰক

## Left side (and chips on a phone)

files-all = সকলো ফাইল
files-pictures = ছবি
files-pdfs = PDF
files-documents = নথি
files-sheets = স্প্ৰেডশ্বীট
files-slides = স্লাইড
files-other = অন্যান্য
files-accounts = একাউণ্টসমূহ
files-drives = ড্ৰাইভ
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = মোৰ সৈতে শ্বেয়াৰ কৰা
files-shown = দেখুওৱা
files-received = পোৱা
files-sent = মই পঠিওৱা

## Over the files

files-count = { $count ->
    [one] { $count }টা ফাইল · { $size }
   *[other] { $count }টা ফাইল · { $size }
}
files-anyone = যিকোনো
files-from-person = { $name }ৰ পৰা
files-time-any = যিকোনো সময়
files-time-today = আজি
files-time-yesterday = কালি
files-time-this-week = এই সপ্তাহ
files-time-last-week = যোৱা সপ্তাহ
files-time-this-month = এই মাহ
files-time-last-month = যোৱা মাহ
files-time-between = { $first } – { $last }
files-time-hint = এটা দিনত ক্লিক কৰক, বা কেইবাটাও দিনৰ ওপৰেৰে টানক
files-time-summary = { $count ->
    [one] { $days } · { $count }টা ফাইল
   *[other] { $days } · { $count }টা ফাইল
}
files-time-clear = মচক
files-time-month-back = আগৰ মাহ
files-time-month-on = পিছৰ মাহ
files-time-wheel = দৈৰ্ঘ্য একে ৰাখি এই তাৰিখবোৰ সলনি কৰিবলৈ স্ক্ৰল কৰক
files-sort-newest = শেহতীয়াখিনি প্ৰথমে
files-sort-oldest = পুৰণিখিনি প্ৰথমে
files-sort-largest = ডাঙৰখিনি প্ৰথমে
files-sort-name = নাম অনুসৰি
files-grid = কাৰ্ড
files-list = তালিকা
files-this-week = এই সপ্তাহ
files-undated = তাৰিখ নাই
files-me = মই
files-no-subject = (কোনো বিষয় নাই)
files-loading = আপোনাৰ মেইলৰ পৰা ফাইল গোটাই আছে…
files-empty = আপোনাৰ মেইলৰ ফাইলবোৰ ইয়াত দেখা যায়।
files-none-match = কোনো ফাইল নিমিলে।
files-load-failed = ফাইলবোৰ পঢ়িব পৰা নগ'ল: { $error }

## A file's menu and buttons

files-open = খোলক
files-open-with = ইয়াৰে খোলক…
files-save = ছেভ কৰক…
files-show-mail = মেইলটো দেখুৱাওক
files-mail-window = মেইলটো নতুন উইণ্ড'ত খোলক
files-forward = ফাইলটো ফৰৱাৰ্ড কৰক
files-from-them = { $name }ৰ ফাইলসমূহ
files-copy-name = ফাইলৰ নাম কপি কৰক
files-name-copied = ফাইলৰ নাম কপি কৰা হ'ল
files-downloading = মেইলটো ডাউনল'ড হৈ আছে…
files-download-failed = এই মেইলটো ডাউনল'ড কৰিব পৰা নগ'ল।

## A cloud drive in place of the mail files

files-drive-mine = মোৰ ড্ৰাইভ
files-drive-mine-onedrive = মোৰ ফাইল
files-drive-results = “{ $words }”
files-drive-count = { $folders ->
    [0] { $files ->
        [one] 1টা ফাইল
       *[other] { $files }টা ফাইল
    }
    [one] 1টা ফ'ল্ডাৰ · { $files ->
        [one] 1টা ফাইল
       *[other] { $files }টা ফাইল
    }
   *[other] { $folders }টা ফ'ল্ডাৰ · { $files ->
        [one] 1টা ফাইল
       *[other] { $files }টা ফাইল
    }
}
files-drive-folders = ফ'ল্ডাৰ
files-drive-files = ফাইলসমূহ
files-drive-folder = ফ'ল্ডাৰ
files-drive-meta = { $what } · { $date }ত সম্পাদিত
files-drive-as-link = { $what } · লিংক হিচাপে
files-drive-google-doc = Google Doc
files-drive-google-sheet = Google Sheet
files-drive-google-slides = Google Slides
files-drive-google-drawing = Google Drawing
files-drive-fetching = আনি আছে…
files-drive-loading = ড্ৰাইভ খুলি আছে…
files-drive-empty = এই ফ'ল্ডাৰটো খালী।
files-drive-unreachable = { $drive }ৰ সৈতে সংযোগ কৰিব পৰা নাই।
files-drive-try-again = পুনৰ চেষ্টা কৰক
files-drive-needs-permission = এই ড্ৰাইভ দেখুৱাবলৈ Katnaক এবাৰ আপোনাৰ অনুমতি লাগে। পুনৰ ছাইন ইন কৰক আৰু Katnaক আপোনাৰ ফাইল চাবলৈ অনুমতি দিয়ক।
files-drive-allow = অনুমতি দিয়ক
files-drive-allow-failed = ছাইন ইন সম্পূৰ্ণ নহ'ল, সেয়ে ড্ৰাইভ বন্ধ হৈ থাকিল।
files-drive-attach = সংলগ্ন কৰক
files-drive-more = অধিক
files-drive-download = ডাউনল'ড কৰক…
files-drive-open-web = { $drive }ত খোলক
files-drive-copy-link = লিংক কপি কৰক
files-drive-link-copied = লিংক কপি কৰা হ'ল
files-drive-share = শ্বেয়াৰ কৰক…
files-drive-rename = নাম সলনি কৰক
files-drive-trash = বিনলৈ নিয়ক
files-drive-trashed = “{ $name }” { $drive }ৰ বিনত আছে
files-drive-renamed = নাম সলনি কৰি “{ $name }” কৰা হ'ল
files-drive-getting = { $drive }ৰ পৰা { $name } আনি আছে…
files-drive-get-failed = { $name } আনিব পৰা নগ'ল: { $error }
files-drive-upload = আপলোড কৰক
files-drive-upload-files = ফাইল আপলোড কৰক
files-drive-upload-folder = ফ'ল্ডাৰ আপলোড কৰক
files-drive-upload-failed = { $name } আপলোড কৰিব পৰা নগ'ল: { $error }
files-drive-upload-needs = আপলোড কৰিবলৈ Katnaক এবাৰ আপোনাৰ অনুমতি লাগে: ছেটিংছ › ডিফ'ল্ট এপ › ফাইলসমূহ পৃষ্ঠাত অনুমতি দিয়ক টিপক।

## The Share dialog of a drive file or folder

files-share-title = “{ $name }” শ্বেয়াৰ কৰক
files-share-add = নাম বা ঠিকনাৰে মানুহ যোগ কৰক
files-share-not-address = “{ $text }” এটা ইমেইল ঠিকনা নহয়
files-share-notify = { $drive }কো তেওঁলোকলৈ ইমেইল পঠিয়াবলৈ দিয়ক
files-share-people = প্ৰৱেশাধিকাৰ থকা লোকসকল
files-share-general = সাধাৰণ প্ৰৱেশাধিকাৰ
files-share-loading = কাৰ প্ৰৱেশাধিকাৰ আছে পঢ়ি আছে…
files-share-restricted = সীমিত
files-share-restricted-about = কেৱল প্ৰৱেশাধিকাৰ থকা লোকেহে লিংকৰে ইয়াক খুলিব পাৰে
files-share-anyone = লিংক থকা যিকোনো লোক
files-share-anyone-can = { $role ->
    [editor] লিংক থকা যিকোনো লোকে সম্পাদনা কৰিব পাৰে
    [commenter] লিংক থকা যিকোনো লোকে মন্তব্য কৰিব পাৰে
   *[viewer] লিংক থকা যিকোনো লোকে চাব পাৰে
}
files-share-anyone-about = { $role ->
    [editor] ইণ্টাৰনেটত লিংক থকা যিকোনো লোকে সম্পাদনা কৰিব পাৰে
    [commenter] ইণ্টাৰনেটত লিংক থকা যিকোনো লোকে মন্তব্য কৰিব পাৰে
   *[viewer] ইণ্টাৰনেটত লিংক থকা যিকোনো লোকে চাব পাৰে
}
files-share-role-owner = গৰাকী
files-share-role-editor = সম্পাদক
files-share-role-commenter = মন্তব্যকাৰী
files-share-role-viewer = দৰ্শক
files-share-you = { $name } (আপুনি)
files-share-domain = { $domain }ৰ সকলো
files-share-inherited = ই থকা ফ'ল্ডাৰৰ পৰা পোৱা প্ৰৱেশাধিকাৰ
files-share-remove = প্ৰৱেশাধিকাৰ আঁতৰাওক
files-share-copy-link = লিংক কপি কৰক
files-share-share = শ্বেয়াৰ কৰক
files-share-done = হ'ল
files-share-sharing = শ্বেয়াৰ কৰি আছে…
files-share-shared = { $count ->
    [one] 1 জন ব্যক্তিৰ সৈতে শ্বেয়াৰ কৰা হ'ল
   *[other] { $count } জন ব্যক্তিৰ সৈতে শ্বেয়াৰ কৰা হ'ল
}
files-share-refused = { $drive }এ { $addresses }ৰ সৈতে শ্বেয়াৰ কৰিব নোৱাৰিলে
files-share-failed = শ্বেয়াৰিং সলনি কৰিব পৰা নগ'ল: { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
    [one] 1টা বস্তু আপলোড হৈ আছে
   *[other] { $count }টা বস্তু আপলোড হৈ আছে
}
files-tray-done = { $count ->
    [one] 1টা আপলোড সম্পূৰ্ণ
   *[other] { $count }টা আপলোড সম্পূৰ্ণ
}
files-tray-some-failed = { $done }টা আপলোড হ'ল, { $failed }টা বিফল
files-tray-minutes-left = { $minutes ->
    [one] প্ৰায় এক মিনিট বাকী
   *[other] প্ৰায় { $minutes } মিনিট বাকী
}
files-tray-seconds-left = এক মিনিটতকৈ কম বাকী
files-tray-starting = আৰম্ভ হৈছে…
files-tray-cancel-all = সকলো বাতিল কৰক
files-tray-cancel = বাতিল কৰক
files-tray-fold = তালিকা লুকুৱাওক
files-tray-unfold = তালিকা দেখুৱাওক
files-tray-close = বন্ধ কৰক
files-tray-progress = { $place } · { $size }ৰ { $sent }
files-tray-in = { $place }ত
files-tray-cancelled = বাতিল কৰা হ'ল
