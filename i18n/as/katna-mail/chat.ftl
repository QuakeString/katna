# Katna Mail, Assamese (অসমীয়া).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

chat-heading = পঢ়া
chat-view = চেটৰ দৰে কথোপকথন
chat-view-detail = মানুহৰ মাজৰ মেইল এটা গোট চেটৰ দৰে পঢ়া যায়: প্ৰতিটো মেইলৰ বাবে এটা বাবল, কেৱল যি লিখা হৈছিল তাৰ সৈতে, আপোনাৰখিনি সোঁফালে। নিউজলেটাৰবোৰ সাধাৰণ ভিউতে থাকে।
chat-view-switch = কথোপকথন চেটৰ দৰে দেখুৱাওক
chat-view-switch-detail = উদ্ধৃত মেইল আৰু চিগনেচাৰ প্ৰতিটো বাবলত ···ৰ পিছত থাকে

chat-switch-chat = চেট
chat-switch-mail = মেইল
chat-people = { $names } আৰু আপুনি · { $count ->
    [one] { $count }টা মেইল
   *[other] { $count }টা মেইল
}
chat-people-heading = { $count ->
    [one] এই চেটত · { $count } জন ব্যক্তি
   *[other] এই চেটত · { $count } জন ব্যক্তি
}
chat-member-mails = { $count ->
    [0] কোনো মেইল নাই
    [one] { $count }টা মেইল
   *[other] { $count }টা মেইল
}
chat-today = আজি
chat-yesterday = কালি
chat-added = { $who }এ { $names }ক যোগ কৰিলে
chat-renamed = { $who }এ বিষয় সলনি কৰি “{ $subject }” কৰিলে
chat-you = আপুনি
chat-not-downloaded = এতিয়ালৈকে ডাউনল'ড হোৱা নাই
chat-forwarded = ফৰৱাৰ্ড কৰা
chat-show-quoted = উদ্ধৃত মেইল আৰু চিগনেচাৰ দেখুৱাওক
chat-hide-quoted = উদ্ধৃত মেইল আৰু চিগনেচাৰ লুকুৱাওক
chat-hide-dots = ··· লুকুৱাওক
chat-show-card = তেওঁৰ কাৰ্ড দেখুৱাওক
chat-reply-all = সকলোকে উত্তৰ দিয়ক
chat-more = অধিক
chat-reply-only = কেৱল { $name }ক উত্তৰ দিয়ক
chat-forward = ফৰৱাৰ্ড কৰক
chat-copy-text = পাঠ কপি কৰক
chat-show-as-mail = মেইল হিচাপে দেখুৱাওক
chat-go-down = শেহতীয়া মেইললৈ যাওক
chat-pin = ওপৰত পিন কৰক
chat-pin-file = ফাইল ওপৰত পিন কৰক
chat-unpin = আনপিন কৰক
chat-unpin-file = ফাইল আনপিন কৰক
chat-pinned-of = { $count }টাৰ { $at } নং পিন কৰা
chat-pins-all = সকলো পিন
chat-pins-heading = পিন কৰা · { $most }টাৰ { $count }টা
chat-pins-drag = ক্ৰম সলনি কৰিবলৈ টানক
chat-pin-from-mail = { $name }ৰ মেইল · { $when }
chat-pin-from-file = { $name }ৰ ফাইল · { $when }
chat-pin-from-text = { $name }ৰ পাঠ · { $when }
chat-pins-full = এই চেটত ইতিমধ্যে 5টা পিন আছে
chat-pins-replace-title = এটা পিন সলনি কৰক
chat-pins-replace-hint = এটা চেটত সৰ্বাধিক 5টা পিন থাকে। কোনটো আঁতৰাব বাছনি কৰক।
chat-pins-replace = সলনি কৰক
chat-pins-cancel = বাতিল কৰক
chat-undo = আনডু কৰক

chat-reply-to = { $names }ক উত্তৰ দিয়ক
chat-send = পঠিয়াওক (Ctrl+Enter)। অধিকৰ বাবে ৰাইট-ক্লিক কৰক বা ধৰি ৰাখক
chat-send-now = এতিয়াই পঠিয়াওক
chat-attach = সংলগ্ন কৰক
chat-attach-photo = ফট'
chat-attach-file = ফাইল
chat-attach-library = ফাইলসমূহৰ পৰা
chat-attach-template = টেমপ্লেট
chat-attach-signature = চিগনেচাৰ
chat-replying-to = { $name }ক উত্তৰ দি আছে
chat-reply-newest = শেহতীয়া মেইলৰ উত্তৰ দিয়ক

## The attach picker (paperclip > From Files)

picker-title = ফাইলসমূহৰ পৰা সংলগ্ন কৰক
picker-search = নাম, ব্যক্তি, বিষয় সন্ধান কৰক
picker-search-drive = এই ড্ৰাইভ সন্ধান কৰক
picker-mail-files = মেইলৰ ফাইল
picker-this-chat = এই কথোপকথন
picker-this-computer = এই কম্পিউটাৰ…
picker-in-chat = এই কথোপকথনত
picker-recent = শেহতীয়া
picker-preview = পূৰ্বদৰ্শন
picker-cancel = বাতিল কৰক
picker-attach = সংলগ্ন কৰক
picker-attach-count = { $count }টা সংলগ্ন কৰক
picker-selected = { $count }টা বাছনি কৰা হৈছে
picker-of-limit = { $limit }ৰ ভিতৰত
picker-in-mail = মেইলত { $size }
picker-drive-links = { $count ->
    [one] 1টা Google Drive লিংক হিচাপে
   *[other] { $count }টা Google Drive লিংক হিচাপে
}
picker-onedrive-links = { $count ->
    [one] 1টা OneDrive লিংক হিচাপে
   *[other] { $count }টা OneDrive লিংক হিচাপে
}
picker-over = { $size }, এটা মেইলে কঢ়িয়াব পৰা { $limit }তকৈ বেছি
picker-getting = { $count ->
    [one] ড্ৰাইভৰ পৰা ফাইলটো আনি আছে…
   *[other] ড্ৰাইভৰ পৰা { $count }টা ফাইল আনি আছে…
}
picker-some-failed = { $count ->
    [one] এটা ফাইল পঢ়িব পৰা নগ'ল
   *[other] { $count }টা ফাইল পঢ়িব পৰা নগ'ল
}
