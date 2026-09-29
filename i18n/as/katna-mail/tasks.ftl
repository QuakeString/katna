# Katna Mail, Assamese (অসমীয়া): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = সৃষ্টি কৰক
tasks-all = সকলো কাৰ্য
tasks-today = আজি
tasks-starred = তৰাচিহ্নিত
tasks-new-list = নতুন তালিকা সৃষ্টি কৰক
tasks-on-this-computer = এই কমপিউটাৰত
tasks-my-tasks = মোৰ কাৰ্য
# The line under an account in the side list whose task lists could not
# come: why, and the one click that fixes it.
tasks-account-sign-in = কাৰ্যসমূহ দেখুৱাবলৈ পুনৰ ছাইন ইন কৰক
tasks-account-signed-in = { $address }ত পুনৰ ছাইন ইন কৰা হ'ল। আপোনাৰ কাৰ্যসমূহ অনা হৈছে…
tasks-account-sign-in-refused = { $provider }এ Katnaক সোমাবলৈ নিদিলে। পুনৰ চেষ্টা কৰক, আৰু আপোনাৰ কাৰ্যসমূহলৈ প্ৰৱেশৰ অনুমতি দিয়ক।
tasks-account-refused = ছাৰ্ভাৰে পাছৱৰ্ডটো গ্ৰহণ নকৰিলে। Yahoo, iCloud, Zoho আৰু আনবোৰক এটা এপ পাছৱৰ্ড লাগে।
tasks-account-change-password = পাছৱৰ্ড সলনি কৰক
tasks-account-change-password-tooltip = ছেটিংছ > একাউণ্টসমূহ খোলক
tasks-account-not-enabled = Katnaৰ বাবে কাৰ্য প্ৰৱেশ এতিয়াও অন কৰা হোৱা নাই।
tasks-account-failed = কাৰ্য তালিকাবোৰ পঢ়িব পৰা নগ'ল।
# $reason is the server's own words, in English.
tasks-account-error = কাৰ্য তালিকাবোৰ পঢ়িব পৰা নগ'ল: { $reason }
tasks-account-none = কোনো কাৰ্য তালিকা পোৱা নগ'ল
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
tasks-account-none-why = কোনো কাৰ্য তালিকা পোৱা নগ'ল: { $reason }
# A Gmail or Outlook account added with a password: its tasks need the
# provider's sign-in.
tasks-account-use-sign-in = { $provider }এ কেৱল { $provider }ৰে ছাইন ইন কৰা Katnaকহে কাৰ্যসমূহ দেখুৱায়।
tasks-account-sign-in-with = { $provider }ৰে ছাইন ইন কৰক
tasks-account-looking = কাৰ্য তালিকা বিচৰা হৈছে…
tasks-account-try-again = পুনৰ চেষ্টা কৰক
tasks-account-try-again-tooltip = এই একাউণ্টৰ কাৰ্যসমূহ এতিয়াই পুনৰ পৰীক্ষা কৰক
tasks-account-fixing = কাম চলি আছে…
tasks-list-name-placeholder = তালিকাৰ নাম

## Lists and tasks

tasks-loading = আপোনাৰ কাৰ্য পঢ়া হৈছে…
tasks-no-lists = আপোনাৰ কাৰ্য তালিকাবোৰ ইয়াত দেখা যাব।
tasks-search = কাৰ্য সন্ধান কৰক
tasks-search-none = আপোনাৰ সন্ধানৰ লগত কোনো কাৰ্য নিমিলে।
tasks-add = কাৰ্য যোগ কৰক
tasks-title-placeholder = শিৰোনাম
tasks-add-step = উপ-কাৰ্য যোগ কৰক
tasks-empty = এতিয়াও কোনো কাৰ্য নাই। ওপৰত এটা যোগ কৰক।
tasks-starred-empty = ইয়াত চাবলৈ কোনো কাৰ্যত তৰাচিহ্ন যোগ কৰক।
tasks-today-empty = আজিৰ বাবে একো নাই।
tasks-today-date = { $weekday }, { $day }
tasks-overdue = ম্যাদ পাৰ হোৱা
tasks-completed = { $count ->
    [one] সম্পূৰ্ণ ({ $count })
   *[other] সম্পূৰ্ণ ({ $count })
}
tasks-list-options = তালিকাৰ বিকল্প
tasks-rename-list = তালিকাৰ নাম সলনি কৰক
tasks-delete-list = তালিকা মচক
tasks-mark-done = সম্পূৰ্ণ বুলি চিহ্নিত কৰক
tasks-mark-open = অসম্পূৰ্ণ বুলি চিহ্নিত কৰক
tasks-star = তৰাচিহ্ন যোগ কৰক
tasks-unstar = তৰাচিহ্ন আঁতৰাওক
tasks-edit-title = শিৰোনাম সম্পাদনা কৰক
tasks-details = বিৱৰণ
tasks-delete = মচক
tasks-move-to = { $list }লৈ স্থানান্তৰ কৰক
tasks-from-mail = মেইল
tasks-open-mail = মেইল খোলক
tasks-from-note = টোকা
tasks-open-note = টোকা খোলক
tasks-note-gone = সেই টোকা আৰু ইয়াত নাই।
tasks-no-subject = (কোনো বিষয় নাই)

## The details dialog

tasks-notes-placeholder = বিৱৰণ যোগ কৰক
tasks-date = তাৰিখ
tasks-no-date = কোনো তাৰিখ নাই
tasks-time-placeholder = সময় যোগ কৰক
tasks-repeat = পুনৰাবৃত্তি
tasks-repeat-never = পুনৰাবৃত্তি নহয়
tasks-repeat-daily = দৈনিক
tasks-repeat-weekly = সাপ্তাহিক
tasks-repeat-monthly = মাহিলী
tasks-repeat-yearly = বাৰ্ষিক
tasks-repeat-other = কাষ্টম
tasks-remind = মোক সোঁৱৰাই দিয়ক
tasks-remind-off = সোঁৱৰাই নিদিব
tasks-remind-on-time = ঠিক সময়তে
tasks-remind-morning = সেইদিনা, { $time }
tasks-remind-hour-before = এঘণ্টা আগতে
tasks-remind-day-before = এদিন আগতে
tasks-cancel = বাতিল কৰক
tasks-save = ছেভ কৰক
tasks-not-a-time = “{ $text }” সময় নহয়, যেনে { $example }।

## Due days

tasks-due-today = আজি
tasks-due-tomorrow = কাইলৈ
tasks-due-yesterday = কালি
tasks-due-at = { $day }, { $time }

## Notes at the bottom

tasks-toast-done = কাৰ্য সম্পূৰ্ণ হ’ল
tasks-toast-next = হ’ল। পৰৱৰ্তীটো { $date } তাৰিখে
tasks-toast-deleted = কাৰ্য মচি পেলোৱা হ’ল
tasks-toast-added = { $count ->
    [one] কাৰ্যত যোগ কৰা হ’ল
   *[other] { $count }টা কাৰ্য যোগ কৰা হ’ল
}
tasks-mail-gone = সেই মেইলটো আৰু ইয়াত নাই।
tasks-toast-list-deleted = তালিকা মচি পেলোৱা হ’ল
tasks-toast-moved = { $list }লৈ স্থানান্তৰ কৰা হ’ল
tasks-toast-rescheduled = কামৰ সময় সলনি কৰা হ’ল
