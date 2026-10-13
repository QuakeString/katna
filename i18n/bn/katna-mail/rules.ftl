# Katna Mail, Bengali (বাংলা).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Folders & rules

settings-rules = নিয়ম
settings-rules-summary = নতুন মেল নিজে থেকেই সাজান, লেবেল দিন, ফরোয়ার্ড করুন বা নীরব রাখুন
settings-rules-intro = নিয়মগুলি এই ক্রমে নতুন মেল নিজে থেকেই সাজায়। ক্রম বদলাতে টেনে আনুন।
settings-rules-all-accounts = সব অ্যাকাউন্ট
settings-rules-new = নতুন নিয়ম
settings-rules-none = এখনও কোনো নিয়ম নেই। একটি নিয়ম প্রেরক, বিষয় বা শব্দ অনুযায়ী নতুন মেল নিজে থেকেই সাজায়।
settings-rules-none-account = এই অ্যাকাউন্টের জন্য এখনও কোনো নিয়ম নেই।
settings-rules-drag = ক্রম বদলাতে টেনে আনুন
settings-rules-edit = নিয়ম সম্পাদনা করুন
settings-rules-turn-off = এই নিয়মটি বন্ধ করুন
settings-rules-turn-on = এই নিয়মটি চালু করুন

## Starter rules: offered under the user's own rules, switched off.
## Turning one on makes it one of the user's rules.

settings-rules-starters = শুরুর নিয়ম
settings-rules-starters-intro = আপনি চালু না করা পর্যন্ত বন্ধ থাকে। এগুলি আপনার সব অ্যাকাউন্টে কাজ করে; বদলাতে কোনোটি সম্পাদনা করুন।
settings-rules-starter-turning-on = “{ $name }” চালু করা হচ্ছে…
settings-rules-starter-failed = “{ $name }” চালু করা যায়নি: { $error }
rules-starter-promotions = প্রচারমূলক মেল নীরব রাখুন
rules-starter-newsletters = নিউজলেটার পড়ার ফোল্ডারে
rules-starter-receipts = রসিদ ও ইনভয়েস
rules-starter-deliveries = ডেলিভারি
rules-starter-train = ট্রেনের টিকিট
rules-starter-flight = বিমানের টিকিট
rules-starter-codes = ওয়ান-টাইম কোড
rules-starter-security = নিরাপত্তা সতর্কতা
rules-starter-social = সামাজিক মেল
rules-starter-invites = ক্যালেন্ডারের আমন্ত্রণ
rules-starter-folder-reading = পড়ার জন্য
rules-starter-folder-receipts = রসিদ
rules-starter-folder-deliveries = ডেলিভারি
rules-starter-folder-travel = ভ্রমণ
rules-starter-folder-social = সামাজিক
rules-runs-katna = Katna-য় চলে
rules-runs-gmail = Gmail-এ চলে
rules-runs-sieve = সার্ভারে চলে
rules-stopped = থেমে গেছে
rules-error-folder-gone = এই নিয়মের ফোল্ডারটি আর নেই। অন্য একটি বেছে নিতে নিয়মটি সম্পাদনা করুন।
rules-error-no-archive = এই অ্যাকাউন্টে কোনো আর্কাইভ ফোল্ডার নেই। অন্য কিছু করাতে নিয়মটি সম্পাদনা করুন।
rules-error-no-trash = এই অ্যাকাউন্টে কোনো ট্র্যাশ ফোল্ডার নেই। অন্য কিছু করাতে নিয়মটি সম্পাদনা করুন।
rules-error-cannot-send = এই অ্যাকাউন্ট মেল পাঠাতে পারে না, তাই নিয়মটি ফরোয়ার্ড করতে পারে না।
rules-error-other = { $error }। নিয়মটি সম্পাদনা করে আবার চালু করুন।

settings-folders = ফোল্ডার
settings-folders-summary = ফোল্ডার প্যানেলে অপঠিত সংখ্যা
settings-folders-unread-counts = প্রতিটি ফোল্ডারে অপঠিত সংখ্যা
settings-folders-unread-counts-detail = বন্ধ থাকলে শুধু ইনবক্স দেখায় কতগুলি অপঠিত

## A rule in one line, on its row: "From contains substack.com → skip the
## inbox, label Reading".

rules-summary = { $when } → { $then }
rules-summary-and = { $first } এবং { $next }
rules-summary-or = { $first } বা { $next }
rules-summary-more = আরও { $count }টি
rules-summary-list = { $first }, { $next }
rules-summary-condition = { $field } { $comparator } { $value }
rules-summary-has-attachment = সংযুক্তি আছে
rules-summary-no-attachment = কোনো সংযুক্তি নেই
rules-summary-mailing-list = মেলিং লিস্ট থেকে
rules-summary-not-mailing-list = মেলিং লিস্ট থেকে নয়
rules-summary-tab = { $tab } ট্যাবে
rules-summary-not-tab = { $tab } ট্যাবে নয়
rules-summary-move = { $folder }-এ সরান
rules-summary-archive = ইনবক্স এড়িয়ে যান
rules-summary-trash = ট্র্যাশে সরান
rules-summary-mark-read = পঠিত হিসেবে চিহ্নিত করুন
rules-summary-star = তারকাচিহ্ন দিন
rules-summary-important = গুরুত্বপূর্ণ হিসেবে চিহ্নিত করুন
rules-summary-label = { $label } লেবেল দিন
rules-summary-forward = { $address }-এ ফরোয়ার্ড করুন
rules-summary-dont-notify = বিজ্ঞপ্তি দেবেন না
rules-summary-read-after = { $count ->
    [one] { $count } দিন পরে পঠিত হিসেবে চিহ্নিত করুন
   *[other] { $count } দিন পরে পঠিত হিসেবে চিহ্নিত করুন
}
rules-summary-folder-gone = মুছে যাওয়া একটি ফোল্ডার

## The rule editor

rules-editor-new-title = নতুন নিয়ম
rules-editor-edit-title = নিয়ম সম্পাদনা করুন
rules-editor-name-hint = নিয়মের নাম
rules-editor-when = যখন কোনো নতুন মেল মেলে
rules-editor-of-these = এগুলির সাথে:
rules-mode-all = সবগুলি
rules-mode-any = যেকোনোটি
rules-field-from = প্রেরক
rules-field-to = প্রাপক
rules-field-cc = Cc
rules-field-any-recipient = প্রাপক বা Cc
rules-field-reply-to = উত্তরের ঠিকানা
rules-field-subject = বিষয়
rules-field-body = টেক্সট
rules-field-attachment-name = সংযুক্তির নাম
rules-field-has-attachment = সংযুক্তি আছে
rules-field-mailing-list = মেলিং লিস্ট থেকে
rules-field-tab = ইনবক্স ট্যাব
rules-comparator-contains = আছে
rules-comparator-not-contains = নেই
rules-comparator-begins-with = দিয়ে শুরু
rules-comparator-ends-with = দিয়ে শেষ
rules-comparator-equals = ঠিক এটি
rules-comparator-matches = প্যাটার্নের সাথে মেলে
rules-has-yes = হ্যাঁ
rules-has-no = না
rules-editor-value-hint = শব্দ বা ঠিকানা
rules-editor-add-condition = একটি শর্ত যোগ করুন
rules-editor-remove = সরান
rules-editor-then = তারপর:
rules-action-move = এখানে সরান
rules-action-archive = ইনবক্স এড়িয়ে যান (আর্কাইভ)
rules-action-trash = ট্র্যাশে সরান
rules-action-mark-read = পঠিত হিসেবে চিহ্নিত করুন
rules-action-star = তারকাচিহ্ন দিন
rules-action-important = গুরুত্বপূর্ণ হিসেবে চিহ্নিত করুন
rules-action-label = লেবেল যোগ করুন
rules-action-forward = এখানে ফরোয়ার্ড করুন
rules-action-dont-notify = বিজ্ঞপ্তি দেবেন না
rules-action-read-after = এত দিন পরে পঠিত হিসেবে চিহ্নিত করুন
rules-editor-choose-folder = একটি ফোল্ডার বেছে নিন
rules-editor-choose-label = একটি লেবেল বেছে নিন
rules-editor-new-folder = নতুন: { $name }
rules-editor-folder-of = { $folder } ({ $account })
rules-editor-forward-hint = ইমেল ঠিকানা
rules-editor-days = দিন
rules-editor-add-action = একটি কাজ যোগ করুন
rules-editor-stop = এখানে থামুন: পরের নিয়মগুলি এই মেলে চলবে না
rules-editor-accounts = অ্যাকাউন্ট:
rules-editor-accounts-none = অ্যাকাউন্ট বেছে নিন
rules-editor-accounts-many = { $count ->
    [one] { $count }টি অ্যাকাউন্ট
   *[other] { $count }টি অ্যাকাউন্ট
}
rules-editor-matches = গত { $days } দিনের { $mails }-এর সাথে মেলে
rules-editor-mails = { $count ->
    [one] { $count }টি মেল
   *[other] { $count }টি মেল
}
rules-editor-counting = যে মেলগুলির সাথে মেলে তা গোনা হচ্ছে…
rules-editor-show = সেগুলি দেখান
rules-editor-also-apply = এই { $count }টিতেও প্রয়োগ করুন
rules-editor-runs-katna = Katna-য় চলে, যতক্ষণ এই কম্পিউটার চালু থাকে।
rules-editor-runs-gmail = Gmail-এ চলে, তাই আপনার ফোনেও এবং এই কম্পিউটার বন্ধ থাকলেও কাজ করে।
rules-editor-runs-sieve = আপনার মেল সার্ভারে চলে, তাই আপনার ফোনেও এবং এই কম্পিউটার বন্ধ থাকলেও কাজ করে।
rules-note-gmail-action = Katna-য় চলে: Gmail ফিল্টার “{ $action }” করতে পারে না।
rules-note-sieve-action = Katna-য় চলে: আপনার মেল সার্ভারের নিয়ম “{ $action }” করতে পারে না।
rules-note-test = { $field } { $comparator }
rules-note-gmail-condition = Katna-য় চলে: Gmail ফিল্টার Katna-র মতো “{ $test }” যাচাই করতে পারে না।
rules-note-sieve-condition = Katna-য় চলে: আপনার মেল সার্ভারের নিয়ম Katna-র মতো “{ $test }” যাচাই করতে পারে না।
rules-note-order = Katna-য় চলে, যেমন অ্যাকাউন্টের আগের একটি নিয়ম চলে: নিয়মগুলি তালিকার ক্রমে চলে।
rules-note-gmail-stop = Katna-য় চলে: Gmail ফিল্টার পরের নিয়মগুলি চলা আটকাতে পারে না।
rules-note-gmail-forward = Katna-য় চলে: Gmail শুধু তার সেটিংসে যাচাই করা ঠিকানায় ফরোয়ার্ড করে, আর { $address } তাদের একটি নয়।
rules-note-gmail-folder = Katna-য় চলে: এই নিয়মের একটি ফোল্ডারের জন্য Gmail-এ কোনো লেবেল নেই।
rules-note-sieve-folder = Katna-য় চলে: এই নিয়মের একটি ফোল্ডার আপনার মেল সার্ভারে নেই।
rules-note-gmail-sign-in = Katna-য় চলে, যতক্ষণ না আপনি আবার Google-এ সাইন ইন করে Katna-কে Gmail ফিল্টার তৈরি করতে দেন।
rules-note-sieve-other-script = Katna-য় চলে: আপনার মেল সার্ভারে নিয়মের অন্য একটি স্ক্রিপ্ট (“{ $name }”) চালু আছে।
rules-note-gmail-failed = Katna-য় চলে: Gmail এটি গ্রহণ করেনি ({ $error })।
rules-note-sieve-failed = Katna-য় চলে: আপনার মেল সার্ভার এটি গ্রহণ করেনি ({ $error })।
rules-editor-cancel = বাতিল করুন
rules-editor-save = সেভ করুন
rules-editor-saving = সেভ করা হচ্ছে…
rules-editor-delete = নিয়ম মুছুন
rules-editor-delete-ask = এই নিয়মটি মুছবেন?
rules-editor-delete-keep = রেখে দিন
rules-editor-delete-confirm = মুছুন
rules-editor-needs-folder = প্রতিটি “এখানে সরান”-এর জন্য একটি ফোল্ডার এবং প্রতিটি “লেবেল যোগ করুন”-এর জন্য একটি লেবেল বেছে নিন।
rules-editor-needs-days = “এত দিন পরে পঠিত হিসেবে চিহ্নিত করুন”-এ 1 থেকে 3650-এর মধ্যে দিনের সংখ্যা লাগে।
rules-saved = নিয়ম সেভ করা হয়েছে
rules-saved-applied = { $count ->
    [one] নিয়ম সেভ করে { $count }টি মেলে প্রয়োগ করা হয়েছে
   *[other] নিয়ম সেভ করে { $count }টি মেলে প্রয়োগ করা হয়েছে
}
rules-apply-failed = নিয়ম সেভ হয়েছে, কিন্তু প্রয়োগ করা যায়নি: { $error }
rules-deleted = নিয়ম মোছা হয়েছে
rules-delete-failed = নিয়মটি মোছা যায়নি: { $error }
rules-change-failed = নিয়মগুলি বদলানো যায়নি: { $error }
