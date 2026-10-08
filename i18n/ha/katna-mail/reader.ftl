# Katna Mail, Hausa (Hausa).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = Rufe
reader-back = Koma baya
reader-mark-unread = Yi alama ba a karanta ba
reader-move-to = Matsar zuwa
reader-snooze = Jinkirta
reader-remind = Tunatar da ni
reader-more = Ƙari
reader-original-colors = Nuna ainihin launuka
reader-dark-colors = Nuna da launuka masu duhu
reader-print-all = Buga duka
reader-new-window = A sabuwar taga
reader-position = { $position } cikin { $total }
reader-newer = Sabuwa
reader-older = Tsohuwa

## Reading pane: the conversation

reader-removed = An cire wannan tattaunawa.
reader-no-subject = (babu jigo)
reader-collapse-all = Naɗe duka
reader-expand-all = Buɗe duka
reader-unknown-sender = (mai aikawa da ba a sani ba)
reader-date-ago = { $date } ({ $ago })
reader-sending = Ana aikawa…
reader-me = ni
reader-to = zuwa ga { $names }
reader-to-label = zuwa ga
reader-tick-delivered = An isar { $when }
reader-tick-no-bounce = An aika { $when }; babu sanarwar gazawa da ta dawo, don haka tabbas ta isa
reader-tick-bounced = Ba a isar ba: ta dawo { $when }
reader-tick-read = An karanta { $when } (rasidin karantawa)
reader-tick-opened = An buɗe, na ƙarshe { $when } (bibiyar buɗewa)
reader-starred = Mai tauraro
reader-chip-remove = Cire { $label }
reader-not-starred = Babu tauraro
reader-too-long = Saƙon ya yi tsayi da yawa don a nuna shi gaba ɗaya.
reader-encrypted-images = Ba a taɓa loda hotuna daga yanar gizo a cikin wasiƙar da aka ɓoye.
reader-window-failed = Ba a iya buɗe sabuwar taga ba.

## Reading pane: message details (opened from "to me")

reader-details-from = daga:
reader-details-to = zuwa:
reader-details-cc = kwafi:
reader-details-date = kwanan wata:
reader-details-subject = jigo:

## Reading pane: downloading a message

reader-downloading = Ana sauke wannan saƙo daga sabar…
reader-download-failed = Ba a iya sauke wannan saƙo ba.
reader-download-failed-reason = Ba a iya sauke wannan saƙon ba. { $reason }
reader-download-offline = Wannan asusun ba a haɗe yake ba. Ku sake haɗawa don sauke wannan saƙo.
reader-try-again = Sake gwadawa

## Reply row

reply-reply = Amsa
reply-reply-all = Amsa wa kowa
reply-forward = Tura

## Encrypted and signed mail

security-decrypting = Ana warware ɓoyewa…
security-checking = Ana duba sa hannu…
security-partly-encrypted = Wani ɓangare ne kawai na wannan saƙo aka ɓoye. An ƙara sauran a wajen kariyar kuma yana iya fitowa daga kowa.
security-partly-signed = Wani ɓangare ne kawai na wannan saƙo aka sa wa hannu. An ƙara sauran a wajen kariyar kuma yana iya fitowa daga kowa.
security-encrypted = Saƙon da aka ɓoye
security-encrypted-smime = Saƙon da aka ɓoye (S/MIME)
security-no-key = Ba za a iya warware wannan saƙo ba: an ɓoye shi da maɓallin da ba ku da shi.
security-cancelled = An soke warware ɓoyewa.
security-damaged = Ba za a iya warware wannan saƙo ba: bayanan da aka ɓoye sun lalace ko an canza su.
security-decrypt-unavailable = Ba za a iya warware wannan saƙo ba: shigar da { $tool } don karanta wasiƙun da aka ɓoye.
security-decrypt-failed = Ba za a iya warware wannan saƙo ba: { $reason }
security-unknown-signer = wanda ba a san shi ba
security-signed-verified = { $signer } ne ya sa hannu · an tabbatar
security-signed-not-sender = { $signer } ne ya sa hannu, wanda ba shi ne mai aikawa ba
security-signed-untrusted = { $signer } ne ya sa hannu, da maɓallin da kuka yi wa alama ba amintacce ba
security-signed-unverified = { $signer } ne ya sa hannu · ba a tabbatar da maɓallin ba
security-bad-signature = Sa hannu mara kyau: an canza wannan saƙo bayan an sa masa hannu, ko sa hannun na jabu ne.
security-signature-expired = { $signer } ne ya sa hannu · wa'adin sa hannun ya ƙare
security-key-expired = { $signer } ne ya sa hannu · wa'adin maɓallin ya ƙare tun daga lokacin
security-key-revoked = { $signer } ne ya sa hannu da maɓallin da aka soke
security-missing-key = An sa hannu da maɓallin da ba ku da shi, don haka ba za a iya duba shi ba
security-missing-key-id = An sa hannu da maɓallin da ba ku da shi ({ $key }), don haka ba za a iya duba shi ba
security-signature-unavailable = An sa hannu; shigar da { $tool } don duba sa hannun
security-signature-error = Ba a iya duba sa hannun ba.
security-look-up-key = Nemo maɓalli

## The key popover: the details of the key a message was signed with, and
## a key to import (looked up, or attached to the message)

key-card-verified = Sa hannu da aka tabbatar
key-card-verified-detail = Sa hannun yana da kyau kuma kun amince da wannan maɓalli.
key-card-unverified = Ba a tabbatar da sa hannu ba
key-card-unverified-detail = Sa hannun yana da kyau, amma babu abin da ya tabbatar maɓallin nasu ne. Ku kwatanta zanen yatsa da su, sannan ku amince da maɓallin a GnuPG (Kleopatra ko gpg --edit-key).
key-card-not-sender = Wani ne daban ya sa hannu
key-card-not-sender-detail = Sa hannun yana da kyau, amma maɓallin ba na mai aikawa ba ne.
key-card-untrusted = Maɓalli ba amintacce ba
key-card-untrusted-detail = Kun yi wa wannan maɓalli alama ba amintacce ba a GnuPG.
key-card-signature-expired = Wa'adin sa hannu ya ƙare
key-card-signature-expired-detail = Sa hannun yana da kyau, amma wa'adinsa ya ƙare.
key-card-key-expired = Wa'adin maɓalli ya ƙare
key-card-key-expired-detail = Sa hannun yana da kyau, amma wa'adin maɓallin ya ƙare tun daga lokacin.
key-card-key-revoked = An soke maɓalli
key-card-key-revoked-detail = Mai shi ya soke wannan maɓalli, don haka ba za a iya amincewa da sa hannun ba.
key-card-bad = Sa hannu mara kyau
key-card-bad-detail = An canza wannan saƙo bayan an sa masa hannu, ko sa hannun na jabu ne.
key-card-signed-by = Wanda ya sa hannu
key-card-belongs-to = Na wane ne
key-card-fingerprint = Zanen yatsa
key-card-signed = An sa hannu
key-card-key = Maɓalli
key-card-kind = { $standard }, { $algorithm }
key-card-created = An ƙirƙira
key-card-expires = Wa'adi zai ƙare
key-card-never = Ba zai ƙare ba
key-card-issued-by = Wanda ya bayar
key-card-found-in = An samo a
key-card-keyring = Ma'ajiyar maɓallan GnuPG ɗinku
key-card-copy = Kwafi zanen yatsa
key-card-import-title = Shigo da wannan maɓalli?
key-card-from-directory = An samo a cikin kundin maɓallan { $domain }.
key-card-from-attachment = Daga abin da aka haɗa { $name }.
key-card-import-note = Daga nan Katna za ta iya duba sa hannun wannan mutum kuma ta ɓoye masa wasiƙa. Don cikakken amincewa da maɓallin, ku kwatanta zanen yatsa da shi.
key-card-cancel = Soke
key-card-import = Shigo da maɓalli
key-card-looking-up = Ana neman maɓallin…
key-card-looking-up-detail = Ana tambayar kundin maɓallan { $domain }.
key-card-not-found = Ba a sami maɓalli ba
key-card-not-found-detail = { $domain } ba ta wallafa maɓalli don wannan adireshi ba. Ku roƙi mai aikawa ya aiko muku da nasa.
key-card-not-kept = Ba za a iya amfani da maɓallin da aka samo ba.
key-card-failed = Ba a iya samo maɓallin ba

## Sender checks: the banner on mail that failed the checks the user's mail
## provider ran on its sender (DMARC, DKIM, SPF), and the "?" on the picture
## of a sender nothing confirmed

sender-failed-title = Wataƙila wannan ba daga { $domain } yake ba
sender-failed-body = Ya kasa binciken mai aikawa na { $provider }. Ku yi hankali da hanyoyin haɗi, abubuwan haɗawa da amsoshi.
sender-provider-unknown = mai ba ku sabis na wasiƙa
sender-details = Cikakkun bayanai
sender-details-hide = Ɓoye cikakkun bayanai
sender-looks-safe = Da alama lafiya
sender-move-to-spam = Matsar zuwa saƙonnin banza
sender-checked-by = { $provider } ya bincika
sender-checked-by-server = { $provider } ya bincika ({ $server })
sender-dmarc = Yankin mai aikawa (DMARC)
sender-dkim = Sa hannu (DKIM)
sender-spf = Sabar aikawa (SPF)
sender-result-pass = Ya wuce
sender-result-fail = Ya kasa
sender-result-unsure = Ba tabbas
sender-result-none = Babu
sender-result-missing = Ba a bincika ba
sender-dmarc-pass = { $domain } ya tabbatar da wannan mai aikawa.
sender-dmarc-fail = Wasiƙar ba ta dace da yadda { $domain } ya ce ana aika wasiƙunsa ba.
sender-dmarc-none = { $domain } bai wallafa wata ƙa'ida don wasiƙunsa ba.
sender-dkim-pass = { $domain } ya sa hannu.
sender-dkim-fail = Sa hannun daga { $domain } bai dace da wasiƙar ba.
sender-dkim-none = Ba a sa hannu a saƙon ba.
sender-spf-pass = An aiko daga sabar da { $domain } ya lissafa.
sender-spf-fail = An aiko daga sabar da { $domain } bai lissafa ba.
sender-spf-none = { $domain } bai lissafa sabobinsa ba.
sender-check-unsure = Binciken bai iya ba da amsa bayyananniya ba.
sender-unconfirmed = { $provider } bai iya tabbatar da cewa wannan ya fito daga { $domain } ba. Kowa zai iya rubuta kowane mai aikawa.
sender-link-title = A buɗe wannan hanyar haɗi?
sender-link-body = Wannan wasiƙa ta kasa binciken mai aikawa. Hanyar haɗin tana zuwa { $host }:
sender-link-cancel = Soke
sender-link-open = Buɗe
tracking-opened = { $who } ya buɗe shi { $count ->
    [one] sau ɗaya
   *[other] sau { $count }
}, na ƙarshe { $when }
tracking-opens-clicks = { $who } ya buɗe shi { $opens ->
    [one] sau ɗaya
   *[other] sau { $opens }
} kuma ya bi mahaɗi { $clicks ->
    [one] sau ɗaya
   *[other] sau { $clicks }
}, na ƙarshe { $when }
tracking-clicked = { $who } ya bi mahaɗi { $clicks ->
    [one] sau ɗaya
   *[other] sau { $clicks }
}, na ƙarshe { $when }
tracking-maybe-opened = Wataƙila { $who } ya buɗe shi (Apple Mail yana loda hotuna don sirri)
tracking-seen-none = Har yanzu babu wanda ya buɗe shi ko ya bi mahaɗi
tracking-receipt = { $who } ya aiko da rasidin karantawa
tracking-receipt-read = { $who } ya karanta shi (rasidin karantawa), { $when }
tracking-receipt-displayed = Rasidin karantawa: { $who } ya buɗe saƙonku
tracking-receipt-other = Rasidin karantawa: { $who } ya share ko ya sarrafa saƙonku ba tare da ya buɗe shi ba

## Remote images and pictures

remote-hidden = An ɓoye hotunan da ke cikin wannan saƙo.
remote-hidden-unconfirmed = An ɓoye hotuna: ba a iya tabbatar da mai aikawa ba.
remote-hidden-failed = An ɓoye hotuna: wannan wasiƙa ta kasa binciken mai aikawa.
remote-show = Nuna hotuna
remote-always-show = Koyaushe nuna daga wannan mai aikawa
remote-picture-use = Yi amfani
remote-picture-too-big = Zaɓi hoto mai girman 8 MB ko ƙasa da haka.
remote-picture-type = Zaɓi hoton PNG, JPEG, GIF, WebP ko SVG.
remote-picture-read-failed = Ba za a iya karanta hoton ba: { $error }
remote-picture-keep-failed = Ba za a iya ajiye hoton ba: { $error }
remote-picture-remove-failed = Ba za a iya cire hoton ba: { $error }

## Attachments

attachment-count = { $count ->
    [one] Abin haɗawa ɗaya
   *[other] Abubuwan haɗawa { $count }
}
attachment-save = Ajiye
attachment-forward = Tura
attachment-save-all = Ajiye duka
attachment-save-all-tooltip = Ajiye kowane abin haɗawa a cikin folda
attachment-save-here = Ajiye a nan
attachment-not-downloaded = Ba a sauke wannan saƙo ba.
attachment-not-found = Ba a sami wannan abin haɗawa a cikin saƙon ba.
attachment-read-failed = Ba a iya karanta { $name } ba
attachment-numbered = abin haɗawa { $number }
attachment-saved-all = { $count ->
    [one] An ajiye fayil { $count } a { $place }
   *[other] An ajiye fayiloli { $count } a { $place }
}
attachment-saved-some = { $total ->
    [one] An ajiye { $saved } cikin fayil { $total } a { $place }. Ba a iya ajiye { $failed } ba
   *[other] An ajiye { $saved } cikin fayiloli { $total } a { $place }. Ba a iya ajiye { $failed } ba
}
attachment-saved-to = An ajiye a { $path }
attachment-save-failed = Ba a iya ajiye { $name } ba: { $error }
attachment-open-failed = Ba a iya buɗe { $name } ba: { $error }
attachment-risky = Wannan fayil na iya gudanar da shiri, don haka Katna ba ta buɗe shi. Ajiye shi maimakon haka.
attachment-encrypted-open = Wannan fayil ya zo a ɓoye. Ajiye shi don buɗe shi a wani wuri.

## Printing

print-failed = Ba a iya bugawa ba: { $error }
print-no-font = ba a sami rubutun haruffa ba
print-opened-as-pdf = An buɗe shi a matsayin PDF don bugawa daga can.
print-preview-title = Samfotin bugawa
print-preview-laying-out = Ana tsara shafuka…
print-preview-pages = { $count ->
    [one] Shafi { $count }
   *[other] Shafuka { $count }
}
print-preview-more = { $count ->
    [one] da ƙarin shafi { $count }
   *[other] da ƙarin shafuka { $count }
}
print-preview-failed = ba a iya nuna shafukan ba
print-preview-paper = Takarda
print-preview-a4 = A4
print-preview-letter = Letter
print-preview-layout = Tsari
print-preview-as-shown = Kamar yadda aka nuna
print-preview-simple = Rubutu mai sauƙi
print-preview-backgrounds = Bango
print-preview-cancel = Soke
print-preview-print = Buga
print-not-downloaded = (Har yanzu ba a sauke shi ba.)
print-encrypted = (An ɓoye shi. Buɗe shi a cikin Katna Mail don buga rubutunsa.)
print-to = Zuwa: { $addresses }
print-cc = Kwafi: { $addresses }

## Message text (right-click menu in the reading pane)

text-pin = Maƙala a sama
text-copy-address = Kwafa adireshi

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = Buɗe wannan saƙo don karanta abubuwan haɗawarsa.
text-copy = Kwafa
text-select-all = Zaɓi duka
