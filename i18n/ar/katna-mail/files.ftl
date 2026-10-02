# Katna Mail, Arabic (العربية).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = البحث في الملفات

## Left side (and chips on a phone)

files-all = كل الملفات
files-pictures = الصور
files-pdfs = ملفات PDF
files-documents = المستندات
files-sheets = جداول البيانات
files-slides = العروض التقديمية
files-other = أخرى
files-accounts = الحسابات
files-drives = المحركات
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = تمت مشاركتها معي
files-shown = المعروضة
files-received = المستلَمة
files-sent = المُرسَلة مني

## Over the files

files-count = { $count ->
    [zero] { $count } ملف · { $size }
    [one] ملف واحد · { $size }
    [two] ملفان · { $size }
    [few] { $count } ملفات · { $size }
    [many] { $count } ملفًا · { $size }
   *[other] { $count } ملف · { $size }
}
files-anyone = أي شخص
files-from-person = من { $name }
files-time-any = أي وقت
files-time-today = اليوم
files-time-yesterday = أمس
files-time-this-week = هذا الأسبوع
files-time-last-week = الأسبوع الماضي
files-time-this-month = هذا الشهر
files-time-last-month = الشهر الماضي
files-time-between = { $first } – { $last }
files-time-hint = انقر على يوم، أو اسحب عبر الأيام
files-time-summary = { $count ->
    [zero] { $days } · { $count } ملف
    [one] { $days } · ملف واحد
    [two] { $days } · ملفان
    [few] { $days } · { $count } ملفات
    [many] { $days } · { $count } ملفًا
   *[other] { $days } · { $count } ملف
}
files-time-clear = مسح
files-time-month-back = الشهر السابق
files-time-month-on = الشهر التالي
files-time-wheel = مرّر لتحريك هذه التواريخ مع الحفاظ على طول المدة
files-sort-newest = الأحدث أولًا
files-sort-oldest = الأقدم أولًا
files-sort-largest = الأكبر أولًا
files-sort-name = حسب الاسم
files-grid = بطاقات
files-list = قائمة
files-this-week = هذا الأسبوع
files-undated = بلا تاريخ
files-me = أنا
files-no-subject = (بلا موضوع)
files-loading = جارٍ جمع الملفات من بريدك…
files-empty = تظهر هنا الملفات الواردة في بريدك.
files-none-match = لا توجد ملفات مطابقة.
files-load-failed = تعذّرت قراءة الملفات: { $error }

## A file's menu and buttons

files-open = فتح
files-open-with = الفتح باستخدام…
files-save = حفظ…
files-show-mail = عرض الرسالة
files-mail-window = فتح الرسالة في نافذة جديدة
files-forward = إعادة توجيه الملف
files-from-them = ملفات من { $name }
files-copy-name = نسخ اسم الملف
files-name-copied = تم نسخ اسم الملف
files-downloading = جارٍ تنزيل الرسالة…
files-download-failed = تعذّر تنزيل هذه الرسالة.

## A cloud drive in place of the mail files

files-drive-mine = My Drive
files-drive-mine-onedrive = ملفاتي
files-drive-results = «{ $words }»
files-drive-count = { $folders ->
    [0] { $files ->
        [zero] { $files } ملف
        [one] ملف واحد
        [two] ملفان
        [few] { $files } ملفات
        [many] { $files } ملفًا
       *[other] { $files } ملف
    }
    [one] مجلد واحد · { $files ->
        [zero] { $files } ملف
        [one] ملف واحد
        [two] ملفان
        [few] { $files } ملفات
        [many] { $files } ملفًا
       *[other] { $files } ملف
    }
    [two] مجلدان · { $files ->
        [zero] { $files } ملف
        [one] ملف واحد
        [two] ملفان
        [few] { $files } ملفات
        [many] { $files } ملفًا
       *[other] { $files } ملف
    }
    [few] { $folders } مجلدات · { $files ->
        [zero] { $files } ملف
        [one] ملف واحد
        [two] ملفان
        [few] { $files } ملفات
        [many] { $files } ملفًا
       *[other] { $files } ملف
    }
    [many] { $folders } مجلدًا · { $files ->
        [zero] { $files } ملف
        [one] ملف واحد
        [two] ملفان
        [few] { $files } ملفات
        [many] { $files } ملفًا
       *[other] { $files } ملف
    }
   *[other] { $folders } مجلد · { $files ->
        [zero] { $files } ملف
        [one] ملف واحد
        [two] ملفان
        [few] { $files } ملفات
        [many] { $files } ملفًا
       *[other] { $files } ملف
    }
}
files-drive-folders = المجلدات
files-drive-files = الملفات
files-drive-folder = مجلد
files-drive-meta = { $what } · آخر تعديل { $date }
files-drive-as-link = { $what } · كرابط
files-drive-google-doc = مستند Google
files-drive-google-sheet = جدول بيانات Google
files-drive-google-slides = عرض تقديمي من Google
files-drive-google-drawing = رسم Google
files-drive-fetching = جارٍ الجلب…
files-drive-loading = جارٍ فتح المحرك…
files-drive-empty = هذا المجلد فارغ.
files-drive-unreachable = تعذّر الوصول إلى { $drive }.
files-drive-try-again = المحاولة مجددًا
files-drive-needs-permission = يحتاج Katna إلى إذنك مرة واحدة لعرض هذا المحرك. سجّل الدخول مجددًا واسمح لـ Katna برؤية ملفاتك.
files-drive-allow = السماح
files-drive-allow-failed = لم يكتمل تسجيل الدخول، لذا يبقى المحرك مغلقًا.
files-drive-attach = إرفاق
files-drive-more = المزيد
files-drive-download = تنزيل…
files-drive-open-web = الفتح في { $drive }
files-drive-copy-link = نسخ الرابط
files-drive-link-copied = تم نسخ الرابط
files-drive-share = مشاركة…
files-drive-rename = إعادة التسمية
files-drive-trash = النقل إلى المهملات
files-drive-trashed = «{ $name }» في مهملات { $drive }
files-drive-renamed = تمت إعادة التسمية إلى «{ $name }»
files-drive-getting = جارٍ جلب { $name } من { $drive }…
files-drive-get-failed = تعذّر جلب { $name }: { $error }
files-drive-upload = رفع
files-drive-upload-files = رفع ملفات
files-drive-upload-folder = رفع مجلد
files-drive-upload-failed = تعذّر رفع { $name }: { $error }
files-drive-upload-needs = للرفع، يحتاج Katna إلى إذنك مرة واحدة: اضغط «السماح» في الإعدادات › التطبيقات التلقائية › صفحة «الملفات».

## The Share dialog of a drive file or folder

files-share-title = مشاركة «{ $name }»
files-share-add = أضف أشخاصًا بالاسم أو العنوان
files-share-not-address = «{ $text }» ليس عنوان بريد إلكتروني
files-share-notify = السماح لـ { $drive } بمراسلتهم أيضًا
files-share-people = الأشخاص الذين لديهم حق الوصول
files-share-general = الوصول العام
files-share-loading = جارٍ قراءة من لديه حق الوصول…
files-share-restricted = مقيَّد
files-share-restricted-about = يمكن فقط للأشخاص الذين لديهم حق الوصول فتحه عبر الرابط
files-share-anyone = أي شخص لديه الرابط
files-share-anyone-can = { $role ->
    [editor] يمكن لأي شخص لديه الرابط التعديل
    [commenter] يمكن لأي شخص لديه الرابط التعليق
   *[viewer] يمكن لأي شخص لديه الرابط العرض
}
files-share-anyone-about = { $role ->
    [editor] يمكن لأي شخص على الإنترنت لديه الرابط التعديل
    [commenter] يمكن لأي شخص على الإنترنت لديه الرابط التعليق
   *[viewer] يمكن لأي شخص على الإنترنت لديه الرابط العرض
}
files-share-role-owner = المالك
files-share-role-editor = محرِّر
files-share-role-commenter = معلِّق
files-share-role-viewer = مُشاهِد
files-share-you = { $name } (أنت)
files-share-domain = الجميع في { $domain }
files-share-inherited = وصول من مجلد يوجد فيه
files-share-remove = إزالة حق الوصول
files-share-copy-link = نسخ الرابط
files-share-share = مشاركة
files-share-done = تم
files-share-close = إغلاق
files-share-sharing = جارٍ المشاركة…
files-share-shared = { $count ->
    [zero] تمت المشاركة مع { $count } شخص
    [one] تمت المشاركة مع شخص واحد
    [two] تمت المشاركة مع شخصين
    [few] تمت المشاركة مع { $count } أشخاص
    [many] تمت المشاركة مع { $count } شخصًا
   *[other] تمت المشاركة مع { $count } شخص
}
files-share-refused = تعذّر على { $drive } المشاركة مع { $addresses }
files-share-failed = تعذّر تغيير المشاركة: { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
    [zero] جارٍ رفع { $count } عنصر
    [one] جارٍ رفع عنصر واحد
    [two] جارٍ رفع عنصرين
    [few] جارٍ رفع { $count } عناصر
    [many] جارٍ رفع { $count } عنصرًا
   *[other] جارٍ رفع { $count } عنصر
}
files-tray-done = { $count ->
    [zero] اكتمل رفع { $count } عنصر
    [one] اكتمل رفع عنصر واحد
    [two] اكتمل رفع عنصرين
    [few] اكتمل رفع { $count } عناصر
    [many] اكتمل رفع { $count } عنصرًا
   *[other] اكتمل رفع { $count } عنصر
}
files-tray-some-failed = تم رفع { $done }، وفشل { $failed }
files-tray-minutes-left = { $minutes ->
    [zero] تبقّى نحو { $minutes } دقيقة
    [one] تبقّت دقيقة تقريبًا
    [two] تبقّت دقيقتان تقريبًا
    [few] تبقّى نحو { $minutes } دقائق
    [many] تبقّى نحو { $minutes } دقيقة
   *[other] تبقّى نحو { $minutes } دقيقة
}
files-tray-seconds-left = تبقّى أقل من دقيقة
files-tray-starting = جارٍ البدء…
files-tray-cancel-all = إلغاء الكل
files-tray-cancel = إلغاء
files-tray-fold = إخفاء القائمة
files-tray-unfold = إظهار القائمة
files-tray-close = إغلاق
files-tray-progress = { $place } · { $sent } من { $size }
files-tray-in = في { $place }
files-tray-cancelled = تم الإلغاء
