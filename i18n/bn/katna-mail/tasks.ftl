# Katna Mail, Bengali (বাংলা): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = তৈরি করুন
tasks-all = সব টাস্ক
tasks-today = আজ
tasks-starred = তারকাচিহ্নিত
tasks-new-list = নতুন তালিকা তৈরি করুন
tasks-on-this-computer = এই কম্পিউটারে
tasks-my-tasks = আমার টাস্ক
tasks-list-name-placeholder = তালিকার নাম

## Lists and tasks

tasks-loading = আপনার টাস্ক পড়া হচ্ছে…
tasks-no-lists = আপনার টাস্ক তালিকাগুলি এখানে দেখা যাবে।
tasks-add = টাস্ক যোগ করুন
tasks-title-placeholder = শিরোনাম
tasks-add-step = সাবটাস্ক যোগ করুন
tasks-empty = এখনও কোনো টাস্ক নেই। ওপরে একটি যোগ করুন।
tasks-starred-empty = এখানে দেখতে কোনো টাস্কে তারকাচিহ্ন দিন।
tasks-today-empty = আজকের জন্য কিছু নেই।
tasks-today-date = { $weekday }, { $day }
tasks-overdue = মেয়াদোত্তীর্ণ
tasks-completed = { $count ->
    [one] সম্পন্ন ({ $count })
   *[other] সম্পন্ন ({ $count })
}
tasks-list-options = তালিকার বিকল্প
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
tasks-toast-added = { $count ->
    [one] টাস্কে যোগ করা হয়েছে
   *[other] { $count }টি টাস্ক যোগ করা হয়েছে
}
tasks-mail-gone = সেই মেলটি আর এখানে নেই।
tasks-toast-list-deleted = তালিকা মুছে ফেলা হয়েছে
tasks-toast-moved = { $list }-এ সরানো হয়েছে
tasks-toast-rescheduled = কাজের সময় বদলানো হয়েছে
