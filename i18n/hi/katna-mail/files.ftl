# Katna Mail, Hindi (हिन्दी).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = फ़ाइलें खोजें

## Left side (and chips on a phone)

files-all = सभी फ़ाइलें
files-pictures = तस्वीरें
files-pdfs = PDF
files-documents = दस्तावेज़
files-sheets = स्प्रेडशीट
files-slides = स्लाइड
files-other = अन्य
files-accounts = खाते
files-drives = ड्राइव
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = मेरे साथ शेयर की गई
files-shown = दिखाएं
files-received = मिली हुई
files-sent = मेरे द्वारा भेजी गई

## Over the files

files-count = { $count ->
    [one] { $count } फ़ाइल · { $size }
   *[other] { $count } फ़ाइलें · { $size }
}
files-anyone = कोई भी
files-from-person = { $name } से
files-time-any = कभी भी
files-time-today = आज
files-time-yesterday = कल
files-time-this-week = इस हफ़्ते
files-time-last-week = पिछले हफ़्ते
files-time-this-month = इस महीने
files-time-last-month = पिछले महीने
files-time-between = { $first } – { $last }
files-time-hint = किसी दिन पर क्लिक करें, या कई दिनों पर खींचें
files-time-summary = { $count ->
    [one] { $days } · { $count } फ़ाइल
   *[other] { $days } · { $count } फ़ाइलें
}
files-time-clear = मिटाएं
files-time-month-back = पिछला महीना
files-time-month-on = अगला महीना
files-time-wheel = इन तारीखों को खिसकाने के लिए स्क्रॉल करें, अवधि वही रहती है
files-sort-newest = सबसे नई पहले
files-sort-oldest = सबसे पुरानी पहले
files-sort-largest = सबसे बड़ी पहले
files-sort-name = नाम के अनुसार
files-grid = कार्ड
files-list = सूची
files-this-week = इस हफ़्ते
files-undated = कोई तारीख नहीं
files-me = मैं
files-no-subject = (कोई विषय नहीं)
files-loading = आपके मेल से फ़ाइलें इकट्ठी की जा रही हैं…
files-empty = आपके मेल की फ़ाइलें यहां दिखती हैं।
files-none-match = कोई फ़ाइल मेल नहीं खाती।
files-load-failed = फ़ाइलें पढ़ी नहीं जा सकीं: { $error }

## A file's menu and buttons

files-open = खोलें
files-open-with = इससे खोलें…
files-save = सेव करें…
files-show-mail = मेल दिखाएं
files-mail-window = मेल को नई विंडो में खोलें
files-forward = फ़ाइल फ़ॉरवर्ड करें
files-from-them = { $name } की फ़ाइलें
files-copy-name = फ़ाइल का नाम कॉपी करें
files-name-copied = फ़ाइल का नाम कॉपी किया गया
files-downloading = मेल डाउनलोड हो रहा है…
files-download-failed = यह मेल डाउनलोड नहीं किया जा सका।

## A cloud drive in place of the mail files

files-drive-mine = मेरी ड्राइव
files-drive-mine-onedrive = मेरी फ़ाइलें
files-drive-results = “{ $words }”
files-drive-count = { $folders ->
    [0] { $files ->
        [one] 1 फ़ाइल
       *[other] { $files } फ़ाइलें
    }
    [one] 1 फ़ोल्डर · { $files ->
        [one] 1 फ़ाइल
       *[other] { $files } फ़ाइलें
    }
   *[other] { $folders } फ़ोल्डर · { $files ->
        [one] 1 फ़ाइल
       *[other] { $files } फ़ाइलें
    }
}
files-drive-folders = फ़ोल्डर
files-drive-files = फ़ाइलें
files-drive-folder = फ़ोल्डर
files-drive-meta = { $what } · { $date } को बदला गया
files-drive-as-link = { $what } · लिंक के रूप में
files-drive-google-doc = Google Doc
files-drive-google-sheet = Google Sheet
files-drive-google-slides = Google Slides
files-drive-google-drawing = Google Drawing
files-drive-fetching = लाया जा रहा है…
files-drive-loading = ड्राइव खोली जा रही है…
files-drive-empty = यह फ़ोल्डर खाली है।
files-drive-unreachable = { $drive } तक नहीं पहुंचा जा सका।
files-drive-try-again = फिर से कोशिश करें
files-drive-needs-permission = यह ड्राइव दिखाने के लिए Katna को एक बार आपकी अनुमति चाहिए। फिर से साइन इन करें और Katna को अपनी फ़ाइलें देखने दें।
files-drive-allow = अनुमति दें
files-drive-allow-failed = साइन इन पूरा नहीं हुआ, इसलिए ड्राइव बंद ही रहेगी।
files-drive-attach = अटैच करें
files-drive-more = ज़्यादा
files-drive-download = डाउनलोड करें…
files-drive-open-web = { $drive } में खोलें
files-drive-copy-link = लिंक कॉपी करें
files-drive-link-copied = लिंक कॉपी किया गया
files-drive-share = शेयर करें…
files-drive-rename = नाम बदलें
files-drive-trash = ट्रैश में ले जाएं
files-drive-trashed = “{ $name }” { $drive } के ट्रैश में है
files-drive-renamed = नाम बदलकर “{ $name }” किया गया
files-drive-getting = { $drive } से { $name } लाई जा रही है…
files-drive-get-failed = { $name } नहीं लाई जा सकी: { $error }
files-drive-upload = अपलोड करें
files-drive-upload-files = फ़ाइलें अपलोड करें
files-drive-upload-folder = फ़ोल्डर अपलोड करें
files-drive-upload-failed = { $name } अपलोड नहीं की जा सकी: { $error }
files-drive-upload-needs = अपलोड करने के लिए Katna को एक बार आपकी अनुमति चाहिए: सेटिंग › डिफ़ॉल्ट ऐप › फ़ाइलें पेज में अनुमति दें दबाएं।

## The Share dialog of a drive file or folder

files-share-title = “{ $name }” शेयर करें
files-share-add = नाम या पते से लोगों को जोड़ें
files-share-not-address = “{ $text }” ईमेल पता नहीं है
files-share-notify = { $drive } को उन्हें ईमेल भी भेजने दें
files-share-people = जिन लोगों के पास ऐक्सेस है
files-share-general = सामान्य ऐक्सेस
files-share-loading = देखा जा रहा है कि किसके पास ऐक्सेस है…
files-share-restricted = प्रतिबंधित
files-share-restricted-about = लिंक से सिर्फ़ ऐक्सेस वाले लोग ही इसे खोल सकते हैं
files-share-anyone = लिंक वाला कोई भी व्यक्ति
files-share-anyone-can = { $role ->
    [editor] लिंक वाला कोई भी व्यक्ति बदलाव कर सकता है
    [commenter] लिंक वाला कोई भी व्यक्ति टिप्पणी कर सकता है
   *[viewer] लिंक वाला कोई भी व्यक्ति देख सकता है
}
files-share-anyone-about = { $role ->
    [editor] इंटरनेट पर लिंक वाला कोई भी व्यक्ति बदलाव कर सकता है
    [commenter] इंटरनेट पर लिंक वाला कोई भी व्यक्ति टिप्पणी कर सकता है
   *[viewer] इंटरनेट पर लिंक वाला कोई भी व्यक्ति देख सकता है
}
files-share-role-owner = मालिक
files-share-role-editor = एडिटर
files-share-role-commenter = टिप्पणीकार
files-share-role-viewer = दर्शक
files-share-you = { $name } (आप)
files-share-domain = { $domain } पर सभी
files-share-inherited = जिस फ़ोल्डर में यह है, उससे मिला ऐक्सेस
files-share-remove = ऐक्सेस हटाएं
files-share-copy-link = लिंक कॉपी करें
files-share-share = शेयर करें
files-share-done = हो गया
files-share-close = बंद करें
files-share-sharing = शेयर किया जा रहा है…
files-share-shared = { $count ->
    [one] 1 व्यक्ति के साथ शेयर किया गया
   *[other] { $count } लोगों के साथ शेयर किया गया
}
files-share-refused = { $drive } { $addresses } के साथ शेयर नहीं कर सका
files-share-failed = शेयरिंग नहीं बदली जा सकी: { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
    [one] 1 आइटम अपलोड हो रहा है
   *[other] { $count } आइटम अपलोड हो रहे हैं
}
files-tray-done = { $count ->
    [one] 1 अपलोड पूरा हुआ
   *[other] { $count } अपलोड पूरे हुए
}
files-tray-some-failed = { $done } अपलोड हुए, { $failed } नहीं हो सके
files-tray-minutes-left = { $minutes ->
    [one] लगभग एक मिनट बाकी
   *[other] लगभग { $minutes } मिनट बाकी
}
files-tray-seconds-left = एक मिनट से कम बाकी
files-tray-starting = शुरू हो रहा है…
files-tray-cancel-all = सभी रद्द करें
files-tray-cancel = रद्द करें
files-tray-fold = सूची छिपाएं
files-tray-unfold = सूची दिखाएं
files-tray-close = बंद करें
files-tray-progress = { $place } · { $size } में से { $sent }
files-tray-in = { $place } में
files-tray-cancelled = रद्द किया गया
