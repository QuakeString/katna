# Katna Mail, Arabic (العربية).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

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
settings-appearance-theme-system = النظام
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
settings-default-apps-documents-detail = Word (docx وdoc) ونصوص OpenDocument (odt) والعروض التقديمية (pptx وppt وodp).
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
settings-appearance-theme-summary = النظام، أو فاتحة، أو داكنة
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
settings-default-apps-documents-summary = مكان فتح مستندات Word ونصوص OpenDocument والعروض التقديمية
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

## Settings: opening at login

settings-open-at-login-failed = تعذّر تغيير الفتح عند تسجيل الدخول: { $error }

## Settings > General > Time

settings-time = الوقت
settings-clock-language = كما تكتبه اللغة
settings-clock-12 = نظام ١٢ ساعة، مثل ٢:٠٥ م
settings-clock-24 = نظام ٢٤ ساعة، مثل ١٤:٠٥
settings-time-summary = نظام ١٢ أو ٢٤ ساعة، أو كما تكتبه اللغة

## Settings > General > Default mail app, Settings > Compose > Grammar

settings-general-mail-app = تطبيق البريد التلقائي
settings-general-mail-app-detail = تفتح روابط البريد الإلكتروني في التطبيقات الأخرى وعلى مواقع الويب رسالة جديدة هنا.
mail-app-is-default = Katna Mail هو تطبيق البريد التلقائي لديك.
mail-app-is-other = تُفتح روابط البريد الإلكتروني في تطبيق آخر.
mail-app-make-default = تعيينه تطبيقًا تلقائيًا
mail-app-make-default-failed = تعذّر تغيير تطبيق البريد التلقائي.
settings-general-mail-app-summary = فتح روابط البريد الإلكتروني من التطبيقات الأخرى ومواقع الويب في Katna Mail
settings-compose-grammar = القواعد النحوية
settings-compose-grammar-detail = يجري التدقيق على هذا الكمبيوتر باستخدام Harper. الإنجليزية فقط حاليًا: لا يُمَسّ النص المكتوب بلغات أخرى.
settings-compose-grammar-check = تدقيق القواعد النحوية
settings-compose-grammar-check-detail = وضع خط تحت الأخطاء النحوية أثناء الكتابة، بالإنجليزية
settings-compose-grammar-summary = وضع خط تحت الأخطاء النحوية أثناء الكتابة، بالإنجليزية
