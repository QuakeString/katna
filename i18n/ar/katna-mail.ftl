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

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = افتح هذه الرسالة لقراءة مرفقاتها.
text-copy = نسخ
text-select-all = تحديد الكل

## Settings page: its tabs

settings-tab-general = عام
settings-tab-inbox = البريد الوارد
settings-tab-accounts = الحسابات
settings-tab-subscriptions = الاشتراكات
settings-tab-appearance = المظهر
settings-tab-shortcuts = الاختصارات
settings-tab-default-apps = التطبيقات التلقائية
settings-tab-folders-rules = المجلدات والقواعد
settings-tab-compose = الكتابة
settings-tab-mcp-server = خادم MCP
settings-tab-feedback = ملاحظات المستخدمين
settings-tab-experimental = تجريبي

## Settings page: tabs still to come

settings-tab-subscriptions-coming = اطّلع على النشرات الإخبارية والقوائم البريدية التي تصلك، وألغِ الاشتراك بنقرة واحدة.
settings-tab-folders-rules-coming = أنشئ المجلدات والتصنيفات وأعِد تسميتها وانقلها وأخفِها، واختر ما تتم مزامنته منها. تفرز القواعد البريد الجديد وتصنّفه وتعيد توجيهه أو تحذفه تلقائيًا، حسب المُرسِل أو الموضوع أو الكلمات.
settings-tab-mcp-server-coming = اسمح لمساعدي الذكاء الاصطناعي على هذا الكمبيوتر بالبحث في بريدك وقراءته وكتابة مسودات الرسائل، بموافقتك.

## Settings > General

settings-general-conversations = عرض المحادثات
settings-general-conversations-group = تجميع الردود على الرسالة نفسها
settings-general-conversations-group-detail = سطر واحد لكل محادثة في القائمة
settings-general-reading = القراءة
settings-general-newest-first = الرسالة الأحدث أولًا
settings-general-newest-first-detail = تبدأ المحادثة بآخر رد فيها
settings-general-full-headers = عرض الرؤوس الكاملة
settings-general-full-headers-detail = تظهر حقول «من» و«إلى» و«نسخة إلى» والتاريخ والموضوع في كل رسالة
settings-general-full-names = الأسماء الكاملة للمستلمين
settings-general-full-names-detail = «إليّ، Ada Lovelace» بدلًا من «إليّ، Ada»
settings-general-mark-read = وضع علامة «مقروءة»
settings-general-mark-read-now = فور فتحها
settings-general-mark-read-1s = بعد فتحها لمدة ثانية واحدة
settings-general-mark-read-3s = بعد فتحها لمدة 3 ثوانٍ
settings-general-mark-read-never = فقط عندما أضع عليها علامة «مقروءة»
settings-general-reply-button = زر الرد
settings-general-reply-all = الرد على الكل
settings-general-reply-all-detail = يرد زر الرد بجانب كل رسالة على الجميع، وليس على المُرسِل فقط
settings-general-remote-images = الصور من الويب
settings-general-remote-images-detail = يُخبر تحميلُ صور الرسالة مُرسِلَها بأنك فتحتها، ومتى، ومن أين تقريبًا. عند إيقاف هذا الخيار، تسأل كل رسالة أولًا، ويمكنك دائمًا عرض صور أي مُرسِل.
settings-general-remote-images-always = عرض الصور دائمًا
settings-general-remote-images-always-detail = في كل رسالة، وليس فقط من المُرسِلين الذين تثق بهم
settings-general-sending = الإرسال
settings-general-sending-detail = المدة التي تنتظرها الرسالة المُرسَلة، حتى يمكن التراجع عن إرسالها.
settings-general-offline = البريد بلا اتصال
settings-general-offline-detail = يتم تنزيل البريد الحديث كاملًا لقراءته بلا اتصال. ويتم تنزيل البريد الأقدم عند فتحه.
settings-general-offline-days = { $count ->
    [zero] { $count } يوم
    [one] يوم واحد
    [two] يومان
    [few] { $count } أيام
    [many] { $count } يومًا
   *[other] { $count } يوم
}
settings-general-offline-years = { $count ->
    [zero] { $count } سنة
    [one] سنة واحدة
    [two] سنتان
    [few] { $count } سنوات
    [many] { $count } سنة
   *[other] { $count } سنة
}
settings-general-offline-all = كل البريد
settings-general-offline-note = اختيار عدد أيام أقل يُبقي على البريد الذي تم تنزيله. لا يتغير أي شيء على الخادم.
settings-general-notifications = الإشعارات
settings-general-notifications-detail = للبريد الجديد في البريد الوارد، حتى عندما يكون Katna Mail مغلقًا.
settings-general-new-mail = إشعاري بالبريد الجديد
settings-general-new-mail-detail = مع أزرار الرد على الكل ووضع علامة «مقروءة» والأرشفة
settings-general-new-mail-sound = تشغيل صوت
settings-general-new-mail-sound-detail = صوت البريد الجديد في سطح المكتب
settings-general-desktop = سطح المكتب
settings-general-open-at-login = فتح Katna Mail عند تسجيل الدخول
settings-general-open-at-login-detail = تتم مزامنة البريد عند تسجيل الدخول في الحالتين، ما دامت الخدمة تعمل
settings-general-tray = عرض Katna في علبة النظام
settings-general-tray-detail = مع عدد الرسائل غير المقروءة وقائمة
settings-general-unread-badge = عدد غير المقروءة على أيقونة شريط المهام
settings-general-unread-badge-detail = عدد رسائل البريد الوارد غير المقروءة

## Settings > Inbox

settings-inbox-tabs = علامات تبويب البريد الوارد
settings-inbox-tabs-detail = فرز البريد الوارد في علامات تبويب، كما يفعل موقع مزوّد بريدك.
settings-inbox-tabs-show = عرض علامات تبويب البريد الوارد
settings-inbox-tabs-show-detail = عند الإيقاف تظهر قائمة واحدة لكل حساب
settings-inbox-no-accounts = أضف حسابًا لاختيار علامات التبويب الخاصة به.
settings-inbox-tabs-automatic = تلقائي: { $tabs } ({ $provider })
settings-inbox-tabs-off = بلا علامات تبويب
settings-inbox-tabs-gmail = الأساسية، العروض الترويجية، الشبكات الاجتماعية، التحديثات، المنتديات
settings-inbox-tabs-focused = المُركَّز وأخرى
settings-inbox-tabs-zoho = البريد الوارد والنشرات الإخبارية والإشعارات
settings-inbox-tabs-shown = علامات التبويب المعروضة. يبقى بريد علامة التبويب التي توقفها في { $tab }.

## Settings > Appearance

settings-appearance-reading-pane = جزء القراءة
settings-appearance-reading-pane-detail = مكان ظهور المحادثة المفتوحة.
settings-appearance-pane-right = بجانب القائمة
settings-appearance-pane-none = بلا تقسيم
settings-appearance-density = الكثافة
settings-appearance-density-default = تلقائية
settings-appearance-density-compact = مضغوطة
settings-appearance-scaling = التحجيم
settings-appearance-scaling-detail = يكبّر كل شيء في Katna Mail أو يصغّره، فوق تحجيم سطح المكتب نفسه: النصوص والأيقونات والمسافات والفواصل. يحتفظ البريد الذي ترسله بحجم خطه. قد تجعل الأحجام الصغيرة جدًا النقر على الأيقونات صعبًا.
settings-appearance-theme = السمة
settings-appearance-theme-system = مثل سطح المكتب
settings-appearance-theme-light = فاتحة
settings-appearance-theme-dark = داكنة
settings-appearance-desktop-colors = ألوان سطح المكتب
settings-appearance-desktop-colors-use = استخدام ألوان سطح المكتب
settings-appearance-desktop-colors-use-detail = نظام الألوان ولون التمييز في سطح المكتب
settings-appearance-app-names = أسماء التطبيقات
settings-appearance-app-names-show = عرض أسماء التطبيقات
settings-appearance-app-names-show-detail = الأسماء أسفل أيقونات التطبيقات في أقصى اليمين
settings-appearance-sender-pictures = صور المُرسِلين
settings-appearance-sender-pictures-show = عرض شعارات الشركات
settings-appearance-sender-pictures-show-detail = يتم البحث عنها حسب نطاق المُرسِل، وليس حسب الرسالة أبدًا، ويُحتفظ بها لمدة أسبوع
settings-appearance-important = علامات «مهمة»
settings-appearance-important-show = عرض علامات «مهمة»
settings-appearance-important-show-detail = بجانب كل رسالة في القائمة
settings-appearance-message-width = اتساع الرسائل
settings-appearance-message-width-limit = تحديد اتساع الرسائل
settings-appearance-message-width-limit-detail = قراءة الأسطر الطويلة أسهل في نافذة عريضة
settings-appearance-mail-colors = ألوان البريد
settings-appearance-mail-colors-detail = صُمّم معظم البريد لصفحة بيضاء. مع السمة الداكنة، تتغير ألوانه إلى ألوان داكنة سهلة القراءة؛ وعند الإيقاف، يحتفظ بألوان مُرسِله على صفحة فاتحة.
settings-appearance-dark-mail = ألوان داكنة للبريد أيضًا
settings-appearance-dark-mail-detail = فقط عندما تكون السمة داكنة
settings-appearance-attachment-previews = معاينات المرفقات
settings-appearance-attachment-previews-show = عرض معاينات المرفقات
settings-appearance-attachment-previews-show-detail = صورة صغيرة لمحتوى كل ملف على بطاقته

## Settings > Default apps

settings-default-apps-intro = مكان فتح المرفقات عند النقر عليها. ويمكن للعارض دائمًا فتح الملف في تطبيق آخر أيضًا. يتم ضبط التطبيقات التلقائية لسطح المكتب في إعداداته الخاصة.
settings-default-apps-pdf = ملفات PDF
settings-default-apps-pdf-detail = الصفحات، مع التكبير والتصغير.
settings-default-apps-pictures = الصور
settings-default-apps-pictures-detail = الصور الفوتوغرافية (بالاتجاه الصحيح) وPNG وGIF وWebP وBMP وTIFF وSVG.
settings-default-apps-text = الملفات النصية
settings-default-apps-text-detail = النص العادي والسجلات والتعليمات البرمجية والنصوص الأخرى.
settings-default-apps-sheets = جداول البيانات
settings-default-apps-sheets-detail = Excel (xlsx وxls) وOpenDocument (ods) وCSV.
settings-default-apps-documents = المستندات
settings-default-apps-documents-detail = Word (docx) ونصوص OpenDocument (odt).
settings-default-apps-katna = عارض Katna Mail
settings-default-apps-system = التطبيق التلقائي لسطح المكتب
settings-default-apps-ask = السؤال عن التطبيق في كل مرة
settings-default-apps-after-saving = بعد الحفظ
settings-default-apps-show-folder = عرض الملفات المحفوظة في مجلدها
settings-default-apps-show-folder-detail = يفتح مدير الملفات مع تحديد المرفقات المحفوظة

## Settings > Compose

settings-compose-send-from = إرسال الرسائل الجديدة من
settings-compose-send-from-detail = يتم دائمًا إرسال الردود وإعادات التوجيه من الحساب الذي تستخدمه.
settings-compose-send-from-current = الحساب الذي تستخدمه
settings-compose-send-on-replies = الإرسال في الردود
settings-compose-send-on-replies-detail = ما يفعله زر «إرسال» في الرد أو إعادة التوجيه. وتوفّر القائمة بجانب «إرسال» الخيار الآخر.
settings-compose-send-plain = إرسال
settings-compose-send-archive = إرسال وأرشفة
settings-compose-signatures = التوقيعات
settings-compose-signatures-detail = يُضاف أسفل رسالتك، بعد سطر «--». اختر توقيعًا آخر في نافذة الإنشاء.
settings-compose-untitled = بلا عنوان
settings-compose-signature-name = الاسم، مثل: العمل
settings-compose-signature-first = توقيعي
settings-compose-signature-numbered = التوقيع { $number }
settings-compose-signature-delete = حذف
settings-compose-signature-deleted = تم حذف التوقيع
settings-compose-signature-new = إنشاء جديد
settings-compose-no-signatures = لا توجد توقيعات بعد.
settings-compose-no-signature = بلا توقيع
settings-compose-for-new-mail = للرسائل الجديدة
settings-compose-for-replies = للردود وإعادة التوجيه
settings-compose-for-replies-detail = في محادثة وقّعت فيها رسالة، يبدأ الرد بذلك التوقيع بدلًا من ذلك.
settings-compose-format = التنسيق
settings-compose-plain-text = الكتابة بنص عادي
settings-compose-plain-text-detail = يبدأ البريد الجديد بلا تنسيق؛ ويمكن التبديل في نافذة الإنشاء
settings-compose-spelling = الإملاء
settings-compose-spell-check = التدقيق الإملائي أثناء الكتابة
settings-compose-spell-check-detail = يوضع خط تحت الكلمات التي بها أخطاء إملائية، مع اقتراحات عند النقر بزر الماوس الأيمن
settings-compose-spell-desktop = لغة سطح المكتب ({ $language })
settings-compose-templates = النماذج
settings-compose-templates-detail = احفظ الرسائل التي تكتبها كثيرًا، وابدأ منها رسالة جديدة أو ردًا.

## Settings > Shortcuts

settings-shortcuts-set = مجموعة الاختصارات
settings-shortcuts-set-detail = ابدأ من مفاتيح تطبيق بريد تعرفه. مفتاح Cmd هو Ctrl هنا. تبقى تغييراتك فوق المجموعة، ويعيد «استعادة الإعدادات التلقائية» مفاتيح المجموعة.
settings-shortcuts-single = اختصارات المفتاح الواحد
settings-shortcuts-single-detail = مفاتيح بلا Ctrl أو Alt، كما في بريد الويب: e للأرشفة، وj وk للتنقل، و/ للبحث. تعمل في القائمة وفي المحادثة المفتوحة، ولا تعمل أبدًا أثناء الكتابة.
settings-shortcuts-single-use = استخدام اختصارات المفتاح الواحد
settings-shortcuts-single-use-detail = تعمل اختصارات Ctrl دائمًا
settings-shortcuts-how = انقر على مفتاح لتغييره، أو على + لإضافة مفتاح، ثم اضغط المفاتيح الجديدة. يلغي Esc العملية.
settings-shortcuts-restore = استعادة الإعدادات التلقائية
settings-shortcuts-no-key = بلا مفتاح
settings-shortcuts-press = اضغط المفاتيح…
settings-shortcuts-then = { $keys } ثم…
settings-shortcuts-moved = أصبح { $keys } ينفّذ «{ $action }» بدلًا من «{ $previous }».
settings-shortcuts-single-off = اختصارات المفتاح الواحد متوقفة، لذا سيعمل هذا المفتاح بعد تفعيلها.
settings-shortcuts-restored = عادت كل الاختصارات إلى مفاتيح مجموعتها.

## Settings search: the line under a result

settings-general-language-summary = لغة التطبيق والتواريخ والأرقام
settings-general-reading-summary = الرسالة الأحدث أولًا، والرؤوس الكاملة، والأسماء الكاملة للمستلمين
settings-general-mark-read-summary = متى توضع علامة «مقروءة» على المحادثة المفتوحة: فورًا، أو بعد ثانية أو 3 ثوانٍ، أو يدويًا
settings-general-reply-button-summary = يرد زر الرد بجانب كل رسالة على الجميع
settings-general-remote-images-summary = عرض الصور في كل رسالة دائمًا
settings-general-sending-summary = التراجع عن الإرسال: المدة التي تنتظرها الرسالة المُرسَلة، حتى يمكن التراجع عن إرسالها
settings-general-offline-summary = عدد أيام البريد الحديث التي يتم تنزيلها كاملة لقراءتها بلا اتصال
settings-general-notifications-summary = إشعارات البريد الجديد وصوتها
settings-general-desktop-summary = فتح Katna Mail عند تسجيل الدخول، وأيقونة علبة النظام، وعدد غير المقروءة على أيقونة شريط المهام
settings-accounts-accounts-summary = إضافة حساب أو إزالته، أو تغيير صورته
settings-appearance-density-summary = أسطر تلقائية أو مضغوطة في القائمة
settings-appearance-scaling-summary = تكبير كل شيء أو تصغيره: النصوص والأيقونات والمسافات والفواصل
settings-appearance-theme-summary = مثل سطح المكتب، أو فاتحة، أو داكنة
settings-appearance-sender-pictures-summary = شعارات الشركات، يتم البحث عنها حسب نطاق المُرسِل
settings-appearance-important-summary = علامة «مهمة» بجانب كل رسالة في القائمة
settings-appearance-mail-colors-summary = ألوان داكنة لبريد HTML في السمة الداكنة، أو ألوان مُرسِله
settings-appearance-attachment-previews-summary = صورة صغيرة لمحتوى كل مرفق
settings-shortcuts-set-summary = البدء من مفاتيح Gmail أو Inbox by Gmail أو Apple Mail أو Outlook أو Thunderbird
settings-shortcuts-single-summary = مفاتيح بلا Ctrl أو Alt، كما في بريد الويب
settings-default-apps-pdf-summary = مكان فتح مرفقات PDF
settings-default-apps-pictures-summary = مكان فتح الصور الفوتوغرافية والصور
settings-default-apps-text-summary = مكان فتح النص العادي والسجلات والتعليمات البرمجية
settings-default-apps-sheets-summary = مكان فتح ملفات Excel وOpenDocument وCSV
settings-default-apps-documents-summary = مكان فتح مستندات Word ونصوص OpenDocument
settings-default-apps-after-saving-summary = عرض المرفقات المحفوظة في مجلدها
settings-compose-send-from-summary = الحساب الذي يُرسَل منه البريد الجديد: الحساب الذي تستخدمه، أو الحساب نفسه دائمًا
settings-compose-send-on-replies-summary = «إرسال» أو «إرسال وأرشفة» المحادثة، في الردود وإعادة التوجيه
settings-compose-signatures-summary = يُضاف أسفل رسالتك، بعد سطر «--»
settings-compose-for-new-mail-summary = التوقيع الذي يبدأ به البريد الجديد
settings-compose-for-replies-summary = التوقيع الذي تبدأ به الردود وإعادات التوجيه
settings-compose-format-summary = كتابة البريد الجديد بنص عادي
settings-compose-spelling-summary = التدقيق الإملائي أثناء الكتابة، ولغة القاموس
settings-compose-templates-summary = قريبًا: احفظ الرسائل التي تكتبها كثيرًا، وابدأ منها رسالة جديدة أو ردًا
settings-feedback-crash-reports-summary = حفظ تقارير الأعطال على هذا الكمبيوتر عند تعطّل Katna Mail أو خدمته في الخلفية
settings-feedback-saved-summary = عرض تقارير الأعطال المحفوظة على هذا الكمبيوتر أو نسخها أو حذفها
settings-feedback-help-improve-summary = إرسال تقارير الأعطال للمساعدة في إصلاح الخلل؛ متوقف ما لم تفعّله
settings-experimental-blur-summary = يظهر سطح المكتب عبر الشريط العلوي بشكل ضبابي، وتبدو القوائم كزجاج مصنفر
settings-search-shortcut = اختصار لوحة المفاتيح
settings-search-tab = علامة تبويب في الإعدادات
settings-search-none = لا توجد إعدادات مطابقة لـ«{ $query }».
settings-search-results = الإعدادات المطابقة لـ«{ $query }»

## Quick settings (the panel that slides in from the right)

quick-title = الإعدادات السريعة
quick-see-all = عرض كل الإعدادات
quick-reading-pane = جزء القراءة
quick-pane-right = بجانب القائمة
quick-pane-none = بلا تقسيم
quick-density = الكثافة
quick-density-default = تلقائية
quick-density-compact = مضغوطة
quick-theme = السمة
quick-theme-system = مثل سطح المكتب
quick-theme-light = فاتحة
quick-theme-dark = داكنة
quick-desktop-colors = ألوان سطح المكتب
quick-desktop-colors-detail = نظام الألوان ولون التمييز في سطح المكتب
quick-app-names = أسماء التطبيقات
quick-app-names-detail = الأسماء أسفل أيقونات التطبيقات في أقصى اليمين
quick-inbox-tabs = علامات تبويب البريد الوارد
quick-inbox-tabs-detail = علامات التبويب لدى مزوّد بريد كل حساب
quick-choose-tabs = اختيار علامات التبويب
quick-choose-tabs-detail = لكل حساب، في الإعدادات
quick-sending = الإرسال
quick-undo-send = التراجع عن الإرسال
quick-undo-send-off = متوقف
quick-undo-send-seconds = { $seconds } ث
quick-signatures = التوقيعات
quick-signatures-none = لا توجد بعد
quick-signatures-one = { $name }، يُستخدم تلقائيًا
quick-signatures-many = { $count ->
    [zero] { $count } توقيع؛ { $name } تلقائيًا
    [one] توقيع واحد؛ { $name } تلقائيًا
    [two] توقيعان؛ { $name } تلقائيًا
    [few] { $count } توقيعات؛ { $name } تلقائيًا
    [many] { $count } توقيعًا؛ { $name } تلقائيًا
   *[other] { $count } توقيع؛ { $name } تلقائيًا
}
quick-signatures-no-default = { $count ->
    [zero] { $count }، بلا توقيع تلقائي
    [one] { $count }، بلا توقيع تلقائي
    [two] { $count }، بلا توقيع تلقائي
    [few] { $count }، بلا توقيع تلقائي
    [many] { $count }، بلا توقيع تلقائي
   *[other] { $count }، بلا توقيع تلقائي
}
quick-signature-untitled = بلا عنوان
quick-threading = سلاسل الرسائل
quick-conversation-view = عرض المحادثات
quick-conversation-view-detail = تجميع الردود على الرسالة نفسها
quick-help = المساعدة
quick-tour = بدء الجولة التعريفية
quick-whats-new = ما الجديد
quick-about = حول Katna

## Settings: opening at login

settings-open-at-login-failed = تعذّر تغيير الفتح عند تسجيل الدخول: { $error }

## Settings > Appearance > Scaling

scale-letter = ع
scale-percent = { $percent }٪
scale-reset = العودة إلى { $percent }٪

## Settings > Experimental > Look & Feel

look-intro = ميزات لا تزال قيد التجربة. قد تتغير أو تختفي.
look-heading = الشكل والمظهر
look-window-frame = إطار النافذة
look-window-frame-detail = مَن يرسم شريط العنوان وأزرار النافذة والزوايا والظل.
look-frame-native-kde = أصلي: إطار KDE، بسمة Plasma الخاصة بك
look-frame-native = أصلي: إطار سطح المكتب
look-frame-katna = Katna: يصبح الشريط العلوي شريط العنوان
look-frame-katna-note-named = يرسم Katna زوايا مستديرة وظلًا خاصًا به. لم يعد الإطار يتبع سمة { $desktop }؛ لكن قواعد النوافذ لا تزال سارية.
look-frame-katna-note = يرسم Katna زوايا مستديرة وظلًا خاصًا به. لم يعد الإطار يتبع سمة سطح المكتب؛ لكن قواعد النوافذ لا تزال سارية.
look-frame-client-side = يترك سطح المكتب لديك الإطار لكل تطبيق، لذا يرسم Katna إطاره الخاص بالفعل.
look-blurred-background = خلفية ضبابية
look-blurred-background-detail = يظهر سطح المكتب عبر الشريط العلوي والمجلدات بشكل ضبابي، وتبدو القوائم والنوافذ المنبثقة كزجاج مصنفر.
look-blur = إضفاء ضبابية على ما خلف النافذة
look-blur-detail = يبقى البريد على بطاقات مُصمتة، ليحتفظ النص بتباينه
look-blur-off-kde = تأثير الضبابية في KDE متوقف. فعّل «الضبابية» في «إعدادات النظام»، «إدارة النوافذ»، «تأثيرات سطح المكتب»، ثم افتح Katna Mail مجددًا.
look-blur-none-gnome = لا يضفي GNOME ضبابية على ما خلف النوافذ.
look-blur-none-x11 = لا يضفي مدير النوافذ لديك ضبابية على ما خلف النوافذ.
look-blur-none-wayland = لا يضفي المُركِّب لديك ضبابية على ما خلف النوافذ.

## Settings > User feedback (crash reports)

feedback-intro-sending = يتم إرسال تقارير الأعطال الجديدة للمساعدة في إصلاح الخلل. لا يغادر أي شيء آخر هذا الكمبيوتر.
feedback-intro-local = لا يرسل Katna أي شيء إلى أي مكان. تبقى تقارير الأعطال على هذا الكمبيوتر، لتطّلع عليها أو ترفقها ببلاغ عن خطأ.
feedback-crash-reports = تقارير الأعطال
feedback-crash-reports-detail = تُكتب عند تعطّل Katna Mail أو خدمته في الخلفية.
feedback-save = حفظ تقارير الأعطال على هذا الكمبيوتر
feedback-save-detail = يُستبعد منها مجلد المنزل واسم المستخدم واسم الكمبيوتر وعناوين البريد الإلكتروني
feedback-saved = تقارير الأعطال المحفوظة
feedback-saved-detail = { $count ->
    [zero] يُحتفظ بأحدث { $count } تقرير.
    [one] يُحتفظ بأحدث تقرير فقط.
    [two] يُحتفظ بأحدث تقريرين.
    [few] يُحتفظ بأحدث { $count } تقارير.
    [many] يُحتفظ بأحدث { $count } تقريرًا.
   *[other] يُحتفظ بأحدث { $count } تقرير.
}
feedback-help-improve = المساعدة في تحسين Katna
feedback-help-improve-detail = متوقف ما لم تفعّله، ويمكنك إيقافه هنا في أي وقت.
feedback-send = إرسال تقارير الأعطال
feedback-send-detail = يُرسَل التقرير المحفوظ، تمامًا كما يمكنك عرضه هنا، إلى متتبّع الأعطال لدى Katna (Sentry، في الاتحاد الأوروبي). بلا عنوان IP أو رسائل أو عناوين بريد إلكتروني
feedback-none-saved = لا توجد تقارير أعطال محفوظة.
feedback-delete-all = حذف الكل
feedback-app-daemon = الخدمة في الخلفية
feedback-report-sent = { $date } · تم الإرسال
feedback-view = عرض
feedback-view-tooltip = فتح التقرير
feedback-copy-tooltip = نسخه للصقه في بلاغ عن خطأ
feedback-copied = تم نسخ تقرير العطل.
feedback-deleted-all = تم حذف تقارير الأعطال.
feedback-read-failed = تعذّرت قراءة تقرير العطل: { $error }
feedback-delete-failed = تعذّر حذف تقرير العطل: { $error }
feedback-delete-all-failed = تعذّر حذف تقارير الأعطال: { $error }

## Menu bar (the KDE global menu)

desktop-menu-file = _ملف
desktop-menu-new-message = _رسالة جديدة
desktop-menu-quit = _إنهاء
desktop-menu-edit = _تحرير
desktop-menu-undo = _تراجع
desktop-menu-select-all = _تحديد الكل
desktop-menu-select-none = _إلغاء التحديد
desktop-menu-find = _بحث…
desktop-menu-view = _عرض
desktop-menu-folder-list = _إظهار قائمة المجلدات
desktop-menu-refresh = _إعادة التحميل
desktop-menu-go = _انتقال
desktop-menu-inbox = _البريد الوارد
desktop-menu-starred = _المميّزة بنجمة
desktop-menu-sent = _المُرسَلة
desktop-menu-drafts = _المسودات
desktop-menu-all-mail = _كل البريد
desktop-menu-next = _المحادثة التالية
desktop-menu-previous = _المحادثة السابقة
desktop-menu-message = _رسالة
desktop-menu-open = _فتح
desktop-menu-reply = _رد
desktop-menu-reply-all = _الرد على الكل
desktop-menu-forward = _إعادة توجيه
desktop-menu-archive = _أرشفة
desktop-menu-delete = _حذف
desktop-menu-spam = _الإبلاغ عن محتوى غير مرغوب فيه
desktop-menu-move-to = _نقل إلى…
desktop-menu-mark-read = _وضع علامة «مقروءة»
desktop-menu-mark-unread = _وضع علامة «غير مقروءة»
desktop-menu-star = _إضافة نجمة
desktop-menu-important = _وضع علامة «مهمة»
desktop-menu-not-important = _وضع علامة «غير مهمة»
desktop-menu-settings = _الإعدادات
desktop-menu-quick-settings = _الإعدادات السريعة
desktop-menu-configure = _ضبط Katna Mail…
desktop-menu-help = _مساعدة
desktop-menu-shortcuts = _اختصارات لوحة المفاتيح
desktop-menu-whats-new = _ما الجديد
desktop-menu-about = _حول Katna

## Settings > Keyboard shortcuts: the groups of the list

shortcut-group-moving = التنقل
shortcut-group-actions = الإجراءات
shortcut-group-go-to = الانتقال إلى
shortcut-group-app = التطبيق

## Settings > Keyboard shortcuts: what each shortcut does

shortcut-next = المحادثة التالية
shortcut-previous = المحادثة السابقة
shortcut-down = النزول في القائمة
shortcut-up = الصعود في القائمة
shortcut-first = الأولى في القائمة
shortcut-last = الأخيرة في القائمة
shortcut-page-down = صفحة لأسفل في القائمة
shortcut-page-up = صفحة لأعلى في القائمة
shortcut-open = فتح المحادثة
shortcut-back = العودة إلى القائمة
shortcut-scroll-down = التمرير لأسفل
shortcut-scroll-up = التمرير لأعلى
shortcut-scroll-page-down = التمرير صفحة لأسفل
shortcut-scroll-page-up = التمرير صفحة لأعلى
shortcut-compose = إنشاء
shortcut-reply = رد
shortcut-reply-all = الرد على الكل
shortcut-forward = إعادة توجيه
shortcut-archive = أرشفة
shortcut-delete = حذف
shortcut-spam = الإبلاغ عن محتوى غير مرغوب فيه
shortcut-move-to = نقل إلى
shortcut-mark-read = وضع علامة «مقروءة»
shortcut-mark-unread = وضع علامة «غير مقروءة»
shortcut-star = إضافة نجمة أو إزالتها
shortcut-important = وضع علامة «مهمة»
shortcut-not-important = وضع علامة «غير مهمة»
shortcut-check = تحديد المحادثة
shortcut-select-all = تحديد كل المحادثات
shortcut-select-none = إلغاء تحديد كل المحادثات
shortcut-undo = التراجع عن آخر إجراء
shortcut-go-inbox = البريد الوارد
shortcut-go-starred = المميّزة بنجمة
shortcut-go-sent = المُرسَلة
shortcut-go-drafts = المسودات
shortcut-go-all = كل البريد
shortcut-search = البحث في البريد
shortcut-navigation = إظهار القائمة أو طيّها
shortcut-quick-settings = الإعدادات السريعة
shortcut-settings = كل الإعدادات
shortcut-shortcuts = اختصارات لوحة المفاتيح
shortcut-reload = التحقق من وجود بريد جديد
shortcut-quit = إنهاء

## Keys pressed one after another, as a shortcut shows them ("G then I")

shortcut-sequence = { $first } ثم { $second }

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
