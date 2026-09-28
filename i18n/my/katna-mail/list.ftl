# Katna Mail, Burmese (မြန်မာ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = အဓိက
tab-promotions = ပရိုမိုးရှင်းများ
tab-social = လူမှုရေး
tab-updates = အပ်ဒိတ်များ
tab-forums = ဖိုရမ်များ
tab-focused = အာရုံစိုက်
tab-other = အခြား
tab-inbox = ဝင်စာ
tab-newsletters = သတင်းလွှာများ
tab-notifications = အကြောင်းကြားချက်များ
tab-new = အသစ် { $count }
tab-provider-other = Katna က စီထားသည်

## Mail list: toolbar

list-select = ရွေးရန်
list-refresh = ပြန်လည်ဆန်းသစ်ရန်
list-more = နောက်ထပ်
list-mark-read = ဖတ်ပြီးအဖြစ် မှတ်ရန်
list-mark-unread = မဖတ်ရသေးအဖြစ် မှတ်ရန်
list-move-to = သို့ ရွှေ့ရန်
list-archive = မှတ်တမ်းသိမ်းရန်
list-spam = စပမ်းအဖြစ် တိုင်ကြားရန်
list-delete = ဖျက်ရန်
list-snooze = ခဏဆိုင်းရန်
list-unsnooze = ခဏဆိုင်းခြင်း ပယ်ရန်
list-newer = ပိုသစ်သော
list-older = ပိုဟောင်းသော
list-range = { $total } ခုအနက် { $first }–{ $last }
list-range-about = ခန့်မှန်း { $total } ခုအနက် { $first }–{ $last }
list-results = “{ $query }” အတွက် ရလဒ်များ
list-results-corrected = “{ $query }” အတွက် ရလဒ်များကို ပြနေသည်
list-search-instead = “{ $query }” ကို အစားထိုး ရှာရန်
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = အားလုံး
list-pick-none = တစ်ခုမျှ မရွေး
list-pick-read = ဖတ်ပြီး
list-pick-unread = မဖတ်ရသေး
list-pick-starred = ကြယ်ပွင့်တပ်ထားသည်
list-pick-unstarred = ကြယ်ပွင့်မတပ်ထား

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] စကားဝိုင်း { $count } ခုလုံးကို ရွေးထားသည်။
   *[message] မက်ဆေ့ဂျ် { $count } စောင်လုံးကို ရွေးထားသည်။
}
list-selected-all-in = { $kind ->
    [conversation] { $folder } ရှိ စကားဝိုင်း { $count } ခုလုံးကို ရွေးထားသည်။
   *[message] { $folder } ရှိ မက်ဆေ့ဂျ် { $count } စောင်လုံးကို ရွေးထားသည်။
}
list-selected-screen = { $kind ->
    [conversation] မျက်နှာပြင်ပေါ်ရှိ စကားဝိုင်း { $count } ခုလုံးကို ရွေးထားသည်။
   *[message] မျက်နှာပြင်ပေါ်ရှိ မက်ဆေ့ဂျ် { $count } စောင်လုံးကို ရွေးထားသည်။
}
list-select-all = { $kind ->
    [conversation] စကားဝိုင်း { $count } ခုလုံးကို ရွေးရန်
   *[message] မက်ဆေ့ဂျ် { $count } စောင်လုံးကို ရွေးရန်
}
list-select-all-in = { $kind ->
    [conversation] { $folder } ရှိ စကားဝိုင်း { $count } ခုလုံးကို ရွေးရန်
   *[message] { $folder } ရှိ မက်ဆေ့ဂျ် { $count } စောင်လုံးကို ရွေးရန်
}
list-selected-picked-screen = { $pick ->
    [read] { $kind ->
        [conversation] မျက်နှာပြင်ပေါ်ရှိ ဖတ်ပြီးသော စကားဝိုင်း { $count } ခုလုံးကို ရွေးထားသည်။
       *[message] မျက်နှာပြင်ပေါ်ရှိ ဖတ်ပြီးသော မက်ဆေ့ဂျ် { $count } စောင်လုံးကို ရွေးထားသည်။
    }
   *[unread] { $kind ->
        [conversation] မျက်နှာပြင်ပေါ်ရှိ မဖတ်ရသေးသော စကားဝိုင်း { $count } ခုလုံးကို ရွေးထားသည်။
       *[message] မျက်နှာပြင်ပေါ်ရှိ မဖတ်ရသေးသော မက်ဆေ့ဂျ် { $count } စောင်လုံးကို ရွေးထားသည်။
    }
    [starred] { $kind ->
        [conversation] မျက်နှာပြင်ပေါ်ရှိ ကြယ်ပွင့်တပ်ထားသော စကားဝိုင်း { $count } ခုလုံးကို ရွေးထားသည်။
       *[message] မျက်နှာပြင်ပေါ်ရှိ ကြယ်ပွင့်တပ်ထားသော မက်ဆေ့ဂျ် { $count } စောင်လုံးကို ရွေးထားသည်။
    }
    [unstarred] { $kind ->
        [conversation] မျက်နှာပြင်ပေါ်ရှိ ကြယ်ပွင့်မတပ်ထားသော စကားဝိုင်း { $count } ခုလုံးကို ရွေးထားသည်။
       *[message] မျက်နှာပြင်ပေါ်ရှိ ကြယ်ပွင့်မတပ်ထားသော မက်ဆေ့ဂျ် { $count } စောင်လုံးကို ရွေးထားသည်။
    }
}
list-select-picked = { $pick ->
    [read] { $kind ->
        [conversation] ဖတ်ပြီးသော စကားဝိုင်း { $count } ခုလုံးကို ရွေးရန်
       *[message] ဖတ်ပြီးသော မက်ဆေ့ဂျ် { $count } စောင်လုံးကို ရွေးရန်
    }
   *[unread] { $kind ->
        [conversation] မဖတ်ရသေးသော စကားဝိုင်း { $count } ခုလုံးကို ရွေးရန်
       *[message] မဖတ်ရသေးသော မက်ဆေ့ဂျ် { $count } စောင်လုံးကို ရွေးရန်
    }
    [starred] { $kind ->
        [conversation] ကြယ်ပွင့်တပ်ထားသော စကားဝိုင်း { $count } ခုလုံးကို ရွေးရန်
       *[message] ကြယ်ပွင့်တပ်ထားသော မက်ဆေ့ဂျ် { $count } စောင်လုံးကို ရွေးရန်
    }
    [unstarred] { $kind ->
        [conversation] ကြယ်ပွင့်မတပ်ထားသော စကားဝိုင်း { $count } ခုလုံးကို ရွေးရန်
       *[message] ကြယ်ပွင့်မတပ်ထားသော မက်ဆေ့ဂျ် { $count } စောင်လုံးကို ရွေးရန်
    }
}
list-select-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $folder } ရှိ ဖတ်ပြီးသော စကားဝိုင်း { $count } ခုလုံးကို ရွေးရန်
       *[message] { $folder } ရှိ ဖတ်ပြီးသော မက်ဆေ့ဂျ် { $count } စောင်လုံးကို ရွေးရန်
    }
   *[unread] { $kind ->
        [conversation] { $folder } ရှိ မဖတ်ရသေးသော စကားဝိုင်း { $count } ခုလုံးကို ရွေးရန်
       *[message] { $folder } ရှိ မဖတ်ရသေးသော မက်ဆေ့ဂျ် { $count } စောင်လုံးကို ရွေးရန်
    }
    [starred] { $kind ->
        [conversation] { $folder } ရှိ ကြယ်ပွင့်တပ်ထားသော စကားဝိုင်း { $count } ခုလုံးကို ရွေးရန်
       *[message] { $folder } ရှိ ကြယ်ပွင့်တပ်ထားသော မက်ဆေ့ဂျ် { $count } စောင်လုံးကို ရွေးရန်
    }
    [unstarred] { $kind ->
        [conversation] { $folder } ရှိ ကြယ်ပွင့်မတပ်ထားသော စကားဝိုင်း { $count } ခုလုံးကို ရွေးရန်
       *[message] { $folder } ရှိ ကြယ်ပွင့်မတပ်ထားသော မက်ဆေ့ဂျ် { $count } စောင်လုံးကို ရွေးရန်
    }
}
list-selected-picked = { $pick ->
    [read] { $kind ->
        [conversation] ဖတ်ပြီးသော စကားဝိုင်း { $count } ခုလုံးကို ရွေးထားသည်။
       *[message] ဖတ်ပြီးသော မက်ဆေ့ဂျ် { $count } စောင်လုံးကို ရွေးထားသည်။
    }
   *[unread] { $kind ->
        [conversation] မဖတ်ရသေးသော စကားဝိုင်း { $count } ခုလုံးကို ရွေးထားသည်။
       *[message] မဖတ်ရသေးသော မက်ဆေ့ဂျ် { $count } စောင်လုံးကို ရွေးထားသည်။
    }
    [starred] { $kind ->
        [conversation] ကြယ်ပွင့်တပ်ထားသော စကားဝိုင်း { $count } ခုလုံးကို ရွေးထားသည်။
       *[message] ကြယ်ပွင့်တပ်ထားသော မက်ဆေ့ဂျ် { $count } စောင်လုံးကို ရွေးထားသည်။
    }
    [unstarred] { $kind ->
        [conversation] ကြယ်ပွင့်မတပ်ထားသော စကားဝိုင်း { $count } ခုလုံးကို ရွေးထားသည်။
       *[message] ကြယ်ပွင့်မတပ်ထားသော မက်ဆေ့ဂျ် { $count } စောင်လုံးကို ရွေးထားသည်။
    }
}
list-selected-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $folder } ရှိ ဖတ်ပြီးသော စကားဝိုင်း { $count } ခုလုံးကို ရွေးထားသည်။
       *[message] { $folder } ရှိ ဖတ်ပြီးသော မက်ဆေ့ဂျ် { $count } စောင်လုံးကို ရွေးထားသည်။
    }
   *[unread] { $kind ->
        [conversation] { $folder } ရှိ မဖတ်ရသေးသော စကားဝိုင်း { $count } ခုလုံးကို ရွေးထားသည်။
       *[message] { $folder } ရှိ မဖတ်ရသေးသော မက်ဆေ့ဂျ် { $count } စောင်လုံးကို ရွေးထားသည်။
    }
    [starred] { $kind ->
        [conversation] { $folder } ရှိ ကြယ်ပွင့်တပ်ထားသော စကားဝိုင်း { $count } ခုလုံးကို ရွေးထားသည်။
       *[message] { $folder } ရှိ ကြယ်ပွင့်တပ်ထားသော မက်ဆေ့ဂျ် { $count } စောင်လုံးကို ရွေးထားသည်။
    }
    [unstarred] { $kind ->
        [conversation] { $folder } ရှိ ကြယ်ပွင့်မတပ်ထားသော စကားဝိုင်း { $count } ခုလုံးကို ရွေးထားသည်။
       *[message] { $folder } ရှိ ကြယ်ပွင့်မတပ်ထားသော မက်ဆေ့ဂျ် { $count } စောင်လုံးကို ရွေးထားသည်။
    }
}
list-picked-none = { $pick ->
    [read] { $kind ->
        [conversation] ဤနေရာတွင် ဖတ်ပြီးသော စကားဝိုင်း မရှိပါ။
       *[message] ဤနေရာတွင် ဖတ်ပြီးသော မက်ဆေ့ဂျ် မရှိပါ။
    }
   *[unread] { $kind ->
        [conversation] ဤနေရာတွင် မဖတ်ရသေးသော စကားဝိုင်း မရှိပါ။
       *[message] ဤနေရာတွင် မဖတ်ရသေးသော မက်ဆေ့ဂျ် မရှိပါ။
    }
    [starred] { $kind ->
        [conversation] ဤနေရာတွင် ကြယ်ပွင့်တပ်ထားသော စကားဝိုင်း မရှိပါ။
       *[message] ဤနေရာတွင် ကြယ်ပွင့်တပ်ထားသော မက်ဆေ့ဂျ် မရှိပါ။
    }
    [unstarred] { $kind ->
        [conversation] ဤနေရာတွင် ကြယ်ပွင့်မတပ်ထားသော စကားဝိုင်း မရှိပါ။
       *[message] ဤနေရာတွင် ကြယ်ပွင့်မတပ်ထားသော မက်ဆေ့ဂျ် မရှိပါ။
    }
}
list-clear-selection = ရွေးချယ်မှု ရှင်းရန်

## Mail list: empty states

list-empty-search = သင့်ရှာဖွေမှုနှင့် ကိုက်ညီသော မက်ဆေ့ဂျ် မရှိပါ။
list-empty-tab = { $tab } တွင် မေးလ် မရှိပါ။
list-empty-tab-unknown = ဤတဘ်တွင် မေးလ် မရှိပါ။
list-empty-folder = { $folder } တွင် မက်ဆေ့ဂျ် မရှိပါ။
list-empty-folder-unknown = ဤဖိုင်တွဲတွင် မက်ဆေ့ဂျ် မရှိပါ။
list-first-sync = သင့်မေးလ်ကို ရယူနေသည်…
list-first-sync-detail = ရောက်လာသည်နှင့် ဤနေရာတွင် ပေါ်လာမည်။

## Mail list: lines

row-removed = ဤမက်ဆေ့ဂျ်ကို ဖယ်ရှားလိုက်ပြီ။
row-starred = ကြယ်ပွင့်တပ်ထားသည်
row-not-starred = ကြယ်ပွင့်မတပ်ထားပါ
row-important = အရေးကြီးသည်။ အရေးမကြီးအဖြစ် မှတ်ရန် နှိပ်ပါ။
row-mark-important = အရေးကြီးအဖြစ် မှတ်ရန်
row-pinned = ထိပ်တွင် ပင်ထိုးထားသည်
row-tracking-none = ခြေရာခံထားသည်။ မဖွင့်ရသေးပါ
row-tracking-opened = { $recipients } ဦးအနက် { $opened } ဦး ဖွင့်ခဲ့သည်
row-tracking-clicked = { $recipients } ဦးအနက် { $opened } ဦး ဖွင့်ခဲ့ပြီး { $clicked } ဦး လင့်ခ်ကို ဖွင့်ခဲ့သည်
row-pin = ထိပ်တွင် ပင်ထိုးရန်
row-unpin = ပင်ဖြုတ်ရန်
row-snoozed-until = { $when } အထိ ခဏဆိုင်းထားသည်

## Mail list: More menu and right-click menu

menu-reply = ပြန်စာရေးရန်
menu-reply-all = အားလုံးကို ပြန်စာရေးရန်
menu-forward = ထပ်ဆင့်ပို့ရန်
menu-archive = မှတ်တမ်းသိမ်းရန်
menu-delete = ဖျက်ရန်
menu-delete-forever = အပြီးဖျက်ရန်
menu-move-to-inbox = ဝင်စာသို့ ရွှေ့ရန်
menu-spam = စပမ်းအဖြစ် တိုင်ကြားရန်
menu-not-spam = စပမ်း မဟုတ်ပါ
menu-mark-read = ဖတ်ပြီးအဖြစ် မှတ်ရန်
menu-mark-unread = မဖတ်ရသေးအဖြစ် မှတ်ရန်
menu-mark-all-read = အားလုံးကို ဖတ်ပြီးအဖြစ် မှတ်ရန်
menu-star = ကြယ်ပွင့်တပ်ရန်
menu-unstar = ကြယ်ပွင့်ဖြုတ်ရန်
menu-important = အရေးကြီးအဖြစ် မှတ်ရန်
menu-not-important = အရေးမကြီးအဖြစ် မှတ်ရန်
menu-pin = ထိပ်တွင် ပင်ထိုးရန်
menu-unpin = ပင်ဖြုတ်ရန်
menu-snooze = ခဏဆိုင်းရန်
menu-unsnooze = ခဏဆိုင်းခြင်း ပယ်ရန်
menu-print-all = အားလုံးကို ပုံနှိပ်ရန်
menu-new-window = ဝင်းဒိုးအသစ်တွင် ဖွင့်ရန်
menu-move-to = သို့ ရွှေ့ရန်
menu-move-to-heading = သို့ ရွှေ့ရန်-
menu-find-from = { $name } ထံမှ မေးလ်များကို ရှာရန်

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] စကားဝိုင်း { $count } ခုကို မှတ်တမ်းသိမ်းလိုက်ပြီ။
   *[message] မက်ဆေ့ဂျ် { $count } စောင်ကို မှတ်တမ်းသိမ်းလိုက်ပြီ။
}
toast-trashed = { $kind ->
    [conversation] စကားဝိုင်း { $count } ခုကို အမှိုက်ပုံးသို့ ရွှေ့လိုက်ပြီ။
   *[message] မက်ဆေ့ဂျ် { $count } စောင်ကို အမှိုက်ပုံးသို့ ရွှေ့လိုက်ပြီ။
}
toast-moved = { $kind ->
    [conversation] စကားဝိုင်း { $count } ခုကို ရွှေ့လိုက်ပြီ။
   *[message] မက်ဆေ့ဂျ် { $count } စောင်ကို ရွှေ့လိုက်ပြီ။
}
toast-starred = { $kind ->
    [conversation] စကားဝိုင်း { $count } ခုကို ကြယ်ပွင့်တပ်လိုက်ပြီ။
   *[message] မက်ဆေ့ဂျ် { $count } စောင်ကို ကြယ်ပွင့်တပ်လိုက်ပြီ။
}
toast-unstarred = { $kind ->
    [conversation] စကားဝိုင်း { $count } ခုမှ ကြယ်ပွင့်ဖြုတ်လိုက်ပြီ။
   *[message] မက်ဆေ့ဂျ် { $count } စောင်မှ ကြယ်ပွင့်ဖြုတ်လိုက်ပြီ။
}
toast-important = { $kind ->
    [conversation] စကားဝိုင်း { $count } ခုကို အရေးကြီးအဖြစ် မှတ်လိုက်ပြီ။
   *[message] မက်ဆေ့ဂျ် { $count } စောင်ကို အရေးကြီးအဖြစ် မှတ်လိုက်ပြီ။
}
toast-not-important = { $kind ->
    [conversation] စကားဝိုင်း { $count } ခုကို အရေးမကြီးအဖြစ် မှတ်လိုက်ပြီ။
   *[message] မက်ဆေ့ဂျ် { $count } စောင်ကို အရေးမကြီးအဖြစ် မှတ်လိုက်ပြီ။
}
toast-pinned = { $kind ->
    [conversation] စကားဝိုင်း { $count } ခုကို ထိပ်တွင် ပင်ထိုးလိုက်ပြီ။
   *[message] မက်ဆေ့ဂျ် { $count } စောင်ကို ထိပ်တွင် ပင်ထိုးလိုက်ပြီ။
}
toast-unpinned = { $kind ->
    [conversation] စကားဝိုင်း { $count } ခုကို ပင်ဖြုတ်လိုက်ပြီ။
   *[message] မက်ဆေ့ဂျ် { $count } စောင်ကို ပင်ဖြုတ်လိုက်ပြီ။
}
toast-snoozed = { $kind ->
    [conversation] စကားဝိုင်း { $count } ခုကို { $when } အထိ ခဏဆိုင်းလိုက်ပြီ။
   *[message] မက်ဆေ့ဂျ် { $count } စောင်ကို { $when } အထိ ခဏဆိုင်းလိုက်ပြီ။
}
toast-unsnoozed = { $kind ->
    [conversation] စကားဝိုင်း { $count } ခု ဝင်စာသို့ ပြန်ရောက်လာပြီ။
   *[message] မက်ဆေ့ဂျ် { $count } စောင် ဝင်စာသို့ ပြန်ရောက်လာပြီ။
}
toast-spam = { $kind ->
    [conversation] စကားဝိုင်း { $count } ခုကို စပမ်းအဖြစ် တိုင်ကြားလိုက်ပြီ။
   *[message] မက်ဆေ့ဂျ် { $count } စောင်ကို စပမ်းအဖြစ် တိုင်ကြားလိုက်ပြီ။
}
toast-not-spam = { $kind ->
    [conversation] စကားဝိုင်း { $count } ခုကို စပမ်း မဟုတ်ဟု မှတ်ပြီး ဝင်စာသို့ ရွှေ့လိုက်ပြီ။
   *[message] မက်ဆေ့ဂျ် { $count } စောင်ကို စပမ်း မဟုတ်ဟု မှတ်ပြီး ဝင်စာသို့ ရွှေ့လိုက်ပြီ။
}
toast-deleted-forever = { $kind ->
    [conversation] စကားဝိုင်း { $count } ခုကို အပြီးဖျက်လိုက်ပြီ။
   *[message] မက်ဆေ့ဂျ် { $count } စောင်ကို အပြီးဖျက်လိုက်ပြီ။
}
toast-marked-read = { $kind ->
    [conversation] စကားဝိုင်း { $count } ခုကို ဖတ်ပြီးအဖြစ် မှတ်လိုက်ပြီ။
   *[message] မက်ဆေ့ဂျ် { $count } စောင်ကို ဖတ်ပြီးအဖြစ် မှတ်လိုက်ပြီ။
}
toast-marked-unread = { $kind ->
    [conversation] စကားဝိုင်း { $count } ခုကို မဖတ်ရသေးအဖြစ် မှတ်လိုက်ပြီ။
   *[message] မက်ဆေ့ဂျ် { $count } စောင်ကို မဖတ်ရသေးအဖြစ် မှတ်လိုက်ပြီ။
}
toast-undone = လုပ်ဆောင်ချက်ကို နောက်ပြန်ဆုတ်လိုက်ပြီ။
toast-nothing-to-undo = နောက်ပြန်ရန် ဘာမျှ မရှိပါ။
toast-cannot-undo-delete-forever = အပြီးဖျက်ထားသော မေးလ်ကို ပြန်ယူ၍ မရပါ။
toast-send-undone = ပို့ခြင်းကို နောက်ပြန်ဆုတ်လိုက်ပြီ။
toast-too-late-to-undo-send = နောက်ပြန်ရန် နောက်ကျသွားပြီ- မက်ဆေ့ဂျ်ကို ပို့ပြီးသွားပြီ။
toast-undo = နောက်ပြန်ရန်
toast-no-spam-folder = ဤအကောင့်တွင် စပမ်းဖိုင်တွဲ မရှိပါ။
