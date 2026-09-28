# Katna Mail, Swedish (Svenska).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = Stäng
reader-back = Tillbaka
reader-mark-unread = Markera som oläst
reader-move-to = Flytta till
reader-more = Mer
reader-original-colors = Visa originalfärger
reader-dark-colors = Visa i mörka färger
reader-print-all = Skriv ut alla
reader-new-window = I nytt fönster
reader-position = { $position } av { $total }
reader-newer = Nyare
reader-older = Äldre

## Reading pane: the conversation

reader-removed = Konversationen har tagits bort.
reader-no-subject = (inget ämne)
reader-collapse-all = Komprimera alla
reader-expand-all = Expandera alla
reader-unknown-sender = (okänd avsändare)
reader-date-ago = { $date } ({ $ago })
reader-sending = Skickar…
reader-me = mig
reader-to = till { $names }
reader-to-label = till
reader-tick-delivered = Levererat { $when }
reader-tick-no-bounce = Skickat { $when }; ingen studs kom tillbaka, så det kom troligen fram
reader-tick-bounced = Inte levererat: studsade { $when }
reader-tick-read = Läst { $when } (läskvitto)
reader-tick-opened = Öppnat, senast { $when } (spårning av öppningar)
reader-starred = Stjärnmärkt
reader-not-starred = Inte stjärnmärkt
reader-too-long = Meddelandet är för långt för att visas i sin helhet.
reader-encrypted-images = Bilder från webben läses aldrig in i krypterad e-post.
reader-window-failed = Det gick inte att öppna ett nytt fönster.

## Reading pane: message details (opened from "to me")

reader-details-from = från:
reader-details-to = till:
reader-details-cc = kopia:
reader-details-date = datum:
reader-details-subject = ämne:

## Reading pane: downloading a message

reader-downloading = Hämtar meddelandet från servern…
reader-download-failed = Det gick inte att hämta meddelandet.
reader-try-again = Försök igen

## Reply row

reply-reply = Svara
reply-reply-all = Svara alla
reply-forward = Vidarebefordra

## Encrypted and signed mail

security-decrypting = Dekrypterar…
security-checking = Kontrollerar signaturen…
security-partly-encrypted = Endast en del av meddelandet är krypterad. Resten lades till utanför skyddet och kan komma från vem som helst.
security-partly-signed = Endast en del av meddelandet är signerad. Resten lades till utanför skyddet och kan komma från vem som helst.
security-encrypted = Krypterat meddelande
security-encrypted-smime = Krypterat meddelande (S/MIME)
security-no-key = Det går inte att dekryptera meddelandet: det krypterades för en nyckel som du inte har.
security-cancelled = Dekrypteringen avbröts.
security-damaged = Det går inte att dekryptera meddelandet: krypterade data är skadade eller har ändrats.
security-decrypt-unavailable = Det går inte att dekryptera meddelandet: installera { $tool } för att läsa krypterad e-post.
security-decrypt-failed = Det går inte att dekryptera meddelandet: { $reason }
security-unknown-signer = en okänd signerare
security-signed-verified = Signerat av { $signer } · verifierat
security-signed-not-sender = Signerat av { $signer }, som inte är avsändaren
security-signed-untrusted = Signerat av { $signer }, med en nyckel som du har markerat som ej betrodd
security-signed-unverified = Signerat av { $signer } · nyckeln är inte verifierad
security-bad-signature = Ogiltig signatur: meddelandet ändrades efter att det signerades, eller så är signaturen förfalskad.
security-signature-expired = Signerat av { $signer } · signaturen har gått ut
security-key-expired = Signerat av { $signer } · nyckeln har gått ut sedan dess
security-key-revoked = Signerat av { $signer } med en nyckel som har återkallats
security-missing-key = Signerat med en nyckel som du inte har, så det kan inte kontrolleras
security-missing-key-id = Signerat med en nyckel som du inte har ({ $key }), så det kan inte kontrolleras
security-signature-unavailable = Signerat; installera { $tool } för att kontrollera signaturen
security-signature-error = Signaturen kunde inte kontrolleras.
tracking-opened = { $who } öppnade det { $count ->
    [one] en gång
   *[other] { $count } gånger
}, senast { $when }
tracking-opens-clicks = { $who } öppnade det { $opens ->
    [one] en gång
   *[other] { $opens } gånger
} och följde en länk { $clicks ->
    [one] en gång
   *[other] { $clicks } gånger
}, senast { $when }
tracking-clicked = { $who } följde en länk { $clicks ->
    [one] en gång
   *[other] { $clicks } gånger
}, senast { $when }
tracking-maybe-opened = { $who } kan ha öppnat det (Apple Mail laddar bilder för att skydda integriteten)
tracking-seen-none = Ingen har öppnat det eller följt en länk än
tracking-receipt = { $who } skickade ett läskvitto
tracking-receipt-displayed = Läskvitto: { $who } öppnade ditt meddelande
tracking-receipt-other = Läskvitto: { $who } raderade eller hanterade ditt meddelande utan att öppna det

## Remote images and pictures

remote-hidden = Bilder i det här meddelandet är dolda.
remote-hidden-unconfirmed = Bilder är dolda: avsändaren kunde inte bekräftas.
remote-show = Visa bilder
remote-always-show = Visa alltid från den här avsändaren
remote-picture-use = Använd
remote-picture-too-big = Välj en bild på högst 8 MB.
remote-picture-type = Välj en PNG-, JPEG-, GIF-, WebP- eller SVG-bild.
remote-picture-read-failed = Det går inte att läsa bilden: { $error }
remote-picture-keep-failed = Det går inte att spara bilden: { $error }
remote-picture-remove-failed = Det går inte att ta bort bilden: { $error }

## Attachments

attachment-count = { $count ->
    [one] En bilaga
   *[other] { $count } bilagor
}
attachment-save = Spara
attachment-save-all = Spara alla
attachment-save-all-tooltip = Spara alla bilagor i en mapp
attachment-save-here = Spara här
attachment-not-downloaded = Meddelandet är inte hämtat.
attachment-not-found = Bilagan hittades inte i meddelandet.
attachment-read-failed = Det gick inte att läsa { $name }
attachment-numbered = bilaga { $number }
attachment-saved-all = { $count ->
    [one] { $count } fil sparades i { $place }
   *[other] { $count } filer sparades i { $place }
}
attachment-saved-some = { $total ->
    [one] { $saved } av { $total } fil sparades i { $place }. Det gick inte att spara { $failed }
   *[other] { $saved } av { $total } filer sparades i { $place }. Det gick inte att spara { $failed }
}
attachment-saved-to = Sparad i { $path }
attachment-save-failed = Det gick inte att spara { $name }: { $error }
attachment-open-failed = Det gick inte att öppna { $name }: { $error }
attachment-risky = Filen kan köra ett program, så Katna öppnar den inte. Spara den i stället.
attachment-encrypted-open = Filen kom krypterad. Spara den för att öppna den någon annanstans.

## Printing

print-failed = Det gick inte att skriva ut: { $error }
print-no-font = inget typsnitt hittades
print-opened-as-pdf = Öppnades som PDF för utskrift därifrån.
print-preview-title = Förhandsgranskning
print-preview-laying-out = Ordnar sidorna…
print-preview-pages = { $count ->
    [one] { $count } sida
   *[other] { $count } sidor
}
print-preview-more = { $count ->
    [one] och { $count } sida till
   *[other] och { $count } sidor till
}
print-preview-failed = sidorna kunde inte visas
print-preview-paper = Papper
print-preview-a4 = A4
print-preview-letter = Letter
print-preview-layout = Layout
print-preview-as-shown = Som den visas
print-preview-simple = Endast text
print-preview-backgrounds = Bakgrunder
print-preview-cancel = Avbryt
print-preview-print = Skriv ut
print-not-downloaded = (Inte hämtat än.)
print-encrypted = (Krypterat. Öppna det i Katna Mail för att skriva ut texten.)
print-to = Till: { $addresses }
print-cc = Kopia: { $addresses }

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = Öppna meddelandet för att läsa bilagorna.
text-copy = Kopiera
text-select-all = Markera allt
