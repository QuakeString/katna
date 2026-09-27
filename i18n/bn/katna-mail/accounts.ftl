# Katna Mail, Bengali (বাংলা).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Accounts

accounts-folder-pane = ফোল্ডার প্যান
accounts-folder-pane-detail = বাঁদিকের প্যানে কোন অ্যাকাউন্টের ফোল্ডার দেখাবে।
accounts-shown-one = একবারে একটি অ্যাকাউন্ট; অ্যাকাউন্ট কার্ডে বদলান
accounts-shown-all = সব অ্যাকাউন্ট, একটির পর একটি
accounts-row = অ্যাকাউন্ট
accounts-row-detail = ফোল্ডার প্যান আর অ্যাকাউন্ট মেনু অ্যাকাউন্টগুলি এই ক্রমে দেখায়; প্রথমটি ডিফল্ট। কোনো অ্যাকাউন্ট সরালে এই কম্পিউটারে তার মেলের Katna-র কপি মুছে যায়। মেল সার্ভারে থেকে যায়।
accounts-none = এখনও কোনো অ্যাকাউন্ট নেই।
accounts-kind-imported = ইমপোর্ট করা
accounts-picture-reset = ডেস্কটপের ছবি ব্যবহার করুন
accounts-picture-change = ছবি বদলান
accounts-picture-remove = ছবি সরান
accounts-rename = নাম বদলান
accounts-name-save = সেভ করুন
accounts-name-cancel = বাতিল করুন
accounts-name-placeholder = আপনার নাম
accounts-rename-failed = অ্যাকাউন্টের নাম বদলানো যায়নি: { $error }
accounts-move-up = উপরে সরান
accounts-move-down = নিচে সরান
accounts-drag = ক্রম বদলাতে টেনে আনুন
accounts-remove = সরান
accounts-delete-all-row = সব ডেটা মুছুন
accounts-delete-all-row-detail = নতুন করে শুরু করুন, নতুন ইনস্টলের মতো।
accounts-delete-all-about = এই কম্পিউটার থেকে প্রতিটি অ্যাকাউন্ট, সব সেভ করা মেল, পরিচিতি ও ক্যালেন্ডার, সার্চ ইনডেক্স, আপনার সেটিংস এবং সেভ করা পাসওয়ার্ড মুছে দেয়। আপনার মেল সার্ভারে কিছুই বদলায় না।
accounts-delete-all-open = Katna-র সব ডেটা মুছুন

## Settings > Accounts: snackbars after deleting

accounts-removed-local = { $address } Katna থেকে সরানো হয়েছে।
accounts-removed = { $address } Katna থেকে সরানো হয়েছে। এর মেল এখনও সার্ভারে আছে।
accounts-all-deleted = Katna-র সব ডেটা এই কম্পিউটার থেকে মুছে ফেলা হয়েছে।

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = { $address } সরাবেন?
accounts-remove-confirm = অ্যাকাউন্ট সরান
accounts-removing = সরানো হচ্ছে…
accounts-remove-local-mail = { $folders ->
    [0] এই অ্যাকাউন্টে ইমপোর্ট করা সব মেল
    [one] এই অ্যাকাউন্টে ইমপোর্ট করা সব মেল, তার ফোল্ডারে
   *[other] এই অ্যাকাউন্টে ইমপোর্ট করা সব মেল, তার { $folders }টি ফোল্ডারে
}
accounts-remove-local-settings = এর Katna সেটিংস
accounts-remove-mail = { $folders ->
    [0] Katna-য় সেভ করা এই অ্যাকাউন্টের সব মেল
    [one] Katna-য় সেভ করা এই অ্যাকাউন্টের সব মেল, তার ফোল্ডারে
   *[other] Katna-য় সেভ করা এই অ্যাকাউন্টের সব মেল, তার { $folders }টি ফোল্ডারে
}
accounts-remove-outbox = আউটবক্সে অপেক্ষারত এর মেসেজগুলি
accounts-remove-settings = এর সেভ করা পাসওয়ার্ড ও Katna সেটিংস
accounts-delete-all-title = Katna-র সব ডেটা মুছবেন?
accounts-delete-all-confirm = সবকিছু মুছুন
accounts-deleting = মোছা হচ্ছে…
accounts-delete-all-accounts = প্রতিটি অ্যাকাউন্ট, এবং Katna-য় সেভ করা সব মেল ও অ্যাটাচমেন্ট
accounts-delete-all-contacts = পরিচিতি, ক্যালেন্ডার ও সার্চ ইনডেক্স
accounts-delete-all-settings = সব সেটিংস, স্বাক্ষর ও কীবোর্ড শর্টকাট
accounts-delete-all-passwords = সেভ করা প্রতিটি পাসওয়ার্ড
accounts-deleted-heading = এই কম্পিউটার থেকে মুছে যাবে:
accounts-cannot-undo = এটি পূর্বাবস্থায় ফেরানো যাবে না।
accounts-server-delete-all = আপনার মেল সার্ভারে কিছুই বদলায় না: আপনার মেল সেখানেই থাকে, আর আবার অ্যাকাউন্ট যোগ করলে আবার ডাউনলোড হয়। ফাইল থেকে ইমপোর্ট করা মেল শুধু Katna-তেই আছে; ফাইলগুলিতে হাত দেওয়া হয় না।
accounts-server-local = এই মেল ফাইল থেকে ইমপোর্ট করা হয়েছিল, তাই এর একমাত্র কপি Katna-র কাছে। যে ফাইলগুলি থেকে এটি এসেছে সেগুলিতে হাত দেওয়া হয় না; ফিরে পেতে সেগুলি আবার ইমপোর্ট করুন।
accounts-server-remove = মেল সার্ভারে কিছুই বদলায় না: আপনার মেল সেখানেই থাকে, আর আবার অ্যাকাউন্টটি যোগ করলে আবার ডাউনলোড হয়।
accounts-confirm-word = মুছুন
accounts-confirm-placeholder = “{ accounts-confirm-word }” লিখুন
accounts-confirm-prompt = নিশ্চিত করতে “{ accounts-confirm-word }” লিখুন:
accounts-cancel = বাতিল করুন
