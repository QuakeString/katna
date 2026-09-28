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
compose-show-trimmed = Ingekorte inhoud tonen
compose-hide-trimmed = Ingekorte inhoud verbergen
compose-remove-trimmed = Geciteerde tekst verwijderen
compose-trimmed-removed = Geciteerde tekst verwijderd

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
compose-draft-failed = Het concept kon niet worden opgeslagen: { $error }
compose-draft-not-opened = Het concept kon niet worden geopend.

## Attachments

compose-picker-insert = Invoegen
compose-picker-attach = Bijvoegen
compose-file-too-large = { $name } is te groot: een bericht kan maximaal { $limit } bevatten.
compose-attachment-size = ({ $size })
compose-remove-attachment = Bijlage verwijderen
compose-attachments-total = { $count ->
    [one] { $count } bestand, { $size }
   *[other] { $count } bestanden, { $size }
}
compose-drop-files = Zet bestanden hier neer
compose-drop-here = Hier neerzetten
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
compose-track = Openen en klikken volgen
compose-tracked = Gevolgd: je ziet wanneer elke ontvanger het opent of een link volgt
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
recipient-not-valid = Geen geldig e-mailadres
recipient-show-address = Adres tonen
recipient-remove = Verwijderen
recipient-bad-title = Controleer het adres
recipient-bad-text = ‘{ $address }’ is geen geldig e-mailadres. Verbeter of verwijder het voordat je verzendt.
recipient-bad-fix = Verbeteren
