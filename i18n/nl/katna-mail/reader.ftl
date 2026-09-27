# Katna Mail, Dutch (Nederlands).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = Sluiten
reader-back = Terug
reader-mark-unread = Markeren als ongelezen
reader-move-to = Verplaatsen naar
reader-more = Meer
reader-print-all = Alles afdrukken
reader-new-window = In nieuw venster
reader-position = { $position } van { $total }
reader-newer = Nieuwer
reader-older = Ouder

## Reading pane: the conversation

reader-removed = Dit gesprek is verwijderd.
reader-no-subject = (geen onderwerp)
reader-collapse-all = Alles samenvouwen
reader-expand-all = Alles uitvouwen
reader-unknown-sender = (onbekende afzender)
reader-date-ago = { $date } ({ $ago })
reader-me = mij
reader-to = aan { $names }
reader-starred = Met ster
reader-not-starred = Zonder ster
reader-too-long = Het bericht is te lang om volledig te tonen.
reader-encrypted-images = Afbeeldingen van internet worden nooit geladen in versleutelde e-mail.
reader-window-failed = Kan geen nieuw venster openen.

## Reading pane: message details (opened from "to me")

reader-details-from = van:
reader-details-to = aan:
reader-details-cc = cc:
reader-details-date = datum:
reader-details-subject = onderwerp:

## Reading pane: downloading a message

reader-downloading = Dit bericht downloaden van de server…
reader-download-failed = Kan dit bericht niet downloaden.
reader-try-again = Opnieuw proberen

## Reply row

reply-reply = Beantwoorden
reply-reply-all = Allen beantwoorden
reply-forward = Doorsturen

## Encrypted and signed mail

security-decrypting = Ontsleutelen…
security-checking = Handtekening controleren…
security-partly-encrypted = Slechts een deel van dit bericht is versleuteld. De rest is buiten de beveiliging toegevoegd en kan van iedereen afkomstig zijn.
security-partly-signed = Slechts een deel van dit bericht is ondertekend. De rest is buiten de beveiliging toegevoegd en kan van iedereen afkomstig zijn.
security-encrypted = Versleuteld bericht
security-encrypted-smime = Versleuteld bericht (S/MIME)
security-no-key = Kan dit bericht niet ontsleutelen: het is versleuteld voor een sleutel die je niet hebt.
security-cancelled = Ontsleutelen is geannuleerd.
security-damaged = Kan dit bericht niet ontsleutelen: de versleutelde gegevens zijn beschadigd of gewijzigd.
security-decrypt-unavailable = Kan dit bericht niet ontsleutelen: installeer { $tool } om versleutelde e-mail te lezen.
security-decrypt-failed = Kan dit bericht niet ontsleutelen: { $reason }
security-unknown-signer = een onbekende ondertekenaar
security-signed-verified = Ondertekend door { $signer } · geverifieerd
security-signed-not-sender = Ondertekend door { $signer }, die niet de afzender is
security-signed-untrusted = Ondertekend door { $signer }, met een sleutel die je als niet vertrouwd hebt gemarkeerd
security-signed-unverified = Ondertekend door { $signer } · de sleutel is niet geverifieerd
security-bad-signature = Ongeldige handtekening: dit bericht is na ondertekening gewijzigd, of de handtekening is vervalst.
security-signature-expired = Ondertekend door { $signer } · de handtekening is verlopen
security-key-expired = Ondertekend door { $signer } · de sleutel is inmiddels verlopen
security-key-revoked = Ondertekend door { $signer } met een sleutel die is ingetrokken
security-missing-key = Ondertekend met een sleutel die je niet hebt, dus kan niet worden gecontroleerd
security-missing-key-id = Ondertekend met een sleutel die je niet hebt ({ $key }), dus kan niet worden gecontroleerd
security-signature-unavailable = Ondertekend; installeer { $tool } om de handtekening te controleren
security-signature-error = De handtekening kan niet worden gecontroleerd.
tracking-opened = { $who } heeft het { $count ->
    [one] één keer
   *[other] { $count } keer
} geopend, laatst { $when }
tracking-opened-clicked = { $who } heeft het geopend en een link gevolgd, { $count ->
    [one] één keer
   *[other] { $count } keer
}, laatst { $when }
tracking-maybe-opened = { $who } heeft het misschien geopend (Apple Mail laadt afbeeldingen voor privacy)
tracking-not-opened = { $who } heeft het nog niet geopend
tracking-receipt = { $who } heeft een leesbevestiging gestuurd
tracking-receipt-displayed = Leesbevestiging: { $who } heeft je bericht geopend
tracking-receipt-other = Leesbevestiging: { $who } heeft je bericht verwijderd of afgehandeld zonder het te openen

## Remote images and pictures

remote-hidden = Afbeeldingen in dit bericht zijn verborgen.
remote-show = Afbeeldingen tonen
remote-always-show = Altijd tonen van deze afzender
remote-picture-use = Gebruiken
remote-picture-too-big = Kies een afbeelding van maximaal 8 MB.
remote-picture-type = Kies een PNG-, JPEG-, GIF-, WebP- of SVG-afbeelding.
remote-picture-read-failed = Kan de afbeelding niet lezen: { $error }
remote-picture-keep-failed = Kan de afbeelding niet bewaren: { $error }
remote-picture-remove-failed = Kan de afbeelding niet verwijderen: { $error }

## Attachments

attachment-count = { $count ->
    [one] Eén bijlage
   *[other] { $count } bijlagen
}
attachment-save = Opslaan
attachment-save-all = Alles opslaan
attachment-save-all-tooltip = Alle bijlagen opslaan in een map
attachment-save-here = Hier opslaan
attachment-not-downloaded = Dit bericht is niet gedownload.
attachment-not-found = Deze bijlage is niet gevonden in het bericht.
attachment-read-failed = Kan { $name } niet lezen
attachment-numbered = bijlage { $number }
attachment-saved-all = { $count ->
    [one] { $count } bestand opgeslagen in { $place }
   *[other] { $count } bestanden opgeslagen in { $place }
}
attachment-saved-some = { $total ->
    [one] { $saved } van { $total } bestand opgeslagen in { $place }. Kan { $failed } niet opslaan
   *[other] { $saved } van { $total } bestanden opgeslagen in { $place }. Kan { $failed } niet opslaan
}
attachment-saved-to = Opgeslagen in { $path }
attachment-save-failed = Kan { $name } niet opslaan: { $error }
attachment-open-failed = Kan { $name } niet openen: { $error }
attachment-risky = Dit bestand kan een programma uitvoeren, dus Katna opent het niet. Sla het in plaats daarvan op.
attachment-encrypted-open = Dit bestand is versleuteld ontvangen. Sla het op om het elders te openen.

## Printing

print-failed = Kan niet afdrukken: { $error }
print-no-font = er is geen lettertype gevonden
print-opened-as-pdf = Geopend als pdf om vanaf daar af te drukken.
print-preview-title = Afdrukvoorbeeld
print-preview-laying-out = Pagina's opmaken…
print-preview-pages = { $count ->
    [one] { $count } pagina
   *[other] { $count } pagina's
}
print-preview-more = { $count ->
    [one] en nog { $count } pagina
   *[other] en nog { $count } pagina's
}
print-preview-failed = de pagina's konden niet worden getoond
print-preview-paper = Papier
print-preview-a4 = A4
print-preview-letter = Letter
print-preview-cancel = Annuleren
print-preview-print = Afdrukken
print-not-downloaded = (Nog niet gedownload.)
print-encrypted = (Versleuteld. Open het in Katna Mail om de tekst af te drukken.)
print-to = Aan: { $addresses }
print-cc = Cc: { $addresses }

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = Open dit bericht om de bijlagen te lezen.
text-copy = Kopiëren
text-select-all = Alles selecteren
