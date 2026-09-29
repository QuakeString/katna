# Katna Mail, Burmese (မြန်မာ): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = ယနေ့
calendar-today-tip = ယနေ့သို့ သွားရန်
calendar-view-day = ရက်
calendar-view-week = အပတ်
calendar-view-month = လ
calendar-view-schedule = အချိန်ဇယား
calendar-previous-day = ယခင်ရက်
calendar-next-day = နောက်ရက်
calendar-previous-week = ယခင်အပတ်
calendar-next-week = နောက်အပတ်
calendar-previous-month = ယခင်လ
calendar-next-month = နောက်လ
calendar-previous-period = ယခင်
calendar-next-period = နောက်
calendar-title-months = { $first } – { $last }
calendar-loading = ဖွင့်နေသည်…
calendar-read-failed = ပြက္ခဒိန်ကို ဖတ်၍မရပါ- { $error }
calendar-local = ဤကွန်ပျူတာ
calendar-account-gone = ဖယ်ရှားထားသော အကောင့်
calendar-empty-title = ပြက္ခဒိန် မရှိသေးပါ
calendar-empty-text = သင်၏ Google နှင့် Microsoft အကောင့်များ၏ ပြက္ခဒိန်များကို ထပ်တူပြုပြီးသည်နှင့် ဤနေရာတွင် ပြပါမည်။ CalDAV ပံ့ပိုးသော အခြားဆာဗာများ၏ ပြက္ခဒိန်များလည်း ပါဝင်ပါသည်။
calendar-schedule-empty = လာမည့် ၂ လအတွင်း စီစဉ်ထားသည် မရှိပါ။
calendar-no-title = (ခေါင်းစဉ်မရှိ)
calendar-all-day = တစ်နေ့လုံး
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }၊ { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = နောက်ထပ် { $count } ခု
calendar-repeats = ထပ်ခါထပ်ခါ
calendar-join = ပါဝင်ရန်
calendar-guests =
    { $count ->
       *[other] ဧည့်သည် { $count } ဦး
    }
calendar-guest-answers = ဟုတ်ကဲ့ { $yes }၊ ဖြစ်နိုင် { $maybe }၊ မဟုတ်ပါ { $no }၊ စောင့်ဆိုင်းနေ { $waiting }
calendar-organizer = စီစဉ်သူ
calendar-optional = ရွေးချယ်နိုင်သည်
calendar-open-web = ဘရောက်ဇာတွင် ဖွင့်ရန်
calendar-close = ပိတ်ရန်

## Adding, changing and deleting events.

calendar-add-title = ခေါင်းစဉ် ထည့်ရန်
calendar-add-location = တည်နေရာ ထည့်ရန်
calendar-add-notes = ဖော်ပြချက် ထည့်ရန်
calendar-add-guests = ဧည့်သည်များ ထည့်ရန်
calendar-remove-guest = ဖယ်ရှားရန်
calendar-add-meet = Google Meet ဗီဒီယိုကွန်ဖရင့် ထည့်ရန်
calendar-add-teams = Teams အစည်းအဝေး ထည့်ရန်
calendar-has-call = ဗီဒီယိုခေါ်ဆိုမှု ထည့်ပြီးပါပြီ
calendar-weekday-day = { $weekday }၊ { $day }
calendar-all-day-box = တစ်ရက်လုံး
calendar-more-options = နောက်ထပ် ရွေးစရာများ
calendar-save = သိမ်းရန်
calendar-saved = ဖြစ်ရပ်ကို သိမ်းပြီးပါပြီ
calendar-deleted = ဖြစ်ရပ်ကို ဖျက်ပြီးပါပြီ
calendar-discard = ပြောင်းလဲမှုများ ပယ်ရန်
calendar-edit = ဖြစ်ရပ် တည်းဖြတ်ရန်
calendar-delete = ဖြစ်ရပ် ဖျက်ရန်
calendar-event-details = ဖြစ်ရပ် အသေးစိတ်
calendar-kind-event = ဖြစ်ရပ်
calendar-kind-focus = အာရုံစူးစိုက်ချိန်
calendar-kind-out-of-office = ရုံးပြင်ပ
calendar-kind-working-location = အလုပ်လုပ်ရာနေရာ
calendar-working-home = အိမ်
calendar-busy = အလုပ်များ
calendar-free = အားလပ်
calendar-cancel = မလုပ်တော့ပါ
calendar-ok = အိုကေ
calendar-read-only = ဤပြက္ခဒိန်ရှိ ဖြစ်ရပ်များကို ပြောင်း၍ မရပါ
calendar-none-editable = ဖြစ်ရပ်ထည့်နိုင်သော ပြက္ခဒိန် မရှိသေးပါ
calendar-no-such-time = ထိုအချိန်သည် သင့်အချိန်ဇုန်တွင် မရှိပါ
calendar-end-before-start = ဖြစ်ရပ်သည် မစတင်မီ ပြီးဆုံးနေသည်
calendar-repeat-never = မထပ်ခါ
calendar-repeat-daily = နေ့စဉ်
calendar-repeat-weekly = အပတ်စဉ် { $weekday }
calendar-repeat-monthly =
    { $nth ->
        [1] လစဉ် ပထမ { $weekday }
        [2] လစဉ် ဒုတိယ { $weekday }
        [3] လစဉ် တတိယ { $weekday }
        [4] လစဉ် စတုတ္ထ { $weekday }
       *[other] လစဉ် နောက်ဆုံး { $weekday }
    }
calendar-repeat-yearly = နှစ်စဉ် { $day }
calendar-repeat-weekdays = အလုပ်ရက်တိုင်း (တနင်္လာမှ သောကြာအထိ)
calendar-repeat-custom = စိတ်ကြိုက်
calendar-reminder-none = အကြောင်းကြားချက် မရှိ
calendar-reminder-at-start = စတင်ချိန်တွင်
calendar-reminder-minutes =
    { $count ->
       *[other] { $count } မိနစ် အလို
    }
calendar-reminder-hours =
    { $count ->
       *[other] { $count } နာရီ အလို
    }
calendar-reminder-days =
    { $count ->
       *[other] { $count } ရက် အလို
    }
calendar-scope-edit-title = ထပ်ခါထပ်ခါ ဖြစ်ရပ်ကို တည်းဖြတ်ရန်
calendar-scope-delete-title = ထပ်ခါထပ်ခါ ဖြစ်ရပ်ကို ဖျက်ရန်
calendar-scope-this = ဤဖြစ်ရပ်
calendar-scope-following = ဤဖြစ်ရပ်နှင့် နောက်ဖြစ်ရပ်များ
calendar-scope-all = ဖြစ်ရပ်အားလုံး
calendar-scope-respond-title = ထပ်ခါထပ်ခါ ဖြစ်ရပ်အတွက် ဖြေကြားရန်
calendar-going = သွားမလား။
calendar-answer-yes = ဟုတ်ကဲ့
calendar-answer-no = မဟုတ်ပါ
calendar-answer-maybe = ဖြစ်နိုင်
calendar-answered-yes = သင် သွားမည်
calendar-answered-no = သင် မသွားပါ
calendar-answered-maybe = သင် သွားချင်သွားနိုင်သည်

## The card at the top of a mail with an invitation.

calendar-invite = ဖိတ်ကြားချက်
calendar-invite-cancelled = ဖြစ်ရပ်ကို ပယ်ဖျက်ထားသည်
calendar-invite-reply = { $name } ပြန်ကြားပြီး
calendar-invite-reply-yes = { $name } လက်ခံပြီး
calendar-invite-reply-no = { $name } ငြင်းပယ်ပြီး
calendar-invite-reply-maybe = { $name } သွားချင်သွားနိုင်သည်
calendar-invite-organizer = { $name } က စီစဉ်သည်
calendar-invite-open = ပြက္ခဒိန်တွင် ဖွင့်ရန်
calendar-invite-not-yet = သင့်ပြက္ခဒိန်တွင် မရှိသေးပါ။ စင့်ခ်လုပ်ပြီးမှ ပြန်ကြားနိုင်ပါမည်။
calendar-invite-by-mail = သင့်ပြက္ခဒိန်တွင် မရှိပါ− သင့်အဖြေကို စီစဉ်သူထံ မေးလ်ဖြင့် ပို့ပါမည်။
calendar-mail-yes = လက်ခံပြီး− { $title }
calendar-mail-yes-body = { $name } ဤဖိတ်ကြားချက်ကို လက်ခံပြီးပါပြီ။
calendar-mail-no = ငြင်းပယ်ပြီး− { $title }
calendar-mail-no-body = { $name } ဤဖိတ်ကြားချက်ကို ငြင်းပယ်ပြီးပါပြီ။
calendar-mail-maybe = ယာယီလက်ခံ− { $title }
calendar-mail-maybe-body = { $name } ဤဖိတ်ကြားချက်ကို ယာယီလက်ခံထားပါသည်။
calendar-invite-your-day = သင့်နေ့
calendar-invite-clashes =
    { $count ->
       *[other] ဖြစ်ရပ် { $count } ခုနှင့် ထပ်နေသည်
    }

## The day's agenda beside the mail.

agenda-show = ယနေ့အစီအစဉ်ကို ပြရန်
agenda-hide = အစီအစဉ်ကို ဖျောက်ရန်
agenda-today = ယနေ့၊ { $date }
agenda-day = { $weekday }၊ { $date }
agenda-empty = ဤရက်တွင် စီစဉ်ထားသည် မရှိပါ။
