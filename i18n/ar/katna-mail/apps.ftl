# Katna Mail, Arabic (العربية).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## App rail (and the bottom bar on a phone)

rail-mail = البريد
rail-calendar = التقويم
rail-contacts = جهات الاتصال
rail-tasks = المهام
rail-notes = الملاحظات
rail-files = الملفات
rail-menu-open = فتح { $app }
rail-menu-settings = إعدادات { $app }
rail-menu-turn-off = إيقاف { $app }…
app-off-title = هل تريد إيقاف { $app }؟
app-off-body = يتوقف Katna عن مزامنة { $app } ويزيله من:
app-off-keep = الاحتفاظ بنسخة على هذا الكمبيوتر
app-off-keep-detail = إعادة تشغيله فورية
app-off-remove = إزالة النسخة الموجودة على هذا الكمبيوتر
app-off-remove-detail = لا يتغير شيء في حساباتك، وعند إعادة تشغيله يُنزَّل مجددًا. يبقى ما هو موجود على هذا الكمبيوتر فقط أو ما لم يُرسَل بعد.
app-off-cancel = إلغاء
app-off-confirm = إيقاف
app-off-done = تم إيقاف { $app }
app-off-note = { $app } متوقف
app-off-turn-on = تشغيل
app-off-leaves-calendar-rail = الشريط الجانبي وCtrl+2
app-off-leaves-calendar-agenda = جدول الأعمال بجانب بريدك
app-off-leaves-calendar-meeting = جدولة اجتماع، و«الفتح في التقويم» في الدعوات
app-off-leaves-calendar-reminders = تذكيرات الأحداث
app-off-leaves-calendar-desktop = الأحداث في KRunner وساعة سطح المكتب
app-off-leaves-contacts-rail = الشريط الجانبي وCtrl+3
app-off-leaves-contacts-card = «إضافة إلى جهات الاتصال» في بطاقة المُرسِل
app-off-leaves-contacts-birthdays = أعياد الميلاد في التقويم
app-off-leaves-tasks-rail = الشريط الجانبي وCtrl+4
app-off-leaves-tasks-mail = «إضافة إلى المهام» في البريد، وShift+T
app-off-leaves-tasks-calendar = المهام في التقويم
app-off-leaves-tasks-tray = «مهمة جديدة» في علبة النظام، وMeta+Alt+T
app-off-leaves-tasks-reminders = تذكيرات المهام
app-off-leaves-notes-rail = الشريط الجانبي وCtrl+5
app-off-leaves-notes-mail = «إضافة ملاحظة» في البريد
app-off-leaves-notes-meetings = ملاحظات الاجتماعات في الأحداث
app-off-leaves-notes-tray = «ملاحظة جديدة» في علبة النظام، وMeta+Alt+N
app-off-leaves-notes-reminders = تذكيرات الملاحظات
app-off-leaves-files-rail = الشريط الجانبي وCtrl+7
app-off-leaves-files-compose = الملفات عند الإرفاق في نافذة الإنشاء

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = قريبًا
app-calendar-promise = تقاويم CalDAV ودعوات الاجتماعات الواردة في بريدك والتذكيرات، بجانب بريدك الوارد.
app-tasks-promise = قوائم مهام تتم مزامنتها مع CalDAV، ومهام تُنشأ من البريد.
app-notes-promise = ملاحظات سريعة، وملاحظات على رسالة أو محادثة للرجوع إليها لاحقًا.

## Contacts page

app-contacts-loading = جارٍ جمع الأشخاص من بريدك…
app-contacts-empty = يظهر هنا الأشخاص الذين تراسلهم.
app-contacts-count = { $count ->
    [zero] { $count } شخص من بريدك، الأكثر مراسلةً أولًا
    [one] شخص واحد من بريدك، الأكثر مراسلةً أولًا
    [two] شخصان من بريدك، الأكثر مراسلةً أولًا
    [few] { $count } أشخاص من بريدك، الأكثر مراسلةً أولًا
    [many] { $count } شخصًا من بريدك، الأكثر مراسلةً أولًا
   *[other] { $count } شخص من بريدك، الأكثر مراسلةً أولًا
}
app-contacts-top = { $count ->
    [zero] أبرز { $count } شخص من بريدك، الأكثر مراسلةً أولًا
    [one] أبرز شخص من بريدك، الأكثر مراسلةً أولًا
    [two] أبرز شخصين من بريدك، الأكثر مراسلةً أولًا
    [few] أبرز { $count } أشخاص من بريدك، الأكثر مراسلةً أولًا
    [many] أبرز { $count } شخصًا من بريدك، الأكثر مراسلةً أولًا
   *[other] أبرز { $count } شخص من بريدك، الأكثر مراسلةً أولًا
}
app-contacts-messages = { $count ->
    [zero] { $count } رسالة
    [one] رسالة واحدة
    [two] رسالتان
    [few] { $count } رسائل
    [many] { $count } رسالة
   *[other] { $count } رسالة
}
app-contacts-last = آخر مراسلة { $date }
top-brand = Katna
