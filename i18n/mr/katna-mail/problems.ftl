# Katna Mail, Marathi (मराठी).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = मेल सर्व्हर

problems-signed-out = { $provider } ने Katna ला { $address } मधून साइन आउट केले. मेल सिंक होणे थांबले.
problems-password-refused = { $provider } ने { $address } चा पासवर्ड नाकारला. तो बदलला असू शकतो.
problems-no-answer = { $provider } { $address } साठी प्रतिसाद देत नाही. Katna प्रयत्न करत राहते.
problems-offline = तुम्ही ऑफलाइन आहात. तुमचा मेल येथेच आहे, आणि तुम्ही पाठवलेला मेल तुम्ही परत ऑनलाइन येईपर्यंत थांबतो.
problems-accounts-need-you = { $count ->
    [one] 1 खात्याला तुमची गरज आहे
   *[other] { $count } खात्यांना तुमची गरज आहे
}
problems-show = दाखवा
problems-later = नंतर
problems-new-password = नवीन पासवर्ड
problems-try-again = पुन्हा प्रयत्न करा

## The New password card

problems-password-title = नवीन पासवर्ड
problems-password-detail = { $provider } ने { $address } चा सेव्ह केलेला पासवर्ड नाकारला. नवीन पासवर्ड टाइप करा; Katna तो ठेवण्यापूर्वी तपासते.
problems-password-placeholder = पासवर्ड
problems-password-show = पासवर्ड दाखवा
problems-password-hide = पासवर्ड लपवा
problems-password-cancel = रद्द करा
problems-password-save = सेव्ह करा
problems-password-checking = तपासत आहे…
problems-password-refused-again = { $provider } ने हा पासवर्डही नाकारला. तो तपासा आणि पुन्हा प्रयत्न करा.
problems-password-saved = { $address } साठी पासवर्ड सेव्ह केला. तुमचा मेल आणत आहे…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = { $count ->
    [one] { $address } च्या मेल सर्व्हरने एक मेसेज हलवणे स्वीकारले नाही, म्हणून तो होता तिथे परत आला आहे.
   *[other] { $address } च्या मेल सर्व्हरने { $count } मेसेज हलवणे स्वीकारले नाही, म्हणून ते होते तिथे परत आले आहेत.
}
problems-refused-flags = { $count ->
    [one] { $address } च्या मेल सर्व्हरने एका मेसेजवर खूण करणे (वाचलेले, तारांकित…) स्वीकारले नाही, म्हणून तो आधीसारखा झाला आहे.
   *[other] { $address } च्या मेल सर्व्हरने { $count } मेसेजवर खूण करणे (वाचलेले, तारांकित…) स्वीकारले नाही, म्हणून ते आधीसारखे झाले आहेत.
}
problems-refused-label = { $count ->
    [one] { $address } च्या मेल सर्व्हरने एका मेसेजची लेबल बदलणे स्वीकारले नाही, म्हणून तो आधीसारखा झाला आहे.
   *[other] { $address } च्या मेल सर्व्हरने { $count } मेसेजची लेबल बदलणे स्वीकारले नाही, म्हणून ते आधीसारखे झाले आहेत.
}
problems-refused-delete = { $count ->
    [one] { $address } च्या मेल सर्व्हरने एक मेसेज हटवणे स्वीकारले नाही, म्हणून तो परत आला आहे.
   *[other] { $address } च्या मेल सर्व्हरने { $count } मेसेज हटवणे स्वीकारले नाही, म्हणून ते परत आले आहेत.
}
problems-refused-other = { $count ->
    [one] { $address } च्या मेल सर्व्हरने एक बदल स्वीकारला नाही, म्हणून Katna ने तो आधीसारखा केला.
   *[other] { $address } च्या मेल सर्व्हरने { $count } बदल स्वीकारले नाहीत, म्हणून Katna ने ते आधीसारखे केले.
}
problems-details = तपशील

## Katna's background service (katna-daemon) isn't running

service-starting = Katna ची बॅकग्राउंड सेवा सुरू करत आहे…
service-failed = Katna ची बॅकग्राउंड सेवा सुरू होत नाही, त्यामुळे मेल सिंक होत नाही.
service-start-again = पुन्हा सुरू करा
service-started-again = Katna ची बॅकग्राउंड सेवा थांबली होती आणि ती पुन्हा सुरू केली.
service-details-title = सेवा सुरू का होत नाही
service-details-body = हे कॉपी करा आणि तुमच्या अहवालासोबत पाठवा. यात कोणताही मेल किंवा पासवर्ड नाही.
service-details-copy = कॉपी करा
service-details-close = बंद करा
