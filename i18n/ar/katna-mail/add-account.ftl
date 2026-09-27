# Katna Mail, Arabic (العربية).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = إضافة حساب بريد
add-account-looking = جارٍ البحث عن خوادم البريد لـ { $address }…
add-account-address-intro = أدخل عنوان بريدك الإلكتروني. سيجد Katna الخوادم نيابةً عنك.
add-account-servers-title = إعدادات الخادم
add-account-servers-intro = المكان الذي يقرأ منه Katna بريد { $address } ويرسله.
add-account-password-title = أدخل كلمة المرور
add-account-signing-in = جارٍ تسجيل الدخول…

## Add a mail account: fields

add-account-field-address = عنوان البريد الإلكتروني
add-account-incoming = البريد الوارد ({ $protocol })
add-account-outgoing = البريد الصادر ({ $protocol })
add-account-field-server = الخادم
add-account-field-port = المنفذ
add-account-security-none = بلا
add-account-field-username = اسم المستخدم
add-account-field-password = كلمة المرور
add-account-show-password = إظهار كلمة المرور
add-account-app-password-hint = يحتاج { $provider } هنا إلى كلمة مرور للتطبيقات، لا كلمة المرور التي تستخدمها على الويب. أنشئ واحدة من إعدادات الأمان في حسابك على { $provider }.
add-account-field-name = اسمك (اختياري)
add-account-name-hint = يظهر للأشخاص الذين تراسلهم.
add-account-servers-pair = { $imap } و{ $smtp }
add-account-servers-found = { $source ->
    [built-in] الخوادم: { $servers }، وُجدت في قائمة المزوّدين لدى Katna.
    [provider] الخوادم: { $servers }، وُجدت في إعدادات مزوّدك.
    [ispdb] الخوادم: { $servers }، وُجدت في قائمة المزوّدين لدى Thunderbird.
    [dns] الخوادم: { $servers }، وُجدت في سجلات DNS لنطاقك.
   *[other] الخوادم: { $servers }، مُخمَّنة؛ تحقّق منها إن فشل تسجيل الدخول.
}
add-account-servers-entered = الخوادم: { $servers }، كما أُدخلت.

## Add a mail account: buttons

add-account-servers-button = إعدادات الخادم
add-account-back = رجوع
add-account-add = إضافة الحساب
add-account-next = التالي
add-account-cancel = إلغاء

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] أدخل خادم البريد الوارد.
   *[outgoing] أدخل خادم البريد الصادر.
}
add-account-server-space = { $kind ->
    [incoming] يحتوي اسم خادم البريد الوارد على مسافة.
   *[outgoing] يحتوي اسم خادم البريد الصادر على مسافة.
}
add-account-port-invalid = { $kind ->
    [incoming] يجب أن يكون منفذ البريد الوارد رقمًا من { $min } إلى { $max }.
   *[outgoing] يجب أن يكون منفذ البريد الصادر رقمًا من { $min } إلى { $max }.
}
add-account-address-empty = أدخل عنوان بريد إلكتروني.
add-account-address-invalid = أدخل عنوان بريد إلكتروني مثل { $example }.
add-account-not-found = تعذّر على Katna العثور على خوادم { $address }، فملأ الأسماء المعتادة. تحقّق منها لدى مزوّدك.
add-account-password-empty = أدخل كلمة المرور.
add-account-name-is-password = الاسم هو نفسه كلمة المرور. اكتب اسمك هناك بدلًا منها، كما ينبغي أن يراه الناس.
add-account-added = تمت إضافة { $address }. جارٍ جلب بريدك…
add-account-app-password-refused = رفض { $provider } كلمة المرور. يحتاج إلى كلمة مرور للتطبيقات، لا كلمة المرور التي تستخدمها على الويب.
add-account-password-refused = رفض الخادم كلمة المرور. تحقّق منها وحاول مجددًا.

## The account menu (from the account button on the top bar)

add-account-menu-another = إضافة حساب آخر
add-account-menu-manage = إدارة الحسابات
