# Katna Mail, Arabic (العربية).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.
snooze-until = تأجيل حتى…
snooze-later-today = لاحقًا اليوم
snooze-tomorrow = غدًا
snooze-this-weekend = نهاية هذا الأسبوع
snooze-next-week = الأسبوع القادم
snooze-pick = اختيار التاريخ والوقت
snooze-back = العودة إلى الأوقات
snooze-type-placeholder = اكتب وقتًا
snooze-type-hint = مثل «الثلاثاء 3 مساءً» أو «غدًا» أو «بعد ساعتين»
snooze-type-hint-unclear = لا يستطيع Katna فهم هذا كوقت
snooze-type-unclear = «{ $text }» ليس وقتًا يعرفه Katna
snooze-tab = تأجيل
remind-tab = ذكّرني
snooze-says = يُخفيها حتى ذلك الحين
remind-says = يُبقيها مكانها ويُشعرك
remind-before-due = قبل موعد استحقاقها
remind-note = ملاحظة (اختيارية)
remind-note-placeholder = الموضوع، إن تُركت فارغة
toast-remind-set = تم ضبط التذكير على { $date }
remind-chat-line = تذكير { $date } · { $title }
remind-done = تم
toast-remind-done = اكتمل التذكير
snooze-chat-line = مؤجَّلة حتى { $date }
snooze-chat-change = تغيير
snooze-cancel = إلغاء
snooze-save = حفظ
snooze-in-the-past = اختر وقتًا بعد الآن.
follow-up-menu = المتابعة إن لم يصل رد…
follow-up-title = المتابعة إن لم يصل رد
follow-up-off = متوقفة
follow-up-days = { $days ->
    [zero] { $days } يوم
    [one] يوم واحد
    [two] يومان
    [few] { $days } أيام
    [many] { $days } يومًا
   *[other] { $days } يوم
}
follow-up-weeks = { $weeks ->
    [zero] { $weeks } أسبوع
    [one] أسبوع واحد
    [two] أسبوعان
    [few] { $weeks } أسابيع
    [many] { $weeks } أسبوعًا
   *[other] { $weeks } أسبوع
}
follow-up-pick = اختيار…
follow-up-pick-title = المتابعة إن لم يصل رد بحلول
follow-up-remind = ذكّرني
follow-up-remind-note = تعود المحادثة إلى أعلى بريدك الوارد
follow-up-send = إرسال رسالة متابعة نيابةً عني
follow-up-send-note = إلى الأشخاص أنفسهم، في المحادثة نفسها
follow-up-send-encrypted = غير متاح للبريد المشفّر
follow-up-text-placeholder = ما تريد كتابته
follow-up-text-named = مرحبًا { $name }، أردت فقط التأكد من أنك رأيت رسالتي أدناه.
follow-up-text = مرحبًا، أردت فقط التأكد من أنك رأيت رسالتي أدناه.
follow-up-template = استخدام نموذج
follow-up-signature = يُضاف توقيعك
follow-up-again = إن لم يصل رد بعد، تابِع مجددًا بعد
follow-up-note = تتوقف فور رد أي شخص في المحادثة. لا تُحتسَب الردود التلقائية.
follow-up-note-send = تتوقف فور رد أي شخص في المحادثة. تُرسَل في أيام العمل من { $start } إلى { $end }، ولا تتأخر أبدًا أكثر من يوم.
follow-up-cancel = إلغاء
follow-up-done = تم
follow-up-chip-send = متابعة بعد { $time }
follow-up-chip-remind = تذكير بعد { $time }
follow-up-chip-send-on = متابعة { $date }
follow-up-chip-remind-on = تذكير { $date }
follow-up-card-title = لا رد بعد
follow-up-card-title-waiting = رسالة المتابعة الخاصة بك منتظرة
follow-up-card-send = يرسل Katna رسالة المتابعة في { $date }. تتوقف عندما يرد أي شخص.
follow-up-card-send-twice = يرسل Katna رسالة المتابعة في { $date }، ثم مرة أخرى لاحقًا. تتوقف عندما يرد أي شخص.
follow-up-card-remind = إن لم يرد أحد، تعود هذه المحادثة إلى بريدك الوارد في { $date }.
follow-up-card-waiting = حان موعدها بينما كان الكمبيوتر مطفأً، لذا لم تُرسَل متأخرة. أرسلها الآن، أو اختر وقتًا جديدًا، أو أوقفها.
follow-up-card-edit = تعديل
follow-up-card-edit-title = المتابعة في
follow-up-card-send-now = الإرسال الآن
follow-up-card-stop = إيقاف
follow-up-chat-send = متابعة · { $date } إن لم يرد أحد
follow-up-chat-step = المتابعة { $step } من { $steps } · { $date } إن لم يرد أحد
follow-up-chat-waiting = متابعة منتظرة · حان موعدها بينما كان الكمبيوتر مطفأً
follow-up-chat-remind = تعود إلى البريد الوارد { $date } إن لم يصل رد
toast-follow-up-sent = تم إرسال رسالة المتابعة
toast-follow-up-stopped = تم إيقاف المتابعة
toast-follow-up-moved = تم نقل المتابعة إلى { $date }
nudge-row = أُرسلت { $days ->
    [zero] اليوم
    [one] قبل يوم واحد
    [two] قبل يومين
    [few] قبل { $days } أيام
    [many] قبل { $days } يومًا
   *[other] قبل { $days } يوم
}. هل تريد المتابعة؟
nudge-row-tip = اكتب رسالة متابعة إلى كل من فيها
nudge-follow-up = متابعة
nudge-dismiss = تجاهل
nudge-card-title = لا رد بعد
nudge-card-text = سألت شيئًا { $days ->
    [zero] اليوم
    [one] قبل يوم واحد
    [two] قبل يومين
    [few] قبل { $days } أيام
    [many] قبل { $days } يومًا
   *[other] قبل { $days } يوم
} ولم يجب أحد.
nudge-chat-line = أُرسلت { $days ->
    [zero] اليوم
    [one] قبل يوم واحد
    [two] قبل يومين
    [few] قبل { $days } أيام
    [many] قبل { $days } يومًا
   *[other] قبل { $days } يوم
}، ولا رد بعد
toast-nudge-dismissed = تم تجاهل التنبيه
