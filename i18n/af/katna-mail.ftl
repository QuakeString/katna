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
