# Katna Mail, Assamese (অসমীয়া).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = মেইল একাউণ্ট যোগ কৰক
add-account-providers-intro = আপোনাৰ মেইল প্ৰদানকাৰী বাছনি কৰক। বাকীখিনি Katnaই বিচাৰি উলিয়াব।
add-account-provider-other = অন্য মেইল
add-account-provider-other-detail = যিকোনো IMAP বা POP3 একাউণ্ট
add-account-provider-google-detail = Gmail আৰু Google Workspace
add-account-provider-microsoft-detail = Outlook আৰু Microsoft 365
add-account-provider-mail = { $provider } Mail
add-account-form-title = { $provider }ত ছাইন ইন কৰক
add-account-form-title-other = আপোনাৰ মেইল একাউণ্ট
add-account-form-intro = Katnaই আপোনাৰ পাছৱৰ্ড আপোনাৰ ছিষ্টেমৰ কীৰিঙত ৰাখে।
add-account-looking = { $address }ৰ মেইল ছাৰ্ভাৰ বিচাৰি থকা হৈছে…
add-account-address-intro = আপোনাৰ ইমেইল ঠিকনা লিখক। Katnaই আপোনাৰ বাবে ছাৰ্ভাৰবোৰ বিচাৰি উলিয়াব।
add-account-servers-title = ছাৰ্ভাৰৰ ছেটিংছ
add-account-servers-intro = Katnaই ক'ৰ পৰা { $address }ৰ মেইল পঢ়ে আৰু পঠিয়ায়।
add-account-signing-in = ছাইন ইন কৰি থকা হৈছে…
add-account-browser-title = আপোনাৰ ব্ৰাউজাৰত আগবাঢ়ক
add-account-browser-intro = Katnaই আপোনাৰ ব্ৰাউজাৰত { $provider }ৰ ছাইন-ইন পৃষ্ঠাখন খুলিছে। তাত ছাইন ইন কৰক আৰু Katnaক আপোনাৰ মেইল পঢ়িবলৈ আৰু পঠিয়াবলৈ অনুমতি দিয়ক, তাৰ পিছত ইয়ালৈ উভতি আহক।
add-account-browser-hint = কোনো পৃষ্ঠা খোলা নাই নেকি? আপোনাৰ ব্ৰাউজাৰৰ উইণ্ড'বোৰ চাওক, নতুবা উভতি গৈ পুনৰ চেষ্টা কৰক।
add-account-stage-browser = আপুনি ব্ৰাউজাৰত ছাইন ইন কৰালৈ অপেক্ষা কৰি আছে…
add-account-stage-signing-in-at = { $server }ত ছাইন ইন কৰি আছে…
add-account-help-app-password-link = এপ পাছৱৰ্ড কেনেকৈ বনাব
add-account-help-turn-on-imap = { $provider }ৰ ৱেব মেইলৰ ছেটিংছত IMAP আৰু POP3 এক্সেছ অন কৰিলেহে ই মেইল এপক সোমাবলৈ দিয়ে।
add-account-help-turn-on-imap-link = কেনেকৈ অন কৰিব

## Add a mail account: fields

add-account-field-address = ইমেইল ঠিকনা
add-account-receive-with = মেইল লাভ কৰক ইয়াৰ দ্বাৰা
add-account-imap-about = IMAPএ আপোনাৰ মেইল আৰু ফ'ল্ডাৰ ছাৰ্ভাৰত ৰাখে, প্ৰতিটো ডিভাইচত একে। সম্ভৱ হ'লে ইয়াকে বাছনি কৰক।
add-account-pop3-about = POP3এ আপোনাৰ মেইল এই কম্পিউটাৰলৈ ডাউনল'ড কৰে। ইয়াত পঢ়া বা স্থানান্তৰ কৰা মেইল ছাৰ্ভাৰত আৰু আপোনাৰ আন ডিভাইচত যেনেকৈ আছে তেনেকৈয়ে থাকে।
add-account-incoming = অহা মেইল ({ $protocol })
add-account-outgoing = যোৱা মেইল ({ $protocol })
add-account-field-server = ছাৰ্ভাৰ
add-account-field-port = প'ৰ্ট
add-account-security-none = নাই
add-account-security-none-warning = এনক্ৰিপ্ট কৰা হোৱা নাই: আপোনাৰ পাছৱৰ্ড আৰু মেইল বাটতে পঢ়িব পাৰি।
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
add-account-sign-in-with = { $provider }ৰে ছাইন ইন কৰক
add-account-sign-in-instead = তাৰ সলনি { $provider }ৰে ছাইন ইন কৰক

## Add a mail account: buttons

add-account-servers-button = ছাৰ্ভাৰৰ ছেটিংছ
add-account-back = উভতি যাওক
add-account-add = একাউণ্ট যোগ কৰক
add-account-done = হ'ল
add-account-another = আন এটা একাউণ্ট যোগ কৰক
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
add-account-app-password-refused = { $provider }এ পাছৱৰ্ডটো গ্ৰহণ নকৰিলে। ইয়াক এটা এপ পাছৱৰ্ড লাগে, ৱেবত ব্যৱহাৰ কৰা পাছৱৰ্ডটো নহয়।
add-account-password-refused = ছাৰ্ভাৰে পাছৱৰ্ডটো গ্ৰহণ নকৰিলে। পৰীক্ষা কৰি পুনৰ চেষ্টা কৰক।
add-account-sign-in-refused = { $provider }এ Katnaক সোমাবলৈ নিদিলে। পুনৰ চেষ্টা কৰক, আৰু আপোনাৰ মেইললৈ প্ৰৱেশৰ অনুমতি দিয়ক।
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] Katnaৰ এই কপিটোৱে এতিয়াও Microsoft একাউণ্টত ছাইন ইন কৰিব নোৱাৰে।
    [Google] Katnaৰ এই কপিটোৱে এতিয়াও Google একাউণ্টত ছাইন ইন কৰিব নোৱাৰে।
   *[other] এই প্ৰদানকাৰীয়ে কেৱল নিজৰ পৃষ্ঠাতহে ছাইন ইন কৰিবলৈ দিয়ে, যিটো Katnaই ইয়াৰ বাবে এতিয়াও কৰিব নোৱাৰে।
}
add-account-smtp-not-found = Katnaই আপোনাৰ মেইল ক'ৰ পৰা পঢ়িব বিচাৰি পালে, কিন্তু ক'লৈ পঠিয়াব বিচাৰি নাপালে। বহিৰ্গামী ছাৰ্ভাৰটো লিখক।

## Add a mail account: the last step

add-account-done-title = আপোনাৰ একাউণ্ট সাজু
add-account-done-intro = Katnaই এতিয়া আপোনাৰ মেইল আনি আছে। নতুন মেইল আহি পোৱাৰ লগে লগে দেখা যাব।
add-account-done-sign-in = ছাইন ইন
add-account-done-signed-in-with = { $provider }ৰ সৈতে, আপোনাৰ ব্ৰাউজাৰত
add-account-done-receiving = মেইল লাভ কৰা
add-account-done-sending = মেইল পঠোৱা
add-account-done-on-server = ছাৰ্ভাৰত থকা মেইল
add-account-done-kept = Katnaত আপুনি নমচালৈকে ৰখা হয়
add-account-done-pop3-hint = ছাৰ্ভাৰত থকা মেইলৰ কি হ'ব, ছেটিংছ > একাউণ্টসমূহত সলনি কৰক।
add-account-done-zoho-title = কাৰ্য আৰু কেলেণ্ডাৰ
add-account-done-zoho-about = Zohoএ এইবোৰ মেইলৰ পৰা পৃথককৈ ৰাখে। এইবোৰ Katnaলৈ আনিবলৈ এবাৰ Zohoৰে ছাইন ইন কৰক।
add-account-done-linked = কাৰ্য আৰু কেলেণ্ডাৰ সংযোগ কৰা হ'ল

## The account menu (from the account button on the top bar)

add-account-menu-another = আন এটা একাউণ্ট যোগ কৰক
app-menu = মুখ্য মেনু
app-menu-back = উভতি যাওক
