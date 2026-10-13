# Katna Mail, Igbo (Igbo): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = Ọrụ ọhụrụ
tasks-all = Ọrụ niile
tasks-today = Taa
tasks-upcoming = Na-abịa
tasks-starred = Nwere kpakpando
tasks-completed-view = Emechara
tasks-new-list = Mepụta ndepụta ọhụrụ
tasks-labels-heading = Leebụl
tasks-on-this-computer = Na kọmputa a
tasks-my-tasks = Ọrụ m
# The line under an account in the side list whose task lists could not
# come: why, and the one click that fixes it.
tasks-account-sign-in = Banye ọzọ iji gosi ọrụ
tasks-account-signed-in = Abanyela ọzọ na { $address }. Na-enweta ọrụ gị…
tasks-account-sign-in-refused = { $provider } ekweghị ka Katna banye. Nwaa ọzọ, ma kwe ka ọ nweta ọrụ gị.
tasks-account-refused = Sava ahụ anabataghị okwuntughe ahụ. Yahoo, iCloud, Zoho na ndị ọzọ chọrọ okwuntughe ngwa.
tasks-account-change-password = Gbanwee okwuntughe
tasks-account-change-password-tooltip = Pịnye okwuntughe ọhụrụ; Katna ga-eji sava lelee ya
tasks-account-not-enabled = Agbanyebeghị ohere ọrụ maka Katna.
tasks-account-failed = Enweghị ike ịgụ ndepụta ọrụ.
# $reason is the server's own words, in English.
tasks-account-error = Enweghị ike ịgụ ndepụta ọrụ: { $reason }
tasks-account-none = Ahụghị ndepụta ọrụ ọ bụla
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
tasks-account-none-why = Ahụghị ndepụta ọrụ ọ bụla: { $reason }
# A Gmail or Outlook account added with a password: its tasks need the
# provider's sign-in.
tasks-account-use-sign-in = { $provider } na-egosi ọrụ naanị Katna banyere na { $provider }.
tasks-account-sign-in-with = Banye na { $provider }
tasks-account-looking = Na-achọ ndepụta ọrụ…
tasks-account-try-again = Nwaa ọzọ
tasks-account-try-again-tooltip = Lelee ọrụ akaụntụ a ọzọ ugbu a
tasks-account-fixing = Na-arụ ọrụ na ya…
tasks-list-name-placeholder = Aha ndepụta

## Lists and tasks

tasks-loading = Na-agụ ọrụ gị…
tasks-no-lists = Ndepụta ọrụ gị ga-apụta ebe a.
tasks-search = Chọọ ọrụ
tasks-search-none = Ọ dịghị ọrụ dabara na ọchụchọ gị.
tasks-add = Tinye ọrụ
tasks-title-placeholder = Isiokwu
tasks-add-step = Tinye obere ọrụ
tasks-empty = Enweghị ọrụ ọ bụla ugbu a. Tinye otu n'elu.
tasks-starred-empty = Tinye kpakpando na ọrụ ka ọ pụta ebe a.
tasks-label-empty = Enweghị ọrụ mepere emepe nwere leebụl a.
tasks-today-empty = Enweghị ihe ga-emecha taa.
tasks-completed-empty = Ọrụ ị mechara na-egosi ebe a.
tasks-upcoming-add = Tinye ọrụ maka { $day }
tasks-upcoming-overdue-day = { $weekday } { $day }
tasks-from-mail-quiet = Site n'ozi
tasks-from-note-quiet = Site na ndetu
tasks-steps-done = { $done }/{ $count }
tasks-today-date = { $weekday }, { $day }
tasks-overdue = Gafere oge
tasks-completed = { $count ->
   *[other] Emechara ({ $count })
}
tasks-list-options = Nhọrọ ndepụta
tasks-sort-by = Hazie site na
tasks-sort-my-order = Usoro m
tasks-sort-date = Ụbọchị
tasks-sort-starred = Kpakpando nso nso a
tasks-sort-title = Aha
tasks-rename-list = Gbanwee aha ndepụta
tasks-delete-list = Hichapụ ndepụta
tasks-mark-done = Depụta dị ka emechara
tasks-mark-open = Depụta dị ka emebeghị
tasks-star = Tinye kpakpando
tasks-unstar = Wepụ kpakpando
tasks-edit-title = Dezie isiokwu
tasks-details = Nkọwa
tasks-delete = Hichapụ
tasks-move-to = Kpọga na { $list }
tasks-from-mail = Ozi
tasks-open-mail = Mepee ozi
tasks-from-note = Ndetu
tasks-open-note = Mepee ndetu
tasks-note-gone = Ndetu ahụ anọghịzi ebe a.
tasks-no-subject = (enweghị isiokwu)

## Several tasks selected (Ctrl+click, Shift+click)

tasks-selected = { $count ->
   *[other] Ahọrọla { $count }
}
tasks-select-clear = Kpochapụ nhọrọ
tasks-select-move = Bugharịa gaa na ndepụta
tasks-select-date = Tọọ ụbọchị
tasks-next-week = Izu na-abịa

## The details dialog

tasks-notes-placeholder = Tinye nkọwa
tasks-date = Ụbọchị
tasks-no-date = Enweghị ụbọchị
tasks-time-placeholder = Tinye oge
tasks-repeat = Megharịa
tasks-repeat-never = Anaghị emegharị
tasks-repeat-daily = Kwa ụbọchị
tasks-repeat-weekly = Kwa izu
tasks-repeat-monthly = Kwa ọnwa
tasks-repeat-yearly = Kwa afọ
tasks-repeat-other = Nke onwe
tasks-remind = Chetara m
tasks-remind-off = Echetala m
tasks-remind-on-time = N'oge ahụ
tasks-remind-morning = N'ụbọchị ahụ, { $time }
tasks-remind-hour-before = Otu awa tupu oge ahụ
tasks-remind-day-before = Otu ụbọchị tupu oge ahụ
tasks-label-add = Tinye leebụl
tasks-label-task = Tinye leebụl n'ọrụ
tasks-files-attach = Gbakwunye faịlụ
tasks-files-pick = Gbakwunye
tasks-file-open = Mepee
tasks-file-remove = Wepụ faịlụ
tasks-file-here = Naanị na kọmputa a
tasks-cancel = Kagbuo
tasks-save = Chekwaa
tasks-not-a-time = “{ $text }” abụghị oge, dịka { $example }.

## Due days

tasks-due-today = Taa
tasks-due-tomorrow = Echi
tasks-due-yesterday = Ata
tasks-due-at = { $day }, { $time }

## Notes at the bottom

tasks-toast-done = Emechara ọrụ
tasks-toast-next = Emechara. Nke ọzọ dị na { $date }
tasks-toast-deleted = Ehichapụla ọrụ
tasks-files-added = { $count ->
   *[other] Agbakwunyela faịlụ { $count }
}
tasks-file-removed = Ewepụla “{ $name }”
tasks-files-left-out = Agbakwunyeghị: { $names }. Ọrụ na-anabata faịlụ ruru { $limit }, ọ bụghị folda.
tasks-file-missing = Faịlụ ahụ anọghịzi ebe a.
tasks-toast-added = { $count ->
   *[other] Etinyela ọrụ { $count }
}
tasks-mail-gone = Ozi ahụ anọghịzi ebe a.
tasks-toast-list-deleted = Ehichapụla ndepụta
tasks-toast-moved = Akpọgara na { $list }
# A task dragged to another place in its own list.
tasks-toast-placed = Akpọgara ọrụ
tasks-toast-rescheduled = Agbanwere oge ọrụ
tasks-toast-rescheduled-several = { $count ->
   *[other] Agbanweela ụbọchị ọrụ { $count }
}
tasks-toast-done-several = { $count ->
   *[other] Emechaala ọrụ { $count }
}
tasks-toast-open-several = { $count ->
   *[other] Akaala ọrụ { $count } akara dị ka emechabeghị
}
tasks-toast-starred = { $count ->
   *[other] Etinyela kpakpando n'ọrụ { $count }
}
tasks-toast-unstarred = { $count ->
   *[other] Ewepụla kpakpando n'ọrụ { $count }
}
tasks-toast-deleted-several = { $count ->
   *[other] Ehichapụla ọrụ { $count }
}
