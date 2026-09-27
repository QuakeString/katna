# Katna Mail, Arabic (العربية).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## About Katna (the dialog from the "i" button on the top bar)

about-tooltip = حول Katna
about-tagline = البريد والتقويم لسطح مكتب Linux
about-whats-new = ما الجديد
about-changelog = سجل التغييرات
about-source = الشيفرة المصدرية
about-coffee = اشترِ لي فنجان قهوة
about-coming-soon = قريبًا
about-coffee-scan = أو امسح الرمز بهاتفك.
about-follow = تابِع المطوّر
about-love-title = صُنع بحب من أجل Rust وKDE وLinux
about-love-text = تجعل Rust كتابة تطبيق بريد سريع وآمن متعة: لا يحتوي Katna على أي شيفرة unsafe. استلهم Katna فكرته من سطح مكتب Plasma من KDE ومجموعة PIM الخاصة به، ويشكّل Linux ومجتمع البرمجيات الحرة الأرض التي يقف عليها. شكرًا لكم، وشكرًا للمكتبات أدناه.
about-kde-text = تبني KDE سطح المكتب الذي يشعر فيه Katna بأنه في بيته أكثر من أي مكان آخر، ويصنعه متطوعون ويموّله أشخاص مثلك. إن كنت تستمتع بـ Plasma أو بتطبيقات KDE، ففكّر في التبرع لـ KDE.
about-donate-kde = التبرع لـ KDE
about-gpui-title = مبني على GPUI، من مشروع Zed
about-gpui-text = واجهة Katna Mail كلها مبنية على GPUI، إطار عمل الواجهات السريع المسرَّع بوحدة معالجة الرسومات الذي صنعته Zed Industries لمحرر Zed. كل بكسل وكل حركة وكل نافذة تراها يرسمها هذا الإطار. شكرًا لفريق Zed على بنائه بشكل مفتوح. Apache-2.0.
about-gpui-github = GPUI على GitHub
about-personal-title = مشروع شخصي
about-personal-text = لا يحاول Katna Mail أن يكون جديدًا أو ثوريًا. إنه تطبيق البريد الذي أراده مطوّره، وميزاته ومظهره مستعارة من Gmail وMailspring وThunderbird. ولم يكن ممكنًا إلا بفضل ما وصلت إليه النماذج اللغوية الكبيرة (LLM).
about-built-on = مبني على برمجيات حرة
about-credit-pimalaya = IMAP وSMTP وتسجيل الدخول (io-imap وio-smtp وio-sasl)
about-credit-imap-codec = قراءة IMAP وكتابته
about-credit-tantivy = البحث
about-credit-sqlite = مخزن البريد
about-credit-rustls = الاتصالات الآمنة
about-credit-mail-parser = قراءة البريد، من Stalwart Labs
about-credit-html5ever = بريد HTML، من مشروع Servo
about-credit-zbus = التواصل مع سطح المكتب عبر D-Bus والبوابات
about-credit-oo7 = كلمات المرور في حلقة مفاتيح سطح المكتب
about-credit-hayro = عرض ملفات PDF وطباعتها
about-credit-calamine = معاينات جداول البيانات
about-credit-resvg = صور SVG
about-credit-jiff = التواريخ والمناطق الزمنية
about-credit-spellbook = التدقيق الإملائي، من محرر Helix
about-credit-smol = إنجاز أشياء كثيرة في وقت واحد
about-all-libraries = كل المكتبات التي يستخدمها Katna ({ $count })
about-library-authors = من تأليف { $authors }
about-license = Katna برنامج حر بموجب رخصة GNU GPL، الإصدار 3 أو أحدث.
about-close = إغلاق

## What’s new (shown after an update)

whats-new-title = ما الجديد في Katna Mail
whats-new-updated = تم التحديث إلى الإصدار { $version }
whats-new-version = الإصدار { $version }
whats-new-more = { $count ->
    [zero] و{ $count } تغيير آخر في سجل التغييرات الكامل.
    [one] وتغيير آخر في سجل التغييرات الكامل.
    [two] وتغييران آخران في سجل التغييرات الكامل.
    [few] و{ $count } تغييرات أخرى في سجل التغييرات الكامل.
    [many] و{ $count } تغييرًا آخر في سجل التغييرات الكامل.
   *[other] و{ $count } تغيير آخر في سجل التغييرات الكامل.
}
whats-new-changelog = سجل التغييرات الكامل
whats-new-got-it = حسنًا

## First run: welcome page

onboarding-welcome-title = مرحبًا بك في Katna Mail
onboarding-welcome-lead = بريدك على حاسوبك: سريع البحث، ومقروء دون اتصال، وخاص.
onboarding-fast-title = سريع، حتى دون اتصال
onboarding-fast-text = يحتفظ Katna بنسخة من بريدك هنا، فيكون فتحه والبحث فيه فوريًا، مع اتصال أو دونه.
onboarding-providers-title = يعمل مع بريدك
onboarding-providers-text = Gmail وOutlook وYahoo وiCloud وأي حساب IMAP أو POP آخر.
onboarding-private-title = خاص
onboarding-private-text = يصل بريدك مباشرة من مزوّدك إلى هذا الحاسوب. لا يراه أي خادم لـ Katna.
onboarding-get-started = لنبدأ

## First run: adding an account

onboarding-service-checking = جارٍ التحقق من خدمة Katna في الخلفية…
onboarding-service-running = خدمة Katna في الخلفية تعمل.
onboarding-service-missing = خدمة Katna في الخلفية لا تعمل
onboarding-service-start = هي التي تجلب بريدك وترسله. شغّلها من الطرفية، ثم تحقّق مجددًا:
onboarding-check-again = التحقق مجددًا
onboarding-account-title = أضف حساب بريدك
onboarding-account-lead = اكتب عنوان بريدك الإلكتروني وكلمة المرور، وسيجد Katna إعدادات الخادم. تحتاج Gmail وYahoo وiCloud إلى كلمة مرور للتطبيقات، تُنشئها من إعدادات الأمان في حسابك.
onboarding-add-account = إضافة حساب
onboarding-back = رجوع

## First run: choosing the look

onboarding-look-title = اجعله على ذوقك
onboarding-look-lead = اختر كيف يُفتح البريد وكيف يبدو Katna. يمكنك تغيير ذلك في أي وقت من الإعدادات السريعة.
onboarding-reading-pane = جزء القراءة
onboarding-pane-right = بجانب القائمة
onboarding-pane-none = بلا تقسيم
onboarding-theme = السمة
onboarding-theme-system = النظام
onboarding-theme-light = فاتحة
onboarding-theme-dark = داكنة
onboarding-density = الكثافة
onboarding-density-default = تلقائية
onboarding-density-compact = مضغوطة
onboarding-continue = متابعة

## First run: done

onboarding-ready-title = كل شيء جاهز
onboarding-ready-lead = يجلب Katna بريدك الآن. يظهر فور وصوله، ويظهر البريد الجديد من تلقاء نفسه.
onboarding-ready-lead-address = يجلب Katna بريد { $address } الآن. يظهر فور وصوله، ويظهر البريد الجديد من تلقاء نفسه.
onboarding-ready-tour = هل تريد جولة مدتها دقيقة واحدة لترى مكان كل شيء؟
onboarding-skip = تخطٍّ الآن
onboarding-take-tour = بدء الجولة

## Asking to send crash reports (on its own and on the first-run pages)

share-title = ساعد في تحسين Katna
share-lead = عندما يتعطل Katna، يحفظ تقريرًا على هذا الحاسوب. يساعد إرسال هذه التقارير في إصلاح ما حدث. يمكنك تغيير ذلك في أي وقت من الإعدادات > ملاحظات المستخدمين.
share-sent = ما يُرسَل
share-sent-detail = تقرير العطل كما يمكنك عرضه في الإعدادات: ما الذي تعطّل وأين في Katna، والإصدار، ونظام Linux وسطح المكتب لديك، وآخر أسطر سجل Katna، وقد تذكر أسماء مجلدات البريد.
share-never-sent = ما لا يُرسَل أبدًا
share-never-sent-detail = رسائلك وجهات اتصالك وكلمات مرورك وعنوان IP واسم المستخدم واسم الحاسوب. تُزال عناوين البريد الإلكتروني من التقرير.
share-where = إلى أين يذهب
share-where-detail = إلى متتبّع أعطال Katna على Sentry، والمخزَّن في الاتحاد الأوروبي. لا يربط أي معرّف التقارير بك.
share-dont-send = عدم الإرسال
share-send = إرسال تقارير الأعطال
share-sending = سيتم إرسال تقارير الأعطال. شكرًا لك.
share-local = تبقى تقارير الأعطال على هذا الحاسوب.

## The tour (cards pointing at each part of the window)

tour-welcome-title = مرحبًا بك في Katna Mail
tour-welcome-text = جولة مدتها دقيقة واحدة تريك مكان كل شيء.
tour-not-now = ليس الآن
tour-start = بدء الجولة
tour-close = إغلاق
tour-skip = تخطي الجولة
tour-back = السابق
tour-done = تم
tour-next = التالي
tour-step = { $step } من { $total }
tour-compose-title = اكتب رسالة
tour-compose-text = يفتح زر «إنشاء» رسالة جديدة في أسفل النافذة، فتتابع القراءة وأنت تكتب.
tour-search-title = ابحث في بريدك كله
tour-search-text = يعمل البحث دون اتصال أيضًا. يضيف الزر في طرف مربع البحث عوامل تصفية: المرسِل والمستلِم والموضوع والتواريخ والمرفقات.
tour-menu-title = إظهار المجلدات أو إخفاؤها
tour-menu-text = يطوي هذا الزر قائمة المجلدات. وأثناء إخفائها، أبقِ المؤشر على «البريد» في الشريط الجانبي لترى المجلدات.
tour-apps-title = تطبيقاتك
tour-apps-text = البريد يسكن هنا الآن. وسينضم إليه في هذا الشريط التقويم وجهات الاتصال والمهام والملاحظات والخلاصات.
tour-tabs-title = علامات تبويب البريد الوارد
tour-tabs-text = يُفرز البريد الجديد في الأساسية والعروض الترويجية والشبكات الاجتماعية والتحديثات والمنتديات. يمكنك إيقاف علامات التبويب من الإعدادات السريعة.
tour-list-title = رسائلك
tour-list-text = انقر على رسالة لقراءتها. مرّر المؤشر فوقها لإجراءات سريعة، أو انقر بالزر الأيمن للمزيد، أو حدّد عدة رسائل للتعامل معها معًا.
tour-settings-title = الإعدادات السريعة
tour-settings-text = غيّر من هنا جزء القراءة والكثافة والسمة. ويمكن بدء الجولة مجددًا من هناك أيضًا.
tour-account-title = حسابك
tour-account-text = اعرف الحساب الذي تستخدمه، وأضف حسابًا آخر.

## Crash notice (a bar at the bottom after a crash)

crash-daemon = { $more ->
    [0] توقفت خدمة Katna في الخلفية على نحو غير متوقع.
    [one] توقفت خدمة Katna في الخلفية على نحو غير متوقع. يوجد تقرير أعطال آخر محفوظ.
    [two] توقفت خدمة Katna في الخلفية على نحو غير متوقع. يوجد تقريرا أعطال آخران محفوظان.
    [few] توقفت خدمة Katna في الخلفية على نحو غير متوقع. توجد { $more } تقارير أعطال أخرى محفوظة.
    [many] توقفت خدمة Katna في الخلفية على نحو غير متوقع. يوجد { $more } تقريرًا آخر محفوظًا.
   *[other] توقفت خدمة Katna في الخلفية على نحو غير متوقع. يوجد { $more } تقرير أعطال آخر محفوظ.
}
crash-mail = { $more ->
    [0] أُغلق Katna Mail على نحو غير متوقع في المرة السابقة.
    [one] أُغلق Katna Mail على نحو غير متوقع في المرة السابقة. يوجد تقرير أعطال آخر محفوظ.
    [two] أُغلق Katna Mail على نحو غير متوقع في المرة السابقة. يوجد تقريرا أعطال آخران محفوظان.
    [few] أُغلق Katna Mail على نحو غير متوقع في المرة السابقة. توجد { $more } تقارير أعطال أخرى محفوظة.
    [many] أُغلق Katna Mail على نحو غير متوقع في المرة السابقة. يوجد { $more } تقريرًا آخر محفوظًا.
   *[other] أُغلق Katna Mail على نحو غير متوقع في المرة السابقة. يوجد { $more } تقرير أعطال آخر محفوظ.
}
crash-view = عرض التقرير
crash-view-tooltip = فتح التقرير المحفوظ على هذا الحاسوب
crash-copy = نسخ التقرير
crash-close = إغلاق
