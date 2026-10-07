# Katna Mail, Arabic (العربية).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings page: its tabs

settings-tab-general = عام
settings-tab-notifications = الإشعارات
settings-tab-inbox = البريد الوارد
settings-tab-accounts = الحسابات
settings-tab-katna-account = حساب Katna
settings-tab-subscriptions = الاشتراك
settings-tab-appearance = المظهر
settings-tab-shortcuts = الاختصارات
settings-tab-default-apps = التطبيقات التلقائية
settings-tab-folders-rules = المجلدات والقواعد
settings-tab-compose = الكتابة
settings-tab-mcp-server = خادم MCP
settings-tab-feedback = ملاحظات المستخدمين
settings-tab-experimental = تجريبي

## Settings page: tabs still to come

settings-tab-folders-rules-coming = أنشئ المجلدات والتصنيفات وأعِد تسميتها وانقلها وأخفِها، واختر ما تتم مزامنته منها. تفرز القواعد البريد الجديد وتصنّفه وتعيد توجيهه أو تحذفه تلقائيًا، حسب المُرسِل أو الموضوع أو الكلمات.

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
settings-translation = الترجمة
settings-translation-detail = يمكن قراءة البريد المكتوب بلغة أخرى بلغتك.
settings-translation-offer = عرض الترجمة
settings-translation-offer-detail = يُرسَل نص الرسالة إلى خادم Katna لترجمته، فقط عندما تطلب ذلك أو تختار ترجمة لغتها دائمًا. ولا تُرسَل المرفقات أبدًا.
settings-translation-reading = الترجمة إلى
settings-translation-always = الترجمة دائمًا
settings-translation-never = عدم العرض أبدًا للغة
settings-translation-none = لا شيء بعد. اختر من شريط الترجمة في أي رسالة.
settings-general-mark-read = وضع علامة «مقروءة»
settings-general-mark-read-now = فور فتحها
settings-general-mark-read-1s = بعد فتحها لمدة ثانية واحدة
settings-general-mark-read-3s = بعد فتحها لمدة 3 ثوانٍ
settings-general-mark-read-never = فقط عندما أضع عليها علامة «مقروءة»
settings-general-auto-advance = التقدم التلقائي
settings-general-auto-advance-detail = بعد حذف المحادثة المفتوحة أو أرشفتها أو نقلها
settings-general-auto-advance-next = فتح المحادثة التالية
settings-general-auto-advance-previous = فتح المحادثة السابقة
settings-general-auto-advance-list = العودة إلى القائمة
settings-general-confirm-delete = الحذف
settings-general-confirm-delete-ask = السؤال قبل حذف عدة محادثات
settings-general-confirm-delete-ask-detail = الحذف النهائي يسأل دائمًا
settings-general-reply-button = زر الرد
settings-general-reply-all = الرد على الكل
settings-general-reply-all-detail = يرد زر الرد بجانب كل رسالة على الجميع، وليس على المُرسِل فقط
settings-general-remote-images = الصور من الويب
settings-general-remote-images-detail = يُخبر تحميلُ صور الرسالة مُرسِلَها بأنك فتحتها، ومتى، ومن أين تقريبًا. عند إيقاف هذا الخيار، تسأل كل رسالة أولًا، ويمكنك دائمًا عرض صور أي مُرسِل.
settings-general-remote-images-always = عرض الصور دائمًا
settings-general-remote-images-always-detail = في كل رسالة، وليس فقط من المُرسِلين الذين تثق بهم
settings-general-sending = الإرسال
settings-general-sending-detail = المدة التي تنتظرها الرسالة المُرسَلة، حتى يمكن التراجع عن إرسالها.
settings-general-video-calls = مكالمات الفيديو
settings-general-video-calls-detail = يستخدم «بدء مكالمة فيديو» Google Meet لحسابات Gmail. أما الحسابات الأخرى فتحصل على غرفة Jitsi Meet على هذا الخادم، ويمكن لأي شخص لديه الرابط الانضمام.
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
settings-general-notifications-detail = للبريد الجديد في المجلدات التي تُصدر إشعارات، حتى عندما يكون Katna Mail مغلقًا.
settings-general-new-mail = إشعاري بالبريد الجديد
settings-general-new-mail-detail = مع أزرار الرد على الكل ووضع علامة «مقروءة» والأرشفة
settings-notifications-sounds = الأصوات
settings-notifications-sounds-detail = من سمة الأصوات في سطح المكتب. تبقى المجلدات والمحادثات والمُرسِلون المكتومون صامتين، وكذلك كل شيء أثناء وضع «عدم الإزعاج».
sounds-new-mail = بريد جديد
sounds-new-mail-detail = في المجلدات التي تُصدر إشعارات
sounds-reminders = التذكيرات
sounds-reminders-detail = أحداث التقويم والمهام
sounds-mail-back = عودة البريد إلى البريد الوارد
sounds-mail-back-detail = البريد المؤجَّل، والبريد الذي لم يرد عليه أحد
sounds-sent = تم إرسال البريد
sounds-sent-detail = بمجرد خروج الرسالة
sounds-not-sent = لم يتم إرسال البريد
sounds-not-sent-detail = عند فشل الإرسال
sounds-play = تشغيل
sound-katna-chime = نغمة Katna
sound-new-email = بريد إلكتروني جديد
sound-new-message = رسالة جديدة
sound-sent = تم الإرسال
sound-alarm = منبّه
sound-bell = جرس
sound-complete = اكتمال
sound-information = معلومة
sound-warning = تحذير
sound-error = خطأ
sound-reminder = تذكير
sound-default = إشعار
settings-general-updates = التحديثات
settings-general-updates-detail = ثبّت إصدارًا جديدًا من «حول»، أو من الإشعار الذي يفيد بأنه جاهز.
settings-general-auto-download = تنزيل التحديثات تلقائيًا
settings-general-auto-download-detail = أبدًا على اتصال محدود البيانات. لا يُثبَّت شيء حتى تضغط «تحديث».
settings-general-reset-cache = إعادة تعيين الذاكرة المؤقتة
settings-general-reset-cache-detail = عندما يبدو البريد خاطئًا أو قديمًا، أو لتحرير مساحة على القرص. لا يتغير أي شيء على خوادم بريدك.
settings-general-desktop = سطح المكتب
settings-general-start-at-login = تشغيل Katna عند تسجيل الدخول
settings-general-start-at-login-detail = يزامن البريد ويعرض إشعارات البريد الجديد وأيقونة علبة النظام، دون فتح النافذة
settings-general-login-window = فتح نافذة Katna Mail أيضًا
settings-general-login-window-detail = تُفتح النافذة أيضًا عند تسجيل الدخول
settings-general-tray = عرض Katna في علبة النظام
settings-general-tray-detail = مع عدد الرسائل غير المقروءة وقائمة
settings-general-tray-color = أيقونة علبة النظام بالألوان
settings-general-tray-color-detail = عند الإيقاف، تكون بلون واحد مثل أيقونات اللوحة الأخرى. يبقى عدد غير المقروءة باللون الأحمر.
settings-general-unread-badge = عدد غير المقروءة على أيقونة شريط المهام
settings-general-unread-badge-detail = البريد غير المقروء في المجلدات التي يُحتسب عددها
settings-notifications-count = العدد على شريط المهام
settings-notifications-count-detail = وكذلك العدد على أيقونة علبة النظام.
settings-notifications-notify = إشعار
settings-notifications-counts = العدّ
settings-notifications-muted = المكتومة
settings-notifications-muted-detail = المجلدات والحسابات والمحادثات والمُرسِلون الذين لا يُصدر بريدهم الجديد إشعارات ولا يُحتسب.
settings-notifications-nothing-muted = لا يوجد شيء مكتوم. اكتم مجلدًا من قائمة النقر بزر الماوس الأيمن عليه أو من الجرس أعلى القائمة.
settings-notifications-until = حتى { $when }
settings-notifications-until-unmuted = حتى تعيد تفعيله
settings-notifications-a-conversation = محادثة
settings-general-search-triggers = البحث من سطح المكتب
settings-general-search-triggers-detail = اكتب إحدى هذه الكلمات ومسافة في KRunner أو بحث GNOME، ثم ما تريد العثور عليه، للبحث في بريدك كما يفعل مربع البحث هنا. افصل بين الكلمات بفواصل.
settings-general-search-triggers-none = لا كلمات؛ تعمل «mail:» فقط

## Settings > Inbox

settings-inbox-tabs = علامات تبويب البريد الوارد
settings-inbox-tabs-detail = فرز البريد الوارد في علامات تبويب، كما يفعل موقع مزوّد بريدك.
settings-inbox-tabs-show = عرض علامات تبويب البريد الوارد
settings-inbox-tabs-show-detail = عند الإيقاف تظهر قائمة واحدة لكل حساب
settings-inbox-no-accounts = أضف حسابًا لاختيار علامات التبويب الخاصة به.
settings-inbox-unified = البريد الوارد الموحّد
settings-inbox-unified-detail = علامات تبويب مشتركة بين كل الحسابات. تظهر كل رسالة في علامة التبويب الخاصة بنوعها؛ ويبقى بريد علامة التبويب التي يوقفها حساب ما في علامة التبويب الأولى.
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
settings-appearance-theme = الوضع
settings-appearance-theme-system = النظام
settings-appearance-theme-light = فاتحة
settings-appearance-theme-dark = داكنة
settings-appearance-theme-forced = نظام الألوان المختار أدناه له جانب فاتح فقط أو داكن فقط، لذا فهو الذي يحدد.
settings-appearance-colors = الألوان
settings-appearance-colors-detail = لكل نظام ألوان جانب فاتح وجانب داكن، لذا يعمل «الوضع» مع كل منها. تعرض المعاينات الجانبين بلون التمييز المختار أدناه.
settings-appearance-colors-built-in = مضمَّنة
settings-appearance-colors-from-system = من نظامك
settings-appearance-colors-system = النظام
settings-appearance-colors-system-detail = يتّبع سطح المكتب
settings-appearance-colors-light-only = فاتح فقط
settings-appearance-colors-dark-only = داكن فقط
settings-appearance-colors-yours = أنظمتك
scheme-customize-card = تخصيص…
scheme-customize-card-detail = انطلاقًا من النظام المختار
scheme-import-card = استيراد…
scheme-import-card-detail = ملف Katna أو KDE
settings-appearance-accent = لون التمييز
settings-appearance-accent-detail = لون زر «إنشاء»، والتطبيق المعروض في الشريط الجانبي، والأعداد، والعناصر المميَّزة
settings-appearance-accent-scheme = من الألوان
settings-appearance-accent-system = النظام
settings-appearance-accent-more = ألوان أخرى
scheme-katna = Katna
scheme-clear = صافٍ
scheme-graphite = جرافيت
scheme-nord = Nord
scheme-solarized = Solarized
scheme-dracula = Dracula
scheme-gruvbox = Gruvbox
scheme-catppuccin = Catppuccin
scheme-tokyo-night = Tokyo Night
scheme-one = One
scheme-rose-pine = Rosé Pine
scheme-everforest = Everforest
scheme-kanagawa = Kanagawa
scheme-ayu = Ayu
scheme-copy-name = نسخة من { $name }
scheme-customize = تخصيص
scheme-edit = تعديل
scheme-duplicate = تكرار
scheme-export = تصدير
scheme-delete = حذف
scheme-deleted = تم حذف «{ $name }»
scheme-exported = تم حفظ «{ $name }»
scheme-import = استيراد نظام ألوان
scheme-import-failed = ليس نظام ألوان يمكن لـ Katna قراءته: { $error }
scheme-editor-new = نظام ألوان جديد
scheme-editor-edit = تعديل نظام الألوان
scheme-editor-name = الاسم
scheme-editor-light = الجانب الفاتح
scheme-editor-dark = الجانب الداكن
scheme-editor-make-dark = إنشاء الداكن من الفاتح
scheme-editor-add-dark = إضافة جانب داكن
scheme-editor-add-light = إضافة جانب فاتح
scheme-editor-remove-side = إزالة هذا الجانب
scheme-editor-readable = سهل القراءة
scheme-editor-hard-to-read = صعب القراءة: { $colors }
scheme-picker-dropper = الالتقاط من الشاشة
scheme-picker-in-scheme = في هذا النظام
scheme-picker-recent = الأخيرة
scheme-picker-system = منتقي النظام…
scheme-seed-page = الصفحة
scheme-seed-cards = البطاقات
scheme-seed-text = النص
scheme-seed-faint = النص الباهت
scheme-seed-accent = لون التمييز
scheme-seed-bar-text = نص الشريط العلوي
scheme-seed-on-accent = النص على لون التمييز
scheme-seed-error = الخطأ
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
settings-files-page = صفحة «الملفات»
settings-files-page-detail = المرفقات التي تعرضها صفحة «الملفات»
settings-files-leave-out-small = استبعاد الصور الصغيرة
settings-files-leave-out-small-detail = الشعارات والأيقونات في التوقيعات، التي تأتي مع رسائل كثيرة
settings-files-smaller-than = أصغر من
settings-files-kb = كيلوبايت
settings-files-narrower-than = أو أضيق أو أقصر من
settings-files-px = بكسل
settings-files-more-tip = أكثر
settings-files-less-tip = أقل
settings-files-sizes-note = تُقرأ أحجام البكسل بعد تنزيل الرسالة؛ وحتى ذلك الحين يُحكم على صورها بحجم الملف وحده.
settings-files-drives = المحركات في «الملفات»
settings-files-drives-detail = المحرك الخاص بكل حساب، معروضًا في صفحة «الملفات»
settings-files-drive-needs = { $address } · يحتاج Katna إلى الإذن مرة واحدة

## Settings > Compose

settings-compose-send-from = إرسال الرسائل الجديدة من
settings-compose-send-from-detail = تبدأ الرسائل الجديدة من هذا الحساب، ويختار صف «من» حسابًا آخر. تُرسَل الردود وإعادات التوجيه دائمًا من الحساب الذي وصلت إليه الرسالة الأصلية.
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
settings-compose-no-templates = لا توجد نماذج بعد. في رسالة، اختر «النماذج» ثم «حفظ كنموذج».
settings-compose-template-new = إنشاء جديد
settings-compose-template-new-name = نموذج جديد
settings-compose-template-subject = الموضوع
settings-compose-template-text = نص النموذج
settings-compose-template-fields = تُملأ {"{"}first name{"}"} و{"{"}name{"}"} و{"{"}my name{"}"} باسم المستلم واسمك.
settings-compose-template-remove-file = إزالة المرفق
settings-compose-template-save = حفظ
settings-compose-template-saved = تم حفظ النموذج
settings-compose-template-needs-name = أعطِ النموذج اسمًا
settings-compose-template-delete = حذف النموذج
settings-compose-template-deleted = تم حذف النموذج
settings-compose-template-delete-failed = تعذّر حذف النموذج: { $error }

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
settings-translation-summary = ترجمة البريد المكتوب بلغات أخرى عبر خادم Katna، إلى اللغة التي تختارها
settings-general-mark-read-summary = متى توضع علامة «مقروءة» على المحادثة المفتوحة: فورًا، أو بعد ثانية أو 3 ثوانٍ، أو يدويًا
settings-general-auto-advance-summary = ما يُفتح بعد حذف المحادثة المفتوحة أو أرشفتها أو نقلها: التالية أو السابقة أو القائمة
settings-general-confirm-delete-summary = السؤال قبل نقل عدة محادثات إلى المهملات
settings-general-reply-button-summary = يرد زر الرد بجانب كل رسالة على الجميع
settings-general-remote-images-summary = عرض الصور في كل رسالة دائمًا
settings-general-sending-summary = التراجع عن الإرسال: المدة التي تنتظرها الرسالة المُرسَلة، حتى يمكن التراجع عن إرسالها
settings-general-video-calls-summary = خادم Jitsi Meet لمكالمات الفيديو الجديدة من الحسابات التي لا تستخدم Google Meet
settings-general-offline-summary = عدد أيام البريد الحديث التي يتم تنزيلها كاملة لقراءتها بلا اتصال
settings-general-notifications-summary = إشعارات البريد الجديد
settings-notifications-sounds-summary = صوت البريد الجديد والتذكيرات والبريد المُرسَل أو غير المُرسَل
settings-general-updates-summary = تنزيل إصدارات Katna الجديدة من تلقاء نفسها
settings-general-reset-cache-summary = حذف البريد المنزّل وصور المُرسِلين وفهرس البحث، وتنزيلها مجددًا
settings-general-desktop-summary = تشغيل Katna عند تسجيل الدخول، وأيقونة علبة النظام
settings-notifications-count-summary = عدد غير المقروءة على أيقونة شريط المهام
settings-notifications-muted-summary = إلغاء كتم المجلدات والحسابات والمحادثات والمُرسِلين
settings-accounts-accounts-summary = إضافة حساب أو إزالته، أو تغيير صورته
settings-appearance-density-summary = أسطر تلقائية أو مضغوطة في القائمة
settings-appearance-scaling-summary = تكبير كل شيء أو تصغيره: النصوص والأيقونات والمسافات والفواصل
settings-appearance-theme-summary = النظام، أو فاتحة، أو داكنة
settings-appearance-colors-summary = أنظمة الألوان: نظام سطح المكتب، أو نظام Katna، أو نظام مضمَّن مثل Nord أو Solarized
settings-appearance-accent-summary = لون المجلد المحدد وزر «إنشاء» والأعداد
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
settings-files-page-summary = استبعاد الصور الصغيرة، مثل شعارات التوقيعات، من صفحة «الملفات»، واختيار المحركات التي تعرضها
settings-compose-send-from-summary = الحساب الذي يُرسَل منه البريد الجديد: الأول، أو حساب آخر، أو الحساب الذي تستخدمه
settings-compose-send-on-replies-summary = «إرسال» أو «إرسال وأرشفة» المحادثة، في الردود وإعادة التوجيه
settings-compose-signatures-summary = يُضاف أسفل رسالتك، بعد سطر «--»
settings-compose-for-new-mail-summary = التوقيع الذي يبدأ به البريد الجديد
settings-compose-for-replies-summary = التوقيع الذي تبدأ به الردود وإعادات التوجيه
settings-compose-format-summary = كتابة البريد الجديد بنص عادي
settings-compose-spelling-summary = التدقيق الإملائي أثناء الكتابة، ولغة القاموس
settings-general-search-triggers-summary = كلمات تبحث في بريدك من KRunner أو بحث GNOME
settings-compose-templates-summary = احفظ الرسائل التي تكتبها كثيرًا، وابدأ منها رسالة جديدة أو ردًا
settings-feedback-crash-reports-summary = حفظ تقارير الأعطال على هذا الكمبيوتر عند تعطّل Katna Mail أو خدمته في الخلفية
settings-feedback-saved-summary = عرض تقارير الأعطال المحفوظة على هذا الكمبيوتر أو نسخها أو حذفها
settings-feedback-help-improve-summary = إرسال تقارير الأعطال للمساعدة في إصلاح الخلل؛ متوقف ما لم تفعّله
settings-experimental-blur-summary = إضفاء ضبابية على خلفية النافذة، أو جعل القوائم ونوافذ الحوار كالزجاج المصنفر، أو كليهما
settings-search-shortcut = اختصار لوحة المفاتيح
settings-search-tab = علامة تبويب في الإعدادات
settings-search-none = لا توجد إعدادات مطابقة لـ«{ $query }».
settings-search-results = الإعدادات المطابقة لـ«{ $query }»

## Settings: opening at login

settings-open-at-login-failed = تعذّر تغيير التشغيل عند تسجيل الدخول: { $error }

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
settings-compose-suggestions = اقتراحات الكتابة
settings-compose-suggestions-detail = تُتعلَّم العبارات على هذا الكمبيوتر من البريد الذي أرسلته والبريد الذي تردّ عليه. اضغط Tab لقبول اقتراح، أو واصل الكتابة.
settings-compose-suggestions-on = الاقتراح أثناء الكتابة
settings-compose-suggestions-on-detail = عرض التتمة المحتملة للعبارة باللون الرمادي أثناء الكتابة
settings-ai-autocomplete = اقتراحات أطول بالذكاء الاصطناعي
settings-ai-autocomplete-detail = إكمال الجملة بعد توقفك، باستخدام الذكاء الاصطناعي المختار أدناه. لا يحدث أبدًا للبريد المشفّر.
settings-ai-answered = استخدام البريد الذي تردّ عليه
settings-ai-answered-detail = تخمينات أفضل للأسماء والتواريخ؛ ويُرسل نصًا أكثر
settings-ai = المساعدة في الكتابة بالذكاء الاصطناعي
settings-ai-detail = حدّد نصًا في رسالة واضغط على رمز الوميض (أو Ctrl+J) لإعادة صياغته. يُرسل النص الذي تختاره فقط، ولا يُحتفظ بشيء.
settings-ai-katna = Katna AI
settings-ai-own = مفتاحك الخاص
settings-ai-off = متوقف
settings-ai-katna-detail = مجاني لمدة 30 يومًا من أول استخدام، ثم 5 دولارات شهريًا. يستخدم حساب Katna الخاص بك.
settings-ai-own-detail = يذهب مفتاحك إلى خدمتك فقط. يُحفظ في سلسلة مفاتيح النظام، وليس في إعدادات Katna.
settings-ai-service = خدمة الذكاء الاصطناعي الخاصة بك
settings-ai-service-detail = أي خدمة تدعم واجهة OpenAI البرمجية تعمل ضمن «أخرى»، مثل Ollama أو LM Studio على هذا الكمبيوتر.
settings-ai-other = أخرى
settings-ai-address = العنوان
settings-ai-model = النموذج
settings-ai-models = النماذج التي تقدّمها هذه الخدمة
settings-ai-key = مفتاح API
settings-ai-key-paste = الصق مفتاحك
settings-ai-key-save = حفظ المفتاح
settings-ai-key-saved = تم حفظ مفتاح.
settings-ai-key-remove = إزالة
settings-ai-key-none = لم يتم حفظ أي مفتاح بعد.
settings-ai-key-saved-toast = تم حفظ المفتاح
settings-ai-key-removed = تمت إزالة المفتاح
settings-ai-encrypted-title = البريد المشفّر
settings-ai-encrypted = عرض «إعادة الصياغة» في البريد المشفّر
settings-ai-encrypted-detail = يسأل في كل مرة قبل إرسال نص من رسالة مشفّرة
settings-compose-grammar-summary = وضع خط تحت الأخطاء النحوية أثناء الكتابة، بالإنجليزية
settings-compose-suggestions-summary = عرض التتمة المحتملة للعبارة باللون الرمادي أثناء الكتابة
settings-ai-summary = إعادة صياغة النص المحدد وإكمال الجمل باستخدام Katna AI أو مفتاحك الخاص
