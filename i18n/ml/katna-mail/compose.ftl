# Katna Mail, Malayalam (മലയാളം).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Compose window: title bar

compose-new-message = പുതിയ സന്ദേശം
compose-restore = പുനഃസ്ഥാപിക്കുക
compose-minimize = ചെറുതാക്കുക
compose-exit-full-screen = പൂർണ്ണ സ്ക്രീനിൽ നിന്ന് പുറത്തുകടക്കുക
compose-open-window = പുതിയ വിൻഡോയിൽ തുറക്കുക
compose-save-close = സംരക്ഷിച്ച് അടയ്ക്കുക
compose-back-to-mail = മെയിൽ വിൻഡോയിലേക്ക് മടങ്ങുക
compose-pop-out-reply = മറുപടി പ്രത്യേക വിൻഡോയിൽ തുറക്കുക
compose-show-trimmed = ചുരുക്കിയ ഉള്ളടക്കം കാണിക്കുക

## Recipients and subject

compose-to = സ്വീകർത്താവ്
compose-cc = Cc
compose-bcc = Bcc
compose-from = അയച്ചയാൾ
compose-from-choose = മറ്റൊരു അക്കൗണ്ടിൽ നിന്ന് അയയ്ക്കുക
compose-recipients = സ്വീകർത്താക്കൾ
compose-subject = വിഷയം

## Sending (the notes at the bottom of the window)

compose-open-elsewhere = ആദ്യം തുറന്നിരിക്കുന്ന സന്ദേശം അയയ്ക്കുകയോ നിരസിക്കുകയോ ചെയ്യുക.
compose-bad-address = “{ $address }” ഒരു ഇമെയിൽ വിലാസമല്ല.
compose-no-recipients = കുറഞ്ഞത് ഒരു സ്വീകർത്താവിനെയെങ്കിലും ചേർക്കുക.
compose-attachments-too-large = അറ്റാച്ച്‌മെന്റുകളുടെ വലുപ്പം { $size } ആണ്; മെയിൽ സെർവറുകൾ { $limit } വരെ മാത്രമേ സ്വീകരിക്കൂ.
compose-no-account = മെയിൽ അയയ്ക്കാൻ ഒരു അക്കൗണ്ട് ചേർക്കുക.
compose-past-time = ഭാവിയിലെ ഒരു സമയം തിരഞ്ഞെടുക്കുക.
compose-scheduling = ഷെഡ്യൂൾ ചെയ്യുന്നു…
compose-sending = അയയ്ക്കുന്നു…
compose-scheduled = { $when } ന് അയയ്ക്കാൻ ഷെഡ്യൂൾ ചെയ്തു
compose-sent-archived = അയച്ചു, ആർക്കൈവ് ചെയ്തു
compose-sent = സന്ദേശം അയച്ചു
compose-discarded = ഡ്രാഫ്റ്റ് നിരസിച്ചു
compose-draft-saved = ഡ്രാഫ്റ്റ് സംരക്ഷിച്ചു
compose-draft-failed = ഡ്രാഫ്റ്റ് സംരക്ഷിക്കാനായില്ല: { $error }
compose-draft-not-opened = ഡ്രാഫ്റ്റ് തുറക്കാനായില്ല.

## Attachments

compose-picker-insert = ചേർക്കുക
compose-picker-attach = അറ്റാച്ച് ചെയ്യുക
compose-file-too-large = { $name } വളരെ വലുതാണ്: ഒരു സന്ദേശത്തിൽ { $limit } വരെ മാത്രമേ ഉൾക്കൊള്ളാനാകൂ.
compose-attachment-size = ({ $size })
compose-remove-attachment = അറ്റാച്ച്‌മെന്റ് നീക്കം ചെയ്യുക
compose-attachments-total = { $count ->
    [one] { $count } ഫയൽ, { $size }
   *[other] { $count } ഫയലുകൾ, { $size }
}
compose-drop-files = ഫയലുകൾ ഇവിടെ ഇടുക
compose-drop-here = ഇവിടെ ഇടുക
compose-paste-keep-formatting = ഫോർമാറ്റിംഗ് നിലനിർത്തുക
compose-paste-table = പട്ടിക
compose-paste-picture = ചിത്രം
compose-paste-plain-text = പ്ലെയിൻ ടെക്സ്റ്റ്
compose-paste-inline = ടെക്സ്റ്റിനുള്ളിൽ
compose-paste-attachment = അറ്റാച്ച്‌മെന്റ്

## Encryption and signing (the toggles by the recipients)

compose-encrypt = എൻക്രിപ്റ്റ് ചെയ്യുക
compose-encrypted = എൻക്രിപ്റ്റ് ചെയ്‌തു: സ്വീകർത്താക്കൾക്ക് മാത്രമേ ഇത് വായിക്കാനാകൂ
compose-sign = ഒപ്പിടുക
compose-signed = ഒപ്പിട്ടു: ഇത് നിങ്ങളിൽ നിന്നാണെന്ന് സ്വീകർത്താക്കൾക്ക് പരിശോധിക്കാം
compose-track = തുറക്കലും ക്ലിക്കുകളും ട്രാക്ക് ചെയ്യുക
compose-tracked = ട്രാക്ക് ചെയ്യുന്നു: ഓരോ സ്വീകർത്താവും ഇത് എപ്പോൾ തുറക്കുന്നു അല്ലെങ്കിൽ ലിങ്ക് തുറക്കുന്നു എന്ന് നിങ്ങൾക്ക് കാണാം
compose-track-unavailable = ഒപ്പിട്ടതും എൻക്രിപ്റ്റ് ചെയ്തതും പ്ലെയിൻ ടെക്സ്റ്റ് ആയതുമായ മെയിൽ ട്രാക്ക് ചെയ്യാൻ കഴിയില്ല
compose-track-sign-in = തുറക്കലും ക്ലിക്കുകളും ട്രാക്ക് ചെയ്യാൻ ഒരു Katna അക്കൗണ്ടിൽ സൈൻ ഇൻ ചെയ്യുക
compose-receipt = വായിച്ചതിന്റെ രസീത് അഭ്യർത്ഥിക്കുക
compose-receipt-on = വായിച്ചതിന്റെ രസീത് അഭ്യർത്ഥിച്ചു: അത് അയയ്ക്കാൻ സ്വീകർത്താവിന്റെ ആപ്പ് അവരോട് ചോദിച്ചേക്കാം

## Spelling

spell-no-dictionary = { $language } എന്നതിനുള്ള അക്ഷരത്തെറ്റ് നിഘണ്ടു ഇൻസ്റ്റാൾ ചെയ്തിട്ടില്ല (ഉദാഹരണത്തിന് hunspell-en_us).
spell-dictionary-error = അക്ഷരത്തെറ്റ് നിഘണ്ടു: { $error }

## Grammar checking (the right-click menu on a grammar mistake)

grammar-replace = “{ $words }”
grammar-add = “{ $words }” ചേർക്കുക
grammar-remove = “{ $words }” നീക്കം ചെയ്യുക
grammar-ignore = അവഗണിക്കുക

## Send checks (asked before a message goes out)

send-check-attachment-title = ഫയലുകൾ അറ്റാച്ച് ചെയ്യാൻ ഉദ്ദേശിച്ചിരുന്നോ?
send-check-attachment-text = നിങ്ങൾ ഒരു അറ്റാച്ച്‌മെന്റിനെക്കുറിച്ച് എഴുതി, പക്ഷേ ഒന്നും അറ്റാച്ച് ചെയ്തിട്ടില്ല.
send-check-attach = ഒരു ഫയൽ അറ്റാച്ച് ചെയ്യുക
send-check-subject-title = വിഷയമില്ലാതെ അയയ്ക്കണോ?
send-check-subject-text = ഈ സന്ദേശത്തിന് വിഷയമില്ല.
send-check-add-subject = വിഷയം ചേർക്കുക
send-check-send-anyway = എന്തായാലും അയയ്ക്കുക
recipient-not-valid = സാധുവായ ഇമെയിൽ വിലാസമല്ല
recipient-show-address = വിലാസം കാണിക്കുക
recipient-remove = നീക്കം ചെയ്യുക
recipient-bad-title = വിലാസം പരിശോധിക്കുക
recipient-bad-text = “{ $address }” സാധുവായ ഇമെയിൽ വിലാസമല്ല. അയയ്ക്കുന്നതിന് മുമ്പ് അത് ശരിയാക്കുകയോ നീക്കം ചെയ്യുകയോ ചെയ്യുക.
recipient-bad-fix = ശരിയാക്കുക
