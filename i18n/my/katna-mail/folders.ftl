# Katna Mail, Burmese (မြန်မာ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Navigation (the folders pane)

nav-labels = အညွှန်းများ
nav-folders = ဖိုင်တွဲများ
nav-label-new = အညွှန်းအသစ် ပြုလုပ်ရန်
nav-folder-new = ဖိုင်တွဲအသစ် ပြုလုပ်ရန်
nav-menu-check-mail = မေးလ်အသစ် စစ်ဆေးရန်
nav-menu-check-inbox = ဤဝင်စာကို စစ်ဆေးရန်
nav-unified-leave-out = ပေါင်းစည်း ဝင်စာမှ ချန်ထားရန်
nav-unified-bring-back = ပေါင်းစည်း ဝင်စာသို့ ပြန်ထည့်ရန်
nav-menu-sign-in-again = ထပ်မံ ဝင်ရောက်ရန်
nav-menu-new-mail = ဤအကောင့်မှ မေးလ်အသစ်
nav-menu-account-settings = အကောင့် ဆက်တင်များ
nav-account-checked = စင့်ခ်ဖြစ်နေသည် · { $ago } စစ်ဆေးခဲ့သည်
nav-account-in-sync = စင့်ခ်ဖြစ်နေသည်
nav-account-connecting = ချိတ်ဆက်နေသည်…
nav-account-offline = အော့ဖ်လိုင်း၊ ထပ်ကြိုးစားနေသည်
nav-account-signed-out = { $provider } ဝင်ရောက်မှု သက်တမ်းကုန်သွားပြီ
nav-account-password-refused = စကားဝှက်ကို လက်မခံပါ
nav-account-storage = { $total } အနက် { $used } သုံးထားသည်
nav-menu-new-subfolder = အထဲတွင် ဖိုင်တွဲအသစ်
nav-menu-new-sublabel = အထဲတွင် အညွှန်းအသစ်
nav-menu-rename = အမည်ပြောင်းရန်
nav-menu-delete = ဖျက်ရန်
nav-menu-empty-trash = အမှိုက်ပုံး ရှင်းလင်းရန်
nav-account-unnamed = အကောင့် { $number }
nav-all-accounts = အကောင့်အားလုံး
nav-expand = ဖိုင်တွဲများ ပြရန်
nav-collapse = ဖိုင်တွဲများ ဝှက်ရန်
storage-used = { $total } အနက် { $percent }% သုံးထားသည်
storage-used-detail = { $address }- { $total } အနက် { $used } သုံးထားသည်

## Special folders (the user's own folders keep their names)

folder-inbox = ဝင်စာ
folder-starred = ကြယ်ပွင့်တပ်ထားသည်
folder-snoozed = ခဏဆိုင်းထားသည်
folder-unread = မဖတ်ရသေး
folder-important = အရေးကြီး
folder-drafts = မူကြမ်းများ
folder-sent = ပို့ပြီး
folder-archive = မှတ်တမ်း
folder-spam = စပမ်း
folder-trash = အမှိုက်ပုံး
folder-all-mail = မေးလ်အားလုံး
folder-scheduled = အချိန်သတ်မှတ်ထားသည်
folder-waiting = ပြန်စာ စောင့်နေသည်
folder-waiting-short = စောင့်နေသည်
folder-reminders = သတိပေးချက်များ
folder-outbox = ထွက်စာ
folder-activity = လှုပ်ရှားမှု

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = အညွှန်းအသစ်
label-folder-new-title = ဖိုင်တွဲအသစ်
label-prompt = အညွှန်းအမည်အသစ် ထည့်ပါ-
label-folder-prompt = ဖိုင်တွဲအမည်အသစ် ထည့်ပါ-
label-name-hint = အညွှန်းအမည်
label-folder-name-hint = ဖိုင်တွဲအမည်
label-nest = အညွှန်းကို ဤအောက်တွင် ထည့်ရန်-
label-folder-nest = ဖိုင်တွဲကို ဤအောက်တွင် ထည့်ရန်-
label-cancel = မလုပ်တော့ပါ
label-create = ပြုလုပ်ရန်
label-creating = ပြုလုပ်နေသည်…
label-created = အညွှန်း “{ $name }” ကို ပြုလုပ်ပြီးပါပြီ။
label-folder-created = ဖိုင်တွဲ “{ $name }” ကို ပြုလုပ်ပြီးပါပြီ။
label-rename-title = အညွှန်း အမည်ပြောင်းရန်
label-folder-rename-title = ဖိုင်တွဲ အမည်ပြောင်းရန်
label-rename = အမည်ပြောင်းရန်
label-renaming = အမည်ပြောင်းနေသည်…
label-renamed = အညွှန်းကို “{ $name }” သို့ အမည်ပြောင်းလိုက်ပြီ။
label-folder-renamed = ဖိုင်တွဲကို “{ $name }” သို့ အမည်ပြောင်းလိုက်ပြီ။

## Deleting a folder or label (asked first)

folder-delete-title = “{ $name }” ကို ဖျက်မလား။
folder-delete-body = { $count ->
    [0] ၎င်းတွင် မေးလ် မရှိပါ။ ဖိုင်တွဲကို ဆာဗာမှ ဖယ်ရှားမည်ဖြစ်၍ ဝဘ်မေးလ်နှင့် သင့်ဖုန်းမှလည်း ပျောက်သွားမည်။
   *[other] { $kind ->
        [conversation] { $count ->
            [one] ၎င်း၏ စကားဝိုင်း { $count } ခုသည် အမှိုက်ပုံးသို့ ရောက်သွားမည်ဖြစ်၍ ပြန်ယူနိုင်ပါသေးသည်။
           *[other] ၎င်း၏ စကားဝိုင်း { $count } ခုသည် အမှိုက်ပုံးသို့ ရောက်သွားမည်ဖြစ်၍ ပြန်ယူနိုင်ပါသေးသည်။
        }
       *[message] { $count ->
            [one] ၎င်း၏ မက်ဆေ့ဂျ် { $count } စောင်သည် အမှိုက်ပုံးသို့ ရောက်သွားမည်ဖြစ်၍ ပြန်ယူနိုင်ပါသေးသည်။
           *[other] ၎င်း၏ မက်ဆေ့ဂျ် { $count } စောင်သည် အမှိုက်ပုံးသို့ ရောက်သွားမည်ဖြစ်၍ ပြန်ယူနိုင်ပါသေးသည်။
        }
    } ဖိုင်တွဲကို ဆာဗာမှ ဖယ်ရှားမည်ဖြစ်၍ ဝဘ်မေးလ်နှင့် သင့်ဖုန်းမှလည်း ပျောက်သွားမည်။
}
folder-delete-forever-body = { $count ->
    [0] ၎င်းတွင် မေးလ် မရှိပါ။ ဖိုင်တွဲကို ဆာဗာမှ ဖယ်ရှားမည်ဖြစ်၍ ဝဘ်မေးလ်နှင့် သင့်ဖုန်းမှလည်း ပျောက်သွားမည်။
   *[other] { $kind ->
        [conversation] { $count ->
            [one] ၎င်း၏ စကားဝိုင်း { $count } ခုကို အပြီးဖျက်မည်။ ဤအကောင့်တွင် အမှိုက်ပုံး မရှိပါ။
           *[other] ၎င်း၏ စကားဝိုင်း { $count } ခုကို အပြီးဖျက်မည်။ ဤအကောင့်တွင် အမှိုက်ပုံး မရှိပါ။
        }
       *[message] { $count ->
            [one] ၎င်း၏ မက်ဆေ့ဂျ် { $count } စောင်ကို အပြီးဖျက်မည်။ ဤအကောင့်တွင် အမှိုက်ပုံး မရှိပါ။
           *[other] ၎င်း၏ မက်ဆေ့ဂျ် { $count } စောင်ကို အပြီးဖျက်မည်။ ဤအကောင့်တွင် အမှိုက်ပုံး မရှိပါ။
        }
    } ဖိုင်တွဲကို ဆာဗာမှ ဖယ်ရှားမည်ဖြစ်၍ ဝဘ်မေးလ်နှင့် သင့်ဖုန်းမှလည်း ပျောက်သွားမည်။
}
folder-delete-label-body = အညွှန်းကို ဖယ်ရှားမည်။ ၎င်း၏ မေးလ်များသည် မေးလ်အားလုံးနှင့် ၎င်း၏ အခြားအညွှန်းများတွင် ကျန်ရှိမည်။
folder-delete-confirm = ဖိုင်တွဲ ဖျက်ရန်
folder-delete-label-confirm = အညွှန်း ဖျက်ရန်
folder-deleted = ဖိုင်တွဲ “{ $name }” ကို ဖျက်လိုက်ပြီ
label-deleted = အညွှန်း “{ $name }” ကို ဖျက်လိုက်ပြီ
