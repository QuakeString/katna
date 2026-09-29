# Katna Mail, Arabic (العربية): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = اليوم
calendar-today-tip = الانتقال إلى اليوم
calendar-view-day = يوم
calendar-view-week = أسبوع
calendar-view-month = شهر
calendar-view-year = السنة
calendar-view-schedule = جدول زمني
calendar-view-days =
    { $count ->
        [zero] { $count } يوم
        [one] يوم واحد
        [two] يومان
        [few] { $count } أيام
        [many] { $count } يومًا
       *[other] { $count } يوم
    }
calendar-options = خيارات
calendar-density = الكثافة
calendar-density-responsive = متجاوب مع شاشتك
calendar-density-comfortable = مريحة
calendar-density-compact = مضغوطة
calendar-custom-days = عرض مخصص
calendar-second-zone = المنطقة الزمنية الثانية
calendar-zone-none = بدون
calendar-zone = { $zone } ({ $offset })
calendar-share-free = مشاركة الأوقات الفارغة
calendar-free-subject = أوقاتي الفارغة
calendar-free-intro = إليك بعض الأوقات التي أكون فيها متفرغًا ({ $zone }):
calendar-free-day = { $weekday } { $date }: { $times }
calendar-free-range = { $start } – { $end }
calendar-free-none = ليس لدي وقت فارغ في أيام العمل القليلة القادمة.
calendar-previous-day = اليوم السابق
calendar-next-day = اليوم التالي
calendar-previous-week = الأسبوع السابق
calendar-next-week = الأسبوع التالي
calendar-previous-month = الشهر السابق
calendar-next-month = الشهر التالي
calendar-previous-year = السنة السابقة
calendar-next-year = السنة التالية
calendar-previous-period = الأقدم
calendar-next-period = الأحدث
calendar-title-months = { $first } – { $last }
calendar-loading = جارٍ التحميل…
calendar-read-failed = تعذّرت قراءة التقويم: { $error }
calendar-sets = مجموعات التقويمات
calendar-set-add = حفظ التقويمات المعروضة كمجموعة
calendar-set-name = اسم المجموعة
calendar-set-remove = إزالة المجموعة
calendar-local = هذا الكمبيوتر
calendar-account-gone = حساب محذوف
calendar-account-sign-in = سجّل الدخول مجددًا لإظهار التقاويم
calendar-account-signed-in = تم تسجيل الدخول إلى { $address } مجددًا. جارٍ جلب تقاويمك…
calendar-account-sign-in-refused = لم يسمح { $provider } لـ Katna بالدخول. حاول مجددًا، واسمح بالوصول إلى تقاويمك.
calendar-account-refused = لم يسمح الخادم لـ Katna بالوصول إلى التقاويم.
calendar-account-not-enabled = لم يُفعَّل الوصول إلى التقويم لـ Katna بعد.
calendar-account-failed = تعذّرت قراءة التقاويم.
calendar-account-error = تعذّرت قراءة التقاويم: { $reason }
calendar-account-none = لم يُعثر على أي تقويم
calendar-account-looking = جارٍ البحث عن التقاويم…
calendar-account-try-again = إعادة المحاولة
calendar-account-try-again-tooltip = التحقق من تقاويم هذا الحساب مجددًا الآن
calendar-account-fixing = جارٍ العمل على ذلك…
calendar-birthdays = أعياد الميلاد
calendar-birthday-of = عيد ميلاد { $name }
calendar-empty-title = لا توجد تقاويم بعد
calendar-empty-text = تظهر هنا تقاويم حساباتك على Google وMicrosoft بعد مزامنتها، وكذلك تقاويم الخوادم الأخرى التي تدعم CalDAV.
calendar-schedule-empty = لا شيء مخطط له خلال الشهرين القادمين.
calendar-search = البحث في الأحداث
calendar-search-past = الأحداث السابقة
calendar-search-none = لا توجد أحداث تطابق بحثك.
calendar-no-title = (بلا عنوان)
calendar-all-day = طوال اليوم
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }، { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = { $count } أخرى
calendar-repeats = تكرار
calendar-join = انضمام
calendar-email-guests = إرسال رسالة إلكترونية إلى الضيوف
calendar-running-late = سأتأخر
calendar-late-subject = سأتأخر: { $title }
calendar-late-body = أعتذر، سأتأخر بضع دقائق عن { $title }. سأصل قريبًا.
calendar-guests =
    { $count ->
        [zero] لا ضيوف
        [one] { $count } ضيف
        [two] ضيفان
        [few] { $count } ضيوف
        [many] { $count } ضيفًا
       *[other] { $count } ضيف
    }
calendar-guest-answers = { $yes } نعم، { $maybe } ربما، { $no } لا، { $waiting } في الانتظار
calendar-organizer = المنظِّم
calendar-optional = اختياري
calendar-open-web = فتح في المتصفح
calendar-open-contact = فتح جهة الاتصال
calendar-close = إغلاق

## Adding, changing and deleting events.

calendar-add-title = إضافة عنوان
calendar-add-location = إضافة موقع
calendar-add-notes = إضافة وصف
calendar-add-guests = إضافة ضيوف
calendar-remove-guest = إزالة
calendar-add-meet = إضافة مؤتمر عبر الفيديو من Google Meet
calendar-add-teams = إضافة اجتماع Teams
calendar-has-call = تمت إضافة مكالمة فيديو
calendar-weekday-day = { $weekday }، { $day }
calendar-all-day-box = طوال اليوم
calendar-more-options = خيارات إضافية
calendar-save = حفظ
calendar-saved = تم حفظ الحدث
calendar-deleted = تم حذف الحدث
calendar-discard = تجاهل التغييرات
calendar-edit = تعديل الحدث
calendar-delete = حذف الحدث
calendar-event-details = تفاصيل الحدث
calendar-kind-event = حدث
calendar-kind-focus = وقت التركيز
calendar-kind-out-of-office = خارج المكتب
calendar-kind-working-location = موقع العمل
calendar-working-home = المنزل
calendar-busy = مشغول
calendar-free = متاح
calendar-cancel = إلغاء
calendar-ok = حسنًا
calendar-read-only = لا يمكنك تغيير الأحداث في هذا التقويم
calendar-none-editable = لا يوجد بعد تقويم يمكنك إضافة أحداث إليه
calendar-no-such-time = هذا الوقت غير موجود في منطقتك الزمنية
calendar-end-before-start = ينتهي الحدث قبل أن يبدأ
calendar-repeat-never = لا يتكرر
calendar-repeat-daily = يوميًا
calendar-repeat-weekly = أسبوعيًا يوم { $weekday }
calendar-repeat-monthly =
    { $nth ->
        [1] شهريًا في { $weekday } الأول
        [2] شهريًا في { $weekday } الثاني
        [3] شهريًا في { $weekday } الثالث
        [4] شهريًا في { $weekday } الرابع
       *[other] شهريًا في { $weekday } الأخير
    }
calendar-repeat-yearly = سنويًا في { $day }
calendar-repeat-weekdays = كل يوم من أيام الأسبوع (من الاثنين إلى الجمعة)
calendar-repeat-custom = مخصّص
calendar-reminder-none = بدون إشعار
calendar-reminder-at-start = عند البدء
calendar-reminder-minutes =
    { $count ->
        [zero] قبل { $count } دقيقة
        [one] قبل دقيقة واحدة
        [two] قبل دقيقتين
        [few] قبل { $count } دقائق
        [many] قبل { $count } دقيقة
       *[other] قبل { $count } دقيقة
    }
calendar-reminder-hours =
    { $count ->
        [zero] قبل { $count } ساعة
        [one] قبل ساعة واحدة
        [two] قبل ساعتين
        [few] قبل { $count } ساعات
        [many] قبل { $count } ساعة
       *[other] قبل { $count } ساعة
    }
calendar-reminder-days =
    { $count ->
        [zero] قبل { $count } يوم
        [one] قبل يوم واحد
        [two] قبل يومين
        [few] قبل { $count } أيام
        [many] قبل { $count } يومًا
       *[other] قبل { $count } يوم
    }
calendar-scope-edit-title = تعديل حدث متكرر
calendar-scope-delete-title = حذف حدث متكرر
calendar-scope-this = هذا الحدث
calendar-scope-following = هذا الحدث والأحداث التالية
calendar-scope-all = كل الأحداث
calendar-scope-respond-title = الرد على حدث متكرر
calendar-going = هل ستحضر؟
calendar-answer-yes = نعم
calendar-answer-no = لا
calendar-answer-maybe = ربما
calendar-answered-yes = ستحضر
calendar-answered-no = لن تحضر
calendar-answered-maybe = ربما تحضر

## The card at the top of a mail with an invitation.

calendar-invite = دعوة
calendar-invite-cancelled = أُلغي الحدث
calendar-invite-reply = { $name }: ردّ
calendar-invite-reply-yes = { $name }: قبول
calendar-invite-reply-no = { $name }: رفض
calendar-invite-reply-maybe = { $name }: ربما
calendar-invite-organizer = المنظِّم: { $name }
calendar-invite-open = فتح في التقويم
calendar-invite-not-yet = ليس في تقويمك بعد. يمكنك الرد بعد المزامنة.
calendar-invite-by-mail = ليس في تقويمك: سيصل ردك إلى المنظّم بالبريد.
calendar-mail-yes = تم القبول: { $title }
calendar-mail-yes-body = تم قبول هذه الدعوة من { $name }.
calendar-mail-no = تم الرفض: { $title }
calendar-mail-no-body = تم رفض هذه الدعوة من { $name }.
calendar-mail-maybe = مبدئي: { $title }
calendar-mail-maybe-body = تم قبول هذه الدعوة مبدئيًا من { $name }.
calendar-invite-your-day = يومك
calendar-invite-clashes =
    { $count ->
        [zero] يتعارض مع { $count } حدث
        [one] يتعارض مع { $count } حدث
        [two] يتعارض مع { $count } حدثين
        [few] يتعارض مع { $count } أحداث
        [many] يتعارض مع { $count } حدثًا
       *[other] يتعارض مع { $count } حدث
    }

## The day's agenda beside the mail.

agenda-show = عرض أجندة اليوم
agenda-hide = إخفاء الأجندة
agenda-today = اليوم، { $date }
agenda-day = { $weekday }، { $date }
agenda-empty = لا شيء مخطط له في هذا اليوم.
