# Katna Mail, Bengali (বাংলা).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = মেল অ্যাকাউন্ট যোগ করুন
add-account-looking = { $address }-এর মেল সার্ভার খোঁজা হচ্ছে…
add-account-address-intro = আপনার ইমেল ঠিকানা লিখুন। Katna আপনার জন্য সার্ভার খুঁজে দেবে।
add-account-servers-title = সার্ভারের সেটিংস
add-account-servers-intro = Katna কোথা থেকে { $address }-এর মেল পড়ে ও পাঠায়।
add-account-password-title = আপনার পাসওয়ার্ড লিখুন
add-account-signing-in = সাইন ইন করা হচ্ছে…

## Add a mail account: fields

add-account-field-address = ইমেল ঠিকানা
add-account-incoming = আগত মেল ({ $protocol })
add-account-outgoing = বহির্গামী মেল ({ $protocol })
add-account-field-server = সার্ভার
add-account-field-port = পোর্ট
add-account-security-none = কিছু না
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

## Add a mail account: buttons

add-account-servers-button = সার্ভারের সেটিংস
add-account-back = ফিরে যান
add-account-add = অ্যাকাউন্ট যোগ করুন
add-account-next = পরবর্তী
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
add-account-added = { $address } যোগ করা হয়েছে। আপনার মেল আনা হচ্ছে…
add-account-app-password-refused = { $provider } পাসওয়ার্ডটি গ্রহণ করেনি। এর জন্য একটি অ্যাপ পাসওয়ার্ড লাগবে, ওয়েবে যেটি ব্যবহার করেন সেটি নয়।
add-account-password-refused = সার্ভার পাসওয়ার্ডটি গ্রহণ করেনি। যাচাই করে আবার চেষ্টা করুন।

## The account menu (from the account button on the top bar)

add-account-menu-another = আরেকটি অ্যাকাউন্ট যোগ করুন
add-account-menu-manage = অ্যাকাউন্ট পরিচালনা করুন
