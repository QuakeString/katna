# Katna Mail, Dzongkha (རྫོང་ཁ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = གློག་འཕྲིན་སར་བར

problems-signed-out = { $provider } གིས་ Katna འདི་ { $address } ལས་ ཕྱིར་བཏོན་ཡི། གློག་འཕྲིན་མཉམ་སྡེབ་ བཀག་ཡི།
problems-password-refused = { $provider } གིས་ { $address } གི་ཆོག་ཡིག་ ངོས་ལེན་མ་འབད། བསྒྱུར་ཡོདཔ་འོང་།
problems-no-answer = { $provider } གིས་ { $address } གི་དོན་ལུ་ ལན་མི་སློག Katna གིས་ ད་རུང་རྩོལ་བསྐྱེད་དོ།
problems-offline = ཁྱོད་ ཡོངས་འབྲེལ་མེད། ཁྱོད་ཀྱི་གློག་འཕྲིན་ཚུ་ ད་ལྟོ་ཡང་ནཱ་ལུ་ཡོད་ ཁྱོད་ཀྱིས་གཏང་མི་གློག་འཕྲིན་ཚུ་ ཁྱོད་ལོག་འོང་ཚུན་ སྒུགཔ་ཨིན།
problems-accounts-need-you = { $count ->
   *[other] རྩིས་ཐོ་ { $count } ལུ་ ཁྱོད་དགོཔ་ཨིན
}
problems-show = སྟོན།
problems-later = ཤུལ་ལས
problems-new-password = ཆོག་ཡིག་གསརཔ
problems-try-again = ལོག་འབད་རྩོལ་བསྐྱེད།

## The New password card

problems-password-title = ཆོག་ཡིག་གསརཔ
problems-password-detail = { $provider } གིས་ { $address } གི་ སྲུང་ཡོད་པའི་ཆོག་ཡིག་ ངོས་ལེན་མ་འབད། གསརཔ་འདི་ཡིག་དཔར་རྐྱབ། Katna གིས་ མ་བཞག་པའི་ཧེ་མ་ ཞིབ་དཔྱད་འབདཝ་ཨིན།
problems-password-placeholder = ཆོག་ཡིག
problems-password-show = ཆོག་ཡིག་སྟོན།
problems-password-hide = ཆོག་ཡིག་སྦ།
problems-password-cancel = ཆ་མེད་གཏང་།
problems-password-save = སྲུང་།
problems-password-checking = ཞིབ་དཔྱད་འབད་དོ…
problems-password-refused-again = { $provider } གིས་ ཆོག་ཡིག་འདི་ཡང་ ངོས་ལེན་མ་འབད། ཞིབ་དཔྱད་འབད་ཞིནམ་ལས་ ལོག་འབད་རྩོལ་བསྐྱེད།
problems-password-saved = { $address } གི་ཆོག་ཡིག་ སྲུང་ཡི། ཁྱོད་ཀྱི་གློག་འཕྲིན་ལེན་དོ…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = { $count ->
   *[other] { $address } གི་གློག་འཕྲིན་སར་བར་གྱིས་ འཕྲིན་དོན་ { $count } སྤོ་བཤུད་ ངོས་ལེན་མ་འབད་ནི་དེ་འབདཝ་ལས་ ཧེ་མའི་ས་ཁོངས་ལུ་ ལོག་སྡོད་ཡི།
}
problems-refused-flags = { $count ->
   *[other] { $address } གི་གློག་འཕྲིན་སར་བར་གྱིས་ འཕྲིན་དོན་ { $count } ལུ་རྟགས་བཀལ་ནི (ལྷག་ཡོདཔ་ སྐར་མ་བཀལ…) ངོས་ལེན་མ་འབད་ནི་དེ་འབདཝ་ལས་ ཧེ་མ་བཟུམ་སྦེ་ ལོག་སྡོད་ཡི།
}
problems-refused-label = { $count ->
   *[other] { $address } གི་གློག་འཕྲིན་སར་བར་གྱིས་ འཕྲིན་དོན་ { $count } གི་ཁ་ཡིག་བསྒྱུར་ནི་ ངོས་ལེན་མ་འབད་ནི་དེ་འབདཝ་ལས་ ཧེ་མ་བཟུམ་སྦེ་ ལོག་སྡོད་ཡི།
}
problems-refused-delete = { $count ->
   *[other] { $address } གི་གློག་འཕྲིན་སར་བར་གྱིས་ འཕྲིན་དོན་ { $count } བཏོན་གཏང་ནི་ ངོས་ལེན་མ་འབད་ནི་དེ་འབདཝ་ལས་ ལོག་འོང་ཡི།
}
problems-refused-other = { $count ->
   *[other] { $address } གི་གློག་འཕྲིན་སར་བར་གྱིས་ བསྒྱུར་བཅོས་ { $count } ངོས་ལེན་མ་འབད་ནི་དེ་འབདཝ་ལས་ Katna གིས་ ཧེ་མ་བཟུམ་སྦེ་ ལོག་བཞག་ཡི།
}
problems-details = ཁ་གསལ

## Katna's background service (katna-daemon) isn't running

service-starting = Katna གི་རྒྱབ་ཐག་ཞབས་ཏོག་ འགོ་བཙུགས་དོ…
service-failed = Katna གི་རྒྱབ་ཐག་ཞབས་ཏོག་ འགོ་མ་བཙུགས་ཚུགས་པས་ གློག་འཕྲིན་མཉམ་སྡེབ་མི་འབད་བས།
service-start-again = ལོག་འགོ་བཙུགས།
service-started-again = Katna གི་རྒྱབ་ཐག་ཞབས་ཏོག་ བཀག་སྟེ་ ལོག་འགོ་བཙུགས་ཡི།
service-details-title = ཞབས་ཏོག་ ག་ཅི་སྦེ་འགོ་མ་བཙུགས་ཚུགས་ག
service-details-body = འདི་འདྲ་བཤུས་རྐྱབ་སྟེ་ ཁྱོད་ཀྱི་སྙན་ཞུ་དང་གཅིག་ཁར་གཏང་། འདི་ནང་ གློག་འཕྲིན་ ཡང་ན་ ཆོག་ཡིག་ག་ནི་ཡང་མེད།
service-details-copy = འདྲ་བཤུས་རྐྱབ།
service-details-close = ཁ་བསྡམས།
service-not-running = Katna གི་རྒྱབ་ཐག་ཞབས་ཏོག་ གཡོག་བཀོལ་མི་འདུག
service-no-answer = Katna གི་རྒྱབ་ཐག་ཞབས་ཏོག་གིས་ ལན་མ་བྱིན: { $error }
service-no-session = D-Bus ལཱ་ཡུན་མེད: { $error }

## Safe mode: an update left Katna's background service unable to start

safe-line = དུས་མཐུན་བཟོ་ནི་ལུ་དཀའ་ངལ་ཅིག་བྱུང་སྟེ་ Katna ཉེན་སྲུང་ཐབས་ལམ་ནང་ཡོདཔ་ལས་ གློག་འཕྲིན་མཉམ་སྡེབ་མི་འབད་བས།
safe-try-again = ལོག་འབད་རྩོལ་བསྐྱེད།
safe-restore = སོར་ཆུད།
safe-restoring = { $when } གི་ཁྱོད་ཀྱི་གནས་སྡུད་ སོར་ཆུད་འབད་དོ…
safe-restored = { $when } གི་ཁྱོད་ཀྱི་གནས་སྡུད་ སོར་ཆུད་འབད་ཡི། ཧེ་མ་ཡོད་མི་ཚུ་ སྣོད་འཛིན་ཅིག་ནང་བཞག་ཡོད།
safe-show-folder = སྣོད་འཛིན་སྟོན།
safe-restore-failed = ཁྱོད་ཀྱི་གནས་སྡུད་ སོར་ཆུད་འབད་མ་ཚུགས: { $error }
safe-restore-title = དུས་མཐུན་མ་བཟོ་བའི་ཧེ་མ་གི་ ཁྱོད་ཀྱི་གནས་སྡུད་ སོར་ཆུད་འབད་ནི་ཨིན་ན?
safe-restore-body = Katna ཁྱོད་ཀྱིས་གདམ་མི་འདྲ་བཤུས་ལུ་ལོག་འགྱོཝ་ཨིན། དེ་གི་ཤུལ་ལས་འོང་མི་གློག་འཕྲིན་ཚུ་ ཁྱོད་ཀྱི་རྩིས་ཐོ་ཚུ་ནང་ལས་ ལོག་ཕབ་ལེན་འབདཝ་ཨིན།
safe-restore-none = ད་ལྟོ་ཚུན་ འདྲ་བཤུས་མེད། དུས་མཐུན་བཟོ་བ་རེ་རེ་གིས་ ཁྱོད་ཀྱི་གནས་སྡུད་མ་བསྒྱུར་བའི་ཧེ་མ་ Katna གིས་ འདྲ་བཤུས་ཅིག་བཟོཝ་ཨིན།
safe-restore-keep = ད་ལྟོ་ཡོད་མི་ མ་བཏང་བའི་གློག་འཕྲིན་ ཟིན་བྲིས་ དེ་ལས་ ད་ཚུན་མཉམ་སྡེབ་མ་འབད་བའི་བསྒྱུར་བཅོས་ཚུ་ཚུད་དེ་ ཧེ་མ་ར་ སྣོད་འཛིན་ཅིག་ནང་བཞགཔ་ལས་ ག་ནི་ཡང་མི་བརླག།
safe-restore-cancel = ཆ་མེད་གཏང་།
safe-restore-mail = གློག་འཕྲིན
safe-restore-pim = རྩིས་ཐོ་དང་འབྲེལ་བ་ཚུ
safe-restore-blobs = མཉམ་སྦྲགས་ཚུ
safe-report-title = སྐྱོན་སེལ་སྙན་ཞུ
safe-report-body = འདི་འདྲ་བཤུས་རྐྱབ་སྟེ་ ཁྱོད་ཀྱི་སྐྱོན་གྱི་སྙན་ཞུ་ལུ་མཉམ་སྦྲགས་འབད། འདི་ནང་ གློག་འཕྲིན་ ཁ་བྱང་ ཡང་ན་ ཆོག་ཡིག་ག་ནི་ཡང་མེད།
safe-report-restore = སོར་ཆུད…
safe-report-copied = སྐྱོན་སེལ་སྙན་ཞུ་འདྲ་བཤུས་རྐྱབ་ཡི།
