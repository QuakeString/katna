# Katna Mail, Bengali (বাংলা): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = পরিচিতি
contacts-frequent = ঘন ঘন
contacts-other = অন্যান্য পরিচিতি
contacts-other-about = যাঁদের আপনি Gmail থেকে মেল পাঠিয়েছেন কিন্তু সেভ করেননি
contacts-other-email = ইমেল পাঠান
contacts-other-empty = অন্য কোনো পরিচিতি নেই। Gmail থেকে যাঁদের মেল পাঠান কিন্তু সেভ করেন না, তাঁরা এখানে দেখা যায়।
contacts-other-allow = অন্যান্য পরিচিতি দেখতে আপনার Gmail অ্যাকাউন্টে আবার সাইন ইন করুন এবং Katna-কে সেগুলি দেখার অনুমতি দিন।
contacts-labels = লেবেল
contacts-label-options = লেবেল বিকল্প
contacts-label-rename = লেবেলের নাম পরিবর্তন করুন
contacts-label-email = সবাইকে মেল পাঠান
contacts-label-delete = লেবেল মুছুন
contacts-label-new = নতুন লেবেল
contacts-label-name = লেবেলের নাম
contacts-label-button = লেবেল
contacts-label-menu = এইভাবে লেবেল দিন:
contacts-label-added = { $name }-এ যোগ করা হয়েছে
contacts-label-removed = { $name } থেকে সরানো হয়েছে
contacts-label-renamed = লেবেলের নাম পাল্টে { $name } করা হয়েছে
contacts-label-deleted = লেবেল { $name } মুছে ফেলা হয়েছে
contacts-label-no-email = এই লেবেলে কারও ইমেল ঠিকানা নেই
# The column's list of mail accounts, each with the people saved in it.
contacts-accounts = অ্যাকাউন্ট
# The line under an account whose contacts did not come: why, and the one
# click that fixes it.
contacts-account-sign-in = পরিচিতি দেখাতে আবার সাইন ইন করুন
contacts-account-signed-in = { $address }-এ আবার সাইন ইন করা হয়েছে। আপনার পরিচিতিগুলো আনা হচ্ছে…
contacts-account-sign-in-refused = { $provider } Katna-কে ঢুকতে দেয়নি। আবার চেষ্টা করুন, এবং আপনার পরিচিতিতে অ্যাক্সেসের অনুমতি দিন।
contacts-account-password = সার্ভার পাসওয়ার্ডটি গ্রহণ করেনি। Yahoo, iCloud, Zoho ও অন্যদের জন্য একটি অ্যাপ পাসওয়ার্ড লাগে।
contacts-account-change-password = পাসওয়ার্ড বদলান
contacts-account-change-password-tooltip = সেটিংস > অ্যাকাউন্ট খুলুন
contacts-account-failed = পরিচিতিগুলো পড়া যায়নি।
# $reason is the server's own words, in English.
contacts-account-error = পরিচিতিগুলো পড়া যায়নি: { $reason }
contacts-account-none = কোনো ঠিকানা বই পাওয়া যায়নি
contacts-account-looking = পরিচিতি খোঁজা হচ্ছে…
contacts-account-try-again = আবার চেষ্টা করুন
contacts-account-try-again-tooltip = এই অ্যাকাউন্টের পরিচিতিগুলো এখনই আবার দেখুন
contacts-account-fixing = কাজ চলছে…
contacts-manage = ঠিক করুন এবং পরিচালনা করুন
contacts-merge = মার্জ করুন ও ঠিক করুন
contacts-merge-about = { $count ->
    [one] { $count }টি পরামর্শ: যে পরিচিতিগুলি একই ব্যক্তির বলে মনে হচ্ছে
   *[other] { $count }টি পরামর্শ: যে পরিচিতিগুলি একই ব্যক্তির বলে মনে হচ্ছে
}
contacts-merge-none = কোনো ডুপ্লিকেট নেই। একই নাম বা ফোন নম্বরের পরিচিতি এখানে দেখা যাবে।
contacts-merge-count = { $count ->
    [one] { $count }টি পরিচিতি
   *[other] { $count }টি পরিচিতি
}
contacts-merge-all = সব মার্জ করুন
contacts-merge-button = মার্জ করুন
contacts-merge-dismiss = বাতিল করুন
contacts-merged = { $count ->
    [1] পরিচিতি মার্জ হয়েছে
    [one] { $count }টি মার্জ সম্পন্ন
   *[other] { $count }টি মার্জ সম্পন্ন
}
contacts-import = ইমপোর্ট করুন
contacts-export = এক্সপোর্ট করুন
contacts-import-file = vCard বা CSV ফাইল থেকে পরিচিতি ইমপোর্ট করুন
contacts-imported = { $count ->
    [one] { $place }-এ { $count }টি পরিচিতি ইমপোর্ট করা হয়েছে
   *[other] { $place }-এ { $count }টি পরিচিতি ইমপোর্ট করা হয়েছে
}
contacts-imported-some = { $count ->
    [one] { $place }-এ { $count }টি পরিচিতি ইমপোর্ট করা হয়েছে; আগে থেকেই সংরক্ষিত থাকায় { $skipped }টি বাদ দেওয়া হয়েছে
   *[other] { $place }-এ { $count }টি পরিচিতি ইমপোর্ট করা হয়েছে; আগে থেকেই সংরক্ষিত থাকায় { $skipped }টি বাদ দেওয়া হয়েছে
}
contacts-import-none = { $name }-এ কোনো পরিচিতি পাওয়া যায়নি
contacts-import-all-saved = { $name }-এর সবাই আগে থেকেই সংরক্ষিত আছেন
contacts-import-failed = { $name } পড়া যায়নি: { $error }
contacts-exported = { $count ->
    [one] { $path }-এ { $count }টি পরিচিতি এক্সপোর্ট করা হয়েছে
   *[other] { $path }-এ { $count }টি পরিচিতি এক্সপোর্ট করা হয়েছে
}
contacts-export-none = এক্সপোর্ট করার মতো কোনো পরিচিতি নেই
contacts-export-failed = পরিচিতি এক্সপোর্ট করা যায়নি: { $error }
contacts-print = প্রিন্ট করুন
contacts-print-title = পরিচিতি
contacts-print-none = প্রিন্ট করার মতো কোনো পরিচিতি নেই
contacts-print-typed = { $value } ({ $kind })
contacts-print-birthday = জন্মদিন: { $day }
contacts-print-nickname = ডাকনাম: { $name }
contacts-create = পরিচিতি তৈরি করুন

## Search and the list

contacts-search = পরিচিতি খুঁজুন
contacts-loading = পরিচিতি লোড হচ্ছে…
contacts-empty = এখনও কোনো সেভ করা পরিচিতি নেই। Gmail, Outlook বা আপনার মেল পরিষেবায় সেভ করা পরিচিতি এখানে দেখা যায়।
contacts-empty-no-books = আপনার অ্যাকাউন্টগুলির পরিচিতি সিঙ্ক হলেই এখানে দেখা যাবে।
contacts-none-found = আপনার অনুসন্ধানের সাথে কোনো পরিচিতি মেলেনি।
contacts-starred = { $count ->
    [one] তারকাচিহ্নিত পরিচিতি ({ $count })
   *[other] তারকাচিহ্নিত পরিচিতি ({ $count })
}
contacts-count = পরিচিতি ({ $count })
contacts-col-name = নাম
contacts-col-email = ইমেল
contacts-col-phone = ফোন নম্বর
contacts-col-job = পদবি ও কোম্পানি
contacts-col-labels = লেবেল

## Asking to allow contacts, for accounts signed in before Katna read them

contacts-allow = Katna-কে { $address }-এর পরিচিতি পড়ার অনুমতি দিন।
contacts-allow-many = { $more ->
    [one] Katna-কে { $address } এবং আরও { $more }টি অ্যাকাউন্টের পরিচিতি পড়ার অনুমতি দিন।
   *[other] Katna-কে { $address } এবং আরও { $more }টি অ্যাকাউন্টের পরিচিতি পড়ার অনুমতি দিন।
}
contacts-allow-button = অনুমতি দিন

## A contact's page

contacts-back = পরিচিতিতে ফিরে যান
contacts-edit = সম্পাদনা করুন
contacts-delete = মুছুন
contacts-qr = QR কোড হিসেবে শেয়ার করুন
contacts-qr-about = পরিচিতিটি সংরক্ষণ করতে ফোনের ক্যামেরা দিয়ে এটি স্ক্যান করুন।
contacts-qr-too-long = QR কোডে ধরানোর জন্য এই পরিচিতিতে খুব বেশি তথ্য আছে।
contacts-qr-done = হয়েছে
contacts-deleted = { $name } মুছে ফেলা হয়েছে
contacts-added = { $name }-কে পরিচিতিতে যোগ করা হয়েছে
contacts-find-mail = মেল
contacts-details = যোগাযোগের বিবরণ
contacts-saved-in = যেখানে সেভ করা
contacts-notes = নোট
contacts-birthday = জন্মদিন
contacts-nickname = ডাকনাম
contacts-this-computer = এই কম্পিউটার
contacts-kind-home = বাড়ি
contacts-kind-work = কর্মস্থল
contacts-kind-mobile = মোবাইল
contacts-kind-other = অন্যান্য
contacts-source-google = Google পরিচিতি
contacts-source-microsoft = Outlook পরিচিতি
contacts-source-carddav = CardDAV

## Creating and changing a contact

contacts-edit-new-title = পরিচিতি তৈরি করুন
contacts-edit-title = পরিচিতি সম্পাদনা করুন
contacts-edit-save = সেভ করুন
contacts-edit-saving = সেভ হচ্ছে…
contacts-edit-cancel = বাতিল করুন
contacts-saved = পরিচিতি সেভ করা হয়েছে
contacts-edit-save-to = এখানে সেভ করুন
contacts-edit-changes-go-to = পরিবর্তনগুলি { $place }-এ সেভ করা হবে।
contacts-edit-given = প্রথম নাম
contacts-edit-family = শেষ নাম
contacts-edit-company = কোম্পানি
contacts-edit-job = পদবি
contacts-edit-email = ইমেল
contacts-edit-phone = ফোন
contacts-edit-with-kind = { $field } ({ $kind })
contacts-edit-add-email = ইমেল যোগ করুন
contacts-edit-add-phone = ফোন যোগ করুন
contacts-edit-street = রাস্তার ঠিকানা
contacts-edit-city = শহর
contacts-edit-postcode = পোস্টাল কোড
contacts-edit-country = দেশ
contacts-edit-birthday = জন্মদিন (YYYY-MM-DD)
contacts-edit-empty = আগে একটি নাম, ইমেল বা ফোন নম্বর যোগ করুন।
