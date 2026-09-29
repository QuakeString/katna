# Katna Mail, Dzongkha (རྫོང་ཁ): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = འབྲེལ་བ་ཚུ
contacts-frequent = ཆེས་མང་སྤྱོད་མི
contacts-labels = ཁ་ཡིག་ཚུ
contacts-create = འབྲེལ་བ་གསར་བསྐྲུན་འབད།

## Search and the list

contacts-search = འབྲེལ་བ་ཚུ་ཨ་ཙེ་
contacts-loading = འབྲེལ་བ་ཚུ་ཐོ་བཀོད་འབད་དོ…
contacts-empty = ད་ལྟོ་ཡང་ཉར་ཚགས་འབད་ཡོད་པའི་འབྲེལ་བ་མིན་འདུག Gmail, Outlook ཡང་ན་ ཁྱོད་ཀྱི་གློག་འཕྲིན་ཞབས་ཏོག་ནང་ ཉར་ཚགས་འབད་ཡོད་པའི་འབྲེལ་བ་ཚུ་ འདི་ཁར་སྟོན་འོང་།
contacts-empty-no-books = ཁྱོད་ཀྱི་རྩིས་ཐོ་ཚུ་གི་འབྲེལ་བ་ཚུ་ མཉམ་བསྒྲིགས་འབད་ཞིནམ་ལས་ འདི་ཁར་སྟོན་འོང་།
contacts-none-found = ཁྱོད་ཀྱི་འཚོལ་ཞིབ་དང་མཐུན་པའི་འབྲེལ་བ་མིན་འདུག
contacts-starred = { $count ->
   *[other] སྐར་མ་བཀལ་ཡོད་པའི་འབྲེལ་བ་ཚུ་ ({ $count })
}
contacts-count = འབྲེལ་བ་ཚུ་ ({ $count })
contacts-col-name = མིང་
contacts-col-email = གློག་འཕྲིན་
contacts-col-phone = ཁ་པར་ཨང་།
contacts-col-job = ལས་ཀའི་མིང་དང་ཀམ་པ་ནི།
contacts-col-labels = ཁ་ཡིག་ཚུ

## Asking to allow contacts, for accounts signed in before Katna read them

contacts-allow = Katna ལུ་ { $address } གི་འབྲེལ་བ་ཚུ་ ལྷག་ནིའི་ཆོག་ཐམ་སྤྲོད།
contacts-allow-many = { $more ->
   *[other] Katna ལུ་ { $address } དང་ ལྷག་པའི་རྩིས་ཐོ་ { $more } གི་འབྲེལ་བ་ཚུ་ ལྷག་ནིའི་ཆོག་ཐམ་སྤྲོད།
}
contacts-allow-button = ཆོག་ཐམ་སྤྲོད།

## A contact's page

contacts-back = འབྲེལ་བ་ཚུ་ལུ་ལོག་འགྱོ།
contacts-edit = ཞུན་དག་འབད།
contacts-delete = བཏོན་གཏང་།
contacts-deleted = { $name } བཏོན་གཏང་ཡི།
contacts-find-mail = གློག་འཕྲིན
contacts-details = འབྲེལ་བའི་ཕྲ་ཞིབ།
contacts-saved-in = ཉར་ཚགས་འབད་ཡོད་སའི་ས་ཁོངས།
contacts-notes = དྲན་ཐོ་ཚུ
contacts-birthday = སྐྱེས་ཚེས།
contacts-nickname = མིང་ཐ་སྙད།
contacts-this-computer = གློག་རིག་འདི།
contacts-kind-home = ཁྱིམ།
contacts-kind-work = ལས་ཀ།
contacts-kind-mobile = སྒུལ་བདེ།
contacts-kind-other = གཞན།
contacts-source-google = Google འབྲེལ་བ་ཚུ
contacts-source-microsoft = Outlook འབྲེལ་བ་ཚུ
contacts-source-carddav = CardDAV

## Creating and changing a contact

contacts-edit-new-title = འབྲེལ་བ་གསར་བསྐྲུན་འབད།
contacts-edit-title = འབྲེལ་བ་ཞུན་དག་འབད།
contacts-edit-save = སྲུང་།
contacts-edit-saving = སྲུང་དོ…
contacts-edit-cancel = ཆ་མེད་གཏང་།
contacts-saved = འབྲེལ་བ་སྲུངས་ཡི།
contacts-edit-save-to = འདི་ནང་སྲུང་།
contacts-edit-changes-go-to = བསྒྱུར་བཅོས་ཚུ་ { $place } ནང་སྲུངམ་ཨིན།
contacts-edit-given = མིང་དང་པོ
contacts-edit-family = མིང་མཇུག
contacts-edit-company = ཀམ་པ་ནི།
contacts-edit-job = ལས་ཀའི་མིང་།
contacts-edit-email = གློག་འཕྲིན་
contacts-edit-phone = ཁ་པར།
contacts-edit-with-kind = { $field } ({ $kind })
contacts-edit-add-email = གློག་འཕྲིན་ཁ་སྣོན་འབད།
contacts-edit-add-phone = ཁ་པར་ཁ་སྣོན་འབད།
contacts-edit-street = ལམ་ཁའི་ཁ་བྱང་།
contacts-edit-city = གྲོང་ཁྱེར།
contacts-edit-postcode = ཡིག་ཚང་ཨང་།
contacts-edit-country = རྒྱལ་ཁབ།
contacts-edit-birthday = སྐྱེས་ཚེས། (YYYY-MM-DD)
contacts-edit-empty = ཐོག་མར་མིང་ ཡང་ན་ གློག་འཕྲིན་ ཡང་ན་ ཁ་པར་ཨང་ཁ་སྣོན་འབད།
