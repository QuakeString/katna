# Katna Mail, Bengali (বাংলা).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = মেল অ্যাকাউন্ট যোগ করুন
add-account-providers-intro = আপনার মেল প্রদানকারী বেছে নিন। বাকিটা Katna খুঁজে নেবে।
add-account-provider-other = অন্য মেল
add-account-provider-other-detail = যেকোনো IMAP বা POP3 অ্যাকাউন্ট
add-account-provider-google-detail = Gmail ও Google Workspace
add-account-provider-microsoft-detail = Outlook ও Microsoft 365
add-account-provider-mail = { $provider } Mail
add-account-form-title = { $provider }-এ সাইন ইন করুন
add-account-form-title-other = আপনার মেল অ্যাকাউন্ট
add-account-form-intro = Katna আপনার পাসওয়ার্ড সিস্টেমের কীরিং-এ রাখে।
add-account-looking = { $address }-এর মেল সার্ভার খোঁজা হচ্ছে…
add-account-address-intro = আপনার ইমেল ঠিকানা লিখুন। Katna আপনার জন্য সার্ভার খুঁজে দেবে।
add-account-servers-title = সার্ভারের সেটিংস
add-account-servers-intro = Katna কোথা থেকে { $address }-এর মেল পড়ে ও পাঠায়।
add-account-signing-in = সাইন ইন করা হচ্ছে…
add-account-browser-title = আপনার ব্রাউজারে চালিয়ে যান
add-account-browser-intro = Katna আপনার ব্রাউজারে { $provider }-এর সাইন-ইন পৃষ্ঠা খুলেছে। সেখানে সাইন ইন করুন এবং Katna-কে আপনার মেল পড়তে ও পাঠাতে অনুমতি দিন, তারপর এখানে ফিরে আসুন।
add-account-browser-hint = কোনো পৃষ্ঠা খোলেনি? আপনার ব্রাউজারের উইন্ডোগুলি দেখুন, অথবা ফিরে গিয়ে আবার চেষ্টা করুন।
add-account-stage-browser = আপনার ব্রাউজারে সাইন ইন করার অপেক্ষায়…
add-account-stage-signing-in-at = { $server }-এ সাইন ইন করা হচ্ছে…
add-account-help-app-password-link = কীভাবে অ্যাপ পাসওয়ার্ড তৈরি করবেন
add-account-help-turn-on-imap = { $provider } মেল অ্যাপগুলিকে তখনই ঢুকতে দেয়, যখন তার ওয়েব মেলের সেটিংসে IMAP ও POP3 অ্যাক্সেস চালু থাকে।
add-account-help-turn-on-imap-link = কীভাবে চালু করবেন

## Add a mail account: fields

add-account-field-address = ইমেল ঠিকানা
add-account-receive-with = মেল গ্রহণ করুন যা দিয়ে
add-account-imap-about = IMAP আপনার মেল ও ফোল্ডার সার্ভারে রাখে, প্রতিটি ডিভাইসে একই রকম। সম্ভব হলে এটিই বেছে নিন।
add-account-pop3-about = POP3 আপনার মেল এই কম্পিউটারে ডাউনলোড করে। এখানে যে মেল পড়েন বা সরান, তা সার্ভারে ও আপনার অন্য ডিভাইসে যেমন ছিল তেমনই থাকে।
add-account-incoming = আগত মেল ({ $protocol })
add-account-outgoing = বহির্গামী মেল ({ $protocol })
add-account-field-server = সার্ভার
add-account-field-port = পোর্ট
add-account-security-none = কিছু না
add-account-security-none-warning = এনক্রিপ্ট করা নয়: আপনার পাসওয়ার্ড ও মেল পথে পড়ে ফেলা যেতে পারে।
add-account-field-username = ব্যবহারকারীর নাম
add-account-field-password = পাসওয়ার্ড
add-account-show-password = পাসওয়ার্ড দেখান
add-account-app-password-hint = এখানে { $provider }-এর একটি অ্যাপ পাসওয়ার্ড লাগবে, ওয়েবে যেটি ব্যবহার করেন সেটি নয়। আপনার { $provider } অ্যাকাউন্টের নিরাপত্তা সেটিংসে একটি তৈরি করুন।
add-account-field-name = আপনার নাম (ঐচ্ছিক)
add-account-name-hint = আপনি যাঁদের লেখেন তাঁরা এটি দেখবেন।
add-account-servers-pair = { $imap } ও { $smtp }
add-account-servers-found = { $source ->
    [built-in] সার্ভার: { $servers }, Katna-র প্রোভাইডার তালিকায় পাওয়া গেছে।
    [provider] সার্ভার: { $servers }, আপনার প্রোভাইডারের সেটিংসে পাওয়া গেছে।
    [ispdb] সার্ভার: { $servers }, Thunderbird-এর প্রোভাইডার তালিকায় পাওয়া গেছে।
    [dns] সার্ভার: { $servers }, আপনার ডোমেনের DNS রেকর্ডে পাওয়া গেছে।
   *[other] সার্ভার: { $servers }, অনুমান করা; সাইন ইন ব্যর্থ হলে যাচাই করুন।
}
add-account-servers-entered = সার্ভার: { $servers }, যেমন লেখা হয়েছে।
add-account-sign-in-with = { $provider } দিয়ে সাইন ইন করুন
add-account-sign-in-instead = এর বদলে { $provider } দিয়ে সাইন ইন করুন

## Add a mail account: buttons

add-account-servers-button = সার্ভারের সেটিংস
add-account-back = ফিরে যান
add-account-add = অ্যাকাউন্ট যোগ করুন
add-account-done = হয়ে গেছে
add-account-another = আরেকটি অ্যাকাউন্ট যোগ করুন
add-account-cancel = বাতিল করুন

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] আগত মেলের সার্ভার লিখুন।
   *[outgoing] বহির্গামী মেলের সার্ভার লিখুন।
}
add-account-server-space = { $kind ->
    [incoming] আগত মেলের সার্ভারের নামে একটি স্পেস আছে।
   *[outgoing] বহির্গামী মেলের সার্ভারের নামে একটি স্পেস আছে।
}
add-account-port-invalid = { $kind ->
    [incoming] আগত মেলের পোর্ট { $min } থেকে { $max }-এর মধ্যে একটি সংখ্যা হতে হবে।
   *[outgoing] বহির্গামী মেলের পোর্ট { $min } থেকে { $max }-এর মধ্যে একটি সংখ্যা হতে হবে।
}
add-account-address-empty = একটি ইমেল ঠিকানা লিখুন।
add-account-address-invalid = { $example }-এর মতো একটি ইমেল ঠিকানা লিখুন।
add-account-not-found = Katna { $address }-এর সার্ভার খুঁজে পায়নি, তাই সাধারণ নামগুলো বসিয়ে দিয়েছে। আপনার প্রোভাইডারের কাছে যাচাই করে নিন।
add-account-password-empty = পাসওয়ার্ড লিখুন।
add-account-name-is-password = নামটি পাসওয়ার্ডের মতোই। তার বদলে সেখানে আপনার নাম লিখুন, যেভাবে লোকে দেখবে।
add-account-app-password-refused = { $provider } পাসওয়ার্ডটি গ্রহণ করেনি। এর জন্য একটি অ্যাপ পাসওয়ার্ড লাগবে, ওয়েবে যেটি ব্যবহার করেন সেটি নয়।
add-account-password-refused = সার্ভার পাসওয়ার্ডটি গ্রহণ করেনি। যাচাই করে আবার চেষ্টা করুন।
add-account-sign-in-refused = { $provider } Katna-কে ঢুকতে দেয়নি। আবার চেষ্টা করুন, এবং আপনার মেলে অ্যাক্সেসের অনুমতি দিন।
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] Katna-র এই কপিটি এখনও Microsoft অ্যাকাউন্টে সাইন ইন করতে পারে না।
    [Google] Katna-র এই কপিটি এখনও Google অ্যাকাউন্টে সাইন ইন করতে পারে না।
   *[other] এই প্রোভাইডার শুধু নিজের পৃষ্ঠাতেই সাইন ইন করতে দেয়, যা Katna এর জন্য এখনও করতে পারে না।
}
add-account-smtp-not-found = Katna আপনার মেল কোথা থেকে পড়তে হবে তা খুঁজে পেয়েছে, কিন্তু কোথা থেকে পাঠাতে হবে তা পায়নি। আউটগোয়িং সার্ভার লিখুন।

## Add a mail account: the last step

add-account-done-title = আপনার অ্যাকাউন্ট প্রস্তুত
add-account-done-intro = Katna এখন আপনার মেল আনছে। নতুন মেল আসার সাথে সাথে দেখা যাবে।
add-account-done-sign-in = সাইন ইন
add-account-done-signed-in-with = { $provider } দিয়ে, আপনার ব্রাউজারে
add-account-done-receiving = মেল গ্রহণ
add-account-done-sending = মেল পাঠানো
add-account-done-on-server = সার্ভারে থাকা মেল
add-account-done-kept = Katna-তে না মোছা পর্যন্ত রাখা হয়
add-account-done-pop3-hint = সার্ভারের মেলের কী হবে তা সেটিংস > অ্যাকাউন্ট-এ বদলান।
add-account-done-zoho-title = টাস্ক ও ক্যালেন্ডার
add-account-done-zoho-about = Zoho এগুলি মেল থেকে আলাদা রাখে। Katna-তে আনতে একবার Zoho দিয়ে সাইন ইন করুন।
add-account-done-linked = টাস্ক ও ক্যালেন্ডার যুক্ত হয়েছে

## The account menu (from the account button on the top bar)

add-account-menu-another = আরেকটি অ্যাকাউন্ট যোগ করুন
app-menu = প্রধান মেনু
app-menu-back = ফিরে যান
