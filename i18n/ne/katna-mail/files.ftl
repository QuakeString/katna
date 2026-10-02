# Katna Mail, Nepali (नेपाली).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = फाइलहरू खोज्नुहोस्

## Left side (and chips on a phone)

files-all = सबै फाइलहरू
files-pictures = तस्बिरहरू
files-pdfs = PDF हरू
files-documents = कागजातहरू
files-sheets = स्प्रेडसिटहरू
files-slides = स्लाइडहरू
files-other = अन्य
files-accounts = खाताहरू
files-drives = ड्राइभहरू
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = मसँग सेयर गरिएका
files-shown = देखाइएका
files-received = प्राप्त
files-sent = मैले पठाएका

## Over the files

files-count = { $count ->
    [one] { $count } फाइल · { $size }
   *[other] { $count } फाइलहरू · { $size }
}
files-anyone = जो कोही
files-from-person = { $name } बाट
files-time-any = जुनसुकै समय
files-time-today = आज
files-time-yesterday = हिजो
files-time-this-week = यो हप्ता
files-time-last-week = गत हप्ता
files-time-this-month = यो महिना
files-time-last-month = गत महिना
files-time-between = { $first } – { $last }
files-time-hint = कुनै दिनमा क्लिक गर्नुहोस्, वा दिनहरूमाथि तान्नुहोस्
files-time-summary = { $count ->
    [one] { $days } · { $count } फाइल
   *[other] { $days } · { $count } फाइलहरू
}
files-time-clear = हटाउनुहोस्
files-time-month-back = अघिल्लो महिना
files-time-month-on = अर्को महिना
files-time-wheel = लम्बाइ उस्तै राखेर यी मितिहरू सार्न स्क्रोल गर्नुहोस्
files-sort-newest = सबैभन्दा नयाँ पहिले
files-sort-oldest = सबैभन्दा पुरानो पहिले
files-sort-largest = सबैभन्दा ठूलो पहिले
files-sort-name = नामअनुसार
files-grid = कार्डहरू
files-list = सूची
files-this-week = यो हप्ता
files-undated = मिति छैन
files-me = म
files-no-subject = (विषय छैन)
files-loading = तपाईंको मेलबाट फाइलहरू जम्मा गर्दै…
files-empty = तपाईंको मेलका फाइलहरू यहाँ देखिन्छन्।
files-none-match = कुनै फाइल मिलेन।
files-load-failed = फाइलहरू पढ्न सकिएन: { $error }

## A file's menu and buttons

files-open = खोल्नुहोस्
files-open-with = यसमा खोल्नुहोस्…
files-save = सेभ गर्नुहोस्…
files-show-mail = मेल देखाउनुहोस्
files-mail-window = मेल नयाँ विन्डोमा खोल्नुहोस्
files-forward = फाइल फर्वार्ड गर्नुहोस्
files-from-them = { $name } का फाइलहरू
files-copy-name = फाइलको नाम कपी गर्नुहोस्
files-name-copied = फाइलको नाम कपी गरियो
files-downloading = मेल डाउनलोड गर्दै…
files-download-failed = यो मेल डाउनलोड गर्न सकिएन।

## A cloud drive in place of the mail files

files-drive-mine = मेरो ड्राइभ
files-drive-mine-onedrive = मेरा फाइलहरू
files-drive-results = “{ $words }”
files-drive-count = { $folders ->
    [0] { $files ->
        [one] 1 फाइल
       *[other] { $files } फाइलहरू
    }
    [one] 1 फोल्डर · { $files ->
        [one] 1 फाइल
       *[other] { $files } फाइलहरू
    }
   *[other] { $folders } फोल्डरहरू · { $files ->
        [one] 1 फाइल
       *[other] { $files } फाइलहरू
    }
}
files-drive-folders = फोल्डरहरू
files-drive-files = फाइलहरू
files-drive-folder = फोल्डर
files-drive-meta = { $what } · { $date } मा सम्पादन गरिएको
files-drive-as-link = { $what } · लिङ्कका रूपमा
files-drive-google-doc = Google Doc
files-drive-google-sheet = Google Sheet
files-drive-google-slides = Google Slides
files-drive-google-drawing = Google Drawing
files-drive-fetching = ल्याउँदै…
files-drive-loading = ड्राइभ खोल्दै…
files-drive-empty = यो फोल्डर खाली छ।
files-drive-unreachable = { $drive } सम्म पुग्न सकिएन।
files-drive-try-again = फेरि प्रयास गर्नुहोस्
files-drive-needs-permission = यो ड्राइभ देखाउन Katna लाई एक पटक तपाईंको अनुमति चाहिन्छ। फेरि साइन इन गर्नुहोस् र Katna लाई तपाईंका फाइलहरू हेर्न दिनुहोस्।
files-drive-allow = अनुमति दिनुहोस्
files-drive-allow-failed = साइन इन पूरा भएन, त्यसैले ड्राइभ बन्दै रहन्छ।
files-drive-attach = संलग्न गर्नुहोस्
files-drive-more = थप
files-drive-download = डाउनलोड गर्नुहोस्…
files-drive-open-web = { $drive } मा खोल्नुहोस्
files-drive-copy-link = लिङ्क कपी गर्नुहोस्
files-drive-link-copied = लिङ्क कपी गरियो
files-drive-share = सेयर गर्नुहोस्…
files-drive-rename = नाम बदल्नुहोस्
files-drive-trash = रद्दीटोकरीमा सार्नुहोस्
files-drive-trashed = “{ $name }” { $drive } को रद्दीटोकरीमा छ
files-drive-renamed = नाम बदलेर “{ $name }” बनाइयो
files-drive-getting = { $drive } बाट { $name } ल्याउँदै…
files-drive-get-failed = { $name } ल्याउन सकिएन: { $error }
files-drive-upload = अपलोड गर्नुहोस्
files-drive-upload-files = फाइलहरू अपलोड गर्नुहोस्
files-drive-upload-folder = फोल्डर अपलोड गर्नुहोस्
files-drive-upload-failed = { $name } अपलोड गर्न सकिएन: { $error }
files-drive-upload-needs = अपलोड गर्न Katna लाई एक पटक तपाईंको अनुमति चाहिन्छ: सेटिङहरू › पूर्वनिर्धारित एपहरू › फाइलहरू पृष्ठमा अनुमति दिनुहोस् थिच्नुहोस्।

## The Share dialog of a drive file or folder

files-share-title = “{ $name }” सेयर गर्नुहोस्
files-share-add = नाम वा ठेगानाद्वारा मानिसहरू थप्नुहोस्
files-share-not-address = “{ $text }” इमेल ठेगाना होइन
files-share-notify = { $drive } ले उहाँहरूलाई इमेल पनि पठाओस्
files-share-people = पहुँच भएका मानिसहरू
files-share-general = सामान्य पहुँच
files-share-loading = कससँग पहुँच छ पढ्दै…
files-share-restricted = सीमित
files-share-restricted-about = पहुँच भएका मानिसहरूले मात्र लिङ्कबाट यो खोल्न सक्छन्
files-share-anyone = लिङ्क भएका जो कोही
files-share-anyone-can = { $role ->
    [editor] लिङ्क भएका जो कोहीले सम्पादन गर्न सक्छन्
    [commenter] लिङ्क भएका जो कोहीले टिप्पणी गर्न सक्छन्
   *[viewer] लिङ्क भएका जो कोहीले हेर्न सक्छन्
}
files-share-anyone-about = { $role ->
    [editor] इन्टरनेटमा लिङ्क भएका जो कोहीले सम्पादन गर्न सक्छन्
    [commenter] इन्टरनेटमा लिङ्क भएका जो कोहीले टिप्पणी गर्न सक्छन्
   *[viewer] इन्टरनेटमा लिङ्क भएका जो कोहीले हेर्न सक्छन्
}
files-share-role-owner = मालिक
files-share-role-editor = सम्पादक
files-share-role-commenter = टिप्पणीकर्ता
files-share-role-viewer = दर्शक
files-share-you = { $name } (तपाईं)
files-share-domain = { $domain } का सबै
files-share-inherited = यो रहेको फोल्डरबाट पाएको पहुँच
files-share-remove = पहुँच हटाउनुहोस्
files-share-copy-link = लिङ्क कपी गर्नुहोस्
files-share-share = सेयर गर्नुहोस्
files-share-done = सकियो
files-share-sharing = सेयर गर्दै…
files-share-shared = { $count ->
    [one] 1 व्यक्तिसँग सेयर गरियो
   *[other] { $count } व्यक्तिसँग सेयर गरियो
}
files-share-refused = { $drive } ले { $addresses } सँग सेयर गर्न सकेन
files-share-failed = सेयरिङ बदल्न सकिएन: { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
    [one] 1 वस्तु अपलोड गर्दै
   *[other] { $count } वस्तुहरू अपलोड गर्दै
}
files-tray-done = { $count ->
    [one] 1 अपलोड सकियो
   *[other] { $count } अपलोड सकिए
}
files-tray-some-failed = { $done } अपलोड भए, { $failed } असफल
files-tray-minutes-left = { $minutes ->
    [one] लगभग एक मिनेट बाँकी
   *[other] लगभग { $minutes } मिनेट बाँकी
}
files-tray-seconds-left = एक मिनेटभन्दा कम बाँकी
files-tray-starting = सुरु गर्दै…
files-tray-cancel-all = सबै रद्द गर्नुहोस्
files-tray-cancel = रद्द गर्नुहोस्
files-tray-fold = सूची लुकाउनुहोस्
files-tray-unfold = सूची देखाउनुहोस्
files-tray-close = बन्द गर्नुहोस्
files-tray-progress = { $place } · { $size } मध्ये { $sent }
files-tray-in = { $place } मा
files-tray-cancelled = रद्द गरियो
