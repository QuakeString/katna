# Katna Mail, Marathi (मराठी).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Accounts

accounts-folder-pane = फोल्डर पेन
accounts-folder-pane-detail = डावीकडील पेन कोणत्या खात्यांचे फोल्डर दाखवते.
accounts-shown-one = एका वेळी एक खाते; खाते कार्डमध्ये बदला
accounts-shown-all = सर्व खाती, एकामागोमाग एक
accounts-row = खाती
accounts-row-detail = फोल्डर पेन आणि खाते मेनू खाती याच क्रमाने दाखवतात; पहिले खाते डीफॉल्ट असते. खाते काढल्यास या कॉम्प्युटरवरील त्याच्या मेलची Katna ची प्रत हटवली जाते. मेल सर्व्हरवर राहतो.
accounts-none = अजून कोणतेही खाते नाही.
accounts-kind-imported = आयात केलेले
accounts-picture-reset = डेस्कटॉपचे चित्र वापरा
accounts-picture-change = चित्र बदला
accounts-picture-remove = चित्र काढा
accounts-rename = नाव बदला
accounts-name-save = सेव्ह करा
accounts-name-cancel = रद्द करा
accounts-name-placeholder = तुमचे नाव
accounts-rename-failed = खात्याचे नाव बदलता आले नाही: { $error }
accounts-move-up = वर हलवा
accounts-move-down = खाली हलवा
accounts-drag = क्रम बदलण्यासाठी ओढा
accounts-remove = काढा
accounts-delete-all-row = सर्व डेटा हटवा
accounts-delete-all-row-detail = नव्या इंस्टॉलप्रमाणे पुन्हा सुरुवात करा.
accounts-delete-all-about = या कॉम्प्युटरवरून प्रत्येक खाते, सर्व साठवलेला मेल, संपर्क आणि कॅलेंडर, शोध इंडेक्स, तुमच्या सेटिंग्ज आणि सेव्ह केलेले पासवर्ड हटवते. तुमच्या मेल सर्व्हरवर काहीही बदलत नाही.
accounts-delete-all-open = Katna चा सर्व डेटा हटवा

## Settings > Accounts: snackbars after deleting

accounts-removed-local = { $address } Katna मधून काढले.
accounts-removed = { $address } Katna मधून काढले. त्याचा मेल अजूनही सर्व्हरवर आहे.
accounts-all-deleted = Katna चा सर्व डेटा या कॉम्प्युटरवरून हटवला.

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = { $address } काढायचे?
accounts-remove-confirm = खाते काढा
accounts-removing = काढत आहे…
accounts-remove-local-mail = { $folders ->
    [0] या खात्यात आयात केलेला सर्व मेल
    [one] या खात्यात आयात केलेला सर्व मेल, त्याच्या फोल्डरमधील
   *[other] या खात्यात आयात केलेला सर्व मेल, त्याच्या { $folders } फोल्डरमधील
}
accounts-remove-local-settings = त्याच्या Katna सेटिंग्ज
accounts-remove-mail = { $folders ->
    [0] Katna ने साठवलेला या खात्याचा सर्व मेल
    [one] Katna ने साठवलेला या खात्याचा सर्व मेल, त्याच्या फोल्डरमधील
   *[other] Katna ने साठवलेला या खात्याचा सर्व मेल, त्याच्या { $folders } फोल्डरमधील
}
accounts-remove-outbox = आउटबॉक्समध्ये थांबलेले त्याचे मेसेज
accounts-remove-settings = त्याचा सेव्ह केलेला पासवर्ड आणि त्याच्या Katna सेटिंग्ज
accounts-delete-all-title = Katna चा सर्व डेटा हटवायचा?
accounts-delete-all-confirm = सर्व काही हटवा
accounts-deleting = हटवत आहे…
accounts-delete-all-accounts = प्रत्येक खाते, आणि Katna ने साठवलेले सर्व मेल आणि अटॅचमेंट
accounts-delete-all-contacts = संपर्क, कॅलेंडर आणि शोध इंडेक्स
accounts-delete-all-settings = सर्व सेटिंग्ज, स्वाक्षऱ्या आणि कीबोर्ड शॉर्टकट
accounts-delete-all-passwords = प्रत्येक सेव्ह केलेला पासवर्ड
accounts-deleted-heading = या कॉम्प्युटरवरून हटवले जाईल:
accounts-cannot-undo = हे पूर्ववत करता येणार नाही.
accounts-server-delete-all = तुमच्या मेल सर्व्हरवर काहीही बदलत नाही: तुमचा मेल तिथेच राहतो, आणि खाते पुन्हा जोडल्यास तो पुन्हा डाउनलोड होतो. फाइलमधून आयात केलेला मेल फक्त Katna मध्ये आहे; त्या फाइलना हात लावला जात नाही.
accounts-server-local = हा मेल फाइलमधून आयात केला होता, त्यामुळे त्याची एकमेव प्रत Katna कडे आहे. ज्या फाइलमधून तो आला त्यांना हात लावला जात नाही; तो परत मिळवण्यासाठी त्या पुन्हा आयात करा.
accounts-server-remove = मेल सर्व्हरवर काहीही बदलत नाही: तुमचा मेल तिथेच राहतो, आणि खाते पुन्हा जोडल्यास तो पुन्हा डाउनलोड होतो.
accounts-confirm-word = हटवा
accounts-confirm-placeholder = “{ accounts-confirm-word }” टाइप करा
accounts-confirm-prompt = खात्री करण्यासाठी “{ accounts-confirm-word }” टाइप करा:
accounts-cancel = रद्द करा
reset-cache-about = Katna ने डाउनलोड केलेले मेल आणि अटॅचमेंट, प्रेषकाची चित्रे आणि शोध इंडेक्स हटवते, मग अलीकडचा मेल पुन्हा डाउनलोड करते. खाती, सेटिंग्ज आणि फक्त या कॉम्प्युटरवर असलेला मेल तसेच राहतात.
reset-cache-button = कॅशे रीसेट करा
reset-cache-title = कॅशे रीसेट करायचे?
reset-cache-deleted = हटवले जाईल, मग पुन्हा डाउनलोड होईल:
reset-cache-mail = तुमच्या IMAP सर्व्हरवरून डाउनलोड केलेले मेल आणि अटॅचमेंट: अलीकडचा मेल आत्ताच पुन्हा डाउनलोड होतो, जुना मेल तुम्ही उघडता तेव्हा
reset-cache-index = शोध इंडेक्स, जो लगेच पुन्हा तयार होतो
reset-cache-pictures = प्रेषकाची चित्रे
reset-cache-kept = राहील: तुमची खाती, पासवर्ड आणि सेटिंग्ज; तारे, लेबल, वाचलेल्याच्या खुणा आणि पिन; मसुदे, आउटबॉक्स आणि अजून सर्व्हरवर न पोहोचलेले बदल; आणि POP3 खात्यांमधील किंवा आयात केलेल्या फायलींमधील मेल, ज्याची दुसरी प्रत नसू शकते. तुमच्या मेल सर्व्हरवर काहीही बदलत नाही.
reset-cache-confirm = कॅशे रीसेट करा
reset-cache-busy = रीसेट करत आहे…
reset-cache-done = कॅशे रीसेट झाले. अलीकडचा मेल पुन्हा डाउनलोड होत आहे.
reset-cache-done-freed = कॅशे रीसेट झाले आणि { $size } जागा मोकळी झाली. अलीकडचा मेल पुन्हा डाउनलोड होत आहे.
