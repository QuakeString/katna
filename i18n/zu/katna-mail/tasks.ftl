# Katna Mail, Zulu (isiZulu): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = Umsebenzi omusha
tasks-all = Yonke imisebenzi
tasks-today = Namuhla
tasks-upcoming = Okuzayo
tasks-starred = Okunenkanyezi
tasks-completed-view = Okuqediwe
tasks-new-list = Dala uhlu olusha
tasks-labels-heading = Amalebula
tasks-on-this-computer = Kule khompyutha
tasks-my-tasks = Imisebenzi yami
# The line under an account in the side list whose task lists could not
# come: why, and the one click that fixes it.
tasks-account-sign-in = Ngena futhi ukuze ubonise imisebenzi
tasks-account-signed-in = Ungene futhi ku-{ $address }. Kutholwa imisebenzi yakho…
tasks-account-sign-in-refused = I-{ $provider } ayizange ivumele i-Katna ingene. Zama futhi, bese uvumela ukufinyelela emisebenzini yakho.
tasks-account-refused = Iseva ayizange yamukele iphasiwedi. I-Yahoo, i-iCloud, i-Zoho nabanye badinga iphasiwedi yohlelo lokusebenza.
tasks-account-change-password = Shintsha iphasiwedi
tasks-account-change-password-tooltip = Thayipha iphasiwedi entsha; i-Katna iyayihlola neseva
tasks-account-not-enabled = Ukufinyelela kwemisebenzi kwe-Katna akukavulwa.
tasks-account-failed = Izinhlu zemisebenzi azikwazanga ukufundwa.
# $reason is the server's own words, in English.
tasks-account-error = Izinhlu zemisebenzi azikwazanga ukufundwa: { $reason }
tasks-account-none = Azikho izinhlu zemisebenzi ezitholakele
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
tasks-account-none-why = Azikho izinhlu zemisebenzi ezitholakele: { $reason }
# A Gmail or Outlook account added with a password: its tasks need the
# provider's sign-in.
tasks-account-use-sign-in = I-{ $provider } ibonisa imisebenzi kuphela ku-Katna engene nge-{ $provider }.
tasks-account-sign-in-with = Ngena nge-{ $provider }
tasks-account-looking = Kufunwa izinhlu zemisebenzi…
tasks-account-try-again = Zama futhi
tasks-account-try-again-tooltip = Hlola imisebenzi yale akhawunti futhi manje
tasks-account-fixing = Kuyasebenzwa kukho…
tasks-list-name-placeholder = Igama loluhlu

## Lists and tasks

tasks-loading = Iyafunda imisebenzi yakho…
tasks-no-lists = Izinhlu zakho zemisebenzi zizovela lapha.
tasks-search = Sesha imisebenzi
tasks-search-none = Ayikho imisebenzi efana nosesho lwakho.
tasks-add = Engeza umsebenzi
tasks-title-placeholder = Isihloko
tasks-add-step = Engeza umsebenzi omncane
tasks-empty = Ayikho imisebenzi okwamanje. Engeza owodwa ngenhla.
tasks-starred-empty = Faka inkanyezi emsebenzini ukuze uwubone lapha.
tasks-label-empty = Ayikho imisebenzi evulekile enale lebula.
tasks-today-empty = Akukho okufanele namuhla.
tasks-completed-empty = Imisebenzi oyiqedayo ivela lapha.
tasks-upcoming-add = Engeza umsebenzi we-{ $day }
tasks-upcoming-overdue-day = { $weekday } { $day }
tasks-from-mail-quiet = Kusuka kumeyili
tasks-from-note-quiet = Kusuka enothini
tasks-steps-done = { $done }/{ $count }
tasks-today-date = { $weekday }, { $day }
tasks-overdue = Okwedlulelwe isikhathi
tasks-completed = { $count ->
    [one] Kuqediwe ({ $count })
   *[other] Kuqediwe ({ $count })
}
tasks-list-options = Izinketho zohlu
tasks-sort-by = Hlunga nge-
tasks-sort-my-order = Ukulandelana kwami
tasks-sort-date = Usuku
tasks-sort-starred = Okufakwe inkanyezi muva nje
tasks-sort-title = Isihloko
tasks-rename-list = Qamba kabusha uhlu
tasks-delete-list = Susa uhlu
tasks-mark-done = Maka njengokuqediwe
tasks-mark-open = Maka njengokungaqediwe
tasks-star = Engeza inkanyezi
tasks-unstar = Susa inkanyezi
tasks-edit-title = Hlela isihloko
tasks-details = Imininingwane
tasks-delete = Susa
tasks-move-to = Hambisa ku-{ $list }
tasks-from-mail = Imeyili
tasks-open-mail = Vula imeyili
tasks-from-note = Inothi
tasks-open-note = Vula inothi
tasks-note-gone = Leli nothi alisekho lapha.
tasks-no-subject = (asikho isihloko)

## Several tasks selected (Ctrl+click, Shift+click)

tasks-selected = { $count ->
    [one] { $count } ukhethiwe
   *[other] { $count } akhethiwe
}
tasks-select-clear = Sula ukukhetha
tasks-select-move = Hambisa ohlwini
tasks-select-date = Setha usuku
tasks-next-week = Iviki elizayo

## The details dialog

tasks-notes-placeholder = Engeza imininingwane
tasks-date = Usuku
tasks-no-date = Alukho usuku
tasks-time-placeholder = Engeza isikhathi
tasks-repeat = Phinda
tasks-repeat-never = Akuphindeki
tasks-repeat-daily = Nsuku zonke
tasks-repeat-weekly = Njalo ngesonto
tasks-repeat-monthly = Njalo ngenyanga
tasks-repeat-yearly = Njalo ngonyaka
tasks-repeat-other = Ngokwezifiso
tasks-remind = Ngikhumbuze
tasks-remind-off = Ungangikhumbuzi
tasks-remind-on-time = Ngesikhathi
tasks-remind-morning = Ngalolo suku, { $time }
tasks-remind-hour-before = Ihora elilodwa ngaphambili
tasks-remind-day-before = Usuku olulodwa ngaphambili
tasks-label-add = Engeza ilebula
tasks-label-task = Faka ilebula emsebenzini
tasks-files-attach = Namathisela amafayela
tasks-files-pick = Namathisela
tasks-file-open = Vula
tasks-file-remove = Susa ifayela
tasks-file-here = Kule khompyutha kuphela
tasks-cancel = Khansela
tasks-save = Londoloza
tasks-not-a-time = “{ $text }” akuyona isikhathi, isibonelo { $example }.

## Due days

tasks-due-today = Namuhla
tasks-due-tomorrow = Kusasa
tasks-due-yesterday = Izolo
tasks-due-at = { $day }, { $time }

## Notes at the bottom

tasks-toast-done = Umsebenzi uqediwe
tasks-toast-next = Kwenziwe. Olandelayo ngomhla ka-{ $date }
tasks-toast-deleted = Umsebenzi ususiwe
tasks-files-added = { $count ->
    [one] Ifayela linamathiselwe
   *[other] Amafayela angu-{ $count } anamathiselwe
}
tasks-file-removed = Kususwe “{ $name }”
tasks-files-left-out = Akunamathiselwe: { $names }. Umsebenzi uthatha amafayela afika ku-{ $limit }, hhayi amafolda.
tasks-file-missing = Lelo fayela alisekho lapha.
tasks-toast-added = { $count ->
    [one] Kwengezwe Kumisebenzi
   *[other] Imisebenzi engu-{ $count } yengeziwe
}
tasks-mail-gone = Leyo meyili ayisekho lapha.
tasks-toast-list-deleted = Uhlu lususiwe
tasks-toast-moved = Kuhanjiswe ku-{ $list }
# A task dragged to another place in its own list.
tasks-toast-placed = Umsebenzi uhanjisiwe
tasks-toast-rescheduled = Umsebenzi uhlelwe kabusha
tasks-toast-rescheduled-several = { $count ->
    [one] Umsebenzi uhlelwe kabusha
   *[other] Imisebenzi engu-{ $count } ihlelwe kabusha
}
tasks-toast-done-several = { $count ->
    [one] Umsebenzi uqediwe
   *[other] Imisebenzi engu-{ $count } iqediwe
}
tasks-toast-open-several = { $count ->
    [one] Umsebenzi umakwe njengongaqediwe
   *[other] Imisebenzi engu-{ $count } imakwe njengengaqediwe
}
tasks-toast-starred = { $count ->
    [one] Umsebenzi ufakwe inkanyezi
   *[other] Imisebenzi engu-{ $count } ifakwe inkanyezi
}
tasks-toast-unstarred = { $count ->
    [one] Inkanyezi isusiwe
   *[other] Izinkanyezi zisusiwe emisebenzini engu-{ $count }
}
tasks-toast-deleted-several = { $count ->
    [one] Umsebenzi ususiwe
   *[other] Imisebenzi engu-{ $count } isusiwe
}
