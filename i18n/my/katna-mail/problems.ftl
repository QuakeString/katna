# Katna Mail, Burmese (မြန်မာ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = မေးလ်ဆာဗာ
problems-signed-out = { $provider } က { $address } မှ Katna ကို ထွက်လိုက်သည်။ မေးလ် စင့်ခ်လုပ်ခြင်း ရပ်သွားပြီ။
problems-password-refused = { $provider } က { $address } အတွက် စကားဝှက်ကို ငြင်းပယ်သည်။ ၎င်း ပြောင်းသွားနိုင်သည်။
problems-no-answer = { $provider } က { $address } အတွက် တုံ့ပြန်မှု မရှိပါ။ Katna က ဆက်ကြိုးစားနေသည်။
problems-offline = သင် အော့ဖ်လိုင်း ဖြစ်နေသည်။ သင့်မေးလ်များ ဤနေရာတွင် ရှိနေဆဲဖြစ်ပြီး သင်ပို့သော မေးလ်များသည် အွန်လိုင်း ပြန်ဖြစ်သည်အထိ စောင့်နေမည်။
problems-accounts-need-you = { $count ->
   *[other] အကောင့် { $count } ခုက သင့်ကို လိုအပ်နေသည်
}
problems-show = ပြရန်
problems-later = နောက်မှ
problems-new-password = စကားဝှက်အသစ်
problems-try-again = ထပ်ကြိုးစားရန်

## The New password card

problems-password-title = စကားဝှက်အသစ်
problems-password-detail = { $provider } က { $address } အတွက် သိမ်းထားသော စကားဝှက်ကို ငြင်းပယ်သည်။ အသစ်ကို ရိုက်ထည့်ပါ၊ Katna က မသိမ်းမီ စစ်ဆေးမည်။
problems-password-placeholder = စကားဝှက်
problems-password-show = စကားဝှက် ပြရန်
problems-password-hide = စကားဝှက် ဖျောက်ရန်
problems-password-cancel = မလုပ်တော့ပါ
problems-password-save = သိမ်းရန်
problems-password-checking = စစ်ဆေးနေသည်…
problems-password-refused-again = { $provider } က ဤစကားဝှက်ကိုလည်း ငြင်းပယ်သည်။ စစ်ဆေးပြီး ထပ်ကြိုးစားပါ။
problems-password-saved = { $address } အတွက် စကားဝှက် သိမ်းပြီး။ သင့်မေးလ်ကို ရယူနေသည်…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = { $address } ၏ မေးလ်ဆာဗာက { $count ->
   *[other] မက်ဆေ့ဂျ် { $count } စောင် ရွှေ့ခြင်းကို လက်မခံသဖြင့် ၎င်းတို့ မူလနေရာသို့ ပြန်ရောက်သွားပြီ။
}
problems-refused-flags = { $address } ၏ မေးလ်ဆာဗာက { $count ->
   *[other] မက်ဆေ့ဂျ် { $count } စောင်ကို မှတ်ခြင်း (ဖတ်ပြီး၊ ကြယ်ပွင့်…) ကို လက်မခံသဖြင့် ၎င်းတို့ မူလအတိုင်း ပြန်ဖြစ်သွားပြီ။
}
problems-refused-label = { $address } ၏ မေးလ်ဆာဗာက { $count ->
   *[other] မက်ဆေ့ဂျ် { $count } စောင်၏ အညွှန်းများ ပြောင်းခြင်းကို လက်မခံသဖြင့် ၎င်းတို့ မူလအတိုင်း ပြန်ဖြစ်သွားပြီ။
}
problems-refused-delete = { $address } ၏ မေးလ်ဆာဗာက { $count ->
   *[other] မက်ဆေ့ဂျ် { $count } စောင် ဖျက်ခြင်းကို လက်မခံသဖြင့် ၎င်းတို့ ပြန်ရောက်လာပြီ။
}
problems-refused-other = { $address } ၏ မေးလ်ဆာဗာက { $count ->
   *[other] ပြောင်းလဲမှု { $count } ခုကို လက်မခံသဖြင့် Katna က မူလအတိုင်း ပြန်ထားလိုက်သည်။
}
problems-details = အသေးစိတ်

## Katna's background service (katna-daemon) isn't running

service-starting = Katna ၏ နောက်ခံ ဝန်ဆောင်မှုကို စတင်နေသည်…
service-failed = Katna ၏ နောက်ခံ ဝန်ဆောင်မှု မစတင်နိုင်သဖြင့် မေးလ် စင့်ခ်မလုပ်ပါ။
service-start-again = ထပ်စတင်ရန်
service-started-again = Katna ၏ နောက်ခံ ဝန်ဆောင်မှု ရပ်သွားပြီး ပြန်စတင်လိုက်သည်။
service-details-title = ဝန်ဆောင်မှု မစတင်နိုင်ရသည့် အကြောင်း
service-details-body = ဤအရာကို ကူးပြီး သင့်အစီရင်ခံစာနှင့်အတူ ပို့ပါ။ ၎င်းတွင် မေးလ် သို့မဟုတ် စကားဝှက် မပါပါ။
service-details-copy = ကူးရန်
service-details-close = ပိတ်ရန်
service-not-running = Katna နောက်ခံဝန်ဆောင်မှု လည်ပတ်မနေပါ။
service-no-answer = Katna နောက်ခံဝန်ဆောင်မှု မဖြေကြားပါ- { $error }
service-no-session = D-Bus ဆက်ရှင် မရှိပါ- { $error }

## Safe mode: an update left Katna's background service unable to start

safe-line = အပ်ဒိတ်တွင် ပြဿနာရှိခဲ့သဖြင့် Katna သည် လုံခြုံရေးမုဒ်တွင် ရှိနေပြီး မေးလ် စင့်ခ်မလုပ်ပါ။
safe-try-again = ထပ်ကြိုးစားရန်
safe-restore = ပြန်ယူရန်
safe-restoring = { $when } မှ သင့်ဒေတာကို ပြန်ယူနေသည်…
safe-restored = { $when } မှ သင့်ဒေတာကို ပြန်ယူပြီးပါပြီ။ ယခင်ရှိခဲ့သည်များကို ဖိုင်တွဲတစ်ခုတွင် သိမ်းထားသည်။
safe-show-folder = ဖိုင်တွဲ ပြရန်
safe-restore-failed = သင့်ဒေတာကို ပြန်ယူ၍ မရပါ- { $error }
safe-restore-title = အပ်ဒိတ်မတိုင်မီက သင့်ဒေတာကို ပြန်ယူမလား?
safe-restore-body = Katna သည် သင်ရွေးသော မိတ္တူသို့ ပြန်သွားမည်။ ၎င်းနောက်မှ ရောက်လာသော မေးလ်များကို သင့်အကောင့်များမှ ပြန်ဒေါင်းလုဒ်လုပ်မည်။
safe-restore-none = မိတ္တူ မရှိသေးပါ။ အပ်ဒိတ်တစ်ခုစီက သင့်ဒေတာကို မပြောင်းမီ Katna က မိတ္တူတစ်ခု ပြုလုပ်ထားသည်။
safe-restore-keep = မပို့ရသေးသော မေးလ်၊ မူကြမ်းများနှင့် စင့်ခ်မလုပ်ရသေးသော အပြောင်းအလဲများ အပါအဝင် ယခုရှိနေသည်များကို ဖိုင်တွဲတစ်ခုတွင် အရင်သိမ်းထားသဖြင့် ဘာမျှ မပျောက်ပါ။
safe-restore-cancel = မလုပ်တော့ပါ
safe-restore-mail = မေးလ်
safe-restore-pim = အကောင့်များနှင့် အဆက်အသွယ်များ
safe-restore-blobs = ပူးတွဲဖိုင်များ
safe-report-title = ဒီဘတ် အစီရင်ခံစာ
safe-report-body = ၎င်းကို ကူးပြီး သင့်ချို့ယွင်းချက် အစီရင်ခံစာတွင် ပူးတွဲပါ။ ၎င်းတွင် မေးလ်၊ လိပ်စာ သို့မဟုတ် စကားဝှက် မပါပါ။
safe-report-restore = ပြန်ယူရန်…
safe-report-copied = ဒီဘတ် အစီရင်ခံစာ ကူးပြီး
