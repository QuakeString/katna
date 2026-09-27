# Katna Mail, Arabic (العربية).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Language picker (top bar and Settings > General)

language-tooltip = اللغة: { $language }
language-tooltip-system = اللغة: { $language }، حسب النظام
language-search = البحث عن لغة
language-system-default = لغة النظام
language-system-now = حاليًا: { $language }
language-no-match = لا توجد لغة مطابقة لـ«{ $query }»
language-machine = ترجمة آلية. ساعِد في تحسينها
language-setting = اللغة
language-setting-detail = لغة القوائم والأزرار والرسائل، وتنسيق التواريخ والأرقام. يتّبع خيار «لغة النظام» إعدادات سطح المكتب.

## Dates and sizes

ago-just-now = الآن
ago-minutes = { $count ->
    [zero] منذ { $count } دقيقة
    [one] منذ دقيقة واحدة
    [two] منذ دقيقتين
    [few] منذ { $count } دقائق
    [many] منذ { $count } دقيقة
   *[other] منذ { $count } دقيقة
}
ago-hours = { $count ->
    [zero] منذ { $count } ساعة
    [one] منذ ساعة واحدة
    [two] منذ ساعتين
    [few] منذ { $count } ساعات
    [many] منذ { $count } ساعة
   *[other] منذ { $count } ساعة
}
ago-days = { $count ->
    [zero] منذ { $count } يوم
    [one] منذ يوم واحد
    [two] منذ يومين
    [few] منذ { $count } أيام
    [many] منذ { $count } يومًا
   *[other] منذ { $count } يوم
}
size-bytes = { $count ->
    [zero] { $count } بايت
    [one] بايت واحد
    [two] بايتان
    [few] { $count } بايتات
    [many] { $count } بايت
   *[other] { $count } بايت
}
size-kb = { $size } كيلوبايت
size-mb = { $size } ميغابايت
size-gb = { $size } غيغابايت
size-tb = { $size } تيرابايت

## Top bar

folders-hide = إخفاء المجلدات
folders-show = إظهار المجلدات
compose = إنشاء
search = بحث
search-mail = البحث في البريد
search-settings = البحث في الإعدادات
search-clear = محو البحث
search-options-show = عرض خيارات البحث
settings = الإعدادات
account-add = إضافة حساب

## App rail (and the bottom bar on a phone)

rail-mail = البريد
rail-calendar = التقويم
rail-contacts = جهات الاتصال
rail-tasks = المهام
rail-notes = الملاحظات
rail-feeds = الخلاصات

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = قريبًا
app-calendar-promise = تقاويم CalDAV ودعوات الاجتماعات الواردة في بريدك والتذكيرات، بجانب بريدك الوارد.
app-tasks-promise = قوائم مهام تتم مزامنتها مع CalDAV، ومهام تُنشأ من البريد.
app-notes-promise = ملاحظات سريعة، وملاحظات على رسالة أو محادثة للرجوع إليها لاحقًا.
app-feeds-promise = اقرأ خلاصات RSS وAtom بجانب بريدك.

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

## Navigation (the folders pane)

nav-labels = التصنيفات
nav-folders = المجلدات
nav-label-new = إنشاء تصنيف جديد
nav-folder-new = إنشاء مجلد جديد
nav-account-unnamed = الحساب { $number }
nav-tab-new = { $count ->
    [zero] { $count } جديدة
    [one] { $count } جديدة
    [two] { $count } جديدة
    [few] { $count } جديدة
    [many] { $count } جديدة
   *[other] { $count } جديدة
}

## Special folders (the user's own folders keep their names)

folder-inbox = البريد الوارد
folder-starred = المميّزة بنجمة
folder-drafts = المسودات
folder-sent = المُرسَلة
folder-archive = الأرشيف
folder-spam = الرسائل غير المرغوب فيها
folder-trash = المهملات
folder-all-mail = كل البريد
folder-scheduled = المُجدوَلة

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

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = الأساسية
tab-promotions = العروض الترويجية
tab-social = الشبكات الاجتماعية
tab-updates = التحديثات
tab-forums = المنتديات
tab-focused = المُركَّز
tab-other = أخرى
tab-inbox = البريد الوارد
tab-newsletters = النشرات الإخبارية
tab-notifications = الإشعارات
tab-new = { $count } جديدة
tab-provider-other = يفرزها Katna

## Mail list: toolbar

list-select = تحديد
list-refresh = إعادة التحميل
list-more = المزيد
list-mark-read = وضع علامة «مقروءة»
list-mark-unread = وضع علامة «غير مقروءة»
list-move-to = نقل إلى
list-archive = أرشفة
list-spam = الإبلاغ عن محتوى غير مرغوب فيه
list-delete = حذف
list-newer = أحدث
list-older = أقدم
list-range = { $first }–{ $last } من { $total }
list-range-about = { $first }–{ $last } من حوالي { $total }
list-results = نتائج «{ $query }»
list-results-corrected = يتم عرض نتائج «{ $query }»
list-search-instead = البحث عن «{ $query }» بدلًا من ذلك
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = الكل
list-pick-none = بلا تحديد
list-pick-read = المقروءة
list-pick-unread = غير المقروءة
list-pick-starred = المميّزة بنجمة
list-pick-unstarred = غير المميّزة بنجمة

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [zero] تم تحديد كل المحادثات الـ{ $count }.
        [one] تم تحديد محادثة واحدة.
        [two] تم تحديد المحادثتين.
        [few] تم تحديد كل المحادثات الـ{ $count }.
        [many] تم تحديد كل المحادثات الـ{ $count }.
       *[other] تم تحديد كل المحادثات الـ{ $count }.
    }
   *[message] { $count ->
        [zero] تم تحديد كل الرسائل الـ{ $count }.
        [one] تم تحديد رسالة واحدة.
        [two] تم تحديد الرسالتين.
        [few] تم تحديد كل الرسائل الـ{ $count }.
        [many] تم تحديد كل الرسائل الـ{ $count }.
       *[other] تم تحديد كل الرسائل الـ{ $count }.
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [zero] تم تحديد كل المحادثات الـ{ $count } في { $folder }.
        [one] تم تحديد محادثة واحدة في { $folder }.
        [two] تم تحديد المحادثتين في { $folder }.
        [few] تم تحديد كل المحادثات الـ{ $count } في { $folder }.
        [many] تم تحديد كل المحادثات الـ{ $count } في { $folder }.
       *[other] تم تحديد كل المحادثات الـ{ $count } في { $folder }.
    }
   *[message] { $count ->
        [zero] تم تحديد كل الرسائل الـ{ $count } في { $folder }.
        [one] تم تحديد رسالة واحدة في { $folder }.
        [two] تم تحديد الرسالتين في { $folder }.
        [few] تم تحديد كل الرسائل الـ{ $count } في { $folder }.
        [many] تم تحديد كل الرسائل الـ{ $count } في { $folder }.
       *[other] تم تحديد كل الرسائل الـ{ $count } في { $folder }.
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [zero] تم تحديد كل المحادثات الـ{ $count } على الشاشة.
        [one] تم تحديد محادثة واحدة على الشاشة.
        [two] تم تحديد المحادثتين على الشاشة.
        [few] تم تحديد كل المحادثات الـ{ $count } على الشاشة.
        [many] تم تحديد كل المحادثات الـ{ $count } على الشاشة.
       *[other] تم تحديد كل المحادثات الـ{ $count } على الشاشة.
    }
   *[message] { $count ->
        [zero] تم تحديد كل الرسائل الـ{ $count } على الشاشة.
        [one] تم تحديد رسالة واحدة على الشاشة.
        [two] تم تحديد الرسالتين على الشاشة.
        [few] تم تحديد كل الرسائل الـ{ $count } على الشاشة.
        [many] تم تحديد كل الرسائل الـ{ $count } على الشاشة.
       *[other] تم تحديد كل الرسائل الـ{ $count } على الشاشة.
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [zero] تحديد كل المحادثات الـ{ $count }
        [one] تحديد محادثة واحدة
        [two] تحديد المحادثتين
        [few] تحديد كل المحادثات الـ{ $count }
        [many] تحديد كل المحادثات الـ{ $count }
       *[other] تحديد كل المحادثات الـ{ $count }
    }
   *[message] { $count ->
        [zero] تحديد كل الرسائل الـ{ $count }
        [one] تحديد رسالة واحدة
        [two] تحديد الرسالتين
        [few] تحديد كل الرسائل الـ{ $count }
        [many] تحديد كل الرسائل الـ{ $count }
       *[other] تحديد كل الرسائل الـ{ $count }
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [zero] تحديد كل المحادثات الـ{ $count } في { $folder }
        [one] تحديد محادثة واحدة في { $folder }
        [two] تحديد المحادثتين في { $folder }
        [few] تحديد كل المحادثات الـ{ $count } في { $folder }
        [many] تحديد كل المحادثات الـ{ $count } في { $folder }
       *[other] تحديد كل المحادثات الـ{ $count } في { $folder }
    }
   *[message] { $count ->
        [zero] تحديد كل الرسائل الـ{ $count } في { $folder }
        [one] تحديد رسالة واحدة في { $folder }
        [two] تحديد الرسالتين في { $folder }
        [few] تحديد كل الرسائل الـ{ $count } في { $folder }
        [many] تحديد كل الرسائل الـ{ $count } في { $folder }
       *[other] تحديد كل الرسائل الـ{ $count } في { $folder }
    }
}
list-clear-selection = محو التحديد

## Mail list: empty states

list-empty-search = لا توجد رسائل مطابقة لبحثك.
list-empty-tab = لا يوجد بريد في { $tab }.
list-empty-tab-unknown = لا يوجد بريد في علامة التبويب هذه.
list-empty-folder = لا توجد رسائل في { $folder }.
list-empty-folder-unknown = لا توجد رسائل في هذا المجلد.
list-first-sync = جارٍ جلب بريدك…
list-first-sync-detail = يظهر هنا فور وصوله.

## Mail list: lines

row-removed = تمت إزالة هذه الرسالة.
row-starred = مميّزة بنجمة
row-not-starred = غير مميّزة بنجمة
row-important = مهمة. انقر لوضع علامة «غير مهمة».
row-mark-important = وضع علامة «مهمة»
row-pinned = مثبّتة في الأعلى
row-pin = تثبيت في الأعلى
row-unpin = إلغاء التثبيت

## Mail list: More menu and right-click menu

menu-reply = رد
menu-reply-all = الرد على الكل
menu-forward = إعادة توجيه
menu-archive = أرشفة
menu-delete = حذف
menu-spam = الإبلاغ عن محتوى غير مرغوب فيه
menu-mark-read = وضع علامة «مقروءة»
menu-mark-unread = وضع علامة «غير مقروءة»
menu-mark-all-read = وضع علامة «مقروءة» على الكل
menu-star = إضافة نجمة
menu-unstar = إزالة النجمة
menu-important = وضع علامة «مهمة»
menu-not-important = وضع علامة «غير مهمة»
menu-pin = تثبيت في الأعلى
menu-unpin = إلغاء التثبيت
menu-print-all = طباعة الكل
menu-new-window = فتح في نافذة جديدة
menu-move-to = نقل إلى
menu-move-to-heading = نقل إلى:
menu-find-from = البحث عن رسائل من { $name }

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [zero] تمت أرشفة { $count } محادثة.
        [one] تمت أرشفة المحادثة.
        [two] تمت أرشفة محادثتين.
        [few] تمت أرشفة { $count } محادثات.
        [many] تمت أرشفة { $count } محادثة.
       *[other] تمت أرشفة { $count } محادثة.
    }
   *[message] { $count ->
        [zero] تمت أرشفة { $count } رسالة.
        [one] تمت أرشفة الرسالة.
        [two] تمت أرشفة رسالتين.
        [few] تمت أرشفة { $count } رسائل.
        [many] تمت أرشفة { $count } رسالة.
       *[other] تمت أرشفة { $count } رسالة.
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [zero] تم نقل { $count } محادثة إلى المهملات.
        [one] تم نقل المحادثة إلى المهملات.
        [two] تم نقل محادثتين إلى المهملات.
        [few] تم نقل { $count } محادثات إلى المهملات.
        [many] تم نقل { $count } محادثة إلى المهملات.
       *[other] تم نقل { $count } محادثة إلى المهملات.
    }
   *[message] { $count ->
        [zero] تم نقل { $count } رسالة إلى المهملات.
        [one] تم نقل الرسالة إلى المهملات.
        [two] تم نقل رسالتين إلى المهملات.
        [few] تم نقل { $count } رسائل إلى المهملات.
        [many] تم نقل { $count } رسالة إلى المهملات.
       *[other] تم نقل { $count } رسالة إلى المهملات.
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [zero] تم نقل { $count } محادثة.
        [one] تم نقل المحادثة.
        [two] تم نقل محادثتين.
        [few] تم نقل { $count } محادثات.
        [many] تم نقل { $count } محادثة.
       *[other] تم نقل { $count } محادثة.
    }
   *[message] { $count ->
        [zero] تم نقل { $count } رسالة.
        [one] تم نقل الرسالة.
        [two] تم نقل رسالتين.
        [few] تم نقل { $count } رسائل.
        [many] تم نقل { $count } رسالة.
       *[other] تم نقل { $count } رسالة.
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [zero] تم تمييز { $count } محادثة بنجمة.
        [one] تم تمييز المحادثة بنجمة.
        [two] تم تمييز محادثتين بنجمة.
        [few] تم تمييز { $count } محادثات بنجمة.
        [many] تم تمييز { $count } محادثة بنجمة.
       *[other] تم تمييز { $count } محادثة بنجمة.
    }
   *[message] { $count ->
        [zero] تم تمييز { $count } رسالة بنجمة.
        [one] تم تمييز الرسالة بنجمة.
        [two] تم تمييز رسالتين بنجمة.
        [few] تم تمييز { $count } رسائل بنجمة.
        [many] تم تمييز { $count } رسالة بنجمة.
       *[other] تم تمييز { $count } رسالة بنجمة.
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [zero] تمت إزالة النجمة من { $count } محادثة.
        [one] تمت إزالة النجمة من المحادثة.
        [two] تمت إزالة النجمة من محادثتين.
        [few] تمت إزالة النجمة من { $count } محادثات.
        [many] تمت إزالة النجمة من { $count } محادثة.
       *[other] تمت إزالة النجمة من { $count } محادثة.
    }
   *[message] { $count ->
        [zero] تمت إزالة النجمة من { $count } رسالة.
        [one] تمت إزالة النجمة من الرسالة.
        [two] تمت إزالة النجمة من رسالتين.
        [few] تمت إزالة النجمة من { $count } رسائل.
        [many] تمت إزالة النجمة من { $count } رسالة.
       *[other] تمت إزالة النجمة من { $count } رسالة.
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [zero] تم وضع علامة «مهمة» على { $count } محادثة.
        [one] تم وضع علامة «مهمة» على المحادثة.
        [two] تم وضع علامة «مهمة» على محادثتين.
        [few] تم وضع علامة «مهمة» على { $count } محادثات.
        [many] تم وضع علامة «مهمة» على { $count } محادثة.
       *[other] تم وضع علامة «مهمة» على { $count } محادثة.
    }
   *[message] { $count ->
        [zero] تم وضع علامة «مهمة» على { $count } رسالة.
        [one] تم وضع علامة «مهمة» على الرسالة.
        [two] تم وضع علامة «مهمة» على رسالتين.
        [few] تم وضع علامة «مهمة» على { $count } رسائل.
        [many] تم وضع علامة «مهمة» على { $count } رسالة.
       *[other] تم وضع علامة «مهمة» على { $count } رسالة.
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [zero] تم وضع علامة «غير مهمة» على { $count } محادثة.
        [one] تم وضع علامة «غير مهمة» على المحادثة.
        [two] تم وضع علامة «غير مهمة» على محادثتين.
        [few] تم وضع علامة «غير مهمة» على { $count } محادثات.
        [many] تم وضع علامة «غير مهمة» على { $count } محادثة.
       *[other] تم وضع علامة «غير مهمة» على { $count } محادثة.
    }
   *[message] { $count ->
        [zero] تم وضع علامة «غير مهمة» على { $count } رسالة.
        [one] تم وضع علامة «غير مهمة» على الرسالة.
        [two] تم وضع علامة «غير مهمة» على رسالتين.
        [few] تم وضع علامة «غير مهمة» على { $count } رسائل.
        [many] تم وضع علامة «غير مهمة» على { $count } رسالة.
       *[other] تم وضع علامة «غير مهمة» على { $count } رسالة.
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [zero] تم تثبيت { $count } محادثة في الأعلى.
        [one] تم تثبيت المحادثة في الأعلى.
        [two] تم تثبيت محادثتين في الأعلى.
        [few] تم تثبيت { $count } محادثات في الأعلى.
        [many] تم تثبيت { $count } محادثة في الأعلى.
       *[other] تم تثبيت { $count } محادثة في الأعلى.
    }
   *[message] { $count ->
        [zero] تم تثبيت { $count } رسالة في الأعلى.
        [one] تم تثبيت الرسالة في الأعلى.
        [two] تم تثبيت رسالتين في الأعلى.
        [few] تم تثبيت { $count } رسائل في الأعلى.
        [many] تم تثبيت { $count } رسالة في الأعلى.
       *[other] تم تثبيت { $count } رسالة في الأعلى.
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [zero] تم إلغاء تثبيت { $count } محادثة.
        [one] تم إلغاء تثبيت المحادثة.
        [two] تم إلغاء تثبيت محادثتين.
        [few] تم إلغاء تثبيت { $count } محادثات.
        [many] تم إلغاء تثبيت { $count } محادثة.
       *[other] تم إلغاء تثبيت { $count } محادثة.
    }
   *[message] { $count ->
        [zero] تم إلغاء تثبيت { $count } رسالة.
        [one] تم إلغاء تثبيت الرسالة.
        [two] تم إلغاء تثبيت رسالتين.
        [few] تم إلغاء تثبيت { $count } رسائل.
        [many] تم إلغاء تثبيت { $count } رسالة.
       *[other] تم إلغاء تثبيت { $count } رسالة.
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [zero] تم الإبلاغ عن { $count } محادثة كمحتوى غير مرغوب فيه.
        [one] تم الإبلاغ عن المحادثة كمحتوى غير مرغوب فيه.
        [two] تم الإبلاغ عن محادثتين كمحتوى غير مرغوب فيه.
        [few] تم الإبلاغ عن { $count } محادثات كمحتوى غير مرغوب فيه.
        [many] تم الإبلاغ عن { $count } محادثة كمحتوى غير مرغوب فيه.
       *[other] تم الإبلاغ عن { $count } محادثة كمحتوى غير مرغوب فيه.
    }
   *[message] { $count ->
        [zero] تم الإبلاغ عن { $count } رسالة كمحتوى غير مرغوب فيه.
        [one] تم الإبلاغ عن الرسالة كمحتوى غير مرغوب فيه.
        [two] تم الإبلاغ عن رسالتين كمحتوى غير مرغوب فيه.
        [few] تم الإبلاغ عن { $count } رسائل كمحتوى غير مرغوب فيه.
        [many] تم الإبلاغ عن { $count } رسالة كمحتوى غير مرغوب فيه.
       *[other] تم الإبلاغ عن { $count } رسالة كمحتوى غير مرغوب فيه.
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [zero] تم حذف { $count } محادثة نهائيًا.
        [one] تم حذف المحادثة نهائيًا.
        [two] تم حذف محادثتين نهائيًا.
        [few] تم حذف { $count } محادثات نهائيًا.
        [many] تم حذف { $count } محادثة نهائيًا.
       *[other] تم حذف { $count } محادثة نهائيًا.
    }
   *[message] { $count ->
        [zero] تم حذف { $count } رسالة نهائيًا.
        [one] تم حذف الرسالة نهائيًا.
        [two] تم حذف رسالتين نهائيًا.
        [few] تم حذف { $count } رسائل نهائيًا.
        [many] تم حذف { $count } رسالة نهائيًا.
       *[other] تم حذف { $count } رسالة نهائيًا.
    }
}
toast-undone = تم التراجع عن الإجراء.
toast-undo = تراجع
toast-no-spam-folder = لا يحتوي هذا الحساب على مجلد للرسائل غير المرغوب فيها.

## Reading pane: toolbar

reader-close = إغلاق
reader-back = رجوع
reader-mark-unread = وضع علامة «غير مقروءة»
reader-move-to = نقل إلى
reader-more = المزيد
reader-print-all = طباعة الكل
reader-new-window = في نافذة جديدة
reader-position = { $position } من { $total }
reader-newer = أحدث
reader-older = أقدم

## Reading pane: the conversation

reader-removed = تمت إزالة هذه المحادثة.
reader-no-subject = (بلا موضوع)
reader-collapse-all = تصغير الكل
reader-expand-all = توسيع الكل
reader-unknown-sender = (مُرسِل غير معروف)
reader-date-ago = { $date } ({ $ago })
reader-me = أنا
reader-to = إلى { $names }
reader-starred = مميّزة بنجمة
reader-not-starred = غير مميّزة بنجمة
reader-too-long = الرسالة طويلة جدًا بحيث لا يمكن عرضها بالكامل.
reader-encrypted-images = لا يتم أبدًا تحميل الصور من الويب في البريد المشفّر.
reader-window-failed = تعذّر فتح نافذة جديدة.

## Reading pane: message details (opened from "to me")

reader-details-from = من:
reader-details-to = إلى:
reader-details-cc = نسخة إلى:
reader-details-date = التاريخ:
reader-details-subject = الموضوع:

## Reading pane: downloading a message

reader-downloading = جارٍ تنزيل هذه الرسالة من الخادم…
reader-download-failed = تعذّر تنزيل هذه الرسالة.
reader-try-again = إعادة المحاولة

## Reply row

reply-reply = رد
reply-reply-all = الرد على الكل
reply-forward = إعادة توجيه

## Encrypted and signed mail

security-decrypting = جارٍ فك التشفير…
security-checking = جارٍ التحقق من التوقيع…
security-partly-encrypted = جزء فقط من هذه الرسالة مشفّر. أُضيف الباقي خارج الحماية وقد يكون مصدره أي شخص.
security-partly-signed = جزء فقط من هذه الرسالة موقَّع. أُضيف الباقي خارج الحماية وقد يكون مصدره أي شخص.
security-encrypted = رسالة مشفّرة
security-encrypted-smime = رسالة مشفّرة (S/MIME)
security-no-key = يتعذّر فك تشفير هذه الرسالة: شُفّرت لمفتاح ليس لديك.
security-cancelled = تم إلغاء فك التشفير.
security-damaged = يتعذّر فك تشفير هذه الرسالة: البيانات المشفّرة تالفة أو تم تغييرها.
security-decrypt-unavailable = يتعذّر فك تشفير هذه الرسالة: ثبّت { $tool } لقراءة البريد المشفّر.
security-decrypt-failed = يتعذّر فك تشفير هذه الرسالة: { $reason }
security-unknown-signer = موقِّع غير معروف
security-signed-verified = وقّعها { $signer } · تم التحقق
security-signed-not-sender = وقّعها { $signer }، وهو ليس المُرسِل
security-signed-untrusted = وقّعها { $signer } بمفتاح وضعت عليه علامة «غير موثوق به»
security-signed-unverified = وقّعها { $signer } · لم يتم التحقق من المفتاح
security-bad-signature = توقيع غير صالح: تم تغيير هذه الرسالة بعد توقيعها، أو أن التوقيع مزوَّر.
security-signature-expired = وقّعها { $signer } · انتهت صلاحية التوقيع
security-key-expired = وقّعها { $signer } · انتهت صلاحية المفتاح منذ ذلك الحين
security-key-revoked = وقّعها { $signer } بمفتاح تم إبطاله
security-missing-key = موقَّعة بمفتاح ليس لديك، لذا يتعذّر التحقق منها
security-missing-key-id = موقَّعة بمفتاح ليس لديك ({ $key })، لذا يتعذّر التحقق منها
security-signature-unavailable = موقَّعة؛ ثبّت { $tool } للتحقق من التوقيع
security-signature-error = تعذّر التحقق من التوقيع.

## Remote images and pictures

remote-hidden = الصور في هذه الرسالة مخفية.
remote-show = عرض الصور
remote-always-show = العرض دائمًا من هذا المُرسِل
remote-picture-use = استخدام
remote-picture-too-big = اختر صورة لا يزيد حجمها عن 8 ميغابايت.
remote-picture-type = اختر صورة بتنسيق PNG أو JPEG أو GIF أو WebP أو SVG.
remote-picture-read-failed = تتعذّر قراءة الصورة: { $error }
remote-picture-keep-failed = يتعذّر الاحتفاظ بالصورة: { $error }
remote-picture-remove-failed = تتعذّر إزالة الصورة: { $error }

## Attachments

attachment-count = { $count ->
    [zero] { $count } مرفق
    [one] مرفق واحد
    [two] مرفقان
    [few] { $count } مرفقات
    [many] { $count } مرفقًا
   *[other] { $count } مرفق
}
attachment-save = حفظ
attachment-save-all = حفظ الكل
attachment-save-all-tooltip = حفظ كل المرفقات في مجلد
attachment-save-here = الحفظ هنا
attachment-not-downloaded = لم يتم تنزيل هذه الرسالة.
attachment-not-found = تعذّر العثور على هذا المرفق في الرسالة.
attachment-read-failed = تعذّرت قراءة { $name }
attachment-numbered = المرفق { $number }
attachment-saved-all = { $count ->
    [zero] تم حفظ { $count } ملف في { $place }
    [one] تم حفظ ملف واحد في { $place }
    [two] تم حفظ ملفين في { $place }
    [few] تم حفظ { $count } ملفات في { $place }
    [many] تم حفظ { $count } ملفًا في { $place }
   *[other] تم حفظ { $count } ملف في { $place }
}
attachment-saved-some = { $total ->
    [zero] تم حفظ { $saved } من أصل { $total } ملف في { $place }. تعذّر حفظ { $failed }
    [one] تم حفظ { $saved } من أصل ملف واحد في { $place }. تعذّر حفظ { $failed }
    [two] تم حفظ { $saved } من أصل ملفين في { $place }. تعذّر حفظ { $failed }
    [few] تم حفظ { $saved } من أصل { $total } ملفات في { $place }. تعذّر حفظ { $failed }
    [many] تم حفظ { $saved } من أصل { $total } ملفًا في { $place }. تعذّر حفظ { $failed }
   *[other] تم حفظ { $saved } من أصل { $total } ملف في { $place }. تعذّر حفظ { $failed }
}
attachment-saved-to = تم الحفظ في { $path }
attachment-save-failed = تعذّر حفظ { $name }: { $error }
attachment-open-failed = تعذّر فتح { $name }: { $error }
attachment-risky = قد يشغّل هذا الملف برنامجًا، لذا لا يفتحه Katna. احفظه بدلًا من ذلك.
attachment-encrypted-open = وصل هذا الملف مشفّرًا. احفظه لفتحه في مكان آخر.

## Printing

print-failed = تعذّرت الطباعة: { $error }
print-no-font = لم يتم العثور على أي خط
print-opened-as-pdf = تم الفتح كملف PDF للطباعة منه.
print-not-downloaded = (لم يتم التنزيل بعد.)
print-encrypted = (مشفّرة. افتحها في Katna Mail لطباعة نصها.)
print-to = إلى: { $addresses }
print-cc = نسخة إلى: { $addresses }
