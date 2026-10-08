# Katna Mail, Bengali (বাংলা).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Navigation (the folders pane)

nav-labels = লেবেল
nav-folders = ফোল্ডার
nav-label-new = নতুন লেবেল তৈরি করুন
nav-folder-new = নতুন ফোল্ডার তৈরি করুন
nav-menu-check-mail = নতুন মেল যাচাই করুন
nav-menu-check-inbox = এই ইনবক্স যাচাই করুন
nav-unified-leave-out = একীভূত ইনবক্স থেকে বাদ দিন
nav-unified-bring-back = একীভূত ইনবক্সে ফিরিয়ে আনুন
nav-menu-sign-in-again = আবার সাইন ইন করুন
nav-menu-new-mail = এই অ্যাকাউন্ট থেকে নতুন মেল
nav-menu-account-settings = অ্যাকাউন্ট সেটিংস
nav-account-checked = সিঙ্ক হয়েছে · যাচাই করা হয়েছে { $ago }
nav-account-in-sync = সিঙ্ক হয়েছে
nav-account-connecting = সংযোগ করা হচ্ছে…
nav-account-offline = অফলাইন, আবার চেষ্টা করা হচ্ছে
nav-account-signed-out = { $provider } সাইন-ইনের মেয়াদ শেষ
nav-account-password-refused = পাসওয়ার্ড গ্রহণ করা হয়নি
nav-account-storage = { $total }-এর মধ্যে { $used } ব্যবহৃত
nav-menu-new-subfolder = ভেতরে নতুন ফোল্ডার
nav-menu-new-sublabel = ভেতরে নতুন লেবেল
nav-menu-rename = নাম বদলান
nav-menu-delete = মুছুন
nav-menu-empty-trash = ট্র্যাশ খালি করুন
nav-account-unnamed = অ্যাকাউন্ট { $number }
nav-all-accounts = সব অ্যাকাউন্ট
nav-expand = ফোল্ডার দেখান
nav-collapse = ফোল্ডার লুকান
storage-used = { $total }-এর { $percent }% ব্যবহৃত
storage-used-detail = { $address }: { $total }-এর { $used } ব্যবহৃত

## Special folders (the user's own folders keep their names)

folder-inbox = ইনবক্স
folder-starred = তারকাচিহ্নিত
folder-snoozed = স্নুজ করা
folder-unread = অপঠিত
folder-important = গুরুত্বপূর্ণ
folder-drafts = খসড়া
folder-sent = পাঠানো হয়েছে
folder-archive = আর্কাইভ
folder-spam = স্প্যাম
folder-trash = ট্র্যাশ
folder-all-mail = সব মেল
folder-scheduled = শিডিউল করা
folder-waiting = উত্তরের অপেক্ষায়
folder-waiting-short = অপেক্ষায়
folder-reminders = রিমাইন্ডার
folder-outbox = আউটবক্স
folder-activity = কার্যকলাপ

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = নতুন লেবেল
label-folder-new-title = নতুন ফোল্ডার
label-prompt = অনুগ্রহ করে নতুন লেবেলের নাম লিখুন:
label-folder-prompt = অনুগ্রহ করে নতুন ফোল্ডারের নাম লিখুন:
label-name-hint = লেবেলের নাম
label-folder-name-hint = ফোল্ডারের নাম
label-nest = লেবেলটি এর মধ্যে রাখুন:
label-folder-nest = ফোল্ডারটি এর মধ্যে রাখুন:
label-cancel = বাতিল করুন
label-create = তৈরি করুন
label-creating = তৈরি করা হচ্ছে…
label-created = “{ $name }” লেবেল তৈরি করা হয়েছে।
label-folder-created = “{ $name }” ফোল্ডার তৈরি করা হয়েছে।
label-rename-title = লেবেলের নাম বদলান
label-folder-rename-title = ফোল্ডারের নাম বদলান
label-rename = নাম বদলান
label-renaming = নাম বদলানো হচ্ছে…
label-renamed = লেবেলের নাম বদলে “{ $name }” করা হয়েছে।
label-folder-renamed = ফোল্ডারের নাম বদলে “{ $name }” করা হয়েছে।

## Deleting a folder or label (asked first)

folder-delete-title = “{ $name }” মুছবেন?
folder-delete-body = { $count ->
    [0] এতে কোনো মেল নেই। ফোল্ডারটি সার্ভার থেকে সরানো হবে, তাই ওয়েবমেল ও আপনার ফোন থেকেও এটি চলে যাবে।
   *[other] { $kind ->
        [conversation] { $count ->
            [one] এর { $count }টি কথোপকথন ট্র্যাশে যাবে, তাই আপনি এখনও সেটি ফিরিয়ে আনতে পারবেন।
           *[other] এর { $count }টি কথোপকথন ট্র্যাশে যাবে, তাই আপনি এখনও সেগুলি ফিরিয়ে আনতে পারবেন।
        }
       *[message] { $count ->
            [one] এর { $count }টি মেসেজ ট্র্যাশে যাবে, তাই আপনি এখনও সেটি ফিরিয়ে আনতে পারবেন।
           *[other] এর { $count }টি মেসেজ ট্র্যাশে যাবে, তাই আপনি এখনও সেগুলি ফিরিয়ে আনতে পারবেন।
        }
    } ফোল্ডারটি সার্ভার থেকে সরানো হবে, তাই ওয়েবমেল ও আপনার ফোন থেকেও এটি চলে যাবে।
}
folder-delete-forever-body = { $count ->
    [0] এতে কোনো মেল নেই। ফোল্ডারটি সার্ভার থেকে সরানো হবে, তাই ওয়েবমেল ও আপনার ফোন থেকেও এটি চলে যাবে।
   *[other] { $kind ->
        [conversation] { $count ->
            [one] এর { $count }টি কথোপকথন চিরতরে মুছে যাবে; এই অ্যাকাউন্টে কোনো ট্র্যাশ নেই।
           *[other] এর { $count }টি কথোপকথন চিরতরে মুছে যাবে; এই অ্যাকাউন্টে কোনো ট্র্যাশ নেই।
        }
       *[message] { $count ->
            [one] এর { $count }টি মেসেজ চিরতরে মুছে যাবে; এই অ্যাকাউন্টে কোনো ট্র্যাশ নেই।
           *[other] এর { $count }টি মেসেজ চিরতরে মুছে যাবে; এই অ্যাকাউন্টে কোনো ট্র্যাশ নেই।
        }
    } ফোল্ডারটি সার্ভার থেকে সরানো হবে, তাই ওয়েবমেল ও আপনার ফোন থেকেও এটি চলে যাবে।
}
folder-delete-label-body = লেবেলটি সরানো হবে। এর মেল সব মেল-এ এবং এর অন্য লেবেলগুলিতে থেকে যাবে।
folder-delete-confirm = ফোল্ডার মুছুন
folder-delete-label-confirm = লেবেল মুছুন
folder-deleted = “{ $name }” ফোল্ডার মুছে ফেলা হয়েছে
label-deleted = “{ $name }” লেবেল মুছে ফেলা হয়েছে
