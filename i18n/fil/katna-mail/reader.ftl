# Katna Mail, Filipino (Filipino).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = Isara
reader-back = Bumalik
reader-mark-unread = Markahan bilang hindi pa nabasa
reader-move-to = Ilipat sa
reader-snooze = I-snooze
reader-remind = Paalalahanan ako
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
reader-sending = Ipinapadala…
reader-me = ako
reader-to = para kay { $names }
reader-to-label = para kay
reader-tick-delivered = Nakarating { $when }
reader-tick-no-bounce = Ipinadala { $when }; walang mensaheng bumalik, kaya malamang nakarating na ito
reader-tick-bounced = Hindi nakarating: bumalik { $when }
reader-tick-read = Nabasa { $when } (read receipt)
reader-tick-opened = Binuksan, huli noong { $when } (open tracking)
reader-starred = Naka-star
reader-chip-remove = Alisin ang { $label }
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
reader-download-failed-reason = Hindi ma-download ang mensaheng ito. { $reason }
reader-download-offline = Offline ang account na ito. Mag-online para i-download ang mensaheng ito.
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
security-look-up-key = Hanapin ang key

## The key popover: the details of the key a message was signed with, and
## a key to import (looked up, or attached to the message)

key-card-verified = Na-verify na lagda
key-card-verified-detail = Wasto ang lagda at pinagkakatiwalaan mo ang key na ito.
key-card-unverified = Hindi na-verify ang lagda
key-card-unverified-detail = Wasto ang lagda, pero walang nagpapatunay na sa kanila ang key. Ikumpara ang fingerprint sa kanila, tapos pagkatiwalaan ang key sa GnuPG (Kleopatra o gpg --edit-key).
key-card-not-sender = Nilagdaan ng ibang tao
key-card-not-sender-detail = Wasto ang lagda, pero hindi sa nagpadala ang key.
key-card-untrusted = Hindi pinagkakatiwalaan ang key
key-card-untrusted-detail = Minarkahan mo ang key na ito bilang hindi pinagkakatiwalaan sa GnuPG.
key-card-signature-expired = Nag-expire na ang lagda
key-card-signature-expired-detail = Wasto ang lagda noon, pero nag-expire na ito.
key-card-key-expired = Nag-expire na ang key
key-card-key-expired-detail = Wasto ang lagda, pero nag-expire na ang key mula noon.
key-card-key-revoked = Binawi ang key
key-card-key-revoked-detail = Binawi ng may-ari nito ang key na ito, kaya hindi mapagkakatiwalaan ang lagda.
key-card-bad = Hindi wastong lagda
key-card-bad-detail = Binago ang mensaheng ito matapos itong lagdaan, o peke ang lagda.
key-card-signed-by = Nilagdaan ni
key-card-belongs-to = Pag-aari ni
key-card-fingerprint = Fingerprint
key-card-signed = Nilagdaan
key-card-key = Key
key-card-kind = { $standard }, { $algorithm }
key-card-created = Ginawa
key-card-expires = Mag-e-expire
key-card-never = Hindi kailanman
key-card-issued-by = Inisyu ng
key-card-found-in = Nakita sa
key-card-keyring = Ang iyong GnuPG keyring
key-card-copy = Kopyahin ang fingerprint
key-card-import-title = I-import ang key na ito?
key-card-from-directory = Nakita sa key directory ng { $domain }.
key-card-from-attachment = Mula sa attachment na { $name }.
key-card-import-note = Masusuri na ng Katna ang mga lagda ng taong ito at makakapag-encrypt ng mail para sa kanila. Para lubos na pagkatiwalaan ang key, ikumpara ang fingerprint sa kanila.
key-card-cancel = Kanselahin
key-card-import = I-import ang key
key-card-looking-up = Hinahanap ang key…
key-card-looking-up-detail = Nagtatanong sa key directory ng { $domain }.
key-card-not-found = Walang nakitang key
key-card-not-found-detail = Hindi nagpa-publish ang { $domain } ng key para sa address na ito. Hilingin sa nagpadala na ipadala sa iyo ang key nila.
key-card-not-kept = Hindi magagamit ang nakitang key.
key-card-failed = Hindi makuha ang key

## Sender checks: the banner on mail that failed the checks the user's mail
## provider ran on its sender (DMARC, DKIM, SPF), and the "?" on the picture
## of a sender nothing confirmed

sender-failed-title = Maaaring hindi ito galing sa { $domain }
sender-failed-body = Hindi ito pumasa sa mga pagsusuri ng { $provider } sa nagpadala. Mag-ingat sa mga link, attachment at pagsagot.
sender-provider-unknown = iyong mail provider
sender-details = Mga detalye
sender-details-hide = Itago ang mga detalye
sender-looks-safe = Mukhang ligtas
sender-move-to-spam = Ilipat sa spam
sender-checked-by = Sinuri ng { $provider }
sender-checked-by-server = Sinuri ng { $provider } ({ $server })
sender-dmarc = Domain ng nagpadala (DMARC)
sender-dkim = Lagda (DKIM)
sender-spf = Server na nagpadala (SPF)
sender-result-pass = Pumasa
sender-result-fail = Bumagsak
sender-result-unsure = Hindi tiyak
sender-result-none = Wala
sender-result-missing = Hindi nasuri
sender-dmarc-pass = Kinukumpirma ng { $domain } ang nagpadalang ito.
sender-dmarc-fail = Hindi tugma ang mail sa sinasabi ng { $domain } kung paano ipinapadala ang mail nito.
sender-dmarc-none = Walang inilalathalang patakaran ang { $domain } para sa mail nito.
sender-dkim-pass = Nilagdaan ng { $domain }.
sender-dkim-fail = Hindi tugma sa mail ang lagda mula sa { $domain }.
sender-dkim-none = Walang lagda ang mensahe.
sender-spf-pass = Ipinadala mula sa server na nakalista sa { $domain }.
sender-spf-fail = Ipinadala mula sa server na hindi nakalista sa { $domain }.
sender-spf-none = Hindi inililista ng { $domain } ang mga server nito.
sender-check-unsure = Hindi makapagbigay ng malinaw na sagot ang pagsusuri.
sender-unconfirmed = Hindi makumpirma ng { $provider } na galing ito sa { $domain }. Kahit sino ay puwedeng maglagay ng anumang nagpadala.
sender-link-title = Buksan ang link na ito?
sender-link-body = Hindi pumasa ang mail na ito sa mga pagsusuri sa nagpadala. Papunta ang link sa { $host }:
sender-link-cancel = Kanselahin
sender-link-open = Buksan

## Open and click tracking and read receipts (the eye's popover beside a
## sent message's star, and the line above a read receipt)

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
tracking-seen-none = Wala pang nagbukas nito o sumunod sa isang link
tracking-receipt = Nagpadala si { $who } ng read receipt
tracking-receipt-read = Binasa ito ni { $who } (read receipt), { $when }
tracking-receipt-displayed = Read receipt: binuksan ni { $who } ang mensahe mo
tracking-receipt-other = Read receipt: dinelete o inasikaso ni { $who } ang mensahe mo nang hindi ito binubuksan

## Remote images and pictures

remote-hidden = Nakatago ang mga larawan sa mensaheng ito.
remote-hidden-unconfirmed = Nakatago ang mga larawan: hindi makumpirma ang nagpadala.
remote-hidden-failed = Nakatago ang mga larawan: hindi pumasa ang mail na ito sa mga pagsusuri sa nagpadala.
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
attachment-forward = Ipasa
attachment-save-all = I-save lahat
attachment-save-all-tooltip = I-save ang bawat attachment sa isang folder
attachment-save-here = I-save dito
attachment-not-downloaded = Hindi na-download ang mensaheng ito.
attachment-open-message = Buksan ang mensaheng ito para mabasa ang mga attachment nito.
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

## Message text (right-click menu in the reading pane)

text-pin = I-pin sa itaas
text-copy-address = Kopyahin ang address
text-copy = Kopyahin
text-select-all = Piliin lahat
