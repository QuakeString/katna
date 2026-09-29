# Katna Mail, Bengali (বাংলা): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = আজ
calendar-today-tip = আজকে যান
calendar-view-day = দিন
calendar-view-week = সপ্তাহ
calendar-view-month = মাস
calendar-view-year = বছর
calendar-view-schedule = সময়সূচি
calendar-view-days =
    { $count ->
        [one] { $count } দিন
       *[other] { $count } দিন
    }
calendar-options = বিকল্প
calendar-density = ঘনত্ব
calendar-density-responsive = আপনার স্ক্রিন অনুযায়ী
calendar-density-comfortable = আরামদায়ক
calendar-density-compact = কমপ্যাক্ট
calendar-custom-days = কাস্টম ভিউ
calendar-second-zone = দ্বিতীয় সময় অঞ্চল
calendar-zone-none = কোনওটিই নয়
calendar-zone = { $zone } ({ $offset })
calendar-share-free = ফ্রি সময় শেয়ার করুন
calendar-free-subject = আমার ফ্রি সময়
calendar-free-intro = আমি যখন ফ্রি থাকব, এমন কিছু সময় ({ $zone }):
calendar-free-day = { $weekday } { $date }: { $times }
calendar-free-range = { $start } – { $end }
calendar-free-none = আগামী কয়েক কার্যদিবসে আমার কোনো ফ্রি সময় নেই।
calendar-previous-day = আগের দিন
calendar-next-day = পরের দিন
calendar-previous-week = আগের সপ্তাহ
calendar-next-week = পরের সপ্তাহ
calendar-previous-month = আগের মাস
calendar-next-month = পরের মাস
calendar-previous-year = আগের বছর
calendar-next-year = পরের বছর
calendar-previous-period = আগে
calendar-next-period = পরে
calendar-title-months = { $first } – { $last }
calendar-loading = লোড হচ্ছে…
calendar-read-failed = ক্যালেন্ডার পড়া যায়নি: { $error }
calendar-sets = ক্যালেন্ডার সেট
calendar-set-add = দেখানো ক্যালেন্ডারগুলো একটি সেট হিসেবে সংরক্ষণ করুন
calendar-set-name = সেটের নাম
calendar-set-remove = সেট সরান
calendar-local = এই কম্পিউটারে
calendar-account-gone = সরানো অ্যাকাউন্ট
calendar-account-sign-in = ক্যালেন্ডার দেখাতে আবার সাইন ইন করুন
calendar-account-signed-in = { $address }-এ আবার সাইন ইন করা হয়েছে। আপনার ক্যালেন্ডারগুলো আনা হচ্ছে…
calendar-account-sign-in-refused = { $provider } Katna-কে ঢুকতে দেয়নি। আবার চেষ্টা করুন, এবং আপনার ক্যালেন্ডারে অ্যাক্সেসের অনুমতি দিন।
calendar-account-refused = সার্ভার Katna-কে ক্যালেন্ডারগুলোতে ঢুকতে দেয়নি।
calendar-account-not-enabled = Katna-র জন্য ক্যালেন্ডার অ্যাক্সেস এখনও চালু করা হয়নি।
calendar-account-failed = ক্যালেন্ডারগুলো পড়া যায়নি।
calendar-account-error = ক্যালেন্ডারগুলো পড়া যায়নি: { $reason }
calendar-account-none = কোনো ক্যালেন্ডার পাওয়া যায়নি
calendar-account-looking = ক্যালেন্ডার খোঁজা হচ্ছে…
calendar-account-try-again = আবার চেষ্টা করুন
calendar-account-try-again-tooltip = এই অ্যাকাউন্টের ক্যালেন্ডারগুলো এখনই আবার দেখুন
calendar-account-fixing = কাজ চলছে…
calendar-birthdays = জন্মদিন
calendar-birthday-of = { $name }-এর জন্মদিন
calendar-empty-title = এখনও কোনো ক্যালেন্ডার নেই
calendar-empty-text = Katna আপনার Google ও Microsoft অ্যাকাউন্টের ক্যালেন্ডার এবং CalDAV দেয় এমন অন্য সার্ভারের ক্যালেন্ডার সিঙ্ক হলে এখানে দেখায়।
calendar-schedule-empty = পরবর্তী দুই মাসে কিছু পরিকল্পিত নেই।
calendar-search = ইভেন্ট খুঁজুন
calendar-search-past = অতীতের ইভেন্ট
calendar-search-none = আপনার অনুসন্ধানের সাথে কোনো ইভেন্ট মেলেনি।
calendar-no-title = (শিরোনাম নেই)
calendar-all-day = সারাদিন
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }, { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = আরও { $count }
calendar-repeats = পুনরাবৃত্ত হয়
calendar-join = যোগ দিন
calendar-email-guests = অতিথিদের ইমেল করুন
calendar-running-late = দেরি হচ্ছে
calendar-late-subject = দেরি হচ্ছে: { $title }
calendar-late-body = দুঃখিত, { $title }-এর জন্য আমার কয়েক মিনিট দেরি হচ্ছে। আমি শীঘ্রই পৌঁছে যাব।
calendar-guests =
    { $count ->
        [one] { $count } জন অতিথি
       *[other] { $count } জন অতিথি
    }
calendar-guest-answers = { $yes } হ্যাঁ, { $maybe } হয়তো, { $no } না, { $waiting } অপেক্ষারত
calendar-organizer = আয়োজক
calendar-optional = ঐচ্ছিক
calendar-open-web = ব্রাউজারে খুলুন
calendar-open-contact = পরিচিতি খুলুন
calendar-close = বন্ধ করুন

## Adding, changing and deleting events.

calendar-add-title = শিরোনাম যোগ করুন
calendar-add-location = অবস্থান যোগ করুন
calendar-add-notes = বিবরণ যোগ করুন
calendar-add-guests = অতিথি যোগ করুন
calendar-remove-guest = সরান
calendar-add-meet = Google Meet ভিডিও কল যোগ করুন
calendar-add-teams = Teams মিটিং যোগ করুন
calendar-has-call = ভিডিও কল যোগ করা হয়েছে
calendar-weekday-day = { $weekday }, { $day }
calendar-all-day-box = সারাদিন
calendar-more-options = আরও বিকল্প
calendar-save = সেভ করুন
calendar-saved = ইভেন্ট সেভ হয়েছে
calendar-deleted = ইভেন্ট মুছে ফেলা হয়েছে
calendar-discard = পরিবর্তন বাতিল করুন
calendar-edit = ইভেন্ট সম্পাদনা করুন
calendar-delete = ইভেন্ট মুছুন
calendar-event-details = ইভেন্টের বিবরণ
calendar-kind-event = ইভেন্ট
calendar-kind-focus = ফোকাস টাইম
calendar-kind-out-of-office = অফিসের বাইরে
calendar-kind-working-location = কাজের জায়গা
calendar-working-home = বাড়ি
calendar-busy = ব্যস্ত
calendar-free = ফ্রি
calendar-cancel = বাতিল করুন
calendar-ok = ঠিক আছে
calendar-read-only = এই ক্যালেন্ডারের ইভেন্ট আপনি পরিবর্তন করতে পারবেন না
calendar-none-editable = এখনও এমন কোনো ক্যালেন্ডার নেই যাতে আপনি ইভেন্ট যোগ করতে পারেন
calendar-no-such-time = আপনার টাইম জোনে এই সময়টি নেই
calendar-end-before-start = ইভেন্টটি শুরুর আগেই শেষ হয়ে যাচ্ছে
calendar-repeat-never = পুনরাবৃত্তি হয় না
calendar-repeat-daily = প্রতিদিন
calendar-repeat-weekly = প্রতি সপ্তাহে { $weekday }
calendar-repeat-monthly =
    { $nth ->
        [1] প্রতি মাসের প্রথম { $weekday }
        [2] প্রতি মাসের দ্বিতীয় { $weekday }
        [3] প্রতি মাসের তৃতীয় { $weekday }
        [4] প্রতি মাসের চতুর্থ { $weekday }
       *[other] প্রতি মাসের শেষ { $weekday }
    }
calendar-repeat-yearly = প্রতি বছর { $day }
calendar-repeat-weekdays = প্রতিটি কার্যদিবস (সোমবার থেকে শুক্রবার)
calendar-repeat-custom = কাস্টম
calendar-reminder-none = কোনো বিজ্ঞপ্তি নেই
calendar-reminder-at-start = শুরুর সময়
calendar-reminder-minutes =
    { $count ->
        [one] { $count } মিনিট আগে
       *[other] { $count } মিনিট আগে
    }
calendar-reminder-hours =
    { $count ->
        [one] { $count } ঘণ্টা আগে
       *[other] { $count } ঘণ্টা আগে
    }
calendar-reminder-days =
    { $count ->
        [one] { $count } দিন আগে
       *[other] { $count } দিন আগে
    }
calendar-scope-edit-title = পুনরাবৃত্ত ইভেন্ট সম্পাদনা করুন
calendar-scope-delete-title = পুনরাবৃত্ত ইভেন্ট মুছুন
calendar-scope-this = এই ইভেন্ট
calendar-scope-following = এই ও পরবর্তী ইভেন্টগুলি
calendar-scope-all = সব ইভেন্ট
calendar-scope-respond-title = পুনরাবৃত্ত ইভেন্টের উত্তর
calendar-going = আপনি কি যাচ্ছেন?
calendar-answer-yes = হ্যাঁ
calendar-answer-no = না
calendar-answer-maybe = হয়তো
calendar-answered-yes = আপনি যাচ্ছেন
calendar-answered-no = আপনি যাচ্ছেন না
calendar-answered-maybe = আপনি হয়তো যাবেন

## The card at the top of a mail with an invitation.

calendar-invite = আমন্ত্রণ
calendar-invite-cancelled = ইভেন্ট বাতিল করা হয়েছে
calendar-invite-reply = { $name } উত্তর দিয়েছেন
calendar-invite-reply-yes = { $name } গ্রহণ করেছেন
calendar-invite-reply-no = { $name } প্রত্যাখ্যান করেছেন
calendar-invite-reply-maybe = { $name } হয়তো যাবেন
calendar-invite-organizer = আয়োজক: { $name }
calendar-invite-open = ক্যালেন্ডারে খুলুন
calendar-invite-not-yet = এখনও আপনার ক্যালেন্ডারে নেই। সিঙ্ক হলে উত্তর দেওয়া যাবে।
calendar-invite-by-mail = আপনার ক্যালেন্ডারে নেই: আপনার উত্তর মেলে আয়োজকের কাছে যাবে।
calendar-mail-yes = গৃহীত: { $title }
calendar-mail-yes-body = { $name } এই আমন্ত্রণ গ্রহণ করেছেন।
calendar-mail-no = প্রত্যাখ্যাত: { $title }
calendar-mail-no-body = { $name } এই আমন্ত্রণ প্রত্যাখ্যান করেছেন।
calendar-mail-maybe = সাময়িকভাবে গৃহীত: { $title }
calendar-mail-maybe-body = { $name } এই আমন্ত্রণ সাময়িকভাবে গ্রহণ করেছেন।
calendar-invite-your-day = আপনার দিন
calendar-invite-clashes =
    { $count ->
        [one] { $count }টি ইভেন্টের সঙ্গে সংঘাত
       *[other] { $count }টি ইভেন্টের সঙ্গে সংঘাত
    }

## The day's agenda beside the mail.

agenda-show = দিনের এজেন্ডা দেখান
agenda-hide = এজেন্ডা লুকান
agenda-today = আজ, { $date }
agenda-day = { $weekday }, { $date }
agenda-empty = এই দিনে কিছু পরিকল্পিত নেই।
