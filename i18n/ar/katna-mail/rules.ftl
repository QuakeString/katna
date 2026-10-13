# Katna Mail, Arabic (العربية).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Folders & rules

settings-rules = القواعد
settings-rules-summary = فرز البريد الجديد أو تصنيفه أو إعادة توجيهه أو إسكاته تلقائيًا
settings-rules-intro = تفرز القواعد البريد الجديد تلقائيًا، بهذا الترتيب. اسحب لإعادة الترتيب.
settings-rules-all-accounts = كل الحسابات
settings-rules-new = قاعدة جديدة
settings-rules-none = لا توجد قواعد بعد. تفرز القاعدة البريد الجديد تلقائيًا: حسب المُرسِل أو الموضوع أو الكلمات.
settings-rules-none-account = لا توجد قواعد لهذا الحساب بعد.
settings-rules-drag = اسحب لإعادة الترتيب
settings-rules-edit = تعديل القاعدة
settings-rules-turn-off = إيقاف هذه القاعدة
settings-rules-turn-on = تشغيل هذه القاعدة

## Starter rules: offered under the user's own rules, switched off.

## Turning one on makes it one of the user's rules.

settings-rules-starters = قواعد جاهزة
settings-rules-starters-intro = متوقفة حتى تشغّل إحداها. تعمل مع كل حساباتك؛ عدّل أيًّا منها لتغييرها.
settings-rules-starter-turning-on = جارٍ تشغيل «{ $name }»…
settings-rules-starter-failed = تعذّر تشغيل «{ $name }»: { $error }
rules-starter-promotions = إسكات العروض الترويجية
rules-starter-newsletters = النشرات الإخبارية إلى «للقراءة»
rules-starter-receipts = الإيصالات والفواتير
rules-starter-deliveries = الشحنات
rules-starter-train = تذاكر القطار
rules-starter-flight = تذاكر الطيران
rules-starter-codes = رموز لمرة واحدة
rules-starter-security = تنبيهات الأمان
rules-starter-social = بريد الشبكات الاجتماعية
rules-starter-invites = دعوات التقويم
rules-starter-folder-reading = للقراءة
rules-starter-folder-receipts = الإيصالات
rules-starter-folder-deliveries = الشحنات
rules-starter-folder-travel = السفر
rules-starter-folder-social = الشبكات الاجتماعية
rules-runs-katna = تعمل في Katna
rules-runs-gmail = تعمل على Gmail
rules-runs-sieve = تعمل على الخادم
rules-stopped = متوقفة
rules-error-folder-gone = المجلد الذي تستخدمه هذه القاعدة لم يعد موجودًا. عدّل القاعدة لاختيار مجلد آخر.
rules-error-no-archive = لا يوجد مجلد أرشيف في هذا الحساب. عدّل القاعدة لتفعل شيئًا آخر.
rules-error-no-trash = لا يوجد مجلد مهملات في هذا الحساب. عدّل القاعدة لتفعل شيئًا آخر.
rules-error-cannot-send = لا يستطيع هذا الحساب إرسال البريد، لذا لا تستطيع القاعدة إعادة توجيهه.
rules-error-other = { $error }. عدّل القاعدة وأعد تشغيلها.
settings-folders = المجلدات
settings-folders-summary = أعداد غير المقروءة في جزء المجلدات
settings-folders-unread-counts = عدد غير المقروءة على كل مجلد
settings-folders-unread-counts-detail = متوقف: يُظهر البريد الوارد وحده عدد غير المقروءة

## A rule in one line, on its row: "From contains substack.com → skip the

## inbox, label Reading".

rules-summary = { $when } ← { $then }
rules-summary-and = { $first } و{ $next }
rules-summary-or = { $first } أو { $next }
rules-summary-more = { $count } أخرى
rules-summary-list = { $first }، { $next }
rules-summary-condition = { $field } { $comparator } { $value }
rules-summary-has-attachment = بها مرفق
rules-summary-no-attachment = بلا مرفق
rules-summary-mailing-list = من قائمة بريدية
rules-summary-not-mailing-list = ليست من قائمة بريدية
rules-summary-tab = في علامة التبويب { $tab }
rules-summary-not-tab = ليست في علامة التبويب { $tab }
rules-summary-move = النقل إلى { $folder }
rules-summary-archive = تخطي البريد الوارد
rules-summary-trash = النقل إلى المهملات
rules-summary-mark-read = وضع علامة «مقروءة»
rules-summary-star = تمييز بنجمة
rules-summary-important = وضع علامة «مهمة»
rules-summary-label = التصنيف { $label }
rules-summary-forward = إعادة التوجيه إلى { $address }
rules-summary-dont-notify = عدم الإشعار
rules-summary-read-after = { $count ->
    [zero] وضع علامة «مقروءة» فورًا
    [one] وضع علامة «مقروءة» بعد يوم واحد
    [two] وضع علامة «مقروءة» بعد يومين
    [few] وضع علامة «مقروءة» بعد { $count } أيام
    [many] وضع علامة «مقروءة» بعد { $count } يومًا
   *[other] وضع علامة «مقروءة» بعد { $count } يوم
}
rules-summary-folder-gone = مجلد لم يعد موجودًا

## The rule editor

rules-editor-new-title = قاعدة جديدة
rules-editor-edit-title = تعديل القاعدة
rules-editor-name-hint = اسم القاعدة
rules-editor-when = عندما تطابق رسالة جديدة
rules-editor-of-these = من هذه:
rules-mode-all = الكل
rules-mode-any = أيًّا
rules-field-from = من
rules-field-to = إلى
rules-field-cc = نسخة
rules-field-any-recipient = إلى أو نسخة
rules-field-reply-to = الرد إلى
rules-field-subject = الموضوع
rules-field-body = النص
rules-field-attachment-name = اسم المرفق
rules-field-has-attachment = بها مرفق
rules-field-mailing-list = من قائمة بريدية
rules-field-tab = علامة تبويب البريد الوارد
rules-comparator-contains = يحتوي على
rules-comparator-not-contains = لا يحتوي على
rules-comparator-begins-with = يبدأ بـ
rules-comparator-ends-with = ينتهي بـ
rules-comparator-equals = يساوي تمامًا
rules-comparator-matches = يطابق النمط
rules-has-yes = نعم
rules-has-no = لا
rules-editor-value-hint = كلمات أو عنوان
rules-editor-add-condition = إضافة شرط
rules-editor-remove = إزالة
rules-editor-then = ثم:
rules-action-move = نقل إلى
rules-action-archive = تخطي البريد الوارد (أرشفة)
rules-action-trash = النقل إلى المهملات
rules-action-mark-read = وضع علامة «مقروءة»
rules-action-star = تمييز بنجمة
rules-action-important = وضع علامة «مهمة»
rules-action-label = إضافة تصنيف
rules-action-forward = إعادة توجيه إلى
rules-action-dont-notify = عدم الإشعار
rules-action-read-after = وضع علامة «مقروءة» بعد
rules-editor-choose-folder = اختيار مجلد
rules-editor-choose-label = اختيار تصنيف
rules-editor-new-folder = جديد: { $name }
rules-editor-folder-of = { $folder } ({ $account })
rules-editor-forward-hint = عنوان البريد الإلكتروني
rules-editor-days = أيام
rules-editor-add-action = إضافة إجراء
rules-editor-stop = التوقف هنا: لا تُطبَّق القواعد اللاحقة على هذه الرسالة
rules-editor-accounts = الحسابات:
rules-editor-accounts-none = اختيار الحسابات
rules-editor-accounts-many = { $count ->
    [zero] لا حسابات
    [one] حساب واحد
    [two] حسابان
    [few] { $count } حسابات
    [many] { $count } حسابًا
   *[other] { $count } حساب
}
rules-editor-matches = تطابق { $mails } من آخر { $days } يوم
rules-editor-mails = { $count ->
    [zero] { $count } رسالة
    [one] رسالة واحدة
    [two] رسالتين
    [few] { $count } رسائل
    [many] { $count } رسالة
   *[other] { $count } رسالة
}
rules-editor-counting = جارٍ عدّ الرسائل المطابقة…
rules-editor-show = إظهارها
rules-editor-also-apply = تطبيقها أيضًا على هذه الرسائل ({ $count })
rules-editor-runs-katna = تعمل في Katna، ما دام هذا الكمبيوتر يعمل.
rules-editor-runs-gmail = تعمل على Gmail، لذا تعمل أيضًا على هاتفك وعندما يكون هذا الكمبيوتر مطفأً.
rules-editor-runs-sieve = تعمل على خادم بريدك، لذا تعمل أيضًا على هاتفك وعندما يكون هذا الكمبيوتر مطفأً.
rules-note-gmail-action = تعمل في Katna: لا تستطيع عوامل تصفية Gmail تنفيذ «{ $action }».
rules-note-sieve-action = تعمل في Katna: لا تستطيع قواعد خادم بريدك تنفيذ «{ $action }».
rules-note-test = { $field } { $comparator }
rules-note-gmail-condition = تعمل في Katna: لا تستطيع عوامل تصفية Gmail اختبار «{ $test }» كما يفعل Katna.
rules-note-sieve-condition = تعمل في Katna: لا تستطيع قواعد خادم بريدك اختبار «{ $test }» كما يفعل Katna.
rules-note-order = تعمل في Katna، كقاعدة سابقة في الحساب: تُطبَّق القواعد بترتيب القائمة.
rules-note-gmail-stop = تعمل في Katna: لا تستطيع عوامل تصفية Gmail منع تطبيق القواعد اللاحقة.
rules-note-gmail-forward = تعمل في Katna: لا يعيد Gmail التوجيه إلا إلى العناوين المُتحقَّق منها في إعداداته، و{ $address } ليس منها.
rules-note-gmail-folder = تعمل في Katna: لا يوجد في Gmail تصنيف لمجلد تستخدمه هذه القاعدة.
rules-note-sieve-folder = تعمل في Katna: لا يوجد على خادم بريدك مجلد تستخدمه هذه القاعدة.
rules-note-gmail-sign-in = تعمل في Katna حتى تسجّل الدخول إلى Google مجددًا وتسمح لـ Katna بإنشاء عوامل تصفية Gmail.
rules-note-sieve-other-script = تعمل في Katna: هناك برنامج قواعد آخر («{ $name }») نشط على خادم بريدك.
rules-note-gmail-failed = تعمل في Katna: لم يقبلها Gmail ({ $error }).
rules-note-sieve-failed = تعمل في Katna: لم يقبلها خادم بريدك ({ $error }).
rules-editor-cancel = إلغاء
rules-editor-save = حفظ
rules-editor-saving = جارٍ الحفظ…
rules-editor-delete = حذف القاعدة
rules-editor-delete-ask = هل تريد حذف هذه القاعدة؟
rules-editor-delete-keep = الاحتفاظ بها
rules-editor-delete-confirm = حذف
rules-editor-needs-folder = اختر مجلدًا لكل «نقل إلى» وتصنيفًا لكل «إضافة تصنيف».
rules-editor-needs-days = يتطلب الإجراء «وضع علامة مقروءة بعد» عددًا من الأيام، من 1 إلى 3650.
rules-saved = تم حفظ القاعدة
rules-saved-applied = { $count ->
    [zero] تم حفظ القاعدة
    [one] تم حفظ القاعدة وتطبيقها على رسالة واحدة
    [two] تم حفظ القاعدة وتطبيقها على رسالتين
    [few] تم حفظ القاعدة وتطبيقها على { $count } رسائل
    [many] تم حفظ القاعدة وتطبيقها على { $count } رسالة
   *[other] تم حفظ القاعدة وتطبيقها على { $count } رسالة
}
rules-apply-failed = تم حفظ القاعدة، لكن فشل تطبيقها: { $error }
rules-deleted = تم حذف القاعدة
rules-delete-failed = تعذّر حذف القاعدة: { $error }
rules-change-failed = تعذّر تغيير القواعد: { $error }
