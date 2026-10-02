# Katna Mail, Afrikaans (Afrikaans).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

chat-heading = Lees
chat-view = Gesprekke as kletse
chat-view-detail = E-pos tussen mense lees soos 'n groepklets: 'n borrel vir elke e-pos met net wat geskryf is, joune aan die regterkant. Nuusbriewe behou die gewone aansig.
chat-view-switch = Wys gesprekke as kletse
chat-view-switch-detail = Die aangehaalde e-pos en handtekeninge wag agter ··· in elke borrel

chat-switch-chat = Klets
chat-switch-mail = E-pos
chat-people = { $names } en jy · { $count ->
    [one] { $count } e-pos
   *[other] { $count } e-posse
}
chat-people-heading = { $count ->
    [one] In hierdie klets · { $count } persoon
   *[other] In hierdie klets · { $count } mense
}
chat-member-mails = { $count ->
    [0] Geen e-posse
    [one] { $count } e-pos
   *[other] { $count } e-posse
}
chat-today = Vandag
chat-yesterday = Gister
chat-added = { $who } het { $names } bygevoeg
chat-renamed = { $who } het die onderwerp verander na “{ $subject }”
chat-you = Jy
chat-not-downloaded = Nog nie afgelaai nie
chat-forwarded = Aangestuur
chat-show-quoted = Wys die aangehaalde e-pos en handtekening
chat-hide-quoted = Versteek die aangehaalde e-pos en handtekening
chat-hide-dots = Versteek ···
chat-show-card = Wys hul kaart
chat-reply-all = Antwoord almal
chat-more = Meer
chat-reply-only = Antwoord net { $name }
chat-forward = Stuur aan
chat-copy-text = Kopieer teks
chat-show-as-mail = Wys as e-pos
chat-pin = Speld bo vas
chat-pin-file = Speld lêer bo vas
chat-unpin = Ontspeld
chat-unpin-file = Ontspeld lêer
chat-pinned-of = Vasgespeld { $at } van { $count }
chat-pins-all = Alle vasgespelde
chat-pins-heading = Vasgespeld · { $count } van { $most }
chat-pins-drag = Sleep om te herrangskik
chat-pin-from-mail = E-pos van { $name } · { $when }
chat-pin-from-file = Lêer van { $name } · { $when }
chat-pin-from-text = Teks van { $name } · { $when }
chat-pins-full = Hierdie klets het reeds 5 vasgespelde items
chat-pins-replace-title = Vervang 'n vasgespelde item
chat-pins-replace-hint = 'n Klets hou tot 5 vasgespelde items. Kies die een om af te haal.
chat-pins-replace = Vervang
chat-pins-cancel = Kanselleer
chat-undo = Ontdoen

chat-reply-to = Antwoord { $names }
chat-send = Stuur (Ctrl+Enter)
chat-attach = Heg aan
chat-attach-photo = Foto
chat-attach-file = Lêer
chat-attach-library = Uit Lêers
chat-attach-template = Sjabloon
chat-attach-signature = Handtekening
chat-replying-to = Antwoord tans { $name }
chat-reply-newest = Antwoord die nuutste e-pos

## The attach picker (paperclip > From Files)

picker-title = Heg aan uit Lêers
picker-search = Soek name, mense, onderwerpe
picker-search-drive = Soek in hierdie skyf
picker-mail-files = E-poslêers
picker-this-chat = Hierdie gesprek
picker-this-computer = Hierdie rekenaar…
picker-in-chat = IN HIERDIE GESPREK
picker-recent = ONLANGS
picker-preview = Voorskou
picker-cancel = Kanselleer
picker-attach = Heg aan
picker-attach-count = Heg { $count } aan
picker-selected = { $count } gekies
picker-of-limit = van { $limit }
picker-in-mail = { $size } in die e-pos
picker-drive-links = { $count ->
    [one] 1 as 'n Google Drive-skakel
   *[other] { $count } as Google Drive-skakels
}
picker-onedrive-links = { $count ->
    [one] 1 as 'n OneDrive-skakel
   *[other] { $count } as OneDrive-skakels
}
picker-over = { $size }, meer as die { $limit } wat 'n e-pos kan dra
picker-getting = { $count ->
    [one] Haal tans die lêer van die skyf af…
   *[other] Haal tans { $count } lêers van die skyf af…
}
picker-some-failed = { $count ->
    [one] Een lêer kon nie gelees word nie
   *[other] { $count } lêers kon nie gelees word nie
}
