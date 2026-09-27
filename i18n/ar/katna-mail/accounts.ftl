# Katna Mail, Arabic (العربية).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Accounts

accounts-folder-pane = جزء المجلدات
accounts-folder-pane-detail = الحسابات التي يعرض الجزء الأيمن مجلداتها.
accounts-shown-one = حساب واحد في كل مرة؛ يمكنك التبديل من بطاقة الحساب
accounts-shown-all = كل الحسابات، الواحد تلو الآخر
accounts-row = الحسابات
accounts-row-detail = تؤدي إزالة حساب إلى حذف نسخة Katna من بريده على هذا الكمبيوتر. يبقى البريد على الخادم.
accounts-none = لا توجد حسابات بعد.
accounts-kind-imported = مستورَد
accounts-picture-reset = استخدام صورة سطح المكتب
accounts-picture-change = تغيير الصورة
accounts-remove = إزالة
accounts-delete-all-row = حذف كل البيانات
accounts-delete-all-row-detail = البدء من جديد، كما في تثبيت جديد.
accounts-delete-all-about = يحذف من هذا الكمبيوتر كل الحسابات، وكل البريد المخزَّن، وجهات الاتصال والتقاويم، وفهرس البحث، وإعداداتك وكلمات المرور المحفوظة. لا يتغير أي شيء على خوادم بريدك.
accounts-delete-all-open = حذف كل بيانات Katna

## Settings > Accounts: snackbars after deleting

accounts-removed-local = تمت إزالة { $address } من Katna.
accounts-removed = تمت إزالة { $address } من Katna. لا يزال بريده على الخادم.
accounts-all-deleted = تم حذف كل بيانات Katna من هذا الكمبيوتر.

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = هل تريد إزالة { $address }؟
accounts-remove-confirm = إزالة الحساب
accounts-removing = جارٍ الإزالة…
accounts-remove-local-mail = { $folders ->
    [0] كل البريد المستورد إلى هذا الحساب
    [one] كل البريد المستورد إلى هذا الحساب في مجلده
    [two] كل البريد المستورد إلى هذا الحساب في مجلديه
    [few] كل البريد المستورد إلى هذا الحساب في مجلداته البالغ عددها { $folders }
    [many] كل البريد المستورد إلى هذا الحساب في مجلداته البالغ عددها { $folders }
   *[other] كل البريد المستورد إلى هذا الحساب في مجلداته البالغ عددها { $folders }
}
accounts-remove-local-settings = إعداداته في Katna
accounts-remove-mail = { $folders ->
    [0] كل بريد هذا الحساب الذي خزّنه Katna
    [one] كل بريد هذا الحساب الذي خزّنه Katna في مجلده
    [two] كل بريد هذا الحساب الذي خزّنه Katna في مجلديه
    [few] كل بريد هذا الحساب الذي خزّنه Katna في مجلداته البالغ عددها { $folders }
    [many] كل بريد هذا الحساب الذي خزّنه Katna في مجلداته البالغ عددها { $folders }
   *[other] كل بريد هذا الحساب الذي خزّنه Katna في مجلداته البالغ عددها { $folders }
}
accounts-remove-outbox = رسائله المنتظرة في صندوق الصادر
accounts-remove-settings = كلمة المرور المحفوظة له وإعداداته في Katna
accounts-delete-all-title = هل تريد حذف كل بيانات Katna؟
accounts-delete-all-confirm = حذف كل شيء
accounts-deleting = جارٍ الحذف…
accounts-delete-all-accounts = كل الحسابات، وكل البريد والمرفقات التي خزّنها Katna
accounts-delete-all-contacts = جهات الاتصال والتقاويم وفهرس البحث
accounts-delete-all-settings = كل الإعدادات والتوقيعات واختصارات لوحة المفاتيح
accounts-delete-all-passwords = كل كلمات المرور المحفوظة
accounts-deleted-heading = سيُحذف من هذا الكمبيوتر:
accounts-cannot-undo = لا يمكن التراجع عن هذا الإجراء.
accounts-server-delete-all = لا يتغير أي شيء على خوادم بريدك: يبقى بريدك هناك، وإضافة حساب مجددًا تنزّله مرة أخرى. البريد المستورد من ملفات موجود في Katna فقط؛ ولا يتم المساس بالملفات.
accounts-server-local = تم استيراد هذا البريد من ملفات، لذا يملك Katna النسخة الوحيدة منه. لا يتم المساس بالملفات التي جاء منها؛ استوردها مجددًا لاستعادته.
accounts-server-remove = لا يتغير أي شيء على خادم البريد: يبقى بريدك هناك، وإضافة الحساب مجددًا تنزّله مرة أخرى.
accounts-confirm-word = حذف
accounts-confirm-placeholder = اكتب «{ accounts-confirm-word }»
accounts-confirm-prompt = للتأكيد، اكتب «{ accounts-confirm-word }»:
accounts-cancel = إلغاء
