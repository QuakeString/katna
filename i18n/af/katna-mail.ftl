# Katna Mail, Afrikaans (Afrikaans).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Language picker (top bar and Settings > General)

language-tooltip = Taal: { $language }
language-tooltip-system = Taal: { $language }, volg die stelsel
language-search = Soek taal
language-system-default = Stelselverstek
language-system-now = Nou { $language }
language-no-match = Geen taal pas by “{ $query }” nie
language-machine = Masjienvertaal. Help om dit te verbeter
language-setting = Taal
language-setting-detail = Die taal van kieslyste, knoppies en boodskappe, en die formaat van datums en getalle. Stelselverstek volg die werkskerm.

## Dates and sizes

ago-just-now = nou net
ago-minutes = { $count ->
    [one] { $count } minuut gelede
   *[other] { $count } minute gelede
}
ago-hours = { $count ->
    [one] { $count } uur gelede
   *[other] { $count } uur gelede
}
ago-days = { $count ->
    [one] { $count } dag gelede
   *[other] { $count } dae gelede
}
size-bytes = { $count ->
    [one] { $count } greep
   *[other] { $count } grepe
}
size-kb = { $size } KB
size-mb = { $size } MB
size-gb = { $size } GB
size-tb = { $size } TB

## Top bar

folders-hide = Versteek vouers
folders-show = Wys vouers
compose = Skryf
search = Soek
search-mail = Soek e-pos
search-settings = Soek instellings
search-clear = Vee soektog uit
search-options-show = Wys soekopsies
settings = Instellings
account-add = Voeg 'n rekening by

## App rail (and the bottom bar on a phone)

rail-mail = E-pos
rail-calendar = Kalender
rail-contacts = Kontakte
rail-tasks = Take
rail-notes = Notas
rail-feeds = Voere

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = Kom binnekort
app-calendar-promise = Jou CalDAV-kalenders, vergaderingsuitnodigings uit jou e-pos en herinneringe, langs jou inkassie.
app-tasks-promise = Taaklyste wat met CalDAV sinkroniseer, en take wat van e-pos gemaak is.
app-notes-promise = Vinnige notas, en notas oor 'n e-pos of gesprek vir later.
app-feeds-promise = Lees RSS- en Atom-voere langs jou e-pos.

## Contacts page

app-contacts-loading = Versamel tans mense uit jou e-pos…
app-contacts-empty = Mense met wie jy skryf, verskyn hier.
app-contacts-count = { $count ->
    [one] { $count } persoon uit jou e-pos, dié met wie jy die meeste skryf eerste
   *[other] { $count } mense uit jou e-pos, dié met wie jy die meeste skryf eerste
}
app-contacts-top = { $count ->
    [one] Die persoon met wie jy die meeste skryf
   *[other] Die top { $count } mense uit jou e-pos, dié met wie jy die meeste skryf eerste
}
app-contacts-messages = { $count ->
    [one] { $count } boodskap
   *[other] { $count } boodskappe
}
app-contacts-last = laas { $date }

## Navigation (the folders pane)

nav-labels = Etikette
nav-folders = Vouers
nav-label-new = Skep nuwe etiket
nav-folder-new = Skep nuwe vouer
nav-account-unnamed = Rekening { $number }
nav-tab-new = { $count ->
    [one] { $count } nuut
   *[other] { $count } nuut
}

## Special folders (the user's own folders keep their names)

folder-inbox = Inkassie
folder-starred = Gester
folder-drafts = Konsepte
folder-sent = Gestuur
folder-archive = Argief
folder-spam = Strooipos
folder-trash = Asblik
folder-all-mail = Alle pos
folder-scheduled = Geskeduleer

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = Nuwe etiket
label-folder-new-title = Nuwe vouer
label-prompt = Voer 'n nuwe etiketnaam in:
label-folder-prompt = Voer 'n nuwe vouernaam in:
label-name-hint = Etiketnaam
label-folder-name-hint = Vouernaam
label-nest = Nes etiket onder:
label-folder-nest = Nes vouer onder:
label-cancel = Kanselleer
label-create = Skep
label-creating = Skep tans…
label-created = Etiket “{ $name }” is geskep.
label-folder-created = Vouer “{ $name }” is geskep.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = Primêr
tab-promotions = Promosies
tab-social = Sosiaal
tab-updates = Opdaterings
tab-forums = Forums
tab-focused = Gefokus
tab-other = Ander
tab-inbox = Inkassie
tab-newsletters = Nuusbriewe
tab-notifications = Kennisgewings
tab-new = { $count } nuut
tab-provider-other = gesorteer deur Katna

## Mail list: toolbar

list-select = Kies
list-refresh = Herlaai
list-more = Meer
list-mark-read = Merk as gelees
list-mark-unread = Merk as ongelees
list-move-to = Skuif na
list-archive = Argiveer
list-spam = Rapporteer strooipos
list-delete = Vee uit
list-newer = Nuwer
list-older = Ouer
list-range = { $first }–{ $last } van { $total }
list-range-about = { $first }–{ $last } van ongeveer { $total }
list-results = Resultate vir “{ $query }”
list-results-corrected = Wys resultate vir “{ $query }”
list-search-instead = Soek eerder vir “{ $query }”
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = Alles
list-pick-none = Geen
list-pick-read = Gelees
list-pick-unread = Ongelees
list-pick-starred = Gester
list-pick-unstarred = Ongester

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] { $count } gesprek is gekies.
       *[other] Al { $count } gesprekke is gekies.
    }
   *[message] { $count ->
        [one] { $count } boodskap is gekies.
       *[other] Al { $count } boodskappe is gekies.
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $count } gesprek in { $folder } is gekies.
       *[other] Al { $count } gesprekke in { $folder } is gekies.
    }
   *[message] { $count ->
        [one] { $count } boodskap in { $folder } is gekies.
       *[other] Al { $count } boodskappe in { $folder } is gekies.
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] { $count } gesprek op hierdie bladsy is gekies.
       *[other] Al { $count } gesprekke op hierdie bladsy is gekies.
    }
   *[message] { $count ->
        [one] { $count } boodskap op hierdie bladsy is gekies.
       *[other] Al { $count } boodskappe op hierdie bladsy is gekies.
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] Kies { $count } gesprek
       *[other] Kies al { $count } gesprekke
    }
   *[message] { $count ->
        [one] Kies { $count } boodskap
       *[other] Kies al { $count } boodskappe
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] Kies { $count } gesprek in { $folder }
       *[other] Kies al { $count } gesprekke in { $folder }
    }
   *[message] { $count ->
        [one] Kies { $count } boodskap in { $folder }
       *[other] Kies al { $count } boodskappe in { $folder }
    }
}
list-clear-selection = Vee keuse uit

## Mail list: empty states

list-empty-search = Geen boodskappe pas by jou soektog nie.
list-empty-tab = Geen e-pos in { $tab } nie.
list-empty-tab-unknown = Geen e-pos in hierdie oortjie nie.
list-empty-folder = Geen boodskappe in { $folder } nie.
list-empty-folder-unknown = Geen boodskappe in hierdie vouer nie.
list-first-sync = Kry tans jou e-pos…
list-first-sync-detail = Dit verskyn hier soos dit aankom.

## Mail list: lines

row-removed = Hierdie boodskap is verwyder.
row-starred = Gester
row-not-starred = Nie gester nie
row-important = Belangrik. Klik om as nie belangrik nie te merk.
row-mark-important = Merk as belangrik
row-pinned = Bo vasgespeld
row-pin = Speld bo vas
row-unpin = Ontspeld

## Mail list: More menu and right-click menu

menu-reply = Antwoord
menu-reply-all = Antwoord almal
menu-forward = Stuur aan
menu-archive = Argiveer
menu-delete = Vee uit
menu-spam = Rapporteer strooipos
menu-mark-read = Merk as gelees
menu-mark-unread = Merk as ongelees
menu-mark-all-read = Merk almal as gelees
menu-star = Voeg ster by
menu-unstar = Verwyder ster
menu-important = Merk as belangrik
menu-not-important = Merk as nie belangrik nie
menu-pin = Speld bo vas
menu-unpin = Ontspeld
menu-print-all = Druk alles
menu-new-window = Maak oop in nuwe venster
menu-move-to = Skuif na
menu-move-to-heading = Skuif na:
menu-find-from = Vind e-posse van { $name }

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] Gesprek geargiveer.
       *[other] { $count } gesprekke geargiveer.
    }
   *[message] { $count ->
        [one] Boodskap geargiveer.
       *[other] { $count } boodskappe geargiveer.
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] Gesprek na die asblik geskuif.
       *[other] { $count } gesprekke na die asblik geskuif.
    }
   *[message] { $count ->
        [one] Boodskap na die asblik geskuif.
       *[other] { $count } boodskappe na die asblik geskuif.
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] Gesprek geskuif.
       *[other] { $count } gesprekke geskuif.
    }
   *[message] { $count ->
        [one] Boodskap geskuif.
       *[other] { $count } boodskappe geskuif.
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] Gesprek gester.
       *[other] { $count } gesprekke gester.
    }
   *[message] { $count ->
        [one] Boodskap gester.
       *[other] { $count } boodskappe gester.
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] Ster van gesprek verwyder.
       *[other] Ster van { $count } gesprekke verwyder.
    }
   *[message] { $count ->
        [one] Ster van boodskap verwyder.
       *[other] Ster van { $count } boodskappe verwyder.
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] Gesprek as belangrik gemerk.
       *[other] { $count } gesprekke as belangrik gemerk.
    }
   *[message] { $count ->
        [one] Boodskap as belangrik gemerk.
       *[other] { $count } boodskappe as belangrik gemerk.
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] Gesprek as nie belangrik nie gemerk.
       *[other] { $count } gesprekke as nie belangrik nie gemerk.
    }
   *[message] { $count ->
        [one] Boodskap as nie belangrik nie gemerk.
       *[other] { $count } boodskappe as nie belangrik nie gemerk.
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] Gesprek bo vasgespeld.
       *[other] { $count } gesprekke bo vasgespeld.
    }
   *[message] { $count ->
        [one] Boodskap bo vasgespeld.
       *[other] { $count } boodskappe bo vasgespeld.
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] Gesprek ontspeld.
       *[other] { $count } gesprekke ontspeld.
    }
   *[message] { $count ->
        [one] Boodskap ontspeld.
       *[other] { $count } boodskappe ontspeld.
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] Gesprek as strooipos gerapporteer.
       *[other] { $count } gesprekke as strooipos gerapporteer.
    }
   *[message] { $count ->
        [one] Boodskap as strooipos gerapporteer.
       *[other] { $count } boodskappe as strooipos gerapporteer.
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] Gesprek permanent uitgevee.
       *[other] { $count } gesprekke permanent uitgevee.
    }
   *[message] { $count ->
        [one] Boodskap permanent uitgevee.
       *[other] { $count } boodskappe permanent uitgevee.
    }
}
toast-undone = Aksie ontdoen.
toast-undo = Ontdoen
toast-no-spam-folder = Hierdie rekening het geen strooiposvouer nie.

## Reading pane: toolbar

reader-close = Maak toe
reader-back = Terug
reader-mark-unread = Merk as ongelees
reader-move-to = Skuif na
reader-more = Meer
reader-print-all = Druk alles
reader-new-window = In nuwe venster
reader-position = { $position } van { $total }
reader-newer = Nuwer
reader-older = Ouer

## Reading pane: the conversation

reader-removed = Hierdie gesprek is verwyder.
reader-no-subject = (geen onderwerp)
reader-collapse-all = Vou almal in
reader-expand-all = Vou almal uit
reader-unknown-sender = (onbekende sender)
reader-date-ago = { $date } ({ $ago })
reader-me = my
reader-to = aan { $names }
reader-starred = Gester
reader-not-starred = Nie gester nie
reader-too-long = Die boodskap is te lank om volledig te wys.
reader-encrypted-images = Prente van die web word nooit in geënkripteerde e-pos gelaai nie.
reader-window-failed = Kon nie 'n nuwe venster oopmaak nie.

## Reading pane: message details (opened from "to me")

reader-details-from = van:
reader-details-to = aan:
reader-details-cc = afskrif:
reader-details-date = datum:
reader-details-subject = onderwerp:

## Reading pane: downloading a message

reader-downloading = Laai tans hierdie boodskap van die bediener af…
reader-download-failed = Kon nie hierdie boodskap aflaai nie.
reader-try-again = Probeer weer

## Reply row

reply-reply = Antwoord
reply-reply-all = Antwoord almal
reply-forward = Stuur aan

## Encrypted and signed mail

security-decrypting = Dekripteer tans…
security-checking = Kontroleer tans die handtekening…
security-partly-encrypted = Slegs 'n deel van hierdie boodskap is geënkripteer. Die res is buite die beskerming bygevoeg en kan van enigiemand kom.
security-partly-signed = Slegs 'n deel van hierdie boodskap is onderteken. Die res is buite die beskerming bygevoeg en kan van enigiemand kom.
security-encrypted = Geënkripteerde boodskap
security-encrypted-smime = Geënkripteerde boodskap (S/MIME)
security-no-key = Kan nie hierdie boodskap dekripteer nie: dit is geënkripteer vir 'n sleutel wat jy nie het nie.
security-cancelled = Dekriptering is gekanselleer.
security-damaged = Kan nie hierdie boodskap dekripteer nie: die geënkripteerde data is beskadig of is verander.
security-decrypt-unavailable = Kan nie hierdie boodskap dekripteer nie: installeer { $tool } om geënkripteerde e-pos te lees.
security-decrypt-failed = Kan nie hierdie boodskap dekripteer nie: { $reason }
security-unknown-signer = 'n onbekende ondertekenaar
security-signed-verified = Onderteken deur { $signer } · geverifieer
security-signed-not-sender = Onderteken deur { $signer }, wat nie die sender is nie
security-signed-untrusted = Onderteken deur { $signer }, met 'n sleutel wat jy as onbetroubaar gemerk het
security-signed-unverified = Onderteken deur { $signer } · die sleutel is nie geverifieer nie
security-bad-signature = Ongeldige handtekening: hierdie boodskap is verander nadat dit onderteken is, of die handtekening is vervals.
security-signature-expired = Onderteken deur { $signer } · die handtekening het verval
security-key-expired = Onderteken deur { $signer } · die sleutel het sedertdien verval
security-key-revoked = Onderteken deur { $signer } met 'n sleutel wat herroep is
security-missing-key = Onderteken met 'n sleutel wat jy nie het nie, dus kan dit nie gekontroleer word nie
security-missing-key-id = Onderteken met 'n sleutel wat jy nie het nie ({ $key }), dus kan dit nie gekontroleer word nie
security-signature-unavailable = Onderteken; installeer { $tool } om die handtekening te kontroleer
security-signature-error = Die handtekening kon nie gekontroleer word nie.

## Remote images and pictures

remote-hidden = Prente in hierdie boodskap is versteek.
remote-show = Wys prente
remote-always-show = Wys altyd van hierdie sender
remote-picture-use = Gebruik
remote-picture-too-big = Kies 'n prent van 8 MB of kleiner.
remote-picture-type = Kies 'n PNG-, JPEG-, GIF-, WebP- of SVG-prent.
remote-picture-read-failed = Kan nie die prent lees nie: { $error }
remote-picture-keep-failed = Kan nie die prent hou nie: { $error }
remote-picture-remove-failed = Kan nie die prent verwyder nie: { $error }

## Attachments

attachment-count = { $count ->
    [one] Een aanhegsel
   *[other] { $count } aanhegsels
}
attachment-save = Stoor
attachment-save-all = Stoor alles
attachment-save-all-tooltip = Stoor elke aanhegsel in 'n vouer
attachment-save-here = Stoor hier
attachment-not-downloaded = Hierdie boodskap is nie afgelaai nie.
attachment-not-found = Hierdie aanhegsel kon nie in die boodskap gevind word nie.
attachment-read-failed = Kon nie { $name } lees nie
attachment-numbered = aanhegsel { $number }
attachment-saved-all = { $count ->
    [one] { $count } lêer in { $place } gestoor
   *[other] { $count } lêers in { $place } gestoor
}
attachment-saved-some = { $total ->
    [one] { $saved } van { $total } lêer in { $place } gestoor. Kon nie stoor nie: { $failed }
   *[other] { $saved } van { $total } lêers in { $place } gestoor. Kon nie stoor nie: { $failed }
}
attachment-saved-to = Gestoor in { $path }
attachment-save-failed = Kon nie { $name } stoor nie: { $error }
attachment-open-failed = Kon nie { $name } oopmaak nie: { $error }
attachment-risky = Hierdie lêer kan 'n program laat loop, dus maak Katna dit nie oop nie. Stoor dit eerder.
attachment-encrypted-open = Hierdie lêer het geënkripteer aangekom. Stoor dit om dit elders oop te maak.

## Printing

print-failed = Kon nie druk nie: { $error }
print-no-font = geen lettertipe is gevind nie
print-opened-as-pdf = As 'n PDF oopgemaak om van daar af te druk.
print-not-downloaded = (Nog nie afgelaai nie.)
print-encrypted = (Geënkripteer. Maak dit in Katna Mail oop om die teks te druk.)
print-to = Aan: { $addresses }
print-cc = Afskrif: { $addresses }

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = Maak hierdie boodskap oop om sy aanhegsels te lees.
text-copy = Kopieer
text-select-all = Kies alles

## Settings page: its tabs

settings-tab-general = Algemeen
settings-tab-inbox = Inkassie
settings-tab-accounts = Rekeninge
settings-tab-subscriptions = Intekeninge
settings-tab-appearance = Voorkoms
settings-tab-shortcuts = Kortpaaie
settings-tab-default-apps = Verstekprogramme
settings-tab-folders-rules = Vouers en reëls
settings-tab-compose = Skryf
settings-tab-mcp-server = MCP-bediener
settings-tab-feedback = Gebruikersterugvoer
settings-tab-experimental = Eksperimenteel

## Settings page: tabs still to come

settings-tab-subscriptions-coming = Sien die nuusbriewe en poslyste wat jy kry, en beëindig jou intekening met een klik.
settings-tab-folders-rules-coming = Skep, hernoem, skuif en versteek vouers en etikette, en kies watter een sinkroniseer. Reëls sorteer, etiketteer, stuur aan of vee nuwe e-pos vanself uit, volgens sender, onderwerp of woorde.
settings-tab-mcp-server-coming = Laat KI-assistente op hierdie rekenaar jou e-pos deursoek, lees en konsepte skryf, met jou toestemming.

## Settings > General

settings-general-conversations = Gespreksaansig
settings-general-conversations-group = Groepeer antwoorde op dieselfde e-pos
settings-general-conversations-group-detail = Een reël per gesprek in die lys
settings-general-reading = Lees
settings-general-newest-first = Nuutste boodskap eerste
settings-general-newest-first-detail = 'n Gesprek begin met die jongste antwoord
settings-general-full-headers = Wys volledige opskrifte
settings-general-full-headers-detail = Van, aan, afskrif, datum en onderwerp is oop op elke boodskap
settings-general-full-names = Volle name van ontvangers
settings-general-full-names-detail = “aan my, Ada Lovelace” eerder as “aan my, Ada”
settings-general-mark-read = Merk as gelees
settings-general-mark-read-now = Sodra dit oopmaak
settings-general-mark-read-1s = Nadat dit 1 sekonde oop is
settings-general-mark-read-3s = Nadat dit 3 sekondes oop is
settings-general-mark-read-never = Net wanneer ek dit as gelees merk
settings-general-reply-button = Antwoordknoppie
settings-general-reply-all = Antwoord almal
settings-general-reply-all-detail = Die antwoordknoppie langs elke boodskap antwoord almal, nie net die sender nie
settings-general-remote-images = Prente van die web
settings-general-remote-images-detail = As jy 'n boodskap se prente laai, weet die sender dat jy dit oopgemaak het, wanneer, en ongeveer waar. Wanneer dit af is, vra elke boodskap eers, en jy kan altyd 'n sender se prente wys.
settings-general-remote-images-always = Wys altyd prente
settings-general-remote-images-always-detail = In elke boodskap, nie net van senders wat jy vertrou nie
settings-general-sending = Stuur
settings-general-sending-detail = Hoe lank 'n gestuurde boodskap wag, sodat dit teruggeneem kan word.
settings-general-offline = Vanlyn e-pos
settings-general-offline-detail = Onlangse e-pos word volledig afgelaai om sonder 'n verbinding te lees. Ouer e-pos word afgelaai wanneer jy dit oopmaak.
settings-general-offline-days = { $count ->
    [one] { $count } dag
   *[other] { $count } dae
}
settings-general-offline-years = { $count ->
    [one] { $count } jaar
   *[other] { $count } jaar
}
settings-general-offline-all = Alle pos
settings-general-offline-note = As jy minder dae kies, bly e-pos wat reeds afgelaai is. Niks verander op die bediener nie.
settings-general-notifications = Kennisgewings
settings-general-notifications-detail = Vir nuwe e-pos in die inkassie, selfs terwyl Katna Mail toe is.
settings-general-new-mail = Stel my in kennis van nuwe e-pos
settings-general-new-mail-detail = Met Antwoord almal, Merk as gelees en Argiveer
settings-general-new-mail-sound = Speel 'n klank
settings-general-new-mail-sound-detail = Die werkskerm se nuwe-e-pos-klank
settings-general-desktop = Werkskerm
settings-general-open-at-login = Maak Katna Mail oop by aanmelding
settings-general-open-at-login-detail = E-pos sinkroniseer in elk geval by aanmelding, terwyl die diens loop
settings-general-tray = Wys Katna in die stelselbalk
settings-general-tray-detail = Met die ongeleesde telling en 'n kieslys
settings-general-unread-badge = Ongeleesde telling op die taakbalkikoon
settings-general-unread-badge-detail = Hoeveel inkassieboodskappe ongelees is

## Settings > Inbox

settings-inbox-tabs = Inkassie-oortjies
settings-inbox-tabs-detail = Sorteer die inkassie in oortjies, soos jou e-posverskaffer se webwerf doen.
settings-inbox-tabs-show = Wys inkassie-oortjies
settings-inbox-tabs-show-detail = Wanneer dit af is, word een lys vir elke rekening gewys
settings-inbox-no-accounts = Voeg 'n rekening by om sy oortjies te kies.
settings-inbox-tabs-automatic = Outomaties: { $tabs } ({ $provider })
settings-inbox-tabs-off = Geen oortjies
settings-inbox-tabs-gmail = Primêr, Promosies, Sosiaal, Opdaterings, Forums
settings-inbox-tabs-focused = Gefokus en Ander
settings-inbox-tabs-zoho = Inkassie, Nuusbriewe en Kennisgewings
settings-inbox-tabs-shown = Oortjies wat gewys word. E-pos van 'n oortjie wat jy afskakel, bly in { $tab }.

## Settings > Appearance

settings-appearance-reading-pane = Leesvenster
settings-appearance-reading-pane-detail = Waar 'n oop gesprek gewys word.
settings-appearance-pane-right = Regs van die lys
settings-appearance-pane-none = Geen verdeling
settings-appearance-density = Digtheid
settings-appearance-density-default = Verstek
settings-appearance-density-compact = Kompak
settings-appearance-scaling = Skalering
settings-appearance-scaling-detail = Maak alles in Katna Mail groter of kleiner, bo-op die werkskerm se eie skaal: teks, ikone, spasiëring en skeidslyne. E-pos wat jy stuur, behou sy eie lettergrootte. Baie klein groottes kan dit moeilik maak om op ikone te klik.
settings-appearance-theme = Tema
settings-appearance-theme-system = Dieselfde as die werkskerm
settings-appearance-theme-light = Lig
settings-appearance-theme-dark = Donker
settings-appearance-desktop-colors = Werkskermkleure
settings-appearance-desktop-colors-use = Gebruik die werkskerm se kleure
settings-appearance-desktop-colors-use-detail = Die kleurskema en aksentkleur van die werkskerm
settings-appearance-app-names = Programname
settings-appearance-app-names-show = Wys programname
settings-appearance-app-names-show-detail = Name onder die programikone heel links
settings-appearance-sender-pictures = Senderprente
settings-appearance-sender-pictures-show = Wys maatskappylogo's
settings-appearance-sender-pictures-show-detail = Opgesoek volgens die sender se domein, nooit volgens boodskap nie, en 'n week lank gehou
settings-appearance-important = Belangrik-merkers
settings-appearance-important-show = Wys Belangrik-merkers
settings-appearance-important-show-detail = Langs elke boodskap in die lys
settings-appearance-message-width = Boodskapwydte
settings-appearance-message-width-limit = Beperk die wydte van boodskappe
settings-appearance-message-width-limit-detail = Lang reëls is so makliker om in 'n wye venster te lees
settings-appearance-mail-colors = E-poskleure
settings-appearance-mail-colors-detail = Die meeste e-pos is vir 'n wit bladsy ontwerp. Met 'n donker tema word die kleure verander na donker kleure wat goed lees; wanneer dit af is, behou dit die sender se kleure op 'n ligte bladsy.
settings-appearance-dark-mail = Donker kleure ook vir e-pos
settings-appearance-dark-mail-detail = Net terwyl die tema donker is
settings-appearance-attachment-previews = Aanhegselvoorskoue
settings-appearance-attachment-previews-show = Wys voorskoue van aanhegsels
settings-appearance-attachment-previews-show-detail = 'n Klein prentjie van elke lêer se inhoud op sy kaart

## Settings > Default apps

settings-default-apps-intro = Waar aanhegsels oopmaak wanneer jy daarop klik. Die kyker kan 'n lêer ook altyd in 'n ander program oopmaak. Die werkskerm se verstekprogramme word in sy eie instellings gestel.
settings-default-apps-pdf = PDF-lêers
settings-default-apps-pdf-detail = Bladsye, met zoem.
settings-default-apps-pictures = Prente
settings-default-apps-pictures-detail = Foto's (regop gedraai), PNG, GIF, WebP, BMP, TIFF en SVG.
settings-default-apps-text = Tekslêers
settings-default-apps-text-detail = Gewone teks, logboeke, kode en ander teks.
settings-default-apps-sheets = Sigblaaie
settings-default-apps-sheets-detail = Excel (xlsx, xls), OpenDocument (ods) en CSV.
settings-default-apps-documents = Dokumente
settings-default-apps-documents-detail = Word (docx) en OpenDocument-teks (odt).
settings-default-apps-katna = Katna Mail se kyker
settings-default-apps-system = Die werkskerm se verstekprogram
settings-default-apps-ask = Vra elke keer watter program
settings-default-apps-after-saving = Ná stoor
settings-default-apps-show-folder = Wys gestoorde lêers in hul vouer
settings-default-apps-show-folder-detail = Maak die lêerbestuurder oop met die gestoorde aanhegsels gekies

## Settings > Compose

settings-compose-send-from = Stuur nuwe boodskappe van
settings-compose-send-from-detail = Antwoorde en aangestuurde boodskappe gaan altyd uit van die rekening waarin jy is.
settings-compose-send-from-current = Die rekening waarin jy is
settings-compose-send-on-replies = Stuur op antwoorde
settings-compose-send-on-replies-detail = Wat Stuur doen op 'n antwoord of aanstuur. Die kieslys langs Stuur bied die ander keuse.
settings-compose-send-plain = Stuur
settings-compose-send-archive = Stuur en argiveer
settings-compose-signatures = Handtekeninge
settings-compose-signatures-detail = Onder jou boodskap bygevoeg, ná 'n “--”-reël. Kies 'n ander een in die skryfvenster.
settings-compose-untitled = Titelloos
settings-compose-signature-name = Naam, soos Werk
settings-compose-signature-first = My handtekening
settings-compose-signature-numbered = Handtekening { $number }
settings-compose-signature-delete = Vee uit
settings-compose-signature-deleted = Handtekening uitgevee
settings-compose-signature-new = Skep nuwe
settings-compose-no-signatures = Nog geen handtekeninge nie.
settings-compose-no-signature = Geen handtekening
settings-compose-for-new-mail = Vir nuwe e-pos
settings-compose-for-replies = Vir antwoorde en aangestuurde boodskappe
settings-compose-for-replies-detail = In 'n gesprek waarin jy 'n boodskap onderteken het, begin 'n antwoord eerder met daardie handtekening.
settings-compose-format = Formaat
settings-compose-plain-text = Skryf in gewone teks
settings-compose-plain-text-detail = Nuwe e-pos begin sonder formatering; die skryfvenster kan omskakel
settings-compose-spelling = Spelling
settings-compose-spell-check = Kontroleer spelling terwyl ek skryf
settings-compose-spell-check-detail = Verkeerd gespelde woorde word onderstreep, met voorstelle as jy regs klik
settings-compose-spell-desktop = Werkskerm se taal ({ $language })
settings-compose-templates = Sjablone
settings-compose-templates-detail = Stoor e-pos wat jy gereeld skryf, en begin nuwe e-pos of 'n antwoord daarmee.

## Settings > Shortcuts

settings-shortcuts-set = Kortpadstel
settings-shortcuts-set-detail = Begin met die sleutels van 'n e-posprogram wat jy ken. Cmd is hier Ctrl. Jou eie veranderinge bly bo-op die stel, en Herstel verstek gaan terug na die stel se sleutels.
settings-shortcuts-single = Enkelsleutelkortpaaie
settings-shortcuts-single-detail = Sleutels sonder Ctrl of Alt, soos in webpos: e argiveer, j en k skuif, / soek. Hulle werk in die lys en die oop gesprek, nooit terwyl jy tik nie.
settings-shortcuts-single-use = Gebruik enkelsleutelkortpaaie
settings-shortcuts-single-use-detail = Ctrl-kortpaaie werk altyd
settings-shortcuts-how = Klik op 'n sleutel om dit te verander, of op + om een by te voeg, en druk dan die nuwe sleutels. Esc kanselleer.
settings-shortcuts-restore = Herstel verstek
settings-shortcuts-no-key = Geen sleutel
settings-shortcuts-press = Druk sleutels…
settings-shortcuts-then = { $keys } dan…
settings-shortcuts-moved = { $keys } doen nou “{ $action }” in plaas van “{ $previous }”.
settings-shortcuts-single-off = Enkelsleutelkortpaaie is af, dus werk hierdie sleutel sodra hulle aan is.
settings-shortcuts-restored = Elke kortpad het weer sy stel se sleutels.

## Settings search: the line under a result

settings-general-language-summary = Taal van die program, datums en getalle
settings-general-reading-summary = Nuutste boodskap eerste, volledige opskrifte, volle name van ontvangers
settings-general-mark-read-summary = Wanneer 'n oop gesprek as gelees gemerk word: dadelik, ná 1 of 3 sekondes, of met die hand
settings-general-reply-button-summary = Die antwoordknoppie langs elke boodskap antwoord almal
settings-general-remote-images-summary = Wys altyd die prente van elke boodskap
settings-general-sending-summary = Ontdoen stuur: hoe lank 'n gestuurde boodskap wag, sodat dit teruggeneem kan word
settings-general-offline-summary = Hoeveel dae se onlangse e-pos volledig afgelaai word om sonder 'n verbinding te lees
settings-general-notifications-summary = Kennisgewings van nuwe e-pos en hul klank
settings-general-desktop-summary = Maak Katna Mail oop by aanmelding, die stelselbalkikoon en die ongeleesde telling op die taakbalkikoon
settings-accounts-accounts-summary = Voeg 'n rekening by of verwyder een, of verander sy prent
settings-appearance-density-summary = Verstek- of kompakte reëls in die lys
settings-appearance-scaling-summary = Maak alles groter of kleiner: teks, ikone, spasiëring en skeidslyne
settings-appearance-theme-summary = Dieselfde as die werkskerm, lig of donker
settings-appearance-sender-pictures-summary = Maatskappylogo's, opgesoek volgens die sender se domein
settings-appearance-important-summary = Die Belangrik-merker langs elke boodskap in die lys
settings-appearance-mail-colors-summary = Donker kleure vir HTML-e-pos in 'n donker tema, of die sender se kleure
settings-appearance-attachment-previews-summary = 'n Klein prentjie van elke aanhegsel se inhoud
settings-shortcuts-set-summary = Begin met die sleutels van Gmail, Inbox by Gmail, Apple Mail, Outlook of Thunderbird
settings-shortcuts-single-summary = Sleutels sonder Ctrl of Alt, soos in webpos
settings-default-apps-pdf-summary = Waar PDF-aanhegsels oopmaak
settings-default-apps-pictures-summary = Waar foto's en prente oopmaak
settings-default-apps-text-summary = Waar gewone teks, logboeke en kode oopmaak
settings-default-apps-sheets-summary = Waar Excel-, OpenDocument- en CSV-lêers oopmaak
settings-default-apps-documents-summary = Waar Word- en OpenDocument-teks oopmaak
settings-default-apps-after-saving-summary = Wys gestoorde aanhegsels in hul vouer
settings-compose-send-from-summary = Die rekening waarvandaan nuwe e-pos uitgaan: die een waarin jy is, of altyd dieselfde een
settings-compose-send-on-replies-summary = Stuur, of Stuur en argiveer die gesprek, op antwoorde en aangestuurde boodskappe
settings-compose-signatures-summary = Onder jou boodskap bygevoeg, ná 'n “--”-reël
settings-compose-for-new-mail-summary = Die handtekening waarmee nuwe e-pos begin
settings-compose-for-replies-summary = Die handtekening waarmee antwoorde en aangestuurde boodskappe begin
settings-compose-format-summary = Skryf nuwe e-pos in gewone teks
settings-compose-spelling-summary = Kontroleer spelling tydens skryf, en die woordeboek se taal
settings-compose-templates-summary = Kom binnekort: stoor e-pos wat jy gereeld skryf, en begin nuwe e-pos of 'n antwoord daarmee
settings-feedback-crash-reports-summary = Stoor omvalverslae op hierdie rekenaar wanneer Katna Mail of sy agtergronddiens omval
settings-feedback-saved-summary = Bekyk, kopieer of vee die omvalverslae uit wat op hierdie rekenaar gestoor is
settings-feedback-help-improve-summary = Stuur omvalverslae om te help regmaak wat skeefgeloop het; af tensy jy dit aanskakel
settings-experimental-blur-summary = Die werkskerm skyn vaag deur die boonste balk, en kieslyste is soos matglas
settings-search-shortcut = Kortpadsleutel
settings-search-tab = Instellingsoortjie
settings-search-none = Geen instellings pas by “{ $query }” nie.
settings-search-results = Instellings wat by “{ $query }” pas

## Quick settings (the panel that slides in from the right)

quick-title = Vinnige instellings
quick-see-all = Sien alle instellings
quick-reading-pane = Leesvenster
quick-pane-right = Regs van die lys
quick-pane-none = Geen verdeling
quick-density = Digtheid
quick-density-default = Verstek
quick-density-compact = Kompak
quick-theme = Tema
quick-theme-system = Dieselfde as die werkskerm
quick-theme-light = Lig
quick-theme-dark = Donker
quick-desktop-colors = Werkskermkleure
quick-desktop-colors-detail = Die kleurskema en aksentkleur van die werkskerm
quick-app-names = Programname
quick-app-names-detail = Name onder die programikone heel links
quick-inbox-tabs = Inkassie-oortjies
quick-inbox-tabs-detail = Die oortjies van elke rekening se e-posverskaffer
quick-choose-tabs = Kies oortjies
quick-choose-tabs-detail = Per rekening, in Instellings
quick-sending = Stuur
quick-undo-send = Ontdoen stuur
quick-undo-send-off = Af
quick-undo-send-seconds = { $seconds } s
quick-signatures = Handtekeninge
quick-signatures-none = Nog geen
quick-signatures-one = { $name }, by verstek gebruik
quick-signatures-many = { $count ->
    [one] { $count } handtekening; { $name } by verstek
   *[other] { $count } handtekeninge; { $name } by verstek
}
quick-signatures-no-default = { $count ->
    [one] { $count }, geen by verstek nie
   *[other] { $count }, geen by verstek nie
}
quick-signature-untitled = Titelloos
quick-threading = E-posdrade
quick-conversation-view = Gespreksaansig
quick-conversation-view-detail = Groepeer antwoorde op dieselfde e-pos
quick-help = Hulp
quick-tour = Neem die toer
quick-whats-new = Wat's nuut
quick-about = Meer oor Katna

## Settings: opening at login

settings-open-at-login-failed = Kon nie oopmaak by aanmelding verander nie: { $error }

## Settings > Appearance > Scaling

scale-letter = A
scale-percent = { $percent }%
scale-reset = Terug na { $percent }%

## Settings > Experimental > Look & Feel

look-intro = Kenmerke wat nog uitgetoets word. Hulle kan verander of verdwyn.
look-heading = Voorkoms en gevoel
look-window-frame = Vensterraam
look-window-frame-detail = Wie die titelbalk, die vensterknoppies, die hoeke en die skaduwee teken.
look-frame-native-kde = Inheems: KDE se raam, in jou Plasma-tema
look-frame-native = Inheems: die werkskerm se raam
look-frame-katna = Katna: die boonste balk word die titelbalk
look-frame-katna-note-named = Katna teken geronde hoeke en sy eie skaduwee. Die raam volg nie meer die { $desktop }-tema nie; vensterreëls geld steeds.
look-frame-katna-note = Katna teken geronde hoeke en sy eie skaduwee. Die raam volg nie meer die werkskermtema nie; vensterreëls geld steeds.
look-frame-client-side = Jou werkskerm laat die raam aan elke program oor, dus teken Katna reeds sy eie.
look-blurred-background = Vaag agtergrond
look-blurred-background-detail = Die werkskerm skyn vaag deur die boonste balk en die vouers, en kieslyste en opspringers is van matglas.
look-blur = Maak vaag wat agter die venster is
look-blur-detail = E-pos bly op soliede kaarte, sodat teks sy kontras behou
look-blur-off-kde = KDE se vervaag-effek is af. Skakel Vervaag aan in Stelselinstellings, Vensterbestuur, Werkskermeffekte, en maak dan Katna Mail weer oop.
look-blur-none-gnome = GNOME maak nie vaag wat agter vensters is nie.
look-blur-none-x11 = Jou vensterbestuurder maak nie vaag wat agter vensters is nie.
look-blur-none-wayland = Jou saamsteller maak nie vaag wat agter vensters is nie.

## Settings > User feedback (crash reports)

feedback-intro-sending = Nuwe omvalverslae word gestuur om te help regmaak wat skeefgeloop het. Niks anders verlaat hierdie rekenaar nie.
feedback-intro-local = Katna stuur niks nêrens heen nie. Omvalverslae bly op hierdie rekenaar, sodat jy daarna kan kyk of dit by 'n foutverslag kan aanheg.
feedback-crash-reports = Omvalverslae
feedback-crash-reports-detail = Geskryf wanneer Katna Mail of sy agtergronddiens omval.
feedback-save = Stoor omvalverslae op hierdie rekenaar
feedback-save-detail = Jou tuisvouer, gebruiker- en rekenaarname en e-posadresse word weggelaat
feedback-saved = Gestoorde omvalverslae
feedback-saved-detail = { $count ->
    [one] Die nuutste { $count } word gehou.
   *[other] Die nuutste { $count } word gehou.
}
feedback-help-improve = Help om Katna te verbeter
feedback-help-improve-detail = Af tensy jy dit aanskakel, en jy kan dit enige tyd hier afskakel.
feedback-send = Stuur omvalverslae
feedback-send-detail = Die gestoorde verslag, presies soos jy dit hier kan bekyk, gaan na Katna se omvalspoorder (Sentry, in die EU). Geen IP-adres, boodskappe of e-posadresse nie
feedback-none-saved = Geen omvalverslae is gestoor nie.
feedback-delete-all = Vee alles uit
feedback-app-daemon = Agtergronddiens
feedback-report-sent = { $date } · Gestuur
feedback-view = Bekyk
feedback-view-tooltip = Maak die verslag oop
feedback-copy-tooltip = Kopieer dit om in 'n foutverslag te plak
feedback-copied = Omvalverslag gekopieer.
feedback-deleted-all = Omvalverslae uitgevee.
feedback-read-failed = Kon nie die omvalverslag lees nie: { $error }
feedback-delete-failed = Kon nie die omvalverslag uitvee nie: { $error }
feedback-delete-all-failed = Kon nie die omvalverslae uitvee nie: { $error }

## Menu bar (the KDE global menu)

desktop-menu-file = _Lêer
desktop-menu-new-message = _Nuwe boodskap
desktop-menu-quit = _Verlaat
desktop-menu-edit = _Wysig
desktop-menu-undo = _Ontdoen
desktop-menu-select-all = Kies _alles
desktop-menu-select-none = Kies _niks
desktop-menu-find = _Vind…
desktop-menu-view = _Aansig
desktop-menu-folder-list = Wys _vouerlys
desktop-menu-refresh = _Herlaai
desktop-menu-go = _Gaan
desktop-menu-inbox = _Inkassie
desktop-menu-starred = _Gester
desktop-menu-sent = Ge_stuur
desktop-menu-drafts = _Konsepte
desktop-menu-all-mail = _Alle pos
desktop-menu-next = _Volgende gesprek
desktop-menu-previous = V_orige gesprek
desktop-menu-message = _Boodskap
desktop-menu-open = _Maak oop
desktop-menu-reply = _Antwoord
desktop-menu-reply-all = Antwoord a_lmal
desktop-menu-forward = Stuur a_an
desktop-menu-archive = A_rgiveer
desktop-menu-delete = Vee _uit
desktop-menu-spam = Rapporteer _strooipos
desktop-menu-move-to = S_kuif na…
desktop-menu-mark-read = Merk as g_elees
desktop-menu-mark-unread = Merk as o_ngelees
desktop-menu-star = Voeg s_ter by
desktop-menu-important = Merk as _belangrik
desktop-menu-not-important = Merk as _nie belangrik nie
desktop-menu-settings = _Instellings
desktop-menu-quick-settings = _Vinnige instellings
desktop-menu-configure = _Stel Katna Mail op…
desktop-menu-help = _Hulp
desktop-menu-shortcuts = _Kortpadsleutels
desktop-menu-whats-new = _Wat's nuut
desktop-menu-about = _Meer oor Katna

## Settings > Keyboard shortcuts: the groups of the list

shortcut-group-moving = Rondbeweeg
shortcut-group-actions = Aksies
shortcut-group-go-to = Gaan na
shortcut-group-app = Program

## Settings > Keyboard shortcuts: what each shortcut does

shortcut-next = Volgende gesprek
shortcut-previous = Vorige gesprek
shortcut-down = Beweeg af in die lys
shortcut-up = Beweeg op in die lys
shortcut-first = Eerste in die lys
shortcut-last = Laaste in die lys
shortcut-page-down = 'n Bladsy af in die lys
shortcut-page-up = 'n Bladsy op in die lys
shortcut-open = Maak gesprek oop
shortcut-back = Terug na die lys
shortcut-scroll-down = Rol af
shortcut-scroll-up = Rol op
shortcut-scroll-page-down = Rol 'n bladsy af
shortcut-scroll-page-up = Rol 'n bladsy op
shortcut-compose = Skryf
shortcut-reply = Antwoord
shortcut-reply-all = Antwoord almal
shortcut-forward = Stuur aan
shortcut-archive = Argiveer
shortcut-delete = Vee uit
shortcut-spam = Rapporteer strooipos
shortcut-move-to = Skuif na
shortcut-mark-read = Merk as gelees
shortcut-mark-unread = Merk as ongelees
shortcut-star = Voeg ster by of verwyder dit
shortcut-important = Merk as belangrik
shortcut-not-important = Merk as nie belangrik nie
shortcut-check = Merk die gesprek
shortcut-select-all = Merk alle gesprekke
shortcut-select-none = Ontmerk alle gesprekke
shortcut-undo = Ontdoen die laaste aksie
shortcut-go-inbox = Inkassie
shortcut-go-starred = Gester
shortcut-go-sent = Gestuur
shortcut-go-drafts = Konsepte
shortcut-go-all = Alle pos
shortcut-search = Soek e-pos
shortcut-navigation = Wys of vou die kieslys
shortcut-quick-settings = Vinnige instellings
shortcut-settings = Alle instellings
shortcut-shortcuts = Kortpadsleutels
shortcut-reload = Kyk vir nuwe e-pos
shortcut-quit = Verlaat

## Keys pressed one after another, as a shortcut shows them ("G then I")

shortcut-sequence = { $first } dan { $second }

## Settings > Accounts

accounts-folder-pane = Vouerpaneel
accounts-folder-pane-detail = Watter rekeninge se vouers die paneel aan die linkerkant wys.
accounts-shown-one = Een rekening op 'n slag; wissel in die rekeningkaart
accounts-shown-all = Alle rekeninge, een ná die ander
accounts-row = Rekeninge
accounts-row-detail = As jy 'n rekening verwyder, word Katna se kopie van sy e-pos op hierdie rekenaar uitgevee. Die e-pos bly op die bediener.
accounts-none = Nog geen rekeninge nie.
accounts-kind-imported = Ingevoer
accounts-picture-reset = Gebruik werkskermprent
accounts-picture-change = Verander prent
accounts-remove = Verwyder
accounts-delete-all-row = Vee alle data uit
accounts-delete-all-row-detail = Begin oor, soos met 'n nuwe installasie.
accounts-delete-all-about = Vee elke rekening, alle gestoorde e-pos, kontakte en kalenders, die soekindeks, jou instellings en gestoorde wagwoorde van hierdie rekenaar uit. Niks verander op jou e-posbedieners nie.
accounts-delete-all-open = Vee alle Katna-data uit

## Settings > Accounts: snackbars after deleting

accounts-removed-local = { $address } is uit Katna verwyder.
accounts-removed = { $address } is uit Katna verwyder. Sy e-pos is steeds op die bediener.
accounts-all-deleted = Alle Katna-data is van hierdie rekenaar uitgevee.

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = Verwyder { $address }?
accounts-remove-confirm = Verwyder rekening
accounts-removing = Verwyder tans…
accounts-remove-local-mail = { $folders ->
    [0] Alle e-pos wat in hierdie rekening ingevoer is
    [one] Alle e-pos wat in hierdie rekening ingevoer is, in sy vouer
   *[other] Alle e-pos wat in hierdie rekening ingevoer is, in sy { $folders } vouers
}
accounts-remove-local-settings = Sy Katna-instellings
accounts-remove-mail = { $folders ->
    [0] Al hierdie rekening se e-pos wat Katna gestoor het
    [one] Al hierdie rekening se e-pos wat Katna in sy vouer gestoor het
   *[other] Al hierdie rekening se e-pos wat Katna in sy { $folders } vouers gestoor het
}
accounts-remove-outbox = Sy boodskappe wat in die uitkassie wag
accounts-remove-settings = Sy gestoorde wagwoord en sy Katna-instellings
accounts-delete-all-title = Vee alle Katna-data uit?
accounts-delete-all-confirm = Vee alles uit
accounts-deleting = Vee tans uit…
accounts-delete-all-accounts = Elke rekening, en alle e-pos en aanhegsels wat Katna gestoor het
accounts-delete-all-contacts = Kontakte, kalenders en die soekindeks
accounts-delete-all-settings = Alle instellings, handtekeninge en kortpadsleutels
accounts-delete-all-passwords = Elke gestoorde wagwoord
accounts-deleted-heading = Van hierdie rekenaar uitgevee:
accounts-cannot-undo = Dit kan nie ontdoen word nie.
accounts-server-delete-all = Niks verander op jou e-posbedieners nie: jou e-pos bly daar, en as jy 'n rekening weer byvoeg, word dit weer afgelaai. E-pos wat uit lêers ingevoer is, is net in Katna; die lêers word nie aangeraak nie.
accounts-server-local = Hierdie e-pos is uit lêers ingevoer, dus het Katna die enigste kopie. Die lêers waaruit dit kom, word nie aangeraak nie; voer hulle weer in om dit terug te kry.
accounts-server-remove = Niks verander op die e-posbediener nie: jou e-pos bly daar, en as jy die rekening weer byvoeg, word dit weer afgelaai.
accounts-confirm-word = skrap
accounts-confirm-placeholder = Tik “{ accounts-confirm-word }”
accounts-confirm-prompt = Tik “{ accounts-confirm-word }” om te bevestig:
accounts-cancel = Kanselleer
