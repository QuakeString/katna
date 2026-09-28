# Katna Mail, Burmese (မြန်မာ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Compose window: title bar

compose-new-message = မက်ဆေ့ဂျ်အသစ်
compose-restore = မူလအရွယ်ပြန်ထားရန်
compose-minimize = ချုံ့ရန်
compose-exit-full-screen = မျက်နှာပြင်အပြည့်မှ ထွက်ရန်
compose-open-window = ဝင်းဒိုးအသစ်တွင် ဖွင့်ရန်
compose-save-close = သိမ်းပြီး ပိတ်ရန်
compose-back-to-mail = မေးလ်ဝင်းဒိုးသို့ ပြန်သွားရန်
compose-pop-out-reply = ပြန်စာကို သီးခြားဝင်းဒိုးတွင် ဖွင့်ရန်
compose-edit-recipients = လက်ခံသူများကို တည်းဖြတ်ရန်
compose-summary-cc = မိတ္တူ- { $names }
compose-summary-bcc = လျှို့ဝှက်မိတ္တူ- { $names }
compose-more-recipients = နောက်ထပ် { $count } ဦး
compose-show-trimmed = ဖြတ်ထားသော အကြောင်းအရာကို ပြရန်
compose-hide-trimmed = ဖြတ်ထားသော အကြောင်းအရာကို ဝှက်ရန်
compose-remove-trimmed = ကိုးကားထားသော စာသားကို ဖယ်ရှားရန်
compose-trimmed-removed = ကိုးကားထားသော စာသားကို ဖယ်ရှားပြီးပါပြီ

## Recipients and subject

compose-to = သို့
compose-cc = မိတ္တူ
compose-bcc = လျှို့ဝှက်မိတ္တူ
compose-from = မှ
compose-from-choose = အခြားအကောင့်မှ ပို့ရန်
compose-recipients = လက်ခံသူများ
compose-subject = ခေါင်းစဉ်

## Sending (the notes at the bottom of the window)

compose-open-elsewhere = ဖွင့်ထားသော မက်ဆေ့ဂျ်ကို အရင် ပို့ပါ သို့မဟုတ် ပယ်ပါ။
compose-bad-address = “{ $address }” သည် အီးမေးလ်လိပ်စာ မဟုတ်ပါ။
compose-no-recipients = လက်ခံသူ အနည်းဆုံး တစ်ဦး ထည့်ပါ။
compose-attachments-too-large = ပူးတွဲဖိုင်များသည် { $size } ရှိသည်။ မေးလ်ဆာဗာများက { $limit } အထိသာ လက်ခံသည်။
compose-no-account = မေးလ်ပို့ရန် အကောင့်တစ်ခု ထည့်ပါ။
compose-past-time = အနာဂတ်အချိန်တစ်ခု ရွေးပါ။
compose-scheduling = အချိန်ဇယားသတ်မှတ်နေသည်…
compose-sending = ပို့နေသည်…
compose-scheduled = { $when } တွင် ပို့ရန် အချိန်ဇယား သတ်မှတ်ပြီး
compose-sent-archived = ပို့ပြီး မှတ်တမ်းသိမ်းပြီး
compose-sent = မက်ဆေ့ဂျ် ပို့ပြီး
compose-discarded = မူကြမ်းကို ပယ်လိုက်ပြီ
compose-draft-saved = မူကြမ်းကို သိမ်းလိုက်ပြီ
compose-draft-failed = မူကြမ်းကို သိမ်း၍ မရပါ- { $error }
compose-draft-not-opened = မူကြမ်းကို ဖွင့်၍ မရပါ။

## Attachments

compose-picker-insert = ထည့်သွင်းရန်
compose-picker-attach = ပူးတွဲရန်
compose-file-too-large = { $name } သည် ကြီးလွန်းသည်- မက်ဆေ့ဂျ်တစ်ခုတွင် { $limit } အထိသာ ပါနိုင်သည်။
compose-attachment-size = ({ $size })
compose-remove-attachment = ပူးတွဲဖိုင်ကို ဖယ်ရှားရန်
compose-attachments-total = ဖိုင် { $count } ခု၊ { $size }
compose-drive-note = { $name } သည် { $limit } ထက်ကျော်နေသဖြင့် သင့် Google Drive သို့ ပို့ပြီး မက်ဆေ့ဂျ်တွင် လင့်ခ်ပါမည်။
compose-drive-tip = သင့် Google Drive ထဲတွင်ရှိသည်။ မက်ဆေ့ဂျ်တွင် လင့်ခ်ပါမည်
compose-drive-uploading = အပ်လုဒ်လုပ်နေသည် { $percent }%
compose-drive-allow = Drive ကို ခွင့်ပြုရန်
compose-drive-allow-tip = Katna က ဖိုင်ကြီးများကို သင့် Drive ထဲ ထည့်နိုင်ရန် Google ဖြင့် ထပ်မံ ဝင်ရောက်ပါ
compose-drive-retry = ထပ်စမ်းကြည့်ရန်
compose-drive-sends-when-uploaded = { $name } အပ်လုဒ်လုပ်ပြီးသည်နှင့် ပို့မည်
compose-drive-not-uploaded = { $name } သည် Google Drive ထဲတွင် မရှိသေးပါ
compose-drive-share-failed = Google Drive ထဲရှိ ဖိုင်များကို မျှဝေ၍ မရပါ- { $error }
compose-drive-share-title = ဖိုင်များကို လူတိုင်းနှင့် မျှဝေမလား။
compose-drive-share-text = { $count ->
   *[other] Google Drive က Google အကောင့်မရှိသော { $addresses } နှင့် ဖိုင်များကို မျှဝေ၍ မရပါ။ ထို့အစား လင့်ခ်ရှိသူတိုင်း ဖွင့်ကြည့်နိုင်ပါမည်။
}
compose-drive-share-link = လင့်ခ်ဖြင့် မျှဝေရန်
compose-drive-send-without = မမျှဝေဘဲ ပို့ရန်
compose-drive-share-cancel = မလုပ်တော့ပါ
compose-drive-card-detail = { $size } · Google Drive
compose-drop-files = ဖိုင်များကို ဤနေရာတွင် ချပါ
compose-drop-here = ဤနေရာတွင် ချပါ
compose-paste-keep-formatting = ပုံစံချမှုကို ထားရန်
compose-paste-table = ဇယား
compose-paste-picture = ပုံ
compose-paste-plain-text = စာသားသက်သက်
compose-paste-inline = စာထဲတွင်
compose-paste-attachment = ပူးတွဲဖိုင်

## Encryption and signing (the toggles by the recipients)

compose-encrypt = ကုဒ်ဝှက်ရန်
compose-encrypted = ကုဒ်ဝှက်ထားသည်- လက်ခံသူများသာ ဖတ်နိုင်သည်
compose-sign = လက်မှတ်ထိုးရန်
compose-signed = လက်မှတ်ထိုးထားသည်- သင့်ထံမှ ဖြစ်ကြောင်း လက်ခံသူများ စစ်ဆေးနိုင်သည်
compose-track = ဖွင့်ခြင်းနှင့် နှိပ်ခြင်းကို ခြေရာခံရန်
compose-tracked = ခြေရာခံထားသည်- လက်ခံသူတစ်ဦးစီ ၎င်းကို ဖွင့်သည့်အချိန် သို့မဟုတ် လင့်ခ်ကို ဖွင့်သည့်အချိန်ကို သင် မြင်ရမည်
compose-track-clicks = လင့်ခ်နှိပ်မှုကို ခြေရာခံရန် (စာသားသက်သက်တွင် ဖွင့်ခြင်းကို မပြနိုင်ပါ)
compose-tracked-clicks = ခြေရာခံထားသည်- လက်ခံသူတစ်ဦးစီ လင့်ခ်ကို နှိပ်သည့်အချိန်ကို သင် မြင်ရမည်
compose-track-sign-in = ဖွင့်ခြင်းနှင့် နှိပ်ခြင်းကို ခြေရာခံရန် Katna အကောင့်သို့ ဝင်ရောက်ပါ
compose-receipt = ဖတ်ပြီးကြောင်း အသိအမှတ်ပြုချက် တောင်းရန်
compose-receipt-on = ဖတ်ပြီးကြောင်း အသိအမှတ်ပြုချက် တောင်းထားသည်- လက်ခံသူ၏ အက်ပ်က ၎င်းကို ပို့ရန် သူတို့ကို မေးနိုင်သည်
compose-delivery = ပို့ဆောင်ပြီးကြောင်း အသိအမှတ်ပြုချက် တောင်းရန်
compose-delivery-on = ပို့ဆောင်ပြီးကြောင်း အသိအမှတ်ပြုချက် တောင်းထားသည်- လက်ခံသူတစ်ဦးစီ၏ ဆာဗာက ၎င်းကို လက်ခံသည့်အခါ သင့်မေးလ်ဆာဗာက သင့်ထံ အီးမေးလ် ပို့ပေးမည်
compose-delivery-unavailable = သင့်မေးလ်ဆာဗာသည် ပို့ဆောင်ပြီးကြောင်း အသိအမှတ်ပြုချက်များ မပို့ပါ

## Spelling

spell-no-dictionary = { $language } အတွက် စာလုံးပေါင်းအဘိဓာန် ထည့်သွင်းမထားပါ (ဥပမာ hunspell-en_us)။
spell-dictionary-error = စာလုံးပေါင်းအဘိဓာန်- { $error }

## Grammar checking (the right-click menu on a grammar mistake)

grammar-replace = “{ $words }”
grammar-add = “{ $words }” ထည့်ရန်
grammar-remove = “{ $words }” ဖယ်ရှားရန်
grammar-ignore = လျစ်လျူရှုရန်

## Send checks (asked before a message goes out)

send-check-attachment-title = ဖိုင်များ ပူးတွဲရန် ရည်ရွယ်ခဲ့ပါသလား။
send-check-attachment-text = ပူးတွဲဖိုင်အကြောင်း ရေးထားသော်လည်း ဘာမျှ ပူးတွဲမထားပါ။
send-check-attach = ဖိုင်တစ်ခု ပူးတွဲရန်
send-check-subject-title = ခေါင်းစဉ်မပါဘဲ ပို့မလား။
send-check-subject-text = ဤမက်ဆေ့ဂျ်တွင် ခေါင်းစဉ်မရှိပါ။
send-check-add-subject = ခေါင်းစဉ်ထည့်ရန်
send-check-send-anyway = မည်သို့ပင်ဖြစ်စေ ပို့ရန်
recipient-not-valid = မှန်ကန်သော အီးမေးလ်လိပ်စာ မဟုတ်ပါ
recipient-show-address = လိပ်စာ ပြရန်
recipient-remove = ဖယ်ရှားရန်
recipient-bad-title = လိပ်စာကို စစ်ဆေးပါ
recipient-bad-text = “{ $address }” သည် မှန်ကန်သော အီးမေးလ်လိပ်စာ မဟုတ်ပါ။ မပို့မီ ၎င်းကို ပြင်ပါ သို့မဟုတ် ဖယ်ရှားပါ။
recipient-bad-fix = ပြင်ရန်
