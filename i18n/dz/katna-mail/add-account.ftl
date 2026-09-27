# Katna Mail, Dzongkha (རྫོང་ཁ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = གློག་འཕྲིན་རྩིས་ཐོ་ཁ་སྐོང་འབད།
add-account-looking = { $address } གི་ གློག་འཕྲིན་སར་བར་ཚུ་འཚོལ་དོ…
add-account-address-intro = ཁྱོད་ཀྱི་གློག་འཕྲིན་ཁ་བྱང་བཙུགས། Katna གིས་ ཁྱོད་ཀྱི་དོན་ལུ་ སར་བར་ཚུ་འཚོལཝ་ཨིན།
add-account-servers-title = སར་བར་སྒྲིག་སྟངས
add-account-servers-intro = Katna གིས་ { $address } གི་གློག་འཕྲིན་ ལྷག་ནི་དང་གཏང་སའི་ས་གནས།
add-account-password-title = ཁྱོད་ཀྱི་ཆོག་ཡིག་བཙུགས།
add-account-signing-in = ནང་བསྐྱོད་འབད་དོ…

## Add a mail account: fields

add-account-field-address = གློག་འཕྲིན་ཁ་བྱང
add-account-incoming = ནང་འོང་གློག་འཕྲིན ({ $protocol })
add-account-outgoing = ཕྱིར་འགྱོ་གློག་འཕྲིན ({ $protocol })
add-account-field-server = སར་བར
add-account-field-port = འདྲེན་ལམ
add-account-security-none = མེད
add-account-field-username = ལག་ལེན་པའི་མིང
add-account-field-password = ཆོག་ཡིག
add-account-show-password = ཆོག་ཡིག་སྟོན།
add-account-app-password-hint = { $provider } ལུ་ ནཱ་ལུ་ གློག་རིམ་ཆོག་ཡིག་དགོཔ་ཨིན། ཁྱོད་ཀྱིས་ ཝེབ་གུ་ལག་ལེན་འཐབ་མི་འདི་མེན། ཁྱོད་ཀྱི་ { $provider } རྩིས་ཐོའི་ ཉེན་སྲུང་སྒྲིག་སྟངས་ནང་ གཅིག་བཟོ།
add-account-field-name = ཁྱོད་ཀྱི་མིང (གདམ་ཁ)
add-account-name-hint = ཁྱོད་ཀྱིས་ཡི་གུ་བྲི་མི་ མི་ཚུ་ལུ་སྟོནམ་ཨིན།
add-account-servers-pair = { $imap } དང་ { $smtp }
add-account-servers-found = { $source ->
    [built-in] སར་བར: { $servers }། Katna གི་བྱིན་མིའི་ཐོ་ཡིག་ནང་ཐོབ་ཅི།
    [provider] སར་བར: { $servers }། ཁྱོད་ཀྱི་བྱིན་མིའི་སྒྲིག་སྟངས་ནང་ཐོབ་ཅི།
    [ispdb] སར་བར: { $servers }། Thunderbird གི་བྱིན་མིའི་ཐོ་ཡིག་ནང་ཐོབ་ཅི།
    [dns] སར་བར: { $servers }། ཁྱོད་ཀྱི་ཌོ་མེན་གྱི་ DNS ཐོ་བཀོད་ནང་ཐོབ་ཅི།
   *[other] སར་བར: { $servers }། ཚོད་དཔག་ཨིན། ནང་བསྐྱོད་འཐུས་ཤོར་བྱུང་པ་ཅིན་ ཞིབ་དཔྱད་འབད།
}
add-account-servers-entered = སར་བར: { $servers }། བཙུགས་མི་བཟུམ་སྦེ།

## Add a mail account: buttons

add-account-servers-button = སར་བར་སྒྲིག་སྟངས
add-account-back = ལོག
add-account-add = རྩིས་ཐོ་ཁ་སྐོང་འབད།
add-account-next = ཤུལ་མམ།
add-account-cancel = ཆ་མེད་གཏང་།

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] ནང་འོང་སར་བར་བཙུགས།
   *[outgoing] ཕྱིར་འགྱོ་སར་བར་བཙུགས།
}
add-account-server-space = { $kind ->
    [incoming] ནང་འོང་སར་བར་གྱི་མིང་ནང་ བར་སྟོང་ཅིག་ཡོད།
   *[outgoing] ཕྱིར་འགྱོ་སར་བར་གྱི་མིང་ནང་ བར་སྟོང་ཅིག་ཡོད།
}
add-account-port-invalid = { $kind ->
    [incoming] ནང་འོང་འདྲེན་ལམ་འདི་ { $min } ལས་ { $max } ཚུན་གྱི་ ཨང་ཅིག་ཨིན་དགོ།
   *[outgoing] ཕྱིར་འགྱོ་འདྲེན་ལམ་འདི་ { $min } ལས་ { $max } ཚུན་གྱི་ ཨང་ཅིག་ཨིན་དགོ།
}
add-account-address-empty = གློག་འཕྲིན་ཁ་བྱང་ཅིག་བཙུགས།
add-account-address-invalid = { $example } བཟུམ་གྱི་ གློག་འཕྲིན་ཁ་བྱང་ཅིག་བཙུགས།
add-account-not-found = Katna གིས་ { $address } གི་སར་བར་ཚུ་ འཚོལ་མ་ཐོབ། དེ་འབདཝ་ལས་ སྤྱིར་བཏང་མིང་ཚུ་བཙུགས་ཅི། ཁྱོད་ཀྱི་བྱིན་མི་དང་གཅིག་ཁར་ ཞིབ་དཔྱད་འབད།
add-account-password-empty = ཆོག་ཡིག་བཙུགས།
add-account-name-is-password = མིང་འདི་ ཆོག་ཡིག་དང་གཅིག་པ་ཨིན། དེའི་ཚབ་ལུ་ མི་ཚུ་གིས་མཐོང་དགོཔ་བཟུམ་སྦེ་ ཁྱོད་ཀྱི་མིང་ དེ་ཁར་ཡིག་དཔར་རྐྱབ།
add-account-added = { $address } ཁ་སྐོང་འབད་ཡི། ཁྱོད་ཀྱི་གློག་འཕྲིན་ལེན་དོ…
add-account-app-password-refused = { $provider } གིས་ ཆོག་ཡིག་ངོས་ལེན་མ་འབད། གློག་རིམ་ཆོག་ཡིག་དགོཔ་ཨིན། ཁྱོད་ཀྱིས་ ཝེབ་གུ་ལག་ལེན་འཐབ་མི་འདི་མེན།
add-account-password-refused = སར་བར་གྱིས་ ཆོག་ཡིག་ངོས་ལེན་མ་འབད། ཞིབ་དཔྱད་འབད་ཞིནམ་ལས་ ལོག་འབད་རྩོལ་བསྐྱེད།

## The account menu (from the account button on the top bar)

add-account-menu-another = རྩིས་ཐོ་གཞན་ཅིག་ཁ་སྐོང་འབད།
add-account-menu-manage = རྩིས་ཐོ་ཚུ་འཛིན་སྐྱོང་འབད།
