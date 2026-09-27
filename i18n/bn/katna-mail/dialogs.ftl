# Katna Mail, Bengali (বাংলা).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## About Katna (the dialog from the "i" button on the top bar)

about-tooltip = Katna সম্পর্কে
about-tagline = Linux ডেস্কটপের জন্য মেল ও ক্যালেন্ডার
about-whats-new = নতুন কী আছে
about-changelog = পরিবর্তনের তালিকা
about-source = সোর্স কোড
about-coffee = আমাকে এক কাপ কফি খাওয়ান
about-coming-soon = শীঘ্রই আসছে
about-follow = নির্মাতাকে ফলো করুন
about-love-title = Rust, KDE ও Linux-এর প্রতি ভালোবাসা নিয়ে তৈরি
about-love-text = Rust-এর জন্য একটি দ্রুত ও নিরাপদ মেল অ্যাপ লেখা আনন্দের: Katna-তে কোনো unsafe কোড নেই। KDE-র Plasma ডেস্কটপ ও তার PIM স্যুট Katna-কে অনুপ্রাণিত করেছে, আর Linux ও মুক্ত সফটওয়্যার সম্প্রদায় সেই ভিত গড়ে যার ওপর এটি দাঁড়িয়ে। ধন্যবাদ, আর নিচের লাইব্রেরিগুলোকেও ধন্যবাদ।
about-kde-text = KDE সেই ডেস্কটপ তৈরি করে যেখানে Katna সবচেয়ে স্বচ্ছন্দ বোধ করে, আর সেটি স্বেচ্ছাসেবকদের তৈরি এবং আপনার মতো মানুষের অর্থে চলে। Plasma বা KDE-র অ্যাপ ভালো লাগলে KDE-কে অনুদান দেওয়ার কথা ভেবে দেখুন।
about-donate-kde = KDE-কে অনুদান দিন
about-gpui-title = Zed প্রকল্পের GPUI-এর ওপর তৈরি
about-gpui-text = Katna Mail-এর পুরো ইন্টারফেস GPUI-এর ওপর তৈরি, যা Zed Industries Zed এডিটরের জন্য বানানো একটি দ্রুত, GPU-চালিত UI ফ্রেমওয়ার্ক। আপনি যত পিক্সেল, অ্যানিমেশন ও উইন্ডো দেখেন, সবই এটি আঁকে। ধন্যবাদ, Zed টিম, এটি উন্মুক্তভাবে তৈরি করার জন্য। Apache-2.0.
about-gpui-github = GitHub-এ GPUI
about-personal-title = একটি ব্যক্তিগত প্রকল্প
about-personal-text = Katna Mail নতুন বা বৈপ্লবিক হওয়ার চেষ্টা করে না। এটি সেই মেল অ্যাপ যা এর নির্মাতা চেয়েছিলেন, আর এর ফিচার ও চেহারা Gmail, Mailspring ও Thunderbird থেকে ধার করা। LLM এতদূর এগিয়েছে বলেই এটি সম্ভব হয়েছে।
about-built-on = মুক্ত সফটওয়্যারের ওপর তৈরি
about-credit-pimalaya = IMAP, SMTP ও সাইন ইন (io-imap, io-smtp, io-sasl)
about-credit-imap-codec = IMAP পড়া ও লেখা
about-credit-tantivy = অনুসন্ধান
about-credit-sqlite = মেলের ভাণ্ডার
about-credit-rustls = নিরাপদ সংযোগ
about-credit-mail-parser = মেল পড়া, Stalwart Labs থেকে
about-credit-html5ever = HTML মেল, Servo প্রকল্প থেকে
about-credit-zbus = D-Bus ও পোর্টালের মাধ্যমে ডেস্কটপের সঙ্গে যোগাযোগ
about-credit-oo7 = ডেস্কটপের কিরিং-এ পাসওয়ার্ড
about-credit-hayro = PDF দেখা ও প্রিন্ট করা
about-credit-calamine = স্প্রেডশিটের প্রিভিউ
about-credit-resvg = SVG ছবি
about-credit-jiff = তারিখ ও টাইম জোন
about-credit-spellbook = বানান পরীক্ষা, Helix এডিটর থেকে
about-credit-smol = একসঙ্গে অনেক কাজ করা
about-all-libraries = Katna-র ব্যবহৃত সব লাইব্রেরি ({ $count })
about-library-authors = নির্মাতা: { $authors }
about-license = Katna মুক্ত সফটওয়্যার, GNU GPL সংস্করণ ৩ বা তার পরের সংস্করণের অধীনে।
about-close = বন্ধ করুন

## What’s new (shown after an update)

whats-new-title = Katna Mail-এ নতুন কী আছে
whats-new-updated = সংস্করণ { $version }-এ আপডেট হয়েছে
whats-new-version = সংস্করণ { $version }
whats-new-more = { $count ->
    [one] আর পুরো পরিবর্তনের তালিকায় আরও { $count }টি।
   *[other] আর পুরো পরিবর্তনের তালিকায় আরও { $count }টি।
}
whats-new-changelog = পুরো পরিবর্তনের তালিকা
whats-new-got-it = বুঝেছি

## First run: welcome page

onboarding-welcome-title = Katna Mail-এ স্বাগতম
onboarding-welcome-lead = আপনার মেল আপনার নিজের কম্পিউটারে: দ্রুত খোঁজা যায়, অফলাইনেও পড়া যায়, আর ব্যক্তিগত থাকে।
onboarding-fast-title = দ্রুত, অফলাইনেও
onboarding-fast-text = Katna আপনার মেলের একটি কপি এখানে রাখে, তাই সংযোগ থাকুক বা না থাকুক, খোলা ও খোঁজা হয় সঙ্গে সঙ্গে।
onboarding-providers-title = আপনার মেলের সঙ্গে কাজ করে
onboarding-providers-text = Gmail, Outlook, Yahoo, iCloud এবং অন্য যেকোনো IMAP বা POP অ্যাকাউন্ট।
onboarding-private-title = ব্যক্তিগত
onboarding-private-text = আপনার মেল সরাসরি আপনার প্রোভাইডার থেকে এই কম্পিউটারে আসে। কোনো Katna সার্ভার তা দেখে না।
onboarding-get-started = শুরু করুন

## First run: adding an account

onboarding-service-checking = Katna ব্যাকগ্রাউন্ড পরিষেবা পরীক্ষা করা হচ্ছে…
onboarding-service-running = Katna ব্যাকগ্রাউন্ড পরিষেবা চলছে।
onboarding-service-missing = Katna ব্যাকগ্রাউন্ড পরিষেবা চলছে না
onboarding-service-start = এটি আপনার মেল আনে ও পাঠায়। একটি টার্মিনাল থেকে এটি চালু করুন, তারপর আবার পরীক্ষা করুন:
onboarding-check-again = আবার পরীক্ষা করুন
onboarding-account-title = আপনার মেল অ্যাকাউন্ট যোগ করুন
onboarding-account-lead = আপনার ইমেল ঠিকানা ও পাসওয়ার্ড লিখুন, Katna সার্ভারের সেটিংস খুঁজে নেবে। Gmail, Yahoo ও iCloud-এর জন্য একটি অ্যাপ পাসওয়ার্ড লাগে, যা আপনার অ্যাকাউন্টের নিরাপত্তা সেটিংসে তৈরি করা হয়।
onboarding-add-account = অ্যাকাউন্ট যোগ করুন
onboarding-back = ফিরে যান

## First run: choosing the look

onboarding-look-title = নিজের মতো করে নিন
onboarding-look-lead = মেল কীভাবে খুলবে আর Katna দেখতে কেমন হবে, বেছে নিন। দ্রুত সেটিংসে যেকোনো সময় এগুলো বদলাতে পারবেন।
onboarding-reading-pane = রিডিং প্যান
onboarding-pane-right = তালিকার ডানদিকে
onboarding-pane-none = কোনো বিভাজন নেই
onboarding-theme = থিম
onboarding-theme-system = সিস্টেম
onboarding-theme-light = লাইট
onboarding-theme-dark = ডার্ক
onboarding-density = ঘনত্ব
onboarding-density-default = ডিফল্ট
onboarding-density-compact = কমপ্যাক্ট
onboarding-continue = চালিয়ে যান

## First run: done

onboarding-ready-title = সব তৈরি
onboarding-ready-lead = Katna আপনার মেল আনছে। মেল আসার সঙ্গে সঙ্গে দেখা যাবে, আর নতুন মেল নিজে থেকেই চলে আসবে।
onboarding-ready-lead-address = Katna { $address }-এর মেল আনছে। মেল আসার সঙ্গে সঙ্গে দেখা যাবে, আর নতুন মেল নিজে থেকেই চলে আসবে।
onboarding-ready-tour = কোথায় কী আছে দেখতে এক মিনিটের একটি ঘোরাঘুরি করবেন?
onboarding-skip = এখন থাক
onboarding-take-tour = ঘুরে দেখুন

## Asking to send crash reports (on its own and on the first-run pages)

share-title = Katna উন্নত করতে সাহায্য করুন
share-lead = Katna ক্র্যাশ করলে এই কম্পিউটারে একটি রিপোর্ট সেভ করে। এই রিপোর্টগুলো পাঠালে সমস্যা ঠিক করতে সাহায্য হয়। সেটিংস > ব্যবহারকারীর মতামত-এ যেকোনো সময় এটি বদলাতে পারবেন।
share-sent = কী পাঠানো হয়
share-sent-detail = সেটিংসে যেমন দেখতে পান ঠিক তেমন ক্র্যাশ রিপোর্ট: কী ক্র্যাশ করেছে ও Katna-র কোথায়, সংস্করণ, আপনার Linux সিস্টেম ও ডেস্কটপ, আর Katna-র লগের শেষ কয়েকটি লাইন, যাতে মেল ফোল্ডারের নাম থাকতে পারে।
share-never-sent = কী কখনো পাঠানো হয় না
share-never-sent-detail = আপনার মেসেজ, পরিচিতি, পাসওয়ার্ড, IP ঠিকানা, ব্যবহারকারীর নাম বা কম্পিউটারের নাম। রিপোর্ট থেকে ইমেল ঠিকানা সরিয়ে ফেলা হয়।
share-where = কোথায় যায়
share-where-detail = Sentry-তে Katna-র ক্র্যাশ ট্র্যাকারে, যা EU-তে সংরক্ষিত। কোনো ID রিপোর্টগুলোকে আপনার সঙ্গে যুক্ত করে না।
share-dont-send = পাঠাবেন না
share-send = ক্র্যাশ রিপোর্ট পাঠান
share-sending = ক্র্যাশ রিপোর্ট পাঠানো হবে। ধন্যবাদ।
share-local = ক্র্যাশ রিপোর্ট এই কম্পিউটারেই থাকবে।

## The tour (cards pointing at each part of the window)

tour-welcome-title = Katna Mail-এ স্বাগতম
tour-welcome-text = এক মিনিটের একটি ঘোরাঘুরি দেখায় কোথায় কী আছে।
tour-not-now = এখন নয়
tour-start = ঘুরে দেখুন
tour-close = বন্ধ করুন
tour-skip = ঘোরাঘুরি বাদ দিন
tour-back = পেছনে
tour-done = হয়ে গেছে
tour-next = পরবর্তী
tour-step = { $total }-এর মধ্যে { $step }
tour-compose-title = একটি মেসেজ লিখুন
tour-compose-text = লিখুন বোতামটি নিচে ডানদিকে একটি নতুন মেসেজ খোলে, তাই লিখতে লিখতেও পড়া চালিয়ে যেতে পারেন।
tour-search-title = আপনার সব মেল খুঁজুন
tour-search-text = অফলাইনেও খোঁজা যায়। ডান প্রান্তের বোতামটি ফিল্টার যোগ করে: প্রেরক, প্রাপক, বিষয়, তারিখ ও সংযুক্তি।
tour-menu-title = ফোল্ডার দেখান বা লুকান
tour-menu-text = এই বোতামটি ফোল্ডারের তালিকা গুটিয়ে রাখে। লুকানো থাকলে ফোল্ডারগুলো দেখতে বাঁদিকের মেল-এর ওপর পয়েন্টার রাখুন।
tour-apps-title = আপনার অ্যাপ
tour-apps-text = মেল এখন এখানে থাকে। ক্যালেন্ডার, পরিচিতি, টাস্ক, নোট ও ফিড এই বারে এর সঙ্গে যোগ দেবে।
tour-tabs-title = ইনবক্স ট্যাব
tour-tabs-text = নতুন মেল প্রাথমিক, প্রচার, সামাজিক, আপডেট ও ফোরাম-এ ভাগ করা হয়। দ্রুত সেটিংসে ট্যাবগুলো বন্ধ করতে পারেন।
tour-list-title = আপনার মেসেজ
tour-list-text = কোনো মেসেজ পড়তে তাতে ক্লিক করুন। দ্রুত কাজের জন্য তার ওপর পয়েন্টার রাখুন, আরও বিকল্পের জন্য ডান-ক্লিক করুন, অথবা একসঙ্গে কাজ করতে কয়েকটিতে টিক দিন।
tour-settings-title = দ্রুত সেটিংস
tour-settings-text = রিডিং প্যান, ঘনত্ব ও থিম এখানে বদলান। ঘোরাঘুরিটিও সেখান থেকে আবার শুরু করা যায়।
tour-account-title = আপনার অ্যাকাউন্ট
tour-account-text = আপনি কোন অ্যাকাউন্টে আছেন দেখুন, আর আরেকটি যোগ করুন।

## Crash notice (a bar at the bottom after a crash)

crash-daemon = { $more ->
    [0] Katna-র ব্যাকগ্রাউন্ড পরিষেবা হঠাৎ বন্ধ হয়ে গেছে।
    [one] Katna-র ব্যাকগ্রাউন্ড পরিষেবা হঠাৎ বন্ধ হয়ে গেছে। আরও { $more }টি ক্র্যাশ রিপোর্ট সেভ করা আছে।
   *[other] Katna-র ব্যাকগ্রাউন্ড পরিষেবা হঠাৎ বন্ধ হয়ে গেছে। আরও { $more }টি ক্র্যাশ রিপোর্ট সেভ করা আছে।
}
crash-mail = { $more ->
    [0] গতবার Katna Mail হঠাৎ বন্ধ হয়ে গিয়েছিল।
    [one] গতবার Katna Mail হঠাৎ বন্ধ হয়ে গিয়েছিল। আরও { $more }টি ক্র্যাশ রিপোর্ট সেভ করা আছে।
   *[other] গতবার Katna Mail হঠাৎ বন্ধ হয়ে গিয়েছিল। আরও { $more }টি ক্র্যাশ রিপোর্ট সেভ করা আছে।
}
crash-view = রিপোর্ট দেখুন
crash-view-tooltip = এই কম্পিউটারে সেভ করা রিপোর্টটি খুলুন
crash-copy = রিপোর্ট কপি করুন
crash-close = বন্ধ করুন
