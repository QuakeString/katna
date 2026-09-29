# Katna Mail, Dzongkha (རྫོང་ཁ): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = བཟོ།
tasks-all = ལཱ་ཆ་མཉམ།
tasks-today = ད་རིས
tasks-starred = སྐར་མ་བཀལ་ཡོདཔ
tasks-new-list = ཐོ་ཡིག་གསརཔ་བཟོ།
tasks-on-this-computer = གློག་ཀླད་འདི་གུ།
tasks-my-tasks = ངེ་གི་ལཱ་ཚུ།
# The line under an account in the side list whose task lists could not
# come: why, and the one click that fixes it.
tasks-account-sign-in = ལཱ་ཚུ་སྟོན་ནི་ལུ་ ལོག་ནང་བསྐྱོད་འབད།
tasks-account-signed-in = { $address } ནང་ ལོག་ནང་བསྐྱོད་འབད་ཡི། ཁྱོད་ཀྱི་ལཱ་ཚུ་ལེན་དོ…
tasks-account-sign-in-refused = { $provider } གིས་ Katna ནང་ན་ འཛུལ་མ་བཅུག ལོག་འབད་རྩོལ་བསྐྱེད་ཞིནམ་ལས་ ཁྱོད་ཀྱི་ལཱ་ཚུ་ལུ་ འཛུལ་སྤྱོད་ཀྱི་གནང་བ་བྱིན།
tasks-account-refused = སར་བར་གྱིས་ ཆོག་ཡིག་ངོས་ལེན་མ་འབད། Yahoo དང་ iCloud Zoho དེ་ལས་ གཞན་ཚུ་ལུ་ གློག་རིམ་ཆོག་ཡིག་དགོཔ་ཨིན།
tasks-account-change-password = ཆོག་ཡིག་བསྒྱུར།
tasks-account-change-password-tooltip = སྒྲིག་སྟངས་ > རྩིས་ཐོ་ཚུ་ ཁ་ཕྱེ།
tasks-account-not-enabled = Katna གི་དོན་ལུ་ ལཱ་འཛུལ་སྤྱོད་ ད་ཚུན་ཚོད་ ཁ་མ་ཕྱེ་བས།
tasks-account-failed = ལཱ་ཐོ་ཡིག་ཚུ་ལྷག་མ་ཚུགས།
# $reason is the server's own words, in English.
tasks-account-error = ལཱ་ཐོ་ཡིག་ཚུ་ལྷག་མ་ཚུགས་: { $reason }
tasks-account-none = ལཱ་ཐོ་ཡིག་ག་ནི་ཡང་ མ་ཐོབ།
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
tasks-account-none-why = ལཱ་ཐོ་ཡིག་ག་ནི་ཡང་ མ་ཐོབ།: { $reason }
# A Gmail or Outlook account added with a password: its tasks need the
# provider's sign-in.
tasks-account-use-sign-in = { $provider } གིས་ ལཱ་ཚུ་ { $provider } གིས་ནང་བསྐྱོད་འབད་མི་ Katna ལུ་རྐྱངམ་ཅིག་སྟོནམ་ཨིན།
tasks-account-sign-in-with = { $provider } གིས་ ནང་བསྐྱོད་འབད།
tasks-account-looking = ལཱ་ཐོ་ཡིག་ཚུ་འཚོལ་དོ…
tasks-account-try-again = ལོག་འབད་རྩོལ་བསྐྱེད།
tasks-account-try-again-tooltip = རྩིས་ཐོ་འདི་གི་ལཱ་ཚུ་ ད་ལྟོ་ལོག་ཞིབ་དཔྱད་འབད།
tasks-account-fixing = ལཱ་འབད་དོ…
tasks-list-name-placeholder = ཐོ་ཡིག་གི་མིང་།

## Lists and tasks

tasks-loading = ཁྱོད་ཀྱི་ལཱ་ཚུ་ལྷག་དོ…
tasks-no-lists = ཁྱོད་ཀྱི་ལཱ་ཐོ་ཡིག་ཚུ་འདི་ལུ་སྟོནམ་ཨིན།
tasks-search = ལཱ་ཚུ་འཚོལ།
tasks-search-none = ཁྱོད་ཀྱི་འཚོལ་ཞིབ་དང་མཐུན་པའི་ལཱ་མིན་འདུག
tasks-add = ལཱ་ཅིག་ཁ་སྣོན་འབད།
tasks-title-placeholder = མགོ་མིང་།
tasks-add-step = ལཱ་ཆུང་ཅིག་ཁ་སྣོན་འབད།
tasks-empty = ལཱ་ཅིག་ཡང་མིན་འདུག ཡར་ལུ་ཅིག་ཁ་སྣོན་འབད།
tasks-starred-empty = འདི་ལུ་མཐོང་ནི་ལུ་ ལཱ་ཅིག་ལུ་སྐར་མ་བཀལ།
tasks-today-empty = ད་རིས་ཀྱི་ཆེ་ལུ་ ཅི་མི་འདུག།
tasks-today-date = { $weekday }, { $day }
tasks-overdue = ཚེས་ཐིག་ལས་ལྷག་པ།
tasks-completed = { $count ->
   *[other] མཇུག་བསྡུ་ཡོད་མི་ ({ $count })
}
tasks-list-options = ཐོ་ཡིག་གི་གདམ་ཁ།
tasks-rename-list = ཐོ་ཡིག་གི་མིང་བསྒྱུར།
tasks-delete-list = ཐོ་ཡིག་བཏོན་གཏང་།
tasks-mark-done = མཇུག་བསྡུ་ཡོདཔ་སྦེ་རྟགས་བཀལ།
tasks-mark-open = མཇུག་མ་བསྡུ་བར་རྟགས་བཀལ།
tasks-star = སྐར་མ་བཀལ།
tasks-unstar = སྐར་མ་བཏོན།
tasks-edit-title = མགོ་མིང་ཞུན་དག་འབད།
tasks-details = ཁ་གསལ།
tasks-delete = བཏོན་གཏང་།
tasks-move-to = { $list } ནང་སྤོ།
tasks-from-mail = གློག་འཕྲིན
tasks-open-mail = གློག་འཕྲིན་ཁ་ཕྱེ།
tasks-from-note = དྲན་ཐོ
tasks-open-note = དྲན་ཐོ་ཁ་ཕྱེ།
tasks-note-gone = དྲན་ཐོ་དེ་ད་ལུ་འདི་ལུ་མིན་འདུག
tasks-no-subject = (དོན་ཚན་མེད)

## The details dialog

tasks-notes-placeholder = ཁ་གསལ་ཁ་སྣོན་འབད།
tasks-date = ཚེས་གྲངས།
tasks-no-date = ཚེས་གྲངས་མེད།
tasks-time-placeholder = ཆུ་ཚོད་ཁ་སྣོན་འབད།
tasks-repeat = ཁྱད་བསྐྱར།
tasks-repeat-never = ཁྱད་བསྐྱར་མི་འབད།
tasks-repeat-daily = ཉིནམ་ཐེར།
tasks-repeat-weekly = བདུན་ཕྲག་ཐེར།
tasks-repeat-monthly = ཟླ་བ་ཐེར།
tasks-repeat-yearly = ལོ་ཐེར།
tasks-repeat-other = རང་བཟོ།
tasks-remind = ང་ལུ་དྲན་སྐུལ་འབད་
tasks-remind-off = དྲན་སྐུལ་མ་འབད།
tasks-remind-on-time = དུས་ཚོད་དེ་ནང་
tasks-remind-morning = ཉིན་དེ་ལུ་, { $time }
tasks-remind-hour-before = ཆུ་ཚོད་གཅིག་ཧེ་མ་
tasks-remind-day-before = ཉིན་གཅིག་ཧེ་མ་
tasks-cancel = ཆ་མེད་གཏང་།
tasks-save = སྲུང་།
tasks-not-a-time = “{ $text }” འདི་ ཆུ་ཚོད་མེན། དཔེར་ན་ { $example }།

## Due days

tasks-due-today = ད་རིས།
tasks-due-tomorrow = ནངས་པ།
tasks-due-yesterday = ཁ་ཙ།
tasks-due-at = { $day }, { $time }

## Notes at the bottom

tasks-toast-done = ལཱ་མཇུག་བསྡུཝ་ཨིན།
tasks-toast-next = འབད་ཚར་ཡི། ཤུལ་མམ་དེ་ { $date } ལུ།
tasks-toast-deleted = ལཱ་བཏོན་གཏང་ཡི།
tasks-toast-added = { $count ->
   *[other] ལཱ་ { $count } ཁ་སྣོན་འབད་ཡི།
}
tasks-mail-gone = གློག་འཕྲིན་དེ་ད་ལུ་འདི་ལུ་མིན་འདུག
tasks-toast-list-deleted = ཐོ་ཡིག་བཏོན་གཏང་ཡི།
tasks-toast-moved = { $list } ནང་སྤོ་ཡི།
tasks-toast-rescheduled = ལས་འགན་གྱི་དུས་ཚོད་བསྒྱུར་ཡི།
