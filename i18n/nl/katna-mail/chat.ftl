# Katna Mail, Dutch (Nederlands).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

chat-heading = Lezen
chat-view = Gesprekken als chats
chat-view-detail = E-mail tussen mensen leest als een groepschat: een ballon per e-mail met alleen wat er geschreven is, die van jou rechts. Nieuwsbrieven houden de gewone weergave.
chat-view-switch = Gesprekken als chats tonen
chat-view-switch-detail = De geciteerde e-mail en handtekeningen staan achter ··· in elke ballon

chat-switch-chat = Chat
chat-switch-mail = E-mail
chat-people = { $names } en jij · { $count ->
    [one] { $count } e-mail
   *[other] { $count } e-mails
}
chat-people-heading = { $count ->
    [one] In deze chat · { $count } persoon
   *[other] In deze chat · { $count } personen
}
chat-member-mails = { $count ->
    [0] Geen e-mails
    [one] { $count } e-mail
   *[other] { $count } e-mails
}
chat-today = Vandaag
chat-yesterday = Gisteren
chat-added = { $who } heeft { $names } toegevoegd
chat-renamed = { $who } heeft het onderwerp gewijzigd in “{ $subject }”
chat-you = Jij
chat-not-downloaded = Nog niet gedownload
chat-forwarded = Doorgestuurd
chat-show-quoted = De geciteerde e-mail en handtekening tonen
chat-hide-quoted = De geciteerde e-mail en handtekening verbergen
chat-hide-dots = ··· verbergen
chat-show-card = Kaartje tonen
chat-reply-all = Allen beantwoorden
chat-more = Meer
chat-reply-only = Alleen { $name } beantwoorden
chat-forward = Doorsturen
chat-copy-text = Tekst kopiëren
chat-show-as-mail = Tonen als e-mail
chat-pin = Bovenaan vastzetten
chat-pin-file = Bestand bovenaan vastzetten
chat-unpin = Losmaken
chat-unpin-file = Bestand losmaken
chat-pinned-of = Vastgezet { $at } van { $count }
chat-pins-all = Alles vastgezet
chat-pins-heading = Vastgezet · { $count } van { $most }
chat-pins-drag = Sleep om te herschikken
chat-pin-from-mail = E-mail van { $name } · { $when }
chat-pin-from-file = Bestand van { $name } · { $when }
chat-pin-from-text = Tekst van { $name } · { $when }
chat-pins-full = Deze chat heeft al 5 vastgezette items
chat-pins-replace-title = Een vastgezet item vervangen
chat-pins-replace-hint = Een chat kan tot 5 vastgezette items hebben. Kies welke eraf moet.
chat-pins-replace = Vervangen
chat-pins-cancel = Annuleren
chat-undo = Ongedaan maken

chat-reply-to = Antwoord aan { $names }
chat-send = Versturen (Ctrl+Enter)
chat-attach = Bijvoegen
chat-attach-photo = Foto
chat-attach-file = Bestand
chat-attach-library = Uit Bestanden
chat-attach-template = Sjabloon
chat-attach-signature = Handtekening
chat-replying-to = Antwoord aan { $name }
chat-reply-newest = De nieuwste e-mail beantwoorden

## The attach picker (paperclip > From Files)

picker-title = Bijvoegen uit Bestanden
picker-search = Zoeken op namen, mensen, onderwerpen
picker-search-drive = Deze drive doorzoeken
picker-mail-files = E-mailbestanden
picker-this-chat = Dit gesprek
picker-this-computer = Deze computer…
picker-in-chat = IN DIT GESPREK
picker-recent = RECENT
picker-preview = Voorbeeld
picker-cancel = Annuleren
picker-attach = Bijvoegen
picker-attach-count = { $count } bijvoegen
picker-selected = { $count } geselecteerd
picker-of-limit = van { $limit }
picker-in-mail = { $size } in de e-mail
picker-drive-links = { $count ->
    [one] 1 als Google Drive-link
   *[other] { $count } als Google Drive-links
}
picker-onedrive-links = { $count ->
    [one] 1 als OneDrive-link
   *[other] { $count } als OneDrive-links
}
picker-over = { $size }, meer dan de { $limit } die een e-mail kan bevatten
picker-getting = { $count ->
    [one] Het bestand wordt van de drive gehaald…
   *[other] { $count } bestanden worden van de drive gehaald…
}
picker-some-failed = { $count ->
    [one] Eén bestand kon niet worden gelezen
   *[other] { $count } bestanden konden niet worden gelezen
}
