# Katna Mail, Bengali (বাংলা).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = প্রাথমিক
tab-promotions = প্রচার
tab-social = সামাজিক
tab-updates = আপডেট
tab-forums = ফোরাম
tab-focused = ফোকাসড
tab-other = অন্যান্য
tab-inbox = ইনবক্স
tab-newsletters = নিউজলেটার
tab-notifications = বিজ্ঞপ্তি
tab-provider-other = Katna সাজিয়েছে

## Mail list: toolbar

list-select = বেছে নিন
list-refresh = রিফ্রেশ করুন
list-back-to-top = উপরে ফিরে যান
list-checking = নতুন মেল যাচাই করা হচ্ছে…
list-more = আরও
list-mark-read = পঠিত হিসেবে চিহ্নিত করুন
list-mark-unread = অপঠিত হিসেবে চিহ্নিত করুন
list-move-to = এখানে সরান
list-archive = আর্কাইভ করুন
list-spam = স্প্যাম হিসেবে রিপোর্ট করুন
list-delete = মুছুন
list-snooze = স্নুজ করুন
list-unsnooze = স্নুজ বাতিল করুন
list-newer = নতুন
list-older = পুরনো
list-range = { $total }টির মধ্যে { $first }–{ $last }
list-range-about = প্রায় { $total }টির মধ্যে { $first }–{ $last }
list-results = “{ $query }”-এর ফলাফল
list-results-corrected = “{ $query }”-এর ফলাফল দেখানো হচ্ছে
list-search-instead = এর বদলে “{ $query }” খুঁজুন
list-search-no-index = সার্চ এখনও প্রস্তুত নয়: ইনডেক্স এখনও তৈরি হয়নি।
list-search-not-ready = সার্চ এখনও প্রস্তুত নয়: { $error }
list-files-more = +{ $count }
list-replied = আপনি উত্তর দিয়েছেন

## Mail list: Select menu (which lines to tick)

list-pick-all = সব
list-pick-none = কোনোটিই নয়
list-pick-read = পঠিত
list-pick-unread = অপঠিত
list-pick-starred = তারকাচিহ্নিত
list-pick-unstarred = তারকাচিহ্নহীন

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] { $count }টি কথোপকথন বেছে নেওয়া হয়েছে।
       *[other] সবকটি { $count }টি কথোপকথন বেছে নেওয়া হয়েছে।
    }
   *[message] { $count ->
        [one] { $count }টি মেসেজ বেছে নেওয়া হয়েছে।
       *[other] সবকটি { $count }টি মেসেজ বেছে নেওয়া হয়েছে।
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder }-এর { $count }টি কথোপকথন বেছে নেওয়া হয়েছে।
       *[other] { $folder }-এর সবকটি { $count }টি কথোপকথন বেছে নেওয়া হয়েছে।
    }
   *[message] { $count ->
        [one] { $folder }-এর { $count }টি মেসেজ বেছে নেওয়া হয়েছে।
       *[other] { $folder }-এর সবকটি { $count }টি মেসেজ বেছে নেওয়া হয়েছে।
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] স্ক্রিনের { $count }টি কথোপকথন বেছে নেওয়া হয়েছে।
       *[other] স্ক্রিনের সবকটি { $count }টি কথোপকথন বেছে নেওয়া হয়েছে।
    }
   *[message] { $count ->
        [one] স্ক্রিনের { $count }টি মেসেজ বেছে নেওয়া হয়েছে।
       *[other] স্ক্রিনের সবকটি { $count }টি মেসেজ বেছে নেওয়া হয়েছে।
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] { $count }টি কথোপকথন বেছে নিন
       *[other] সবকটি { $count }টি কথোপকথন বেছে নিন
    }
   *[message] { $count ->
        [one] { $count }টি মেসেজ বেছে নিন
       *[other] সবকটি { $count }টি মেসেজ বেছে নিন
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder }-এর { $count }টি কথোপকথন বেছে নিন
       *[other] { $folder }-এর সবকটি { $count }টি কথোপকথন বেছে নিন
    }
   *[message] { $count ->
        [one] { $folder }-এর { $count }টি মেসেজ বেছে নিন
       *[other] { $folder }-এর সবকটি { $count }টি মেসেজ বেছে নিন
    }
}
list-selected-picked-screen = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] স্ক্রিনের { $count }টি পঠিত কথোপকথন বেছে নেওয়া হয়েছে।
           *[other] স্ক্রিনের সবকটি { $count }টি পঠিত কথোপকথন বেছে নেওয়া হয়েছে।
        }
       *[message] { $count ->
            [one] স্ক্রিনের { $count }টি পঠিত মেসেজ বেছে নেওয়া হয়েছে।
           *[other] স্ক্রিনের সবকটি { $count }টি পঠিত মেসেজ বেছে নেওয়া হয়েছে।
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] স্ক্রিনের { $count }টি অপঠিত কথোপকথন বেছে নেওয়া হয়েছে।
           *[other] স্ক্রিনের সবকটি { $count }টি অপঠিত কথোপকথন বেছে নেওয়া হয়েছে।
        }
       *[message] { $count ->
            [one] স্ক্রিনের { $count }টি অপঠিত মেসেজ বেছে নেওয়া হয়েছে।
           *[other] স্ক্রিনের সবকটি { $count }টি অপঠিত মেসেজ বেছে নেওয়া হয়েছে।
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] স্ক্রিনের { $count }টি তারকাচিহ্নিত কথোপকথন বেছে নেওয়া হয়েছে।
           *[other] স্ক্রিনের সবকটি { $count }টি তারকাচিহ্নিত কথোপকথন বেছে নেওয়া হয়েছে।
        }
       *[message] { $count ->
            [one] স্ক্রিনের { $count }টি তারকাচিহ্নিত মেসেজ বেছে নেওয়া হয়েছে।
           *[other] স্ক্রিনের সবকটি { $count }টি তারকাচিহ্নিত মেসেজ বেছে নেওয়া হয়েছে।
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] স্ক্রিনের { $count }টি তারকাচিহ্নহীন কথোপকথন বেছে নেওয়া হয়েছে।
           *[other] স্ক্রিনের সবকটি { $count }টি তারকাচিহ্নহীন কথোপকথন বেছে নেওয়া হয়েছে।
        }
       *[message] { $count ->
            [one] স্ক্রিনের { $count }টি তারকাচিহ্নহীন মেসেজ বেছে নেওয়া হয়েছে।
           *[other] স্ক্রিনের সবকটি { $count }টি তারকাচিহ্নহীন মেসেজ বেছে নেওয়া হয়েছে।
        }
    }
}
list-select-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $count }টি পঠিত কথোপকথন বেছে নিন
           *[other] সবকটি { $count }টি পঠিত কথোপকথন বেছে নিন
        }
       *[message] { $count ->
            [one] { $count }টি পঠিত মেসেজ বেছে নিন
           *[other] সবকটি { $count }টি পঠিত মেসেজ বেছে নিন
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $count }টি অপঠিত কথোপকথন বেছে নিন
           *[other] সবকটি { $count }টি অপঠিত কথোপকথন বেছে নিন
        }
       *[message] { $count ->
            [one] { $count }টি অপঠিত মেসেজ বেছে নিন
           *[other] সবকটি { $count }টি অপঠিত মেসেজ বেছে নিন
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $count }টি তারকাচিহ্নিত কথোপকথন বেছে নিন
           *[other] সবকটি { $count }টি তারকাচিহ্নিত কথোপকথন বেছে নিন
        }
       *[message] { $count ->
            [one] { $count }টি তারকাচিহ্নিত মেসেজ বেছে নিন
           *[other] সবকটি { $count }টি তারকাচিহ্নিত মেসেজ বেছে নিন
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $count }টি তারকাচিহ্নহীন কথোপকথন বেছে নিন
           *[other] সবকটি { $count }টি তারকাচিহ্নহীন কথোপকথন বেছে নিন
        }
       *[message] { $count ->
            [one] { $count }টি তারকাচিহ্নহীন মেসেজ বেছে নিন
           *[other] সবকটি { $count }টি তারকাচিহ্নহীন মেসেজ বেছে নিন
        }
    }
}
list-select-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $folder }-এর { $count }টি পঠিত কথোপকথন বেছে নিন
           *[other] { $folder }-এর সবকটি { $count }টি পঠিত কথোপকথন বেছে নিন
        }
       *[message] { $count ->
            [one] { $folder }-এর { $count }টি পঠিত মেসেজ বেছে নিন
           *[other] { $folder }-এর সবকটি { $count }টি পঠিত মেসেজ বেছে নিন
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $folder }-এর { $count }টি অপঠিত কথোপকথন বেছে নিন
           *[other] { $folder }-এর সবকটি { $count }টি অপঠিত কথোপকথন বেছে নিন
        }
       *[message] { $count ->
            [one] { $folder }-এর { $count }টি অপঠিত মেসেজ বেছে নিন
           *[other] { $folder }-এর সবকটি { $count }টি অপঠিত মেসেজ বেছে নিন
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $folder }-এর { $count }টি তারকাচিহ্নিত কথোপকথন বেছে নিন
           *[other] { $folder }-এর সবকটি { $count }টি তারকাচিহ্নিত কথোপকথন বেছে নিন
        }
       *[message] { $count ->
            [one] { $folder }-এর { $count }টি তারকাচিহ্নিত মেসেজ বেছে নিন
           *[other] { $folder }-এর সবকটি { $count }টি তারকাচিহ্নিত মেসেজ বেছে নিন
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $folder }-এর { $count }টি তারকাচিহ্নহীন কথোপকথন বেছে নিন
           *[other] { $folder }-এর সবকটি { $count }টি তারকাচিহ্নহীন কথোপকথন বেছে নিন
        }
       *[message] { $count ->
            [one] { $folder }-এর { $count }টি তারকাচিহ্নহীন মেসেজ বেছে নিন
           *[other] { $folder }-এর সবকটি { $count }টি তারকাচিহ্নহীন মেসেজ বেছে নিন
        }
    }
}
list-selected-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $count }টি পঠিত কথোপকথন বেছে নেওয়া হয়েছে।
           *[other] সবকটি { $count }টি পঠিত কথোপকথন বেছে নেওয়া হয়েছে।
        }
       *[message] { $count ->
            [one] { $count }টি পঠিত মেসেজ বেছে নেওয়া হয়েছে।
           *[other] সবকটি { $count }টি পঠিত মেসেজ বেছে নেওয়া হয়েছে।
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $count }টি অপঠিত কথোপকথন বেছে নেওয়া হয়েছে।
           *[other] সবকটি { $count }টি অপঠিত কথোপকথন বেছে নেওয়া হয়েছে।
        }
       *[message] { $count ->
            [one] { $count }টি অপঠিত মেসেজ বেছে নেওয়া হয়েছে।
           *[other] সবকটি { $count }টি অপঠিত মেসেজ বেছে নেওয়া হয়েছে।
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $count }টি তারকাচিহ্নিত কথোপকথন বেছে নেওয়া হয়েছে।
           *[other] সবকটি { $count }টি তারকাচিহ্নিত কথোপকথন বেছে নেওয়া হয়েছে।
        }
       *[message] { $count ->
            [one] { $count }টি তারকাচিহ্নিত মেসেজ বেছে নেওয়া হয়েছে।
           *[other] সবকটি { $count }টি তারকাচিহ্নিত মেসেজ বেছে নেওয়া হয়েছে।
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $count }টি তারকাচিহ্নহীন কথোপকথন বেছে নেওয়া হয়েছে।
           *[other] সবকটি { $count }টি তারকাচিহ্নহীন কথোপকথন বেছে নেওয়া হয়েছে।
        }
       *[message] { $count ->
            [one] { $count }টি তারকাচিহ্নহীন মেসেজ বেছে নেওয়া হয়েছে।
           *[other] সবকটি { $count }টি তারকাচিহ্নহীন মেসেজ বেছে নেওয়া হয়েছে।
        }
    }
}
list-selected-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $folder }-এর { $count }টি পঠিত কথোপকথন বেছে নেওয়া হয়েছে।
           *[other] { $folder }-এর সবকটি { $count }টি পঠিত কথোপকথন বেছে নেওয়া হয়েছে।
        }
       *[message] { $count ->
            [one] { $folder }-এর { $count }টি পঠিত মেসেজ বেছে নেওয়া হয়েছে।
           *[other] { $folder }-এর সবকটি { $count }টি পঠিত মেসেজ বেছে নেওয়া হয়েছে।
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $folder }-এর { $count }টি অপঠিত কথোপকথন বেছে নেওয়া হয়েছে।
           *[other] { $folder }-এর সবকটি { $count }টি অপঠিত কথোপকথন বেছে নেওয়া হয়েছে।
        }
       *[message] { $count ->
            [one] { $folder }-এর { $count }টি অপঠিত মেসেজ বেছে নেওয়া হয়েছে।
           *[other] { $folder }-এর সবকটি { $count }টি অপঠিত মেসেজ বেছে নেওয়া হয়েছে।
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $folder }-এর { $count }টি তারকাচিহ্নিত কথোপকথন বেছে নেওয়া হয়েছে।
           *[other] { $folder }-এর সবকটি { $count }টি তারকাচিহ্নিত কথোপকথন বেছে নেওয়া হয়েছে।
        }
       *[message] { $count ->
            [one] { $folder }-এর { $count }টি তারকাচিহ্নিত মেসেজ বেছে নেওয়া হয়েছে।
           *[other] { $folder }-এর সবকটি { $count }টি তারকাচিহ্নিত মেসেজ বেছে নেওয়া হয়েছে।
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $folder }-এর { $count }টি তারকাচিহ্নহীন কথোপকথন বেছে নেওয়া হয়েছে।
           *[other] { $folder }-এর সবকটি { $count }টি তারকাচিহ্নহীন কথোপকথন বেছে নেওয়া হয়েছে।
        }
       *[message] { $count ->
            [one] { $folder }-এর { $count }টি তারকাচিহ্নহীন মেসেজ বেছে নেওয়া হয়েছে।
           *[other] { $folder }-এর সবকটি { $count }টি তারকাচিহ্নহীন মেসেজ বেছে নেওয়া হয়েছে।
        }
    }
}
list-picked-none = { $pick ->
    [read] { $kind ->
        [conversation] এখানে কোনো পঠিত কথোপকথন নেই।
       *[message] এখানে কোনো পঠিত মেসেজ নেই।
    }
   *[unread] { $kind ->
        [conversation] এখানে কোনো অপঠিত কথোপকথন নেই।
       *[message] এখানে কোনো অপঠিত মেসেজ নেই।
    }
    [starred] { $kind ->
        [conversation] এখানে কোনো তারকাচিহ্নিত কথোপকথন নেই।
       *[message] এখানে কোনো তারকাচিহ্নিত মেসেজ নেই।
    }
    [unstarred] { $kind ->
        [conversation] এখানে কোনো তারকাচিহ্নহীন কথোপকথন নেই।
       *[message] এখানে কোনো তারকাচিহ্নহীন মেসেজ নেই।
    }
}
list-clear-selection = বাছাই মুছুন

## Mail list: empty states

list-empty-search = আপনার সার্চের সাথে কোনো মেসেজ মেলেনি।
list-empty-tab = { $tab }-এ কোনো মেল নেই।
list-empty-tab-unknown = এই ট্যাবে কোনো মেল নেই।
list-empty-folder = { $folder }-এ কোনো মেসেজ নেই।
list-empty-folder-unknown = এই ফোল্ডারে কোনো মেসেজ নেই।
list-empty-waiting = কিছুই উত্তরের অপেক্ষায় নেই।
list-empty-reminders = কোনো রিমাইন্ডার নেই। যোগ করতে কোনো মেলে H চাপুন।
list-first-sync = আপনার মেল আনা হচ্ছে…
list-first-sync-detail = মেল আসার সাথে সাথে এখানে দেখাবে।
list-store-unreadable = মেল স্টোর খোলা যায়নি

## Mail list: lines

row-no-subject = (কোনো বিষয় নেই)
row-unknown-sender = (অজানা প্রেরক)
row-to = প্রাপক:
row-no-recipients = (কোনো প্রাপক নেই)
row-names-separator = {", "}
row-me = আমি

## Mail list: lines

row-removed = এই মেসেজটি সরিয়ে দেওয়া হয়েছে।
row-starred = তারকাচিহ্নিত
row-not-starred = তারকাচিহ্নিত নয়
row-important = গুরুত্বপূর্ণ। গুরুত্বপূর্ণ নয় হিসেবে চিহ্নিত করতে ক্লিক করুন।
row-mark-important = গুরুত্বপূর্ণ হিসেবে চিহ্নিত করুন
row-pinned = উপরে পিন করা
row-task = টাস্ক
row-task-open = টাস্ক খুলুন: { $title }
row-tracking-none = ট্র্যাক করা হচ্ছে। এখনও খোলা হয়নি
row-tracking-opened = { $recipients } জনের মধ্যে { $opened } জন খুলেছেন
row-tracking-clicked = { $recipients } জনের মধ্যে { $opened } জন খুলেছেন, { $clicked } জন লিঙ্ক খুলেছেন
row-pin = উপরে পিন করুন
row-unpin = আনপিন করুন
row-snoozed-until = { $when } পর্যন্ত স্নুজ করা হয়েছে
row-snoozed-day-time = { $day } { $time }
snoozed-group-today = আজ
snoozed-group-tomorrow = আগামীকাল
snoozed-group-this-week = এই সপ্তাহে
snoozed-group-later = পরে
row-follow-up-step = ফলো-আপ { $steps }-এর মধ্যে { $step } · { $date }
row-follow-up-waiting = ফলো-আপ অপেক্ষায়
row-reminder = রিমাইন্ডার { $date }

## Mail list: More menu and right-click menu

menu-reply = উত্তর দিন
menu-reply-all = সবাইকে উত্তর দিন
menu-forward = ফরোয়ার্ড করুন
menu-archive = আর্কাইভ করুন
menu-delete = মুছুন
menu-delete-forever = চিরতরে মুছুন
menu-move-to-inbox = ইনবক্সে সরান
menu-spam = স্প্যাম হিসেবে রিপোর্ট করুন
menu-not-spam = স্প্যাম নয়
menu-mark-read = পঠিত হিসেবে চিহ্নিত করুন
menu-mark-unread = অপঠিত হিসেবে চিহ্নিত করুন
menu-mark-all-read = সবগুলি পঠিত হিসেবে চিহ্নিত করুন
menu-star = তারকাচিহ্ন দিন
menu-unstar = তারকাচিহ্ন সরান
menu-important = গুরুত্বপূর্ণ হিসেবে চিহ্নিত করুন
menu-not-important = গুরুত্বপূর্ণ নয় হিসেবে চিহ্নিত করুন
menu-pin = উপরে পিন করুন
menu-unpin = আনপিন করুন
menu-snooze = স্নুজ করুন
menu-remind = মনে করিয়ে দিন
menu-unsnooze = স্নুজ বাতিল করুন
menu-add-to-tasks = টাস্কে যোগ করুন
menu-schedule-meeting = মিটিং শিডিউল করুন
menu-start-call = ভিডিও কল শুরু করুন
menu-add-note = নোট যোগ করুন
menu-print-all = সব প্রিন্ট করুন
menu-new-window = নতুন উইন্ডোতে খুলুন
menu-move-to = এখানে সরান
# Opens a submenu: Add to Tasks, Add a note, Schedule a meeting and Start a
# video call.
menu-follow-up = ফলো আপ
# Opens a submenu of the rarer actions: Report spam, Mark as important and
# Pin to top.
menu-more = আরও
menu-move-to-heading = এখানে সরান:
menu-move-to-search = এখানে সরান…
menu-label-as = লেবেল দিন
menu-label-as-search = লেবেল দিন…
menu-no-folder = “{ $name }” নামে কোনো ফোল্ডার নেই
menu-no-label = “{ $name }” নামে কোনো লেবেল নেই
menu-create-folder = “{ $name }” তৈরি করুন
menu-always-move = { $name }-এর মেল সবসময় এখানে সরান
toast-always-move-failed = মেল সরানো হয়েছে, কিন্তু নিয়মটি তৈরি হয়নি: { $error }
drag-mail = { $kind ->
    [conversation] { $count ->
        [one] { $count }টি কথোপকথন
       *[other] { $count }টি কথোপকথন
    }
   *[message] { $count ->
        [one] { $count }টি মেসেজ
       *[other] { $count }টি মেসেজ
    }
}
menu-find-from = { $name }-এর পাঠানো ইমেল খুঁজুন
menu-make-rule = নিয়ম তৈরি করুন…

## Snackbar after an action on mail in the list

toast-key-imported = কী ইমপোর্ট করা হয়েছে
toast-key-updated = এই কী আগেই আপনার কাছে ছিল; এখন এটি হালনাগাদ
toast-key-removed = কী সরানো হয়েছে
toast-key-not-removed = কী সরানো যায়নি
toast-fingerprint-copied = ফিঙ্গারপ্রিন্ট কপি করা হয়েছে

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথন আর্কাইভ করা হয়েছে।
       *[other] { $count }টি কথোপকথন আর্কাইভ করা হয়েছে।
    }
   *[message] { $count ->
        [one] মেসেজ আর্কাইভ করা হয়েছে।
       *[other] { $count }টি মেসেজ আর্কাইভ করা হয়েছে।
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথন ট্র্যাশে সরানো হয়েছে।
       *[other] { $count }টি কথোপকথন ট্র্যাশে সরানো হয়েছে।
    }
   *[message] { $count ->
        [one] মেসেজ ট্র্যাশে সরানো হয়েছে।
       *[other] { $count }টি মেসেজ ট্র্যাশে সরানো হয়েছে।
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথন সরানো হয়েছে।
       *[other] { $count }টি কথোপকথন সরানো হয়েছে।
    }
   *[message] { $count ->
        [one] মেসেজ সরানো হয়েছে।
       *[other] { $count }টি মেসেজ সরানো হয়েছে।
    }
}
toast-label-added = “{ $label }” লেবেল যোগ করা হয়েছে।
toast-label-removed = “{ $label }” লেবেল সরানো হয়েছে।
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথনে তারকাচিহ্ন দেওয়া হয়েছে।
       *[other] { $count }টি কথোপকথনে তারকাচিহ্ন দেওয়া হয়েছে।
    }
   *[message] { $count ->
        [one] মেসেজে তারকাচিহ্ন দেওয়া হয়েছে।
       *[other] { $count }টি মেসেজে তারকাচিহ্ন দেওয়া হয়েছে।
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথন থেকে তারকাচিহ্ন সরানো হয়েছে।
       *[other] { $count }টি কথোপকথন থেকে তারকাচিহ্ন সরানো হয়েছে।
    }
   *[message] { $count ->
        [one] মেসেজ থেকে তারকাচিহ্ন সরানো হয়েছে।
       *[other] { $count }টি মেসেজ থেকে তারকাচিহ্ন সরানো হয়েছে।
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথন গুরুত্বপূর্ণ হিসেবে চিহ্নিত করা হয়েছে।
       *[other] { $count }টি কথোপকথন গুরুত্বপূর্ণ হিসেবে চিহ্নিত করা হয়েছে।
    }
   *[message] { $count ->
        [one] মেসেজ গুরুত্বপূর্ণ হিসেবে চিহ্নিত করা হয়েছে।
       *[other] { $count }টি মেসেজ গুরুত্বপূর্ণ হিসেবে চিহ্নিত করা হয়েছে।
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথন গুরুত্বপূর্ণ নয় হিসেবে চিহ্নিত করা হয়েছে।
       *[other] { $count }টি কথোপকথন গুরুত্বপূর্ণ নয় হিসেবে চিহ্নিত করা হয়েছে।
    }
   *[message] { $count ->
        [one] মেসেজ গুরুত্বপূর্ণ নয় হিসেবে চিহ্নিত করা হয়েছে।
       *[other] { $count }টি মেসেজ গুরুত্বপূর্ণ নয় হিসেবে চিহ্নিত করা হয়েছে।
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথন উপরে পিন করা হয়েছে।
       *[other] { $count }টি কথোপকথন উপরে পিন করা হয়েছে।
    }
   *[message] { $count ->
        [one] মেসেজ উপরে পিন করা হয়েছে।
       *[other] { $count }টি মেসেজ উপরে পিন করা হয়েছে।
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথন আনপিন করা হয়েছে।
       *[other] { $count }টি কথোপকথন আনপিন করা হয়েছে।
    }
   *[message] { $count ->
        [one] মেসেজ আনপিন করা হয়েছে।
       *[other] { $count }টি মেসেজ আনপিন করা হয়েছে।
    }
}
toast-snoozed = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথন { $when } পর্যন্ত স্নুজ করা হয়েছে।
       *[other] { $count }টি কথোপকথন { $when } পর্যন্ত স্নুজ করা হয়েছে।
    }
   *[message] { $count ->
        [one] মেসেজ { $when } পর্যন্ত স্নুজ করা হয়েছে।
       *[other] { $count }টি মেসেজ { $when } পর্যন্ত স্নুজ করা হয়েছে।
    }
}
toast-unsnoozed = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথন ইনবক্সে ফিরে এসেছে।
       *[other] { $count }টি কথোপকথন ইনবক্সে ফিরে এসেছে।
    }
   *[message] { $count ->
        [one] মেসেজ ইনবক্সে ফিরে এসেছে।
       *[other] { $count }টি মেসেজ ইনবক্সে ফিরে এসেছে।
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথন স্প্যাম হিসেবে রিপোর্ট করা হয়েছে।
       *[other] { $count }টি কথোপকথন স্প্যাম হিসেবে রিপোর্ট করা হয়েছে।
    }
   *[message] { $count ->
        [one] মেসেজ স্প্যাম হিসেবে রিপোর্ট করা হয়েছে।
       *[other] { $count }টি মেসেজ স্প্যাম হিসেবে রিপোর্ট করা হয়েছে।
    }
}
toast-not-spam = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথন স্প্যাম নয় হিসেবে চিহ্নিত করে ইনবক্সে সরানো হয়েছে।
       *[other] { $count }টি কথোপকথন স্প্যাম নয় হিসেবে চিহ্নিত করে ইনবক্সে সরানো হয়েছে।
    }
   *[message] { $count ->
        [one] মেসেজ স্প্যাম নয় হিসেবে চিহ্নিত করে ইনবক্সে সরানো হয়েছে।
       *[other] { $count }টি মেসেজ স্প্যাম নয় হিসেবে চিহ্নিত করে ইনবক্সে সরানো হয়েছে।
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথন চিরতরে মুছে ফেলা হয়েছে।
       *[other] { $count }টি কথোপকথন চিরতরে মুছে ফেলা হয়েছে।
    }
   *[message] { $count ->
        [one] মেসেজ চিরতরে মুছে ফেলা হয়েছে।
       *[other] { $count }টি মেসেজ চিরতরে মুছে ফেলা হয়েছে।
    }
}
toast-marked-read = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথন পঠিত হিসেবে চিহ্নিত করা হয়েছে।
       *[other] { $count }টি কথোপকথন পঠিত হিসেবে চিহ্নিত করা হয়েছে।
    }
   *[message] { $count ->
        [one] মেসেজ পঠিত হিসেবে চিহ্নিত করা হয়েছে।
       *[other] { $count }টি মেসেজ পঠিত হিসেবে চিহ্নিত করা হয়েছে।
    }
}
toast-marked-unread = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথন অপঠিত হিসেবে চিহ্নিত করা হয়েছে।
       *[other] { $count }টি কথোপকথন অপঠিত হিসেবে চিহ্নিত করা হয়েছে।
    }
   *[message] { $count ->
        [one] মেসেজ অপঠিত হিসেবে চিহ্নিত করা হয়েছে।
       *[other] { $count }টি মেসেজ অপঠিত হিসেবে চিহ্নিত করা হয়েছে।
    }
}
toast-undone = কাজটি পূর্বাবস্থায় ফেরানো হয়েছে।
toast-nothing-to-undo = পূর্বাবস্থায় ফেরানোর মতো কিছু নেই।
toast-cannot-undo-delete-forever = চিরতরে মুছে ফেলা মেল আর ফিরিয়ে আনা যায় না।
toast-send-undone = পাঠানো পূর্বাবস্থায় ফেরানো হয়েছে।
toast-too-late-to-undo-send = পূর্বাবস্থায় ফেরানোর সময় পেরিয়ে গেছে: মেসেজটি ইতিমধ্যে পাঠানো হয়েছে।
toast-undo = পূর্বাবস্থায় ফেরান
toast-close = বন্ধ করুন
toast-no-spam-folder = এই অ্যাকাউন্টে কোনো স্প্যাম ফোল্ডার নেই।
