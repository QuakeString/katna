# Katna Mail, Assamese (অসমীয়া).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = মেইল ছাৰ্ভাৰ

problems-signed-out = { $provider }এ Katna-ক { $address }ৰ পৰা ছাইন আউট কৰিলে। মেইল ছিংক বন্ধ হ'ল।
problems-password-refused = { $provider }এ { $address }ৰ পাছৱৰ্ডটো অগ্ৰাহ্য কৰিলে। হয়তো ই সলনি হৈছে।
problems-no-answer = { $provider }এ { $address }ৰ বাবে সঁহাৰি দিয়া নাই। Katna-এ চেষ্টা কৰি থাকে।
problems-offline = আপুনি অফলাইন আছে। আপোনাৰ মেইল ইয়াতে আছে, আৰু আপুনি পঠোৱা মেইল আপুনি পুনৰ অনলাইন নোহোৱালৈকে অপেক্ষা কৰে।
problems-accounts-need-you = { $count ->
    [one] 1টা একাউণ্টক আপোনাক লাগে
   *[other] { $count }টা একাউণ্টক আপোনাক লাগে
}
problems-show = দেখুৱাওক
problems-later = পিছত
problems-new-password = নতুন পাছৱৰ্ড
problems-try-again = পুনৰ চেষ্টা কৰক

## The New password card

problems-password-title = নতুন পাছৱৰ্ড
problems-password-detail = { $provider }এ { $address }ৰ ছেভ কৰা পাছৱৰ্ডটো অগ্ৰাহ্য কৰিলে। নতুনটো টাইপ কৰক; Katna-এ ৰখাৰ আগতে ইয়াক পৰীক্ষা কৰে।
problems-password-placeholder = পাছৱৰ্ড
problems-password-show = পাছৱৰ্ড দেখুৱাওক
problems-password-hide = পাছৱৰ্ড লুকুৱাওক
problems-password-cancel = বাতিল কৰক
problems-password-save = ছেভ কৰক
problems-password-checking = পৰীক্ষা কৰি আছে…
problems-password-refused-again = { $provider }এ এই পাছৱৰ্ডটোও অগ্ৰাহ্য কৰিলে। পৰীক্ষা কৰি পুনৰ চেষ্টা কৰক।
problems-password-saved = { $address }ৰ বাবে পাছৱৰ্ড ছেভ কৰা হ'ল। আপোনাৰ মেইল অনা হৈছে…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = { $address }ৰ মেইল ছাৰ্ভাৰে { $count ->
    [one] এটা বাৰ্তা স্থানান্তৰ কৰাটো গ্ৰহণ নকৰিলে, সেয়েহে ই আগৰ ঠাইলৈ ঘূৰি আহিছে।
   *[other] { $count }টা বাৰ্তা স্থানান্তৰ কৰাটো গ্ৰহণ নকৰিলে, সেয়েহে সেইবোৰ আগৰ ঠাইলৈ ঘূৰি আহিছে।
}
problems-refused-flags = { $address }ৰ মেইল ছাৰ্ভাৰে { $count ->
    [one] এটা বাৰ্তা চিহ্নিত (পঢ়া, তৰাচিহ্নিত…) কৰাটো গ্ৰহণ নকৰিলে, সেয়েহে ই আগৰ দৰে হৈছে।
   *[other] { $count }টা বাৰ্তা চিহ্নিত (পঢ়া, তৰাচিহ্নিত…) কৰাটো গ্ৰহণ নকৰিলে, সেয়েহে সেইবোৰ আগৰ দৰে হৈছে।
}
problems-refused-label = { $address }ৰ মেইল ছাৰ্ভাৰে { $count ->
    [one] এটা বাৰ্তাৰ লেবেল সলনি কৰাটো গ্ৰহণ নকৰিলে, সেয়েহে ই আগৰ দৰে হৈছে।
   *[other] { $count }টা বাৰ্তাৰ লেবেল সলনি কৰাটো গ্ৰহণ নকৰিলে, সেয়েহে সেইবোৰ আগৰ দৰে হৈছে।
}
problems-refused-delete = { $address }ৰ মেইল ছাৰ্ভাৰে { $count ->
    [one] এটা বাৰ্তা মচাটো গ্ৰহণ নকৰিলে, সেয়েহে ই ঘূৰি আহিছে।
   *[other] { $count }টা বাৰ্তা মচাটো গ্ৰহণ নকৰিলে, সেয়েহে সেইবোৰ ঘূৰি আহিছে।
}
problems-refused-other = { $address }ৰ মেইল ছাৰ্ভাৰে { $count ->
    [one] এটা সলনি গ্ৰহণ নকৰিলে, সেয়েহে Katna-এ ইয়াক আগৰ দৰে কৰি দিলে।
   *[other] { $count }টা সলনি গ্ৰহণ নকৰিলে, সেয়েহে Katna-এ সেইবোৰ আগৰ দৰে কৰি দিলে।
}
problems-details = বিৱৰণ

## Katna's background service (katna-daemon) isn't running

service-starting = Katna-ৰ পটভূমি সেৱা আৰম্ভ কৰা হৈছে…
service-failed = Katna-ৰ পটভূমি সেৱা আৰম্ভ হোৱা নাই, সেয়েহে মেইল ছিংক হোৱা নাই।
service-start-again = পুনৰ আৰম্ভ কৰক
service-started-again = Katna-ৰ পটভূমি সেৱা বন্ধ হৈছিল আৰু পুনৰ আৰম্ভ কৰা হ'ল।
service-details-title = সেৱাটো কিয় আৰম্ভ হোৱা নাই
service-details-body = এইখিনি কপি কৰি আপোনাৰ ৰিপৰ্টৰ সৈতে পঠিয়াওক। ইয়াত কোনো মেইল বা পাছৱৰ্ড নাই।
service-details-copy = কপি কৰক
service-details-close = বন্ধ কৰক
