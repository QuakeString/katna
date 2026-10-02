# Katna Mail, Arabic (العربية).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.
compose-ai-rephrase-tip = إعادة الصياغة (Ctrl+J)
compose-ai-tone-clearer = أوضح
compose-ai-tone-shorter = أقصر
compose-ai-tone-friendlier = أكثر وُدًّا
compose-ai-tone-formal = رسمي
compose-ai-tone-grammar = تصحيح القواعد
compose-ai-tone-longer = أطول
compose-ai-custom = أخبره كيف…
compose-ai-more = طرق أخرى
compose-ai-replace = استبدال
compose-ai-again = المحاولة مجددًا
compose-ai-below = الإضافة أسفله
compose-ai-copy = نسخ
compose-ai-cancel = إلغاء
compose-ai-rephrase = إعادة الصياغة
compose-ai-replaced = تمت إعادة الصياغة
compose-ai-added = تمت الإضافة أسفله
compose-ai-copied = تم النسخ
compose-ai-katna = Katna AI
compose-ai-own = خدمة الذكاء الاصطناعي الخاصة بك
compose-ai-trial-left = { $service } · { $days ->
    [zero] بقي { $days } يوم مجاني
    [one] بقي يوم مجاني واحد
    [two] بقي يومان مجانيان
    [few] بقيت { $days } أيام مجانية
    [many] بقي { $days } يومًا مجانيًا
   *[other] بقي { $days } يوم مجاني
}
compose-ai-encrypted = ستكون هذه الرسالة مشفّرة. تُرسل إعادة الصياغة النص المحدد إلى { $service } دون تشفير. هل تريد إعادة الصياغة على أي حال؟
compose-ai-sign-in = يحتاج Katna AI إلى حساب Katna. سجّل الدخول لاستخدامه، أو استخدم مفتاحك الخاص.
compose-ai-pay = انتهى شهرك المجاني من Katna AI. سعره 5 دولارات شهريًا، أو يمكنك استخدام مفتاحك الخاص.
compose-ai-too-many = طلبات كثيرة جدًا حاليًا. حاول مجددًا بعد قليل.
compose-ai-no-key = أضف مفتاح { $service } في الإعدادات لإعادة الصياغة.
compose-ai-bad-key = لم يقبل { $service } مفتاحك. تحقق منه في الإعدادات.
compose-ai-off = المساعدة في الكتابة بالذكاء الاصطناعي متوقفة في الإعدادات.
compose-ai-failed = تعذّر الوصول إلى { $service }. حاول مجددًا.
compose-ai-try-again = المحاولة مجددًا
compose-ai-open-settings = فتح الإعدادات
compose-ai-write-reply-tip = كتابة رد (Ctrl+J)
compose-ai-write-note-tip = كتابة ملاحظة (Ctrl+J)
compose-ai-rephrase-empty-tip = اكتب شيئًا لإعادة صياغته
compose-ai-write-reply = كتابة رد
compose-ai-write-note = كتابة ملاحظة
compose-ai-write-from = { $count ->
    [zero] من { $count } رسالة
    [one] من رسالة واحدة
    [two] من رسالتين
    [few] من { $count } رسائل
    [many] من { $count } رسالة
   *[other] من { $count } رسالة
}
compose-ai-write-ideas = أفكار من المحادثة
compose-ai-write-own = أو قل ما يجب أن يقوله…
compose-ai-write-short = قصير
compose-ai-write-longer = أطول
compose-ai-write-friendly = ودّي
compose-ai-write-formal = رسمي
compose-ai-write-insert = إدراج
compose-ai-write-back = أفكار أخرى
compose-ai-written = تمت إضافة المسودة
compose-ai-write-encrypted = هذه المحادثة مشفّرة. تُرسل كتابة الرد رسائلها إلى { $service } دون تشفير. هل تريد الكتابة على أي حال؟
compose-ai-write-anyway = كتابة
compose-ai-write-encrypted-off = هذه المحادثة مشفّرة، والإعدادات تُبعد المساعدة في الكتابة عن البريد المشفّر.
compose-ai-subject-tip = إعادة صياغة الموضوع
compose-ai-subject-title = طرق أخرى لقوله
compose-ai-subject-done = تم تغيير الموضوع

## Summing up a conversation: the list's right-click menu, the reading

## pane's sparkle, the chat's strip and the card each opens.

summary-summarize = تلخيص
summary-hide = إخفاء الملخص
summary-close = إغلاق
summary-fold = طيّ
summary-title = الملخص
summary-mails = { $count ->
    [zero] { $count } رسالة
    [one] رسالة واحدة
    [two] رسالتان
    [few] { $count } رسائل
    [many] { $count } رسالة
   *[other] { $count } رسالة
}
summary-of-mails = { $count } من أصل { $total } رسالة
summary-peek-count = { $mails ->
    [zero] { $mails } رسالة
    [one] رسالة واحدة
    [two] رسالتان
    [few] { $mails } رسائل
    [many] { $mails } رسالة
   *[other] { $mails } رسالة
} · { $people ->
    [zero] { $people } شخص
    [one] شخص واحد
    [two] شخصان
    [few] { $people } أشخاص
    [many] { $people } شخصًا
   *[other] { $people } شخص
}
summary-catch-up = { $count ->
    [zero] { $count } رسالة جديدة منذ آخر قراءة
    [one] رسالة جديدة واحدة منذ آخر قراءة
    [two] رسالتان جديدتان منذ آخر قراءة
    [few] { $count } رسائل جديدة منذ آخر قراءة
    [many] { $count } رسالة جديدة منذ آخر قراءة
   *[other] { $count } رسالة جديدة منذ آخر قراءة
}
summary-strip-newer = { $count ->
    [zero] { $count } جديدة منذ ذلك · { $gist }
    [one] واحدة جديدة منذ ذلك · { $gist }
    [two] اثنتان جديدتان منذ ذلك · { $gist }
    [few] { $count } جديدة منذ ذلك · { $gist }
    [many] { $count } جديدة منذ ذلك · { $gist }
   *[other] { $count } جديدة منذ ذلك · { $gist }
}
summary-add-new = { $count ->
    [zero] إضافة { $count } جديدة
    [one] إضافة واحدة جديدة
    [two] إضافة اثنتين جديدتين
    [few] إضافة { $count } جديدة
    [many] إضافة { $count } جديدة
   *[other] إضافة { $count } جديدة
}
summary-point-settled = ما تم الاتفاق عليه
summary-point-money = المال
summary-point-dates = التواريخ
summary-point-next = الخطوة التالية
summary-point-open = مسائل مفتوحة
summary-files = الملفات
summary-for-you = لك
summary-from-mail = { $name }، { $date }
summary-you = أنت
summary-made = { $service } · { $time }
summary-not-read = { $service } · لم توضع علامة «مقروءة»
summary-copy = نسخ
summary-copied = تم نسخ الملخص
summary-again = التلخيص مجددًا
summary-open = فتح
summary-open-tip = فتح المحادثة
summary-reply = رد
summary-reply-tip = كتابة رد بالذكاء الاصطناعي
summary-reply-to = الرد على { $name }
summary-reply-summary = الملخص
summary-reply-send = إرسال
summary-reply-open = فتح
summary-asking = جارٍ سؤال { $service }…
summary-stop = إيقاف
summary-cancel = إلغاء
summary-send = إرسال وتلخيص
summary-ask-short = في انتظار موافقتك
summary-encrypted = هذه المحادثة مشفّرة. يُرسل التلخيص نصها إلى { $service } دون تشفير.
summary-encrypted-off = هذه المحادثة مشفّرة، والإعدادات تُبعد المساعدة في الكتابة عن البريد المشفّر.
summary-try-again = المحاولة مجددًا
summary-open-settings = فتح الإعدادات
