# Katna Mail, Bengali (বাংলা).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Why a message has not gone out (one line under its subject)

outbox-not-sent = পাঠানো হয়নি, কারণ { $reason }।
outbox-retrying = এখনও পাঠানো হয়নি, কারণ { $reason }। Katna নিজে থেকেই আবার চেষ্টা করবে।
outbox-waiting-sign-in = আপনি { $address }-এ আবার সাইন ইন করার অপেক্ষায়। তখনই এটি চলে যাবে।
outbox-waiting-password = { $address }-এর নতুন পাসওয়ার্ডের অপেক্ষায়। তখনই এটি চলে যাবে।
outbox-waiting-connection = সংযোগের অপেক্ষায়। আপনি আবার অনলাইন হলে এটি চলে যাবে।

## Reasons, read from the mail server's answer (they follow "because")

outbox-reason-no-recipients = এর কোনো প্রাপক নেই
outbox-reason-address = যে ঠিকানায় পাঠানো হচ্ছে তার একটির অস্তিত্ব নেই
outbox-reason-too-large = এটি মেল সার্ভারের জন্য খুব বড়
outbox-reason-blocked = মেল সার্ভার এটি আটকে দিয়েছে
outbox-reason-gone = এই কম্পিউটারে এর কপিটি আর নেই
outbox-reason-refused = মেল সার্ভার এটি প্রত্যাখ্যান করেছে

## Buttons and notes

outbox-try-again = আবার চেষ্টা করুন
outbox-edit = সম্পাদনা করুন
outbox-delete = মুছুন
outbox-deleted = আউটবক্স থেকে মোছা হয়েছে
outbox-sending-again = আবার পাঠানো হচ্ছে…

outbox-snackbar-not-sent = “{ $subject }” পাঠানো হয়নি, কারণ { $reason }।
outbox-open = আউটবক্স
