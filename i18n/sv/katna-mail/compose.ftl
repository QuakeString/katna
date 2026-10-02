# Katna Mail, Swedish (Svenska).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Compose window: title bar

compose-new-message = Nytt meddelande
compose-restore = Återställ
compose-minimize = Minimera
compose-exit-full-screen = Avsluta helskärm
compose-open-window = Öppna i ett nytt fönster
compose-save-close = Spara och stäng
compose-back-to-mail = Tillbaka till e-postfönstret
compose-pop-out-reply = Öppna svaret i eget fönster
compose-edit-recipients = Redigera mottagare
compose-summary-cc = Kopia: { $names }
compose-summary-bcc = Dold kopia: { $names }
compose-more-recipients = { $count } till
compose-show-trimmed = Visa förkortat innehåll
compose-hide-trimmed = Dölj förkortat innehåll
compose-remove-trimmed = Ta bort citerad text
compose-trimmed-removed = Den citerade texten har tagits bort

## Recipients and subject

compose-to = Till
compose-cc = Kopia
compose-bcc = Dold kopia
compose-from = Från
compose-from-choose = Skicka från ett annat konto
compose-recipients = Mottagare
compose-subject = Ämne

## Sending (the notes at the bottom of the window)

compose-open-elsewhere = Skicka eller släng det öppna meddelandet först.
compose-bad-address = ”{ $address }” är ingen e-postadress.
compose-no-recipients = Lägg till minst en mottagare.
compose-attachments-too-large = Bilagorna är { $size }; e-postservrar tar emot upp till { $limit }.
compose-no-account = Lägg till ett konto att skicka e-post från.
compose-past-time = Välj en tidpunkt i framtiden.
compose-scheduling = Schemalägger…
compose-sending = Skickar…
compose-scheduled = Schemalagt att skickas { $when }
compose-sent-archived = Skickat och arkiverat
compose-sent = Meddelandet har skickats
compose-discarded = Utkastet har slängts
compose-draft-saved = Utkastet har sparats
compose-draft-saving = Sparar…
compose-draft-failed = Utkastet kunde inte sparas: { $error }
compose-draft-not-opened = Utkastet kunde inte öppnas.

## Attachments

compose-picker-insert = Infoga
compose-picker-attach = Bifoga
compose-file-too-large = { $name } är för stor: ett meddelande kan innehålla upp till { $limit }.
compose-forward-files-missing = Det vidarebefordrade meddelandets filer är inte hämtade, så de bifogas inte.
compose-attachment-size = ({ $size })
compose-remove-attachment = Ta bort bilaga
compose-attachment-open-tip = Öppna för att kontrollera den
compose-attachments-total = { $count ->
    [one] { $count } fil, { $size }
   *[other] { $count } filer, { $size }
}
compose-drive-note = { $name } är större än { $limit }, så den läggs i din Google Drive och meddelandet innehåller en länk.
compose-drive-tip = I din Google Drive; meddelandet innehåller en länk
compose-drive-uploading = Laddar upp { $percent } %
compose-drive-allow = Tillåt Drive
compose-drive-allow-tip = Logga in med Google igen så att Katna kan lägga stora filer i din Drive
compose-drive-retry = Försök igen
compose-drive-sends-when-uploaded = Skickas när { $name } har laddats upp
compose-drive-not-uploaded = { $name } finns inte i Google Drive än
compose-drive-share-failed = Det gick inte att dela filerna i Google Drive: { $error }
compose-drive-share-title = Dela filerna med alla?
compose-drive-share-text = { $count ->
    [one] Google Drive kan inte dela filerna med { $addresses }, som inte har något Google-konto. Alla som har länken kan i stället öppna dem.
   *[other] Google Drive kan inte dela filerna med { $addresses }, som inte har något Google-konto. Alla som har länken kan i stället öppna dem.
}
compose-drive-share-link = Dela med länk
compose-drive-send-without = Skicka utan att dela
compose-drive-share-cancel = Avbryt
compose-drive-card-detail = { $size } · Google Drive
compose-drive-card-name = Google Drive
compose-onedrive-note = { $name } är större än { $limit }, så den läggs i din OneDrive och meddelandet innehåller en länk.
compose-onedrive-tip = I din OneDrive; meddelandet innehåller en länk
compose-onedrive-allow = Tillåt OneDrive
compose-onedrive-allow-tip = Logga in med Microsoft igen så att Katna kan lägga stora filer i din OneDrive
compose-onedrive-not-uploaded = { $name } finns inte i OneDrive än
compose-onedrive-share-failed = Det gick inte att dela filerna i OneDrive: { $error }
compose-onedrive-share-text = { $count ->
    [one] OneDrive kan inte dela filerna med { $addresses }. Alla som har länken kan i stället öppna dem.
   *[other] OneDrive kan inte dela filerna med { $addresses }. Alla som har länken kan i stället öppna dem.
}
compose-onedrive-card-detail = { $size } · OneDrive
compose-onedrive-card-name = OneDrive
compose-drop-files = Släpp filer här
compose-drop-here = Släpp här

## Paste options (a small bar under what was just pasted or dropped)

compose-paste-keep-formatting = Behåll formatering
compose-paste-table = Tabell
compose-paste-picture = Bild
compose-paste-plain-text = Oformaterad text
compose-paste-inline = I texten
compose-paste-attachment = Bilaga

## Encryption and signing (the toggles by the recipients)

compose-encrypt = Kryptera
compose-encrypted = Krypterat: bara mottagarna kan läsa det
compose-sign = Signera
compose-signed = Signerat: mottagarna kan kontrollera att det kommer från dig

## Open and click tracking and read receipts (toggles after Sign)

compose-track = Spåra öppningar och klick
compose-tracked = Spårat: du ser när varje mottagare öppnar det eller följer en länk
compose-track-clicks = Spåra länkklick (oformaterad text kan inte visa öppningar)
compose-tracked-clicks = Spårat: du ser när varje mottagare följer en länk
compose-track-sign-in = Logga in på ett Katna-konto för att spåra öppningar och klick
compose-receipt = Begär läskvitto
compose-receipt-on = Läskvitto begärt: mottagarens app kan be hen att skicka ett
compose-delivery = Begär leveranskvitto
compose-delivery-on = Leveranskvitto begärt: din e-postserver mejlar dig när varje mottagares server tar emot meddelandet
compose-delivery-unavailable = Din e-postserver skickar inte leveranskvitton

## Spelling

spell-no-dictionary = Ingen stavningsordlista för { $language } är installerad (till exempel hunspell-en_us).
spell-dictionary-error = Stavningsordlista: { $error }

## Grammar checking (the right-click menu on a grammar mistake)

grammar-replace = ”{ $words }”
grammar-add = Lägg till ”{ $words }”
grammar-remove = Ta bort ”{ $words }”
grammar-ignore = Ignorera

## Send checks (asked before a message goes out)

send-check-attachment-title = Tänkte du bifoga filer?
send-check-attachment-text = Du skrev om en bilaga, men inget är bifogat.
send-check-attach = Bifoga en fil
send-check-subject-title = Skicka utan ämne?
send-check-subject-text = Det här meddelandet har inget ämne.
send-check-add-subject = Lägg till ämne
send-check-send-anyway = Skicka ändå

## Recipients (To, Cc and Bcc)

recipient-not-valid = Ingen giltig e-postadress
recipient-show-address = Visa adress
recipient-remove = Ta bort
recipient-bad-title = Kontrollera adressen
recipient-bad-text = ”{ $address }” är ingen giltig e-postadress. Rätta eller ta bort den innan du skickar.
recipient-bad-fix = Rätta
