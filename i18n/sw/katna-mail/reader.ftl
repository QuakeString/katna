# Katna Mail, Swahili (Kiswahili).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = Funga
reader-back = Rudi
reader-mark-unread = Tia alama kuwa haijasomwa
reader-move-to = Hamishia
reader-more = Zaidi
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
reader-me = mimi
reader-to = kwa { $names }
reader-starred = Ina nyota
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
tracking-opened = { $who } ameufungua { $count ->
    [one] mara moja
   *[other] mara { $count }
}, mara ya mwisho { $when }
tracking-opened-clicked = { $who } ameufungua na kufuata kiungo { $count ->
    [one] mara moja
   *[other] mara { $count }
}, mara ya mwisho { $when }
tracking-maybe-opened = Huenda { $who } ameufungua (Apple Mail hupakia picha kwa ajili ya faragha)
tracking-not-opened = { $who } bado hajaufungua
tracking-receipt = { $who } ametuma stakabadhi ya kusoma
tracking-receipt-displayed = Stakabadhi ya kusoma: { $who } amefungua ujumbe wako
tracking-receipt-other = Stakabadhi ya kusoma: { $who } amefuta au ameshughulikia ujumbe wako bila kuufungua

## Remote images and pictures

remote-hidden = Picha katika ujumbe huu zimefichwa.
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
print-preview-cancel = Ghairi
print-preview-print = Chapisha
print-not-downloaded = (Bado haujapakuliwa.)
print-encrypted = (Umesimbwa. Ufungue katika Katna Mail ili uchapishe maandishi yake.)
print-to = Kwa: { $addresses }
print-cc = Nakala: { $addresses }

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = Fungua ujumbe huu ili usome viambatisho vyake.
text-copy = Nakili
text-select-all = Chagua zote
