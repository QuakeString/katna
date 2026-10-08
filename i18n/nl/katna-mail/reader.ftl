# Katna Mail, Dutch (Nederlands).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = Sluiten
reader-back = Terug
reader-mark-unread = Markeren als ongelezen
reader-move-to = Verplaatsen naar
reader-snooze = Snoozen
reader-remind = Herinner mij
reader-more = Meer
reader-original-colors = Originele kleuren tonen
reader-dark-colors = In donkere kleuren tonen
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
reader-sending = Verzenden…
reader-me = mij
reader-to = aan { $names }
reader-to-label = aan
reader-tick-delivered = Afgeleverd { $when }
reader-tick-no-bounce = Verstuurd { $when }; er kwam geen bounce terug, dus het is vrijwel zeker aangekomen
reader-tick-bounced = Niet afgeleverd: teruggestuurd { $when }
reader-tick-read = Gelezen { $when } (leesbevestiging)
reader-tick-opened = Geopend, laatst { $when } (volgen van openen)
reader-starred = Met ster
reader-chip-remove = { $label } verwijderen
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
reader-download-failed-reason = Kan dit bericht niet downloaden. { $reason }
reader-download-offline = Dit account is offline. Ga online om dit bericht te downloaden.
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
security-look-up-key = Sleutel opzoeken

## The key popover: the details of the key a message was signed with, and
## a key to import (looked up, or attached to the message)

key-card-verified = Geverifieerde handtekening
key-card-verified-detail = De handtekening is geldig en je vertrouwt deze sleutel.
key-card-unverified = Handtekening niet geverifieerd
key-card-unverified-detail = De handtekening is geldig, maar niets bevestigt dat de sleutel van deze persoon is. Vergelijk de vingerafdruk met hem of haar en vertrouw de sleutel daarna in GnuPG (Kleopatra of gpg --edit-key).
key-card-not-sender = Ondertekend door iemand anders
key-card-not-sender-detail = De handtekening is geldig, maar de sleutel is niet van de afzender.
key-card-untrusted = Sleutel niet vertrouwd
key-card-untrusted-detail = Je hebt deze sleutel in GnuPG als niet vertrouwd gemarkeerd.
key-card-signature-expired = Handtekening verlopen
key-card-signature-expired-detail = De handtekening was geldig, maar is verlopen.
key-card-key-expired = Sleutel verlopen
key-card-key-expired-detail = De handtekening is geldig, maar de sleutel is inmiddels verlopen.
key-card-key-revoked = Sleutel ingetrokken
key-card-key-revoked-detail = De eigenaar heeft deze sleutel ingetrokken, dus de handtekening kan niet worden vertrouwd.
key-card-bad = Ongeldige handtekening
key-card-bad-detail = Dit bericht is na ondertekening gewijzigd, of de handtekening is vervalst.
key-card-signed-by = Ondertekend door
key-card-belongs-to = Hoort bij
key-card-fingerprint = Vingerafdruk
key-card-signed = Ondertekend
key-card-key = Sleutel
key-card-kind = { $standard }, { $algorithm }
key-card-created = Gemaakt
key-card-expires = Verloopt
key-card-never = Nooit
key-card-issued-by = Uitgegeven door
key-card-found-in = Gevonden in
key-card-keyring = Je GnuPG-sleutelbos
key-card-copy = Vingerafdruk kopiëren
key-card-import-title = Deze sleutel importeren?
key-card-from-directory = Gevonden in de sleutelmap van { $domain }.
key-card-from-attachment = Uit de bijlage { $name }.
key-card-import-note = Katna kan dan de handtekeningen van deze persoon controleren en versleutelde e-mail naar hem of haar sturen. Vergelijk de vingerafdruk met deze persoon om de sleutel volledig te vertrouwen.
key-card-cancel = Annuleren
key-card-import = Sleutel importeren
key-card-looking-up = Sleutel opzoeken…
key-card-looking-up-detail = De sleutelmap van { $domain } wordt gevraagd.
key-card-not-found = Geen sleutel gevonden
key-card-not-found-detail = { $domain } publiceert geen sleutel voor dit adres. Vraag de afzender je de zijne te sturen.
key-card-not-kept = De gevonden sleutel kan niet worden gebruikt.
key-card-failed = Kan de sleutel niet ophalen

## Sender checks: the banner on mail that failed the checks the user's mail
## provider ran on its sender (DMARC, DKIM, SPF), and the "?" on the picture
## of a sender nothing confirmed

sender-failed-title = Dit komt mogelijk niet van { $domain }
sender-failed-body = Het is niet door de afzendercontroles van { $provider } gekomen. Wees voorzichtig met links, bijlagen en antwoorden.
sender-provider-unknown = je e-mailprovider
sender-details = Details
sender-details-hide = Details verbergen
sender-looks-safe = Lijkt veilig
sender-move-to-spam = Naar spam verplaatsen
sender-checked-by = Gecontroleerd door { $provider }
sender-checked-by-server = Gecontroleerd door { $provider } ({ $server })
sender-dmarc = Afzenderdomein (DMARC)
sender-dkim = Handtekening (DKIM)
sender-spf = Verzendende server (SPF)
sender-result-pass = Geslaagd
sender-result-fail = Mislukt
sender-result-unsure = Onzeker
sender-result-none = Geen
sender-result-missing = Niet gecontroleerd
sender-dmarc-pass = { $domain } bevestigt deze afzender.
sender-dmarc-fail = De e-mail komt niet overeen met hoe { $domain } zegt dat zijn e-mail wordt verstuurd.
sender-dmarc-none = { $domain } publiceert geen regels voor zijn e-mail.
sender-dkim-pass = Ondertekend door { $domain }.
sender-dkim-fail = De handtekening van { $domain } komt niet overeen met de e-mail.
sender-dkim-none = Het bericht was niet ondertekend.
sender-spf-pass = Verstuurd vanaf een server die { $domain } vermeldt.
sender-spf-fail = Verstuurd vanaf een server die { $domain } niet vermeldt.
sender-spf-none = { $domain } vermeldt zijn servers niet.
sender-check-unsure = De controle gaf geen duidelijk antwoord.
sender-unconfirmed = { $provider } kon niet bevestigen dat dit van { $domain } komt. Iedereen kan elke afzender invullen.
sender-link-title = Deze link openen?
sender-link-body = Deze e-mail is niet door de afzendercontroles gekomen. De link gaat naar { $host }:
sender-link-cancel = Annuleren
sender-link-open = Openen

## Open and click tracking and read receipts (the eye's popover beside a
## sent message's star, and the line above a read receipt)

tracking-opened = { $who } heeft het { $count ->
    [one] één keer
   *[other] { $count } keer
} geopend, laatst { $when }
tracking-opens-clicks = { $who } heeft het { $opens ->
    [one] één keer
   *[other] { $opens } keer
} geopend en { $clicks ->
    [one] één keer
   *[other] { $clicks } keer
} een link gevolgd, laatst { $when }
tracking-clicked = { $who } heeft { $clicks ->
    [one] één keer
   *[other] { $clicks } keer
} een link gevolgd, laatst { $when }
tracking-maybe-opened = { $who } heeft het misschien geopend (Apple Mail laadt afbeeldingen voor privacy)
tracking-seen-none = Nog niemand heeft het geopend of een link gevolgd
tracking-receipt = { $who } heeft een leesbevestiging gestuurd
tracking-receipt-read = { $who } heeft het gelezen (leesbevestiging), { $when }
tracking-receipt-displayed = Leesbevestiging: { $who } heeft je bericht geopend
tracking-receipt-other = Leesbevestiging: { $who } heeft je bericht verwijderd of afgehandeld zonder het te openen

## Remote images and pictures

remote-hidden = Afbeeldingen in dit bericht zijn verborgen.
remote-hidden-unconfirmed = Afbeeldingen verborgen: de afzender kon niet worden bevestigd.
remote-hidden-failed = Afbeeldingen verborgen: deze e-mail is niet door de afzendercontroles gekomen.
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
attachment-forward = Doorsturen
attachment-save-all = Alles opslaan
attachment-save-all-tooltip = Alle bijlagen opslaan in een map
attachment-save-here = Hier opslaan
attachment-not-downloaded = Dit bericht is niet gedownload.
attachment-open-message = Open dit bericht om de bijlagen te lezen.
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
print-preview-layout = Opmaak
print-preview-as-shown = Zoals weergegeven
print-preview-simple = Alleen tekst
print-preview-backgrounds = Achtergronden
print-preview-cancel = Annuleren
print-preview-print = Afdrukken
print-not-downloaded = (Nog niet gedownload.)
print-encrypted = (Versleuteld. Open het in Katna Mail om de tekst af te drukken.)
print-to = Aan: { $addresses }
print-cc = Cc: { $addresses }

## Message text (right-click menu in the reading pane)

text-pin = Bovenaan vastzetten
text-copy-address = Adres kopiëren
text-copy = Kopiëren
text-select-all = Alles selecteren
