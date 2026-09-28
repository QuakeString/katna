# Katna Mail, Assamese (অসমীয়া).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = মেইল একাউণ্ট যোগ কৰক
add-account-looking = { $address }ৰ মেইল ছাৰ্ভাৰ বিচাৰি থকা হৈছে…
add-account-address-intro = আপোনাৰ ইমেইল ঠিকনা লিখক। Katnaই আপোনাৰ বাবে ছাৰ্ভাৰবোৰ বিচাৰি উলিয়াব।
add-account-servers-title = ছাৰ্ভাৰৰ ছেটিংছ
add-account-servers-intro = Katnaই ক'ৰ পৰা { $address }ৰ মেইল পঢ়ে আৰু পঠিয়ায়।
add-account-password-title = আপোনাৰ পাছৱৰ্ড লিখক
add-account-signing-in = ছাইন ইন কৰি থকা হৈছে…
add-account-browser-title = আপোনাৰ ব্ৰাউজাৰত আগবাঢ়ক
add-account-browser-intro = Katnaই আপোনাৰ ব্ৰাউজাৰত { $provider }ৰ ছাইন-ইন পৃষ্ঠাখন খুলিছে। তাত ছাইন ইন কৰক আৰু Katnaক আপোনাৰ মেইল পঢ়িবলৈ আৰু পঠিয়াবলৈ অনুমতি দিয়ক, তাৰ পিছত ইয়ালৈ উভতি আহক।
add-account-browser-hint = কোনো পৃষ্ঠা খোলা নাই নেকি? আপোনাৰ ব্ৰাউজাৰৰ উইণ্ড'বোৰ চাওক, নতুবা উভতি গৈ পুনৰ চেষ্টা কৰক।

## Add a mail account: fields

add-account-field-address = ইমেইল ঠিকনা
add-account-incoming = অহা মেইল ({ $protocol })
add-account-outgoing = যোৱা মেইল ({ $protocol })
add-account-field-server = ছাৰ্ভাৰ
add-account-field-port = প'ৰ্ট
add-account-security-none = নাই
add-account-field-username = ব্যৱহাৰকাৰীৰ নাম
add-account-field-password = পাছৱৰ্ড
add-account-show-password = পাছৱৰ্ড দেখুৱাওক
add-account-app-password-hint = ইয়াত { $provider }ক এটা এপ পাছৱৰ্ড লাগে, ৱেবত ব্যৱহাৰ কৰা পাছৱৰ্ডটো নহয়। আপোনাৰ { $provider } একাউণ্টৰ সুৰক্ষা ছেটিংছত এটা বনাওক।
add-account-field-name = আপোনাৰ নাম (ঐচ্ছিক)
add-account-name-hint = আপুনি লিখা লোকসকলক দেখুওৱা হয়।
add-account-servers-pair = { $imap } আৰু { $smtp }
add-account-servers-found = { $source ->
    [built-in] ছাৰ্ভাৰ: { $servers }, Katnaৰ প্ৰদানকাৰীৰ তালিকাত পোৱা গৈছে।
    [provider] ছাৰ্ভাৰ: { $servers }, আপোনাৰ প্ৰদানকাৰীৰ ছেটিংছত পোৱা গৈছে।
    [ispdb] ছাৰ্ভাৰ: { $servers }, Thunderbirdৰ প্ৰদানকাৰীৰ তালিকাত পোৱা গৈছে।
    [dns] ছাৰ্ভাৰ: { $servers }, আপোনাৰ ড'মেইনৰ DNS ৰেকৰ্ডত পোৱা গৈছে।
   *[other] ছাৰ্ভাৰ: { $servers }, অনুমান কৰা; ছাইন ইন বিফল হ'লে পৰীক্ষা কৰক।
}
add-account-servers-entered = ছাৰ্ভাৰ: { $servers }, যেনেকৈ লিখা হৈছে।
add-account-or = বা
add-account-sign-in-with = { $provider }ৰে ছাইন ইন কৰক
add-account-sign-in-instead = তাৰ সলনি { $provider }ৰে ছাইন ইন কৰক

## Add a mail account: buttons

add-account-servers-button = ছাৰ্ভাৰৰ ছেটিংছ
add-account-back = উভতি যাওক
add-account-add = একাউণ্ট যোগ কৰক
add-account-next = পৰৱৰ্তী
add-account-cancel = বাতিল কৰক

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] অহা মেইলৰ ছাৰ্ভাৰ লিখক।
   *[outgoing] যোৱা মেইলৰ ছাৰ্ভাৰ লিখক।
}
add-account-server-space = { $kind ->
    [incoming] অহা মেইলৰ ছাৰ্ভাৰৰ নামত এটা স্পেচ আছে।
   *[outgoing] যোৱা মেইলৰ ছাৰ্ভাৰৰ নামত এটা স্পেচ আছে।
}
add-account-port-invalid = { $kind ->
    [incoming] অহা মেইলৰ প'ৰ্ট { $min }ৰ পৰা { $max }লৈ এটা সংখ্যা হ'ব লাগিব।
   *[outgoing] যোৱা মেইলৰ প'ৰ্ট { $min }ৰ পৰা { $max }লৈ এটা সংখ্যা হ'ব লাগিব।
}
add-account-address-empty = এটা ইমেইল ঠিকনা লিখক।
add-account-address-invalid = { $example }ৰ দৰে এটা ইমেইল ঠিকনা লিখক।
add-account-not-found = Katnaই { $address }ৰ ছাৰ্ভাৰ বিচাৰি নাপালে, সেয়ে সাধাৰণ নামবোৰ ভৰাই দিলে। আপোনাৰ প্ৰদানকাৰীৰ সৈতে পৰীক্ষা কৰক।
add-account-password-empty = পাছৱৰ্ড লিখক।
add-account-name-is-password = নামটো পাছৱৰ্ডৰ সৈতে একে। তাৰ সলনি তাত আপোনাৰ নাম লিখক, মানুহে যিদৰে দেখা উচিত।
add-account-added = { $address } যোগ কৰা হ'ল। আপোনাৰ মেইল অনা হৈছে…
add-account-app-password-refused = { $provider }এ পাছৱৰ্ডটো গ্ৰহণ নকৰিলে। ইয়াক এটা এপ পাছৱৰ্ড লাগে, ৱেবত ব্যৱহাৰ কৰা পাছৱৰ্ডটো নহয়।
add-account-password-refused = ছাৰ্ভাৰে পাছৱৰ্ডটো গ্ৰহণ নকৰিলে। পৰীক্ষা কৰি পুনৰ চেষ্টা কৰক।
add-account-sign-in-refused = { $provider }এ Katnaক সোমাবলৈ নিদিলে। পুনৰ চেষ্টা কৰক, আৰু আপোনাৰ মেইললৈ প্ৰৱেশৰ অনুমতি দিয়ক।
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] Katnaৰ এই কপিটোৱে এতিয়াও Microsoft একাউণ্টত ছাইন ইন কৰিব নোৱাৰে।
    [Google] Katnaৰ এই কপিটোৱে এতিয়াও Google একাউণ্টত ছাইন ইন কৰিব নোৱাৰে।
   *[other] এই প্ৰদানকাৰীয়ে কেৱল নিজৰ পৃষ্ঠাতহে ছাইন ইন কৰিবলৈ দিয়ে, যিটো Katnaই ইয়াৰ বাবে এতিয়াও কৰিব নোৱাৰে।
}
add-account-signed-in = { $provider }ৰে ছাইন ইন কৰা হ'ল। আপোনাৰ মেইল অনা হৈছে…

## The account menu (from the account button on the top bar)

add-account-menu-another = আন এটা একাউণ্ট যোগ কৰক
add-account-menu-manage = একাউণ্টসমূহ পৰিচালনা কৰক
