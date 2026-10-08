# Katna Mail, Bengali (বাংলা).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification

notify-new-emails = { $count }টি নতুন ইমেল
notify-and-more = আরও { $count }টি
notify-no-subject = (কোনো বিষয় নেই)
notify-unknown-sender = অজানা প্রেরক
notify-snooze-back = স্নুজ থেকে ফিরে এসেছে
notify-no-reply = এখনও কোনো উত্তর নেই
notify-no-reply-to = “{ $subject }”-এর উত্তর কেউ দেয়নি।
notify-follow-up-sent = ফলো-আপ পাঠানো হয়েছে
notify-follow-up-sent-to = “{ $subject }”-এর উত্তর কেউ দেয়নি, তাই Katna ফলো-আপ পাঠিয়েছে।
notify-follow-up-waiting = ফলো-আপ পাঠানো হয়নি
notify-follow-up-waiting-to = এই কম্পিউটার বন্ধ থাকার সময় এটি পাঠানোর কথা ছিল। “{ $subject }” আবার আপনার ইনবক্সে আছে।
notify-tracking-opened = { $who } { $subject } খুলেছেন
notify-tracking-clicked = { $who } { $subject }-এর একটি লিঙ্কে ক্লিক করেছেন
notify-update-ready = Katna Mail আপডেট করা যেতে পারে
notify-update-ready-body = সংস্করণ { $version } ডাউনলোড হয়ে গেছে। আপডেট সেটি ইনস্টল করে এবং Katna Mail রিস্টার্ট করে।
notify-update = আপডেট

## Something needs the user, shown once per problem

notify-signed-out = আবার সাইন ইন করুন
notify-signed-out-body = { $provider } { $address } থেকে Katna-কে সাইন আউট করেছে। মেল সিঙ্ক বন্ধ হয়ে গেছে।
notify-sign-in = সাইন ইন করুন
notify-password-refused = পাসওয়ার্ড গ্রহণ করা হয়নি
notify-password-refused-body = মেল সার্ভার { $address }-এর পাসওয়ার্ড গ্রহণ করেনি। সেটি হয়তো বদলে গেছে।
notify-new-password = নতুন পাসওয়ার্ড
notify-not-sent = “{ $subject }” পাঠানো যায়নি
notify-not-sent-no-subject = একটি মেসেজ পাঠানো যায়নি
notify-not-sent-body = এটি আউটবক্সে আছে, সেখানে কারণ লেখা আছে।
notify-open-outbox = আউটবক্স খুলুন
notify-event-now = এখনই
notify-event-in-minutes = { $count ->
    [one] { $count } মিনিট পরে
   *[other] { $count } মিনিট পরে
}
notify-event-in-hours = { $count ->
    [one] { $count } ঘণ্টা পরে
   *[other] { $count } ঘণ্টা পরে
}
notify-event-in-days = { $count ->
    [1] আগামীকাল
    [one] { $count } দিন পরে
   *[other] { $count } দিন পরে
}
notify-event-all-day = সারাদিন
notify-event-join = যোগ দিন
notify-event-snooze = 5 মিনিট স্নুজ করুন
notify-task-done = সম্পন্ন হিসেবে চিহ্নিত করুন

## Its buttons

notify-open = খুলুন
notify-peek = এক ঝলক দেখুন
notify-reply = উত্তর দিন
notify-reply-placeholder = { $name }-কে উত্তর দিন…
notify-send = পাঠান
notify-reply-all = সবাইকে উত্তর দিন
notify-mark-read = পঠিত হিসেবে চিহ্নিত করুন
notify-mark-all-read = সবগুলি পঠিত হিসেবে চিহ্নিত করুন
notify-archive = আর্কাইভ করুন
notify-snooze-hour = 1 ঘণ্টা স্নুজ করুন
notify-snooze-tomorrow = আগামীকাল
notify-copy-code = { $code } কপি করুন
notify-link-verify = { $domain }-এ যাচাই করুন
notify-link-confirm = { $domain }-এ নিশ্চিত করুন
notify-link-activate = { $domain }-এ চালু করুন

## After Archive on a notification: a short note in the same place

notify-archived = আর্কাইভ করা হয়েছে
notify-archived-count = { $count ->
    [one] { $count }টি মেসেজ ইনবক্স থেকে সরানো হয়েছে
   *[other] { $count }টি মেসেজ ইনবক্স থেকে সরানো হয়েছে
}
notify-undo = পূর্বাবস্থায় ফেরান

## After Copy on a notification: a short note, with the code under the title

notify-code-copied = কোড কপি করা হয়েছে
notify-code-not-copied = কোডটি কপি করা যায়নি

## After a reply typed into a notification: a note in the same place while
## it waits for the undo time

notify-reply-sent = { $name }-কে উত্তর পাঠানো হয়েছে
notify-open-in-katna = Katna-তে খুলুন
