# Katna Mail, Dzongkha (རྫོང་ཁ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Compose window: title bar

compose-new-message = འཕྲིན་དོན་གསརཔ
compose-restore = སོར་ཆུད།
compose-minimize = ཆུང་ཀུ་བཟོ།
compose-exit-full-screen = གསལ་གཞི་གངམ་ལས་ཕྱིར་ཐོན།
compose-open-window = སྒོ་སྒྲིག་གསརཔ་ནང་ཁ་ཕྱེ།
compose-save-close = སྲུང་སྟེ་ཁ་བསྡམས།
compose-back-to-mail = གློག་འཕྲིན་སྒོ་སྒྲིག་ལུ་ལོག
compose-pop-out-reply = ལན་འདི་ སྒོ་སྒྲིག་སོ་སོ་ནང་ཁ་ཕྱེ།
compose-edit-recipients = ལེན་མི་ཚུ་ཞུན་དག་འབད།
compose-summary-cc = Cc: { $names }
compose-summary-bcc = Bcc: { $names }
compose-show-trimmed = བཅད་ཡོད་པའི་ནང་དོན་སྟོན།
compose-hide-trimmed = བཅད་ཡོད་པའི་ནང་དོན་སྦ།
compose-remove-trimmed = ལུང་འདྲེན་འབད་མི་ཚིག་ཡིག་བཏོན་གཏང་།
compose-trimmed-removed = ལུང་འདྲེན་འབད་མི་ཚིག་ཡིག་ བཏོན་གཏང་ཡི།

## Recipients and subject

compose-to = ལུ
compose-cc = Cc
compose-bcc = Bcc
compose-from = ལས
compose-from-choose = རྩིས་ཐོ་གཞན་ཅིག་ལས་གཏང་།
compose-recipients = ལེན་མི་ཚུ
compose-subject = དོན་ཚན

## Sending (the notes at the bottom of the window)

compose-open-elsewhere = ཁ་ཕྱེ་ཡོད་པའི་འཕྲིན་དོན་དེ་ ཧེ་མ་ལས་གཏང་ ཡང་ན་བཏོན་གཏང་།
compose-bad-address = “{ $address }” འདི་ གློག་འཕྲིན་ཁ་བྱང་མེན།
compose-no-recipients = ཉུང་མཐའ་ལེན་མི་གཅིག་ཁ་སྐོང་རྐྱབ།
compose-attachments-too-large = མཉམ་སྦྲགས་ཚུ་ { $size } ཨིན། གློག་འཕྲིན་སར་བར་ཚུ་གིས་ { $limit } ཚུན་ལེནམ་ཨིན།
compose-no-account = གློག་འཕྲིན་གཏང་ནི་གི་རྩིས་ཐོ་ཅིག་ཁ་སྐོང་རྐྱབ།
compose-past-time = མ་འོངས་པའི་དུས་ཚོད་ཅིག་གདམ།
compose-scheduling = དུས་ཚོད་བཀོད་དོ…
compose-sending = གཏང་དོ…
compose-scheduled = { $when } ལུ་གཏང་ནི་གི་དུས་ཚོད་བཀོད་ཡི
compose-sent-archived = བཏང་སྟེ་ཡིག་མཛོད་ནང་བཙུགས་ཡི
compose-sent = འཕྲིན་དོན་བཏང་ཡི
compose-discarded = ཟིན་བྲིས་བཏོན་གཏང་ཡི
compose-draft-saved = ཟིན་བྲིས་སྲུང་བཞག་འབད་ཡི
compose-draft-failed = ཟིན་བྲིས་སྲུང་བཞག་འབད་མ་ཚུགས: { $error }
compose-draft-not-opened = ཟིན་བྲིས་ཁ་ཕྱེ་མ་ཚུགས།

## Attachments

compose-picker-insert = བཙུགས།
compose-picker-attach = མཉམ་སྦྲགས།
compose-file-too-large = { $name } འདི་ སྦོམ་དྲགས་པས། འཕྲིན་དོན་ཅིག་ནང་ { $limit } ཚུན་འབག་ཚུགས།
compose-attachment-size = ({ $size })
compose-remove-attachment = མཉམ་སྦྲགས་རྩ་བསྐྲད་གཏང་།
compose-attachments-total = { $count ->
   *[other] ཡིག་སྣོད་ { $count }། { $size }
}
compose-drop-files = ཡིག་སྣོད་ཚུ་ ནཱ་ལུ་བཀོག
compose-drop-here = ནཱ་ལུ་བཀོག
compose-paste-keep-formatting = རྩ་སྒྲིག་བཞག
compose-paste-table = ཐིག་ཁྲམ
compose-paste-picture = པར
compose-paste-plain-text = ཚིག་ཡིག་རྐྱང་པ
compose-paste-inline = ཚིག་ཡིག་ནང་
compose-paste-attachment = མཉམ་སྦྲགས

## Encryption and signing (the toggles by the recipients)

compose-encrypt = གསང་བཟོ་འབད།
compose-encrypted = གསང་བཟོ་འབད་ཡོདཔ: ལེན་མི་ཚུ་གིས་རྐྱངམ་ཅིག་ ལྷག་ཚུགས
compose-sign = མིང་རྟགས་བཀོད།
compose-signed = མིང་རྟགས་བཀོད་ཡོདཔ: ལེན་མི་ཚུ་གིས་ ཁྱོད་ལས་ཨིནམ་ཞིབ་དཔྱད་འབད་ཚུགས
compose-track = ཁ་ཕྱེ་མི་དང་ ཨེབ་གཏང་ཚུ་ རྗེས་འཚོལ་འབད།
compose-tracked = རྗེས་འཚོལ་འབད་དོ: ལེན་མི་རེ་རེ་གིས་ ནམ་ཁ་ཕྱེཝ་ཨིན་ན་ ཡང་ན་ འབྲེལ་མཐུད་ནམ་ཁ་ཕྱེཝ་ཨིན་ན་ ཁྱོད་ཀྱིས་མཐོང་འོང་།
compose-track-unavailable = མིང་རྟགས་བཀོད་མི་ གསང་བཟོ་འབད་མི་ དེ་ལས་ ཚིག་ཡིག་རྐྱང་པའི་གློག་འཕྲིན་ཚུ་ རྗེས་འཚོལ་འབད་མི་ཚུགས།
compose-track-sign-in = ཁ་ཕྱེ་མི་དང་ ཨེབ་གཏང་ཚུ་ རྗེས་འཚོལ་འབད་ནིའི་དོན་ལུ་ Katna རྩིས་ཐོ་ནང་ ནང་བསྐྱོད་འབད།
compose-receipt = ལྷག་ཡོདཔ་ཨིན་པའི་ཁ་བྱང་ ཞུ།
compose-receipt-on = ལྷག་ཡོདཔ་ཨིན་པའི་ཁ་བྱང་ཞུ་ཡོདཔ: ལེན་མི་གི་གློག་རིམ་གྱིས་ ཁོང་ལུ་ གཏང་ནི་ཨིན་ན་ འདྲི་འོང་།
compose-delivery = ལྷོད་ཡོདཔ་ཨིན་པའི་ཁ་བྱང་ ཞུ།
compose-delivery-on = ལྷོད་ཡོདཔ་ཨིན་པའི་ཁ་བྱང་ཞུ་ཡོདཔ: ལེན་མི་རེ་རེ་གི་སར་བར་གྱིས་ ལེན་པའི་སྐབས་ ཁྱོད་ཀྱི་གློག་འཕྲིན་སར་བར་གྱིས་ ཁྱོད་ལུ་ གློག་འཕྲིན་ཅིག་གཏང་འོང་།
compose-delivery-unavailable = ཁྱོད་ཀྱི་གློག་འཕྲིན་སར་བར་གྱིས་ ལྷོད་ཡོདཔ་ཨིན་པའི་ཁ་བྱང་ མི་གཏང་།

## Spelling

spell-no-dictionary = { $language } གི་དོན་ལུ་ ཡིག་སྦྱོར་ཚིག་མཛོད་གཞི་བཙུགས་འབད་མེད (དཔེར་ན་ hunspell-en_us)།
spell-dictionary-error = ཡིག་སྦྱོར་ཚིག་མཛོད: { $error }

## Grammar checking (the right-click menu on a grammar mistake)

grammar-replace = “{ $words }”
grammar-add = “{ $words }” ཁ་སྐོང་།
grammar-remove = “{ $words }” རྩ་བསྐྲད་གཏང་།
grammar-ignore = སྣང་མེད་བཞག

## Send checks (asked before a message goes out)

send-check-attachment-title = ཡིག་སྣོད་མཉམ་སྦྲགས་འབད་ནི་ཨིན་ན?
send-check-attachment-text = ཁྱོད་ཀྱིས་ མཉམ་སྦྲགས་ཀྱི་སྐོར་ལས་བྲིས་ཡོད་རུང་ ག་ནི་ཡང་མཉམ་སྦྲགས་འབད་མེད།
send-check-attach = ཡིག་སྣོད་ཅིག་མཉམ་སྦྲགས་འབད།
send-check-subject-title = དོན་ཚན་མེད་པར་གཏང་ནི་ཨིན་ན?
send-check-subject-text = འཕྲིན་དོན་འདི་ལུ་ དོན་ཚན་མིན་འདུག
send-check-add-subject = དོན་ཚན་ཁ་སྐོང་རྐྱབ།
send-check-send-anyway = ག་དེ་འབད་རུང་གཏང་།
recipient-not-valid = ནུས་ཅན་གྱི་གློག་འཕྲིན་ཁ་བྱང་མེན།
recipient-show-address = ཁ་བྱང་སྟོན།
recipient-remove = བཏོན།
recipient-bad-title = ཁ་བྱང་ཞིབ་དཔྱད་འབད།
recipient-bad-text = “{ $address }” འདི་ ནུས་ཅན་གྱི་གློག་འཕྲིན་ཁ་བྱང་མེན། མ་གཏང་བའི་ཧེ་མ་ ནོར་བཅོས་འབད་ ཡང་ན་ བཏོན་གཏང་།
recipient-bad-fix = ནོར་བཅོས་འབད།
