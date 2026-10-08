# Katna Mail, Burmese (မြန်မာ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification (the desktop shows it, even with Katna Mail closed)

notify-new-emails = အီးမေးလ်အသစ် { $count } စောင်
notify-and-more = နှင့် နောက်ထပ် { $count } စောင်
notify-no-subject = (ခေါင်းစဉ်မရှိ)
notify-unknown-sender = မသိသော ပို့သူ

## Reminders the user asked for (same buttons)

notify-snooze-back = ခဏဆိုင်းထားရာမှ ပြန်ရောက်လာပြီ
notify-no-reply = ပြန်စာ မရသေးပါ
notify-no-reply-to = “{ $subject }” ကို မည်သူမျှ ပြန်စာ မပို့သေးပါ။
notify-follow-up-sent = နောက်ဆက်တွဲ ပို့ပြီး
notify-follow-up-sent-to = “{ $subject }” ကို မည်သူမျှ ပြန်စာ မရေးသဖြင့် Katna က နောက်ဆက်တွဲ ပို့လိုက်သည်။
notify-follow-up-waiting = နောက်ဆက်တွဲ မပို့ရပါ
notify-follow-up-waiting-to = ဤကွန်ပျူတာ ပိတ်ထားစဉ် အချိန်ကျသွားသည်။ “{ $subject }” သည် သင့်ဝင်စာသို့ ပြန်ရောက်နေပြီ။

## Open and click tracking (only for mail sent with "Track opens and clicks")

notify-tracking-opened = { $who } က { $subject } ကို ဖွင့်လိုက်သည်
notify-tracking-clicked = { $who } က { $subject } ထဲရှိ လင့်ခ်တစ်ခုကို နှိပ်လိုက်သည်

## An update of Katna is downloaded and ready to install

notify-update-ready = Katna Mail ကို အပ်ဒိတ်လုပ်နိုင်သည်
notify-update-ready-body = ဗားရှင်း { $version } ကို ဒေါင်းလုဒ်လုပ်ပြီးပါပြီ။ အပ်ဒိတ်ကို နှိပ်လျှင် ၎င်းကို ထည့်သွင်းပြီး Katna Mail ကို ပြန်လည်စတင်ပေးသည်။
notify-update = အပ်ဒိတ်

## Something needs the user, shown once per problem

notify-signed-out = ထပ်မံ ဝင်ရောက်ပါ
notify-signed-out-body = { $provider } က { $address } မှ Katna ကို ထွက်လိုက်သည်။ မေးလ် စင့်ခ်လုပ်ခြင်း ရပ်သွားပြီ။
notify-sign-in = ဝင်ရောက်ရန်
notify-password-refused = စကားဝှက်ကို ငြင်းပယ်သည်
notify-password-refused-body = မေးလ်ဆာဗာက { $address } အတွက် စကားဝှက်ကို ငြင်းပယ်သည်။ ၎င်း ပြောင်းသွားနိုင်သည်။
notify-new-password = စကားဝှက်အသစ်
notify-not-sent = “{ $subject }” ကို မပို့ရပါ
notify-not-sent-no-subject = မက်ဆေ့ဂျ်တစ်စောင် မပို့ရပါ
notify-not-sent-body = ၎င်းသည် ထွက်စာတွင် ရှိပြီး အကြောင်းရင်းကို ပြထားသည်။
notify-open-outbox = ထွက်စာ ဖွင့်ရန်

## Reminders of calendar events

notify-event-now = ယခု
notify-event-in-minutes = { $count ->
   *[other] { $count } မိနစ်အတွင်း
}
notify-event-in-hours = { $count ->
   *[other] { $count } နာရီအတွင်း
}
notify-event-in-days = { $count ->
    [1] မနက်ဖြန်
   *[other] { $count } ရက်အတွင်း
}
notify-event-all-day = တစ်ရက်လုံး
notify-event-join = ပါဝင်ရန်
notify-event-snooze = 5 မိနစ် ခဏဆိုင်းရန်
notify-task-done = ပြီးစီးကြောင်း အမှတ်အသားပြုရန်

## The buttons of new-mail notifications and reminders

notify-open = ဖွင့်ရန်
notify-peek = ကြည့်ရန်
notify-reply = ပြန်စာရေးရန်
notify-reply-placeholder = { $name } ထံ ပြန်စာရေးရန်…
notify-send = ပို့ရန်
notify-reply-quote-header = { $date } တွင် { $from } က ရေးခဲ့သည်-
notify-reply-quote-header-no-date = { $from } က ရေးခဲ့သည်-
notify-reply-all = အားလုံးကို ပြန်စာရေးရန်
notify-mark-read = ဖတ်ပြီးအဖြစ် မှတ်ရန်
notify-mark-all-read = အားလုံးကို ဖတ်ပြီးအဖြစ် မှတ်ရန်
notify-archive = မှတ်တမ်းသိမ်းရန်
notify-snooze-hour = ၁ နာရီ ခဏဆိုင်းရန်
notify-snooze-tomorrow = မနက်ဖြန်
notify-copy-code = { $code } ကူးရန်
notify-link-verify = { $domain } တွင် အတည်ပြုရန်
notify-link-confirm = { $domain } တွင် အတည်ပြုရန်
notify-link-activate = { $domain } တွင် စတင်အသုံးပြုရန်

## After Archive on a notification: a short note in the same place

notify-archived = မှတ်တမ်းသိမ်းပြီး
notify-archived-count = { $count ->
   *[other] မက်ဆေ့ဂျ် { $count } စောင်ကို ဝင်စာမှ ရွှေ့ပြီး
}
notify-undo = နောက်ပြန်ရန်

## After Copy on a notification: a short note, with the code under the title

notify-code-copied = ကုဒ် ကူးပြီး
notify-code-not-copied = ကုဒ်ကို မကူးနိုင်ပါ

## it waits for the undo time

notify-reply-sent = { $name } ထံ ပြန်စာ ပို့ပြီး
notify-open-in-katna = Katna တွင် ဖွင့်ရန်
