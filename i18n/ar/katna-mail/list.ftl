# Katna Mail, Arabic (العربية).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

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
