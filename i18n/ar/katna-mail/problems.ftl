# Katna Mail, Arabic (العربية).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = خادم البريد
problems-signed-out = سجّل { $provider } خروج Katna من { $address }. توقفت مزامنة البريد.
problems-password-refused = رفض { $provider } كلمة المرور لـ { $address }. ربما تغيّرت.
problems-no-answer = لا يستجيب { $provider } لـ { $address }. يواصل Katna المحاولة.
problems-offline = أنت غير متصل. لا يزال بريدك هنا، والبريد الذي ترسله ينتظر حتى تعود للاتصال.
problems-accounts-need-you = { $count ->
    [zero] لا حسابات تحتاج إليك
    [one] حساب واحد يحتاج إليك
    [two] حسابان يحتاجان إليك
    [few] { $count } حسابات تحتاج إليك
    [many] { $count } حسابًا تحتاج إليك
   *[other] { $count } حساب تحتاج إليك
}
problems-show = إظهار
problems-later = لاحقًا
problems-new-password = كلمة مرور جديدة
problems-try-again = إعادة المحاولة

## The New password card

problems-password-title = كلمة مرور جديدة
problems-password-detail = رفض { $provider } كلمة المرور المحفوظة لـ { $address }. اكتب الجديدة؛ يتحقق منها Katna قبل الاحتفاظ بها.
problems-password-placeholder = كلمة المرور
problems-password-show = إظهار كلمة المرور
problems-password-hide = إخفاء كلمة المرور
problems-password-cancel = إلغاء
problems-password-save = حفظ
problems-password-checking = جارٍ التحقق…
problems-password-refused-again = رفض { $provider } كلمة المرور هذه أيضًا. تحقق منها وحاول مجددًا.
problems-password-saved = تم حفظ كلمة المرور لـ { $address }. جارٍ جلب بريدك…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = لم يقبل خادم بريد { $address } نقل { $count ->
    [zero] أي رسالة.
    [one] رسالة، لذا أُعيدت إلى مكانها.
    [two] رسالتين، لذا أُعيدتا إلى مكانهما.
    [few] { $count } رسائل، لذا أُعيدت إلى مكانها.
    [many] { $count } رسالة، لذا أُعيدت إلى مكانها.
   *[other] { $count } رسالة، لذا أُعيدت إلى مكانها.
}
problems-refused-flags = لم يقبل خادم بريد { $address } وضع علامة على { $count ->
    [zero] أي رسالة.
    [one] رسالة (مقروءة، مميّزة بنجمة…)، لذا أُعيدت كما كانت.
    [two] رسالتين (مقروءة، مميّزة بنجمة…)، لذا أُعيدتا كما كانتا.
    [few] { $count } رسائل (مقروءة، مميّزة بنجمة…)، لذا أُعيدت كما كانت.
    [many] { $count } رسالة (مقروءة، مميّزة بنجمة…)، لذا أُعيدت كما كانت.
   *[other] { $count } رسالة (مقروءة، مميّزة بنجمة…)، لذا أُعيدت كما كانت.
}
problems-refused-label = لم يقبل خادم بريد { $address } تغيير تصنيفات { $count ->
    [zero] أي رسالة.
    [one] رسالة، لذا أُعيدت كما كانت.
    [two] رسالتين، لذا أُعيدتا كما كانتا.
    [few] { $count } رسائل، لذا أُعيدت كما كانت.
    [many] { $count } رسالة، لذا أُعيدت كما كانت.
   *[other] { $count } رسالة، لذا أُعيدت كما كانت.
}
problems-refused-delete = لم يقبل خادم بريد { $address } حذف { $count ->
    [zero] أي رسالة.
    [one] رسالة، لذا أُعيدت.
    [two] رسالتين، لذا أُعيدتا.
    [few] { $count } رسائل، لذا أُعيدت.
    [many] { $count } رسالة، لذا أُعيدت.
   *[other] { $count } رسالة، لذا أُعيدت.
}
problems-refused-other = لم يقبل خادم بريد { $address } { $count ->
    [zero] أي تغيير.
    [one] تغييرًا، لذا أعاده Katna كما كان.
    [two] تغييرين، لذا أعادهما Katna كما كانا.
    [few] { $count } تغييرات، لذا أعادها Katna كما كانت.
    [many] { $count } تغييرًا، لذا أعادها Katna كما كانت.
   *[other] { $count } تغيير، لذا أعادها Katna كما كانت.
}
problems-details = التفاصيل

## Katna's background service (katna-daemon) isn't running

service-starting = جارٍ بدء خدمة Katna في الخلفية…
service-failed = لا تبدأ خدمة Katna في الخلفية، لذا لا تتم مزامنة البريد.
service-start-again = البدء مجددًا
service-started-again = توقفت خدمة Katna في الخلفية وأُعيد تشغيلها.
service-details-title = لماذا لا تبدأ الخدمة
service-details-body = انسخ هذا وأرسله مع بلاغك. لا يحتوي على أي بريد أو كلمات مرور.
service-details-copy = نسخ
service-details-close = إغلاق
