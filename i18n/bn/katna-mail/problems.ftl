# Katna Mail, Bengali (বাংলা).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = মেল সার্ভার

problems-signed-out = { $provider } Katna-কে { $address } থেকে সাইন আউট করে দিয়েছে। মেল সিঙ্ক বন্ধ হয়ে গেছে।
problems-password-refused = { $provider } { $address }-এর পাসওয়ার্ড প্রত্যাখ্যান করেছে। সেটি হয়তো বদলে গেছে।
problems-no-answer = { $provider } { $address }-এর জন্য সাড়া দিচ্ছে না। Katna চেষ্টা চালিয়ে যাচ্ছে।
problems-offline = আপনি অফলাইন। আপনার মেল এখানেই আছে, আর আপনি যে মেল পাঠান তা আপনি ফিরে আসা পর্যন্ত অপেক্ষা করবে।
problems-accounts-need-you = { $count ->
    [one] 1টি অ্যাকাউন্টের আপনাকে দরকার
   *[other] { $count }টি অ্যাকাউন্টের আপনাকে দরকার
}
problems-show = দেখান
problems-later = পরে
problems-new-password = নতুন পাসওয়ার্ড
problems-try-again = আবার চেষ্টা করুন

## The New password card

problems-password-title = নতুন পাসওয়ার্ড
problems-password-detail = { $provider } { $address }-এর সেভ করা পাসওয়ার্ড প্রত্যাখ্যান করেছে। নতুনটি টাইপ করুন; রাখার আগে Katna সেটি যাচাই করবে।
problems-password-placeholder = পাসওয়ার্ড
problems-password-show = পাসওয়ার্ড দেখান
problems-password-hide = পাসওয়ার্ড লুকান
problems-password-cancel = বাতিল করুন
problems-password-save = সেভ করুন
problems-password-checking = যাচাই করা হচ্ছে…
problems-password-refused-again = { $provider } এই পাসওয়ার্ডটিও প্রত্যাখ্যান করেছে। যাচাই করে আবার চেষ্টা করুন।
problems-password-saved = { $address }-এর পাসওয়ার্ড সেভ করা হয়েছে। আপনার মেল আনা হচ্ছে…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = { $address }-এর মেল সার্ভার { $count ->
    [one] একটি মেসেজ সরানো গ্রহণ করেনি, তাই সেটি আগের জায়গায় ফিরে গেছে।
   *[other] { $count }টি মেসেজ সরানো গ্রহণ করেনি, তাই সেগুলি আগের জায়গায় ফিরে গেছে।
}
problems-refused-flags = { $address }-এর মেল সার্ভার { $count ->
    [one] একটি মেসেজ চিহ্নিত করা (পঠিত, তারকাচিহ্নিত…) গ্রহণ করেনি, তাই সেটি আগের মতো আছে।
   *[other] { $count }টি মেসেজ চিহ্নিত করা (পঠিত, তারকাচিহ্নিত…) গ্রহণ করেনি, তাই সেগুলি আগের মতো আছে।
}
problems-refused-label = { $address }-এর মেল সার্ভার { $count ->
    [one] একটি মেসেজের লেবেল বদলানো গ্রহণ করেনি, তাই সেটি আগের মতো আছে।
   *[other] { $count }টি মেসেজের লেবেল বদলানো গ্রহণ করেনি, তাই সেগুলি আগের মতো আছে।
}
problems-refused-delete = { $address }-এর মেল সার্ভার { $count ->
    [one] একটি মেসেজ মোছা গ্রহণ করেনি, তাই সেটি ফিরে এসেছে।
   *[other] { $count }টি মেসেজ মোছা গ্রহণ করেনি, তাই সেগুলি ফিরে এসেছে।
}
problems-refused-other = { $address }-এর মেল সার্ভার { $count ->
    [one] একটি পরিবর্তন গ্রহণ করেনি, তাই Katna সেটি আগের মতো ফিরিয়ে দিয়েছে।
   *[other] { $count }টি পরিবর্তন গ্রহণ করেনি, তাই Katna সেগুলি আগের মতো ফিরিয়ে দিয়েছে।
}
problems-details = বিস্তারিত

## Katna's background service (katna-daemon) isn't running

service-starting = Katna-র ব্যাকগ্রাউন্ড সার্ভিস চালু হচ্ছে…
service-failed = Katna-র ব্যাকগ্রাউন্ড সার্ভিস চালু হচ্ছে না, তাই মেল সিঙ্ক হচ্ছে না।
service-start-again = আবার চালু করুন
service-started-again = Katna-র ব্যাকগ্রাউন্ড সার্ভিস বন্ধ হয়ে গিয়েছিল, আবার চালু করা হয়েছে।
service-details-title = সার্ভিসটি কেন চালু হচ্ছে না
service-details-body = এটি কপি করে আপনার রিপোর্টের সাথে পাঠান। এতে কোনো মেল বা পাসওয়ার্ড নেই।
service-details-copy = কপি করুন
service-details-close = বন্ধ করুন
