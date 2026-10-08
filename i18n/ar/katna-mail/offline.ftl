# Katna Mail, Arabic (العربية).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Taking an account offline and back

offline-go-offline = قطع الاتصال
offline-for-hour = لمدة ساعة واحدة
offline-until-tomorrow = حتى الغد
offline-until-online = حتى أعيد تشغيله
offline-go-online = الاتصال
offline-work-offline = العمل بلا اتصال

## How an offline account shows

offline-state = غير متصل
offline-until = غير متصل حتى { $time }
offline-until-day = غير متصل حتى { $day } { $time }
offline-waiting = { $state } · { $count ->
    [zero] لا شيء منتظر
    [one] { $count } منتظرة
    [two] { $count } منتظرتان
    [few] { $count } منتظرة
    [many] { $count } منتظرة
   *[other] { $count } منتظرة
}
offline-click-online = انقر للاتصال
offline-account-tip = { $account } غير متصل
offline-tag = غير متصل
offline-compose = { $account } غير متصل. تنتظر هذه الرسالة في صندوق الصادر وتُرسَل عند عودة الحساب للاتصال.

## Settings > Accounts

offline-settings-row = متصل
offline-settings-detail = أوقف حسابًا ليتوقف Katna عن الاتصال به. يبقى بريده هنا لتقرأه، وما تفعله في هذه الأثناء يُرسَل عند إعادة تشغيله.
