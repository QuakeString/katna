# Katna Mail, Swahili (Kiswahili): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = Madokezo
notes-view-reminders = Vikumbusho
notes-view-archive = Kumbukumbu
notes-view-trash = Tupio
notes-edit-labels = Hariri lebo
notes-search = Tafuta madokezo
notes-loading = Inafungua madokezo yako…

## Board

notes-take-a-note = Andika dokezo…
notes-new-list = Orodha mpya
notes-new-note = Dokezo jipya
notes-pinned = Zilizobandikwa
notes-others = Zingine
notes-empty = Madokezo unayoongeza yataonekana hapa
notes-archive-empty = Madokezo yako yaliyohifadhiwa kwenye kumbukumbu yataonekana hapa
notes-trash-empty = Hakuna madokezo kwenye Tupio
notes-none-found = Hakuna madokezo yanayolingana
notes-label-empty = Bado hakuna madokezo yenye lebo hii
notes-reminders-empty = Madokezo yenye vikumbusho vijavyo huonekana hapa
notes-trash-note = Madokezo yaliyo kwenye Tupio hufutwa baada ya siku 7.
notes-empty-trash = Safisha Tupio
notes-ticked = { $count ->
    [one] + kipengee { $count } kilichotiwa alama
   *[other] + vipengee { $count } vilivyotiwa alama
}
notes-select = Chagua dokezo
notes-selected = { $count ->
    [one] { $count } limechaguliwa
   *[other] { $count } yamechaguliwa
}
notes-select-clear = Futa uteuzi

## A note's buttons

notes-pin = Bandika dokezo
notes-unpin = Ondoa dokezo lililobandikwa
notes-archive = Weka kwenye kumbukumbu
notes-unarchive = Toa kwenye kumbukumbu
notes-delete = Futa dokezo
notes-restore = Rejesha
notes-delete-forever = Futa milele
notes-color = Chaguo za mandharinyuma
notes-checkboxes = Onyesha au ficha visanduku vya kuteua
notes-labels = Lebo
notes-close = Funga
notes-more = Zaidi
notes-make-copy = Tengeneza nakala
notes-remind = Nikumbushe
notes-add-picture = Ongeza picha
notes-history = Historia ya matoleo
notes-ai = Nisaidie kuandika
notes-send-as-mail = Tuma kama barua
notes-save-markdown = Hifadhi kama Markdown
notes-save-pdf = Hifadhi kama PDF

## The open note

notes-title = Kichwa
notes-edited = Ilihaririwa { $date }
notes-on-this-computer = Kwenye kompyuta hii
notes-where = Mahali dokezo hili linapohifadhiwa
notes-untitled = Dokezo lisilo na kichwa

## Pictures

notes-picture-choose = Ongeza picha
notes-picture-remove = Ondoa picha
notes-picture-too-big = Picha hadi { $size } zinaweza kuwekwa kwenye dokezo
notes-picture-kind = Faili hilo si picha ambayo Katna inaweza kuonyesha
notes-picture-unreadable = Imeshindwa kusoma { $name }: { $error }

## Reminders

notes-remind-me = Nikumbushe
notes-remind-off = Ondoa kikumbusho
notes-remind-in-the-past = Chagua wakati ambao bado haujapita
notes-remind-today = Leo, { $time }
notes-remind-tomorrow = Kesho, { $time }
notes-remind-weekday = { $day }, { $time }
notes-reminder-set = Kikumbusho kimewekwa { $when }
notes-reminder-off = Kikumbusho kimeondolewa

## Links between notes

notes-link-note = Unganisha dokezo
notes-link-new = Dokezo jipya “{ $title }”
notes-linked-from = Limeunganishwa kutoka
notes-link-gone = Dokezo hilo halipo hapa tena
notes-new-note-gone = Dokezo jipya limetoweka.

## Version history

notes-versions = Matoleo
notes-version-now = Sasa
notes-version-here = Wewe, kwenye kompyuta hii
notes-version-yesterday = Jana, { $time }
notes-version-changes = { $count ->
    [one] Badiliko { $count }
   *[other] Mabadiliko { $count }
}
notes-version-from = Kutoka { $device }
notes-version-elsewhere = Kutoka kifaa kingine
notes-version-created = Limeundwa
notes-version-restore = Rejesha toleo hili
notes-version-restored = Toleo limerejeshwa
notes-history-none = Bado hakuna matoleo ya awali

## AI help

notes-ai-tidy = Nadhifisha maandishi
notes-ai-checklist = Igeuze kuwa orodha ya kukagua
notes-ai-summarise = Fupisha
notes-ai-empty = Andika kitu kwanza
notes-ai-tidied = Maandishi yamenadhifishwa. Ctrl+Z huyarudisha.
notes-ai-listed = Imegeuzwa kuwa orodha ya kukagua. Ctrl+Z huirudisha.
notes-ai-summarised = Muhtasari umeongezwa juu

## Labels

notes-label-note = Weka lebo kwenye dokezo
notes-label-name = Weka jina la lebo
notes-label-create = Unda “{ $name }”
notes-label-remove = Ondoa lebo
notes-label-delete = Futa lebo
notes-labels-none = Bado hakuna lebo. Ongeza kutoka kwenye kitufe cha lebo cha dokezo.
notes-labels-done = Nimemaliza
notes-label-renamed = Lebo imepewa jina jipya “{ $name }”
notes-label-deleted = Lebo “{ $name }” imefutwa

## A note about a mail

notes-mail = Barua
notes-open-mail = Fungua barua
notes-open-note = Fungua dokezo

## Meeting notes

notes-meeting-take = Andika madokezo ya mkutano
notes-meeting-title = { $title } · { $date }
notes-meeting-attendees = Waliohudhuria: { $names }
notes-meeting-notes = Madokezo
notes-meeting-actions = Hatua za kuchukua
notes-event = Tukio
notes-open-event = Fungua tukio

## Formatting

notes-format = Uumbizaji
notes-format-heading-1 = Kichwa cha 1
notes-format-heading-2 = Kichwa cha 2
notes-format-normal = Maandishi ya kawaida
notes-format-bold = Herufi nzito
notes-format-italic = Italiki
notes-format-underline = Pigia mstari
notes-format-quote = Nukuu
notes-format-code = Msimbo
notes-format-divider = Kitenganishi
notes-format-clear = Futa uumbizaji

## Tasks

notes-make-task = Ifanye jukumu

## Colors (tooltips)

notes-color-none = Bila rangi
notes-color-coral = Matumbawe
notes-color-peach = Pichi
notes-color-sand = Mchanga
notes-color-mint = Nanaa
notes-color-sage = Kijani kijivu
notes-color-fog = Ukungu
notes-color-storm = Dhoruba
notes-color-dusk = Machweo
notes-color-blossom = Ua
notes-color-clay = Udongo
notes-color-chalk = Chaki

## Messages at the foot of the window

notes-archived = Dokezo limewekwa kwenye kumbukumbu
notes-unarchived = Dokezo limetolewa kwenye kumbukumbu
notes-trashed = Dokezo limehamishiwa kwenye Tupio
notes-restored = Dokezo limerejeshwa
notes-saved = Dokezo limehifadhiwa
notes-pinned-count = { $count ->
    [one] Dokezo limebandikwa
   *[other] Madokezo { $count } yamebandikwa
}
notes-unpinned-count = { $count ->
    [one] Dokezo limebanduliwa
   *[other] Madokezo { $count } yamebanduliwa
}
notes-colored-count = { $count ->
    [one] Rangi imebadilishwa
   *[other] Rangi imebadilishwa kwenye madokezo { $count }
}
notes-archived-count = { $count ->
    [one] Dokezo limewekwa kwenye kumbukumbu
   *[other] Madokezo { $count } yamewekwa kwenye kumbukumbu
}
notes-unarchived-count = { $count ->
    [one] Dokezo limetolewa kwenye kumbukumbu
   *[other] Madokezo { $count } yametolewa kwenye kumbukumbu
}
notes-trashed-count = { $count ->
    [one] Dokezo limehamishiwa kwenye Tupio
   *[other] Madokezo { $count } yamehamishiwa kwenye Tupio
}
notes-restored-count = { $count ->
    [one] Dokezo limerejeshwa
   *[other] Madokezo { $count } yamerejeshwa
}
notes-copied-count = { $count ->
    [one] Nakala imetengenezwa
   *[other] Nakala { $count } zimetengenezwa
}
notes-empty-discarded = Dokezo tupu limetupwa
notes-mail-gone = Barua hiyo haipo hapa tena
notes-deleted-forever = { $count ->
    [one] Dokezo limefutwa milele
   *[other] Madokezo { $count } yamefutwa milele
}
