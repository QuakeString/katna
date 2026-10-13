# Katna Mail, Dutch (Nederlands).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Compose window: title bar

compose-new-message = Nieuw bericht
compose-restore = Herstellen
compose-minimize = Minimaliseren
compose-exit-full-screen = Volledig scherm sluiten
compose-open-window = Openen in een nieuw venster
compose-save-close = Opslaan en sluiten
compose-back-to-mail = Terug naar het e-mailvenster
compose-pop-out-reply = Antwoord in apart venster
compose-edit-recipients = Ontvangers bewerken
compose-summary-cc = Cc: { $names }
compose-summary-bcc = Bcc: { $names }
compose-more-recipients = { $count } meer
compose-show-trimmed = Ingekorte inhoud tonen
compose-hide-trimmed = Ingekorte inhoud verbergen
compose-remove-trimmed = Geciteerde tekst verwijderen
compose-trimmed-removed = Geciteerde tekst verwijderd

## The quoted or forwarded message, in the mail itself

compose-quote-header = Op { $date } schreef { $from }:
compose-forward-header = ---------- Doorgestuurd bericht ---------
compose-forward-from = Van: { $from }
compose-forward-date = Datum: { $date }
compose-forward-subject = Onderwerp: { $subject }
compose-forward-to = Aan: { $to }
compose-forward-cc = Cc: { $cc }

## Recipients and subject

compose-to = Aan
compose-cc = Cc
compose-bcc = Bcc
compose-from = Van
compose-from-choose = Verzenden vanaf een ander account
compose-recipients = Ontvangers
compose-subject = Onderwerp

## Sending (the notes at the bottom of the window)

compose-open-elsewhere = Verzend of verwijder eerst het geopende bericht.
compose-bad-address = ‘{ $address }’ is geen e-mailadres.
compose-no-recipients = Voeg minstens één ontvanger toe.
compose-attachments-too-large = De bijlagen zijn { $size }; mailservers accepteren tot { $limit }.
compose-no-account = Voeg een account toe om e-mail vanaf te verzenden.
compose-past-time = Kies een tijdstip in de toekomst.
compose-scheduling = Inplannen…
compose-sending = Verzenden…
compose-scheduled = Verzending gepland voor { $when }
compose-sent-archived = Verzonden en gearchiveerd
compose-sent = Bericht verzonden
compose-discarded = Concept verwijderd
compose-draft-saved = Concept opgeslagen
compose-draft-saving = Opslaan…
compose-draft-failed = Het concept kon niet worden opgeslagen: { $error }
compose-draft-not-opened = Het concept kon niet worden geopend.

## Attachments

compose-picker-insert = Invoegen
compose-picker-attach = Bijvoegen
compose-file-too-large = { $name } is te groot: een bericht kan maximaal { $limit } bevatten.
compose-forward-files-missing = De bestanden van het doorgestuurde bericht zijn niet gedownload, dus ze zijn niet bijgevoegd.
compose-attachment-size = ({ $size })
compose-remove-attachment = Bijlage verwijderen
compose-attachment-open-tip = Openen om te controleren
compose-attachments-total = { $count ->
    [one] { $count } bestand, { $size }
   *[other] { $count } bestanden, { $size }
}
compose-drive-note = { $name } is groter dan { $limit }, dus het bestand gaat naar je Google Drive en het bericht bevat een link.
compose-drive-tip = In je Google Drive; het bericht bevat een link
compose-drive-uploading = Uploaden { $percent }%
compose-drive-allow = Drive toestaan
compose-drive-allow-tip = Meld je opnieuw aan bij Google zodat Katna grote bestanden in je Drive kan zetten
compose-drive-retry = Opnieuw proberen
compose-drive-sends-when-uploaded = Wordt verzonden zodra { $name } is geüpload
compose-drive-not-uploaded = { $name } staat nog niet in Google Drive
compose-drive-share-failed = De bestanden in Google Drive konden niet worden gedeeld: { $error }
compose-drive-share-title = De bestanden met iedereen delen?
compose-drive-share-text = { $count ->
    [one] Google Drive kan de bestanden niet delen met { $addresses }, die geen Google-account heeft. In plaats daarvan kan iedereen met de link ze openen.
   *[other] Google Drive kan de bestanden niet delen met { $addresses }, die geen Google-account hebben. In plaats daarvan kan iedereen met de link ze openen.
}
compose-drive-share-link = Delen met link
compose-drive-send-without = Verzenden zonder te delen
compose-drive-share-cancel = Annuleren
compose-drive-card-detail = { $size } · Google Drive
compose-drive-card-name = Google Drive
compose-onedrive-note = { $name } is groter dan { $limit }, dus het bestand gaat naar je OneDrive en het bericht bevat een link.
compose-onedrive-tip = In je OneDrive; het bericht bevat een link
compose-onedrive-allow = OneDrive toestaan
compose-onedrive-allow-tip = Meld je opnieuw aan bij Microsoft zodat Katna grote bestanden in je OneDrive kan zetten
compose-onedrive-not-uploaded = { $name } staat nog niet in OneDrive
compose-onedrive-share-failed = De bestanden in OneDrive konden niet worden gedeeld: { $error }
compose-onedrive-share-text = { $count ->
    [one] OneDrive kan de bestanden niet delen met { $addresses }. In plaats daarvan kan iedereen met de link ze openen.
   *[other] OneDrive kan de bestanden niet delen met { $addresses }. In plaats daarvan kan iedereen met de link ze openen.
}
compose-onedrive-card-detail = { $size } · OneDrive
compose-onedrive-card-name = OneDrive
compose-drop-files = Zet bestanden hier neer
compose-drop-here = Hier neerzetten

## Paste options (a small bar under what was just pasted or dropped)

compose-paste-keep-formatting = Opmaak behouden
compose-paste-table = Tabel
compose-paste-picture = Afbeelding
compose-paste-plain-text = Platte tekst
compose-paste-inline = In de tekst
compose-paste-attachment = Bijlage

## Encryption and signing (the toggles by the recipients)

compose-encrypt = Versleutelen
compose-encrypted = Versleuteld: alleen de ontvangers kunnen het lezen
compose-sign = Ondertekenen
compose-signed = Ondertekend: ontvangers kunnen controleren dat het van jou komt

## Open and click tracking and read receipts (toggles after Sign)

compose-track = Openen en klikken volgen
compose-tracked = Gevolgd: je ziet wanneer elke ontvanger het opent of een link volgt
compose-track-clicks = Linkklikken volgen (platte tekst kan geen openingen tonen)
compose-tracked-clicks = Gevolgd: je ziet wanneer elke ontvanger een link volgt
compose-track-sign-in = Meld je aan bij een Katna-account om openen en klikken te volgen
compose-receipt = Leesbevestiging vragen
compose-receipt-on = Leesbevestiging gevraagd: de app van de ontvanger kan vragen er een te sturen
compose-delivery = Ontvangstbevestiging vragen
compose-delivery-on = Ontvangstbevestiging gevraagd: je mailserver mailt je wanneer de server van elke ontvanger het bericht accepteert
compose-delivery-unavailable = Je mailserver stuurt geen ontvangstbevestigingen

## Spelling

spell-no-dictionary = Er is geen spellingwoordenboek voor { $language } geïnstalleerd (bijvoorbeeld hunspell-en_us).
spell-dictionary-error = Spellingwoordenboek: { $error }

## Grammar checking (the right-click menu on a grammar mistake)

grammar-replace = ‘{ $words }’
grammar-add = ‘{ $words }’ toevoegen
grammar-remove = ‘{ $words }’ verwijderen
grammar-ignore = Negeren

## Send checks (asked before a message goes out)

send-check-attachment-title = Wilde je bestanden bijvoegen?
send-check-attachment-text = Je schreef over een bijlage, maar er is niets bijgevoegd.
send-check-attach = Bestand bijvoegen
send-check-subject-title = Verzenden zonder onderwerp?
send-check-subject-text = Dit bericht heeft geen onderwerp.
send-check-add-subject = Onderwerp toevoegen
send-check-send-anyway = Toch verzenden

## Recipients (To, Cc and Bcc)

recipient-not-valid = Geen geldig e-mailadres
recipient-show-address = Adres tonen
recipient-remove = Verwijderen
recipient-bad-title = Controleer het adres
recipient-bad-text = ‘{ $address }’ is geen geldig e-mailadres. Verbeter of verwijder het voordat je verzendt.
recipient-bad-fix = Verbeteren
