# Katna Mail, Bengali (বাংলা).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = ফাইল খুঁজুন

## Left side (and chips on a phone)

files-all = সব ফাইল
files-pictures = ছবি
files-pdfs = PDF
files-documents = ডকুমেন্ট
files-sheets = স্প্রেডশিট
files-slides = স্লাইড
files-other = অন্যান্য
files-accounts = অ্যাকাউন্ট
files-drives = ড্রাইভ
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = আমার সাথে শেয়ার করা
files-shown = দেখানো হচ্ছে
files-received = প্রাপ্ত
files-sent = আমার পাঠানো

## Over the files

files-count = { $count ->
    [one] { $count }টি ফাইল · { $size }
   *[other] { $count }টি ফাইল · { $size }
}
files-anyone = যে কেউ
files-from-person = { $name }-এর কাছ থেকে
files-time-any = যেকোনো সময়
files-time-today = আজ
files-time-yesterday = গতকাল
files-time-this-week = এই সপ্তাহ
files-time-last-week = গত সপ্তাহ
files-time-this-month = এই মাস
files-time-last-month = গত মাস
files-time-between = { $first } – { $last }
files-time-hint = একটি দিনে ক্লিক করুন, বা কয়েকটি দিনের উপর দিয়ে টেনে আনুন
files-time-summary = { $count ->
    [one] { $days } · { $count }টি ফাইল
   *[other] { $days } · { $count }টি ফাইল
}
files-time-clear = মুছুন
files-time-month-back = আগের মাস
files-time-month-on = পরের মাস
files-time-wheel = দৈর্ঘ্য একই রেখে এই তারিখগুলি সরাতে স্ক্রল করুন
files-sort-newest = সবচেয়ে নতুন আগে
files-sort-oldest = সবচেয়ে পুরনো আগে
files-sort-largest = সবচেয়ে বড় আগে
files-sort-name = নাম অনুযায়ী
files-grid = কার্ড
files-list = তালিকা
files-this-week = এই সপ্তাহ
files-undated = কোনো তারিখ নেই
files-me = আমি
files-no-subject = (কোনো বিষয় নেই)
files-loading = আপনার মেল থেকে ফাইল জোগাড় করা হচ্ছে…
files-empty = আপনার মেলের ফাইল এখানে দেখা যাবে।
files-none-match = কোনো ফাইল মেলেনি।
files-load-failed = ফাইল পড়া যায়নি: { $error }

## A file's menu and buttons

files-open = খুলুন
files-open-with = এটি দিয়ে খুলুন…
files-save = সেভ করুন…
files-show-mail = মেলটি দেখান
files-mail-window = মেলটি নতুন উইন্ডোতে খুলুন
files-forward = ফাইলটি ফরোয়ার্ড করুন
files-from-them = { $name }-এর ফাইল
files-copy-name = ফাইলের নাম কপি করুন
files-name-copied = ফাইলের নাম কপি করা হয়েছে
files-downloading = মেলটি ডাউনলোড হচ্ছে…
files-download-failed = এই মেলটি ডাউনলোড করা যায়নি।

## A cloud drive in place of the mail files

files-drive-mine = আমার ড্রাইভ
files-drive-mine-onedrive = আমার ফাইল
files-drive-results = “{ $words }”
files-drive-count = { $folders ->
    [0] { $files ->
        [one] 1টি ফাইল
       *[other] { $files }টি ফাইল
    }
    [one] 1টি ফোল্ডার · { $files ->
        [one] 1টি ফাইল
       *[other] { $files }টি ফাইল
    }
   *[other] { $folders }টি ফোল্ডার · { $files ->
        [one] 1টি ফাইল
       *[other] { $files }টি ফাইল
    }
}
files-drive-folders = ফোল্ডার
files-drive-files = ফাইল
files-drive-folder = ফোল্ডার
files-drive-meta = { $what } · সম্পাদিত { $date }
files-drive-as-link = { $what } · লিঙ্ক হিসেবে
files-drive-google-doc = Google Doc
files-drive-google-sheet = Google Sheet
files-drive-google-slides = Google Slides
files-drive-google-drawing = Google Drawing
files-drive-fetching = আনা হচ্ছে…
files-drive-loading = ড্রাইভ খোলা হচ্ছে…
files-drive-empty = এই ফোল্ডারটি খালি।
files-drive-unreachable = { $drive }-এর সাথে যোগাযোগ করা যাচ্ছে না।
files-drive-try-again = আবার চেষ্টা করুন
files-drive-needs-permission = এই ড্রাইভ দেখাতে Katna-র একবার আপনার অনুমতি দরকার। আবার সাইন ইন করুন এবং Katna-কে আপনার ফাইল দেখার অনুমতি দিন।
files-drive-allow = অনুমতি দিন
files-drive-allow-failed = সাইন ইন সম্পূর্ণ হয়নি, তাই ড্রাইভ বন্ধ থাকছে।
files-drive-attach = সংযুক্ত করুন
files-drive-more = আরও
files-drive-download = ডাউনলোড করুন…
files-drive-open-web = { $drive }-এ খুলুন
files-drive-copy-link = লিঙ্ক কপি করুন
files-drive-link-copied = লিঙ্ক কপি করা হয়েছে
files-drive-share = শেয়ার করুন…
files-drive-rename = নাম বদলান
files-drive-trash = ট্র্যাশে সরান
files-drive-trashed = “{ $name }” { $drive }-এর ট্র্যাশে আছে
files-drive-renamed = নাম বদলে “{ $name }” করা হয়েছে
files-drive-getting = { $drive } থেকে { $name } আনা হচ্ছে…
files-drive-get-failed = { $name } আনা যায়নি: { $error }
files-drive-upload = আপলোড
files-drive-upload-files = ফাইল আপলোড করুন
files-drive-upload-folder = ফোল্ডার আপলোড করুন
files-drive-upload-failed = { $name } আপলোড করা যায়নি: { $error }
files-drive-upload-needs = আপলোড করতে Katna-র একবার আপনার অনুমতি দরকার: সেটিংস › ডিফল্ট অ্যাপ › ফাইল পেজ-এ অনুমতি দিন চাপুন।

## The Share dialog of a drive file or folder

files-share-title = “{ $name }” শেয়ার করুন
files-share-add = নাম বা ঠিকানা দিয়ে মানুষ যোগ করুন
files-share-not-address = “{ $text }” কোনো ইমেল ঠিকানা নয়
files-share-notify = { $drive }-কেও ওঁদের ইমেল করতে দিন
files-share-people = যাঁদের অ্যাক্সেস আছে
files-share-general = সাধারণ অ্যাক্সেস
files-share-loading = কার অ্যাক্সেস আছে দেখা হচ্ছে…
files-share-restricted = সীমিত
files-share-restricted-about = শুধু অ্যাক্সেস থাকা মানুষেরাই লিঙ্ক দিয়ে এটি খুলতে পারবেন
files-share-anyone = লিঙ্ক থাকা যে কেউ
files-share-anyone-can = { $role ->
    [editor] লিঙ্ক থাকা যে কেউ সম্পাদনা করতে পারবেন
    [commenter] লিঙ্ক থাকা যে কেউ মন্তব্য করতে পারবেন
   *[viewer] লিঙ্ক থাকা যে কেউ দেখতে পারবেন
}
files-share-anyone-about = { $role ->
    [editor] ইন্টারনেটে লিঙ্ক থাকা যে কেউ সম্পাদনা করতে পারবেন
    [commenter] ইন্টারনেটে লিঙ্ক থাকা যে কেউ মন্তব্য করতে পারবেন
   *[viewer] ইন্টারনেটে লিঙ্ক থাকা যে কেউ দেখতে পারবেন
}
files-share-role-owner = মালিক
files-share-role-editor = সম্পাদক
files-share-role-commenter = মন্তব্যকারী
files-share-role-viewer = দর্শক
files-share-you = { $name } (আপনি)
files-share-domain = { $domain }-এর সবাই
files-share-inherited = যে ফোল্ডারে আছে সেখান থেকে পাওয়া অ্যাক্সেস
files-share-remove = অ্যাক্সেস সরান
files-share-copy-link = লিঙ্ক কপি করুন
files-share-share = শেয়ার করুন
files-share-done = হয়ে গেছে
files-share-close = বন্ধ করুন
files-share-sharing = শেয়ার করা হচ্ছে…
files-share-shared = { $count ->
    [one] 1 জনের সাথে শেয়ার করা হয়েছে
   *[other] { $count } জনের সাথে শেয়ার করা হয়েছে
}
files-share-refused = { $drive } { $addresses }-এর সাথে শেয়ার করতে পারেনি
files-share-failed = শেয়ারিং বদলানো যায়নি: { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
    [one] 1টি আইটেম আপলোড হচ্ছে
   *[other] { $count }টি আইটেম আপলোড হচ্ছে
}
files-tray-done = { $count ->
    [one] 1টি আপলোড সম্পূর্ণ
   *[other] { $count }টি আপলোড সম্পূর্ণ
}
files-tray-some-failed = { $done }টি আপলোড হয়েছে, { $failed }টি ব্যর্থ
files-tray-minutes-left = { $minutes ->
    [one] প্রায় এক মিনিট বাকি
   *[other] প্রায় { $minutes } মিনিট বাকি
}
files-tray-seconds-left = এক মিনিটেরও কম বাকি
files-tray-starting = শুরু হচ্ছে…
files-tray-cancel-all = সব বাতিল করুন
files-tray-cancel = বাতিল করুন
files-tray-fold = তালিকা লুকান
files-tray-unfold = তালিকা দেখান
files-tray-close = বন্ধ করুন
files-tray-progress = { $place } · { $size }-এর মধ্যে { $sent }
files-tray-in = { $place }-এ
files-tray-cancelled = বাতিল করা হয়েছে
