# Katna Mail, Sinhala (සිංහල): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = සටහන්
notes-view-reminders = සිහිකැඳවීම්
notes-view-archive = සංරක්ෂිත
notes-view-trash = කුණු කූඩය
notes-edit-labels = ලේබල සංස්කරණය කරන්න
notes-search = සටහන් සොයන්න
notes-loading = ඔබේ සටහන් විවෘත කරමින්…

## Board

notes-take-a-note = සටහනක් ගන්න…
notes-new-list = නව ලැයිස්තුව
notes-new-note = නව සටහන
notes-pinned = අමුණන ලද
notes-others = වෙනත්
notes-empty = ඔබ එක් කරන සටහන් මෙහි දිස්වේ
notes-archive-empty = ඔබේ සංරක්ෂිත සටහන් මෙහි දිස්වේ
notes-trash-empty = කුණු කූඩයේ සටහන් නැත
notes-none-found = ගැළපෙන සටහන් නැත
notes-label-empty = මෙම ලේබලය සහිත සටහන් තවම නැත
notes-reminders-empty = ඉදිරි සිහිකැඳවීම් සහිත සටහන් මෙහි දිස්වේ
notes-trash-note = කුණු කූඩයේ ඇති සටහන් දින 7කට පසු මකා දැමේ.
notes-empty-trash = කුණු කූඩය හිස් කරන්න
notes-ticked = { $count ->
    [one] + ටික් කළ අයිතම { $count }
   *[other] + ටික් කළ අයිතම { $count }
}
notes-select = සටහන තෝරන්න
notes-selected = { $count ->
    [one] { $count }ක් තෝරා ඇත
   *[other] { $count }ක් තෝරා ඇත
}
notes-select-clear = තේරීම හිස් කරන්න

## A note's buttons

notes-pin = සටහන ඉහළට අමුණන්න
notes-unpin = සටහනේ ඇමිණීම ඉවත් කරන්න
notes-archive = සංරක්ෂණය කරන්න
notes-unarchive = සංරක්ෂණයෙන් ඉවත් කරන්න
notes-delete = සටහන මකන්න
notes-restore = ප්‍රතිසාධනය කරන්න
notes-delete-forever = සදහටම මකන්න
notes-color = පසුබිම් වර්ණය
notes-checkboxes = ටික් කොටු පෙන්වන්න හෝ සඟවන්න
notes-labels = ලේබල
notes-close = වසන්න
notes-more = තවත්
notes-make-copy = පිටපතක් සාදන්න
notes-remind = මට මතක් කරන්න
notes-add-picture = පින්තූරයක් එක් කරන්න
notes-history = අනුවාද ඉතිහාසය
notes-ai = ලිවීමට උදවු කරන්න
notes-send-as-mail = තැපැලක් ලෙස යවන්න
notes-save-markdown = Markdown ලෙස සුරකින්න
notes-save-pdf = PDF ලෙස සුරකින්න

## The open note

notes-title = මාතෘකාව
notes-edited = සංස්කරණය කළේ: { $date }
notes-on-this-computer = මෙම පරිගණකයේ
notes-where = මෙම සටහන තබා ඇත්තේ කොහේද
notes-untitled = නම් නොකළ සටහන

## Pictures

notes-picture-choose = පින්තූර එක් කරන්න
notes-picture-remove = පින්තූරය ඉවත් කරන්න
notes-picture-too-big = සටහනකට { $size } දක්වා පින්තූර එක් කළ හැක
notes-picture-kind = එම ගොනුව Katna ට පෙන්විය හැකි පින්තූරයක් නොවේ
notes-picture-unreadable = { $name } කියවිය නොහැකි විය: { $error }

## Reminders

notes-remind-me = මට මතක් කරන්න
notes-remind-off = සිහිකැඳවීම ඉවත් කරන්න
notes-remind-in-the-past = තවම පසු නොවූ වේලාවක් තෝරන්න
notes-remind-today = අද, { $time }
notes-remind-tomorrow = හෙට, { $time }
notes-remind-weekday = { $day }, { $time }
notes-reminder-set = { $when } සඳහා සිහිකැඳවීම සකසා ඇත
notes-reminder-off = සිහිකැඳවීම ඉවත් කළා

## Links between notes

notes-link-note = සටහනක් සම්බන්ධ කරන්න
notes-link-new = නව සටහන “{ $title }”
notes-linked-from = සම්බන්ධ කර ඇත්තේ
notes-link-gone = එම සටහන තවදුරටත් මෙහි නැත

## Version history

notes-versions = අනුවාද
notes-version-now = දැන්
notes-version-here = ඔබ, මෙම පරිගණකයේ
notes-version-yesterday = ඊයේ, { $time }
notes-version-changes = { $count ->
    [one] වෙනස්කම් { $count }
   *[other] වෙනස්කම් { $count }
}
notes-version-from = { $device } වෙතින්
notes-version-elsewhere = වෙනත් උපාංගයකින්
notes-version-created = සාදන ලදී
notes-version-restore = මෙම අනුවාදය ප්‍රතිසාධනය කරන්න
notes-version-restored = අනුවාදය ප්‍රතිසාධනය කළා
notes-history-none = තවම පෙර අනුවාද නැත

## AI help

notes-ai-tidy = පෙළ පිළිවෙළ කරන්න
notes-ai-checklist = පරීක්ෂා ලැයිස්තුවක් බවට පත් කරන්න
notes-ai-summarise = සාරාංශ කරන්න
notes-ai-empty = පළමුව යමක් ලියන්න
notes-ai-tidied = පෙළ පිළිවෙළ කළා. Ctrl+Z එය ආපසු දමයි.
notes-ai-listed = පරීක්ෂා ලැයිස්තුවක් බවට පත් කළා. Ctrl+Z එය ආපසු දමයි.
notes-ai-summarised = සාරාංශය ඉහළින් එක් කළා

## Labels

notes-label-note = සටහන ලේබල් කරන්න
notes-label-name = ලේබලයේ නම ඇතුළු කරන්න
notes-label-create = “{ $name }” සාදන්න
notes-label-remove = ලේබලය ඉවත් කරන්න
notes-label-delete = ලේබලය මකන්න
notes-labels-none = තවම ලේබල නැත. සටහනක ලේබල බොත්තමෙන් එකක් එක් කරන්න.
notes-labels-done = නිමයි
notes-label-renamed = ලේබලයේ නම “{ $name }” ලෙස වෙනස් කරන ලදී
notes-label-deleted = ලේබලය “{ $name }” මකන ලදී

## A note about a mail

notes-mail = තැපැල්
notes-open-mail = තැපැල් විවෘත කරන්න
notes-open-note = සටහන විවෘත කරන්න

## Meeting notes

notes-meeting-take = රැස්වීම් සටහන් ගන්න
notes-meeting-title = { $title } · { $date }
notes-meeting-attendees = සහභාගී වන්නන්: { $names }
notes-meeting-notes = සටහන්
notes-meeting-actions = කළ යුතු දේ
notes-event = සිදුවීම
notes-open-event = සිදුවීම විවෘත කරන්න

## Formatting

notes-format = හැඩතල ගැන්වීම
notes-format-heading-1 = ශීර්ෂය 1
notes-format-heading-2 = ශීර්ෂය 2
notes-format-normal = සාමාන්‍ය පෙළ
notes-format-bold = තද
notes-format-italic = ඇල
notes-format-underline = යටි ඉර
notes-format-quote = උපුටනය
notes-format-code = කේතය
notes-format-divider = බෙදුම් රේඛාව
notes-format-clear = හැඩතල ගැන්වීම ඉවත් කරන්න

## Tasks

notes-make-task = කාර්යයක් කරන්න

## Colors (tooltips)

notes-color-none = වර්ණයක් නැත
notes-color-coral = පබළු
notes-color-peach = පීච්
notes-color-sand = වැලි
notes-color-mint = මින්ට්
notes-color-sage = සේජ්
notes-color-fog = මීදුම
notes-color-storm = කුණාටුව
notes-color-dusk = සැන්දෑව
notes-color-blossom = මල්
notes-color-clay = මැටි
notes-color-chalk = චොක්

## Messages at the foot of the window

notes-archived = සටහන සංරක්ෂණය කරන ලදී
notes-unarchived = සටහන සංරක්ෂණයෙන් ඉවත් කරන ලදී
notes-trashed = සටහන කුණු කූඩයට ගෙන යන ලදී
notes-restored = සටහන ප්‍රතිසාධනය කරන ලදී
notes-saved = සටහන සුරැකුණා
notes-pinned-count = { $count ->
    [one] සටහන ඉහළට ඇමිණුවා
   *[other] සටහන් { $count } ඉහළට ඇමිණුවා
}
notes-unpinned-count = { $count ->
    [one] සටහනේ ඇමිණීම ඉවත් කළා
   *[other] සටහන් { $count } ක ඇමිණීම ඉවත් කළා
}
notes-colored-count = { $count ->
    [one] වර්ණය වෙනස් කළා
   *[other] සටහන් { $count } ක වර්ණය වෙනස් කළා
}
notes-archived-count = { $count ->
    [one] සටහන සංරක්ෂණය කළා
   *[other] සටහන් { $count } සංරක්ෂණය කළා
}
notes-unarchived-count = { $count ->
    [one] සටහන සංරක්ෂණයෙන් ඉවත් කළා
   *[other] සටහන් { $count } සංරක්ෂණයෙන් ඉවත් කළා
}
notes-trashed-count = { $count ->
    [one] සටහන කුණු කූඩයට ගෙන ගියා
   *[other] සටහන් { $count } කුණු කූඩයට ගෙන ගියා
}
notes-restored-count = { $count ->
    [one] සටහන ප්‍රතිසාධනය කළා
   *[other] සටහන් { $count } ප්‍රතිසාධනය කළා
}
notes-copied-count = { $count ->
    [one] පිටපතක් සෑදුවා
   *[other] පිටපත් { $count } සෑදුවා
}
notes-empty-discarded = හිස් සටහන ඉවත දමන ලදී
notes-mail-gone = එම තැපැල් තවදුරටත් මෙහි නැත
notes-deleted-forever = { $count ->
    [one] සටහන සදහටම මකා දමන ලදී
   *[other] සටහන් { $count }ක් සදහටම මකා දමන ලදී
}
