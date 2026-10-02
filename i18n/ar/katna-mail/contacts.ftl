# Katna Mail, Arabic (العربية): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = جهات الاتصال
contacts-frequent = الأكثر تواصلاً
contacts-other = جهات الاتصال الأخرى
contacts-other-about = أشخاص راسلتهم من Gmail ولكن لم تحفظهم
contacts-other-email = إرسال رسالة إلكترونية
contacts-other-empty = لا توجد جهات اتصال أخرى. سيظهر هنا الأشخاص الذين تراسلهم من Gmail ولا تحفظهم.
contacts-other-allow = لعرض جهات الاتصال الأخرى، سجّل الدخول إلى حساب Gmail مرة أخرى واسمح لـ Katna بعرضها.
contacts-labels = التصنيفات
contacts-label-options = خيارات التصنيف
contacts-label-rename = إعادة تسمية التصنيف
contacts-label-email = إرسال بريد إلى الجميع
contacts-label-delete = حذف التصنيف
contacts-label-new = تصنيف جديد
contacts-label-name = اسم التصنيف
contacts-label-button = تصنيف
contacts-label-menu = تصنيف باسم:
contacts-label-added = تمت الإضافة إلى { $name }
contacts-label-removed = تمت الإزالة من { $name }
contacts-label-renamed = تمت إعادة تسمية التصنيف إلى { $name }
contacts-label-deleted = تم حذف التصنيف { $name }
contacts-label-no-email = لا أحد في هذا التصنيف لديه عنوان بريد إلكتروني
# The column's list of mail accounts, each with the people saved in it.
contacts-accounts = الحسابات
# The line under an account whose contacts did not come: why, and the one
# click that fixes it.
contacts-account-sign-in = سجّل الدخول مجددًا لإظهار جهات الاتصال
contacts-account-signed-in = تم تسجيل الدخول إلى { $address } مجددًا. جارٍ جلب جهات اتصالك…
contacts-account-sign-in-refused = لم يسمح { $provider } لـ Katna بالدخول. حاول مجددًا، واسمح بالوصول إلى جهات اتصالك.
contacts-account-password = لم يقبل الخادم كلمة المرور. تحتاج Yahoo وiCloud وZoho وغيرها إلى كلمة مرور للتطبيقات.
contacts-account-change-password = تغيير كلمة المرور
contacts-account-change-password-tooltip = فتح الإعدادات > الحسابات
contacts-account-failed = تعذّرت قراءة جهات الاتصال.
# $reason is the server's own words, in English.
contacts-account-error = تعذّرت قراءة جهات الاتصال: { $reason }
contacts-account-none = لم يُعثر على دفتر عناوين
# $reason is what the server answered, in English: "CardDAV https://dav.example.in/: status 404".
contacts-account-none-why = لم يُعثر على دفتر عناوين: { $reason }
# A Gmail or Outlook account added with a password: its contacts need the
# provider's sign-in.
contacts-account-use-sign-in = لا يعرض { $provider } جهات الاتصال إلا لـ Katna المسجَّل دخوله باستخدام { $provider }.
contacts-account-sign-in-with = تسجيل الدخول باستخدام { $provider }
contacts-account-looking = جارٍ البحث عن جهات الاتصال…
contacts-account-try-again = إعادة المحاولة
contacts-account-try-again-tooltip = التحقق من جهات اتصال هذا الحساب مجددًا الآن
contacts-account-fixing = جارٍ العمل على ذلك…
contacts-manage = إصلاح وإدارة
contacts-merge = دمج وإصلاح
contacts-merge-about = { $count ->
    [zero] { $count } اقتراح: جهات اتصال تبدو كأنها الشخص نفسه
    [one] { $count } اقتراح: جهات اتصال تبدو كأنها الشخص نفسه
    [two] { $count } اقتراحان: جهات اتصال تبدو كأنها الشخص نفسه
    [few] { $count } اقتراحات: جهات اتصال تبدو كأنها الشخص نفسه
    [many] { $count } اقتراح: جهات اتصال تبدو كأنها الشخص نفسه
   *[other] { $count } اقتراح: جهات اتصال تبدو كأنها الشخص نفسه
}
contacts-merge-none = لا توجد نسخ مكررة. تظهر هنا جهات الاتصال التي لها الاسم أو رقم الهاتف نفسه.
contacts-merge-count = { $count ->
    [zero] { $count } جهة اتصال
    [one] { $count } جهة اتصال
    [two] { $count } جهتا اتصال
    [few] { $count } جهات اتصال
    [many] { $count } جهة اتصال
   *[other] { $count } جهة اتصال
}
contacts-merge-all = دمج الكل
contacts-merge-button = دمج
contacts-merge-dismiss = تجاهل
contacts-merged = { $count ->
    [1] تم دمج جهات الاتصال
    [zero] تم إجراء { $count } عملية دمج
    [one] تم إجراء { $count } عملية دمج
    [two] تم إجراء { $count } عمليتي دمج
    [few] تم إجراء { $count } عمليات دمج
    [many] تم إجراء { $count } عملية دمج
   *[other] تم إجراء { $count } عملية دمج
}
contacts-import = استيراد
contacts-export = تصدير
contacts-import-file = استيراد جهات الاتصال من ملف vCard أو CSV
contacts-imported = { $count ->
    [zero] تم استيراد { $count } جهة اتصال إلى { $place }
    [one] تم استيراد { $count } جهة اتصال إلى { $place }
    [two] تم استيراد { $count } جهتي اتصال إلى { $place }
    [few] تم استيراد { $count } جهات اتصال إلى { $place }
    [many] تم استيراد { $count } جهة اتصال إلى { $place }
   *[other] تم استيراد { $count } جهة اتصال إلى { $place }
}
contacts-imported-some = { $count ->
    [zero] تم استيراد { $count } جهة اتصال إلى { $place }؛ وتم تجاوز { $skipped } محفوظة مسبقًا
    [one] تم استيراد { $count } جهة اتصال إلى { $place }؛ وتم تجاوز { $skipped } محفوظة مسبقًا
    [two] تم استيراد { $count } جهتي اتصال إلى { $place }؛ وتم تجاوز { $skipped } محفوظة مسبقًا
    [few] تم استيراد { $count } جهات اتصال إلى { $place }؛ وتم تجاوز { $skipped } محفوظة مسبقًا
    [many] تم استيراد { $count } جهة اتصال إلى { $place }؛ وتم تجاوز { $skipped } محفوظة مسبقًا
   *[other] تم استيراد { $count } جهة اتصال إلى { $place }؛ وتم تجاوز { $skipped } محفوظة مسبقًا
}
contacts-import-none = لم يتم العثور على جهات اتصال في { $name }
contacts-import-all-saved = جميع الأشخاص في { $name } محفوظون مسبقًا
contacts-import-failed = تعذّرت قراءة { $name }: { $error }
contacts-exported = { $count ->
    [zero] تم تصدير { $count } جهة اتصال إلى { $path }
    [one] تم تصدير { $count } جهة اتصال إلى { $path }
    [two] تم تصدير { $count } جهتي اتصال إلى { $path }
    [few] تم تصدير { $count } جهات اتصال إلى { $path }
    [many] تم تصدير { $count } جهة اتصال إلى { $path }
   *[other] تم تصدير { $count } جهة اتصال إلى { $path }
}
contacts-export-none = لا توجد جهات اتصال لتصديرها
contacts-export-failed = تعذّر تصدير جهات الاتصال: { $error }
contacts-print = طباعة
contacts-print-title = جهات الاتصال
contacts-print-none = لا توجد جهات اتصال للطباعة
contacts-print-typed = { $value } ({ $kind })
contacts-print-birthday = عيد الميلاد: { $day }
contacts-print-nickname = الاسم المستعار: { $name }
contacts-create = جهة اتصال جديدة

## Search and the list

contacts-search = البحث في جهات الاتصال
contacts-loading = جارٍ تحميل جهات الاتصال…
contacts-empty = لا توجد جهات اتصال محفوظة بعد. تظهر هنا جهات الاتصال التي تحفظها في Gmail أو Outlook أو خدمة البريد لديك.
contacts-empty-no-books = تظهر هنا جهات الاتصال من حساباتك بعد مزامنتها.
contacts-none-found = لا توجد جهات اتصال تطابق بحثك.
contacts-starred = { $count ->
    [zero] جهات الاتصال المميّزة بنجمة ({ $count })
    [one] جهة الاتصال المميّزة بنجمة ({ $count })
    [two] جهتا الاتصال المميّزتان بنجمة ({ $count })
    [few] جهات الاتصال المميّزة بنجمة ({ $count })
    [many] جهات الاتصال المميّزة بنجمة ({ $count })
   *[other] جهات الاتصال المميّزة بنجمة ({ $count })
}
contacts-count = جهات الاتصال ({ $count })
contacts-col-name = الاسم
contacts-col-email = البريد الإلكتروني
contacts-col-phone = رقم الهاتف
contacts-col-job = المسمّى الوظيفي والشركة
contacts-col-labels = التصنيفات

## Asking to allow contacts, for accounts signed in before Katna read them

contacts-allow = السماح لـ Katna بقراءة جهات الاتصال في { $address }.
contacts-allow-many = { $more ->
    [zero] السماح لـ Katna بقراءة جهات الاتصال في { $address }.
    [one] السماح لـ Katna بقراءة جهات الاتصال في { $address } وفي حساب آخر.
    [two] السماح لـ Katna بقراءة جهات الاتصال في { $address } وفي حسابين آخرين.
    [few] السماح لـ Katna بقراءة جهات الاتصال في { $address } وفي { $more } حسابات أخرى.
    [many] السماح لـ Katna بقراءة جهات الاتصال في { $address } وفي { $more } حسابًا آخر.
   *[other] السماح لـ Katna بقراءة جهات الاتصال في { $address } وفي { $more } حساب آخر.
}
contacts-allow-button = السماح

## A contact's page

contacts-back = العودة إلى جهات الاتصال
contacts-edit = تعديل
contacts-delete = حذف
contacts-qr = المشاركة كرمز QR
contacts-qr-about = امسح هذا الرمز بكاميرا الهاتف لحفظ جهة الاتصال.
contacts-qr-too-long = تحتوي جهة الاتصال هذه على تفاصيل أكثر من أن يتسع لها رمز QR.
contacts-qr-done = تم
contacts-deleted = تم حذف { $name }
contacts-added = تمت إضافة { $name } إلى جهات الاتصال
contacts-find-mail = البريد
contacts-details = تفاصيل جهة الاتصال
contacts-saved-in = محفوظة في
contacts-notes = الملاحظات
contacts-birthday = تاريخ الميلاد
contacts-nickname = اللقب
contacts-this-computer = هذا الكمبيوتر
contacts-kind-home = المنزل
contacts-kind-work = العمل
contacts-kind-mobile = الجوال
contacts-kind-other = أخرى
contacts-source-google = جهات اتصال Google
contacts-source-microsoft = جهات اتصال Outlook
contacts-source-carddav = CardDAV

## Creating and changing a contact

contacts-edit-new-title = إنشاء جهة اتصال
contacts-edit-title = تعديل جهة الاتصال
contacts-edit-save = حفظ
contacts-edit-saving = جارٍ الحفظ…
contacts-edit-cancel = إلغاء
contacts-saved = تم حفظ جهة الاتصال
contacts-edit-save-to = الحفظ في
contacts-edit-changes-go-to = يتم حفظ التغييرات في { $place }.
contacts-edit-given = الاسم الأول
contacts-edit-family = اسم العائلة
contacts-edit-company = الشركة
contacts-edit-job = المسمى الوظيفي
contacts-edit-email = البريد الإلكتروني
contacts-edit-phone = الهاتف
contacts-edit-with-kind = { $field } ({ $kind })
contacts-edit-add-email = إضافة بريد إلكتروني
contacts-edit-add-phone = إضافة هاتف
contacts-edit-street = عنوان الشارع
contacts-edit-city = المدينة
contacts-edit-postcode = الرمز البريدي
contacts-edit-country = البلد
contacts-edit-birthday = تاريخ الميلاد (YYYY-MM-DD)
contacts-edit-empty = أضف اسمًا أو بريدًا إلكترونيًا أو رقم هاتف أولًا.
