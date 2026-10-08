# Katna Mail, Dzongkha (རྫོང་ཁ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification

notify-new-emails = ཡིག་འཕྲིན་གསརཔ་ { $count }
notify-and-more = དང་ གཞན་ { $count }
notify-no-subject = (དོན་ཚན་མེད)
notify-unknown-sender = གཏང་མི་མ་ཤེསཔ
notify-snooze-back = ཤུལ་མར་བཞག་མི་ལས་ ལོག་འོང་ཡི།
notify-no-reply = ད་ཚུན་ ལན་མ་འོང་།
notify-no-reply-to = “{ $subject }” ལུ་ མི་སུ་གིས་ཡང་ ལན་མ་སློག།
notify-follow-up-sent = རྗེས་འདེད་འཕྲིན་དོན་བཏང་ཡི
notify-follow-up-sent-to = “{ $subject }” ལུ་ མི་སུ་གིས་ཡང་ལན་མ་སློག་པས་ Katna གིས་ རྗེས་འདེད་འབད་ཡི།
notify-follow-up-waiting = རྗེས་འདེད་འཕྲིན་དོན་མ་བཏང
notify-follow-up-waiting-to = གློག་རིག་འདི་ཁ་བསྡམས་ཏེ་ཡོད་པའི་སྐབས་ དུས་ཚོད་ལྷོད་ཡི། “{ $subject }” ཁྱོད་ཀྱི་ནང་འབྱོར་སྒྲོམ་ནང་ལོག་འོང་ཡི།
notify-tracking-opened = { $who } གིས་ { $subject } ཁ་ཕྱེ་ཡི
notify-tracking-clicked = { $who } གིས་ { $subject } ནང་གི་འབྲེལ་མཐུད་གཅིག་ཨེབ་གཏང་འབད་ཡི

## An update of Katna is downloaded and ready to install

notify-update-ready = Katna Mail དུས་མཐུན་བཟོ་བཏུབ།
notify-update-ready-body = ཐོན་རིམ་ { $version } ཕབ་ལེན་འབད་ཡོད། དུས་མཐུན་བཟོ་ནི་གིས་དེ་གཞི་བཙུགས་འབད་དེ་ Katna Mail་ལོག་འགོ་བཙུགས་འབདཝ་ཨིན།
notify-update = དུས་མཐུན་བཟོ།

## Something needs the user, shown once per problem

notify-signed-out = ལོག་ནང་བསྐྱོད་འབད།
notify-signed-out-body = { $provider } གིས་ Katna འདི་ { $address } ལས་ཕྱིར་བཏོན་ཡི། གློག་འཕྲིན་མཉམ་སྡེབ་འབད་ནི་བཀག་ཡི།
notify-sign-in = ནང་བསྐྱོད་འབད།
notify-password-refused = ཆོག་ཡིག་ངོས་ལེན་མ་འབད
notify-password-refused-body = གློག་འཕྲིན་སར་བར་གྱིས་ { $address } གི་ཆོག་ཡིག་ངོས་ལེན་མ་འབད། འདི་བསྒྱུར་ཡོདཔ་འོང་།
notify-new-password = ཆོག་ཡིག་གསརཔ
notify-not-sent = “{ $subject }” མ་བཏང་།
notify-not-sent-no-subject = འཕྲིན་དོན་ཅིག་མ་བཏང་།
notify-not-sent-body = འདི་ ཕྱིར་གཏང་སྒྲོམ་ནང་ཡོད། དེ་ནང་ རྒྱུ་མཚན་སྟོནམ་ཨིན།
notify-open-outbox = ཕྱིར་གཏང་སྒྲོམ་ཁ་ཕྱེ།
notify-event-now = ད་ལྟོ
notify-event-in-minutes = { $count ->
   *[other] སྐར་མ་ { $count } ནང་
}
notify-event-in-hours = { $count ->
   *[other] ཆུ་ཚོད་ { $count } ནང་
}
notify-event-in-days = { $count ->
    [1] ནངས་པ
   *[other] ཉིནམ་ { $count } ནང་
}
notify-event-all-day = ཉིན་མོ་ཧྲིལ་བུ
notify-event-join = ཚུད་གནང་
notify-event-snooze = སྐར་མ་ 5 ཤུལ་མར་བཞགཔ
notify-task-done = ཚར་ཡི་ཟེར་རྟགས་བཀོད་

## Its buttons

notify-open = ཁ་ཕྱེ།
notify-peek = བལྟ།
notify-reply = ལན་སློག
notify-reply-placeholder = { $name } ལུ་ལན་སློག…
notify-send = གཏང་།
notify-reply-quote-header = { $date } ལུ་ { $from } གིས་བྲིས་མི:
notify-reply-quote-header-no-date = { $from } གིས་བྲིས་མི:
notify-reply-all = ཆ་མཉམ་ལུ་ལན་སློག
notify-mark-read = ལྷག་ཡོདཔ་སྦེ་རྟགས་བཀལ།
notify-mark-all-read = ཆ་མཉམ་ལྷག་ཡོདཔ་སྦེ་རྟགས་བཀལ།
notify-archive = ཡིག་མཛོད་ནང་བཙུགས།
notify-snooze-hour = ཆུ་ཚོད་ ༡ ཤུལ་མར་བཞག
notify-snooze-tomorrow = ནངས་པ
notify-copy-code = { $code } འདྲ་བཤུས་རྐྱབས།
notify-link-verify = { $domain } གུ་ བདེན་དཔྱད་འབད།
notify-link-confirm = { $domain } གུ་ ངེས་དཔྱད་འབད།
notify-link-activate = { $domain } གུ་ ཤུགས་ལྡན་བཏང་།

## After Archive on a notification: a short note in the same place

notify-archived = ཡིག་མཛོད་ནང་བཙུགས་ཡི།
notify-archived-count = { $count ->
   *[other] འཕྲིན་དོན་ { $count } ནང་འབྱོར་སྒྲོམ་ལས་ཕྱི་ཁར་སྤོ་ཡི།
}
notify-undo = ལོག་འབད།

## After Copy on a notification: a short note, with the code under the title

notify-code-copied = ཨང་རྟགས་འདྲ་བཤུས་རྐྱབ་ཡི
notify-code-not-copied = ཨང་རྟགས་འདྲ་བཤུས་རྐྱབ་མ་ཚུགས

## After a reply typed into a notification: a note in the same place while
## it waits for the undo time

notify-reply-sent = { $name } ལུ་ལན་བཏང་ཡི།
notify-open-in-katna = Katna ནང་ཁ་ཕྱེ།
