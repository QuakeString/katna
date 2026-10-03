# Katna Mail, Marathi (मराठी).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = फाइल्स शोधा

## Left side (and chips on a phone)

files-all = सर्व फाइल्स
files-pictures = चित्रे
files-pdfs = PDF
files-documents = दस्तऐवज
files-sheets = स्प्रेडशीट
files-slides = स्लाइड
files-other = इतर
files-accounts = खाती
files-drives = ड्राइव्ह
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = माझ्यासोबत शेअर केलेले
files-shown = दाखवलेले
files-received = मिळालेले
files-sent = मी पाठवलेले

## Over the files

files-count = { $count ->
    [one] { $count } फाइल · { $size }
   *[other] { $count } फाइल्स · { $size }
}
files-anyone = कोणीही
files-from-person = { $name } यांच्याकडून
files-time-any = कधीही
files-time-today = आज
files-time-yesterday = काल
files-time-this-week = हा आठवडा
files-time-last-week = मागचा आठवडा
files-time-this-month = हा महिना
files-time-last-month = मागचा महिना
files-time-between = { $first } – { $last }
files-time-hint = एखाद्या दिवसावर क्लिक करा, किंवा दिवसांवरून ड्रॅग करा
files-time-summary = { $count ->
    [one] { $days } · { $count } फाइल
   *[other] { $days } · { $count } फाइल्स
}
files-time-clear = साफ करा
files-time-month-back = मागचा महिना
files-time-month-on = पुढचा महिना
files-time-wheel = लांबी तशीच ठेवून या तारखा हलवण्यासाठी स्क्रोल करा
files-sort-newest = सर्वात नवीन आधी
files-sort-oldest = सर्वात जुने आधी
files-sort-largest = सर्वात मोठे आधी
files-sort-name = नावानुसार
files-grid = कार्ड
files-list = यादी
files-this-week = हा आठवडा
files-undated = तारीख नाही
files-me = मी
files-no-subject = (विषय नाही)
files-loading = तुमच्या मेलमधून फाइल्स गोळा करत आहे…
files-empty = तुमच्या मेलमधील फाइल्स येथे दिसतात.
files-none-match = कोणत्याही फाइल्स जुळत नाहीत.
files-load-failed = फाइल्स वाचता आल्या नाहीत: { $error }

## A file's menu and buttons

files-open = उघडा
files-open-with = यासह उघडा…
files-save = सेव्ह करा…
files-show-mail = मेल दाखवा
files-mail-window = मेल नवीन विंडोमध्ये उघडा
files-forward = फाइल फॉरवर्ड करा
files-from-them = { $name } यांच्याकडील फाइल्स
files-copy-name = फाइलचे नाव कॉपी करा
files-name-copied = फाइलचे नाव कॉपी केले
files-downloading = मेल डाउनलोड करत आहे…
files-download-failed = हा मेल डाउनलोड करता आला नाही.

## A cloud drive in place of the mail files

files-drive-mine = माझी ड्राइव्ह
files-drive-mine-onedrive = माझ्या फाइल्स
files-drive-results = “{ $words }”
files-drive-count = { $folders ->
    [0] { $files ->
        [one] 1 फाइल
       *[other] { $files } फाइल्स
    }
    [one] 1 फोल्डर · { $files ->
        [one] 1 फाइल
       *[other] { $files } फाइल्स
    }
   *[other] { $folders } फोल्डर · { $files ->
        [one] 1 फाइल
       *[other] { $files } फाइल्स
    }
}
files-drive-folders = फोल्डर
files-drive-files = फाइल्स
files-drive-folder = फोल्डर
files-drive-meta = { $what } · { $date } रोजी संपादित
files-drive-as-link = { $what } · लिंक म्हणून
files-drive-google-doc = Google Doc
files-drive-google-sheet = Google Sheet
files-drive-google-slides = Google Slides
files-drive-google-drawing = Google Drawing
files-drive-fetching = मिळवत आहे…
files-drive-loading = ड्राइव्ह उघडत आहे…
files-drive-empty = हे फोल्डर रिकामे आहे.
files-drive-unreachable = { $drive } पर्यंत पोहोचता येत नाही.
files-drive-try-again = पुन्हा प्रयत्न करा
files-drive-needs-permission = ही ड्राइव्ह दाखवण्यासाठी Katna ला एकदा तुमची परवानगी हवी आहे. पुन्हा साइन इन करा आणि Katna ला तुमच्या फाइल्स पाहू द्या.
files-drive-allow = परवानगी द्या
files-drive-allow-failed = साइन इन पूर्ण झाले नाही, त्यामुळे ड्राइव्ह बंदच राहते.
files-drive-attach = अटॅच करा
files-drive-more = अधिक
files-drive-download = डाउनलोड करा…
files-drive-open-web = { $drive } मध्ये उघडा
files-drive-copy-link = लिंक कॉपी करा
files-drive-link-copied = लिंक कॉपी केली
files-drive-share = शेअर करा…
files-drive-rename = नाव बदला
files-drive-trash = कचरापेटीत हलवा
files-drive-trashed = “{ $name }” { $drive } च्या कचरापेटीत आहे
files-drive-renamed = नाव बदलून “{ $name }” केले
files-drive-getting = { $drive } वरून { $name } मिळवत आहे…
files-drive-get-failed = { $name } मिळवता आले नाही: { $error }
files-drive-upload = अपलोड करा
files-drive-upload-files = फाइल्स अपलोड करा
files-drive-upload-folder = फोल्डर अपलोड करा
files-drive-upload-failed = { $name } अपलोड करता आले नाही: { $error }
files-drive-upload-needs = अपलोड करण्यासाठी Katna ला एकदा तुमची परवानगी हवी आहे: सेटिंग्ज › डीफॉल्ट ॲप्स › फाइल्स पेज मध्ये परवानगी द्या दाबा.

## The Share dialog of a drive file or folder

files-share-title = “{ $name }” शेअर करा
files-share-add = नाव किंवा पत्त्याने लोकांना जोडा
files-share-not-address = “{ $text }” हा ईमेल पत्ता नाही
files-share-notify = { $drive } ला त्यांना ईमेलही पाठवू द्या
files-share-people = ॲक्सेस असलेले लोक
files-share-general = सामान्य ॲक्सेस
files-share-loading = कोणाला ॲक्सेस आहे ते वाचत आहे…
files-share-restricted = प्रतिबंधित
files-share-restricted-about = फक्त ॲक्सेस असलेले लोकच लिंकने ते उघडू शकतात
files-share-anyone = लिंक असलेली कोणतीही व्यक्ती
files-share-anyone-can = { $role ->
    [editor] लिंक असलेली कोणतीही व्यक्ती संपादित करू शकते
    [commenter] लिंक असलेली कोणतीही व्यक्ती टिप्पणी करू शकते
   *[viewer] लिंक असलेली कोणतीही व्यक्ती पाहू शकते
}
files-share-anyone-about = { $role ->
    [editor] इंटरनेटवरील लिंक असलेली कोणतीही व्यक्ती संपादित करू शकते
    [commenter] इंटरनेटवरील लिंक असलेली कोणतीही व्यक्ती टिप्पणी करू शकते
   *[viewer] इंटरनेटवरील लिंक असलेली कोणतीही व्यक्ती पाहू शकते
}
files-share-role-owner = मालक
files-share-role-editor = संपादक
files-share-role-commenter = टिप्पणीकार
files-share-role-viewer = दर्शक
files-share-you = { $name } (तुम्ही)
files-share-domain = { $domain } मधील सर्वजण
files-share-inherited = ते असलेल्या फोल्डरकडून मिळालेला ॲक्सेस
files-share-remove = ॲक्सेस काढा
files-share-copy-link = लिंक कॉपी करा
files-share-share = शेअर करा
files-share-done = झाले
files-share-close = बंद करा
files-share-sharing = शेअर करत आहे…
files-share-shared = { $count ->
    [one] 1 व्यक्तीसोबत शेअर केले
   *[other] { $count } लोकांसोबत शेअर केले
}
files-share-refused = { $drive } ला { $addresses } सोबत शेअर करता आले नाही
files-share-failed = शेअरिंग बदलता आले नाही: { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
    [one] 1 आयटम अपलोड करत आहे
   *[other] { $count } आयटम अपलोड करत आहे
}
files-tray-done = { $count ->
    [one] 1 अपलोड पूर्ण
   *[other] { $count } अपलोड पूर्ण
}
files-tray-some-failed = { $done } अपलोड झाले, { $failed } अयशस्वी
files-tray-minutes-left = { $minutes ->
    [one] सुमारे एक मिनिट बाकी
   *[other] सुमारे { $minutes } मिनिटे बाकी
}
files-tray-seconds-left = एका मिनिटापेक्षा कमी बाकी
files-tray-starting = सुरू करत आहे…
files-tray-cancel-all = सर्व रद्द करा
files-tray-cancel = रद्द करा
files-tray-fold = यादी लपवा
files-tray-unfold = यादी दाखवा
files-tray-close = बंद करा
files-tray-progress = { $place } · { $size } पैकी { $sent }
files-tray-in = { $place } मध्ये
files-tray-cancelled = रद्द केले
