# Katna Mail, Assamese (অসমীয়া).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification

notify-new-emails = { $count }টা নতুন ইমেইল
notify-and-more = আৰু { $count }টা
notify-no-subject = (কোনো বিষয় নাই)
notify-unknown-sender = অজ্ঞাত প্ৰেৰক
notify-snooze-back = স্নুজৰ পৰা উভতি আহিল
notify-no-reply = এতিয়াও কোনো উত্তৰ নাই
notify-no-reply-to = “{ $subject }”ৰ উত্তৰ কোনেও দিয়া নাই।
notify-follow-up-sent = ফল'-আপ পঠিওৱা হ'ল
notify-follow-up-sent-to = “{ $subject }”ৰ উত্তৰ কোনেও দিয়া নাছিল, সেয়ে Katnaই ফল'-আপ পঠিয়ালে।
notify-follow-up-waiting = ফল'-আপ পঠিওৱা হোৱা নাই
notify-follow-up-waiting-to = এই কম্পিউটাৰ বন্ধ থকা সময়ত ইয়াৰ সময় হৈছিল। “{ $subject }” আপোনাৰ ইনবক্সলৈ উভতি আহিছে।
notify-tracking-opened = { $who }-এ { $subject } খুলিছে
notify-tracking-clicked = { $who }-এ { $subject }ত থকা এটা লিংকত ক্লিক কৰিছে
notify-update-ready = Katna Mail আপডেট কৰিব পাৰি
notify-update-ready-body = সংস্কৰণ { $version } ডাউনল'ড হৈ গৈছে। আপডেটে ইয়াক ইনষ্টল কৰে আৰু Katna Mail ৰিষ্টাৰ্ট কৰে।
notify-update = আপডেট

## Something needs the user, shown once per problem

notify-signed-out = পুনৰ ছাইন ইন কৰক
notify-signed-out-body = { $provider }এ Katnaক { $address }ৰ পৰা ছাইন আউট কৰিলে। মেইল ছিংক বন্ধ হৈছে।
notify-sign-in = ছাইন ইন কৰক
notify-password-refused = পাছৱৰ্ড অগ্ৰাহ্য হ'ল
notify-password-refused-body = মেইল ছাৰ্ভাৰে { $address }ৰ পাছৱৰ্ড গ্ৰহণ নকৰিলে। ই সলনি হ'ব পাৰে।
notify-new-password = নতুন পাছৱৰ্ড
notify-not-sent = “{ $subject }” পঠিওৱা নহ'ল
notify-not-sent-no-subject = এটা বাৰ্তা পঠিওৱা নহ'ল
notify-not-sent-body = ই আউটবক্সত আছে, তাত কাৰণ লিখা আছে।
notify-open-outbox = আউটবক্স খোলক
notify-event-now = এতিয়া
notify-event-in-minutes = { $count ->
    [one] { $count } মিনিটত
   *[other] { $count } মিনিটত
}
notify-event-in-hours = { $count ->
    [one] { $count } ঘণ্টাত
   *[other] { $count } ঘণ্টাত
}
notify-event-in-days = { $count ->
    [1] কাইলৈ
    [one] { $count } দিনত
   *[other] { $count } দিনত
}
notify-event-all-day = গোটেই দিনটো
notify-event-join = যোগদান কৰক
notify-event-snooze = 5 মিনিট স্নুজ কৰক
notify-task-done = সম্পূৰ্ণ বুলি চিহ্নিত কৰক

## Its buttons

notify-open = খোলক
notify-peek = এবাৰ চাওক
notify-reply = উত্তৰ দিয়ক
notify-reply-placeholder = { $name }ক উত্তৰ দিয়ক…
notify-send = পঠিয়াওক
notify-reply-quote-header = { $date } তাৰিখে { $from }-এ লিখিছিল:
notify-reply-quote-header-no-date = { $from }-এ লিখিছিল:
notify-reply-all = সকলোকে উত্তৰ দিয়ক
notify-mark-read = পঢ়া বুলি চিহ্নিত কৰক
notify-mark-all-read = সকলোবোৰ পঢ়া বুলি চিহ্নিত কৰক
notify-archive = আৰ্কাইভ কৰক
notify-snooze-hour = 1 ঘণ্টা স্নুজ কৰক
notify-snooze-tomorrow = কাইলৈ
notify-copy-code = { $code } কপি কৰক
notify-link-verify = { $domain }ত সত্যাপন কৰক
notify-link-confirm = { $domain }ত নিশ্চিত কৰক
notify-link-activate = { $domain }ত সক্ৰিয় কৰক

## After Archive on a notification: a short note in the same place

notify-archived = আৰ্কাইভ কৰা হ'ল
notify-archived-count = { $count ->
    [one] { $count }টা বাৰ্তা ইনবক্সৰ পৰা আঁতৰোৱা হ'ল
   *[other] { $count }টা বাৰ্তা ইনবক্সৰ পৰা আঁতৰোৱা হ'ল
}
notify-undo = আনডু কৰক

## After Copy on a notification: a short note, with the code under the title

notify-code-copied = ক'ড কপি কৰা হ'ল
notify-code-not-copied = ক'ডটো কপি কৰিব পৰা নগ'ল

## After a reply typed into a notification: a note in the same place while
## it waits for the undo time

notify-reply-sent = { $name }লৈ উত্তৰ পঠিওৱা হ'ল
notify-open-in-katna = Katnaত খোলক
