# Katna Mail, Bengali (বাংলা).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = বন্ধ করুন
reader-back = ফিরে যান
reader-mark-unread = অপঠিত হিসেবে চিহ্নিত করুন
reader-move-to = এখানে সরান
reader-snooze = স্নুজ করুন
reader-remind = মনে করিয়ে দিন
reader-more = আরও
reader-original-colors = আসল রং দেখান
reader-dark-colors = গাঢ় রঙে দেখান
reader-print-all = সব প্রিন্ট করুন
reader-new-window = নতুন উইন্ডোতে
reader-position = { $total }টির মধ্যে { $position }
reader-newer = নতুন
reader-older = পুরনো

## Reading pane: the conversation

reader-removed = এই কথোপকথনটি সরিয়ে দেওয়া হয়েছে।
reader-no-subject = (কোনো বিষয় নেই)
reader-collapse-all = সব সঙ্কুচিত করুন
reader-expand-all = সব প্রসারিত করুন
reader-unknown-sender = (অজানা প্রেরক)
reader-date-ago = { $date } ({ $ago })
reader-sending = পাঠানো হচ্ছে…
reader-me = আমাকে
reader-to = প্রাপক: { $names }
reader-to-label = প্রাপক:
reader-tick-delivered = পৌঁছেছে { $when }
reader-tick-no-bounce = পাঠানো হয়েছে { $when }; ব্যর্থতার কোনো বার্তা ফিরে আসেনি, তাই সম্ভবত পৌঁছেছে
reader-tick-bounced = পৌঁছায়নি: { $when } ফিরে এসেছে
reader-tick-read = পড়া হয়েছে { $when } (পঠিত রসিদ)
reader-tick-opened = খোলা হয়েছে, শেষবার { $when } (ওপেন ট্র্যাকিং)
reader-starred = তারকাচিহ্নিত
reader-chip-remove = { $label } সরান
reader-not-starred = তারকাচিহ্নিত নয়
reader-too-long = মেসেজটি এত বড় যে পুরোটা দেখানো যাচ্ছে না।
reader-encrypted-images = এনক্রিপ্ট করা মেলে ওয়েব থেকে ছবি কখনো লোড করা হয় না।
reader-window-failed = নতুন উইন্ডো খোলা যায়নি।

## Reading pane: message details (opened from "to me")

reader-details-from = প্রেরক:
reader-details-to = প্রাপক:
reader-details-cc = cc:
reader-details-date = তারিখ:
reader-details-subject = বিষয়:

## Reading pane: downloading a message

reader-downloading = সার্ভার থেকে এই মেসেজটি ডাউনলোড করা হচ্ছে…
reader-download-failed = এই মেসেজটি ডাউনলোড করা যায়নি।
reader-download-failed-reason = এই মেসেজটি ডাউনলোড করা যায়নি। { $reason }
reader-download-offline = এই অ্যাকাউন্টটি অফলাইন। এই মেসেজটি ডাউনলোড করতে অনলাইনে যান।
reader-try-again = আবার চেষ্টা করুন

## Reply row

reply-reply = উত্তর দিন
reply-reply-all = সবাইকে উত্তর দিন
reply-forward = ফরোয়ার্ড করুন

## Encrypted and signed mail

security-decrypting = ডিক্রিপ্ট করা হচ্ছে…
security-checking = স্বাক্ষর যাচাই করা হচ্ছে…
security-partly-encrypted = এই মেসেজের শুধু একটি অংশ এনক্রিপ্ট করা। বাকি অংশ সুরক্ষার বাইরে যোগ করা হয়েছে এবং যে কেউ পাঠিয়ে থাকতে পারে।
security-partly-signed = এই মেসেজের শুধু একটি অংশে স্বাক্ষর আছে। বাকি অংশ সুরক্ষার বাইরে যোগ করা হয়েছে এবং যে কেউ পাঠিয়ে থাকতে পারে।
security-encrypted = এনক্রিপ্ট করা মেসেজ
security-encrypted-smime = এনক্রিপ্ট করা মেসেজ (S/MIME)
security-no-key = এই মেসেজ ডিক্রিপ্ট করা যাচ্ছে না: এটি এমন একটি কী-এর জন্য এনক্রিপ্ট করা হয়েছে যা আপনার কাছে নেই।
security-cancelled = ডিক্রিপ্ট করা বাতিল করা হয়েছে।
security-damaged = এই মেসেজ ডিক্রিপ্ট করা যাচ্ছে না: এনক্রিপ্ট করা ডেটা ক্ষতিগ্রস্ত বা পরিবর্তিত হয়েছে।
security-decrypt-unavailable = এই মেসেজ ডিক্রিপ্ট করা যাচ্ছে না: এনক্রিপ্ট করা মেল পড়তে { $tool } ইনস্টল করুন।
security-decrypt-failed = এই মেসেজ ডিক্রিপ্ট করা যাচ্ছে না: { $reason }
security-unknown-signer = অজানা স্বাক্ষরকারী
security-signed-verified = স্বাক্ষর করেছেন { $signer } · যাচাই করা
security-signed-not-sender = স্বাক্ষর করেছেন { $signer }, যিনি প্রেরক নন
security-signed-untrusted = স্বাক্ষর করেছেন { $signer }, এমন একটি কী দিয়ে যা আপনি অবিশ্বস্ত হিসেবে চিহ্নিত করেছেন
security-signed-unverified = স্বাক্ষর করেছেন { $signer } · কী যাচাই করা নেই
security-bad-signature = ভুল স্বাক্ষর: স্বাক্ষরের পরে এই মেসেজ বদলানো হয়েছে, অথবা স্বাক্ষরটি জাল।
security-signature-expired = স্বাক্ষর করেছেন { $signer } · স্বাক্ষরের মেয়াদ শেষ হয়ে গেছে
security-key-expired = স্বাক্ষর করেছেন { $signer } · এরপর কী-এর মেয়াদ শেষ হয়ে গেছে
security-key-revoked = স্বাক্ষর করেছেন { $signer }, এমন একটি কী দিয়ে যা প্রত্যাহার করা হয়েছে
security-missing-key = এমন একটি কী দিয়ে স্বাক্ষর করা যা আপনার কাছে নেই, তাই যাচাই করা যাচ্ছে না
security-missing-key-id = এমন একটি কী ({ $key }) দিয়ে স্বাক্ষর করা যা আপনার কাছে নেই, তাই যাচাই করা যাচ্ছে না
security-signature-unavailable = স্বাক্ষরিত; স্বাক্ষর যাচাই করতে { $tool } ইনস্টল করুন
security-signature-error = স্বাক্ষর যাচাই করা যায়নি।
security-look-up-key = কী খুঁজুন

## The key popover: the details of the key a message was signed with, and
## a key to import (looked up, or attached to the message)

key-card-verified = যাচাই করা স্বাক্ষর
key-card-verified-detail = স্বাক্ষরটি ঠিক আছে এবং আপনি এই কী বিশ্বাস করেন।
key-card-unverified = স্বাক্ষর যাচাই করা নেই
key-card-unverified-detail = স্বাক্ষরটি ঠিক আছে, কিন্তু কী-টি যে তাঁরই, তা কিছুই নিশ্চিত করে না। তাঁর সাথে ফিঙ্গারপ্রিন্ট মিলিয়ে নিন, তারপর GnuPG-তে কী-টি বিশ্বাস করুন (Kleopatra বা gpg --edit-key)।
key-card-not-sender = অন্য কেউ স্বাক্ষর করেছেন
key-card-not-sender-detail = স্বাক্ষরটি ঠিক আছে, কিন্তু কী-টি প্রেরকের নয়।
key-card-untrusted = কী বিশ্বস্ত নয়
key-card-untrusted-detail = আপনি GnuPG-তে এই কী অবিশ্বস্ত হিসেবে চিহ্নিত করেছেন।
key-card-signature-expired = স্বাক্ষরের মেয়াদ শেষ
key-card-signature-expired-detail = স্বাক্ষরটি ঠিক ছিল, কিন্তু এর মেয়াদ শেষ হয়ে গেছে।
key-card-key-expired = কী-এর মেয়াদ শেষ
key-card-key-expired-detail = স্বাক্ষরটি ঠিক আছে, কিন্তু এরপর কী-এর মেয়াদ শেষ হয়ে গেছে।
key-card-key-revoked = কী প্রত্যাহার করা হয়েছে
key-card-key-revoked-detail = এর মালিক এই কী প্রত্যাহার করেছেন, তাই স্বাক্ষরটি বিশ্বাস করা যায় না।
key-card-bad = ভুল স্বাক্ষর
key-card-bad-detail = স্বাক্ষরের পরে এই মেসেজ বদলানো হয়েছে, অথবা স্বাক্ষরটি জাল।
key-card-signed-by = স্বাক্ষরকারী
key-card-belongs-to = মালিক
key-card-fingerprint = ফিঙ্গারপ্রিন্ট
key-card-signed = স্বাক্ষরের সময়
key-card-key = কী
key-card-kind = { $standard }, { $algorithm }
key-card-created = তৈরি
key-card-expires = মেয়াদ শেষ
key-card-never = কখনো না
key-card-issued-by = ইস্যুকারী
key-card-found-in = যেখানে পাওয়া গেছে
key-card-keyring = আপনার GnuPG কী-রিং
key-card-copy = ফিঙ্গারপ্রিন্ট কপি করুন
key-card-import-title = এই কী ইমপোর্ট করবেন?
key-card-from-directory = { $domain }-এর কী ডিরেক্টরিতে পাওয়া গেছে।
key-card-from-attachment = { $name } সংযুক্তি থেকে।
key-card-import-note = এরপর Katna এই ব্যক্তির স্বাক্ষর যাচাই করতে এবং তাঁকে এনক্রিপ্ট করা মেল পাঠাতে পারবে। কী-টি পুরোপুরি বিশ্বাস করতে তাঁর সাথে ফিঙ্গারপ্রিন্ট মিলিয়ে নিন।
key-card-cancel = বাতিল করুন
key-card-import = কী ইমপোর্ট করুন
key-card-looking-up = কী খোঁজা হচ্ছে…
key-card-looking-up-detail = { $domain }-এর কী ডিরেক্টরিতে জিজ্ঞেস করা হচ্ছে।
key-card-not-found = কোনো কী পাওয়া যায়নি
key-card-not-found-detail = { $domain } এই ঠিকানার জন্য কোনো কী প্রকাশ করে না। প্রেরককে তাঁর কী পাঠাতে বলুন।
key-card-not-kept = যে কী পাওয়া গেছে সেটি ব্যবহার করা যাবে না।
key-card-failed = কী পাওয়া যায়নি

## Sender checks: the banner on mail that failed the checks the user's mail
## provider ran on its sender (DMARC, DKIM, SPF), and the "?" on the picture
## of a sender nothing confirmed

sender-failed-title = এটি হয়তো { $domain } থেকে আসেনি
sender-failed-body = এটি { $provider }-এর প্রেরক যাচাইয়ে উত্তীর্ণ হয়নি। লিঙ্ক, সংযুক্তি ও উত্তরের ব্যাপারে সাবধান থাকুন।
sender-provider-unknown = আপনার মেল পরিষেবা
sender-details = বিস্তারিত
sender-details-hide = বিস্তারিত লুকান
sender-looks-safe = নিরাপদ মনে হচ্ছে
sender-move-to-spam = স্প্যামে সরান
sender-checked-by = যাচাই করেছে { $provider }
sender-checked-by-server = যাচাই করেছে { $provider } ({ $server })
sender-dmarc = প্রেরকের ডোমেন (DMARC)
sender-dkim = স্বাক্ষর (DKIM)
sender-spf = পাঠানোর সার্ভার (SPF)
sender-result-pass = উত্তীর্ণ
sender-result-fail = ব্যর্থ
sender-result-unsure = নিশ্চিত নয়
sender-result-none = নেই
sender-result-missing = যাচাই করা হয়নি
sender-dmarc-pass = { $domain } এই প্রেরককে নিশ্চিত করে।
sender-dmarc-fail = { $domain } তার মেল যেভাবে পাঠানো হয় বলে জানায়, এই মেল তার সাথে মেলে না।
sender-dmarc-none = { $domain } তার মেলের জন্য কোনো নিয়ম প্রকাশ করে না।
sender-dkim-pass = { $domain } স্বাক্ষর করেছে।
sender-dkim-fail = { $domain }-এর স্বাক্ষর এই মেলের সাথে মেলে না।
sender-dkim-none = মেসেজটিতে স্বাক্ষর করা হয়নি।
sender-spf-pass = { $domain }-এর তালিকাভুক্ত একটি সার্ভার থেকে পাঠানো।
sender-spf-fail = এমন একটি সার্ভার থেকে পাঠানো যা { $domain }-এর তালিকায় নেই।
sender-spf-none = { $domain } তার সার্ভারের তালিকা দেয় না।
sender-check-unsure = যাচাই থেকে স্পষ্ট উত্তর পাওয়া যায়নি।
sender-unconfirmed = { $provider } নিশ্চিত করতে পারেনি যে এটি { $domain } থেকে এসেছে। যে কেউ যেকোনো প্রেরকের নাম লিখতে পারে।
sender-link-title = এই লিঙ্ক খুলবেন?
sender-link-body = এই মেল প্রেরক যাচাইয়ে উত্তীর্ণ হয়নি। লিঙ্কটি যাচ্ছে { $host }-এ:
sender-link-cancel = বাতিল করুন
sender-link-open = খুলুন
tracking-opened = { $who } এটি { $count ->
    [one] একবার
   *[other] { $count } বার
} খুলেছেন, শেষবার { $when }
tracking-opens-clicks = { $who } এটি { $opens ->
    [one] একবার
   *[other] { $opens } বার
} খুলেছেন এবং { $clicks ->
    [one] একবার
   *[other] { $clicks } বার
} লিঙ্ক খুলেছেন, শেষবার { $when }
tracking-clicked = { $who } { $clicks ->
    [one] একবার
   *[other] { $clicks } বার
} লিঙ্ক খুলেছেন, শেষবার { $when }
tracking-maybe-opened = { $who } হয়তো এটি খুলেছেন (গোপনীয়তার জন্য Apple Mail ছবি লোড করে)
tracking-seen-none = এখনও কেউ এটি খোলেননি বা কোনো লিঙ্ক খোলেননি
tracking-receipt = { $who } একটি পঠিত রসিদ পাঠিয়েছেন
tracking-receipt-read = { $who } এটি পড়েছেন (রিড রিসিপ্ট), { $when }
tracking-receipt-displayed = পঠিত রসিদ: { $who } আপনার মেসেজ খুলেছেন
tracking-receipt-other = পঠিত রসিদ: { $who } আপনার মেসেজ না খুলেই মুছেছেন বা অন্যভাবে সামলেছেন

## Remote images and pictures

remote-hidden = এই মেসেজের ছবিগুলি লুকানো আছে।
remote-hidden-unconfirmed = ছবি লুকানো আছে: প্রেরককে নিশ্চিত করা যায়নি।
remote-hidden-failed = ছবি লুকানো আছে: এই মেল প্রেরক যাচাইয়ে উত্তীর্ণ হয়নি।
remote-show = ছবি দেখান
remote-always-show = এই প্রেরকের ছবি সবসময় দেখান
remote-picture-use = ব্যবহার করুন
remote-picture-too-big = 8 MB বা তার কম আকারের ছবি বেছে নিন।
remote-picture-type = PNG, JPEG, GIF, WebP বা SVG ছবি বেছে নিন।
remote-picture-read-failed = ছবিটি পড়া যাচ্ছে না: { $error }
remote-picture-keep-failed = ছবিটি রাখা যাচ্ছে না: { $error }
remote-picture-remove-failed = ছবিটি সরানো যাচ্ছে না: { $error }

## Attachments

attachment-count = { $count ->
    [one] একটি অ্যাটাচমেন্ট
   *[other] { $count }টি অ্যাটাচমেন্ট
}
attachment-save = সেভ করুন
attachment-forward = ফরোয়ার্ড করুন
attachment-save-all = সব সেভ করুন
attachment-save-all-tooltip = সব অ্যাটাচমেন্ট একটি ফোল্ডারে সেভ করুন
attachment-save-here = এখানে সেভ করুন
attachment-not-downloaded = এই মেসেজটি ডাউনলোড করা হয়নি।
attachment-not-found = মেসেজে এই অ্যাটাচমেন্টটি পাওয়া যায়নি।
attachment-read-failed = { $name } পড়া যায়নি
attachment-numbered = অ্যাটাচমেন্ট { $number }
attachment-saved-all = { $count ->
    [one] { $place }-এ { $count }টি ফাইল সেভ করা হয়েছে
   *[other] { $place }-এ { $count }টি ফাইল সেভ করা হয়েছে
}
attachment-saved-some = { $total ->
    [one] { $place }-এ { $total }টির মধ্যে { $saved }টি ফাইল সেভ করা হয়েছে। { $failed } সেভ করা যায়নি
   *[other] { $place }-এ { $total }টির মধ্যে { $saved }টি ফাইল সেভ করা হয়েছে। { $failed } সেভ করা যায়নি
}
attachment-saved-to = { $path }-এ সেভ করা হয়েছে
attachment-save-failed = { $name } সেভ করা যায়নি: { $error }
attachment-open-failed = { $name } খোলা যায়নি: { $error }
attachment-risky = এই ফাইলটি কোনো প্রোগ্রাম চালাতে পারে, তাই Katna এটি খোলে না। এর বদলে এটি সেভ করুন।
attachment-encrypted-open = এই ফাইলটি এনক্রিপ্ট করা অবস্থায় এসেছে। অন্য কোথাও খুলতে এটি সেভ করুন।

## Printing

print-failed = প্রিন্ট করা যায়নি: { $error }
print-no-font = কোনো ফন্ট পাওয়া যায়নি
print-opened-as-pdf = PDF হিসেবে খোলা হয়েছে, সেখান থেকে প্রিন্ট করুন।
print-preview-title = প্রিন্ট প্রিভিউ
print-preview-laying-out = পৃষ্ঠাগুলি সাজানো হচ্ছে…
print-preview-pages = { $count ->
    [one] { $count }টি পৃষ্ঠা
   *[other] { $count }টি পৃষ্ঠা
}
print-preview-more = { $count ->
    [one] এবং আরও { $count }টি পৃষ্ঠা
   *[other] এবং আরও { $count }টি পৃষ্ঠা
}
print-preview-failed = পৃষ্ঠাগুলি দেখানো যায়নি
print-preview-paper = কাগজ
print-preview-a4 = A4
print-preview-letter = লেটার
print-preview-layout = লেআউট
print-preview-as-shown = যেমন দেখাচ্ছে
print-preview-simple = সাধারণ লেখা
print-preview-backgrounds = ব্যাকগ্রাউন্ড
print-preview-cancel = বাতিল করুন
print-preview-print = প্রিন্ট করুন
print-not-downloaded = (এখনও ডাউনলোড করা হয়নি।)
print-encrypted = (এনক্রিপ্ট করা। এর লেখা প্রিন্ট করতে Katna Mail-এ খুলুন।)
print-to = প্রাপক: { $addresses }
print-cc = Cc: { $addresses }

## Message text (right-click menu in the reading pane)

text-pin = উপরে পিন করুন
text-copy-address = ঠিকানা কপি করুন

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = অ্যাটাচমেন্টগুলি পড়তে এই মেসেজটি খুলুন।
text-copy = কপি করুন
text-select-all = সব বেছে নিন
