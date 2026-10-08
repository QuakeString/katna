# Katna Mail, Swahili (Kiswahili).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = Funga
reader-back = Rudi
reader-mark-unread = Tia alama kuwa haijasomwa
reader-move-to = Hamishia
reader-snooze = Ahirisha
reader-remind = Nikumbushe
reader-more = Zaidi
reader-original-colors = Onyesha rangi za awali
reader-dark-colors = Onyesha kwa rangi nyeusi
reader-print-all = Chapisha zote
reader-new-window = Katika dirisha jipya
reader-position = { $position } kati ya { $total }
reader-newer = Mpya zaidi
reader-older = Ya zamani zaidi

## Reading pane: the conversation

reader-removed = Mazungumzo haya yaliondolewa.
reader-no-subject = (hakuna mada)
reader-collapse-all = Kunja zote
reader-expand-all = Panua zote
reader-unknown-sender = (mtumaji asiyejulikana)
reader-date-ago = { $date } ({ $ago })
reader-sending = Inatuma…
reader-me = mimi
reader-to = kwa { $names }
reader-to-label = kwa
reader-tick-delivered = Imefikishwa { $when }
reader-tick-no-bounce = Imetumwa { $when }; hakuna barua iliyorudishwa, kwa hivyo huenda imefika
reader-tick-bounced = Haikufikishwa: ilirudishwa { $when }
reader-tick-read = Imesomwa { $when } (stakabadhi ya kusoma)
reader-tick-opened = Imefunguliwa, mara ya mwisho { $when } (ufuatiliaji wa kufunguliwa)
reader-starred = Ina nyota
reader-chip-remove = Ondoa { $label }
reader-not-starred = Haina nyota
reader-too-long = Ujumbe ni mrefu mno kuonyeshwa wote.
reader-encrypted-images = Picha kutoka kwenye wavuti hazipakiwi kamwe katika barua iliyosimbwa.
reader-window-failed = Imeshindwa kufungua dirisha jipya.

## Reading pane: message details (opened from "to me")

reader-details-from = kutoka:
reader-details-to = kwa:
reader-details-cc = nakala:
reader-details-date = tarehe:
reader-details-subject = mada:

## Reading pane: downloading a message

reader-downloading = Inapakua ujumbe huu kutoka kwenye seva…
reader-download-failed = Imeshindwa kupakua ujumbe huu.
reader-download-failed-reason = Imeshindwa kupakua ujumbe huu. { $reason }
reader-download-offline = Akaunti hii iko nje ya mtandao. Rudi mtandaoni ili kupakua ujumbe huu.
reader-try-again = Jaribu tena

## Reply row

reply-reply = Jibu
reply-reply-all = Jibu wote
reply-forward = Sambaza

## Encrypted and signed mail

security-decrypting = Inasimbua…
security-checking = Inakagua sahihi…
security-partly-encrypted = Ni sehemu tu ya ujumbe huu iliyosimbwa. Sehemu iliyobaki iliongezwa nje ya ulinzi na inaweza kutoka kwa mtu yeyote.
security-partly-signed = Ni sehemu tu ya ujumbe huu iliyotiwa sahihi. Sehemu iliyobaki iliongezwa nje ya ulinzi na inaweza kutoka kwa mtu yeyote.
security-encrypted = Ujumbe uliosimbwa
security-encrypted-smime = Ujumbe uliosimbwa (S/MIME)
security-no-key = Haiwezi kusimbua ujumbe huu: ulisimbwa kwa ufunguo usio nao.
security-cancelled = Usimbuaji ulighairiwa.
security-damaged = Haiwezi kusimbua ujumbe huu: data iliyosimbwa imeharibika au ilibadilishwa.
security-decrypt-unavailable = Haiwezi kusimbua ujumbe huu: sakinisha { $tool } ili usome barua zilizosimbwa.
security-decrypt-failed = Haiwezi kusimbua ujumbe huu: { $reason }
security-unknown-signer = mtia sahihi asiyejulikana
security-signed-verified = Imetiwa sahihi na { $signer } · imethibitishwa
security-signed-not-sender = Imetiwa sahihi na { $signer }, ambaye si mtumaji
security-signed-untrusted = Imetiwa sahihi na { $signer }, kwa ufunguo uliotia alama kuwa hauaminiki
security-signed-unverified = Imetiwa sahihi na { $signer } · ufunguo haujathibitishwa
security-bad-signature = Sahihi mbaya: ujumbe huu ulibadilishwa baada ya kutiwa sahihi, au sahihi ni ya kughushi.
security-signature-expired = Imetiwa sahihi na { $signer } · muda wa sahihi umeisha
security-key-expired = Imetiwa sahihi na { $signer } · muda wa ufunguo umeisha tangu hapo
security-key-revoked = Imetiwa sahihi na { $signer } kwa ufunguo uliobatilishwa
security-missing-key = Imetiwa sahihi kwa ufunguo usio nao, kwa hivyo haiwezi kukaguliwa
security-missing-key-id = Imetiwa sahihi kwa ufunguo usio nao ({ $key }), kwa hivyo haiwezi kukaguliwa
security-signature-unavailable = Imetiwa sahihi; sakinisha { $tool } ili kukagua sahihi
security-signature-error = Sahihi haikuweza kukaguliwa.
security-look-up-key = Tafuta ufunguo

## The key popover: the details of the key a message was signed with, and
## a key to import (looked up, or attached to the message)

key-card-verified = Sahihi imethibitishwa
key-card-verified-detail = Sahihi ni nzuri na unauamini ufunguo huu.
key-card-unverified = Sahihi haijathibitishwa
key-card-unverified-detail = Sahihi ni nzuri, lakini hakuna kinachothibitisha kuwa ufunguo ni wake. Linganisha alama ya kidole naye, kisha uamini ufunguo katika GnuPG (Kleopatra au gpg --edit-key).
key-card-not-sender = Imetiwa sahihi na mtu mwingine
key-card-not-sender-detail = Sahihi ni nzuri, lakini ufunguo si wa mtumaji.
key-card-untrusted = Ufunguo hauaminiki
key-card-untrusted-detail = Uliutia alama ufunguo huu kuwa hauaminiki katika GnuPG.
key-card-signature-expired = Muda wa sahihi umeisha
key-card-signature-expired-detail = Sahihi ilikuwa nzuri, lakini muda wake umeisha.
key-card-key-expired = Muda wa ufunguo umeisha
key-card-key-expired-detail = Sahihi ni nzuri, lakini muda wa ufunguo umeisha tangu hapo.
key-card-key-revoked = Ufunguo umebatilishwa
key-card-key-revoked-detail = Mmiliki wake aliubatilisha ufunguo huu, kwa hivyo sahihi haiwezi kuaminiwa.
key-card-bad = Sahihi mbaya
key-card-bad-detail = Ujumbe huu ulibadilishwa baada ya kutiwa sahihi, au sahihi ni ya kughushi.
key-card-signed-by = Imetiwa sahihi na
key-card-belongs-to = Ni wa
key-card-fingerprint = Alama ya kidole
key-card-signed = Imetiwa sahihi
key-card-key = Ufunguo
key-card-kind = { $standard }, { $algorithm }
key-card-created = Uliundwa
key-card-expires = Muda unaisha
key-card-never = Kamwe
key-card-issued-by = Umetolewa na
key-card-found-in = Umepatikana katika
key-card-keyring = Keyring yako ya GnuPG
key-card-copy = Nakili alama ya kidole
key-card-import-title = Uulete ufunguo huu?
key-card-from-directory = Umepatikana katika saraka ya funguo ya { $domain }.
key-card-from-attachment = Kutoka kwenye kiambatisho { $name }.
key-card-import-note = Kisha Katna inaweza kukagua sahihi za mtu huyu na kusimba barua kwake. Ili kuuamini ufunguo kikamilifu, linganisha alama ya kidole naye.
key-card-cancel = Ghairi
key-card-import = Leta ufunguo
key-card-looking-up = Inatafuta ufunguo…
key-card-looking-up-detail = Inauliza saraka ya funguo ya { $domain }.
key-card-not-found = Hakuna ufunguo uliopatikana
key-card-not-found-detail = { $domain } haichapishi ufunguo wa anwani hii. Mwombe mtumaji akutumie wake.
key-card-not-kept = Ufunguo uliopatikana hauwezi kutumika.
key-card-failed = Haikuweza kupata ufunguo

## Sender checks: the banner on mail that failed the checks the user's mail
## provider ran on its sender (DMARC, DKIM, SPF), and the "?" on the picture
## of a sender nothing confirmed

sender-failed-title = Huenda hii haitoki kwa { $domain }
sender-failed-body = Imeshindwa ukaguzi wa mtumaji wa { $provider }. Kuwa mwangalifu na viungo, viambatisho na majibu.
sender-provider-unknown = huduma yako ya barua
sender-details = Maelezo
sender-details-hide = Ficha maelezo
sender-looks-safe = Inaonekana salama
sender-move-to-spam = Hamishia kwenye taka
sender-checked-by = Imekaguliwa na { $provider }
sender-checked-by-server = Imekaguliwa na { $provider } ({ $server })
sender-dmarc = Kikoa cha mtumaji (DMARC)
sender-dkim = Sahihi (DKIM)
sender-spf = Seva inayotuma (SPF)
sender-result-pass = Imepita
sender-result-fail = Imeshindwa
sender-result-unsure = Haijulikani
sender-result-none = Hakuna
sender-result-missing = Haijakaguliwa
sender-dmarc-pass = { $domain } inamthibitisha mtumaji huyu.
sender-dmarc-fail = Barua hii hailingani na jinsi { $domain } inavyosema barua zake hutumwa.
sender-dmarc-none = { $domain } haichapishi sheria zozote za barua zake.
sender-dkim-pass = Imetiwa sahihi na { $domain }.
sender-dkim-fail = Sahihi kutoka { $domain } hailingani na barua hii.
sender-dkim-none = Ujumbe haukutiwa sahihi.
sender-spf-pass = Imetumwa kutoka kwenye seva ambayo { $domain } inaiorodhesha.
sender-spf-fail = Imetumwa kutoka kwenye seva ambayo { $domain } haiiorodheshi.
sender-spf-none = { $domain } haiorodheshi seva zake.
sender-check-unsure = Ukaguzi haukuweza kutoa jibu lililo wazi.
sender-unconfirmed = { $provider } haikuweza kuthibitisha kwamba hii ilitoka kwa { $domain }. Mtu yeyote anaweza kuandika mtumaji yeyote.
sender-link-title = Fungua kiungo hiki?
sender-link-body = Barua hii imeshindwa ukaguzi wa mtumaji. Kiungo kinaenda { $host }:
sender-link-cancel = Ghairi
sender-link-open = Fungua
tracking-opened = { $who } ameufungua { $count ->
    [one] mara moja
   *[other] mara { $count }
}, mara ya mwisho { $when }
tracking-opens-clicks = { $who } ameufungua { $opens ->
    [one] mara moja
   *[other] mara { $opens }
} na kufuata kiungo { $clicks ->
    [one] mara moja
   *[other] mara { $clicks }
}, mara ya mwisho { $when }
tracking-clicked = { $who } amefuata kiungo { $clicks ->
    [one] mara moja
   *[other] mara { $clicks }
}, mara ya mwisho { $when }
tracking-maybe-opened = Huenda { $who } ameufungua (Apple Mail hupakia picha kwa ajili ya faragha)
tracking-seen-none = Bado hakuna aliyeufungua au kufuata kiungo
tracking-receipt = { $who } ametuma stakabadhi ya kusoma
tracking-receipt-read = { $who } ameusoma (stakabadhi ya kusoma), { $when }
tracking-receipt-displayed = Stakabadhi ya kusoma: { $who } amefungua ujumbe wako
tracking-receipt-other = Stakabadhi ya kusoma: { $who } amefuta au ameshughulikia ujumbe wako bila kuufungua

## Remote images and pictures

remote-hidden = Picha katika ujumbe huu zimefichwa.
remote-hidden-unconfirmed = Picha zimefichwa: mtumaji hakuweza kuthibitishwa.
remote-hidden-failed = Picha zimefichwa: barua hii imeshindwa ukaguzi wa mtumaji.
remote-show = Onyesha picha
remote-always-show = Onyesha kila wakati kutoka kwa mtumaji huyu
remote-picture-use = Tumia
remote-picture-too-big = Chagua picha ya MB 8 au chini.
remote-picture-type = Chagua picha ya PNG, JPEG, GIF, WebP au SVG.
remote-picture-read-failed = Haiwezi kusoma picha: { $error }
remote-picture-keep-failed = Haiwezi kuhifadhi picha: { $error }
remote-picture-remove-failed = Haiwezi kuondoa picha: { $error }

## Attachments

attachment-count = { $count ->
    [one] Kiambatisho kimoja
   *[other] Viambatisho { $count }
}
attachment-save = Hifadhi
attachment-forward = Sambaza
attachment-save-all = Hifadhi vyote
attachment-save-all-tooltip = Hifadhi kila kiambatisho kwenye folda
attachment-save-here = Hifadhi hapa
attachment-not-downloaded = Ujumbe huu haujapakuliwa.
attachment-not-found = Kiambatisho hiki hakikupatikana katika ujumbe.
attachment-read-failed = Imeshindwa kusoma { $name }
attachment-numbered = kiambatisho { $number }
attachment-saved-all = { $count ->
    [one] Faili { $count } imehifadhiwa kwenye { $place }
   *[other] Faili { $count } zimehifadhiwa kwenye { $place }
}
attachment-saved-some = { $total ->
    [one] Faili { $saved } kati ya { $total } imehifadhiwa kwenye { $place }. Imeshindwa kuhifadhi { $failed }
   *[other] Faili { $saved } kati ya { $total } zimehifadhiwa kwenye { $place }. Imeshindwa kuhifadhi { $failed }
}
attachment-saved-to = Imehifadhiwa kwenye { $path }
attachment-save-failed = Imeshindwa kuhifadhi { $name }: { $error }
attachment-open-failed = Imeshindwa kufungua { $name }: { $error }
attachment-risky = Faili hii inaweza kuendesha programu, kwa hivyo Katna haiifungui. Ihifadhi badala yake.
attachment-encrypted-open = Faili hii ilikuja ikiwa imesimbwa. Ihifadhi ili uifungue mahali pengine.

## Printing

print-failed = Imeshindwa kuchapisha: { $error }
print-no-font = hakuna fonti iliyopatikana
print-opened-as-pdf = Imefunguliwa kama PDF ili uchapishe kutoka hapo.
print-preview-title = Onyesho la kuchapisha
print-preview-laying-out = Inapanga kurasa…
print-preview-pages = { $count ->
    [one] Ukurasa { $count }
   *[other] Kurasa { $count }
}
print-preview-more = { $count ->
    [one] na ukurasa { $count } zaidi
   *[other] na kurasa { $count } zaidi
}
print-preview-failed = kurasa hazikuweza kuonyeshwa
print-preview-paper = Karatasi
print-preview-a4 = A4
print-preview-letter = Letter
print-preview-layout = Muundo
print-preview-as-shown = Kama inavyoonyeshwa
print-preview-simple = Maandishi tu
print-preview-backgrounds = Mandhari ya nyuma
print-preview-cancel = Ghairi
print-preview-print = Chapisha
print-not-downloaded = (Bado haujapakuliwa.)
print-encrypted = (Umesimbwa. Ufungue katika Katna Mail ili uchapishe maandishi yake.)
print-to = Kwa: { $addresses }
print-cc = Nakala: { $addresses }

## Message text (right-click menu in the reading pane)

text-pin = Bandika juu
text-copy-address = Nakili anwani

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = Fungua ujumbe huu ili usome viambatisho vyake.
text-copy = Nakili
text-select-all = Chagua zote
