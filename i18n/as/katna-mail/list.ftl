# Katna Mail, Assamese (অসমীয়া).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = মুখ্য
tab-promotions = প্ৰচাৰ
tab-social = সামাজিক
tab-updates = আপডেট
tab-forums = ফ'ৰাম
tab-focused = কেন্দ্ৰীভূত
tab-other = অন্যান্য
tab-inbox = ইনবক্স
tab-newsletters = বাতৰি-পত্ৰ
tab-notifications = জাননী
tab-provider-other = Katnaই সজোৱা

## Mail list: toolbar

list-select = বাছনি কৰক
list-refresh = ৰিফ্ৰেছ কৰক
list-back-to-top = ওপৰলৈ উভতি যাওক
list-checking = নতুন মেইল পৰীক্ষা কৰি আছে…
list-more = অধিক
list-mark-read = পঢ়া বুলি চিহ্নিত কৰক
list-mark-unread = নপঢ়া বুলি চিহ্নিত কৰক
list-move-to = ইয়ালৈ স্থানান্তৰ কৰক
list-archive = আৰ্কাইভ কৰক
list-spam = স্পাম বুলি ৰিপৰ্ট কৰক
list-delete = মচক
list-snooze = স্নুজ কৰক
list-unsnooze = স্নুজ বাতিল কৰক
list-newer = নতুন
list-older = পুৰণি
list-range = { $total }ৰ { $first }–{ $last }
list-range-about = প্ৰায় { $total }ৰ { $first }–{ $last }
list-results = “{ $query }”ৰ ফলাফল
list-results-corrected = “{ $query }”ৰ ফলাফল দেখুওৱা হৈছে
list-search-instead = ইয়াৰ সলনি “{ $query }” সন্ধান কৰক
list-search-no-index = সন্ধান সাজু নহয়: সূচী এতিয়াও নিৰ্মাণ কৰা হোৱা নাই।
list-search-not-ready = সন্ধান সাজু নহয়: { $error }
list-files-more = +{ $count }
list-replied = আপুনি উত্তৰ দিছে

## Mail list: Select menu (which lines to tick)

list-pick-all = সকলো
list-pick-none = এটাও নহয়
list-pick-read = পঢ়া
list-pick-unread = নপঢ়া
list-pick-starred = তৰাচিহ্নিত
list-pick-unstarred = তৰাচিহ্নবিহীন

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] সকলো { $count }টা কথোপকথন বাছনি কৰা হৈছে।
       *[other] সকলো { $count }টা কথোপকথন বাছনি কৰা হৈছে।
    }
   *[message] { $count ->
        [one] সকলো { $count }টা বাৰ্তা বাছনি কৰা হৈছে।
       *[other] সকলো { $count }টা বাৰ্তা বাছনি কৰা হৈছে।
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder }ত থকা সকলো { $count }টা কথোপকথন বাছনি কৰা হৈছে।
       *[other] { $folder }ত থকা সকলো { $count }টা কথোপকথন বাছনি কৰা হৈছে।
    }
   *[message] { $count ->
        [one] { $folder }ত থকা সকলো { $count }টা বাৰ্তা বাছনি কৰা হৈছে।
       *[other] { $folder }ত থকা সকলো { $count }টা বাৰ্তা বাছনি কৰা হৈছে।
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] স্ক্ৰীনত থকা সকলো { $count }টা কথোপকথন বাছনি কৰা হৈছে।
       *[other] স্ক্ৰীনত থকা সকলো { $count }টা কথোপকথন বাছনি কৰা হৈছে।
    }
   *[message] { $count ->
        [one] স্ক্ৰীনত থকা সকলো { $count }টা বাৰ্তা বাছনি কৰা হৈছে।
       *[other] স্ক্ৰীনত থকা সকলো { $count }টা বাৰ্তা বাছনি কৰা হৈছে।
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] সকলো { $count }টা কথোপকথন বাছনি কৰক
       *[other] সকলো { $count }টা কথোপকথন বাছনি কৰক
    }
   *[message] { $count ->
        [one] সকলো { $count }টা বাৰ্তা বাছনি কৰক
       *[other] সকলো { $count }টা বাৰ্তা বাছনি কৰক
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder }ত থকা সকলো { $count }টা কথোপকথন বাছনি কৰক
       *[other] { $folder }ত থকা সকলো { $count }টা কথোপকথন বাছনি কৰক
    }
   *[message] { $count ->
        [one] { $folder }ত থকা সকলো { $count }টা বাৰ্তা বাছনি কৰক
       *[other] { $folder }ত থকা সকলো { $count }টা বাৰ্তা বাছনি কৰক
    }
}
list-selected-picked-screen = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] স্ক্ৰীনত থকা { $count }টা পঢ়া কথোপকথন বাছনি কৰা হৈছে।
           *[other] স্ক্ৰীনত থকা সকলো { $count }টা পঢ়া কথোপকথন বাছনি কৰা হৈছে।
        }
       *[message] { $count ->
            [one] স্ক্ৰীনত থকা { $count }টা পঢ়া বাৰ্তা বাছনি কৰা হৈছে।
           *[other] স্ক্ৰীনত থকা সকলো { $count }টা পঢ়া বাৰ্তা বাছনি কৰা হৈছে।
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] স্ক্ৰীনত থকা { $count }টা নপঢ়া কথোপকথন বাছনি কৰা হৈছে।
           *[other] স্ক্ৰীনত থকা সকলো { $count }টা নপঢ়া কথোপকথন বাছনি কৰা হৈছে।
        }
       *[message] { $count ->
            [one] স্ক্ৰীনত থকা { $count }টা নপঢ়া বাৰ্তা বাছনি কৰা হৈছে।
           *[other] স্ক্ৰীনত থকা সকলো { $count }টা নপঢ়া বাৰ্তা বাছনি কৰা হৈছে।
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] স্ক্ৰীনত থকা { $count }টা তৰাচিহ্নিত কথোপকথন বাছনি কৰা হৈছে।
           *[other] স্ক্ৰীনত থকা সকলো { $count }টা তৰাচিহ্নিত কথোপকথন বাছনি কৰা হৈছে।
        }
       *[message] { $count ->
            [one] স্ক্ৰীনত থকা { $count }টা তৰাচিহ্নিত বাৰ্তা বাছনি কৰা হৈছে।
           *[other] স্ক্ৰীনত থকা সকলো { $count }টা তৰাচিহ্নিত বাৰ্তা বাছনি কৰা হৈছে।
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] স্ক্ৰীনত থকা { $count }টা তৰাচিহ্নবিহীন কথোপকথন বাছনি কৰা হৈছে।
           *[other] স্ক্ৰীনত থকা সকলো { $count }টা তৰাচিহ্নবিহীন কথোপকথন বাছনি কৰা হৈছে।
        }
       *[message] { $count ->
            [one] স্ক্ৰীনত থকা { $count }টা তৰাচিহ্নবিহীন বাৰ্তা বাছনি কৰা হৈছে।
           *[other] স্ক্ৰীনত থকা সকলো { $count }টা তৰাচিহ্নবিহীন বাৰ্তা বাছনি কৰা হৈছে।
        }
    }
}
list-select-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $count }টা পঢ়া কথোপকথন বাছনি কৰক
           *[other] সকলো { $count }টা পঢ়া কথোপকথন বাছনি কৰক
        }
       *[message] { $count ->
            [one] { $count }টা পঢ়া বাৰ্তা বাছনি কৰক
           *[other] সকলো { $count }টা পঢ়া বাৰ্তা বাছনি কৰক
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $count }টা নপঢ়া কথোপকথন বাছনি কৰক
           *[other] সকলো { $count }টা নপঢ়া কথোপকথন বাছনি কৰক
        }
       *[message] { $count ->
            [one] { $count }টা নপঢ়া বাৰ্তা বাছনি কৰক
           *[other] সকলো { $count }টা নপঢ়া বাৰ্তা বাছনি কৰক
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $count }টা তৰাচিহ্নিত কথোপকথন বাছনি কৰক
           *[other] সকলো { $count }টা তৰাচিহ্নিত কথোপকথন বাছনি কৰক
        }
       *[message] { $count ->
            [one] { $count }টা তৰাচিহ্নিত বাৰ্তা বাছনি কৰক
           *[other] সকলো { $count }টা তৰাচিহ্নিত বাৰ্তা বাছনি কৰক
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $count }টা তৰাচিহ্নবিহীন কথোপকথন বাছনি কৰক
           *[other] সকলো { $count }টা তৰাচিহ্নবিহীন কথোপকথন বাছনি কৰক
        }
       *[message] { $count ->
            [one] { $count }টা তৰাচিহ্নবিহীন বাৰ্তা বাছনি কৰক
           *[other] সকলো { $count }টা তৰাচিহ্নবিহীন বাৰ্তা বাছনি কৰক
        }
    }
}
list-select-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $folder }ত থকা { $count }টা পঢ়া কথোপকথন বাছনি কৰক
           *[other] { $folder }ত থকা সকলো { $count }টা পঢ়া কথোপকথন বাছনি কৰক
        }
       *[message] { $count ->
            [one] { $folder }ত থকা { $count }টা পঢ়া বাৰ্তা বাছনি কৰক
           *[other] { $folder }ত থকা সকলো { $count }টা পঢ়া বাৰ্তা বাছনি কৰক
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $folder }ত থকা { $count }টা নপঢ়া কথোপকথন বাছনি কৰক
           *[other] { $folder }ত থকা সকলো { $count }টা নপঢ়া কথোপকথন বাছনি কৰক
        }
       *[message] { $count ->
            [one] { $folder }ত থকা { $count }টা নপঢ়া বাৰ্তা বাছনি কৰক
           *[other] { $folder }ত থকা সকলো { $count }টা নপঢ়া বাৰ্তা বাছনি কৰক
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $folder }ত থকা { $count }টা তৰাচিহ্নিত কথোপকথন বাছনি কৰক
           *[other] { $folder }ত থকা সকলো { $count }টা তৰাচিহ্নিত কথোপকথন বাছনি কৰক
        }
       *[message] { $count ->
            [one] { $folder }ত থকা { $count }টা তৰাচিহ্নিত বাৰ্তা বাছনি কৰক
           *[other] { $folder }ত থকা সকলো { $count }টা তৰাচিহ্নিত বাৰ্তা বাছনি কৰক
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $folder }ত থকা { $count }টা তৰাচিহ্নবিহীন কথোপকথন বাছনি কৰক
           *[other] { $folder }ত থকা সকলো { $count }টা তৰাচিহ্নবিহীন কথোপকথন বাছনি কৰক
        }
       *[message] { $count ->
            [one] { $folder }ত থকা { $count }টা তৰাচিহ্নবিহীন বাৰ্তা বাছনি কৰক
           *[other] { $folder }ত থকা সকলো { $count }টা তৰাচিহ্নবিহীন বাৰ্তা বাছনি কৰক
        }
    }
}
list-selected-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $count }টা পঢ়া কথোপকথন বাছনি কৰা হৈছে।
           *[other] সকলো { $count }টা পঢ়া কথোপকথন বাছনি কৰা হৈছে।
        }
       *[message] { $count ->
            [one] { $count }টা পঢ়া বাৰ্তা বাছনি কৰা হৈছে।
           *[other] সকলো { $count }টা পঢ়া বাৰ্তা বাছনি কৰা হৈছে।
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $count }টা নপঢ়া কথোপকথন বাছনি কৰা হৈছে।
           *[other] সকলো { $count }টা নপঢ়া কথোপকথন বাছনি কৰা হৈছে।
        }
       *[message] { $count ->
            [one] { $count }টা নপঢ়া বাৰ্তা বাছনি কৰা হৈছে।
           *[other] সকলো { $count }টা নপঢ়া বাৰ্তা বাছনি কৰা হৈছে।
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $count }টা তৰাচিহ্নিত কথোপকথন বাছনি কৰা হৈছে।
           *[other] সকলো { $count }টা তৰাচিহ্নিত কথোপকথন বাছনি কৰা হৈছে।
        }
       *[message] { $count ->
            [one] { $count }টা তৰাচিহ্নিত বাৰ্তা বাছনি কৰা হৈছে।
           *[other] সকলো { $count }টা তৰাচিহ্নিত বাৰ্তা বাছনি কৰা হৈছে।
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $count }টা তৰাচিহ্নবিহীন কথোপকথন বাছনি কৰা হৈছে।
           *[other] সকলো { $count }টা তৰাচিহ্নবিহীন কথোপকথন বাছনি কৰা হৈছে।
        }
       *[message] { $count ->
            [one] { $count }টা তৰাচিহ্নবিহীন বাৰ্তা বাছনি কৰা হৈছে।
           *[other] সকলো { $count }টা তৰাচিহ্নবিহীন বাৰ্তা বাছনি কৰা হৈছে।
        }
    }
}
list-selected-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $folder }ত থকা { $count }টা পঢ়া কথোপকথন বাছনি কৰা হৈছে।
           *[other] { $folder }ত থকা সকলো { $count }টা পঢ়া কথোপকথন বাছনি কৰা হৈছে।
        }
       *[message] { $count ->
            [one] { $folder }ত থকা { $count }টা পঢ়া বাৰ্তা বাছনি কৰা হৈছে।
           *[other] { $folder }ত থকা সকলো { $count }টা পঢ়া বাৰ্তা বাছনি কৰা হৈছে।
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $folder }ত থকা { $count }টা নপঢ়া কথোপকথন বাছনি কৰা হৈছে।
           *[other] { $folder }ত থকা সকলো { $count }টা নপঢ়া কথোপকথন বাছনি কৰা হৈছে।
        }
       *[message] { $count ->
            [one] { $folder }ত থকা { $count }টা নপঢ়া বাৰ্তা বাছনি কৰা হৈছে।
           *[other] { $folder }ত থকা সকলো { $count }টা নপঢ়া বাৰ্তা বাছনি কৰা হৈছে।
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $folder }ত থকা { $count }টা তৰাচিহ্নিত কথোপকথন বাছনি কৰা হৈছে।
           *[other] { $folder }ত থকা সকলো { $count }টা তৰাচিহ্নিত কথোপকথন বাছনি কৰা হৈছে।
        }
       *[message] { $count ->
            [one] { $folder }ত থকা { $count }টা তৰাচিহ্নিত বাৰ্তা বাছনি কৰা হৈছে।
           *[other] { $folder }ত থকা সকলো { $count }টা তৰাচিহ্নিত বাৰ্তা বাছনি কৰা হৈছে।
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $folder }ত থকা { $count }টা তৰাচিহ্নবিহীন কথোপকথন বাছনি কৰা হৈছে।
           *[other] { $folder }ত থকা সকলো { $count }টা তৰাচিহ্নবিহীন কথোপকথন বাছনি কৰা হৈছে।
        }
       *[message] { $count ->
            [one] { $folder }ত থকা { $count }টা তৰাচিহ্নবিহীন বাৰ্তা বাছনি কৰা হৈছে।
           *[other] { $folder }ত থকা সকলো { $count }টা তৰাচিহ্নবিহীন বাৰ্তা বাছনি কৰা হৈছে।
        }
    }
}
list-picked-none = { $pick ->
    [read] { $kind ->
        [conversation] ইয়াত কোনো পঢ়া কথোপকথন নাই।
       *[message] ইয়াত কোনো পঢ়া বাৰ্তা নাই।
    }
   *[unread] { $kind ->
        [conversation] ইয়াত কোনো নপঢ়া কথোপকথন নাই।
       *[message] ইয়াত কোনো নপঢ়া বাৰ্তা নাই।
    }
    [starred] { $kind ->
        [conversation] ইয়াত কোনো তৰাচিহ্নিত কথোপকথন নাই।
       *[message] ইয়াত কোনো তৰাচিহ্নিত বাৰ্তা নাই।
    }
    [unstarred] { $kind ->
        [conversation] ইয়াত কোনো তৰাচিহ্নবিহীন কথোপকথন নাই।
       *[message] ইয়াত কোনো তৰাচিহ্নবিহীন বাৰ্তা নাই।
    }
}
list-clear-selection = বাছনি আঁতৰাওক

## Mail list: empty states

list-empty-search = আপোনাৰ সন্ধানৰ সৈতে কোনো বাৰ্তা মিলা নাই।
list-empty-tab = { $tab }ত কোনো মেইল নাই।
list-empty-tab-unknown = এই টেবত কোনো মেইল নাই।
list-empty-folder = { $folder }ত কোনো বাৰ্তা নাই।
list-empty-folder-unknown = এই ফ'ল্ডাৰত কোনো বাৰ্তা নাই।
list-empty-waiting = উত্তৰৰ অপেক্ষাত একো নাই।
list-empty-reminders = কোনো সোঁৱৰণী নাই। এটা যোগ কৰিবলৈ মেইলত H টিপক।
list-first-sync = আপোনাৰ মেইল অনা হৈছে…
list-first-sync-detail = মেইল অহাৰ লগে লগে ইয়াত দেখা যাব।
list-store-unreadable = মেইল ষ্ট'ৰ খুলিব পৰা নগ'ল

## Mail list: lines

row-no-subject = (কোনো বিষয় নাই)
row-unknown-sender = (অজ্ঞাত প্ৰেৰক)
row-to = প্ৰাপক:
row-no-recipients = (কোনো প্ৰাপক নাই)
row-names-separator = {", "}
row-me = মই

## Mail list: lines

row-removed = এই বাৰ্তাটো আঁতৰোৱা হ'ল।
row-starred = তৰাচিহ্নিত
row-not-starred = তৰাচিহ্নিত নহয়
row-important = গুৰুত্বপূৰ্ণ। গুৰুত্বপূৰ্ণ নহয় বুলি চিহ্নিত কৰিবলৈ ক্লিক কৰক।
row-mark-important = গুৰুত্বপূৰ্ণ বুলি চিহ্নিত কৰক
row-pinned = ওপৰত পিন কৰা হৈছে
row-task = কাৰ্য
row-task-open = কাৰ্য খোলক: { $title }
row-tracking-none = ট্ৰেক কৰা হৈছে। এতিয়াও খোলা হোৱা নাই
row-tracking-opened = { $recipients }ৰ ভিতৰত { $opened }-এ খুলিছে
row-tracking-clicked = { $recipients }ৰ ভিতৰত { $opened }-এ খুলিছে, { $clicked }-এ লিংক অনুসৰণ কৰিছে
row-pin = ওপৰত পিন কৰক
row-unpin = আনপিন কৰক
row-snoozed-until = { $when }লৈ স্নুজ কৰা হৈছে
row-snoozed-day-time = { $day } { $time }
snoozed-group-today = আজি
snoozed-group-tomorrow = কাইলৈ
snoozed-group-this-week = এই সপ্তাহ
snoozed-group-later = পিছত
row-follow-up-step = { $steps }ৰ { $step } নং ফল'-আপ · { $date }
row-follow-up-waiting = ফল'-আপ অপেক্ষাত
row-reminder = সোঁৱৰণী { $date }

## Mail list: More menu and right-click menu

menu-reply = উত্তৰ দিয়ক
menu-reply-all = সকলোকে উত্তৰ দিয়ক
menu-forward = ফৰৱাৰ্ড কৰক
menu-archive = আৰ্কাইভ কৰক
menu-delete = মচক
menu-delete-forever = চিৰদিনৰ বাবে মচক
menu-move-to-inbox = ইনবক্সলৈ স্থানান্তৰ কৰক
menu-spam = স্পাম বুলি ৰিপৰ্ট কৰক
menu-not-spam = স্পাম নহয়
menu-mark-read = পঢ়া বুলি চিহ্নিত কৰক
menu-mark-unread = নপঢ়া বুলি চিহ্নিত কৰক
menu-mark-all-read = সকলোবোৰ পঢ়া বুলি চিহ্নিত কৰক
menu-star = তৰাচিহ্ন যোগ কৰক
menu-unstar = তৰাচিহ্ন আঁতৰাওক
menu-important = গুৰুত্বপূৰ্ণ বুলি চিহ্নিত কৰক
menu-not-important = গুৰুত্বপূৰ্ণ নহয় বুলি চিহ্নিত কৰক
menu-pin = ওপৰত পিন কৰক
menu-unpin = আনপিন কৰক
menu-snooze = স্নুজ কৰক
menu-remind = মোক সোঁৱৰাওক
menu-unsnooze = স্নুজ বাতিল কৰক
menu-add-to-tasks = কাৰ্যত যোগ কৰক
menu-schedule-meeting = বৈঠক নিৰ্ধাৰণ কৰক
menu-start-call = ভিডিঅ' কল আৰম্ভ কৰক
menu-add-note = টোকা যোগ কৰক
menu-print-all = সকলো প্ৰিণ্ট কৰক
menu-new-window = নতুন ৱিণ্ড'ত খোলক
menu-move-to = ইয়ালৈ স্থানান্তৰ কৰক
# Opens a submenu: Add to Tasks, Add a note, Schedule a meeting and Start a
# video call.
menu-follow-up = ফ'ল' আপ
# Opens a submenu of the rarer actions: Report spam, Mark as important and
# Pin to top.
menu-more = অধিক
menu-move-to-heading = ইয়ালৈ স্থানান্তৰ কৰক:
menu-move-to-search = ইয়ালৈ স্থানান্তৰ কৰক…
menu-label-as = লেবেল দিয়ক
menu-label-as-search = লেবেল দিয়ক…
menu-no-folder = “{ $name }” নামৰ কোনো ফ'ল্ডাৰ নাই
menu-no-label = “{ $name }” নামৰ কোনো লেবেল নাই
menu-create-folder = “{ $name }” সৃষ্টি কৰক
menu-always-move = { $name }ৰ মেইল সদায় ইয়ালৈ স্থানান্তৰ কৰক
toast-always-move-failed = মেইল স্থানান্তৰ কৰা হ'ল, কিন্তু নিয়মটো বনোৱা নহ'ল: { $error }
drag-mail = { $kind ->
    [conversation] { $count ->
        [one] { $count }টা কথোপকথন
       *[other] { $count }টা কথোপকথন
    }
   *[message] { $count ->
        [one] { $count }টা বাৰ্তা
       *[other] { $count }টা বাৰ্তা
    }
}
menu-find-from = { $name }ৰ পৰা অহা ইমেইল বিচাৰক
menu-make-rule = নিয়ম বনাওক…

## Snackbar after an action on mail in the list

toast-key-imported = কী ইমপৰ্ট কৰা হ'ল
toast-key-updated = এই কীটো আপোনাৰ ওচৰত আগৰে পৰা আছিল; এতিয়া ই শেহতীয়া
toast-key-removed = কী আঁতৰোৱা হ'ল
toast-key-not-removed = কীটো আঁতৰাব পৰা নগ'ল
toast-fingerprint-copied = ফিংগাৰপ্ৰিণ্ট কপি কৰা হ'ল

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথনটো আৰ্কাইভ কৰা হ'ল।
       *[other] { $count }টা কথোপকথন আৰ্কাইভ কৰা হ'ল।
    }
   *[message] { $count ->
        [one] বাৰ্তাটো আৰ্কাইভ কৰা হ'ল।
       *[other] { $count }টা বাৰ্তা আৰ্কাইভ কৰা হ'ল।
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথনটো ট্ৰেছলৈ স্থানান্তৰ কৰা হ'ল।
       *[other] { $count }টা কথোপকথন ট্ৰেছলৈ স্থানান্তৰ কৰা হ'ল।
    }
   *[message] { $count ->
        [one] বাৰ্তাটো ট্ৰেছলৈ স্থানান্তৰ কৰা হ'ল।
       *[other] { $count }টা বাৰ্তা ট্ৰেছলৈ স্থানান্তৰ কৰা হ'ল।
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথনটো স্থানান্তৰ কৰা হ'ল।
       *[other] { $count }টা কথোপকথন স্থানান্তৰ কৰা হ'ল।
    }
   *[message] { $count ->
        [one] বাৰ্তাটো স্থানান্তৰ কৰা হ'ল।
       *[other] { $count }টা বাৰ্তা স্থানান্তৰ কৰা হ'ল।
    }
}
toast-label-added = “{ $label }” লেবেল যোগ কৰা হ'ল।
toast-label-removed = “{ $label }” লেবেল আঁতৰোৱা হ'ল।
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথনটো তৰাচিহ্নিত কৰা হ'ল।
       *[other] { $count }টা কথোপকথন তৰাচিহ্নিত কৰা হ'ল।
    }
   *[message] { $count ->
        [one] বাৰ্তাটো তৰাচিহ্নিত কৰা হ'ল।
       *[other] { $count }টা বাৰ্তা তৰাচিহ্নিত কৰা হ'ল।
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথনটোৰ তৰাচিহ্ন আঁতৰোৱা হ'ল।
       *[other] { $count }টা কথোপকথনৰ তৰাচিহ্ন আঁতৰোৱা হ'ল।
    }
   *[message] { $count ->
        [one] বাৰ্তাটোৰ তৰাচিহ্ন আঁতৰোৱা হ'ল।
       *[other] { $count }টা বাৰ্তাৰ তৰাচিহ্ন আঁতৰোৱা হ'ল।
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথনটো গুৰুত্বপূৰ্ণ বুলি চিহ্নিত কৰা হ'ল।
       *[other] { $count }টা কথোপকথন গুৰুত্বপূৰ্ণ বুলি চিহ্নিত কৰা হ'ল।
    }
   *[message] { $count ->
        [one] বাৰ্তাটো গুৰুত্বপূৰ্ণ বুলি চিহ্নিত কৰা হ'ল।
       *[other] { $count }টা বাৰ্তা গুৰুত্বপূৰ্ণ বুলি চিহ্নিত কৰা হ'ল।
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথনটো গুৰুত্বপূৰ্ণ নহয় বুলি চিহ্নিত কৰা হ'ল।
       *[other] { $count }টা কথোপকথন গুৰুত্বপূৰ্ণ নহয় বুলি চিহ্নিত কৰা হ'ল।
    }
   *[message] { $count ->
        [one] বাৰ্তাটো গুৰুত্বপূৰ্ণ নহয় বুলি চিহ্নিত কৰা হ'ল।
       *[other] { $count }টা বাৰ্তা গুৰুত্বপূৰ্ণ নহয় বুলি চিহ্নিত কৰা হ'ল।
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথনটো ওপৰত পিন কৰা হ'ল।
       *[other] { $count }টা কথোপকথন ওপৰত পিন কৰা হ'ল।
    }
   *[message] { $count ->
        [one] বাৰ্তাটো ওপৰত পিন কৰা হ'ল।
       *[other] { $count }টা বাৰ্তা ওপৰত পিন কৰা হ'ল।
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথনটো আনপিন কৰা হ'ল।
       *[other] { $count }টা কথোপকথন আনপিন কৰা হ'ল।
    }
   *[message] { $count ->
        [one] বাৰ্তাটো আনপিন কৰা হ'ল।
       *[other] { $count }টা বাৰ্তা আনপিন কৰা হ'ল।
    }
}
toast-snoozed = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথনটো { $when }লৈ স্নুজ কৰা হ'ল।
       *[other] { $count }টা কথোপকথন { $when }লৈ স্নুজ কৰা হ'ল।
    }
   *[message] { $count ->
        [one] বাৰ্তাটো { $when }লৈ স্নুজ কৰা হ'ল।
       *[other] { $count }টা বাৰ্তা { $when }লৈ স্নুজ কৰা হ'ল।
    }
}
toast-unsnoozed = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথনটো ইনবক্সলৈ উভতি আহিল।
       *[other] { $count }টা কথোপকথন ইনবক্সলৈ উভতি আহিল।
    }
   *[message] { $count ->
        [one] বাৰ্তাটো ইনবক্সলৈ উভতি আহিল।
       *[other] { $count }টা বাৰ্তা ইনবক্সলৈ উভতি আহিল।
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথনটো স্পাম বুলি ৰিপৰ্ট কৰা হ'ল।
       *[other] { $count }টা কথোপকথন স্পাম বুলি ৰিপৰ্ট কৰা হ'ল।
    }
   *[message] { $count ->
        [one] বাৰ্তাটো স্পাম বুলি ৰিপৰ্ট কৰা হ'ল।
       *[other] { $count }টা বাৰ্তা স্পাম বুলি ৰিপৰ্ট কৰা হ'ল।
    }
}
toast-not-spam = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথনটো স্পাম নহয় বুলি চিহ্নিত কৰি ইনবক্সলৈ নিয়া হ'ল।
       *[other] { $count }টা কথোপকথন স্পাম নহয় বুলি চিহ্নিত কৰি ইনবক্সলৈ নিয়া হ'ল।
    }
   *[message] { $count ->
        [one] বাৰ্তাটো স্পাম নহয় বুলি চিহ্নিত কৰি ইনবক্সলৈ নিয়া হ'ল।
       *[other] { $count }টা বাৰ্তা স্পাম নহয় বুলি চিহ্নিত কৰি ইনবক্সলৈ নিয়া হ'ল।
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথনটো চিৰদিনৰ বাবে মচা হ'ল।
       *[other] { $count }টা কথোপকথন চিৰদিনৰ বাবে মচা হ'ল।
    }
   *[message] { $count ->
        [one] বাৰ্তাটো চিৰদিনৰ বাবে মচা হ'ল।
       *[other] { $count }টা বাৰ্তা চিৰদিনৰ বাবে মচা হ'ল।
    }
}
toast-marked-read = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথনটো পঢ়া বুলি চিহ্নিত কৰা হ'ল।
       *[other] { $count }টা কথোপকথন পঢ়া বুলি চিহ্নিত কৰা হ'ল।
    }
   *[message] { $count ->
        [one] বাৰ্তাটো পঢ়া বুলি চিহ্নিত কৰা হ'ল।
       *[other] { $count }টা বাৰ্তা পঢ়া বুলি চিহ্নিত কৰা হ'ল।
    }
}
toast-marked-unread = { $kind ->
    [conversation] { $count ->
        [one] কথোপকথনটো নপঢ়া বুলি চিহ্নিত কৰা হ'ল।
       *[other] { $count }টা কথোপকথন নপঢ়া বুলি চিহ্নিত কৰা হ'ল।
    }
   *[message] { $count ->
        [one] বাৰ্তাটো নপঢ়া বুলি চিহ্নিত কৰা হ'ল।
       *[other] { $count }টা বাৰ্তা নপঢ়া বুলি চিহ্নিত কৰা হ'ল।
    }
}
toast-undone = কাৰ্যটো পূৰ্বাৱস্থালৈ অনা হ'ল।
toast-nothing-to-undo = আনডু কৰিবলৈ একো নাই।
toast-cannot-undo-delete-forever = চিৰদিনৰ বাবে মচা মেইল ঘূৰাই অনা নাযায়।
toast-send-undone = পঠিওৱাটো আনডু কৰা হ'ল।
toast-too-late-to-undo-send = আনডু কৰিবলৈ বহুত পলম হ'ল: বাৰ্তাটো ইতিমধ্যে পঠিওৱা হৈছে।
toast-undo = আনডু কৰক
toast-close = বন্ধ কৰক
toast-no-spam-folder = এই একাউণ্টত কোনো স্পাম ফ'ল্ডাৰ নাই।
