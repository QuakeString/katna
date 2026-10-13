# Katna Mail, Tamil (தமிழ்): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = குறிப்புகள்
notes-view-reminders = நினைவூட்டல்கள்
notes-view-archive = காப்பகம்
notes-view-trash = குப்பை
notes-edit-labels = லேபிள்களைத் திருத்து
notes-search = குறிப்புகளைத் தேடு
notes-loading = உங்கள் குறிப்புகளைத் திறக்கிறது…

## Board

notes-take-a-note = குறிப்பு எடு…
notes-new-list = புதிய பட்டியல்
notes-new-note = புதிய குறிப்பு
notes-pinned = பின் செய்தவை
notes-others = மற்றவை
notes-empty = நீங்கள் சேர்க்கும் குறிப்புகள் இங்கே தோன்றும்
notes-archive-empty = நீங்கள் காப்பகப்படுத்திய குறிப்புகள் இங்கே தோன்றும்
notes-trash-empty = குப்பையில் குறிப்புகள் இல்லை
notes-none-found = பொருந்தும் குறிப்புகள் இல்லை
notes-label-empty = இந்த லேபிளில் இதுவரை குறிப்புகள் இல்லை
notes-reminders-empty = வரவிருக்கும் நினைவூட்டல்கள் உள்ள குறிப்புகள் இங்கே தோன்றும்
notes-trash-note = குப்பையில் உள்ள குறிப்புகள் 7 நாட்களுக்குப் பிறகு நீக்கப்படும்.
notes-empty-trash = குப்பையைக் காலிசெய்
notes-ticked = { $count ->
    [one] + { $count } தேர்வுசெய்த உருப்படி
   *[other] + { $count } தேர்வுசெய்த உருப்படிகள்
}
notes-select = குறிப்பைத் தேர்ந்தெடு
notes-selected = { $count ->
    [one] { $count } தேர்ந்தெடுக்கப்பட்டது
   *[other] { $count } தேர்ந்தெடுக்கப்பட்டன
}
notes-select-clear = தேர்வை அழி

## A note's buttons

notes-pin = குறிப்பைப் பின் செய்
notes-unpin = குறிப்பின் பின்னை நீக்கு
notes-archive = காப்பகப்படுத்து
notes-unarchive = காப்பகத்திலிருந்து நீக்கு
notes-delete = குறிப்பை நீக்கு
notes-restore = மீட்டெடு
notes-delete-forever = நிரந்தரமாக நீக்கு
notes-color = பின்னணி நிறம்
notes-checkboxes = தேர்வுப் பெட்டிகளைக் காட்டு அல்லது மறை
notes-labels = லேபிள்கள்
notes-close = மூடு
notes-more = மேலும்
notes-make-copy = நகலை உருவாக்கு
notes-remind = எனக்கு நினைவூட்டு
notes-add-picture = படத்தைச் சேர்
notes-history = பதிப்பு வரலாறு
notes-ai = எழுத உதவு
notes-send-as-mail = அஞ்சலாக அனுப்பு
notes-save-markdown = Markdown ஆகச் சேமி
notes-save-pdf = PDF ஆகச் சேமி

## The open note

notes-title = தலைப்பு
notes-edited = திருத்தியது: { $date }
notes-on-this-computer = இந்தக் கணினியில்
notes-where = இந்தக் குறிப்பு எங்கே சேமிக்கப்பட்டுள்ளது
notes-untitled = தலைப்பில்லாத குறிப்பு

## Pictures

notes-picture-choose = படங்களைச் சேர்
notes-picture-remove = படத்தை அகற்று
notes-picture-too-big = ஒரு குறிப்பில் { $size } வரையிலான படங்களைச் சேர்க்கலாம்
notes-picture-kind = அந்த ஃபைல் Katna காட்டக்கூடிய படம் அல்ல
notes-picture-unreadable = { $name } ஐப் படிக்க முடியவில்லை: { $error }

## Reminders

notes-remind-me = எனக்கு நினைவூட்டு
notes-remind-off = நினைவூட்டலை அகற்று
notes-remind-in-the-past = இன்னும் கடக்காத நேரத்தைத் தேர்ந்தெடுங்கள்
notes-remind-today = இன்று, { $time }
notes-remind-tomorrow = நாளை, { $time }
notes-remind-weekday = { $day }, { $time }
notes-reminder-set = { $when } க்கு நினைவூட்டல் அமைக்கப்பட்டது
notes-reminder-off = நினைவூட்டல் அகற்றப்பட்டது

## Links between notes

notes-link-note = குறிப்பை இணை
notes-link-new = புதிய குறிப்பு "{ $title }"
notes-linked-from = இவற்றிலிருந்து இணைக்கப்பட்டது
notes-link-gone = அந்தக் குறிப்பு இனி இங்கு இல்லை
notes-new-note-gone = புதிய குறிப்பு காணவில்லை.

## Version history

notes-versions = பதிப்புகள்
notes-version-now = இப்போது
notes-version-here = நீங்கள், இந்தக் கணினியில்
notes-version-yesterday = நேற்று, { $time }
notes-version-changes = { $count ->
    [one] { $count } மாற்றம்
   *[other] { $count } மாற்றங்கள்
}
notes-version-from = { $device } இலிருந்து
notes-version-elsewhere = வேறொரு சாதனத்திலிருந்து
notes-version-created = உருவாக்கப்பட்டது
notes-version-restore = இந்தப் பதிப்பை மீட்டெடு
notes-version-restored = பதிப்பு மீட்டெடுக்கப்பட்டது
notes-history-none = இன்னும் முந்தைய பதிப்புகள் இல்லை

## AI help

notes-ai-tidy = உரையைச் சீர்செய்
notes-ai-checklist = சரிபார்ப்புப் பட்டியலாக மாற்று
notes-ai-summarise = சுருக்கு
notes-ai-empty = முதலில் ஏதாவது எழுதுங்கள்
notes-ai-tidied = உரை சீர்செய்யப்பட்டது. Ctrl+Z முன்பிருந்தபடி மாற்றும்.
notes-ai-listed = சரிபார்ப்புப் பட்டியலாக மாற்றப்பட்டது. Ctrl+Z முன்பிருந்தபடி மாற்றும்.
notes-ai-summarised = மேலே சுருக்கம் சேர்க்கப்பட்டது

## Labels

notes-label-note = குறிப்புக்கு லேபிளிடு
notes-label-name = லேபிள் பெயரை உள்ளிடு
notes-label-create = “{ $name }” ஐ உருவாக்கு
notes-label-remove = லேபிளை அகற்று
notes-label-delete = லேபிளை நீக்கு
notes-labels-none = இதுவரை லேபிள்கள் இல்லை. குறிப்பின் லேபிள் பொத்தானிலிருந்து ஒன்றைச் சேர்.
notes-labels-done = முடிந்தது
notes-label-renamed = லேபிளின் பெயர் “{ $name }” என மாற்றப்பட்டது
notes-label-deleted = லேபிள் “{ $name }” நீக்கப்பட்டது

## A note about a mail

notes-mail = அஞ்சல்
notes-open-mail = அஞ்சலைத் திற
notes-open-note = குறிப்பைத் திற

## Meeting notes

notes-meeting-take = கூட்டக் குறிப்பு எடு
notes-meeting-title = { $title } · { $date }
notes-meeting-attendees = பங்கேற்பாளர்கள்: { $names }
notes-meeting-notes = குறிப்புகள்
notes-meeting-actions = செயல் உருப்படிகள்
notes-event = நிகழ்வு
notes-open-event = நிகழ்வைத் திற

## Formatting

notes-format = வடிவமைப்பு
notes-format-heading-1 = தலைப்பு 1
notes-format-heading-2 = தலைப்பு 2
notes-format-normal = இயல்பான உரை
notes-format-bold = தடிமன்
notes-format-italic = சாய்வு
notes-format-underline = அடிக்கோடு
notes-format-quote = மேற்கோள்
notes-format-code = குறியீடு
notes-format-divider = பிரிகோடு
notes-format-clear = வடிவமைப்பை அழி

## Tasks

notes-make-task = பணியாக்கு

## Colors (tooltips)

notes-color-none = நிறம் இல்லை
notes-color-coral = பவளம்
notes-color-peach = பீச்
notes-color-sand = மணல்
notes-color-mint = புதினா
notes-color-sage = சேஜ்
notes-color-fog = பனிமூட்டம்
notes-color-storm = புயல்
notes-color-dusk = அந்தி
notes-color-blossom = மலர்
notes-color-clay = களிமண்
notes-color-chalk = சுண்ணக்கட்டி

## Messages at the foot of the window

notes-archived = குறிப்பு காப்பகப்படுத்தப்பட்டது
notes-unarchived = குறிப்பு காப்பகத்திலிருந்து நீக்கப்பட்டது
notes-trashed = குறிப்பு குப்பைக்கு நகர்த்தப்பட்டது
notes-restored = குறிப்பு மீட்டெடுக்கப்பட்டது
notes-saved = குறிப்பு சேமிக்கப்பட்டது
notes-pinned-count = { $count ->
    [one] குறிப்பு பின் செய்யப்பட்டது
   *[other] { $count } குறிப்புகள் பின் செய்யப்பட்டன
}
notes-unpinned-count = { $count ->
    [one] குறிப்பின் பின் அகற்றப்பட்டது
   *[other] { $count } குறிப்புகளின் பின் அகற்றப்பட்டது
}
notes-colored-count = { $count ->
    [one] வண்ணம் மாற்றப்பட்டது
   *[other] { $count } குறிப்புகளின் வண்ணம் மாற்றப்பட்டது
}
notes-archived-count = { $count ->
    [one] குறிப்பு காப்பகப்படுத்தப்பட்டது
   *[other] { $count } குறிப்புகள் காப்பகப்படுத்தப்பட்டன
}
notes-unarchived-count = { $count ->
    [one] குறிப்பு காப்பகத்திலிருந்து நீக்கப்பட்டது
   *[other] { $count } குறிப்புகள் காப்பகத்திலிருந்து நீக்கப்பட்டன
}
notes-trashed-count = { $count ->
    [one] குறிப்பு நீக்கியவைக்கு நகர்த்தப்பட்டது
   *[other] { $count } குறிப்புகள் நீக்கியவைக்கு நகர்த்தப்பட்டன
}
notes-restored-count = { $count ->
    [one] குறிப்பு மீட்டெடுக்கப்பட்டது
   *[other] { $count } குறிப்புகள் மீட்டெடுக்கப்பட்டன
}
notes-copied-count = { $count ->
    [one] நகல் உருவாக்கப்பட்டது
   *[other] { $count } நகல்கள் உருவாக்கப்பட்டன
}
notes-empty-discarded = காலியான குறிப்பு நிராகரிக்கப்பட்டது
notes-mail-gone = அந்த அஞ்சல் இனி இங்கு இல்லை
notes-deleted-forever = { $count ->
    [one] குறிப்பு நிரந்தரமாக நீக்கப்பட்டது
   *[other] { $count } குறிப்புகள் நிரந்தரமாக நீக்கப்பட்டன
}
