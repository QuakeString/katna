# Katna Mail, Afrikaans (Afrikaans).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = Maak toe
reader-back = Terug
reader-mark-unread = Merk as ongelees
reader-move-to = Skuif na
reader-snooze = Sluimer
reader-remind = Herinner my
reader-more = Meer
reader-original-colors = Wys oorspronklike kleure
reader-dark-colors = Wys in donker kleure
reader-print-all = Druk alles
reader-new-window = In nuwe venster
reader-position = { $position } van { $total }
reader-newer = Nuwer
reader-older = Ouer

## Reading pane: the conversation

reader-removed = Hierdie gesprek is verwyder.
reader-no-subject = (geen onderwerp)
reader-collapse-all = Vou almal in
reader-expand-all = Vou almal uit
reader-unknown-sender = (onbekende sender)
reader-date-ago = { $date } ({ $ago })
reader-sending = Stuur tans…
reader-me = my
reader-to = aan { $names }
reader-to-label = aan
reader-tick-delivered = Afgelewer { $when }
reader-tick-no-bounce = Gestuur { $when }; geen afleweringsfout het teruggekom nie, so dit het heel waarskynlik aangekom
reader-tick-bounced = Nie afgelewer nie: teruggestuur { $when }
reader-tick-read = Gelees { $when } (leesbewys)
reader-tick-opened = Oopgemaak, laas { $when } (oopmaaknasporing)
reader-starred = Gester
reader-chip-remove = Verwyder { $label }
reader-not-starred = Nie gester nie
reader-too-long = Die boodskap is te lank om volledig te wys.
reader-encrypted-images = Prente van die web word nooit in geënkripteerde e-pos gelaai nie.
reader-window-failed = Kon nie 'n nuwe venster oopmaak nie.

## Reading pane: message details (opened from "to me")

reader-details-from = van:
reader-details-to = aan:
reader-details-cc = afskrif:
reader-details-date = datum:
reader-details-subject = onderwerp:

## Reading pane: downloading a message

reader-downloading = Laai tans hierdie boodskap van die bediener af…
reader-download-failed = Kon nie hierdie boodskap aflaai nie.
reader-download-failed-reason = Kon nie hierdie boodskap aflaai nie. { $reason }
reader-download-offline = Hierdie rekening is vanlyn. Gaan aanlyn om hierdie boodskap af te laai.
reader-try-again = Probeer weer

## Reply row

reply-reply = Antwoord
reply-reply-all = Antwoord almal
reply-forward = Stuur aan

## Encrypted and signed mail

security-decrypting = Dekripteer tans…
security-checking = Kontroleer tans die handtekening…
security-partly-encrypted = Slegs 'n deel van hierdie boodskap is geënkripteer. Die res is buite die beskerming bygevoeg en kan van enigiemand kom.
security-partly-signed = Slegs 'n deel van hierdie boodskap is onderteken. Die res is buite die beskerming bygevoeg en kan van enigiemand kom.
security-encrypted = Geënkripteerde boodskap
security-encrypted-smime = Geënkripteerde boodskap (S/MIME)
security-no-key = Kan nie hierdie boodskap dekripteer nie: dit is geënkripteer vir 'n sleutel wat jy nie het nie.
security-cancelled = Dekriptering is gekanselleer.
security-damaged = Kan nie hierdie boodskap dekripteer nie: die geënkripteerde data is beskadig of is verander.
security-decrypt-unavailable = Kan nie hierdie boodskap dekripteer nie: installeer { $tool } om geënkripteerde e-pos te lees.
security-decrypt-failed = Kan nie hierdie boodskap dekripteer nie: { $reason }
security-unknown-signer = 'n onbekende ondertekenaar
security-signed-verified = Onderteken deur { $signer } · geverifieer
security-signed-not-sender = Onderteken deur { $signer }, wat nie die sender is nie
security-signed-untrusted = Onderteken deur { $signer }, met 'n sleutel wat jy as onbetroubaar gemerk het
security-signed-unverified = Onderteken deur { $signer } · die sleutel is nie geverifieer nie
security-bad-signature = Ongeldige handtekening: hierdie boodskap is verander nadat dit onderteken is, of die handtekening is vervals.
security-signature-expired = Onderteken deur { $signer } · die handtekening het verval
security-key-expired = Onderteken deur { $signer } · die sleutel het sedertdien verval
security-key-revoked = Onderteken deur { $signer } met 'n sleutel wat herroep is
security-missing-key = Onderteken met 'n sleutel wat jy nie het nie, dus kan dit nie gekontroleer word nie
security-missing-key-id = Onderteken met 'n sleutel wat jy nie het nie ({ $key }), dus kan dit nie gekontroleer word nie
security-signature-unavailable = Onderteken; installeer { $tool } om die handtekening te kontroleer
security-signature-error = Die handtekening kon nie gekontroleer word nie.
security-look-up-key = Soek sleutel op

## The key popover: the details of the key a message was signed with, and
## a key to import (looked up, or attached to the message)

key-card-verified = Geverifieerde handtekening
key-card-verified-detail = Die handtekening is geldig en jy vertrou hierdie sleutel.
key-card-unverified = Handtekening nie geverifieer nie
key-card-unverified-detail = Die handtekening is geldig, maar niks bevestig dat die sleutel hulle s'n is nie. Vergelyk die vingerafdruk met hulle en vertrou dan die sleutel in GnuPG (Kleopatra of gpg --edit-key).
key-card-not-sender = Deur iemand anders onderteken
key-card-not-sender-detail = Die handtekening is geldig, maar die sleutel is nie die sender s'n nie.
key-card-untrusted = Sleutel nie vertrou nie
key-card-untrusted-detail = Jy het hierdie sleutel in GnuPG as onbetroubaar gemerk.
key-card-signature-expired = Handtekening het verval
key-card-signature-expired-detail = Die handtekening was geldig, maar dit het verval.
key-card-key-expired = Sleutel het verval
key-card-key-expired-detail = Die handtekening is geldig, maar die sleutel het sedertdien verval.
key-card-key-revoked = Sleutel herroep
key-card-key-revoked-detail = Die eienaar het hierdie sleutel herroep, dus kan die handtekening nie vertrou word nie.
key-card-bad = Ongeldige handtekening
key-card-bad-detail = Hierdie boodskap is verander nadat dit onderteken is, of die handtekening is vervals.
key-card-signed-by = Onderteken deur
key-card-belongs-to = Behoort aan
key-card-fingerprint = Vingerafdruk
key-card-signed = Onderteken
key-card-key = Sleutel
key-card-kind = { $standard }, { $algorithm }
key-card-created = Geskep
key-card-expires = Verval
key-card-never = Nooit
key-card-issued-by = Uitgereik deur
key-card-found-in = Gevind in
key-card-keyring = Jou GnuPG-sleutelring
key-card-copy = Kopieer vingerafdruk
key-card-import-title = Voer hierdie sleutel in?
key-card-from-directory = Gevind in { $domain } se sleutelgids.
key-card-from-attachment = Uit die aanhegsel { $name }.
key-card-import-note = Katna kan dan hierdie persoon se handtekeninge kontroleer en e-pos aan hulle enkripteer. Om die sleutel ten volle te vertrou, vergelyk die vingerafdruk met hulle.
key-card-cancel = Kanselleer
key-card-import = Voer sleutel in
key-card-looking-up = Soek tans die sleutel op…
key-card-looking-up-detail = Vra tans { $domain } se sleutelgids.
key-card-not-found = Geen sleutel gevind nie
key-card-not-found-detail = { $domain } publiseer nie 'n sleutel vir hierdie adres nie. Vra die sender om vir jou hulle s'n te stuur.
key-card-not-kept = Die sleutel wat gevind is, kan nie gebruik word nie.
key-card-failed = Kon nie die sleutel kry nie

## Sender checks: the banner on mail that failed the checks the user's mail
## provider ran on its sender (DMARC, DKIM, SPF), and the "?" on the picture
## of a sender nothing confirmed

sender-failed-title = Dit is dalk nie van { $domain } nie
sender-failed-body = Dit het { $provider } se senderkontroles gedruip. Wees versigtig met skakels, aanhegsels en antwoorde.
sender-provider-unknown = jou e-posdiensverskaffer
sender-details = Besonderhede
sender-details-hide = Versteek besonderhede
sender-looks-safe = Lyk veilig
sender-move-to-spam = Skuif na strooipos
sender-checked-by = Nagegaan deur { $provider }
sender-checked-by-server = Nagegaan deur { $provider } ({ $server })
sender-dmarc = Senderdomein (DMARC)
sender-dkim = Handtekening (DKIM)
sender-spf = Stuurbediener (SPF)
sender-result-pass = Geslaag
sender-result-fail = Gedruip
sender-result-unsure = Nie seker nie
sender-result-none = Geen
sender-result-missing = Nie nagegaan nie
sender-dmarc-pass = { $domain } bevestig hierdie sender.
sender-dmarc-fail = Die e-pos stem nie ooreen met hoe { $domain } sê sy e-pos gestuur word nie.
sender-dmarc-none = { $domain } publiseer geen reëls vir sy e-pos nie.
sender-dkim-pass = Onderteken deur { $domain }.
sender-dkim-fail = Die handtekening van { $domain } stem nie met die e-pos ooreen nie.
sender-dkim-none = Die boodskap is nie onderteken nie.
sender-spf-pass = Gestuur van 'n bediener wat { $domain } lys.
sender-spf-fail = Gestuur van 'n bediener wat { $domain } nie lys nie.
sender-spf-none = { $domain } lys nie sy bedieners nie.
sender-check-unsure = Die kontrole kon nie 'n duidelike antwoord gee nie.
sender-unconfirmed = { $provider } kon nie bevestig dat dit van { $domain } af kom nie. Enigiemand kan enige sender skryf.
sender-link-title = Maak hierdie skakel oop?
sender-link-body = Hierdie e-pos het sy senderkontroles gedruip. Die skakel gaan na { $host }:
sender-link-cancel = Kanselleer
sender-link-open = Maak oop
tracking-opened = { $who } het dit { $count ->
    [one] een keer
   *[other] { $count } keer
} oopgemaak, laas { $when }
tracking-opens-clicks = { $who } het dit { $opens ->
    [one] een keer
   *[other] { $opens } keer
} oopgemaak en { $clicks ->
    [one] een keer
   *[other] { $clicks } keer
} 'n skakel gevolg, laas { $when }
tracking-clicked = { $who } het { $clicks ->
    [one] een keer
   *[other] { $clicks } keer
} 'n skakel gevolg, laas { $when }
tracking-maybe-opened = { $who } het dit dalk oopgemaak (Apple Mail laai prente vir privaatheid)
tracking-seen-none = Niemand het dit nog oopgemaak of 'n skakel gevolg nie
tracking-receipt = { $who } het 'n leesbewys gestuur
tracking-receipt-read = { $who } het dit gelees (leesbewys), { $when }
tracking-receipt-displayed = Leesbewys: { $who } het jou boodskap oopgemaak
tracking-receipt-other = Leesbewys: { $who } het jou boodskap uitgevee of hanteer sonder om dit oop te maak

## Remote images and pictures

remote-hidden = Prente in hierdie boodskap is versteek.
remote-hidden-unconfirmed = Prente is versteek: die sender kon nie bevestig word nie.
remote-hidden-failed = Prente is versteek: hierdie e-pos het sy senderkontroles gedruip.
remote-show = Wys prente
remote-always-show = Wys altyd van hierdie sender
remote-picture-use = Gebruik
remote-picture-too-big = Kies 'n prent van 8 MB of kleiner.
remote-picture-type = Kies 'n PNG-, JPEG-, GIF-, WebP- of SVG-prent.
remote-picture-read-failed = Kan nie die prent lees nie: { $error }
remote-picture-keep-failed = Kan nie die prent hou nie: { $error }
remote-picture-remove-failed = Kan nie die prent verwyder nie: { $error }

## Attachments

attachment-count = { $count ->
    [one] Een aanhegsel
   *[other] { $count } aanhegsels
}
attachment-save = Stoor
attachment-forward = Stuur aan
attachment-save-all = Stoor alles
attachment-save-all-tooltip = Stoor elke aanhegsel in 'n vouer
attachment-save-here = Stoor hier
attachment-not-downloaded = Hierdie boodskap is nie afgelaai nie.
attachment-not-found = Hierdie aanhegsel kon nie in die boodskap gevind word nie.
attachment-read-failed = Kon nie { $name } lees nie
attachment-numbered = aanhegsel { $number }
attachment-saved-all = { $count ->
    [one] { $count } lêer in { $place } gestoor
   *[other] { $count } lêers in { $place } gestoor
}
attachment-saved-some = { $total ->
    [one] { $saved } van { $total } lêer in { $place } gestoor. Kon nie stoor nie: { $failed }
   *[other] { $saved } van { $total } lêers in { $place } gestoor. Kon nie stoor nie: { $failed }
}
attachment-saved-to = Gestoor in { $path }
attachment-save-failed = Kon nie { $name } stoor nie: { $error }
attachment-open-failed = Kon nie { $name } oopmaak nie: { $error }
attachment-risky = Hierdie lêer kan 'n program laat loop, dus maak Katna dit nie oop nie. Stoor dit eerder.
attachment-encrypted-open = Hierdie lêer het geënkripteer aangekom. Stoor dit om dit elders oop te maak.

## Printing

print-failed = Kon nie druk nie: { $error }
print-no-font = geen lettertipe is gevind nie
print-opened-as-pdf = As 'n PDF oopgemaak om van daar af te druk.
print-preview-title = Drukvoorskou
print-preview-laying-out = Bladsye word uitgelê…
print-preview-pages = { $count ->
    [one] { $count } bladsy
   *[other] { $count } bladsye
}
print-preview-more = { $count ->
    [one] en nog { $count } bladsy
   *[other] en nog { $count } bladsye
}
print-preview-failed = die bladsye kon nie gewys word nie
print-preview-paper = Papier
print-preview-a4 = A4
print-preview-letter = US Letter
print-preview-layout = Uitleg
print-preview-as-shown = Soos gewys
print-preview-simple = Eenvoudige teks
print-preview-backgrounds = Agtergronde
print-preview-cancel = Kanselleer
print-preview-print = Druk
print-not-downloaded = (Nog nie afgelaai nie.)
print-encrypted = (Geënkripteer. Maak dit in Katna Mail oop om die teks te druk.)
print-to = Aan: { $addresses }
print-cc = Afskrif: { $addresses }

## Message text (right-click menu in the reading pane)

text-pin = Speld bo vas
text-copy-address = Kopieer adres

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = Maak hierdie boodskap oop om sy aanhegsels te lees.
text-copy = Kopieer
text-select-all = Kies alles
