# Katna Mail, Swedish (Svenska).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = Stäng
reader-back = Tillbaka
reader-mark-unread = Markera som oläst
reader-move-to = Flytta till
reader-snooze = Snooza
reader-remind = Påminn mig
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
reader-chip-remove = Ta bort { $label }
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
reader-download-failed-reason = Det gick inte att hämta det här meddelandet. { $reason }
reader-download-offline = Det här kontot är offline. Gå online för att hämta meddelandet.
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
security-look-up-key = Slå upp nyckeln

## The key popover: the details of the key a message was signed with, and
## a key to import (looked up, or attached to the message)

key-card-verified = Verifierad signatur
key-card-verified-detail = Signaturen är giltig och du litar på den här nyckeln.
key-card-unverified = Signaturen är inte verifierad
key-card-unverified-detail = Signaturen är giltig, men inget bekräftar att nyckeln är deras. Jämför fingeravtrycket med personen och lita sedan på nyckeln i GnuPG (Kleopatra eller gpg --edit-key).
key-card-not-sender = Signerat av någon annan
key-card-not-sender-detail = Signaturen är giltig, men nyckeln tillhör inte avsändaren.
key-card-untrusted = Nyckeln är inte betrodd
key-card-untrusted-detail = Du har markerat den här nyckeln som ej betrodd i GnuPG.
key-card-signature-expired = Signaturen har gått ut
key-card-signature-expired-detail = Signaturen var giltig, men den har gått ut.
key-card-key-expired = Nyckeln har gått ut
key-card-key-expired-detail = Signaturen är giltig, men nyckeln har gått ut sedan dess.
key-card-key-revoked = Nyckeln har återkallats
key-card-key-revoked-detail = Ägaren har återkallat den här nyckeln, så signaturen går inte att lita på.
key-card-bad = Ogiltig signatur
key-card-bad-detail = Meddelandet ändrades efter att det signerades, eller så är signaturen förfalskad.
key-card-signed-by = Signerat av
key-card-belongs-to = Tillhör
key-card-fingerprint = Fingeravtryck
key-card-signed = Signerat
key-card-key = Nyckel
key-card-kind = { $standard }, { $algorithm }
key-card-created = Skapad
key-card-expires = Går ut
key-card-never = Aldrig
key-card-issued-by = Utfärdat av
key-card-found-in = Hittad i
key-card-keyring = Din GnuPG-nyckelring
key-card-copy = Kopiera fingeravtryck
key-card-import-title = Importera den här nyckeln?
key-card-from-directory = Hittad i nyckelkatalogen för { $domain }.
key-card-from-attachment = Från bilagan { $name }.
key-card-import-note = Katna kan sedan kontrollera den här personens signaturer och kryptera e-post till hen. Jämför fingeravtrycket med personen för att lita fullt ut på nyckeln.
key-card-cancel = Avbryt
key-card-import = Importera nyckel
key-card-looking-up = Slår upp nyckeln…
key-card-looking-up-detail = Frågar nyckelkatalogen för { $domain }.
key-card-not-found = Ingen nyckel hittades
key-card-not-found-detail = { $domain } publicerar ingen nyckel för den här adressen. Be avsändaren att skicka sin nyckel till dig.
key-card-not-kept = Nyckeln som hittades kan inte användas.
key-card-failed = Det gick inte att hämta nyckeln

## Sender checks: the banner on mail that failed the checks the user's mail
## provider ran on its sender (DMARC, DKIM, SPF), and the "?" on the picture
## of a sender nothing confirmed

sender-failed-title = Det här kanske inte kommer från { $domain }
sender-failed-body = Det klarade inte avsändarkontrollerna hos { $provider }. Var försiktig med länkar, bilagor och svar.
sender-provider-unknown = din e-postleverantör
sender-details = Detaljer
sender-details-hide = Dölj detaljer
sender-looks-safe = Ser säkert ut
sender-move-to-spam = Flytta till skräppost
sender-checked-by = Kontrollerat av { $provider }
sender-checked-by-server = Kontrollerat av { $provider } ({ $server })
sender-dmarc = Avsändardomän (DMARC)
sender-dkim = Signatur (DKIM)
sender-spf = Avsändande server (SPF)
sender-result-pass = Godkänd
sender-result-fail = Underkänd
sender-result-unsure = Osäkert
sender-result-none = Ingen
sender-result-missing = Inte kontrollerad
sender-dmarc-pass = { $domain } bekräftar den här avsändaren.
sender-dmarc-fail = E-posten stämmer inte med hur { $domain } anger att dess e-post skickas.
sender-dmarc-none = { $domain } publicerar inga regler för sin e-post.
sender-dkim-pass = Signerat av { $domain }.
sender-dkim-fail = Signaturen från { $domain } stämmer inte med e-posten.
sender-dkim-none = Meddelandet var inte signerat.
sender-spf-pass = Skickat från en server som { $domain } listar.
sender-spf-fail = Skickat från en server som { $domain } inte listar.
sender-spf-none = { $domain } listar inte sina servrar.
sender-check-unsure = Kontrollen kunde inte ge ett tydligt svar.
sender-unconfirmed = { $provider } kunde inte bekräfta att det här kommer från { $domain }. Vem som helst kan ange vilken avsändare som helst.
sender-link-title = Öppna den här länken?
sender-link-body = Den här e-posten klarade inte avsändarkontrollerna. Länken går till { $host }:
sender-link-cancel = Avbryt
sender-link-open = Öppna

## Open and click tracking and read receipts (the eye's popover beside a
## sent message's star, and the line above a read receipt)

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
tracking-receipt-read = { $who } läste det (läskvitto), { $when }
tracking-receipt-displayed = Läskvitto: { $who } öppnade ditt meddelande
tracking-receipt-other = Läskvitto: { $who } raderade eller hanterade ditt meddelande utan att öppna det

## Remote images and pictures

remote-hidden = Bilder i det här meddelandet är dolda.
remote-hidden-unconfirmed = Bilder är dolda: avsändaren kunde inte bekräftas.
remote-hidden-failed = Bilder är dolda: den här e-posten klarade inte avsändarkontrollerna.
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
attachment-forward = Vidarebefordra
attachment-save-all = Spara alla
attachment-save-all-tooltip = Spara alla bilagor i en mapp
attachment-save-here = Spara här
attachment-not-downloaded = Meddelandet är inte hämtat.
attachment-open-message = Öppna meddelandet för att läsa bilagorna.
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

## Message text (right-click menu in the reading pane)

text-pin = Fäst högst upp
text-copy-address = Kopiera adress
text-copy = Kopiera
text-select-all = Markera allt
