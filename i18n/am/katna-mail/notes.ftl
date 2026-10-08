# Katna Mail, Amharic (አማርኛ): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = ማስታወሻዎች
notes-view-reminders = አስታዋሾች
notes-view-archive = ማህደር
notes-view-trash = መጣያ
notes-edit-labels = መሰየሚያዎችን አርትዕ
notes-search = ማስታወሻዎችን ፈልግ
notes-loading = ማስታወሻዎችዎን በመክፈት ላይ…

## Board

notes-take-a-note = ማስታወሻ ይያዙ…
notes-new-list = አዲስ ዝርዝር
notes-new-note = አዲስ ማስታወሻ
notes-pinned = የተሰኩ
notes-others = ሌሎች
notes-empty = የሚያክሏቸው ማስታወሻዎች እዚህ ይታያሉ
notes-archive-empty = በማህደር የተቀመጡ ማስታወሻዎችዎ እዚህ ይታያሉ
notes-trash-empty = በመጣያ ውስጥ ምንም ማስታወሻ የለም
notes-none-found = ተዛማጅ ማስታወሻ አልተገኘም
notes-label-empty = ይህ መሰየሚያ ያላቸው ማስታወሻዎች እስካሁን የሉም
notes-reminders-empty = መጪ አስታዋሾች ያላቸው ማስታወሻዎች እዚህ ይታያሉ
notes-trash-note = በመጣያ ውስጥ ያሉ ማስታወሻዎች ከ7 ቀናት በኋላ ይሰረዛሉ።
notes-empty-trash = መጣያውን ባዶ አድርግ
notes-ticked = { $count ->
    [one] + { $count } ምልክት የተደረገበት ንጥል
   *[other] + { $count } ምልክት የተደረገባቸው ንጥሎች
}
notes-select = ማስታወሻ ምረጥ
notes-selected = { $count ->
    [one] { $count } ተመርጧል
   *[other] { $count } ተመርጠዋል
}
notes-select-clear = ምርጫውን አጽዳ

## A note's buttons

notes-pin = ማስታወሻ ሰካ
notes-unpin = የማስታወሻ ስካታን አንሳ
notes-archive = በማህደር አስቀምጥ
notes-unarchive = ከማህደር አውጣ
notes-delete = ማስታወሻ ሰርዝ
notes-restore = እነበረበት መልስ
notes-delete-forever = ለዘለቄታው ሰርዝ
notes-color = የጀርባ አማራጮች
notes-checkboxes = አመልካች ሳጥኖችን አሳይ ወይም ደብቅ
notes-labels = መሰየሚያዎች
notes-close = ዝጋ
notes-more = ተጨማሪ
notes-make-copy = ቅጂ ፍጠር
notes-remind = አስታውሰኝ
notes-add-picture = ሥዕል አክል
notes-history = የስሪት ታሪክ
notes-ai = እንድጽፍ እርዳኝ
notes-send-as-mail = እንደ ደብዳቤ ላክ
notes-save-markdown = እንደ Markdown አስቀምጥ
notes-save-pdf = እንደ PDF አስቀምጥ

## The open note

notes-title = ርዕስ
notes-edited = የተስተካከለው { $date }
notes-on-this-computer = በዚህ ኮምፒውተር ላይ
notes-where = ይህ ማስታወሻ የሚቀመጥበት
notes-untitled = ርዕስ የሌለው ማስታወሻ

## Pictures

notes-picture-choose = ሥዕሎችን አክል
notes-picture-remove = ሥዕሉን አስወግድ
notes-picture-too-big = እስከ { $size } የሆኑ ሥዕሎች ወደ ማስታወሻ መግባት ይችላሉ
notes-picture-kind = ያ ፋይል Katna ሊያሳየው የሚችል ሥዕል አይደለም
notes-picture-unreadable = { $name }ን ማንበብ አልተቻለም፦ { $error }

## Reminders

notes-remind-me = አስታውሰኝ
notes-remind-off = አስታዋሹን አስወግድ
notes-remind-in-the-past = ገና ያላለፈ ሰዓት ይምረጡ
notes-remind-today = ዛሬ፣ { $time }
notes-remind-tomorrow = ነገ፣ { $time }
notes-remind-weekday = { $day }፣ { $time }
notes-reminder-set = አስታዋሽ ለ{ $when } ተዘጋጅቷል
notes-reminder-off = አስታዋሹ ተወግዷል

## Links between notes

notes-link-note = ማስታወሻ አገናኝ
notes-link-new = አዲስ ማስታወሻ «{ $title }»
notes-linked-from = የተገናኘው ከ
notes-link-gone = ያ ማስታወሻ ከእንግዲህ እዚህ የለም

## Version history

notes-versions = ስሪቶች
notes-version-now = አሁን
notes-version-here = እርስዎ፣ በዚህ ኮምፒውተር ላይ
notes-version-yesterday = ትናንት፣ { $time }
notes-version-changes = { $count ->
    [one] { $count } ለውጥ
   *[other] { $count } ለውጦች
}
notes-version-from = ከ{ $device }
notes-version-elsewhere = ከሌላ መሣሪያ
notes-version-created = ተፈጥሯል
notes-version-restore = ይህን ስሪት መልስ
notes-version-restored = ስሪቱ ተመልሷል
notes-history-none = እስካሁን ቀደምት ስሪቶች የሉም

## AI help

notes-ai-tidy = ጽሑፉን አስተካክል
notes-ai-checklist = ወደ ማረጋገጫ ዝርዝር ቀይረው
notes-ai-summarise = አጠቃልል
notes-ai-empty = መጀመሪያ አንድ ነገር ይጻፉ
notes-ai-tidied = ጽሑፉ ተስተካክሏል። Ctrl+Z ይመልሰዋል።
notes-ai-listed = ወደ ማረጋገጫ ዝርዝር ተቀይሯል። Ctrl+Z ይመልሰዋል።
notes-ai-summarised = ማጠቃለያ ከላይ ታክሏል

## Labels

notes-label-note = ማስታወሻ ሰይም
notes-label-name = የመሰየሚያ ስም ያስገቡ
notes-label-create = “{ $name }” ፍጠር
notes-label-remove = መሰየሚያ አስወግድ
notes-label-delete = መሰየሚያ ሰርዝ
notes-labels-none = እስካሁን መሰየሚያዎች የሉም። ከማስታወሻ የመሰየሚያ አዝራር ያክሉ።
notes-labels-done = ተጠናቋል
notes-label-renamed = መሰየሚያ ወደ “{ $name }” ተቀይሯል
notes-label-deleted = መሰየሚያ “{ $name }” ተሰርዟል

## A note about a mail

notes-mail = ደብዳቤ
notes-open-mail = ደብዳቤውን ክፈት
notes-open-note = ማስታወሻውን ክፈት

## Meeting notes

notes-meeting-take = የስብሰባ ማስታወሻ ያዝ
notes-meeting-title = { $title } · { $date }
notes-meeting-attendees = ተሳታፊዎች፡ { $names }
notes-meeting-notes = ማስታወሻዎች
notes-meeting-actions = የድርጊት ንጥሎች
notes-event = ክስተት
notes-open-event = ክስተቱን ክፈት

## Formatting

notes-format = ቅርጸት
notes-format-heading-1 = ርዕስ 1
notes-format-heading-2 = ርዕስ 2
notes-format-normal = መደበኛ ጽሑፍ
notes-format-bold = ደማቅ
notes-format-italic = ሰያፍ
notes-format-underline = ከስር አስምር
notes-format-quote = ጥቅስ
notes-format-code = ኮድ
notes-format-divider = መለያ መስመር
notes-format-clear = ቅርጸትን አጽዳ

## Tasks

notes-make-task = ተግባር አድርገው

## Colors (tooltips)

notes-color-none = ቀለም የለም
notes-color-coral = ኮራል
notes-color-peach = ፒች
notes-color-sand = አሸዋ
notes-color-mint = ሚንት
notes-color-sage = ሴጅ
notes-color-fog = ጭጋግ
notes-color-storm = ማዕበል
notes-color-dusk = ምሽት
notes-color-blossom = አበባ
notes-color-clay = ሸክላ
notes-color-chalk = ጠመኔ

## Messages at the foot of the window

notes-archived = ማስታወሻ በማህደር ተቀምጧል
notes-unarchived = ማስታወሻ ከማህደር ወጥቷል
notes-trashed = ማስታወሻ ወደ መጣያ ተወስዷል
notes-restored = ማስታወሻ እነበረበት ተመልሷል
notes-saved = ማስታወሻው ተቀምጧል
notes-pinned-count = { $count ->
    [one] ማስታወሻው ተሰክቷል
   *[other] { $count } ማስታወሻዎች ተሰክተዋል
}
notes-unpinned-count = { $count ->
    [one] የማስታወሻው ስካታ ተነስቷል
   *[other] የ{ $count } ማስታወሻዎች ስካታ ተነስቷል
}
notes-colored-count = { $count ->
    [one] ቀለሙ ተቀይሯል
   *[other] በ{ $count } ማስታወሻዎች ላይ ቀለሙ ተቀይሯል
}
notes-archived-count = { $count ->
    [one] ማስታወሻው በማህደር ተቀምጧል
   *[other] { $count } ማስታወሻዎች በማህደር ተቀምጠዋል
}
notes-unarchived-count = { $count ->
    [one] ማስታወሻው ከማህደር ወጥቷል
   *[other] { $count } ማስታወሻዎች ከማህደር ወጥተዋል
}
notes-trashed-count = { $count ->
    [one] ማስታወሻው ወደ መጣያ ተወስዷል
   *[other] { $count } ማስታወሻዎች ወደ መጣያ ተወስደዋል
}
notes-restored-count = { $count ->
    [one] ማስታወሻው ተመልሷል
   *[other] { $count } ማስታወሻዎች ተመልሰዋል
}
notes-copied-count = { $count ->
    [one] ቅጂ ተፈጥሯል
   *[other] { $count } ቅጂዎች ተፈጥረዋል
}
notes-empty-discarded = ባዶ ማስታወሻ ተጥሏል
notes-mail-gone = ያ ደብዳቤ ከእንግዲህ እዚህ የለም
notes-deleted-forever = { $count ->
    [one] ማስታወሻ ለዘለቄታው ተሰርዟል
   *[other] { $count } ማስታወሻዎች ለዘለቄታው ተሰርዘዋል
}
