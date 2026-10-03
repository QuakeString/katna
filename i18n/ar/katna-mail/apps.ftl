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
