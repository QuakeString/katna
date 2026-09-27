# Katna Mail, Afrikaans (Afrikaans).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = Maak toe
reader-back = Terug
reader-mark-unread = Merk as ongelees
reader-move-to = Skuif na
reader-more = Meer
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
reader-me = my
reader-to = aan { $names }
reader-starred = Gester
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

## Remote images and pictures

remote-hidden = Prente in hierdie boodskap is versteek.
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
print-preview-cancel = Kanselleer
print-preview-print = Druk
print-not-downloaded = (Nog nie afgelaai nie.)
print-encrypted = (Geënkripteer. Maak dit in Katna Mail oop om die teks te druk.)
print-to = Aan: { $addresses }
print-cc = Afskrif: { $addresses }

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = Maak hierdie boodskap oop om sy aanhegsels te lees.
text-copy = Kopieer
text-select-all = Kies alles
