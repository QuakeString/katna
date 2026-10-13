# Katna Mail, Assamese (অসমীয়া).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Navigation (the folders pane)

nav-labels = লেবেলসমূহ
nav-folders = ফ'ল্ডাৰসমূহ
nav-label-new = নতুন লেবেল সৃষ্টি কৰক
nav-folder-new = নতুন ফ'ল্ডাৰ সৃষ্টি কৰক
nav-menu-check-mail = নতুন মেইল পৰীক্ষা কৰক
nav-menu-check-inbox = এই ইনবক্স পৰীক্ষা কৰক
nav-unified-leave-out = একত্ৰিত ইনবক্সৰ পৰা বাদ দিয়ক
nav-unified-bring-back = একত্ৰিত ইনবক্সলৈ ঘূৰাই আনক
nav-menu-sign-in-again = পুনৰ ছাইন ইন কৰক
nav-menu-new-mail = এই একাউণ্টৰ পৰা নতুন মেইল
nav-menu-account-settings = একাউণ্ট ছেটিংছ
nav-account-checked = ছিংক হৈ আছে · { $ago } পৰীক্ষা কৰা হৈছিল
nav-account-in-sync = ছিংক হৈ আছে
nav-account-connecting = সংযোগ কৰি আছে…
nav-account-offline = অফলাইন, পুনৰ চেষ্টা কৰি আছে
nav-account-signed-out = { $provider } ছাইন ইনৰ ম্যাদ উকলিছে
nav-account-password-refused = পাছৱৰ্ড অগ্ৰাহ্য কৰা হ'ল
nav-account-storage = { $total }ৰ { $used } ব্যৱহৃত
nav-menu-new-subfolder = ভিতৰত নতুন ফ'ল্ডাৰ
nav-menu-new-sublabel = ভিতৰত নতুন লেবেল
nav-menu-rename = নাম সলনি কৰক
nav-menu-delete = মচক
nav-menu-empty-trash = ট্ৰেছ খালী কৰক
nav-account-unnamed = একাউণ্ট { $number }
nav-all-accounts = সকলো একাউণ্ট
nav-expand = ফ'ল্ডাৰ দেখুৱাওক
nav-collapse = ফ'ল্ডাৰ লুকুৱাওক
storage-used = { $total }ৰ { $percent }% ব্যৱহৃত
storage-used-detail = { $address }: { $total }ৰ { $used } ব্যৱহৃত

## Special folders (the user's own folders keep their names)

folder-inbox = ইনবক্স
folder-starred = তৰাচিহ্নিত
folder-snoozed = স্নুজ কৰা
folder-unread = নপঢ়া
folder-important = গুৰুত্বপূৰ্ণ
folder-drafts = ড্ৰাফ্ট
folder-sent = প্ৰেৰিত
folder-archive = আৰ্কাইভ
folder-spam = স্পাম
folder-trash = ট্ৰেছ
folder-all-mail = সকলো মেইল
folder-scheduled = নিৰ্ধাৰিত
folder-waiting = উত্তৰৰ অপেক্ষাত
folder-waiting-short = অপেক্ষাত
folder-reminders = সোঁৱৰণী
folder-outbox = আউটবক্স
folder-activity = কাৰ্যকলাপ
folder-not-on-account = এই একাউণ্টত এনে কোনো ফ'ল্ডাৰ নাই।

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = নতুন লেবেল
label-folder-new-title = নতুন ফ'ল্ডাৰ
label-prompt = অনুগ্ৰহ কৰি নতুন লেবেলৰ নাম দিয়ক:
label-folder-prompt = অনুগ্ৰহ কৰি নতুন ফ'ল্ডাৰৰ নাম দিয়ক:
label-name-hint = লেবেলৰ নাম
label-folder-name-hint = ফ'ল্ডাৰৰ নাম
label-nest = লেবেলটো ইয়াৰ তলত ৰাখক:
label-folder-nest = ফ'ল্ডাৰটো ইয়াৰ তলত ৰাখক:
label-cancel = বাতিল কৰক
label-create = সৃষ্টি কৰক
label-creating = সৃষ্টি কৰি থকা হৈছে…
label-created = “{ $name }” লেবেল সৃষ্টি কৰা হ'ল।
label-folder-created = “{ $name }” ফ'ল্ডাৰ সৃষ্টি কৰা হ'ল।
label-rename-title = লেবেলৰ নাম সলনি কৰক
label-folder-rename-title = ফ'ল্ডাৰৰ নাম সলনি কৰক
label-rename = নাম সলনি কৰক
label-renaming = নাম সলনি কৰি থকা হৈছে…
label-renamed = লেবেলৰ নাম সলনি কৰি “{ $name }” কৰা হ'ল।
label-folder-renamed = ফ'ল্ডাৰৰ নাম সলনি কৰি “{ $name }” কৰা হ'ল।

## Deleting a folder or label (asked first)

folder-delete-title = “{ $name }” মচিবনে?
folder-delete-body = { $count ->
    [0] ইয়াত কোনো মেইল নাই। ফ'ল্ডাৰটো ছাৰ্ভাৰৰ পৰা আঁতৰোৱা হয়, সেয়ে ৱেবমেইল আৰু আপোনাৰ ফোনতো ই নাথাকিব।
   *[other] { $kind ->
        [conversation] { $count ->
            [one] ইয়াৰ { $count }টা কথোপকথন ট্ৰেছলৈ যায়, সেয়ে আপুনি তথাপি ঘূৰাই আনিব পাৰে।
           *[other] ইয়াৰ { $count }টা কথোপকথন ট্ৰেছলৈ যায়, সেয়ে আপুনি তথাপি ঘূৰাই আনিব পাৰে।
        }
       *[message] { $count ->
            [one] ইয়াৰ { $count }টা বাৰ্তা ট্ৰেছলৈ যায়, সেয়ে আপুনি তথাপি ঘূৰাই আনিব পাৰে।
           *[other] ইয়াৰ { $count }টা বাৰ্তা ট্ৰেছলৈ যায়, সেয়ে আপুনি তথাপি ঘূৰাই আনিব পাৰে।
        }
    } ফ'ল্ডাৰটো ছাৰ্ভাৰৰ পৰা আঁতৰোৱা হয়, সেয়ে ৱেবমেইল আৰু আপোনাৰ ফোনতো ই নাথাকিব।
}
folder-delete-forever-body = { $count ->
    [0] ইয়াত কোনো মেইল নাই। ফ'ল্ডাৰটো ছাৰ্ভাৰৰ পৰা আঁতৰোৱা হয়, সেয়ে ৱেবমেইল আৰু আপোনাৰ ফোনতো ই নাথাকিব।
   *[other] { $kind ->
        [conversation] { $count ->
            [one] ইয়াৰ { $count }টা কথোপকথন চিৰদিনৰ বাবে মচা হয়; এই একাউণ্টত ট্ৰেছ নাই।
           *[other] ইয়াৰ { $count }টা কথোপকথন চিৰদিনৰ বাবে মচা হয়; এই একাউণ্টত ট্ৰেছ নাই।
        }
       *[message] { $count ->
            [one] ইয়াৰ { $count }টা বাৰ্তা চিৰদিনৰ বাবে মচা হয়; এই একাউণ্টত ট্ৰেছ নাই।
           *[other] ইয়াৰ { $count }টা বাৰ্তা চিৰদিনৰ বাবে মচা হয়; এই একাউণ্টত ট্ৰেছ নাই।
        }
    } ফ'ল্ডাৰটো ছাৰ্ভাৰৰ পৰা আঁতৰোৱা হয়, সেয়ে ৱেবমেইল আৰু আপোনাৰ ফোনতো ই নাথাকিব।
}
folder-delete-label-body = লেবেলটো আঁতৰোৱা হয়। ইয়াৰ মেইল সকলো মেইলত আৰু ইয়াৰ আন লেবেলবোৰত থাকে।
folder-delete-confirm = ফ'ল্ডাৰ মচক
folder-delete-label-confirm = লেবেল মচক
folder-deleted = “{ $name }” ফ'ল্ডাৰ মচা হ'ল
label-deleted = “{ $name }” লেবেল মচা হ'ল
