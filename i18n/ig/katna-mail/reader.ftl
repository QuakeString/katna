# Katna Mail, Igbo (Igbo).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = Mechie
reader-back = Laghachi
reader-mark-unread = Kaa akara dị ka a gụghị
reader-move-to = Bugharịa gaa
reader-snooze = Yigharịa
reader-remind = Chetara m
reader-more = Ọzọ
reader-original-colors = Gosi agba mbụ
reader-dark-colors = Gosi n'agba ọchịchịrị
reader-print-all = Bipụta niile
reader-new-window = Na windo ọhụrụ
reader-position = { $position } n'ime { $total }
reader-newer = Nke ọhụrụ
reader-older = Nke ochie

## Reading pane: the conversation

reader-removed = Ewepụla mkparịta ụka a.
reader-no-subject = (enweghị isiokwu)
reader-collapse-all = Mechie niile
reader-expand-all = Gbasaa niile
reader-unknown-sender = (onye zitere amaghị)
reader-date-ago = { $date } ({ $ago })
reader-sending = Na-ezipu…
reader-me = mụ
reader-to = gaa { $names }
reader-to-label = gaa
reader-tick-delivered = O rutere { $when }
reader-tick-no-bounce = E zigara ya { $when }; ọ dịghị ozi nlaghachi bịara, ya mere o yiri ka o rutere
reader-tick-bounced = Ọ rughị: ọ laghachiri { $when }
reader-tick-read = A gụrụ ya { $when } (akara na-egosi na a gụrụ ozi)
reader-tick-opened = E mepere ya, nke ikpeazụ { $when } (nsochi mmeghe)
reader-starred = Nwere kpakpando
reader-chip-remove = Wepụ { $label }
reader-not-starred = Enweghị kpakpando
reader-too-long = Ozi a toro ogologo nke ukwuu igosi ya niile.
reader-encrypted-images = A naghị ebudata foto si na weebụ n'ozi ezoro ezo.
reader-window-failed = Enweghị ike imepe windo ọhụrụ.

## Reading pane: message details (opened from "to me")

reader-details-from = si:
reader-details-to = gaa:
reader-details-cc = cc:
reader-details-date = ụbọchị:
reader-details-subject = isiokwu:

## Reading pane: downloading a message

reader-downloading = Na-ebudata ozi a site na sava…
reader-download-failed = Enweghị ike ibudata ozi a.
reader-download-failed-reason = Enweghị ike ibudata ozi a. { $reason }
reader-download-offline = Akaụntụ a anọghị n'ịntanetị. Laghachi n'ịntanetị iji budata ozi a.
reader-try-again = Nwaa ọzọ

## Reply row

reply-reply = Zaa
reply-reply-all = Zaa mmadụ niile
reply-forward = Zigaa

## Encrypted and signed mail

security-decrypting = Na-emeghe nzuzo…
security-checking = Na-enyocha mbinye aka…
security-partly-encrypted = Ọ bụ naanị akụkụ nke ozi a ka ezoro ezo. E tinyere nke fọdụrụ n'èzí nchedo ahụ, ọ nwere ike isi n'aka onye ọ bụla.
security-partly-signed = Ọ bụ naanị akụkụ nke ozi a ka a binyere aka. E tinyere nke fọdụrụ n'èzí nchedo ahụ, ọ nwere ike isi n'aka onye ọ bụla.
security-encrypted = Ozi ezoro ezo
security-encrypted-smime = Ozi ezoro ezo (S/MIME)
security-no-key = Enweghị ike imeghe ozi a: e ji igodo ị na-enweghị zoo ya.
security-cancelled = Akagbuola imeghe nzuzo.
security-damaged = Enweghị ike imeghe ozi a: data ezoro ezo emebiela ma ọ bụ a gbanwere ya.
security-decrypt-unavailable = Enweghị ike imeghe ozi a: wụnye { $tool } ka ị gụọ ozi ezoro ezo.
security-decrypt-failed = Enweghị ike imeghe ozi a: { $reason }
security-unknown-signer = onye binyere aka a na-amaghị
security-signed-verified = { $signer } binyere aka · akwadoro ya
security-signed-not-sender = { $signer } binyere aka, onye na-abụghị onye zitere ya
security-signed-untrusted = { $signer } binyere aka, jiri igodo i kara akara dị ka nke a na-apụghị ịtụkwasị obi
security-signed-unverified = { $signer } binyere aka · a kwadobeghị igodo ahụ
security-bad-signature = Mbinye aka ọjọọ: a gbanwere ozi a mgbe a binyechara ya aka, ma ọ bụ mbinye aka ahụ bụ adịgboroja.
security-signature-expired = { $signer } binyere aka · mbinye aka ahụ agafeela oge ya
security-key-expired = { $signer } binyere aka · igodo ahụ agafeela oge ya kemgbe ahụ
security-key-revoked = { $signer } binyere aka jiri igodo a kagburu
security-missing-key = E ji igodo ị na-enweghị binye aka, ya mere a pụghị inyocha ya
security-missing-key-id = E ji igodo ị na-enweghị ({ $key }) binye aka, ya mere a pụghị inyocha ya
security-signature-unavailable = Abinyere aka; wụnye { $tool } ka ị nyochaa mbinye aka ahụ
security-signature-error = Enweghị ike inyocha mbinye aka ahụ.
security-look-up-key = Chọọ igodo

## The key popover: the details of the key a message was signed with, and
## a key to import (looked up, or attached to the message)

key-card-verified = Mbinye aka akwadoro
key-card-verified-detail = Mbinye aka ahụ dị mma, ị tụkwasịkwara igodo a obi.
key-card-unverified = A kwadobeghị mbinye aka
key-card-unverified-detail = Mbinye aka ahụ dị mma, mana ọ dịghị ihe na-egosi na igodo ahụ bụ nke ha. Jiri fingerprint ahụ tụnyere nke ha, wee tụkwasị igodo ahụ obi na GnuPG (Kleopatra ma ọ bụ gpg --edit-key).
key-card-not-sender = Onye ọzọ binyere aka
key-card-not-sender-detail = Mbinye aka ahụ dị mma, mana igodo ahụ abụghị nke onye zitere ya.
key-card-untrusted = A tụkwasịghị igodo obi
key-card-untrusted-detail = I kara igodo a akara dị ka nke a na-apụghị ịtụkwasị obi na GnuPG.
key-card-signature-expired = Mbinye aka agafeela oge ya
key-card-signature-expired-detail = Mbinye aka ahụ dị mma, mana o agafeela oge ya.
key-card-key-expired = Igodo agafeela oge ya
key-card-key-expired-detail = Mbinye aka ahụ dị mma, mana igodo ahụ agafeela oge ya kemgbe ahụ.
key-card-key-revoked = A kagburu igodo
key-card-key-revoked-detail = Onye nwe igodo a kagburu ya, ya mere a pụghị ịtụkwasị mbinye aka ahụ obi.
key-card-bad = Mbinye aka ọjọọ
key-card-bad-detail = A gbanwere ozi a mgbe a binyechara ya aka, ma ọ bụ mbinye aka ahụ bụ adịgboroja.
key-card-signed-by = Onye binyere aka
key-card-belongs-to = Nke
key-card-fingerprint = Fingerprint
key-card-signed = Abinyere aka
key-card-key = Igodo
key-card-kind = { $standard }, { $algorithm }
key-card-created = E mepụtara
key-card-expires = Ọ ga-agwụ
key-card-never = Ọ dịghị mgbe
key-card-issued-by = Onye nyere ya
key-card-found-in = Achọtara na
key-card-keyring = Igbe igodo GnuPG gị
key-card-copy = Detuo fingerprint
key-card-import-title = Bubata igodo a?
key-card-from-directory = Achọtara ya na ndekọ igodo { $domain }.
key-card-from-attachment = Site na mgbakwunye { $name }.
key-card-import-note = Mgbe ahụ Katna nwere ike inyocha mbinye aka onye a ma zoo ozi ọ na-ezigara ya. Ka ị tụkwasị igodo ahụ obi kpamkpam, jiri fingerprint ahụ tụnyere nke ha.
key-card-cancel = Kagbuo
key-card-import = Bubata igodo
key-card-looking-up = Na-achọ igodo ahụ…
key-card-looking-up-detail = Na-ajụ ndekọ igodo { $domain }.
key-card-not-found = Achọtaghị igodo
key-card-not-found-detail = { $domain } anaghị ebipụta igodo maka adreesị a. Rịọ onye zitere ya ka o zitere gị nke ya.
key-card-not-kept = Enweghị ike iji igodo achọtara.
key-card-failed = Enweghị ike ịnweta igodo ahụ

## Sender checks: the banner on mail that failed the checks the user's mail
## provider ran on its sender (DMARC, DKIM, SPF), and the "?" on the picture
## of a sender nothing confirmed

sender-failed-title = Nke a nwere ike ọ bụghị site na { $domain }
sender-failed-body = O dabaghị na nlele onye zitere nke { $provider }. Kpachara anya na njikọ, mgbakwunye na azịza.
sender-provider-unknown = onye na-enye gị ọrụ ozi
sender-details = Nkọwa
sender-details-hide = Zoo nkọwa
sender-looks-safe = O yiri ihe dị mma
sender-move-to-spam = Bugharịa gaa Spam
sender-checked-by = E nyochara ya site n'aka { $provider }
sender-checked-by-server = E nyochara ya site n'aka { $provider } ({ $server })
sender-dmarc = Ngalaba onye zitere (DMARC)
sender-dkim = Mbinye aka (DKIM)
sender-spf = Sava na-ezipụ (SPF)
sender-result-pass = Ọ gafere
sender-result-fail = Ọ dabaghị
sender-result-unsure = Ejighị n'aka
sender-result-none = Ọ dịghị
sender-result-missing = Enyochabeghị
sender-dmarc-pass = { $domain } kwadoro onye zitere nke a.
sender-dmarc-fail = Ozi a adabaghị n'otú { $domain } si kwuo na a na-ezipụ ozi ya.
sender-dmarc-none = { $domain } anaghị ebipụta iwu ọ bụla maka ozi ya.
sender-dkim-pass = { $domain } bịanyere aka na ya.
sender-dkim-fail = Mbinye aka si na { $domain } adabaghị n'ozi ahụ.
sender-dkim-none = E bịanyeghị aka n'ozi a.
sender-spf-pass = E zitere ya site na sava { $domain } depụtara.
sender-spf-fail = E zitere ya site na sava { $domain } edepụtaghị.
sender-spf-none = { $domain } anaghị edepụta sava ya.
sender-check-unsure = Nlele ahụ enweghị ike inye azịza doro anya.
sender-unconfirmed = { $provider } enweghị ike ịkwado na nke a si na { $domain }. Onye ọ bụla nwere ike ide onye zitere ọ bụla.
sender-link-title = Mepee njikọ a?
sender-link-body = Ozi a adabaghị na nlele onye zitere ya. Njikọ ahụ na-aga { $host }:
sender-link-cancel = Kagbuo
sender-link-open = Mepee
tracking-opened = { $who } mepere ya ugboro { $count }, nke ikpeazụ { $when }
tracking-opens-clicks = { $who } mepere ya ugboro { $opens } ma soro njikọ ugboro { $clicks }, nke ikpeazụ { $when }
tracking-clicked = { $who } soro njikọ ugboro { $clicks }, nke ikpeazụ { $when }
tracking-maybe-opened = O nwere ike ịbụ na { $who } mepere ya (Apple Mail na-ebudata foto maka nzuzo)
tracking-seen-none = Ọ dịbeghị onye mepere ya ma ọ bụ soro njikọ
tracking-receipt = { $who } zitere akara na-egosi na a gụrụ ozi
tracking-receipt-read = { $who } gụrụ ya (akara na-egosi na a gụrụ ozi), { $when }
tracking-receipt-displayed = Akara na-egosi na a gụrụ ozi: { $who } mepere ozi gị
tracking-receipt-other = Akara na-egosi na a gụrụ ozi: { $who } hichapụrụ ma ọ bụ jikwaa ozi gị n'emeghe ya

## Remote images and pictures

remote-hidden = Ezochiri foto ndị dị n'ozi a.
remote-hidden-unconfirmed = Ezochiri foto: enweghị ike ịkwado onye zitere ya.
remote-hidden-failed = Ezochiri foto: ozi a adabaghị na nlele onye zitere ya.
remote-show = Gosi foto
remote-always-show = Na-egosi mgbe niile site n'aka onye zitere a
remote-picture-use = Jiri
remote-picture-too-big = Họrọ foto nke dị 8 MB ma ọ bụ nke na-erughị ya.
remote-picture-type = Họrọ foto PNG, JPEG, GIF, WebP ma ọ bụ SVG.
remote-picture-read-failed = Enweghị ike ịgụ foto ahụ: { $error }
remote-picture-keep-failed = Enweghị ike idobe foto ahụ: { $error }
remote-picture-remove-failed = Enweghị ike iwepụ foto ahụ: { $error }

## Attachments

attachment-count = Mgbakwunye { $count }
attachment-save = Chekwaa
attachment-forward = Zigaa
attachment-save-all = Chekwaa niile
attachment-save-all-tooltip = Chekwaa mgbakwunye ọ bụla na folda
attachment-save-here = Chekwaa ebe a
attachment-not-downloaded = Ebudatabeghị ozi a.
attachment-not-found = Achọtaghị mgbakwunye a n'ozi ahụ.
attachment-read-failed = Enweghị ike ịgụ { $name }
attachment-numbered = mgbakwunye { $number }
attachment-saved-all = Echekwala faịlụ { $count } na { $place }
attachment-saved-some = Echekwala { $saved } n'ime faịlụ { $total } na { $place }. Enweghị ike ichekwa { $failed }
attachment-saved-to = Echekwara na { $path }
attachment-save-failed = Enweghị ike ichekwa { $name }: { $error }
attachment-open-failed = Enweghị ike imepe { $name }: { $error }
attachment-risky = Faịlụ a nwere ike ịgba mmemme, ya mere Katna anaghị emepe ya. Chekwaa ya kama.
attachment-encrypted-open = Faịlụ a bịara ezoro ezo. Chekwaa ya ka ị mepee ya n'ebe ọzọ.

## Printing

print-failed = Enweghị ike ibipụta: { $error }
print-no-font = achọtaghị mkpụrụedemede ọ bụla
print-opened-as-pdf = Emepere ya dị ka PDF ka ị bipụta ya site n'ebe ahụ.
print-preview-title = Nlele tupu ibipụta
print-preview-laying-out = Na-ahazi peeji ndị ahụ…
print-preview-pages = Peeji { $count }
print-preview-more = na peeji { $count } ọzọ
print-preview-failed = enweghị ike igosi peeji ndị ahụ
print-preview-paper = Akwụkwọ
print-preview-a4 = A4
print-preview-letter = Letter
print-preview-layout = Nhazi
print-preview-as-shown = Dị ka e gosiri
print-preview-simple = Ederede nkịtị
print-preview-backgrounds = Ndabere
print-preview-cancel = Kagbuo
print-preview-print = Bipụta
print-not-downloaded = (Ebudatabeghị ya.)
print-encrypted = (Ezoro ezo. Mepee ya na Katna Mail ka ị bipụta ederede ya.)
print-to = Gaa: { $addresses }
print-cc = Cc: { $addresses }

## Message text (right-click menu in the reading pane)

text-pin = Kwụnye n'elu
text-copy-address = Detuo adreesị

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = Mepee ozi a ka ị gụọ mgbakwunye ya.
text-copy = Detuo
text-select-all = Họrọ niile
