# Katna Mail, Afrikaans (Afrikaans).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Compose window: title bar

compose-new-message = Nuwe boodskap
compose-restore = Herstel
compose-minimize = Minimeer
compose-exit-full-screen = Verlaat volskerm
compose-open-window = Maak in 'n nuwe venster oop
compose-save-close = Stoor en maak toe
compose-back-to-mail = Terug na die posvenster
compose-pop-out-reply = Maak antwoord apart oop
compose-edit-recipients = Wysig ontvangers
compose-summary-cc = Cc: { $names }
compose-summary-bcc = Bcc: { $names }
compose-more-recipients = { $count } meer
compose-show-trimmed = Wys verkorte inhoud
compose-hide-trimmed = Versteek verkorte inhoud
compose-remove-trimmed = Verwyder aangehaalde teks
compose-trimmed-removed = Aangehaalde teks verwyder

## Recipients and subject

compose-to = Aan
compose-cc = Cc
compose-bcc = Bcc
compose-from = Van
compose-from-choose = Stuur van 'n ander rekening
compose-recipients = Ontvangers
compose-subject = Onderwerp

## Sending (the notes at the bottom of the window)

compose-open-elsewhere = Stuur of gooi eers die oop boodskap weg.
compose-bad-address = “{ $address }” is nie 'n e-posadres nie.
compose-no-recipients = Voeg ten minste een ontvanger by.
compose-attachments-too-large = Die aanhegsels is { $size }; posbedieners neem tot { $limit }.
compose-no-account = Voeg 'n rekening by om pos van te stuur.
compose-past-time = Kies 'n tyd in die toekoms.
compose-scheduling = Skeduleer tans…
compose-sending = Stuur tans…
compose-scheduled = Stuur geskeduleer vir { $when }
compose-sent-archived = Gestuur en geargiveer
compose-sent = Boodskap gestuur
compose-discarded = Konsep weggegooi
compose-draft-saved = Konsep gestoor
compose-draft-failed = Die konsep kon nie gestoor word nie: { $error }
compose-draft-not-opened = Die konsep kon nie oopgemaak word nie.

## Attachments

compose-picker-insert = Voeg in
compose-picker-attach = Heg aan
compose-file-too-large = { $name } is te groot: 'n boodskap kan tot { $limit } dra.
compose-attachment-size = ({ $size })
compose-remove-attachment = Verwyder aanhegsel
compose-attachments-total = { $count ->
    [one] { $count } lêer, { $size }
   *[other] { $count } lêers, { $size }
}
compose-drive-note = { $name } is meer as { $limit }, so dit gaan na jou Google Drive en die boodskap bevat 'n skakel.
compose-drive-tip = In jou Google Drive; die boodskap bevat 'n skakel
compose-drive-uploading = Laai tans op { $percent }%
compose-drive-allow = Laat Drive toe
compose-drive-allow-tip = Meld weer met Google aan sodat Katna groot lêers in jou Drive kan plaas
compose-drive-retry = Probeer weer
compose-drive-sends-when-uploaded = Stuur sodra { $name } opgelaai is
compose-drive-not-uploaded = { $name } is nog nie in Google Drive nie
compose-drive-share-failed = Kon nie die lêers in Google Drive deel nie: { $error }
compose-drive-share-title = Deel die lêers met almal?
compose-drive-share-text = { $count ->
    [one] Google Drive kan nie die lêers met { $addresses } deel nie, wat nie 'n Google-rekening het nie. Enigeen met die skakel kan dit eerder oopmaak.
   *[other] Google Drive kan nie die lêers met { $addresses } deel nie, wat nie Google-rekeninge het nie. Enigeen met die skakel kan dit eerder oopmaak.
}
compose-drive-share-link = Deel met skakel
compose-drive-send-without = Stuur sonder om te deel
compose-drive-share-cancel = Kanselleer
compose-drive-card-detail = { $size } · Google Drive
compose-drop-files = Los lêers hier
compose-drop-here = Los hier
compose-paste-keep-formatting = Behou formatering
compose-paste-table = Tabel
compose-paste-picture = Prent
compose-paste-plain-text = Gewone teks
compose-paste-inline = In die teks
compose-paste-attachment = Aanhegsel

## Encryption and signing (the toggles by the recipients)

compose-encrypt = Enkripteer
compose-encrypted = Geënkripteer: net die ontvangers kan dit lees
compose-sign = Onderteken
compose-signed = Onderteken: ontvangers kan nagaan dat dit van jou af kom
compose-track = Spoor oopmaak en klikke na
compose-tracked = Nagespoor: jy sien wanneer elke ontvanger dit oopmaak of 'n skakel volg
compose-track-clicks = Spoor skakelklikke na (gewone teks kan nie oopmaak wys nie)
compose-tracked-clicks = Nagespoor: jy sien wanneer elke ontvanger 'n skakel volg
compose-track-sign-in = Meld by 'n Katna-rekening aan om oopmaak en klikke na te spoor
compose-receipt = Vra 'n leesbewys
compose-receipt-on = Leesbewys gevra: die ontvanger se program vra hulle dalk om een te stuur
compose-delivery = Vra 'n afleweringsbewys
compose-delivery-on = Afleweringsbewys gevra: jou e-posbediener stuur vir jou 'n e-pos wanneer elke ontvanger se bediener dit aanvaar
compose-delivery-unavailable = Jou e-posbediener stuur nie afleweringsbewyse nie

## Spelling

spell-no-dictionary = Geen speltoetswoordeboek vir { $language } is geïnstalleer nie (byvoorbeeld hunspell-en_us).
spell-dictionary-error = Speltoetswoordeboek: { $error }

## Grammar checking (the right-click menu on a grammar mistake)

grammar-replace = “{ $words }”
grammar-add = Voeg “{ $words }” by
grammar-remove = Verwyder “{ $words }”
grammar-ignore = Ignoreer

## Send checks (asked before a message goes out)

send-check-attachment-title = Wou jy lêers aanheg?
send-check-attachment-text = Jy het oor 'n aanhegsel geskryf, maar niks is aangeheg nie.
send-check-attach = Heg 'n lêer aan
send-check-subject-title = Stuur sonder onderwerp?
send-check-subject-text = Hierdie boodskap het geen onderwerp nie.
send-check-add-subject = Voeg onderwerp by
send-check-send-anyway = Stuur in elk geval
recipient-not-valid = Nie 'n geldige e-posadres nie
recipient-show-address = Wys adres
recipient-remove = Verwyder
recipient-bad-title = Gaan die adres na
recipient-bad-text = “{ $address }” is nie 'n geldige e-posadres nie. Maak dit reg of verwyder dit voor jy stuur.
recipient-bad-fix = Maak reg
