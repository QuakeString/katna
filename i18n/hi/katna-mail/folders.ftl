# Katna Mail, Hindi (हिन्दी).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Navigation (the folders pane)

nav-labels = लेबल
nav-folders = फ़ोल्डर
nav-label-new = नया लेबल बनाएं
nav-folder-new = नया फ़ोल्डर बनाएं
nav-menu-check-mail = नए मेल जाँचें
nav-menu-check-inbox = यह इनबॉक्स जाँचें
nav-unified-leave-out = एकीकृत इनबॉक्स से बाहर रखें
nav-unified-bring-back = एकीकृत इनबॉक्स में वापस लाएं
nav-menu-sign-in-again = फिर से साइन इन करें
nav-menu-new-mail = इस खाते से नया मेल
nav-menu-account-settings = खाते की सेटिंग
nav-account-checked = सिंक में · { $ago } जाँचा गया
nav-account-in-sync = सिंक में
nav-account-connecting = कनेक्ट हो रहा है…
nav-account-offline = ऑफ़लाइन, फिर से कोशिश हो रही है
nav-account-signed-out = { $provider } साइन इन की अवधि खत्म हो गई
nav-account-password-refused = पासवर्ड अस्वीकार किया गया
nav-account-storage = { $total } में से { $used } इस्तेमाल हुआ
nav-menu-new-subfolder = अंदर नया फ़ोल्डर
nav-menu-new-sublabel = अंदर नया लेबल
nav-menu-rename = नाम बदलें
nav-menu-delete = मिटाएं
nav-menu-empty-trash = ट्रैश खाली करें
nav-account-unnamed = खाता { $number }
nav-all-accounts = सभी खाते
nav-expand = फ़ोल्डर दिखाएं
nav-collapse = फ़ोल्डर छिपाएं
storage-used = { $total } में से { $percent }% इस्तेमाल हुआ
storage-used-detail = { $address }: { $total } में से { $used } इस्तेमाल हुआ

## Special folders (the user's own folders keep their names)

folder-inbox = इनबॉक्स
folder-starred = तारांकित
folder-snoozed = स्नूज़ किए गए
folder-unread = नहीं पढ़े गए
folder-important = ज़रूरी
folder-drafts = ड्राफ़्ट
folder-sent = भेजे गए
folder-archive = संग्रह
folder-spam = स्पैम
folder-trash = ट्रैश
folder-all-mail = सभी मेल
folder-scheduled = शेड्यूल किए गए
folder-waiting = जवाब का इंतज़ार
folder-waiting-short = इंतज़ार में
folder-reminders = रिमाइंडर
folder-outbox = आउटबॉक्स
folder-activity = गतिविधि

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = नया लेबल
label-folder-new-title = नया फ़ोल्डर
label-prompt = कृपया नए लेबल का नाम डालें:
label-folder-prompt = कृपया नए फ़ोल्डर का नाम डालें:
label-name-hint = लेबल का नाम
label-folder-name-hint = फ़ोल्डर का नाम
label-nest = लेबल को इसके अंदर रखें:
label-folder-nest = फ़ोल्डर को इसके अंदर रखें:
label-cancel = रद्द करें
label-create = बनाएं
label-creating = बनाया जा रहा है…
label-created = लेबल “{ $name }” बनाया गया।
label-folder-created = फ़ोल्डर “{ $name }” बनाया गया।
label-rename-title = लेबल का नाम बदलें
label-folder-rename-title = फ़ोल्डर का नाम बदलें
label-rename = नाम बदलें
label-renaming = नाम बदला जा रहा है…
label-renamed = लेबल का नाम बदलकर “{ $name }” कर दिया गया।
label-folder-renamed = फ़ोल्डर का नाम बदलकर “{ $name }” कर दिया गया।

## Deleting a folder or label (asked first)

folder-delete-title = “{ $name }” मिटाएं?
folder-delete-body = { $count ->
    [0] इसमें कोई मेल नहीं है। फ़ोल्डर सर्वर से हटा दिया जाता है, इसलिए वेबमेल और आपके फ़ोन से भी यह हट जाता है।
   *[other] { $kind ->
        [conversation] { $count ->
            [one] इसकी { $count } बातचीत ट्रैश में जाती है, ताकि आप उसे फिर भी वापस ला सकें।
           *[other] इसकी { $count } बातचीत ट्रैश में जाती हैं, ताकि आप उन्हें फिर भी वापस ला सकें।
        }
       *[message] { $count ->
            [one] इसका { $count } मैसेज ट्रैश में जाता है, ताकि आप उसे फिर भी वापस ला सकें।
           *[other] इसके { $count } मैसेज ट्रैश में जाते हैं, ताकि आप उन्हें फिर भी वापस ला सकें।
        }
    } फ़ोल्डर सर्वर से हटा दिया जाता है, इसलिए वेबमेल और आपके फ़ोन से भी यह हट जाता है।
}
folder-delete-forever-body = { $count ->
    [0] इसमें कोई मेल नहीं है। फ़ोल्डर सर्वर से हटा दिया जाता है, इसलिए वेबमेल और आपके फ़ोन से भी यह हट जाता है।
   *[other] { $kind ->
        [conversation] { $count ->
            [one] इसकी { $count } बातचीत हमेशा के लिए मिटा दी जाती है; इस खाते में ट्रैश नहीं है।
           *[other] इसकी { $count } बातचीत हमेशा के लिए मिटा दी जाती हैं; इस खाते में ट्रैश नहीं है।
        }
       *[message] { $count ->
            [one] इसका { $count } मैसेज हमेशा के लिए मिटा दिया जाता है; इस खाते में ट्रैश नहीं है।
           *[other] इसके { $count } मैसेज हमेशा के लिए मिटा दिए जाते हैं; इस खाते में ट्रैश नहीं है।
        }
    } फ़ोल्डर सर्वर से हटा दिया जाता है, इसलिए वेबमेल और आपके फ़ोन से भी यह हट जाता है।
}
folder-delete-label-body = लेबल हटा दिया जाता है। इसके मेल सभी मेल में और अपने दूसरे लेबल में बने रहते हैं।
folder-delete-confirm = फ़ोल्डर मिटाएं
folder-delete-label-confirm = लेबल मिटाएं
folder-deleted = फ़ोल्डर “{ $name }” मिटा दिया गया
label-deleted = लेबल “{ $name }” मिटा दिया गया
