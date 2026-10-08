# Katna Mail, Assamese (অসমীয়া): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = সম্পৰ্কসমূহ
contacts-frequent = ঘনঘন
contacts-other = অন্য সম্পৰ্কসমূহ
contacts-other-about = আপুনি Gmail ৰ পৰা মেইল কৰা কিন্তু ছেভ নকৰা লোকসকল
contacts-other-email = ইমেইল পঠাওক
contacts-other-empty = অন্য সম্পৰ্ক নাই। আপুনি Gmail ৰ পৰা মেইল কৰা কিন্তু ছেভ নকৰা লোকসকল ইয়াত দেখা যায়।
contacts-other-allow = অন্য সম্পৰ্কসমূহ চাবলৈ আপোনাৰ Gmail একাউণ্টত পুনৰ ছাইন ইন কৰক আৰু Katna ক সেইবোৰ চাবলৈ অনুমতি দিয়ক।
contacts-labels = লেবেলসমূহ
contacts-label-options = লেবেলৰ বিকল্প
contacts-label-rename = লেবেলৰ নাম সলনি কৰক
contacts-label-email = সকলোকে মেইল পঠিয়াওক
contacts-label-delete = লেবেল মচক
contacts-label-new = নতুন লেবেল
contacts-label-name = লেবেলৰ নাম
contacts-label-button = লেবেল
contacts-label-menu = এইদৰে লেবেল কৰক:
contacts-label-added = { $name }ত যোগ কৰা হ'ল
contacts-label-removed = { $name }ৰ পৰা আঁতৰোৱা হ'ল
contacts-label-renamed = লেবেলৰ নাম সলনি কৰি { $name } কৰা হ'ল
contacts-label-deleted = লেবেল { $name } মচা হ'ল
contacts-label-no-email = এই লেবেলত কাৰো ইমেইল ঠিকনা নাই
# The column's list of mail accounts, each with the people saved in it.
contacts-accounts = একাউণ্টসমূহ
# The line under an account whose contacts did not come: why, and the one
# click that fixes it.
contacts-account-sign-in = সম্পৰ্কসমূহ দেখুৱাবলৈ পুনৰ ছাইন ইন কৰক
contacts-account-signed-in = { $address }ত পুনৰ ছাইন ইন কৰা হ'ল। আপোনাৰ সম্পৰ্কসমূহ অনা হৈছে…
contacts-account-sign-in-refused = { $provider }এ Katnaক সোমাবলৈ নিদিলে। পুনৰ চেষ্টা কৰক, আৰু আপোনাৰ সম্পৰ্কসমূহলৈ প্ৰৱেশৰ অনুমতি দিয়ক।
contacts-account-password = ছাৰ্ভাৰে পাছৱৰ্ডটো গ্ৰহণ নকৰিলে। Yahoo, iCloud, Zoho আৰু আনবোৰক এটা এপ পাছৱৰ্ড লাগে।
contacts-account-change-password = পাছৱৰ্ড সলনি কৰক
contacts-account-change-password-tooltip = নতুন পাছৱৰ্ড লিখক; Katnaই ছাৰ্ভাৰৰ সৈতে পৰীক্ষা কৰে
contacts-account-failed = সম্পৰ্কসমূহ পঢ়িব পৰা নগ'ল।
# $reason is the server's own words, in English.
contacts-account-error = সম্পৰ্কসমূহ পঢ়িব পৰা নগ'ল: { $reason }
contacts-account-none = কোনো ঠিকনা বহী পোৱা নগ'ল
# $reason is what the server answered, in English: "CardDAV https://dav.example.in/: status 404".
contacts-account-none-why = কোনো ঠিকনা বহী পোৱা নগ'ল: { $reason }
# A Gmail or Outlook account added with a password: its contacts need the
# provider's sign-in.
contacts-account-use-sign-in = { $provider }এ কেৱল { $provider }ৰে ছাইন ইন কৰা Katnaকহে সম্পৰ্কসমূহ দেখুৱায়।
contacts-account-sign-in-with = { $provider }ৰে ছাইন ইন কৰক
contacts-account-looking = সম্পৰ্ক বিচৰা হৈছে…
contacts-account-try-again = পুনৰ চেষ্টা কৰক
contacts-account-try-again-tooltip = এই একাউণ্টৰ সম্পৰ্কসমূহ এতিয়াই পুনৰ পৰীক্ষা কৰক
contacts-account-fixing = কাম চলি আছে…
contacts-manage = ঠিক কৰক আৰু পৰিচালনা কৰক
contacts-merge = মাৰ্জ কৰক আৰু ঠিক কৰক
contacts-merge-about = { $count ->
    [one] { $count }টা পৰামৰ্শ: একেজন ব্যক্তিৰ যেন লগা সম্পৰ্ক
   *[other] { $count }টা পৰামৰ্শ: একেজন ব্যক্তিৰ যেন লগা সম্পৰ্ক
}
contacts-merge-none = কোনো ডুপ্লিকেট নাই। একেই নাম বা ফোন নম্বৰ থকা সম্পৰ্ক ইয়াত দেখা যাব।
contacts-merge-count = { $count ->
    [one] { $count }টা সম্পৰ্ক
   *[other] { $count }টা সম্পৰ্ক
}
contacts-merge-all = সকলো মাৰ্জ কৰক
contacts-merge-button = মাৰ্জ কৰক
contacts-merge-dismiss = অগ্ৰাহ্য কৰক
contacts-merged = { $count ->
    [1] সম্পৰ্ক মাৰ্জ কৰা হ’ল
    [one] { $count }টা মাৰ্জ সম্পূৰ্ণ হ’ল
   *[other] { $count }টা মাৰ্জ সম্পূৰ্ণ হ’ল
}
contacts-import = আমদানি কৰক
contacts-export = ৰপ্তানি কৰক
contacts-import-file = vCard বা CSV ফাইলৰ পৰা সম্পৰ্ক আমদানি কৰক
contacts-imported = { $count ->
    [one] { $place }ত { $count }টা সম্পৰ্ক আমদানি কৰা হ’ল
   *[other] { $place }ত { $count }টা সম্পৰ্ক আমদানি কৰা হ’ল
}
contacts-imported-some = { $count ->
    [one] { $place }ত { $count }টা সম্পৰ্ক আমদানি কৰা হ’ল; আগতেই সংৰক্ষণ কৰা { $skipped }টা বাদ দিয়া হ’ল
   *[other] { $place }ত { $count }টা সম্পৰ্ক আমদানি কৰা হ’ল; আগতেই সংৰক্ষণ কৰা { $skipped }টা বাদ দিয়া হ’ল
}
contacts-import-none = { $name }ত কোনো সম্পৰ্ক পোৱা নগ’ল
contacts-import-all-saved = { $name }ৰ সকলোৱে আগতেই সংৰক্ষিত হৈ আছে
contacts-import-failed = { $name } পঢ়িব পৰা নগ’ল: { $error }
contacts-exported = { $count ->
    [one] { $path }ত { $count }টা সম্পৰ্ক ৰপ্তানি কৰা হ’ল
   *[other] { $path }ত { $count }টা সম্পৰ্ক ৰপ্তানি কৰা হ’ল
}
contacts-export-none = ৰপ্তানি কৰিবলৈ কোনো সম্পৰ্ক নাই
contacts-export-failed = সম্পৰ্ক ৰপ্তানি কৰিব পৰা নগ’ল: { $error }
contacts-print = প্ৰিণ্ট কৰক
contacts-print-title = সম্পৰ্কসমূহ
contacts-print-none = প্ৰিণ্ট কৰিবলৈ কোনো সম্পৰ্ক নাই
contacts-print-typed = { $value } ({ $kind })
contacts-print-birthday = জন্মদিন: { $day }
contacts-print-nickname = ডাকনাম: { $name }
contacts-create = নতুন সম্পৰ্ক

## Search and the list

contacts-search = সম্পৰ্ক সন্ধান কৰক
contacts-loading = সম্পৰ্কসমূহ লোড হৈ আছে…
contacts-empty = এতিয়ালৈকে কোনো সংৰক্ষণ কৰা সম্পৰ্ক নাই। Gmail, Outlook বা আপোনাৰ মেইল সেৱাত সংৰক্ষণ কৰা সম্পৰ্কসমূহ ইয়াত দেখা যায়।
contacts-empty-no-books = আপোনাৰ একাউণ্টসমূহৰ সম্পৰ্কসমূহ ছিংক হ’লেই ইয়াত দেখা যাব।
contacts-none-found = আপোনাৰ সন্ধানৰ লগত কোনো সম্পৰ্ক নিমিলে।
contacts-starred = { $count ->
    [one] তৰাচিহ্নিত সম্পৰ্ক ({ $count })
   *[other] তৰাচিহ্নিত সম্পৰ্ক ({ $count })
}
contacts-count = সম্পৰ্কসমূহ ({ $count })
contacts-col-name = নাম
contacts-col-email = ইমেইল
contacts-col-phone = ফোন নম্বৰ
contacts-col-job = পদবী আৰু কোম্পানী
contacts-col-labels = লেবেলসমূহ

## Asking to allow contacts, for accounts signed in before Katna read them

contacts-allow = Katna-ক { $address }ৰ সম্পৰ্কসমূহ পঢ়িবলৈ অনুমতি দিয়ক।
contacts-allow-many = { $more ->
    [one] Katna-ক { $address } আৰু আৰু { $more }টা একাউণ্টৰ সম্পৰ্কসমূহ পঢ়িবলৈ অনুমতি দিয়ক।
   *[other] Katna-ক { $address } আৰু আৰু { $more }টা একাউণ্টৰ সম্পৰ্কসমূহ পঢ়িবলৈ অনুমতি দিয়ক।
}
contacts-allow-button = অনুমতি দিয়ক

## A contact's page

contacts-back = সম্পৰ্কসমূহলৈ উভতি যাওক
contacts-edit = সম্পাদনা কৰক
contacts-delete = মচক
contacts-qr = QR ক’ড হিচাপে শ্বেয়াৰ কৰক
contacts-qr-about = সম্পৰ্কটো ছেভ কৰিবলৈ ফোনৰ কেমেৰাৰে এইটো স্কেন কৰক।
contacts-qr-too-long = QR ক’ডত ধৰিবলৈ এই সম্পৰ্কত বহুত বেছি তথ্য আছে।
contacts-qr-done = হ’ল
contacts-deleted = { $name } মচা হ'ল
contacts-added = { $name }ক সম্পৰ্কত যোগ কৰা হ'ল
contacts-find-mail = মেইল
contacts-details = সম্পৰ্কৰ বিৱৰণ
contacts-saved-in = ইয়াত সংৰক্ষণ কৰা হৈছে
contacts-notes = টোকাসমূহ
contacts-birthday = জন্মদিন
contacts-nickname = ডাকনাম
contacts-this-computer = এই কম্পিউটাৰ
contacts-kind-home = ঘৰ
contacts-kind-work = কৰ্মস্থান
contacts-kind-mobile = মোবাইল
contacts-kind-other = অন্য
contacts-source-google = Google সম্পৰ্কসমূহ
contacts-source-microsoft = Outlook সম্পৰ্কসমূহ
contacts-source-carddav = CardDAV

## Creating and changing a contact

contacts-edit-new-title = সম্পৰ্ক সৃষ্টি কৰক
contacts-edit-title = সম্পৰ্ক সম্পাদনা কৰক
contacts-edit-save = ছেভ কৰক
contacts-edit-saving = ছেভ কৰি আছে…
contacts-edit-cancel = বাতিল কৰক
contacts-saved = সম্পৰ্ক ছেভ কৰা হ'ল
contacts-edit-save-to = ইয়াত ছেভ কৰক
contacts-edit-changes-go-to = সলনিসমূহ { $place }ত ছেভ কৰা হয়।
contacts-edit-given = প্ৰথম নাম
contacts-edit-family = অন্তিম নাম
contacts-edit-company = কোম্পানী
contacts-edit-job = পদবী
contacts-edit-email = ইমেইল
contacts-edit-phone = ফোন
contacts-edit-with-kind = { $field } ({ $kind })
contacts-edit-add-email = ইমেইল যোগ কৰক
contacts-edit-add-phone = ফোন যোগ কৰক
contacts-edit-street = ৰাস্তাৰ ঠিকনা
contacts-edit-city = নগৰ
contacts-edit-postcode = ডাক কোড
contacts-edit-country = দেশ
contacts-edit-birthday = জন্মদিন (YYYY-MM-DD)
contacts-edit-empty = প্ৰথমে এটা নাম, ইমেইল বা ফোন নম্বৰ যোগ কৰক।
