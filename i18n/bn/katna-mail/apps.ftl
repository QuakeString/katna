# Katna Mail, Bengali (বাংলা).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## App rail (and the bottom bar on a phone)

rail-mail = মেল
rail-calendar = ক্যালেন্ডার
rail-contacts = পরিচিতি
rail-tasks = টাস্ক
rail-notes = নোট
rail-files = ফাইল

## Rail right-click menu

rail-menu-open = { $app } খুলুন
rail-menu-settings = { $app } সেটিংস
rail-menu-turn-off = { $app } বন্ধ করুন…

## Turning an app off (Settings > Apps)

app-off-title = { $app } বন্ধ করবেন?
app-off-body = Katna { $app } সিঙ্ক করা বন্ধ করবে এবং এগুলি থেকে সরিয়ে দেবে:
app-off-keep = এই কম্পিউটারে একটি কপি রাখুন
app-off-keep-detail = আবার চালু করলে সঙ্গে সঙ্গে ফিরে আসে
app-off-remove = এই কম্পিউটারের কপি সরান
app-off-remove-detail = আপনার অ্যাকাউন্টে কিছুই বদলায় না, আর আবার চালু করলে এটি আবার ডাউনলোড হয়। যা শুধু এই কম্পিউটারে আছে বা এখনও পাঠানো হয়নি, তা থেকে যায়।
app-off-cancel = বাতিল করুন
app-off-confirm = বন্ধ করুন
app-off-done = { $app } বন্ধ করা হয়েছে
app-off-note = { $app } বন্ধ আছে
app-off-turn-on = চালু করুন
app-off-leaves-calendar-rail = রেল ও Ctrl+2
app-off-leaves-calendar-agenda = আপনার মেলের পাশের এজেন্ডা
app-off-leaves-calendar-meeting = মিটিং শিডিউল করুন, এবং আমন্ত্রণে ক্যালেন্ডারে খুলুন
app-off-leaves-calendar-reminders = ইভেন্টের রিমাইন্ডার
app-off-leaves-calendar-desktop = KRunner ও ডেস্কটপ ঘড়িতে ইভেন্ট
app-off-leaves-contacts-rail = রেল ও Ctrl+3
app-off-leaves-contacts-card = প্রেরকের কার্ডে পরিচিতিতে যোগ করুন
app-off-leaves-contacts-birthdays = ক্যালেন্ডারে জন্মদিন
app-off-leaves-tasks-rail = রেল ও Ctrl+4
app-off-leaves-tasks-mail = মেলে টাস্কে যোগ করুন, এবং Shift+T
app-off-leaves-tasks-calendar = ক্যালেন্ডারে টাস্ক
app-off-leaves-tasks-tray = ট্রেতে নতুন টাস্ক, এবং Meta+Alt+T
app-off-leaves-tasks-reminders = টাস্কের রিমাইন্ডার
app-off-leaves-notes-rail = রেল ও Ctrl+5
app-off-leaves-notes-mail = মেলে নোট যোগ করুন
app-off-leaves-notes-meetings = ইভেন্টে মিটিংয়ের নোট
app-off-leaves-notes-tray = ট্রেতে নতুন নোট, এবং Meta+Alt+N
app-off-leaves-notes-reminders = নোটের রিমাইন্ডার
app-off-leaves-files-rail = রেল ও Ctrl+7
app-off-leaves-files-compose = লেখার সময় সংযুক্ত করতে ফাইল

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = শীঘ্রই আসছে
app-calendar-promise = আপনার CalDAV ক্যালেন্ডার, মেলে আসা মিটিংয়ের আমন্ত্রণ আর রিমাইন্ডার, আপনার ইনবক্সের ঠিক পাশেই।
app-tasks-promise = CalDAV-এর সাথে সিঙ্ক হওয়া করণীয় তালিকা, আর মেল থেকে তৈরি টাস্ক।
app-notes-promise = চটজলদি নোট, আর পরে দেখার জন্য কোনো মেল বা কথোপকথনে নোট।

## Contacts page

app-contacts-loading = আপনার মেল থেকে লোকজনকে একত্র করা হচ্ছে…
app-contacts-empty = আপনি যাঁদের সাথে মেল আদানপ্রদান করেন, তাঁরা এখানে দেখাবেন।
app-contacts-count = { $count ->
    [one] আপনার মেল থেকে { $count } জন, যাঁর সাথে সবচেয়ে বেশি মেল হয়েছে তিনি আগে
   *[other] আপনার মেল থেকে { $count } জন, যাঁদের সাথে সবচেয়ে বেশি মেল হয়েছে তাঁরা আগে
}
app-contacts-top = { $count ->
    [one] আপনার মেল থেকে শীর্ষ { $count } জন, যাঁর সাথে সবচেয়ে বেশি মেল হয়েছে তিনি আগে
   *[other] আপনার মেল থেকে শীর্ষ { $count } জন, যাঁদের সাথে সবচেয়ে বেশি মেল হয়েছে তাঁরা আগে
}
app-contacts-messages = { $count ->
    [one] { $count }টি মেসেজ
   *[other] { $count }টি মেসেজ
}
app-contacts-last = শেষবার { $date }
top-brand = Katna
