# Katna Mail, Malayalam (മലയാളം).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = മെയിൽ സെർവർ
problems-signed-out = { $provider } { $address }-ൽ നിന്ന് Katna-യെ സൈൻ ഔട്ട് ചെയ്‌തു. മെയിൽ സമന്വയം നിലച്ചു.
problems-password-refused = { $provider } { $address }-ന്റെ പാസ്‌വേഡ് നിരസിച്ചു. അത് മാറിയിരിക്കാം.
problems-no-answer = { $provider } { $address }-ന് പ്രതികരിക്കുന്നില്ല. Katna ശ്രമം തുടരുന്നു.
problems-offline = നിങ്ങൾ ഓഫ്‌ലൈനാണ്. നിങ്ങളുടെ മെയിൽ ഇവിടെ തന്നെയുണ്ട്, നിങ്ങൾ അയയ്ക്കുന്ന മെയിൽ തിരികെ ഓൺലൈനാകുന്നത് വരെ കാത്തിരിക്കും.
problems-accounts-need-you = { $count ->
    [one] 1 അക്കൗണ്ടിന് നിങ്ങളുടെ ശ്രദ്ധ വേണം
   *[other] { $count } അക്കൗണ്ടുകൾക്ക് നിങ്ങളുടെ ശ്രദ്ധ വേണം
}
problems-show = കാണിക്കുക
problems-later = പിന്നീട്
problems-new-password = പുതിയ പാസ്‌വേഡ്
problems-try-again = വീണ്ടും ശ്രമിക്കുക

## The New password card

problems-password-title = പുതിയ പാസ്‌വേഡ്
problems-password-detail = { $provider } { $address }-ന്റെ സേവ് ചെയ്‌ത പാസ്‌വേഡ് നിരസിച്ചു. പുതിയത് ടൈപ്പ് ചെയ്യുക; സൂക്ഷിക്കുന്നതിന് മുമ്പ് Katna അത് പരിശോധിക്കും.
problems-password-placeholder = പാസ്‌വേഡ്
problems-password-show = പാസ്‌വേഡ് കാണിക്കുക
problems-password-hide = പാസ്‌വേഡ് മറയ്ക്കുക
problems-password-cancel = റദ്ദാക്കുക
problems-password-save = സേവ് ചെയ്യുക
problems-password-checking = പരിശോധിക്കുന്നു…
problems-password-refused-again = { $provider } ഈ പാസ്‌വേഡും നിരസിച്ചു. പരിശോധിച്ച് വീണ്ടും ശ്രമിക്കുക.
problems-password-saved = { $address }-ന്റെ പാസ്‌വേഡ് സേവ് ചെയ്‌തു. നിങ്ങളുടെ മെയിൽ എടുക്കുന്നു…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = { $address }-ന്റെ മെയിൽ സെർവർ { $count ->
    [one] ഒരു സന്ദേശം നീക്കുന്നത് സ്വീകരിച്ചില്ല, അതിനാൽ അത് പഴയ സ്ഥലത്ത് തിരിച്ചെത്തി.
   *[other] { $count } സന്ദേശങ്ങൾ നീക്കുന്നത് സ്വീകരിച്ചില്ല, അതിനാൽ അവ പഴയ സ്ഥലത്ത് തിരിച്ചെത്തി.
}
problems-refused-flags = { $address }-ന്റെ മെയിൽ സെർവർ { $count ->
    [one] ഒരു സന്ദേശം അടയാളപ്പെടുത്തുന്നത് (വായിച്ചത്, നക്ഷത്രമിട്ടത്…) സ്വീകരിച്ചില്ല, അതിനാൽ അത് പഴയപടിയായി.
   *[other] { $count } സന്ദേശങ്ങൾ അടയാളപ്പെടുത്തുന്നത് (വായിച്ചത്, നക്ഷത്രമിട്ടത്…) സ്വീകരിച്ചില്ല, അതിനാൽ അവ പഴയപടിയായി.
}
problems-refused-label = { $address }-ന്റെ മെയിൽ സെർവർ { $count ->
    [one] ഒരു സന്ദേശത്തിന്റെ ലേബലുകൾ മാറ്റുന്നത് സ്വീകരിച്ചില്ല, അതിനാൽ അത് പഴയപടിയായി.
   *[other] { $count } സന്ദേശങ്ങളുടെ ലേബലുകൾ മാറ്റുന്നത് സ്വീകരിച്ചില്ല, അതിനാൽ അവ പഴയപടിയായി.
}
problems-refused-delete = { $address }-ന്റെ മെയിൽ സെർവർ { $count ->
    [one] ഒരു സന്ദേശം ഇല്ലാതാക്കുന്നത് സ്വീകരിച്ചില്ല, അതിനാൽ അത് തിരിച്ചെത്തി.
   *[other] { $count } സന്ദേശങ്ങൾ ഇല്ലാതാക്കുന്നത് സ്വീകരിച്ചില്ല, അതിനാൽ അവ തിരിച്ചെത്തി.
}
problems-refused-other = { $address }-ന്റെ മെയിൽ സെർവർ { $count ->
    [one] ഒരു മാറ്റം സ്വീകരിച്ചില്ല, അതിനാൽ Katna അത് പഴയപടിയാക്കി.
   *[other] { $count } മാറ്റങ്ങൾ സ്വീകരിച്ചില്ല, അതിനാൽ Katna അവ പഴയപടിയാക്കി.
}
problems-details = വിശദാംശങ്ങൾ

## Katna's background service (katna-daemon) isn't running

service-starting = Katna-യുടെ പശ്ചാത്തല സേവനം ആരംഭിക്കുന്നു…
service-failed = Katna-യുടെ പശ്ചാത്തല സേവനം ആരംഭിക്കുന്നില്ല, അതിനാൽ മെയിൽ സമന്വയിപ്പിക്കുന്നില്ല.
service-start-again = വീണ്ടും ആരംഭിക്കുക
service-started-again = Katna-യുടെ പശ്ചാത്തല സേവനം നിലച്ചിരുന്നു, അത് വീണ്ടും ആരംഭിച്ചു.
service-details-title = സേവനം ആരംഭിക്കാത്തത് എന്തുകൊണ്ട്
service-details-body = ഇത് പകർത്തി നിങ്ങളുടെ റിപ്പോർട്ടിനൊപ്പം അയയ്ക്കുക. ഇതിൽ മെയിലോ പാസ്‌വേഡുകളോ ഇല്ല.
service-details-copy = പകർത്തുക
service-details-close = അടയ്ക്കുക
service-not-running = Katna പശ്ചാത്തല സേവനം പ്രവർത്തിക്കുന്നില്ല.
service-no-answer = Katna പശ്ചാത്തല സേവനം പ്രതികരിച്ചില്ല: { $error }
service-no-session = D-Bus സെഷൻ ഇല്ല: { $error }

## Safe mode: an update left Katna's background service unable to start

safe-line = അപ്‌ഡേറ്റിലെ ഒരു പ്രശ്‌നം കാരണം Katna സുരക്ഷിത മോഡിലാണ്, അതിനാൽ മെയിൽ സമന്വയിപ്പിക്കുന്നില്ല.
safe-try-again = വീണ്ടും ശ്രമിക്കുക
safe-restore = പുനഃസ്ഥാപിക്കുക
safe-restoring = { $when }-ലെ നിങ്ങളുടെ ഡാറ്റ പുനഃസ്ഥാപിക്കുന്നു…
safe-restored = { $when }-ലെ നിങ്ങളുടെ ഡാറ്റ പുനഃസ്ഥാപിച്ചു. മുമ്പ് ഉണ്ടായിരുന്നത് ഒരു ഫോൾഡറിൽ സൂക്ഷിച്ചിട്ടുണ്ട്.
safe-show-folder = ഫോൾഡർ കാണിക്കുക
safe-restore-failed = നിങ്ങളുടെ ഡാറ്റ പുനഃസ്ഥാപിക്കാനായില്ല: { $error }
safe-restore-title = ഒരു അപ്‌ഡേറ്റിന് മുമ്പുള്ള നിങ്ങളുടെ ഡാറ്റ പുനഃസ്ഥാപിക്കണോ?
safe-restore-body = നിങ്ങൾ തിരഞ്ഞെടുക്കുന്ന പകർപ്പിലേക്ക് Katna തിരികെ പോകുന്നു. അതിന് ശേഷം വന്ന മെയിൽ നിങ്ങളുടെ അക്കൗണ്ടുകളിൽ നിന്ന് വീണ്ടും ഡൗൺലോഡ് ചെയ്യും.
safe-restore-none = ഇതുവരെ പകർപ്പുകളൊന്നുമില്ല. ഓരോ അപ്‌ഡേറ്റും നിങ്ങളുടെ ഡാറ്റ മാറ്റുന്നതിന് മുമ്പ് Katna ഒരു പകർപ്പ് എടുക്കുന്നു.
safe-restore-keep = അയയ്ക്കാത്ത മെയിൽ, ഡ്രാഫ്റ്റുകൾ, ഇനിയും സമന്വയിപ്പിക്കാത്ത മാറ്റങ്ങൾ ഉൾപ്പെടെ ഇപ്പോഴുള്ളതെല്ലാം ആദ്യം ഒരു ഫോൾഡറിൽ സൂക്ഷിക്കുന്നു, അതിനാൽ ഒന്നും നഷ്‌ടമാകില്ല.
safe-restore-cancel = റദ്ദാക്കുക
safe-restore-mail = മെയിൽ
safe-restore-pim = അക്കൗണ്ടുകളും കോൺടാക്റ്റുകളും
safe-restore-blobs = അറ്റാച്ച്‌മെന്റുകൾ
safe-report-title = ഡീബഗ് റിപ്പോർട്ട്
safe-report-body = ഇത് പകർത്തി നിങ്ങളുടെ ബഗ് റിപ്പോർട്ടിൽ അറ്റാച്ച് ചെയ്യുക. ഇതിൽ മെയിലോ വിലാസങ്ങളോ പാസ്‌വേഡുകളോ ഇല്ല.
safe-report-restore = പുനഃസ്ഥാപിക്കുക…
safe-report-copied = ഡീബഗ് റിപ്പോർട്ട് പകർത്തി
