# Katna Mail, Marathi (मराठी).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Navigation (the folders pane)

nav-labels = लेबल
nav-folders = फोल्डर
nav-label-new = नवीन लेबल तयार करा
nav-folder-new = नवीन फोल्डर तयार करा
nav-menu-check-mail = नवीन मेल तपासा
nav-menu-check-inbox = हा इनबॉक्स तपासा
nav-unified-leave-out = एकत्रित इनबॉक्समधून वगळा
nav-unified-bring-back = एकत्रित इनबॉक्समध्ये परत आणा
nav-menu-sign-in-again = पुन्हा साइन इन करा
nav-menu-new-mail = या खात्यावरून नवीन मेल
nav-menu-account-settings = खाते सेटिंग्ज
nav-account-checked = सिंकमध्ये · { $ago } तपासले
nav-account-in-sync = सिंकमध्ये
nav-account-connecting = कनेक्ट करत आहे…
nav-account-offline = ऑफलाइन, पुन्हा प्रयत्न करत आहे
nav-account-signed-out = { $provider } साइन-इनची मुदत संपली
nav-account-password-refused = पासवर्ड नाकारला
nav-account-storage = { $total } पैकी { $used } वापरले
nav-menu-new-subfolder = आत नवीन फोल्डर
nav-menu-new-sublabel = आत नवीन लेबल
nav-menu-rename = नाव बदला
nav-menu-delete = हटवा
nav-menu-empty-trash = कचरापेटी रिकामी करा
nav-account-unnamed = खाते { $number }
nav-all-accounts = सर्व खाती
nav-expand = फोल्डर दाखवा
nav-collapse = फोल्डर लपवा
storage-used = { $total } पैकी { $percent }% वापरले
storage-used-detail = { $address }: { $total } पैकी { $used } वापरले

## Special folders (the user's own folders keep their names)

folder-inbox = इनबॉक्स
folder-starred = तारांकित
folder-snoozed = स्नूझ केलेले
folder-unread = न वाचलेले
folder-important = महत्त्वाचे
folder-drafts = मसुदे
folder-sent = पाठवलेले
folder-archive = संग्रहण
folder-spam = स्पॅम
folder-trash = कचरापेटी
folder-all-mail = सर्व मेल
folder-scheduled = शेड्यूल केलेले
folder-waiting = उत्तराची वाट पाहत आहे
folder-waiting-short = प्रतीक्षेत
folder-reminders = रिमाइंडर
folder-outbox = आउटबॉक्स
folder-activity = ॲक्टिव्हिटी
folder-not-on-account = या खात्यात असे फोल्डर नाही.

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = नवीन लेबल
label-folder-new-title = नवीन फोल्डर
label-prompt = कृपया नवीन लेबलचे नाव एंटर करा:
label-folder-prompt = कृपया नवीन फोल्डरचे नाव एंटर करा:
label-name-hint = लेबलचे नाव
label-folder-name-hint = फोल्डरचे नाव
label-nest = लेबल याच्या खाली ठेवा:
label-folder-nest = फोल्डर याच्या खाली ठेवा:
label-cancel = रद्द करा
label-create = तयार करा
label-creating = तयार करत आहे…
label-created = “{ $name }” लेबल तयार केले.
label-folder-created = “{ $name }” फोल्डर तयार केले.
label-rename-title = लेबलचे नाव बदला
label-folder-rename-title = फोल्डरचे नाव बदला
label-rename = नाव बदला
label-renaming = नाव बदलत आहे…
label-renamed = लेबलचे नाव “{ $name }” असे बदलले.
label-folder-renamed = फोल्डरचे नाव “{ $name }” असे बदलले.

## Deleting a folder or label (asked first)

folder-delete-title = “{ $name }” हटवायचे?
folder-delete-body = { $count ->
    [0] त्यात कोणताही मेल नाही. फोल्डर सर्व्हरवरून काढला जातो, त्यामुळे वेबमेल आणि तुमच्या फोनवरूनही तो जातो.
   *[other] { $kind ->
        [conversation] { $count ->
            [one] त्यातील { $count } संभाषण कचरापेटीत जाते, त्यामुळे तुम्ही ते अजूनही परत मिळवू शकता.
           *[other] त्यातील { $count } संभाषणे कचरापेटीत जातात, त्यामुळे तुम्ही ती अजूनही परत मिळवू शकता.
        }
       *[message] { $count ->
            [one] त्यातील { $count } मेसेज कचरापेटीत जातो, त्यामुळे तुम्ही तो अजूनही परत मिळवू शकता.
           *[other] त्यातील { $count } मेसेज कचरापेटीत जातात, त्यामुळे तुम्ही ते अजूनही परत मिळवू शकता.
        }
    } फोल्डर सर्व्हरवरून काढला जातो, त्यामुळे वेबमेल आणि तुमच्या फोनवरूनही तो जातो.
}
folder-delete-forever-body = { $count ->
    [0] त्यात कोणताही मेल नाही. फोल्डर सर्व्हरवरून काढला जातो, त्यामुळे वेबमेल आणि तुमच्या फोनवरूनही तो जातो.
   *[other] { $kind ->
        [conversation] { $count ->
            [one] त्यातील { $count } संभाषण कायमचे हटवले जाते; या खात्याला कचरापेटी नाही.
           *[other] त्यातील { $count } संभाषणे कायमची हटवली जातात; या खात्याला कचरापेटी नाही.
        }
       *[message] { $count ->
            [one] त्यातील { $count } मेसेज कायमचा हटवला जातो; या खात्याला कचरापेटी नाही.
           *[other] त्यातील { $count } मेसेज कायमचे हटवले जातात; या खात्याला कचरापेटी नाही.
        }
    } फोल्डर सर्व्हरवरून काढला जातो, त्यामुळे वेबमेल आणि तुमच्या फोनवरूनही तो जातो.
}
folder-delete-label-body = लेबल काढले जाते. त्याचा मेल सर्व मेलमध्ये आणि त्याच्या इतर लेबलमध्ये राहतो.
folder-delete-confirm = फोल्डर हटवा
folder-delete-label-confirm = लेबल हटवा
folder-deleted = फोल्डर “{ $name }” हटवला
label-deleted = लेबल “{ $name }” हटवले
