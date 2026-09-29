# Katna Mail, Assamese (অসমীয়া): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = আজি
calendar-today-tip = আজিলৈ যাওক
calendar-view-day = দিন
calendar-view-week = সপ্তাহ
calendar-view-month = মাহ
calendar-view-schedule = সময়সূচী
calendar-previous-day = আগৰ দিন
calendar-next-day = পিছৰ দিন
calendar-previous-week = আগৰ সপ্তাহ
calendar-next-week = পিছৰ সপ্তাহ
calendar-previous-month = আগৰ মাহ
calendar-next-month = পিছৰ মাহ
calendar-previous-period = আগতে
calendar-next-period = পাছত
calendar-title-months = { $first } – { $last }
calendar-loading = ল'ড হৈ আছে…
calendar-read-failed = কেলেণ্ডাৰ পঢ়িব পৰা নগ'ল: { $error }
calendar-local = এই কমপিউটাৰত
calendar-account-gone = আঁতৰোৱা একাউণ্ট
calendar-empty-title = এতিয়াও কোনো কেলেণ্ডাৰ নাই
calendar-empty-text = Katna-এ আপোনাৰ Google আৰু Microsoft একাউণ্টৰ কেলেণ্ডাৰ, আৰু CalDAV দিয়া আন ছাৰ্ভাৰৰ কেলেণ্ডাৰ ছিংক হ'লে ইয়াত দেখুৱায়।
calendar-schedule-empty = পৰৱৰ্তী দুমাহত একো পৰিকল্পনা কৰা হোৱা নাই।
calendar-no-title = (শিৰোনাম নাই)
calendar-all-day = গোটেই দিন
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }, { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = আৰু { $count }
calendar-repeats = পুনৰাবৃত্তি হয়
calendar-join = যোগদান কৰক
calendar-email-guests = অতিথিসকলক ইমেইল কৰক
calendar-running-late = পলম হৈছে
calendar-late-subject = পলম হৈছে: { $title }
calendar-late-body = ক্ষমা কৰিব, { $title }ৰ বাবে মোৰ কেইমিনিটমান পলম হৈছে। মই সোনকালে পাম।
calendar-guests =
    { $count ->
        [one] { $count } জন অতিথি
       *[other] { $count } জন অতিথি
    }
calendar-guest-answers = { $yes } হয়, { $maybe } হয়তো, { $no } নহয়, { $waiting } অপেক্ষাত
calendar-organizer = আয়োজক
calendar-optional = ঐচ্ছিক
calendar-open-web = ব্ৰাউজাৰত খোলক
calendar-close = বন্ধ কৰক

## Adding, changing and deleting events.

calendar-add-title = শিৰোনাম যোগ কৰক
calendar-add-location = অৱস্থান যোগ কৰক
calendar-add-notes = বিৱৰণ যোগ কৰক
calendar-add-guests = অতিথি যোগ কৰক
calendar-remove-guest = আঁতৰাওক
calendar-add-meet = Google Meet ভিডিঅ' কল যোগ কৰক
calendar-add-teams = Teams মিটিং যোগ কৰক
calendar-has-call = ভিডিঅ' কল যোগ কৰা হ'ল
calendar-weekday-day = { $weekday }, { $day }
calendar-all-day-box = গোটেই দিন
calendar-more-options = অধিক বিকল্প
calendar-save = ছেভ কৰক
calendar-saved = ইভেণ্ট ছেভ কৰা হ'ল
calendar-deleted = ইভেণ্ট মচি পেলোৱা হ'ল
calendar-discard = সলনিবোৰ বাতিল কৰক
calendar-edit = ইভেণ্ট সম্পাদনা কৰক
calendar-delete = ইভেণ্ট মচক
calendar-event-details = ইভেণ্টৰ বিৱৰণ
calendar-busy = ব্যস্ত
calendar-free = খালী
calendar-cancel = বাতিল কৰক
calendar-ok = ঠিক আছে
calendar-read-only = আপুনি এই কেলেণ্ডাৰৰ ইভেণ্ট সলনি কৰিব নোৱাৰে
calendar-none-editable = এতিয়ালৈকে এনে কোনো কেলেণ্ডাৰ নাই য'ত আপুনি ইভেণ্ট যোগ কৰিব পাৰে
calendar-no-such-time = আপোনাৰ টাইম জ'নত সেই সময় নাই
calendar-end-before-start = ইভেণ্ট আৰম্ভ হোৱাৰ আগতে শেষ হয়
calendar-repeat-never = পুনৰাবৃত্তি নহয়
calendar-repeat-daily = প্ৰতিদিনে
calendar-repeat-weekly = সাপ্তাহিক: { $weekday }
calendar-repeat-monthly =
    { $nth ->
        [1] মাহিক: প্ৰথম { $weekday }
        [2] মাহিক: দ্বিতীয় { $weekday }
        [3] মাহিক: তৃতীয় { $weekday }
        [4] মাহিক: চতুৰ্থ { $weekday }
       *[other] মাহিক: অন্তিম { $weekday }
    }
calendar-repeat-yearly = বাৰ্ষিক: { $day }
calendar-repeat-weekdays = প্ৰতিটো কামৰ দিন (সোমবাৰৰ পৰা শুক্ৰবাৰলৈ)
calendar-repeat-custom = কাষ্টম
calendar-reminder-none = কোনো জাননী নাই
calendar-reminder-at-start = আৰম্ভণিতে
calendar-reminder-minutes =
    { $count ->
        [one] { $count } মিনিট আগতে
       *[other] { $count } মিনিট আগতে
    }
calendar-reminder-hours =
    { $count ->
        [one] { $count } ঘণ্টা আগতে
       *[other] { $count } ঘণ্টা আগতে
    }
calendar-reminder-days =
    { $count ->
        [one] { $count } দিন আগতে
       *[other] { $count } দিন আগতে
    }
calendar-scope-edit-title = পুনৰাবৃত্ত ইভেণ্ট সম্পাদনা কৰক
calendar-scope-delete-title = পুনৰাবৃত্ত ইভেণ্ট মচক
calendar-scope-this = এই ইভেণ্ট
calendar-scope-following = এই আৰু পিছৰ ইভেণ্টবোৰ
calendar-scope-all = সকলো ইভেণ্ট
calendar-scope-respond-title = পুনৰাবৃত্ত ইভেণ্টৰ বাবে উত্তৰ
calendar-going = আপুনি যাব নেকি?
calendar-answer-yes = হয়
calendar-answer-no = নহয়
calendar-answer-maybe = হয়তো
calendar-answered-yes = আপুনি যাব
calendar-answered-no = আপুনি নাযায়
calendar-answered-maybe = আপুনি হয়তো যাব

## The card at the top of a mail with an invitation.

calendar-invite = আমন্ত্ৰণ
calendar-invite-cancelled = ইভেন্ট বাতিল কৰা হ'ল
calendar-invite-reply = { $name }: উত্তৰ দিছে
calendar-invite-reply-yes = { $name }: গ্ৰহণ কৰিছে
calendar-invite-reply-no = { $name }: প্ৰত্যাখ্যান কৰিছে
calendar-invite-reply-maybe = { $name }: হয়তো যাব
calendar-invite-organizer = আয়োজক: { $name }
calendar-invite-open = কেলেণ্ডাৰত খোলক
calendar-invite-not-yet = এতিয়াও আপোনাৰ কেলেণ্ডাৰত নাই। ছিংক হ'লে উত্তৰ দিব পাৰিব।
calendar-invite-your-day = আপোনাৰ দিন
calendar-invite-clashes =
    { $count ->
        [one] { $count }টা ইভেন্টৰ সৈতে সংঘাত
       *[other] { $count }টা ইভেন্টৰ সৈতে সংঘাত
    }

## The day's agenda beside the mail.

agenda-show = দিনটোৰ কাৰ্যসূচী দেখুৱাওক
agenda-hide = কাৰ্যসূচী লুকুৱাওক
agenda-today = আজি, { $date }
agenda-day = { $weekday }, { $date }
agenda-empty = এই দিনটোত একো পৰিকল্পনা কৰা হোৱা নাই।
