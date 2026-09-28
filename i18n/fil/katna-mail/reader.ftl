# Katna Mail, Filipino (Filipino).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = Isara
reader-back = Bumalik
reader-mark-unread = Markahan bilang hindi pa nabasa
reader-move-to = Ilipat sa
reader-more = Higit pa
reader-original-colors = Ipakita ang orihinal na mga kulay
reader-dark-colors = Ipakita sa madidilim na kulay
reader-print-all = I-print lahat
reader-new-window = Sa bagong window
reader-position = { $position } ng { $total }
reader-newer = Mas bago
reader-older = Mas luma

## Reading pane: the conversation

reader-removed = Inalis ang pag-uusap na ito.
reader-no-subject = (walang subject)
reader-collapse-all = I-collapse lahat
reader-expand-all = I-expand lahat
reader-unknown-sender = (hindi kilalang nagpadala)
reader-date-ago = { $date } ({ $ago })
reader-me = ako
reader-to = para kay { $names }
reader-to-label = para kay
reader-tick-delivered = Nakarating { $when }
reader-tick-no-bounce = Ipinadala { $when }; walang mensaheng bumalik, kaya malamang nakarating na ito
reader-tick-bounced = Hindi nakarating: bumalik { $when }
reader-tick-read = Nabasa { $when } (read receipt)
reader-tick-opened = Binuksan, huli noong { $when } (open tracking)
reader-starred = Naka-star
reader-not-starred = Walang star
reader-too-long = Masyadong mahaba ang mensahe para maipakita nang buo.
reader-encrypted-images = Hindi kailanman nilo-load ang mga larawan mula sa web sa naka-encrypt na mail.
reader-window-failed = Hindi mabuksan ang bagong window.

## Reading pane: message details (opened from "to me")

reader-details-from = mula kay:
reader-details-to = para kay:
reader-details-cc = cc:
reader-details-date = petsa:
reader-details-subject = subject:

## Reading pane: downloading a message

reader-downloading = Dina-download ang mensaheng ito mula sa server…
reader-download-failed = Hindi ma-download ang mensaheng ito.
reader-try-again = Subukang muli

## Reply row

reply-reply = Sumagot
reply-reply-all = Sumagot sa lahat
reply-forward = Ipasa

## Encrypted and signed mail

security-decrypting = Dine-decrypt…
security-checking = Sinusuri ang lagda…
security-partly-encrypted = Bahagi lang ng mensaheng ito ang naka-encrypt. Ang natitira ay idinagdag sa labas ng proteksyon at maaaring galing kahit kanino.
security-partly-signed = Bahagi lang ng mensaheng ito ang may lagda. Ang natitira ay idinagdag sa labas ng proteksyon at maaaring galing kahit kanino.
security-encrypted = Naka-encrypt na mensahe
security-encrypted-smime = Naka-encrypt na mensahe (S/MIME)
security-no-key = Hindi ma-decrypt ang mensaheng ito: na-encrypt ito para sa key na wala sa iyo.
security-cancelled = Kinansela ang pag-decrypt.
security-damaged = Hindi ma-decrypt ang mensaheng ito: sira o binago ang naka-encrypt na data.
security-decrypt-unavailable = Hindi ma-decrypt ang mensaheng ito: i-install ang { $tool } para mabasa ang naka-encrypt na mail.
security-decrypt-failed = Hindi ma-decrypt ang mensaheng ito: { $reason }
security-unknown-signer = isang hindi kilalang lumagda
security-signed-verified = Nilagdaan ni { $signer } · na-verify
security-signed-not-sender = Nilagdaan ni { $signer }, na hindi ang nagpadala
security-signed-untrusted = Nilagdaan ni { $signer }, gamit ang key na minarkahan mong hindi pinagkakatiwalaan
security-signed-unverified = Nilagdaan ni { $signer } · hindi pa na-verify ang key
security-bad-signature = Hindi wastong lagda: binago ang mensaheng ito matapos itong lagdaan, o peke ang lagda.
security-signature-expired = Nilagdaan ni { $signer } · nag-expire na ang lagda
security-key-expired = Nilagdaan ni { $signer } · nag-expire na ang key mula noon
security-key-revoked = Nilagdaan ni { $signer } gamit ang key na binawi na
security-missing-key = Nilagdaan gamit ang key na wala sa iyo, kaya hindi ito masuri
security-missing-key-id = Nilagdaan gamit ang key na wala sa iyo ({ $key }), kaya hindi ito masuri
security-signature-unavailable = May lagda; i-install ang { $tool } para masuri ang lagda
security-signature-error = Hindi masuri ang lagda.
tracking-opened = Binuksan ito ni { $who } nang { $count ->
    [one] isang beses
   *[other] { $count } beses
}, huli noong { $when }
tracking-opens-clicks = Binuksan ito ni { $who } nang { $opens ->
    [one] isang beses
   *[other] { $opens } beses
} at sinundan ang isang link nang { $clicks ->
    [one] isang beses
   *[other] { $clicks } beses
}, huli noong { $when }
tracking-clicked = Sinundan ni { $who } ang isang link nang { $clicks ->
    [one] isang beses
   *[other] { $clicks } beses
}, huli noong { $when }
tracking-maybe-opened = Maaaring binuksan ito ni { $who } (naglo-load ng mga larawan ang Apple Mail para sa privacy)
tracking-not-opened = Hindi pa ito binubuksan ni { $who }
tracking-receipt = Nagpadala si { $who } ng read receipt
tracking-receipt-displayed = Read receipt: binuksan ni { $who } ang mensahe mo
tracking-receipt-other = Read receipt: dinelete o inasikaso ni { $who } ang mensahe mo nang hindi ito binubuksan

## Remote images and pictures

remote-hidden = Nakatago ang mga larawan sa mensaheng ito.
remote-show = Ipakita ang mga larawan
remote-always-show = Palaging ipakita mula sa nagpadalang ito
remote-picture-use = Gamitin
remote-picture-too-big = Pumili ng larawang 8 MB o mas maliit.
remote-picture-type = Pumili ng larawang PNG, JPEG, GIF, WebP o SVG.
remote-picture-read-failed = Hindi mabasa ang larawan: { $error }
remote-picture-keep-failed = Hindi maitabi ang larawan: { $error }
remote-picture-remove-failed = Hindi maalis ang larawan: { $error }

## Attachments

attachment-count = { $count ->
    [one] { $count } attachment
   *[other] { $count } attachment
}
attachment-save = I-save
attachment-save-all = I-save lahat
attachment-save-all-tooltip = I-save ang bawat attachment sa isang folder
attachment-save-here = I-save dito
attachment-not-downloaded = Hindi na-download ang mensaheng ito.
attachment-not-found = Hindi makita ang attachment na ito sa mensahe.
attachment-read-failed = Hindi mabasa ang { $name }
attachment-numbered = attachment { $number }
attachment-saved-all = { $count ->
    [one] Na-save ang { $count } file sa { $place }
   *[other] Na-save ang { $count } file sa { $place }
}
attachment-saved-some = { $total ->
    [one] Na-save ang { $saved } sa { $total } file sa { $place }. Hindi ma-save ang { $failed }
   *[other] Na-save ang { $saved } sa { $total } file sa { $place }. Hindi ma-save ang { $failed }
}
attachment-saved-to = Na-save sa { $path }
attachment-save-failed = Hindi ma-save ang { $name }: { $error }
attachment-open-failed = Hindi mabuksan ang { $name }: { $error }
attachment-risky = Maaaring magpatakbo ng program ang file na ito, kaya hindi ito binubuksan ng Katna. I-save na lang ito.
attachment-encrypted-open = Dumating nang naka-encrypt ang file na ito. I-save ito para mabuksan sa ibang lugar.

## Printing

print-failed = Hindi makapag-print: { $error }
print-no-font = walang nakitang font
print-opened-as-pdf = Binuksan bilang PDF para i-print mula roon.
print-preview-title = Preview ng pag-print
print-preview-laying-out = Inaayos ang mga pahina…
print-preview-pages = { $count ->
    [one] { $count } pahina
   *[other] { $count } na pahina
}
print-preview-more = { $count ->
    [one] at { $count } pang pahina
   *[other] at { $count } pang pahina
}
print-preview-failed = hindi maipakita ang mga pahina
print-preview-paper = Papel
print-preview-a4 = A4
print-preview-letter = Letter
print-preview-layout = Layout
print-preview-as-shown = Gaya ng nakikita
print-preview-simple = Simpleng teksto
print-preview-backgrounds = Mga background
print-preview-cancel = Kanselahin
print-preview-print = I-print
print-not-downloaded = (Hindi pa na-download.)
print-encrypted = (Naka-encrypt. Buksan ito sa Katna Mail para i-print ang text nito.)
print-to = Para kay: { $addresses }
print-cc = Cc: { $addresses }

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = Buksan ang mensaheng ito para mabasa ang mga attachment nito.
text-copy = Kopyahin
text-select-all = Piliin lahat
