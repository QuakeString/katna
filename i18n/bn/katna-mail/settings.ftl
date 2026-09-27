# Katna Mail, Bengali (বাংলা).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings page: its tabs

settings-tab-general = সাধারণ
settings-tab-inbox = ইনবক্স
settings-tab-accounts = অ্যাকাউন্ট
settings-tab-subscriptions = সাবস্ক্রিপশন
settings-tab-appearance = চেহারা
settings-tab-shortcuts = শর্টকাট
settings-tab-default-apps = ডিফল্ট অ্যাপ
settings-tab-folders-rules = ফোল্ডার ও নিয়ম
settings-tab-compose = লিখুন
settings-tab-mcp-server = MCP সার্ভার
settings-tab-feedback = ব্যবহারকারীর মতামত
settings-tab-experimental = পরীক্ষামূলক

## Settings page: tabs still to come

settings-tab-subscriptions-coming = আপনি যেসব নিউজলেটার ও মেলিং লিস্ট পান সেগুলি দেখুন, আর এক ক্লিকে আনসাবস্ক্রাইব করুন।
settings-tab-folders-rules-coming = ফোল্ডার ও লেবেল তৈরি করুন, নাম বদলান, সরান ও লুকান, এবং কোনগুলি সিঙ্ক হবে তা বেছে নিন। নিয়মগুলি প্রেরক, বিষয় বা শব্দ অনুযায়ী নতুন মেল নিজে থেকেই সাজায়, লেবেল দেয়, ফরোয়ার্ড করে বা মুছে দেয়।
settings-tab-mcp-server-coming = এই কম্পিউটারের AI সহকারীদের আপনার অনুমতি নিয়ে আপনার মেল খুঁজতে, পড়তে ও খসড়া লিখতে দিন।

## Settings > General

settings-general-conversations = কথোপকথন ভিউ
settings-general-conversations-group = একই মেলের উত্তরগুলি একসাথে রাখুন
settings-general-conversations-group-detail = তালিকায় প্রতিটি কথোপকথনের জন্য একটি লাইন
settings-general-reading = পড়া
settings-general-newest-first = সবচেয়ে নতুন মেসেজ আগে
settings-general-newest-first-detail = কথোপকথন তার সবচেয়ে নতুন উত্তর দিয়ে শুরু হয়
settings-general-full-headers = সম্পূর্ণ হেডার দেখান
settings-general-full-headers-detail = প্রতিটি মেসেজে প্রেরক, প্রাপক, cc, তারিখ ও বিষয় খোলা থাকে
settings-general-full-names = প্রাপকদের পুরো নাম
settings-general-full-names-detail = “প্রাপক: আমাকে, Ada”-এর বদলে “প্রাপক: আমাকে, Ada Lovelace”
settings-general-mark-read = পঠিত হিসেবে চিহ্নিত করুন
settings-general-mark-read-now = খোলার সাথে সাথেই
settings-general-mark-read-1s = 1 সেকেন্ড খোলা থাকার পরে
settings-general-mark-read-3s = 3 সেকেন্ড খোলা থাকার পরে
settings-general-mark-read-never = শুধু যখন আমি পঠিত হিসেবে চিহ্নিত করি
settings-general-reply-button = উত্তর দেওয়ার বোতাম
settings-general-reply-all = সবাইকে উত্তর দিন
settings-general-reply-all-detail = প্রতিটি মেসেজের পাশের উত্তর বোতাম শুধু প্রেরককে নয়, সবাইকে উত্তর দেয়
settings-general-remote-images = ওয়েব থেকে ছবি
settings-general-remote-images-detail = কোনো মেসেজের ছবি লোড করলে তার প্রেরক জেনে যান যে আপনি সেটি খুলেছেন, কখন খুলেছেন এবং মোটামুটি কোথা থেকে। বন্ধ থাকলে প্রতিটি মেসেজ আগে জিজ্ঞাসা করে, আর আপনি যেকোনো সময় কোনো প্রেরকের ছবি দেখাতে পারেন।
settings-general-remote-images-always = সবসময় ছবি দেখান
settings-general-remote-images-always-detail = প্রতিটি মেসেজে, শুধু বিশ্বস্ত প্রেরকদের মেসেজে নয়
settings-general-sending = পাঠানো
settings-general-sending-detail = পাঠানো মেসেজ কতক্ষণ অপেক্ষা করবে, যাতে সেটি ফিরিয়ে নেওয়া যায়।
settings-general-offline = অফলাইন মেল
settings-general-offline-detail = সাম্প্রতিক মেল পুরোপুরি ডাউনলোড করা হয়, যাতে কানেকশন ছাড়াই পড়া যায়। পুরনো মেল খুললে তখন ডাউনলোড হয়।
settings-general-offline-days = { $count ->
    [one] { $count } দিন
   *[other] { $count } দিন
}
settings-general-offline-years = { $count ->
    [one] { $count } বছর
   *[other] { $count } বছর
}
settings-general-offline-all = সব মেল
settings-general-offline-note = কম দিন বেছে নিলে আগে ডাউনলোড করা মেল থেকে যায়। সার্ভারে কিছুই বদলায় না।
settings-general-notifications = বিজ্ঞপ্তি
settings-general-notifications-detail = ইনবক্সে নতুন মেলের জন্য, Katna Mail বন্ধ থাকলেও।
settings-general-new-mail = নতুন মেলের বিজ্ঞপ্তি দিন
settings-general-new-mail-detail = সবাইকে উত্তর দিন, পঠিত হিসেবে চিহ্নিত করুন এবং আর্কাইভ করুন বোতাম সহ
settings-general-new-mail-sound = শব্দ বাজান
settings-general-new-mail-sound-detail = ডেস্কটপের নতুন মেলের শব্দ
settings-general-desktop = ডেস্কটপ
settings-general-start-at-login = লগ ইন করলে Katna চালু করুন
settings-general-start-at-login-detail = উইন্ডো না খুলেই মেল সিঙ্ক করে এবং নতুন মেলের বিজ্ঞপ্তি ও সিস্টেম ট্রে আইকন দেখায়
settings-general-login-window = Katna Mail-এর উইন্ডোও খুলুন
settings-general-login-window-detail = লগ ইন করলে উইন্ডোটিও খুলে যায়
settings-general-tray = সিস্টেম ট্রে-তে Katna দেখান
settings-general-tray-detail = অপঠিত সংখ্যা ও একটি মেনু সহ
settings-general-unread-badge = টাস্কবারের আইকনে অপঠিত সংখ্যা
settings-general-unread-badge-detail = ইনবক্সের কতগুলি মেসেজ অপঠিত

## Settings > Inbox

settings-inbox-tabs = ইনবক্স ট্যাব
settings-inbox-tabs-detail = ইনবক্সকে ট্যাবে ভাগ করুন, যেমন আপনার মেল প্রদানকারীর ওয়েবসাইট করে।
settings-inbox-tabs-show = ইনবক্স ট্যাব দেখান
settings-inbox-tabs-show-detail = বন্ধ থাকলে প্রতিটি অ্যাকাউন্টের জন্য একটি তালিকা দেখায়
settings-inbox-no-accounts = ট্যাব বেছে নিতে একটি অ্যাকাউন্ট যোগ করুন।
settings-inbox-tabs-automatic = স্বয়ংক্রিয়: { $tabs } ({ $provider })
settings-inbox-tabs-off = কোনো ট্যাব নেই
settings-inbox-tabs-gmail = প্রাথমিক, প্রচার, সামাজিক, আপডেট, ফোরাম
settings-inbox-tabs-focused = ফোকাসড ও অন্যান্য
settings-inbox-tabs-zoho = ইনবক্স, নিউজলেটার ও বিজ্ঞপ্তি
settings-inbox-tabs-shown = দেখানো ট্যাব। যে ট্যাব আপনি বন্ধ করেন, তার মেল { $tab }-এ থাকে।

## Settings > Appearance

settings-appearance-reading-pane = রিডিং প্যান
settings-appearance-reading-pane-detail = খোলা কথোপকথন কোথায় দেখাবে।
settings-appearance-pane-right = তালিকার ডানদিকে
settings-appearance-pane-none = কোনো বিভাজন নেই
settings-appearance-density = ঘনত্ব
settings-appearance-density-default = ডিফল্ট
settings-appearance-density-compact = কমপ্যাক্ট
settings-appearance-scaling = স্কেলিং
settings-appearance-scaling-detail = ডেস্কটপের নিজস্ব স্কেলের উপরে Katna Mail-এর সবকিছু বড় বা ছোট করে: লেখা, আইকন, ফাঁকা জায়গা ও বিভাজক। আপনার পাঠানো মেলের ফন্টের আকার একই থাকে। খুব ছোট আকারে আইকনে ক্লিক করা কঠিন হতে পারে।
settings-appearance-theme = থিম
settings-appearance-theme-system = সিস্টেম
settings-appearance-theme-light = লাইট
settings-appearance-theme-dark = ডার্ক
settings-appearance-desktop-colors = ডেস্কটপের রং
settings-appearance-desktop-colors-use = ডেস্কটপের রং ব্যবহার করুন
settings-appearance-desktop-colors-use-detail = ডেস্কটপের কালার স্কিম ও অ্যাকসেন্ট কালার
settings-appearance-app-names = অ্যাপের নাম
settings-appearance-app-names-show = অ্যাপের নাম দেখান
settings-appearance-app-names-show-detail = একেবারে বাঁদিকে অ্যাপ আইকনের নিচে নাম
settings-appearance-sender-pictures = প্রেরকের ছবি
settings-appearance-sender-pictures-show = কোম্পানির লোগো দেখান
settings-appearance-sender-pictures-show-detail = প্রেরকের ডোমেন দিয়ে খোঁজা হয়, কখনো মেসেজ দিয়ে নয়, এবং এক সপ্তাহ রাখা হয়
settings-appearance-important = গুরুত্বপূর্ণ চিহ্ন
settings-appearance-important-show = গুরুত্বপূর্ণ চিহ্ন দেখান
settings-appearance-important-show-detail = তালিকায় প্রতিটি মেসেজের পাশে
settings-appearance-message-width = মেসেজের প্রস্থ
settings-appearance-message-width-limit = মেসেজের প্রস্থ সীমিত করুন
settings-appearance-message-width-limit-detail = চওড়া উইন্ডোতে লম্বা লাইন পড়া সহজ হয়
settings-appearance-mail-colors = মেলের রং
settings-appearance-mail-colors-detail = বেশিরভাগ মেল সাদা পাতার জন্য ডিজাইন করা। ডার্ক থিমে এর রং বদলে এমন গাঢ় রং করা হয় যা সহজে পড়া যায়; বন্ধ থাকলে মেল হালকা পাতায় প্রেরকের রংই রাখে।
settings-appearance-dark-mail = মেলের জন্যও গাঢ় রং
settings-appearance-dark-mail-detail = শুধু যখন থিম ডার্ক থাকে
settings-appearance-attachment-previews = অ্যাটাচমেন্টের প্রিভিউ
settings-appearance-attachment-previews-show = অ্যাটাচমেন্টের প্রিভিউ দেখান
settings-appearance-attachment-previews-show-detail = প্রতিটি ফাইলের কার্ডে তার বিষয়বস্তুর একটি ছোট ছবি

## Settings > Default apps

settings-default-apps-intro = ক্লিক করলে অ্যাটাচমেন্ট কোথায় খুলবে। ভিউয়ার সবসময় ফাইলটি অন্য অ্যাপেও খুলতে পারে। ডেস্কটপের ডিফল্ট অ্যাপগুলি তার নিজের সেটিংসে ঠিক করা হয়।
settings-default-apps-pdf = PDF ফাইল
settings-default-apps-pdf-detail = পাতা, জুম সহ।
settings-default-apps-pictures = ছবি
settings-default-apps-pictures-detail = ফটো (সোজা করে ঘোরানো), PNG, GIF, WebP, BMP, TIFF ও SVG।
settings-default-apps-text = টেক্সট ফাইল
settings-default-apps-text-detail = সাধারণ টেক্সট, লগ, কোড ও অন্যান্য টেক্সট।
settings-default-apps-sheets = স্প্রেডশিট
settings-default-apps-sheets-detail = Excel (xlsx, xls), OpenDocument (ods) ও CSV।
settings-default-apps-documents = ডকুমেন্ট
settings-default-apps-documents-detail = Word (docx, doc), OpenDocument টেক্সট (odt) ও স্লাইড (pptx, ppt, odp)।
settings-default-apps-katna = Katna Mail-এর ভিউয়ার
settings-default-apps-system = ডেস্কটপের ডিফল্ট অ্যাপ
settings-default-apps-ask = প্রতিবার জিজ্ঞাসা করুন কোন অ্যাপ
settings-default-apps-after-saving = সেভ করার পরে
settings-default-apps-show-folder = সেভ করা ফাইল তাদের ফোল্ডারে দেখান
settings-default-apps-show-folder-detail = ফাইল ম্যানেজার খোলে, সেভ করা অ্যাটাচমেন্টগুলি বাছাই করা অবস্থায়

## Settings > Compose

settings-compose-send-from = নতুন মেসেজ যেখান থেকে পাঠানো হবে
settings-compose-send-from-detail = উত্তর ও ফরোয়ার্ড সবসময় আপনি যে অ্যাকাউন্টে আছেন সেখান থেকে যায়।
settings-compose-send-from-current = আপনি যে অ্যাকাউন্টে আছেন
settings-compose-send-on-replies = উত্তরে পাঠান
settings-compose-send-on-replies-detail = উত্তর বা ফরোয়ার্ডে “পাঠান” কী করে। “পাঠান”-এর পাশের মেনুতে অন্যটি পাবেন।
settings-compose-send-plain = পাঠান
settings-compose-send-archive = পাঠান ও আর্কাইভ করুন
settings-compose-signatures = স্বাক্ষর
settings-compose-signatures-detail = আপনার মেসেজের নিচে, একটি “--” লাইনের পরে যোগ করা হয়। লেখার উইন্ডোতে অন্য একটি বেছে নিন।
settings-compose-untitled = শিরোনামহীন
settings-compose-signature-name = নাম, যেমন অফিস
settings-compose-signature-first = আমার স্বাক্ষর
settings-compose-signature-numbered = স্বাক্ষর { $number }
settings-compose-signature-delete = মুছুন
settings-compose-signature-deleted = স্বাক্ষর মুছে ফেলা হয়েছে
settings-compose-signature-new = নতুন তৈরি করুন
settings-compose-no-signatures = এখনও কোনো স্বাক্ষর নেই।
settings-compose-no-signature = কোনো স্বাক্ষর নেই
settings-compose-for-new-mail = নতুন মেলের জন্য
settings-compose-for-replies = উত্তর ও ফরোয়ার্ডের জন্য
settings-compose-for-replies-detail = যে কথোপকথনে আপনি কোনো মেসেজে স্বাক্ষর দিয়েছেন, সেখানে উত্তর সেই স্বাক্ষর দিয়েই শুরু হয়।
settings-compose-format = ফর্ম্যাট
settings-compose-plain-text = সাধারণ টেক্সটে লিখুন
settings-compose-plain-text-detail = নতুন মেল ফর্ম্যাটিং ছাড়া শুরু হয়; লেখার উইন্ডোতে বদলানো যায়
settings-compose-spelling = বানান
settings-compose-spell-check = লেখার সময় বানান যাচাই করুন
settings-compose-spell-check-detail = ভুল বানানের শব্দের নিচে দাগ থাকে, রাইট-ক্লিকে পরামর্শ পাওয়া যায়
settings-compose-spell-desktop = ডেস্কটপের ভাষা ({ $language })
settings-compose-templates = টেমপ্লেট
settings-compose-templates-detail = যে মেল আপনি প্রায়ই লেখেন তা সেভ করুন, আর সেখান থেকে নতুন মেল বা উত্তর শুরু করুন।

## Settings > Shortcuts

settings-shortcuts-set = শর্টকাট সেট
settings-shortcuts-set-detail = আপনার চেনা কোনো মেল অ্যাপের কী দিয়ে শুরু করুন। এখানে Cmd মানে Ctrl। আপনার নিজের পরিবর্তনগুলি সেটের উপরে থেকে যায়, আর “ডিফল্ট ফিরিয়ে আনুন” সেটের কী-তে ফিরে যায়।
settings-shortcuts-single = এক কী-এর শর্টকাট
settings-shortcuts-single-detail = Ctrl বা Alt ছাড়া কী, যেমন ওয়েবমেলে: e আর্কাইভ করে, j ও k সরায়, / খোঁজে। এগুলি তালিকায় ও খোলা কথোপকথনে কাজ করে, টাইপ করার সময় কখনো নয়।
settings-shortcuts-single-use = এক কী-এর শর্টকাট ব্যবহার করুন
settings-shortcuts-single-use-detail = Ctrl শর্টকাট সবসময় কাজ করে
settings-shortcuts-how = বদলাতে একটি কী-তে ক্লিক করুন, বা নতুন যোগ করতে + চাপুন, তারপর নতুন কী চাপুন। Esc চাপলে বাতিল হয়।
settings-shortcuts-restore = ডিফল্ট ফিরিয়ে আনুন
settings-shortcuts-no-key = কোনো কী নেই
settings-shortcuts-press = কী চাপুন…
settings-shortcuts-then = { $keys } তারপর…
settings-shortcuts-moved = { $keys } এখন “{ $previous }”-এর বদলে “{ $action }” করে।
settings-shortcuts-single-off = এক কী-এর শর্টকাট বন্ধ আছে, তাই এগুলি চালু করলে তবেই এই কী কাজ করবে।
settings-shortcuts-restored = প্রতিটি শর্টকাট আবার তার সেটের কী পেয়েছে।

## Settings search: the line under a result

settings-general-language-summary = অ্যাপ, তারিখ ও সংখ্যার ভাষা
settings-general-reading-summary = সবচেয়ে নতুন মেসেজ আগে, সম্পূর্ণ হেডার, প্রাপকদের পুরো নাম
settings-general-mark-read-summary = খোলা কথোপকথন কখন পঠিত হিসেবে চিহ্নিত হবে: সাথে সাথে, 1 বা 3 সেকেন্ড পরে, বা নিজে হাতে
settings-general-reply-button-summary = প্রতিটি মেসেজের পাশের উত্তর বোতাম সবাইকে উত্তর দেয়
settings-general-remote-images-summary = প্রতিটি মেসেজের ছবি সবসময় দেখান
settings-general-sending-summary = পাঠানো পূর্বাবস্থায় ফেরান: পাঠানো মেসেজ কতক্ষণ অপেক্ষা করবে, যাতে সেটি ফিরিয়ে নেওয়া যায়
settings-general-offline-summary = কত দিনের সাম্প্রতিক মেল পুরোপুরি ডাউনলোড হবে, যাতে কানেকশন ছাড়াই পড়া যায়
settings-general-notifications-summary = নতুন মেলের বিজ্ঞপ্তি ও তার শব্দ
settings-general-desktop-summary = লগ ইন করলে Katna চালু করা, সিস্টেম ট্রে আইকন ও টাস্কবারের আইকনে অপঠিত সংখ্যা
settings-accounts-accounts-summary = অ্যাকাউন্ট যোগ করুন বা সরান, অথবা তার ছবি বদলান
settings-appearance-density-summary = তালিকায় ডিফল্ট বা কমপ্যাক্ট লাইন
settings-appearance-scaling-summary = সবকিছু বড় বা ছোট করুন: লেখা, আইকন, ফাঁকা জায়গা ও বিভাজক
settings-appearance-theme-summary = সিস্টেম, লাইট বা ডার্ক
settings-appearance-sender-pictures-summary = কোম্পানির লোগো, প্রেরকের ডোমেন দিয়ে খোঁজা
settings-appearance-important-summary = তালিকায় প্রতিটি মেসেজের পাশে গুরুত্বপূর্ণ চিহ্ন
settings-appearance-mail-colors-summary = ডার্ক থিমে HTML মেলের জন্য গাঢ় রং, বা প্রেরকের রং
settings-appearance-attachment-previews-summary = প্রতিটি অ্যাটাচমেন্টের বিষয়বস্তুর একটি ছোট ছবি
settings-shortcuts-set-summary = Gmail, Inbox by Gmail, Apple Mail, Outlook বা Thunderbird-এর কী দিয়ে শুরু করুন
settings-shortcuts-single-summary = Ctrl বা Alt ছাড়া কী, যেমন ওয়েবমেলে
settings-default-apps-pdf-summary = PDF অ্যাটাচমেন্ট কোথায় খুলবে
settings-default-apps-pictures-summary = ফটো ও ছবি কোথায় খুলবে
settings-default-apps-text-summary = সাধারণ টেক্সট, লগ ও কোড কোথায় খুলবে
settings-default-apps-sheets-summary = Excel, OpenDocument ও CSV ফাইল কোথায় খুলবে
settings-default-apps-documents-summary = Word, OpenDocument টেক্সট ও স্লাইড কোথায় খুলবে
settings-default-apps-after-saving-summary = সেভ করা অ্যাটাচমেন্ট তাদের ফোল্ডারে দেখান
settings-compose-send-from-summary = নতুন মেল কোন অ্যাকাউন্ট থেকে যাবে: আপনি যেটিতে আছেন, বা সবসময় একই অ্যাকাউন্ট
settings-compose-send-on-replies-summary = উত্তর ও ফরোয়ার্ডে পাঠান, অথবা পাঠান ও কথোপকথন আর্কাইভ করুন
settings-compose-signatures-summary = আপনার মেসেজের নিচে, একটি “--” লাইনের পরে যোগ করা হয়
settings-compose-for-new-mail-summary = নতুন মেল যে স্বাক্ষর দিয়ে শুরু হয়
settings-compose-for-replies-summary = উত্তর ও ফরোয়ার্ড যে স্বাক্ষর দিয়ে শুরু হয়
settings-compose-format-summary = নতুন মেল সাধারণ টেক্সটে লিখুন
settings-compose-spelling-summary = লেখার সময় বানান যাচাই, এবং অভিধানের ভাষা
settings-compose-templates-summary = শীঘ্রই আসছে: যে মেল আপনি প্রায়ই লেখেন তা সেভ করুন, আর সেখান থেকে নতুন মেল বা উত্তর শুরু করুন
settings-feedback-crash-reports-summary = Katna Mail বা তার ব্যাকগ্রাউন্ড পরিষেবা ক্র্যাশ করলে এই কম্পিউটারে ক্র্যাশ রিপোর্ট সেভ করুন
settings-feedback-saved-summary = এই কম্পিউটারে সেভ করা ক্র্যাশ রিপোর্ট দেখুন, কপি করুন বা মুছুন
settings-feedback-help-improve-summary = কী ভুল হয়েছে তা ঠিক করতে সাহায্যের জন্য ক্র্যাশ রিপোর্ট পাঠান; আপনি চালু না করলে বন্ধ থাকে
settings-experimental-blur-summary = উপরের বারের ভেতর দিয়ে ডেস্কটপ ঝাপসা হয়ে দেখা যায়, আর মেনুগুলি ঘষা কাচের মতো দেখায়
settings-search-shortcut = কীবোর্ড শর্টকাট
settings-search-tab = সেটিংস ট্যাব
settings-search-none = “{ $query }”-এর সাথে মেলে এমন কোনো সেটিং নেই।
settings-search-results = “{ $query }”-এর সাথে মেলে এমন সেটিংস

## Settings: opening at login

settings-open-at-login-failed = লগ ইনের সময় চালু হওয়ার সেটিং বদলানো যায়নি: { $error }

## Settings > General > Time

settings-time = সময়
settings-clock-language = ভাষা যেভাবে লেখে
settings-clock-12 = ১২ ঘণ্টা, যেমন ২:০৫ PM
settings-clock-24 = ২৪ ঘণ্টা, যেমন ১৪:০৫
settings-time-summary = ১২ ঘণ্টা বা ২৪ ঘণ্টার ঘড়ি, বা ভাষা যেভাবে লেখে

## Settings > General > Default mail app, Settings > Compose > Grammar

settings-general-mail-app = ডিফল্ট মেল অ্যাপ
settings-general-mail-app-detail = অন্য অ্যাপ ও ওয়েবসাইটের ইমেল লিঙ্ক এখানে নতুন মেসেজ খোলে।
mail-app-is-default = Katna Mail আপনার ডিফল্ট মেল অ্যাপ।
mail-app-is-other = ইমেল লিঙ্ক অন্য অ্যাপে খোলে।
mail-app-make-default = ডিফল্ট করুন
mail-app-make-default-failed = ডিফল্ট মেল অ্যাপ বদলানো যায়নি।
settings-general-mail-app-summary = অন্য অ্যাপ ও ওয়েবসাইটের ইমেল লিঙ্ক Katna Mail-এ খুলুন
settings-compose-grammar = ব্যাকরণ
settings-compose-grammar-detail = এই কম্পিউটারেই Harper দিয়ে যাচাই করা হয়। আপাতত শুধু ইংরেজি: অন্য ভাষার লেখায় হাত দেওয়া হয় না।
settings-compose-grammar-check = ব্যাকরণ যাচাই করুন
settings-compose-grammar-check-detail = লেখার সময় ব্যাকরণের ভুলের নিচে দাগ দিন, ইংরেজিতে
settings-compose-suggestions = লেখার পরামর্শ
settings-compose-suggestions-detail = আপনার পাঠানো মেল আর যে মেলের উত্তর দিচ্ছেন তা থেকে এই কম্পিউটারেই শেখা; কিছুই এর বাইরে যায় না। পরামর্শ নিতে Tab চাপুন, অথবা টাইপ করে যান।
settings-compose-suggestions-on = লেখার সময় পরামর্শ দিন
settings-compose-suggestions-on-detail = টাইপ করার সময় বাক্যাংশের সম্ভাব্য বাকি অংশ ধূসর রঙে দেখান
settings-compose-grammar-summary = লেখার সময় ব্যাকরণের ভুলের নিচে দাগ দিন, ইংরেজিতে
settings-compose-suggestions-summary = টাইপ করার সময় বাক্যাংশের সম্ভাব্য বাকি অংশ ধূসর রঙে দেখান
