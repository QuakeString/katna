# Katna Mail, Marathi (मराठी).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = मेल खाते जोडा
add-account-looking = { $address } चे मेल सर्व्हर शोधत आहे…
add-account-address-intro = तुमचा ईमेल पत्ता टाका. Katna तुमच्यासाठी सर्व्हर शोधते.
add-account-servers-title = सर्व्हर सेटिंग्ज
add-account-servers-intro = { $address } साठी Katna कुठून मेल वाचते आणि पाठवते.
add-account-password-title = तुमचा पासवर्ड टाका
add-account-signing-in = साइन इन करत आहे…

## Add a mail account: fields

add-account-field-address = ईमेल पत्ता
add-account-incoming = येणारा मेल ({ $protocol })
add-account-outgoing = जाणारा मेल ({ $protocol })
add-account-field-server = सर्व्हर
add-account-field-port = पोर्ट
add-account-security-none = काहीही नाही
add-account-field-username = वापरकर्ता नाव
add-account-field-password = पासवर्ड
add-account-show-password = पासवर्ड दाखवा
add-account-app-password-hint = { $provider } ला इथे ॲप पासवर्ड लागतो, तुम्ही वेबवर वापरता तो नाही. तुमच्या { $provider } खात्याच्या सुरक्षा सेटिंग्जमध्ये एक बनवा.
add-account-field-name = तुमचे नाव (ऐच्छिक)
add-account-name-hint = तुम्ही ज्यांना लिहिता त्यांना दिसते.
add-account-servers-pair = { $imap } आणि { $smtp }
add-account-servers-found = { $source ->
    [built-in] सर्व्हर: { $servers }, Katna च्या प्रदात्यांच्या यादीत सापडले.
    [provider] सर्व्हर: { $servers }, तुमच्या प्रदात्याच्या सेटिंग्जमध्ये सापडले.
    [ispdb] सर्व्हर: { $servers }, Thunderbird च्या प्रदात्यांच्या यादीत सापडले.
    [dns] सर्व्हर: { $servers }, तुमच्या डोमेनच्या DNS रेकॉर्डमध्ये सापडले.
   *[other] सर्व्हर: { $servers }, अंदाजाने; साइन इन अयशस्वी झाल्यास ते तपासा.
}
add-account-servers-entered = सर्व्हर: { $servers }, टाकल्याप्रमाणे.

## Add a mail account: buttons

add-account-servers-button = सर्व्हर सेटिंग्ज
add-account-back = मागे
add-account-add = खाते जोडा
add-account-next = पुढे
add-account-cancel = रद्द करा

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] येणाऱ्या मेलचा सर्व्हर टाका.
   *[outgoing] जाणाऱ्या मेलचा सर्व्हर टाका.
}
add-account-server-space = { $kind ->
    [incoming] येणाऱ्या मेलच्या सर्व्हरच्या नावात स्पेस आहे.
   *[outgoing] जाणाऱ्या मेलच्या सर्व्हरच्या नावात स्पेस आहे.
}
add-account-port-invalid = { $kind ->
    [incoming] येणाऱ्या मेलचा पोर्ट { $min } ते { $max } मधील संख्या असावा.
   *[outgoing] जाणाऱ्या मेलचा पोर्ट { $min } ते { $max } मधील संख्या असावा.
}
add-account-address-empty = ईमेल पत्ता टाका.
add-account-address-invalid = { $example } सारखा ईमेल पत्ता टाका.
add-account-not-found = Katna ला { $address } साठी सर्व्हर सापडले नाहीत, म्हणून नेहमीची नावे भरली आहेत. ती तुमच्या प्रदात्याकडे तपासा.
add-account-password-empty = पासवर्ड टाका.
add-account-name-is-password = नाव आणि पासवर्ड सारखेच आहेत. तिथे त्याऐवजी तुमचे नाव लिहा, लोकांना जसे दिसायला हवे तसे.
add-account-added = { $address } जोडले. तुमचा मेल आणत आहे…
add-account-app-password-refused = { $provider } ने पासवर्ड नाकारला. त्याला ॲप पासवर्ड लागतो, तुम्ही वेबवर वापरता तो नाही.
add-account-password-refused = सर्व्हरने पासवर्ड नाकारला. तो तपासा आणि पुन्हा प्रयत्न करा.

## The account menu (from the account button on the top bar)

add-account-menu-another = आणखी एक खाते जोडा
add-account-menu-manage = खाती व्यवस्थापित करा
