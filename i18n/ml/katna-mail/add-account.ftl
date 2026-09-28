# Katna Mail, Malayalam (മലയാളം).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = ഒരു മെയിൽ അക്കൗണ്ട് ചേർക്കുക
add-account-looking = { $address }-ന്റെ മെയിൽ സെർവറുകൾ തിരയുന്നു…
add-account-address-intro = നിങ്ങളുടെ ഇമെയിൽ വിലാസം നൽകുക. Katna നിങ്ങൾക്കായി സെർവറുകൾ കണ്ടെത്തും.
add-account-servers-title = സെർവർ ക്രമീകരണം
add-account-servers-intro = { $address }-നായി Katna മെയിൽ വായിക്കുകയും അയയ്ക്കുകയും ചെയ്യുന്ന ഇടം.
add-account-password-title = നിങ്ങളുടെ പാസ്‌വേഡ് നൽകുക
add-account-signing-in = സൈൻ ഇൻ ചെയ്യുന്നു…
add-account-browser-title = നിങ്ങളുടെ ബ്രൗസറിൽ തുടരുക
add-account-browser-intro = Katna നിങ്ങളുടെ ബ്രൗസറിൽ { $provider } സൈൻ ഇൻ പേജ് തുറന്നു. അവിടെ സൈൻ ഇൻ ചെയ്ത് നിങ്ങളുടെ മെയിൽ വായിക്കാനും അയയ്ക്കാനും Katna-യെ അനുവദിക്കുക, തുടർന്ന് ഇവിടേക്ക് മടങ്ങുക.
add-account-browser-hint = പേജൊന്നും തുറന്നില്ലേ? നിങ്ങളുടെ ബ്രൗസറിന്റെ വിൻഡോകൾ പരിശോധിക്കുക, അല്ലെങ്കിൽ പിന്നോട്ട് പോയി വീണ്ടും ശ്രമിക്കുക.

## Add a mail account: fields

add-account-field-address = ഇമെയിൽ വിലാസം
add-account-incoming = ഇൻകമിംഗ് മെയിൽ ({ $protocol })
add-account-outgoing = ഔട്ട്‌ഗോയിംഗ് മെയിൽ ({ $protocol })
add-account-field-server = സെർവർ
add-account-field-port = പോർട്ട്
add-account-security-none = ഒന്നുമില്ല
add-account-security-none-warning = എൻക്രിപ്റ്റ് ചെയ്തിട്ടില്ല: നിങ്ങളുടെ പാസ്‌വേഡും മെയിലും വഴിയിൽ വായിക്കാൻ കഴിയും.
add-account-field-username = ഉപയോക്തൃനാമം
add-account-field-password = പാസ്‌വേഡ്
add-account-show-password = പാസ്‌വേഡ് കാണിക്കുക
add-account-app-password-hint = ഇവിടെ { $provider }-ന് ഒരു ആപ്പ് പാസ്‌വേഡ് വേണം, വെബിൽ നിങ്ങൾ ഉപയോഗിക്കുന്നതല്ല. നിങ്ങളുടെ { $provider } അക്കൗണ്ടിന്റെ സുരക്ഷാ ക്രമീകരണത്തിൽ ഒന്ന് ഉണ്ടാക്കുക.
add-account-field-name = നിങ്ങളുടെ പേര് (ഓപ്ഷണൽ)
add-account-name-hint = നിങ്ങൾ എഴുതുന്ന ആളുകൾക്ക് കാണിക്കും.
add-account-servers-pair = { $imap }, { $smtp }
add-account-servers-found = { $source ->
    [built-in] സെർവറുകൾ: { $servers }, Katna-യുടെ ദാതാക്കളുടെ പട്ടികയിൽ കണ്ടെത്തി.
    [provider] സെർവറുകൾ: { $servers }, നിങ്ങളുടെ ദാതാവിന്റെ ക്രമീകരണത്തിൽ കണ്ടെത്തി.
    [ispdb] സെർവറുകൾ: { $servers }, Thunderbird-ന്റെ ദാതാക്കളുടെ പട്ടികയിൽ കണ്ടെത്തി.
    [dns] സെർവറുകൾ: { $servers }, നിങ്ങളുടെ ഡൊമെയ്‌നിന്റെ DNS റെക്കോർഡുകളിൽ കണ്ടെത്തി.
   *[other] സെർവറുകൾ: { $servers }, ഊഹിച്ചത്; സൈൻ ഇൻ പരാജയപ്പെട്ടാൽ അവ പരിശോധിക്കുക.
}
add-account-servers-entered = സെർവറുകൾ: { $servers }, നൽകിയതുപോലെ.
add-account-or = അല്ലെങ്കിൽ
add-account-sign-in-with = { $provider } ഉപയോഗിച്ച് സൈൻ ഇൻ ചെയ്യുക
add-account-sign-in-instead = പകരം { $provider } ഉപയോഗിച്ച് സൈൻ ഇൻ ചെയ്യുക

## Add a mail account: buttons

add-account-servers-button = സെർവർ ക്രമീകരണം
add-account-back = പിന്നോട്ട്
add-account-add = അക്കൗണ്ട് ചേർക്കുക
add-account-next = അടുത്തത്
add-account-cancel = റദ്ദാക്കുക

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] ഇൻകമിംഗ് സെർവർ നൽകുക.
   *[outgoing] ഔട്ട്‌ഗോയിംഗ് സെർവർ നൽകുക.
}
add-account-server-space = { $kind ->
    [incoming] ഇൻകമിംഗ് സെർവറിന്റെ പേരിൽ ഒരു സ്‌പേസ് ഉണ്ട്.
   *[outgoing] ഔട്ട്‌ഗോയിംഗ് സെർവറിന്റെ പേരിൽ ഒരു സ്‌പേസ് ഉണ്ട്.
}
add-account-port-invalid = { $kind ->
    [incoming] ഇൻകമിംഗ് പോർട്ട് { $min } മുതൽ { $max } വരെയുള്ള ഒരു സംഖ്യയായിരിക്കണം.
   *[outgoing] ഔട്ട്‌ഗോയിംഗ് പോർട്ട് { $min } മുതൽ { $max } വരെയുള്ള ഒരു സംഖ്യയായിരിക്കണം.
}
add-account-address-empty = ഒരു ഇമെയിൽ വിലാസം നൽകുക.
add-account-address-invalid = { $example } പോലെയുള്ള ഒരു ഇമെയിൽ വിലാസം നൽകുക.
add-account-not-found = Katna-യ്ക്ക് { $address }-ന്റെ സെർവറുകൾ കണ്ടെത്താനായില്ല, അതിനാൽ സാധാരണ പേരുകൾ പൂരിപ്പിച്ചു. നിങ്ങളുടെ ദാതാവുമായി അവ പരിശോധിക്കുക.
add-account-password-empty = പാസ്‌വേഡ് നൽകുക.
add-account-name-is-password = പേര് പാസ്‌വേഡിന് തുല്യമാണ്. പകരം അവിടെ നിങ്ങളുടെ പേര്, ആളുകൾ കാണേണ്ട രീതിയിൽ, ടൈപ്പ് ചെയ്യുക.
add-account-added = { $address } ചേർത്തു. നിങ്ങളുടെ മെയിൽ കൊണ്ടുവരുന്നു…
add-account-app-password-refused = { $provider } പാസ്‌വേഡ് നിരസിച്ചു. അതിന് ഒരു ആപ്പ് പാസ്‌വേഡ് വേണം, വെബിൽ നിങ്ങൾ ഉപയോഗിക്കുന്നതല്ല.
add-account-password-refused = സെർവർ പാസ്‌വേഡ് നിരസിച്ചു. അത് പരിശോധിച്ച് വീണ്ടും ശ്രമിക്കുക.
add-account-sign-in-refused = { $provider } Katna-യെ അകത്ത് കയറ്റിയില്ല. വീണ്ടും ശ്രമിക്കുക, നിങ്ങളുടെ മെയിലിലേക്ക് ആക്‌സസ് അനുവദിക്കുക.
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] Katna-യുടെ ഈ പകർപ്പിന് ഇതുവരെ Microsoft അക്കൗണ്ടുകളിൽ സൈൻ ഇൻ ചെയ്യാനാകില്ല.
    [Google] Katna-യുടെ ഈ പകർപ്പിന് ഇതുവരെ Google അക്കൗണ്ടുകളിൽ സൈൻ ഇൻ ചെയ്യാനാകില്ല.
   *[other] ഈ ദാതാവ് സ്വന്തം പേജിൽ മാത്രമേ സൈൻ ഇൻ അനുവദിക്കൂ, അത് Katna-യ്ക്ക് ഇതുവരെ ചെയ്യാനാകില്ല.
}
add-account-signed-in = { $provider } ഉപയോഗിച്ച് സൈൻ ഇൻ ചെയ്തു. നിങ്ങളുടെ മെയിൽ ലഭ്യമാക്കുന്നു…

## The account menu (from the account button on the top bar)

add-account-menu-another = മറ്റൊരു അക്കൗണ്ട് ചേർക്കുക
add-account-menu-manage = അക്കൗണ്ടുകൾ നിയന്ത്രിക്കുക
app-menu = പ്രധാന മെനു
