# Katna Mail, Bengali (বাংলা): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = নতুন টাস্ক
tasks-all = সব টাস্ক
tasks-today = আজ
tasks-upcoming = আসন্ন
tasks-starred = তারকাচিহ্নিত
tasks-completed-view = সম্পন্ন
tasks-new-list = নতুন তালিকা তৈরি করুন
tasks-labels-heading = লেবেল
tasks-on-this-computer = এই কম্পিউটারে
tasks-my-tasks = আমার টাস্ক
# The line under an account in the side list whose task lists could not
# come: why, and the one click that fixes it.
tasks-account-sign-in = টাস্ক দেখাতে আবার সাইন ইন করুন
tasks-account-signed-in = { $address }-এ আবার সাইন ইন করা হয়েছে। আপনার টাস্ক আনা হচ্ছে…
tasks-account-sign-in-refused = { $provider } Katna-কে ঢুকতে দেয়নি। আবার চেষ্টা করুন, এবং আপনার টাস্কে অ্যাক্সেসের অনুমতি দিন।
tasks-account-refused = সার্ভার পাসওয়ার্ডটি গ্রহণ করেনি। Yahoo, iCloud, Zoho ও অন্যদের জন্য একটি অ্যাপ পাসওয়ার্ড লাগে।
tasks-account-change-password = পাসওয়ার্ড বদলান
tasks-account-change-password-tooltip = নতুন পাসওয়ার্ড লিখুন; Katna সেটি সার্ভারের সাথে যাচাই করবে
tasks-account-not-enabled = Katna-র জন্য টাস্ক অ্যাক্সেস এখনও চালু করা হয়নি।
tasks-account-failed = টাস্ক তালিকাগুলি পড়া যায়নি।
# $reason is the server's own words, in English.
tasks-account-error = টাস্ক তালিকাগুলি পড়া যায়নি: { $reason }
tasks-account-none = কোনো টাস্ক তালিকা পাওয়া যায়নি
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
tasks-account-none-why = কোনো টাস্ক তালিকা পাওয়া যায়নি: { $reason }
# A Gmail or Outlook account added with a password: its tasks need the
# provider's sign-in.
tasks-account-use-sign-in = { $provider } শুধু { $provider } দিয়ে সাইন ইন করা Katna-কেই টাস্ক দেখায়।
tasks-account-sign-in-with = { $provider } দিয়ে সাইন ইন করুন
tasks-account-looking = টাস্ক তালিকা খোঁজা হচ্ছে…
tasks-account-try-again = আবার চেষ্টা করুন
tasks-account-try-again-tooltip = এই অ্যাকাউন্টের টাস্কগুলো এখনই আবার দেখুন
tasks-account-fixing = কাজ চলছে…
tasks-list-name-placeholder = তালিকার নাম

## Lists and tasks

tasks-loading = আপনার টাস্ক পড়া হচ্ছে…
tasks-no-lists = আপনার টাস্ক তালিকাগুলি এখানে দেখা যাবে।
tasks-search = টাস্ক খুঁজুন
tasks-search-none = আপনার অনুসন্ধানের সাথে কোনো টাস্ক মেলেনি।
tasks-add = টাস্ক যোগ করুন
tasks-title-placeholder = শিরোনাম
tasks-add-step = সাবটাস্ক যোগ করুন
tasks-empty = এখনও কোনো টাস্ক নেই। ওপরে একটি যোগ করুন।
tasks-starred-empty = এখানে দেখতে কোনো টাস্কে তারকাচিহ্ন দিন।
tasks-label-empty = এই লেবেলে কোনো খোলা টাস্ক নেই।
tasks-today-empty = আজকের জন্য কিছু নেই।
tasks-completed-empty = আপনি যে টাস্কগুলি সম্পন্ন করবেন সেগুলি এখানে দেখাবে।
tasks-upcoming-add = { $day }-এর জন্য একটি টাস্ক যোগ করুন
tasks-upcoming-overdue-day = { $weekday } { $day }
tasks-from-mail-quiet = মেল থেকে
tasks-from-note-quiet = নোট থেকে
tasks-steps-done = { $done }/{ $count }
tasks-today-date = { $weekday }, { $day }
tasks-overdue = মেয়াদোত্তীর্ণ
tasks-completed = { $count ->
    [one] সম্পন্ন ({ $count })
   *[other] সম্পন্ন ({ $count })
}
tasks-list-options = তালিকার বিকল্প
tasks-sort-by = সাজান
tasks-sort-my-order = আমার ক্রম
tasks-sort-date = তারিখ
tasks-sort-starred = সম্প্রতি তারকাচিহ্নিত
tasks-sort-title = শিরোনাম
tasks-rename-list = তালিকার নাম বদলান
tasks-delete-list = তালিকা মুছুন
tasks-mark-done = সম্পন্ন হিসেবে চিহ্নিত করুন
tasks-mark-open = অসম্পন্ন হিসেবে চিহ্নিত করুন
tasks-star = তারকাচিহ্ন দিন
tasks-unstar = তারকাচিহ্ন সরান
tasks-edit-title = শিরোনাম সম্পাদনা করুন
tasks-details = বিবরণ
tasks-delete = মুছুন
tasks-move-to = { $list }-এ সরান
tasks-from-mail = মেল
tasks-open-mail = মেল খুলুন
tasks-from-note = নোট
tasks-open-note = নোট খুলুন
tasks-note-gone = সেই নোটটি আর এখানে নেই।
tasks-no-subject = (কোনো বিষয় নেই)

## Several tasks selected (Ctrl+click, Shift+click)

tasks-selected = { $count ->
    [one] { $count }টি বেছে নেওয়া হয়েছে
   *[other] { $count }টি বেছে নেওয়া হয়েছে
}
tasks-select-clear = বাছাই মুছুন
tasks-select-move = তালিকায় সরান
tasks-select-date = তারিখ সেট করুন
tasks-next-week = আগামী সপ্তাহ

## The details dialog

tasks-notes-placeholder = বিবরণ যোগ করুন
tasks-date = তারিখ
tasks-no-date = কোনো তারিখ নেই
tasks-time-placeholder = সময় যোগ করুন
tasks-repeat = পুনরাবৃত্তি
tasks-repeat-never = পুনরাবৃত্তি হয় না
tasks-repeat-daily = প্রতিদিন
tasks-repeat-weekly = প্রতি সপ্তাহে
tasks-repeat-monthly = প্রতি মাসে
tasks-repeat-yearly = প্রতি বছর
tasks-repeat-other = কাস্টম
tasks-remind = আমাকে মনে করিয়ে দিন
tasks-remind-off = মনে করাবেন না
tasks-remind-on-time = ঠিক সময়ে
tasks-remind-morning = ওই দিন, { $time }
tasks-remind-hour-before = এক ঘণ্টা আগে
tasks-remind-day-before = একদিন আগে
tasks-label-add = লেবেল যোগ করুন
tasks-label-task = টাস্কে লেবেল দিন
tasks-files-attach = ফাইল সংযুক্ত করুন
tasks-files-pick = সংযুক্ত করুন
tasks-file-open = খুলুন
tasks-file-remove = ফাইল সরান
tasks-file-here = শুধু এই কম্পিউটারে
tasks-cancel = বাতিল করুন
tasks-save = সেভ করুন
tasks-not-a-time = “{ $text }” কোনো সময় নয়, যেমন { $example }।

## Due days

tasks-due-today = আজ
tasks-due-tomorrow = আগামীকাল
tasks-due-yesterday = গতকাল
tasks-due-at = { $day }, { $time }

## Notes at the bottom

tasks-toast-done = টাস্ক সম্পন্ন হয়েছে
tasks-toast-next = হয়ে গেছে। পরেরটি { $date } তারিখে
tasks-toast-deleted = টাস্ক মুছে ফেলা হয়েছে
tasks-files-added = { $count ->
    [one] ফাইল সংযুক্ত করা হয়েছে
   *[other] { $count }টি ফাইল সংযুক্ত করা হয়েছে
}
tasks-file-removed = “{ $name }” সরানো হয়েছে
tasks-files-left-out = সংযুক্ত করা হয়নি: { $names }। একটি টাস্কে { $limit } পর্যন্ত ফাইল রাখা যায়, ফোল্ডার নয়।
tasks-file-missing = সেই ফাইলটি আর এখানে নেই।
tasks-toast-added = { $count ->
    [one] টাস্কে যোগ করা হয়েছে
   *[other] { $count }টি টাস্ক যোগ করা হয়েছে
}
tasks-mail-gone = সেই মেলটি আর এখানে নেই।
tasks-toast-list-deleted = তালিকা মুছে ফেলা হয়েছে
tasks-toast-moved = { $list }-এ সরানো হয়েছে
# A task dragged to another place in its own list.
tasks-toast-placed = টাস্ক সরানো হয়েছে
tasks-toast-rescheduled = কাজের সময় বদলানো হয়েছে
tasks-toast-rescheduled-several = { $count ->
    [one] টাস্কের সময় বদলানো হয়েছে
   *[other] { $count }টি টাস্কের সময় বদলানো হয়েছে
}
tasks-toast-done-several = { $count ->
    [one] টাস্ক সম্পন্ন হয়েছে
   *[other] { $count }টি টাস্ক সম্পন্ন হয়েছে
}
tasks-toast-open-several = { $count ->
    [one] টাস্ক অসম্পন্ন হিসেবে চিহ্নিত করা হয়েছে
   *[other] { $count }টি টাস্ক অসম্পন্ন হিসেবে চিহ্নিত করা হয়েছে
}
tasks-toast-starred = { $count ->
    [one] টাস্কে তারকাচিহ্ন দেওয়া হয়েছে
   *[other] { $count }টি টাস্কে তারকাচিহ্ন দেওয়া হয়েছে
}
tasks-toast-unstarred = { $count ->
    [one] তারকাচিহ্ন সরানো হয়েছে
   *[other] { $count }টি টাস্ক থেকে তারকাচিহ্ন সরানো হয়েছে
}
tasks-toast-deleted-several = { $count ->
    [one] টাস্ক মুছে ফেলা হয়েছে
   *[other] { $count }টি টাস্ক মুছে ফেলা হয়েছে
}
