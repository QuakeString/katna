# Katna Mail, Swahili (Kiswahili).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

chat-heading = Kusoma
chat-view = Mazungumzo kama gumzo
chat-view-detail = Barua kati ya watu husomeka kama gumzo la kikundi: kiputo kwa kila barua chenye kilichoandikwa tu, zako zikiwa upande wa kulia. Majarida hubaki na mwonekano wa kawaida.
chat-view-switch = Onyesha mazungumzo kama gumzo
chat-view-switch-detail = Barua iliyonukuliwa na sahihi husubiri nyuma ya ··· katika kila kiputo

chat-switch-chat = Gumzo
chat-switch-mail = Barua
chat-people = { $names } na wewe · { $count ->
    [one] barua { $count }
   *[other] barua { $count }
}
chat-people-heading = { $count ->
    [one] Katika gumzo hili · mtu { $count }
   *[other] Katika gumzo hili · watu { $count }
}
chat-member-mails = { $count ->
    [0] Hakuna barua
    [one] barua { $count }
   *[other] barua { $count }
}
chat-today = Leo
chat-yesterday = Jana
chat-added = { $who } amemwongeza { $names }
chat-renamed = { $who } amebadilisha mada kuwa “{ $subject }”
chat-you = Wewe
chat-not-downloaded = Bado haijapakuliwa
chat-forwarded = Imesambazwa
chat-show-quoted = Onyesha barua iliyonukuliwa na sahihi
chat-hide-quoted = Ficha barua iliyonukuliwa na sahihi
chat-hide-dots = Ficha ···
chat-show-card = Onyesha kadi yake
chat-reply-all = Jibu wote
chat-more = Zaidi
chat-reply-only = Mjibu { $name } pekee
chat-forward = Sambaza
chat-copy-text = Nakili maandishi
chat-show-as-mail = Onyesha kama barua
chat-pin = Bandika juu
chat-pin-file = Bandika faili juu
chat-unpin = Bandua
chat-unpin-file = Bandua faili
chat-pinned-of = Kibandiko { $at } kati ya { $count }
chat-pins-all = Vibandiko vyote
chat-pins-heading = Vilivyobandikwa · { $count } kati ya { $most }
chat-pins-drag = Buruta ili kupanga upya
chat-pin-from-mail = Barua kutoka kwa { $name } · { $when }
chat-pin-from-file = Faili kutoka kwa { $name } · { $when }
chat-pin-from-text = Maandishi kutoka kwa { $name } · { $when }
chat-pins-full = Gumzo hili tayari lina vibandiko 5
chat-pins-replace-title = Badilisha kibandiko
chat-pins-replace-hint = Gumzo linaweza kuwa na hadi vibandiko 5. Chagua kimoja cha kuondoa.
chat-pins-replace = Badilisha
chat-pins-cancel = Ghairi
chat-undo = Tendua

chat-reply-to = Wajibu { $names }
chat-send = Tuma (Ctrl+Enter)
chat-attach = Ambatisha
chat-attach-photo = Picha
chat-attach-file = Faili
chat-attach-library = Kutoka Faili
chat-attach-template = Kiolezo
chat-attach-signature = Sahihi
chat-replying-to = Unamjibu { $name }
chat-reply-newest = Jibu barua mpya zaidi

## The attach picker (paperclip > From Files)

picker-title = Ambatisha kutoka Faili
picker-search = Tafuta majina, watu, mada
picker-search-drive = Tafuta katika hifadhi hii
picker-mail-files = Faili za barua
picker-this-chat = Mazungumzo haya
picker-this-computer = Kompyuta hii…
picker-in-chat = KATIKA MAZUNGUMZO HAYA
picker-recent = ZA HIVI KARIBUNI
picker-preview = Onyesho la kukagua
picker-cancel = Ghairi
picker-attach = Ambatisha
picker-attach-count = Ambatisha { $count }
picker-selected = { $count } zimechaguliwa
picker-of-limit = kati ya { $limit }
picker-in-mail = { $size } ndani ya barua
picker-drive-links = { $count ->
    [one] 1 kama kiungo cha Google Drive
   *[other] { $count } kama viungo vya Google Drive
}
picker-onedrive-links = { $count ->
    [one] 1 kama kiungo cha OneDrive
   *[other] { $count } kama viungo vya OneDrive
}
picker-over = { $size }, zaidi ya { $limit } ambazo barua inaweza kubeba
picker-getting = { $count ->
    [one] Inapata faili kutoka kwenye hifadhi…
   *[other] Inapata faili { $count } kutoka kwenye hifadhi…
}
picker-some-failed = { $count ->
    [one] Faili moja haikuweza kusomwa
   *[other] Faili { $count } hazikuweza kusomwa
}
