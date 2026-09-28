# Katna Mail, Bengali (বাংলা).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Compose window: title bar

compose-new-message = নতুন মেসেজ
compose-restore = আগের আকারে ফেরান
compose-minimize = ছোট করুন
compose-exit-full-screen = পূর্ণ স্ক্রিন থেকে বেরোন
compose-open-window = নতুন উইন্ডোতে খুলুন
compose-save-close = সেভ করে বন্ধ করুন
compose-back-to-mail = মেল উইন্ডোতে ফিরে যান
compose-pop-out-reply = উত্তর আলাদা উইন্ডোতে খুলুন
compose-show-trimmed = ছাঁটা অংশ দেখান

## Recipients and subject

compose-to = প্রাপক
compose-cc = Cc
compose-bcc = Bcc
compose-from = প্রেরক
compose-from-choose = অন্য অ্যাকাউন্ট থেকে পাঠান
compose-recipients = প্রাপকেরা
compose-subject = বিষয়

## Sending (the notes at the bottom of the window)

compose-open-elsewhere = আগে খোলা মেসেজটি পাঠান বা বাতিল করুন।
compose-bad-address = “{ $address }” কোনো ইমেল ঠিকানা নয়।
compose-no-recipients = অন্তত একজন প্রাপক যোগ করুন।
compose-attachments-too-large = সংযুক্তিগুলির আকার { $size }; মেল সার্ভার সর্বোচ্চ { $limit } নেয়।
compose-no-account = মেল পাঠানোর জন্য একটি অ্যাকাউন্ট যোগ করুন।
compose-past-time = ভবিষ্যতের একটি সময় বেছে নিন।
compose-scheduling = শিডিউল করা হচ্ছে…
compose-sending = পাঠানো হচ্ছে…
compose-scheduled = { $when }-এ পাঠানোর জন্য শিডিউল করা হয়েছে
compose-sent-archived = পাঠানো ও আর্কাইভ করা হয়েছে
compose-sent = মেসেজ পাঠানো হয়েছে
compose-discarded = খসড়া বাতিল করা হয়েছে
compose-draft-saved = খসড়া সেভ করা হয়েছে
compose-draft-failed = খসড়া সেভ করা যায়নি: { $error }
compose-draft-not-opened = খসড়া খোলা যায়নি।

## Attachments

compose-picker-insert = ঢোকান
compose-picker-attach = সংযুক্ত করুন
compose-file-too-large = { $name } খুব বড়: একটি মেসেজে সর্বোচ্চ { $limit } রাখা যায়।
compose-attachment-size = ({ $size })
compose-remove-attachment = সংযুক্তি সরান
compose-attachments-total = { $count ->
    [one] { $count }টি ফাইল, { $size }
   *[other] { $count }টি ফাইল, { $size }
}
compose-drop-files = ফাইলগুলি এখানে ছাড়ুন
compose-drop-here = এখানে ছাড়ুন
compose-paste-keep-formatting = ফরম্যাটিং রাখুন
compose-paste-table = টেবিল
compose-paste-picture = ছবি
compose-paste-plain-text = সাধারণ টেক্সট
compose-paste-inline = টেক্সটের ভেতরে
compose-paste-attachment = সংযুক্তি

## Encryption and signing (the toggles by the recipients)

compose-encrypt = এনক্রিপ্ট করুন
compose-encrypted = এনক্রিপ্ট করা: শুধু প্রাপকেরা পড়তে পারবেন
compose-sign = স্বাক্ষর করুন
compose-signed = স্বাক্ষরিত: প্রাপকেরা যাচাই করতে পারবেন যে এটি আপনার পাঠানো
compose-track = খোলা ও ক্লিক ট্র্যাক করুন
compose-tracked = ট্র্যাক করা হচ্ছে: প্রত্যেক প্রাপক কখন এটি খোলেন বা কোনো লিঙ্ক খোলেন, আপনি দেখতে পাবেন
compose-track-unavailable = স্বাক্ষরিত, এনক্রিপ্ট করা ও সাধারণ টেক্সট মেল ট্র্যাক করা যায় না
compose-track-sign-in = খোলা ও ক্লিক ট্র্যাক করতে একটি Katna অ্যাকাউন্টে সাইন ইন করুন
compose-receipt = পঠিত রসিদ চান
compose-receipt-on = পঠিত রসিদ চাওয়া হয়েছে: প্রাপকের অ্যাপ হয়তো তাঁকে একটি পাঠাতে বলবে

## Spelling

spell-no-dictionary = { $language }-এর জন্য কোনো বানান অভিধান ইনস্টল করা নেই (যেমন hunspell-en_us)।
spell-dictionary-error = বানান অভিধান: { $error }

## Grammar checking (the right-click menu on a grammar mistake)

grammar-replace = “{ $words }”
grammar-add = “{ $words }” যোগ করুন
grammar-remove = “{ $words }” সরান
grammar-ignore = উপেক্ষা করুন

## Send checks (asked before a message goes out)

send-check-attachment-title = আপনি কি ফাইল সংযুক্ত করতে চেয়েছিলেন?
send-check-attachment-text = আপনি একটি সংযুক্তির কথা লিখেছেন, কিন্তু কিছুই সংযুক্ত নেই।
send-check-attach = একটি ফাইল সংযুক্ত করুন
send-check-subject-title = বিষয় ছাড়াই পাঠাবেন?
send-check-subject-text = এই মেসেজের কোনো বিষয় নেই।
send-check-add-subject = বিষয় যোগ করুন
send-check-send-anyway = তবুও পাঠান
recipient-not-valid = বৈধ ইমেল ঠিকানা নয়
recipient-show-address = ঠিকানা দেখান
recipient-remove = সরান
recipient-bad-title = ঠিকানাটি যাচাই করুন
recipient-bad-text = “{ $address }” কোনো বৈধ ইমেল ঠিকানা নয়। পাঠানোর আগে এটি ঠিক করুন বা সরিয়ে দিন।
recipient-bad-fix = ঠিক করুন
