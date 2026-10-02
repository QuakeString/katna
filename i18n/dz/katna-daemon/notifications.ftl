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
notify-tracking-opened = { $who } གིས་ { $subject } ཁ་ཕྱེ་ཡི
notify-tracking-clicked = { $who } གིས་ { $subject } ནང་གི་འབྲེལ་མཐུད་གཅིག་ཨེབ་གཏང་འབད་ཡི

## An update of Katna is downloaded and ready to install

notify-update-ready = Katna Mail དུས་མཐུན་བཟོ་བཏུབ།
notify-update-ready-body = ཐོན་རིམ་ { $version } ཕབ་ལེན་འབད་ཡོད། དུས་མཐུན་བཟོ་ནི་གིས་དེ་གཞི་བཙུགས་འབད་དེ་ Katna Mail་ལོག་འགོ་བཙུགས་འབདཝ་ཨིན།
notify-update = དུས་མཐུན་བཟོ།
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
notify-reply-all = ཆ་མཉམ་ལུ་ལན་སློག
notify-mark-read = ལྷག་ཡོདཔ་སྦེ་རྟགས་བཀལ།
notify-mark-all-read = ཆ་མཉམ་ལྷག་ཡོདཔ་སྦེ་རྟགས་བཀལ།
notify-archive = ཡིག་མཛོད་ནང་བཙུགས།

## After Archive on a notification: a short note in the same place

notify-archived = ཡིག་མཛོད་ནང་བཙུགས་ཡི།
notify-archived-count = { $count ->
   *[other] འཕྲིན་དོན་ { $count } ནང་འབྱོར་སྒྲོམ་ལས་ཕྱི་ཁར་སྤོ་ཡི།
}
notify-undo = ལོག་འབད།

## After a reply typed into a notification: a note in the same place while
## it waits for the undo time

notify-reply-sent = { $name } ལུ་ལན་བཏང་ཡི།
notify-open-in-katna = Katna ནང་ཁ་ཕྱེ།
