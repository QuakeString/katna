# Katna Mail, Bengali (বাংলা).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

chat-heading = পড়া
chat-view = চ্যাট হিসেবে কথোপকথন
chat-view-detail = মানুষের মধ্যে মেল গ্রুপ চ্যাটের মতো পড়া যায়: প্রতিটি মেলের জন্য একটি বাবল, শুধু যা লেখা হয়েছে তা নিয়ে, আপনার নিজেরগুলি ডানদিকে। নিউজলেটার সাধারণ ভিউতেই থাকে।
chat-view-switch = কথোপকথন চ্যাট হিসেবে দেখান
chat-view-switch-detail = উদ্ধৃত মেল ও স্বাক্ষর প্রতিটি বাবলে ···-এর পেছনে থাকে

chat-switch-chat = চ্যাট
chat-switch-mail = মেল
chat-people = { $names } ও আপনি · { $count ->
    [one] { $count }টি মেল
   *[other] { $count }টি মেল
}
chat-people-heading = { $count ->
    [one] এই চ্যাটে · { $count } জন
   *[other] এই চ্যাটে · { $count } জন
}
chat-member-mails = { $count ->
    [0] কোনো মেল নেই
    [one] { $count }টি মেল
   *[other] { $count }টি মেল
}
chat-today = আজ
chat-yesterday = গতকাল
chat-added = { $who } { $names }-কে যোগ করেছেন
chat-renamed = { $who } বিষয় বদলে “{ $subject }” করেছেন
chat-you = আপনি
chat-not-downloaded = এখনও ডাউনলোড হয়নি
chat-forwarded = ফরোয়ার্ড করা
chat-show-quoted = উদ্ধৃত মেল ও স্বাক্ষর দেখান
chat-hide-quoted = উদ্ধৃত মেল ও স্বাক্ষর লুকান
chat-hide-dots = ··· লুকান
chat-show-card = ওঁর কার্ড দেখান
chat-reply-all = সবাইকে উত্তর দিন
chat-more = আরও
chat-reply-only = শুধু { $name }-কে উত্তর দিন
chat-forward = ফরোয়ার্ড করুন
chat-copy-text = লেখা কপি করুন
chat-show-as-mail = মেল হিসেবে দেখান
chat-pin = উপরে পিন করুন
chat-pin-file = ফাইল উপরে পিন করুন
chat-unpin = আনপিন করুন
chat-unpin-file = ফাইল আনপিন করুন
chat-pinned-of = পিন করা { $count }টির মধ্যে { $at }
chat-pins-all = সব পিন
chat-pins-heading = পিন করা · { $most }টির মধ্যে { $count }টি
chat-pins-drag = ক্রম বদলাতে টেনে আনুন
chat-pin-from-mail = { $name }-এর মেল · { $when }
chat-pin-from-file = { $name }-এর ফাইল · { $when }
chat-pin-from-text = { $name }-এর লেখা · { $when }
chat-pins-full = এই চ্যাটে ইতিমধ্যে 5টি পিন আছে
chat-pins-replace-title = একটি পিন বদলান
chat-pins-replace-hint = একটি চ্যাটে সর্বোচ্চ 5টি পিন থাকে। কোনটি সরাবেন বেছে নিন।
chat-pins-replace = বদলে দিন
chat-pins-cancel = বাতিল করুন
chat-undo = পূর্বাবস্থায় ফেরান

chat-reply-to = { $names }-কে উত্তর দিন
chat-send = পাঠান (Ctrl+Enter)
chat-attach = সংযুক্ত করুন
chat-attach-photo = ছবি
chat-attach-file = ফাইল
chat-attach-library = ফাইল থেকে
chat-attach-template = টেমপ্লেট
chat-attach-signature = স্বাক্ষর
chat-replying-to = { $name }-কে উত্তর দেওয়া হচ্ছে
chat-reply-newest = সবচেয়ে নতুন মেলের উত্তর দিন

## The attach picker (paperclip > From Files)

picker-title = ফাইল থেকে সংযুক্ত করুন
picker-search = নাম, মানুষ, বিষয় খুঁজুন
picker-search-drive = এই ড্রাইভে খুঁজুন
picker-mail-files = মেলের ফাইল
picker-this-chat = এই কথোপকথন
picker-this-computer = এই কম্পিউটার…
picker-in-chat = এই কথোপকথনে
picker-recent = সাম্প্রতিক
picker-preview = প্রিভিউ
picker-cancel = বাতিল করুন
picker-attach = সংযুক্ত করুন
picker-attach-count = { $count }টি সংযুক্ত করুন
picker-selected = { $count }টি বাছাই করা
picker-of-limit = { $limit }-এর মধ্যে
picker-in-mail = মেলের ভেতরে { $size }
picker-drive-links = { $count ->
    [one] 1টি Google Drive লিঙ্ক হিসেবে
   *[other] { $count }টি Google Drive লিঙ্ক হিসেবে
}
picker-onedrive-links = { $count ->
    [one] 1টি OneDrive লিঙ্ক হিসেবে
   *[other] { $count }টি OneDrive লিঙ্ক হিসেবে
}
picker-over = { $size }, একটি মেলে সর্বোচ্চ { $limit } যেতে পারে
picker-getting = { $count ->
    [one] ড্রাইভ থেকে ফাইলটি আনা হচ্ছে…
   *[other] ড্রাইভ থেকে { $count }টি ফাইল আনা হচ্ছে…
}
picker-some-failed = { $count ->
    [one] একটি ফাইল পড়া যায়নি
   *[other] { $count }টি ফাইল পড়া যায়নি
}
