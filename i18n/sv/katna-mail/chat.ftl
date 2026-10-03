# Katna Mail, Swedish (Svenska).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

chat-heading = Läsa
chat-view = Konversationer som chattar
chat-view-detail = E-post mellan personer läses som en gruppchatt: en bubbla för varje meddelande med bara det som skrevs, dina egna till höger. Nyhetsbrev behåller den vanliga vyn.
chat-view-switch = Visa konversationer som chattar
chat-view-switch-detail = Citerad e-post och signaturer väntar bakom ··· i varje bubbla

chat-switch-chat = Chatt
chat-switch-mail = E-post
chat-people = { $names } och du · { $count ->
    [one] { $count } meddelande
   *[other] { $count } meddelanden
}
chat-people-heading = { $count ->
    [one] I den här chatten · { $count } person
   *[other] I den här chatten · { $count } personer
}
chat-member-mails = { $count ->
    [0] Inga meddelanden
    [one] { $count } meddelande
   *[other] { $count } meddelanden
}
chat-today = Idag
chat-yesterday = Igår
chat-added = { $who } lade till { $names }
chat-renamed = { $who } ändrade ämnet till ”{ $subject }”
chat-you = Du
chat-not-downloaded = Inte hämtat än
chat-forwarded = Vidarebefordrat
chat-show-quoted = Visa citerad e-post och signatur
chat-hide-quoted = Dölj citerad e-post och signatur
chat-hide-dots = Dölj ···
chat-show-card = Visa kontaktkortet
chat-reply-all = Svara alla
chat-more = Mer
chat-reply-only = Svara bara { $name }
chat-forward = Vidarebefordra
chat-copy-text = Kopiera text
chat-show-as-mail = Visa som e-post
chat-pin = Fäst högst upp
chat-pin-file = Fäst filen högst upp
chat-unpin = Lossa
chat-unpin-file = Lossa filen
chat-pinned-of = Fäst { $at } av { $count }
chat-pins-all = Alla fästa
chat-pins-heading = Fästa · { $count } av { $most }
chat-pins-drag = Dra för att ändra ordning
chat-pin-from-mail = E-post från { $name } · { $when }
chat-pin-from-file = Fil från { $name } · { $when }
chat-pin-from-text = Text från { $name } · { $when }
chat-pins-full = Den här chatten har redan 5 fästa
chat-pins-replace-title = Ersätt en fäst
chat-pins-replace-hint = En chatt kan ha upp till 5 fästa. Välj den som ska tas bort.
chat-pins-replace = Ersätt
chat-pins-cancel = Avbryt
chat-undo = Ångra

chat-reply-to = Svara { $names }
chat-send = Skicka (Ctrl+Enter)
chat-attach = Bifoga
chat-attach-photo = Foto
chat-attach-file = Fil
chat-attach-library = Från Filer
chat-attach-template = Mall
chat-attach-signature = Signatur
chat-replying-to = Svarar { $name }
chat-reply-newest = Svara på det senaste meddelandet

## The attach picker (paperclip > From Files)

picker-title = Bifoga från Filer
picker-search = Sök namn, personer, ämnen
picker-search-drive = Sök i den här lagringen
picker-mail-files = E-postfiler
picker-this-chat = Den här konversationen
picker-this-computer = Den här datorn…
picker-in-chat = I DEN HÄR KONVERSATIONEN
picker-recent = SENASTE
picker-preview = Förhandsvisa
picker-cancel = Avbryt
picker-attach = Bifoga
picker-attach-count = Bifoga { $count }
picker-selected = { $count } markerade
picker-of-limit = av { $limit }
picker-in-mail = { $size } i meddelandet
picker-drive-links = { $count ->
    [one] 1 som Google Drive-länk
   *[other] { $count } som Google Drive-länkar
}
picker-onedrive-links = { $count ->
    [one] 1 som OneDrive-länk
   *[other] { $count } som OneDrive-länkar
}
picker-over = { $size }, mer än de { $limit } ett meddelande kan bära
picker-getting = { $count ->
    [one] Hämtar filen från lagringen…
   *[other] Hämtar { $count } filer från lagringen…
}
picker-some-failed = { $count ->
    [one] En fil kunde inte läsas
   *[other] { $count } filer kunde inte läsas
}
