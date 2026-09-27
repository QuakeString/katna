# Katna Mail, Bengali (বাংলা).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = বন্ধ করুন
reader-back = ফিরে যান
reader-mark-unread = অপঠিত হিসেবে চিহ্নিত করুন
reader-move-to = এখানে সরান
reader-more = আরও
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
reader-me = আমাকে
reader-to = প্রাপক: { $names }
reader-starred = তারকাচিহ্নিত
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
tracking-not-opened = { $who } এখনও এটি খোলেননি
tracking-receipt = { $who } একটি পঠিত রসিদ পাঠিয়েছেন
tracking-receipt-displayed = পঠিত রসিদ: { $who } আপনার মেসেজ খুলেছেন
tracking-receipt-other = পঠিত রসিদ: { $who } আপনার মেসেজ না খুলেই মুছেছেন বা অন্যভাবে সামলেছেন

## Remote images and pictures

remote-hidden = এই মেসেজের ছবিগুলি লুকানো আছে।
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
print-preview-cancel = বাতিল করুন
print-preview-print = প্রিন্ট করুন
print-not-downloaded = (এখনও ডাউনলোড করা হয়নি।)
print-encrypted = (এনক্রিপ্ট করা। এর লেখা প্রিন্ট করতে Katna Mail-এ খুলুন।)
print-to = প্রাপক: { $addresses }
print-cc = Cc: { $addresses }

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = অ্যাটাচমেন্টগুলি পড়তে এই মেসেজটি খুলুন।
text-copy = কপি করুন
text-select-all = সব বেছে নিন
