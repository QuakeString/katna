# Katna Mail, Burmese (မြန်မာ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = မေးလ်အကောင့် ထည့်ရန်
add-account-looking = { $address } ၏ မေးလ်ဆာဗာများကို ရှာနေသည်…
add-account-address-intro = သင့်အီးမေးလ်လိပ်စာကို ထည့်ပါ။ Katna က ဆာဗာများကို ရှာပေးမည်။
add-account-servers-title = ဆာဗာ ဆက်တင်များ
add-account-servers-intro = { $address } အတွက် Katna မေးလ် ဖတ်ပြီး ပို့သည့်နေရာ။
add-account-password-title = သင့်စကားဝှက်ကို ထည့်ပါ
add-account-signing-in = ဝင်ရောက်နေသည်…

## Add a mail account: fields

add-account-field-address = အီးမေးလ်လိပ်စာ
add-account-incoming = ဝင်လာသော မေးလ် ({ $protocol })
add-account-outgoing = ထွက်သွားသော မေးလ် ({ $protocol })
add-account-field-server = ဆာဗာ
add-account-field-port = ပေါ့တ်
add-account-security-none = မရှိ
add-account-field-username = အသုံးပြုသူအမည်
add-account-field-password = စကားဝှက်
add-account-show-password = စကားဝှက် ပြရန်
add-account-app-password-hint = { $provider } သည် ဤနေရာတွင် ဝဘ်ပေါ်တွင် သုံးသော စကားဝှက် မဟုတ်ဘဲ အက်ပ်စကားဝှက် လိုအပ်သည်။ သင့် { $provider } အကောင့်၏ လုံခြုံရေးဆက်တင်များတွင် တစ်ခု ပြုလုပ်ပါ။
add-account-field-name = သင့်အမည် (ရွေးချယ်နိုင်)
add-account-name-hint = သင် စာရေးပို့သူများကို ပြသည်။
add-account-servers-pair = { $imap } နှင့် { $smtp }
add-account-servers-found = { $source ->
    [built-in] ဆာဗာများ- { $servers }၊ Katna ၏ ဝန်ဆောင်မှုပေးသူ စာရင်းတွင် တွေ့သည်။
    [provider] ဆာဗာများ- { $servers }၊ သင့်ဝန်ဆောင်မှုပေးသူ၏ ဆက်တင်များတွင် တွေ့သည်။
    [ispdb] ဆာဗာများ- { $servers }၊ Thunderbird ၏ ဝန်ဆောင်မှုပေးသူ စာရင်းတွင် တွေ့သည်။
    [dns] ဆာဗာများ- { $servers }၊ သင့်ဒိုမိန်း၏ DNS မှတ်တမ်းများတွင် တွေ့သည်။
   *[other] ဆာဗာများ- { $servers }၊ ခန့်မှန်းထားသည်။ ဝင်ရောက်မှု မအောင်မြင်ပါက စစ်ဆေးပါ။
}
add-account-servers-entered = ဆာဗာများ- { $servers }၊ ထည့်သွင်းထားသည့်အတိုင်း။

## Add a mail account: buttons

add-account-servers-button = ဆာဗာ ဆက်တင်များ
add-account-back = နောက်သို့
add-account-add = အကောင့်ထည့်ရန်
add-account-next = ရှေ့သို့
add-account-cancel = မလုပ်တော့ပါ

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] ဝင်လာသော မေးလ်ဆာဗာကို ထည့်ပါ။
   *[outgoing] ထွက်သွားသော မေးလ်ဆာဗာကို ထည့်ပါ။
}
add-account-server-space = { $kind ->
    [incoming] ဝင်လာသော မေးလ်ဆာဗာ အမည်တွင် space ပါနေသည်။
   *[outgoing] ထွက်သွားသော မေးလ်ဆာဗာ အမည်တွင် space ပါနေသည်။
}
add-account-port-invalid = { $kind ->
    [incoming] ဝင်လာသော မေးလ် ပေါ့တ်သည် { $min } မှ { $max } အထိ နံပါတ် ဖြစ်ရမည်။
   *[outgoing] ထွက်သွားသော မေးလ် ပေါ့တ်သည် { $min } မှ { $max } အထိ နံပါတ် ဖြစ်ရမည်။
}
add-account-address-empty = အီးမေးလ်လိပ်စာ ထည့်ပါ။
add-account-address-invalid = { $example } ကဲ့သို့ အီးမေးလ်လိပ်စာ ထည့်ပါ။
add-account-not-found = Katna သည် { $address } အတွက် ဆာဗာများကို ရှာမတွေ့သဖြင့် ပုံမှန်အမည်များကို ဖြည့်ထားသည်။ သင့်ဝန်ဆောင်မှုပေးသူနှင့် စစ်ဆေးပါ။
add-account-password-empty = စကားဝှက်ကို ထည့်ပါ။
add-account-name-is-password = အမည်သည် စကားဝှက်နှင့် တူနေသည်။ ထိုနေရာတွင် လူများ မြင်စေလိုသည့်အတိုင်း သင့်အမည်ကို ရိုက်ထည့်ပါ။
add-account-added = { $address } ကို ထည့်ပြီးပါပြီ။ သင့်မေးလ်ကို ရယူနေသည်…
add-account-app-password-refused = { $provider } က စကားဝှက်ကို ငြင်းပယ်သည်။ ဝဘ်ပေါ်တွင် သုံးသော စကားဝှက် မဟုတ်ဘဲ အက်ပ်စကားဝှက် လိုအပ်သည်။
add-account-password-refused = ဆာဗာက စကားဝှက်ကို ငြင်းပယ်သည်။ စစ်ဆေးပြီး ထပ်ကြိုးစားပါ။

## The account menu (from the account button on the top bar)

add-account-menu-another = နောက်ထပ်အကောင့် ထည့်ရန်
add-account-menu-manage = အကောင့်များ စီမံရန်
