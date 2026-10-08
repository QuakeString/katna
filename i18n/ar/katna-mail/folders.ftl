# Katna Mail, Arabic (العربية).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Navigation (the folders pane)

nav-labels = التصنيفات
nav-folders = المجلدات
nav-label-new = إنشاء تصنيف جديد
nav-folder-new = إنشاء مجلد جديد
nav-menu-check-mail = التحقق من وجود بريد جديد
nav-menu-check-inbox = التحقق من هذا البريد الوارد
nav-unified-leave-out = استبعاد من البريد الوارد الموحّد
nav-unified-bring-back = إعادة إلى البريد الوارد الموحّد
nav-menu-sign-in-again = تسجيل الدخول مجددًا
nav-menu-new-mail = رسالة جديدة من هذا الحساب
nav-menu-account-settings = إعدادات الحساب
nav-account-checked = متزامن · آخر تحقق { $ago }
nav-account-in-sync = متزامن
nav-account-connecting = جارٍ الاتصال…
nav-account-offline = غير متصل، تجري إعادة المحاولة
nav-account-signed-out = انتهت صلاحية تسجيل الدخول إلى { $provider }
nav-account-password-refused = تم رفض كلمة المرور
nav-account-storage = مُستخدَم { $used } من { $total }
nav-menu-new-subfolder = مجلد جديد بداخله
nav-menu-new-sublabel = تصنيف جديد بداخله
nav-menu-rename = إعادة التسمية
nav-menu-delete = حذف
nav-menu-empty-trash = إفراغ المهملات
nav-account-unnamed = الحساب { $number }
nav-all-accounts = كل الحسابات
nav-expand = إظهار المجلدات
nav-collapse = إخفاء المجلدات
storage-used = مُستخدَم { $percent }٪ من { $total }
storage-used-detail = { $address }: مُستخدَم { $used } من { $total }

## Special folders (the user's own folders keep their names)

folder-inbox = البريد الوارد
folder-starred = المميّزة بنجمة
folder-snoozed = المؤجَّلة
folder-unread = غير المقروءة
folder-important = المهمة
folder-drafts = المسودات
folder-sent = المُرسَلة
folder-archive = الأرشيف
folder-spam = الرسائل غير المرغوب فيها
folder-trash = المهملات
folder-all-mail = كل البريد
folder-scheduled = المُجدوَلة
folder-waiting = بانتظار الرد
folder-waiting-short = بانتظار الرد
folder-reminders = التذكيرات
folder-outbox = صندوق الصادر
folder-activity = النشاط

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = تصنيف جديد
label-folder-new-title = مجلد جديد
label-prompt = يُرجى إدخال اسم التصنيف الجديد:
label-folder-prompt = يُرجى إدخال اسم المجلد الجديد:
label-name-hint = اسم التصنيف
label-folder-name-hint = اسم المجلد
label-nest = تضمين التصنيف ضمن:
label-folder-nest = تضمين المجلد ضمن:
label-cancel = إلغاء
label-create = إنشاء
label-creating = جارٍ الإنشاء…
label-created = تم إنشاء التصنيف «{ $name }».
label-folder-created = تم إنشاء المجلد «{ $name }».
label-rename-title = إعادة تسمية التصنيف
label-folder-rename-title = إعادة تسمية المجلد
label-rename = إعادة التسمية
label-renaming = جارٍ إعادة التسمية…
label-renamed = تمت إعادة تسمية التصنيف إلى «{ $name }».
label-folder-renamed = تمت إعادة تسمية المجلد إلى «{ $name }».
folder-delete-title = هل تريد حذف «{ $name }»؟
folder-delete-body = { $count ->
    [0] لا يحتوي على أي بريد. سيُزال المجلد من الخادم، لذا سيختفي أيضًا من بريد الويب ومن هاتفك.
   *[other] { $kind ->
        [conversation] { $count ->
            [zero] لا توجد فيه محادثات.
            [one] تنتقل محادثته الوحيدة إلى المهملات، لذا يمكنك استعادتها.
            [two] تنتقل محادثتاه إلى المهملات، لذا يمكنك استعادتهما.
            [few] تنتقل محادثاته الـ{ $count } إلى المهملات، لذا يمكنك استعادتها.
            [many] تنتقل محادثاته البالغ عددها { $count } محادثة إلى المهملات، لذا يمكنك استعادتها.
           *[other] تنتقل محادثاته البالغ عددها { $count } محادثة إلى المهملات، لذا يمكنك استعادتها.
        }
       *[message] { $count ->
            [zero] لا توجد فيه رسائل.
            [one] تنتقل رسالته الوحيدة إلى المهملات، لذا يمكنك استعادتها.
            [two] تنتقل رسالتاه إلى المهملات، لذا يمكنك استعادتهما.
            [few] تنتقل رسائله الـ{ $count } إلى المهملات، لذا يمكنك استعادتها.
            [many] تنتقل رسائله البالغ عددها { $count } رسالة إلى المهملات، لذا يمكنك استعادتها.
           *[other] تنتقل رسائله البالغ عددها { $count } رسالة إلى المهملات، لذا يمكنك استعادتها.
        }
    } سيُزال المجلد من الخادم، لذا سيختفي أيضًا من بريد الويب ومن هاتفك.
}
folder-delete-forever-body = { $count ->
    [0] لا يحتوي على أي بريد. سيُزال المجلد من الخادم، لذا سيختفي أيضًا من بريد الويب ومن هاتفك.
   *[other] { $kind ->
        [conversation] { $count ->
            [zero] لا توجد فيه محادثات.
            [one] ستُحذف محادثته الوحيدة نهائيًا؛ فلا توجد مهملات في هذا الحساب.
            [two] ستُحذف محادثتاه نهائيًا؛ فلا توجد مهملات في هذا الحساب.
            [few] ستُحذف محادثاته الـ{ $count } نهائيًا؛ فلا توجد مهملات في هذا الحساب.
            [many] ستُحذف محادثاته البالغ عددها { $count } محادثة نهائيًا؛ فلا توجد مهملات في هذا الحساب.
           *[other] ستُحذف محادثاته البالغ عددها { $count } محادثة نهائيًا؛ فلا توجد مهملات في هذا الحساب.
        }
       *[message] { $count ->
            [zero] لا توجد فيه رسائل.
            [one] ستُحذف رسالته الوحيدة نهائيًا؛ فلا توجد مهملات في هذا الحساب.
            [two] ستُحذف رسالتاه نهائيًا؛ فلا توجد مهملات في هذا الحساب.
            [few] ستُحذف رسائله الـ{ $count } نهائيًا؛ فلا توجد مهملات في هذا الحساب.
            [many] ستُحذف رسائله البالغ عددها { $count } رسالة نهائيًا؛ فلا توجد مهملات في هذا الحساب.
           *[other] ستُحذف رسائله البالغ عددها { $count } رسالة نهائيًا؛ فلا توجد مهملات في هذا الحساب.
        }
    } سيُزال المجلد من الخادم، لذا سيختفي أيضًا من بريد الويب ومن هاتفك.
}
folder-delete-label-body = سيُزال التصنيف. يبقى بريده في «كل البريد» وفي تصنيفاته الأخرى.
folder-delete-confirm = حذف المجلد
folder-delete-label-confirm = حذف التصنيف
folder-deleted = تم حذف المجلد «{ $name }»
label-deleted = تم حذف التصنيف «{ $name }»
