# Katna Mail, Marathi (मराठी).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Folders & rules

settings-rules = नियम
settings-rules-summary = नवीन मेल आपोआप क्रमवारी लावा, लेबल लावा, फॉरवर्ड करा किंवा शांत करा
settings-rules-intro = नियम नवीन मेल आपोआप, या क्रमाने, लावतात. क्रम बदलण्यासाठी ओढा.
settings-rules-all-accounts = सर्व खाती
settings-rules-new = नवीन नियम
settings-rules-none = अजून कोणतेही नियम नाहीत. नियम नवीन मेल आपोआप लावतो: प्रेषक, विषय किंवा शब्दांनुसार.
settings-rules-none-account = या खात्यासाठी अजून कोणतेही नियम नाहीत.
settings-rules-drag = क्रम बदलण्यासाठी ओढा
settings-rules-edit = नियम संपादित करा
settings-rules-turn-off = हा नियम बंद करा
settings-rules-turn-on = हा नियम चालू करा

## Starter rules: offered under the user's own rules, switched off.
## Turning one on makes it one of the user's rules.

settings-rules-starters = सुरुवातीचे नियम
settings-rules-starters-intro = तुम्ही चालू करेपर्यंत बंद. ते तुमच्या सर्व खात्यांसाठी चालतात; बदलण्यासाठी एखादा संपादित करा.
settings-rules-starter-turning-on = “{ $name }” चालू करत आहे…
settings-rules-starter-failed = “{ $name }” चालू करता आला नाही: { $error }
rules-starter-promotions = जाहिराती शांत करा
rules-starter-newsletters = वृत्तपत्रे वाचनमध्ये
rules-starter-receipts = पावत्या आणि बिले
rules-starter-deliveries = डिलिव्हरी
rules-starter-train = रेल्वे तिकिटे
rules-starter-flight = विमान तिकिटे
rules-starter-codes = एक-वेळ कोड
rules-starter-security = सुरक्षा सूचना
rules-starter-social = सामाजिक मेल
rules-starter-invites = कॅलेंडर आमंत्रणे
rules-starter-folder-reading = वाचन
rules-starter-folder-receipts = पावत्या
rules-starter-folder-deliveries = डिलिव्हरी
rules-starter-folder-travel = प्रवास
rules-starter-folder-social = सामाजिक
rules-runs-katna = Katna मध्ये चालतो
rules-runs-gmail = Gmail वर चालतो
rules-runs-sieve = सर्व्हरवर चालतो
rules-stopped = थांबला
rules-error-folder-gone = हा नियम वापरत असलेले फोल्डर आता अस्तित्वात नाही. दुसरे निवडण्यासाठी नियम संपादित करा.
rules-error-no-archive = या खात्याला संग्रहण फोल्डर नाही. दुसरे काहीतरी करण्यासाठी नियम संपादित करा.
rules-error-no-trash = या खात्याला कचरापेटी फोल्डर नाही. दुसरे काहीतरी करण्यासाठी नियम संपादित करा.
rules-error-cannot-send = हे खाते मेल पाठवू शकत नाही, त्यामुळे नियम तो फॉरवर्ड करू शकत नाही.
rules-error-other = { $error }. नियम संपादित करा आणि पुन्हा चालू करा.

settings-folders = फोल्डर
settings-folders-summary = फोल्डर पॅनलमधील न वाचलेल्यांची संख्या
settings-folders-unread-counts = प्रत्येक फोल्डरवर न वाचलेल्यांची संख्या
settings-folders-unread-counts-detail = बंद: फक्त इनबॉक्स किती न वाचलेले आहेत ते दाखवतो

## A rule in one line, on its row: "From contains substack.com → skip the
## inbox, label Reading".

rules-summary = { $when } → { $then }
rules-summary-and = { $first } आणि { $next }
rules-summary-or = { $first } किंवा { $next }
rules-summary-more = आणखी { $count }
rules-summary-list = { $first }, { $next }
rules-summary-condition = { $field } { $comparator }: { $value }
rules-summary-has-attachment = अटॅचमेंट आहे
rules-summary-no-attachment = अटॅचमेंट नाही
rules-summary-mailing-list = मेलिंग लिस्टकडून
rules-summary-not-mailing-list = मेलिंग लिस्टकडून नाही
rules-summary-tab = { $tab } टॅबमध्ये
rules-summary-not-tab = { $tab } टॅबमध्ये नाही
rules-summary-move = { $folder } मध्ये हलवा
rules-summary-archive = इनबॉक्स वगळा
rules-summary-trash = कचरापेटीत हलवा
rules-summary-mark-read = वाचलेले म्हणून खूण करा
rules-summary-star = तारांकित करा
rules-summary-important = महत्त्वाचे म्हणून खूण करा
rules-summary-label = { $label } लेबल लावा
rules-summary-forward = { $address } ला फॉरवर्ड करा
rules-summary-dont-notify = सूचना देऊ नका
rules-summary-read-after = { $count ->
    [one] { $count } दिवसानंतर वाचलेले म्हणून खूण करा
   *[other] { $count } दिवसांनंतर वाचलेले म्हणून खूण करा
}
rules-summary-folder-gone = नाहीसे झालेले फोल्डर

## The rule editor

rules-editor-new-title = नवीन नियम
rules-editor-edit-title = नियम संपादित करा
rules-editor-name-hint = नियमाचे नाव
rules-editor-when = जेव्हा नवीन मेल
rules-editor-of-these = यांच्याशी जुळतो:
rules-mode-all = सर्व
rules-mode-any = कोणत्याहीपैकी एक
rules-field-from = प्रेषक
rules-field-to = प्रति
rules-field-cc = Cc
rules-field-any-recipient = प्रति किंवा Cc
rules-field-reply-to = उत्तराचा पत्ता
rules-field-subject = विषय
rules-field-body = मजकूर
rules-field-attachment-name = अटॅचमेंटचे नाव
rules-field-has-attachment = अटॅचमेंट आहे
rules-field-mailing-list = मेलिंग लिस्टकडून
rules-field-tab = इनबॉक्स टॅब
rules-comparator-contains = मध्ये आहे
rules-comparator-not-contains = मध्ये नाही
rules-comparator-begins-with = ने सुरू होते
rules-comparator-ends-with = ने संपते
rules-comparator-equals = नेमके असे आहे
rules-comparator-matches = पॅटर्नशी जुळते
rules-has-yes = होय
rules-has-no = नाही
rules-editor-value-hint = शब्द किंवा पत्ता
rules-editor-add-condition = अट जोडा
rules-editor-remove = काढा
rules-editor-then = मग:
rules-action-move = येथे हलवा
rules-action-archive = इनबॉक्स वगळा (संग्रहित करा)
rules-action-trash = कचरापेटीत हलवा
rules-action-mark-read = वाचलेले म्हणून खूण करा
rules-action-star = तारांकित करा
rules-action-important = महत्त्वाचे म्हणून खूण करा
rules-action-label = लेबल जोडा
rules-action-forward = येथे फॉरवर्ड करा
rules-action-dont-notify = सूचना देऊ नका
rules-action-read-after = यानंतर वाचलेले म्हणून खूण करा
rules-editor-choose-folder = फोल्डर निवडा
rules-editor-choose-label = लेबल निवडा
rules-editor-new-folder = नवीन: { $name }
rules-editor-folder-of = { $folder } ({ $account })
rules-editor-forward-hint = ईमेल पत्ता
rules-editor-days = दिवस
rules-editor-add-action = क्रिया जोडा
rules-editor-stop = येथे थांबा: नंतरचे नियम या मेलवर चालत नाहीत
rules-editor-accounts = खाती:
rules-editor-accounts-none = खाती निवडा
rules-editor-accounts-many = { $count ->
    [one] { $count } खाते
   *[other] { $count } खाती
}
rules-editor-matches = मागील { $days } दिवसांतील { $mails } शी जुळतो
rules-editor-mails = { $count ->
    [one] { $count } मेल
   *[other] { $count } मेल
}
rules-editor-counting = जुळणारा मेल मोजत आहे…
rules-editor-show = ते दाखवा
rules-editor-also-apply = या { $count } वरही लागू करा
rules-editor-runs-katna = हा कॉम्प्युटर चालू असताना Katna मध्ये चालतो.
rules-editor-runs-gmail = Gmail वर चालतो, त्यामुळे तुमच्या फोनवर आणि हा कॉम्प्युटर बंद असतानाही काम करतो.
rules-editor-runs-sieve = तुमच्या मेल सर्व्हरवर चालतो, त्यामुळे तुमच्या फोनवर आणि हा कॉम्प्युटर बंद असतानाही काम करतो.
rules-note-gmail-action = Katna मध्ये चालतो: Gmail फिल्टर “{ $action }” करू शकत नाहीत.
rules-note-sieve-action = Katna मध्ये चालतो: तुमच्या मेल सर्व्हरचे नियम “{ $action }” करू शकत नाहीत.
rules-note-test = { $field } { $comparator }
rules-note-gmail-condition = Katna मध्ये चालतो: Gmail फिल्टर “{ $test }” Katna प्रमाणे तपासू शकत नाहीत.
rules-note-sieve-condition = Katna मध्ये चालतो: तुमच्या मेल सर्व्हरचे नियम “{ $test }” Katna प्रमाणे तपासू शकत नाहीत.
rules-note-order = Katna मध्ये चालतो, कारण खात्याचा आधीचा नियमही तसाच चालतो: नियम यादीच्या क्रमाने चालतात.
rules-note-gmail-stop = Katna मध्ये चालतो: Gmail फिल्टर नंतरचे नियम चालण्यापासून थांबवू शकत नाहीत.
rules-note-gmail-forward = Katna मध्ये चालतो: Gmail फक्त त्याच्या सेटिंग्जमध्ये पडताळलेल्या पत्त्यांवर फॉरवर्ड करते, आणि { $address } त्यापैकी नाही.
rules-note-gmail-folder = Katna मध्ये चालतो: हा नियम वापरत असलेल्या फोल्डरसाठी Gmail मध्ये लेबल नाही.
rules-note-sieve-folder = Katna मध्ये चालतो: हा नियम वापरत असलेले फोल्डर तुमच्या मेल सर्व्हरवर नाही.
rules-note-gmail-sign-in = तुम्ही Google मध्ये पुन्हा साइन इन करून Katna ला Gmail फिल्टर बनवू देईपर्यंत Katna मध्ये चालतो.
rules-note-sieve-other-script = Katna मध्ये चालतो: तुमच्या मेल सर्व्हरवर दुसरी नियम स्क्रिप्ट (“{ $name }”) सक्रिय आहे.
rules-note-gmail-failed = Katna मध्ये चालतो: Gmail ने तो स्वीकारला नाही ({ $error }).
rules-note-sieve-failed = Katna मध्ये चालतो: तुमच्या मेल सर्व्हरने तो स्वीकारला नाही ({ $error }).
rules-editor-cancel = रद्द करा
rules-editor-save = सेव्ह करा
rules-editor-saving = सेव्ह करत आहे…
rules-editor-delete = नियम हटवा
rules-editor-delete-ask = हा नियम हटवायचा?
rules-editor-delete-keep = ठेवा
rules-editor-delete-confirm = हटवा
rules-editor-needs-folder = प्रत्येक “येथे हलवा” साठी फोल्डर आणि प्रत्येक “लेबल जोडा” साठी लेबल निवडा.
rules-editor-needs-days = “यानंतर वाचलेले म्हणून खूण करा” ला 1 ते 3650 पर्यंत दिवसांची संख्या लागते.
rules-saved = नियम सेव्ह केला
rules-saved-applied = { $count ->
    [one] नियम सेव्ह केला आणि { $count } मेलवर लागू केला
   *[other] नियम सेव्ह केला आणि { $count } मेलवर लागू केला
}
rules-apply-failed = नियम सेव्ह केला, पण तो लागू करता आला नाही: { $error }
rules-deleted = नियम हटवला
rules-delete-failed = नियम हटवता आला नाही: { $error }
rules-change-failed = नियम बदलता आले नाहीत: { $error }
