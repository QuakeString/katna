# Katna Mail, Arabic (العربية): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = جهات الاتصال
contacts-frequent = الأكثر تواصلاً
contacts-labels = التصنيفات
contacts-create = إنشاء جهة اتصال

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
contacts-deleted = تم حذف { $name }
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
