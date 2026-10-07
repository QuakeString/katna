# Katna Mail, Khmer (ខ្មែរ): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = កំណត់ចំណាំ
notes-view-reminders = ការរំលឹក
notes-view-archive = ប័ណ្ណសារ
notes-view-trash = ធុងសម្រាម
notes-edit-labels = កែសម្រួលស្លាក
notes-search = ស្វែងរកកំណត់ចំណាំ
notes-loading = កំពុងបើកកំណត់ចំណាំរបស់អ្នក…

## Board

notes-take-a-note = កត់កំណត់ចំណាំ…
notes-new-list = បញ្ជីថ្មី
notes-new-note = កំណត់ចំណាំថ្មី
notes-pinned = បានខ្ទាស់
notes-others = ផ្សេងទៀត
notes-empty = កំណត់ចំណាំដែលអ្នកបន្ថែមនឹងបង្ហាញនៅទីនេះ
notes-archive-empty = កំណត់ចំណាំដែលបានរក្សាទុកក្នុងប័ណ្ណសារនឹងបង្ហាញនៅទីនេះ
notes-trash-empty = គ្មានកំណត់ចំណាំក្នុងធុងសម្រាម
notes-none-found = រកមិនឃើញកំណត់ចំណាំដែលត្រូវគ្នា
notes-label-empty = មិនទាន់មានកំណត់ចំណាំដែលមានស្លាកនេះនៅឡើយ
notes-reminders-empty = កំណត់ចំណាំដែលមានការរំលឹកខាងមុខ នឹងបង្ហាញនៅទីនេះ
notes-trash-note = កំណត់ចំណាំក្នុងធុងសម្រាមនឹងត្រូវបានលុបបន្ទាប់ពី 7 ថ្ងៃ។
notes-empty-trash = សម្អាតធុងសម្រាម
notes-ticked = { $count ->
   *[other] + ធាតុដែលបានធីក { $count }
}
notes-select = ជ្រើសរើសកំណត់ចំណាំ
notes-selected = { $count ->
   *[other] បានជ្រើសរើស { $count }
}
notes-select-clear = សម្អាតការជ្រើសរើស

## A note's buttons

notes-pin = ខ្ទាស់កំណត់ចំណាំ
notes-unpin = ឈប់ខ្ទាស់កំណត់ចំណាំ
notes-archive = ប័ណ្ណសារ
notes-unarchive = ដកចេញពីប័ណ្ណសារ
notes-delete = លុបកំណត់ចំណាំ
notes-restore = ស្ដារឡើងវិញ
notes-delete-forever = លុបជាអចិន្ត្រៃយ៍
notes-color = ជម្រើសផ្ទៃខាងក្រោយ
notes-checkboxes = បង្ហាញ/លាក់ប្រអប់ធីក
notes-labels = ស្លាក
notes-close = បិទ
notes-more = ច្រើនទៀត
notes-make-copy = ធ្វើច្បាប់ចម្លង
notes-remind = រំលឹកខ្ញុំ
notes-add-picture = បន្ថែមរូបភាព
notes-history = ប្រវត្តិកំណែ
notes-ai = ជួយខ្ញុំសរសេរ
notes-send-as-mail = ផ្ញើជាសំបុត្រ
notes-save-markdown = រក្សាទុកជា Markdown
notes-save-pdf = រក្សាទុកជា PDF

## The open note

notes-title = ចំណងជើង
notes-edited = បានកែនៅ { $date }
notes-on-this-computer = នៅលើកុំព្យូទ័រនេះ
notes-where = កន្លែងរក្សាទុកកំណត់ចំណាំនេះ
notes-untitled = កំណត់ចំណាំគ្មានចំណងជើង

## Pictures

notes-picture-choose = បន្ថែមរូបភាព
notes-picture-remove = ដករូបភាពចេញ
notes-picture-too-big = រូបភាពរហូតដល់ { $size } អាចដាក់ក្នុងកំណត់ចំណាំបាន
notes-picture-kind = ឯកសារនោះមិនមែនជារូបភាពដែល Katna អាចបង្ហាញបានទេ
notes-picture-unreadable = មិនអាចអាន { $name } បានទេ៖ { $error }

## Reminders

notes-remind-me = រំលឹកខ្ញុំ
notes-remind-off = ដកការរំលឹកចេញ
notes-remind-in-the-past = ជ្រើសរើសពេលវេលាដែលមិនទាន់កន្លងផុត
notes-remind-today = ថ្ងៃនេះ, { $time }
notes-remind-tomorrow = ថ្ងៃស្អែក, { $time }
notes-remind-weekday = { $day }, { $time }
notes-reminder-set = បានកំណត់ការរំលឹកសម្រាប់ { $when }
notes-reminder-off = បានដកការរំលឹកចេញ

## Links between notes

notes-link-note = ភ្ជាប់កំណត់ចំណាំ
notes-link-new = កំណត់ចំណាំថ្មី “{ $title }”
notes-linked-from = បានភ្ជាប់ពី
notes-link-gone = កំណត់ចំណាំនោះលែងមាននៅទីនេះទៀតហើយ

## Version history

notes-versions = កំណែ
notes-version-now = ឥឡូវនេះ
notes-version-here = អ្នក នៅលើកុំព្យូទ័រនេះ
notes-version-yesterday = ម្សិលមិញ, { $time }
notes-version-changes = { $count ->
   *[other] ការផ្លាស់ប្ដូរ { $count }
}
notes-version-from = ពី { $device }
notes-version-elsewhere = ពីឧបករណ៍ផ្សេងទៀត
notes-version-created = បានបង្កើត
notes-version-restore = ស្ដារកំណែនេះ
notes-version-restored = បានស្ដារកំណែ
notes-history-none = មិនទាន់មានកំណែមុនៗនៅឡើយ

## AI help

notes-ai-tidy = រៀបចំអត្ថបទឱ្យស្អាត
notes-ai-checklist = ប្ដូរទៅជាបញ្ជីធីក
notes-ai-summarise = សង្ខេប
notes-ai-empty = សូមសរសេរអ្វីមួយជាមុនសិន
notes-ai-tidied = បានរៀបចំអត្ថបទ។ Ctrl+Z ដាក់វាវិញ។
notes-ai-listed = បានប្ដូរទៅជាបញ្ជីធីក។ Ctrl+Z ដាក់វាវិញ។
notes-ai-summarised = បានបន្ថែមសេចក្ដីសង្ខេបនៅខាងលើ

## Labels

notes-label-note = ដាក់ស្លាកលើកំណត់ចំណាំ
notes-label-name = បញ្ចូលឈ្មោះស្លាក
notes-label-create = បង្កើត “{ $name }”
notes-label-remove = ដកស្លាកចេញ
notes-label-delete = លុបស្លាក
notes-labels-none = មិនទាន់មានស្លាកទេ។ បន្ថែមបានពីប៊ូតុងស្លាករបស់កំណត់ចំណាំ។
notes-labels-done = រួចរាល់
notes-label-renamed = បានប្តូរឈ្មោះស្លាកទៅ “{ $name }”
notes-label-deleted = បានលុបស្លាក “{ $name }”

## A note about a mail

notes-mail = សំបុត្រ
notes-open-mail = បើកសំបុត្រ
notes-open-note = បើកកំណត់ចំណាំ

## Meeting notes

notes-meeting-take = កត់ត្រាកំណត់ចំណាំកិច្ចប្រជុំ
notes-meeting-title = { $title } · { $date }
notes-meeting-attendees = អ្នកចូលរួម៖ { $names }
notes-meeting-notes = កំណត់ចំណាំ
notes-meeting-actions = ធាតុសកម្មភាព
notes-event = ព្រឹត្តិការណ៍
notes-open-event = បើកព្រឹត្តិការណ៍

## Formatting

notes-format = ការធ្វើទ្រង់ទ្រាយ
notes-format-heading-1 = ក្បាលអត្ថបទ 1
notes-format-heading-2 = ក្បាលអត្ថបទ 2
notes-format-normal = អត្ថបទធម្មតា
notes-format-bold = ដិត
notes-format-italic = ទ្រេត
notes-format-underline = គូសបន្ទាត់ពីក្រោម
notes-format-quote = សម្រង់
notes-format-code = កូដ
notes-format-divider = បន្ទាត់ខណ្ឌ
notes-format-clear = សម្អាតការធ្វើទ្រង់ទ្រាយ

## Tasks

notes-make-task = ធ្វើជាកិច្ចការ

## Colors (tooltips)

notes-color-none = គ្មានពណ៌
notes-color-coral = ពណ៌ផ្កាថ្ម
notes-color-peach = ពណ៌ប៉េស
notes-color-sand = ពណ៌ខ្សាច់
notes-color-mint = ពណ៌បៃតងខ្ចី
notes-color-sage = ពណ៌បៃតងប្រផេះ
notes-color-fog = ពណ៌អ័ព្ទ
notes-color-storm = ពណ៌ព្យុះ
notes-color-dusk = ពណ៌ព្រលប់
notes-color-blossom = ពណ៌ផ្កា
notes-color-clay = ពណ៌ដីឥដ្ឋ
notes-color-chalk = ពណ៌ដីស

## Messages at the foot of the window

notes-archived = បានរក្សាកំណត់ចំណាំក្នុងប័ណ្ណសារ
notes-unarchived = បានដកកំណត់ចំណាំចេញពីប័ណ្ណសារ
notes-trashed = បានផ្លាស់ទីកំណត់ចំណាំទៅធុងសម្រាម
notes-restored = បានស្ដារកំណត់ចំណាំឡើងវិញ
notes-saved = បានរក្សាទុកកំណត់ចំណាំ
notes-pinned-count = { $count ->
   *[other] បានខ្ទាស់កំណត់ចំណាំ { $count }
}
notes-unpinned-count = { $count ->
   *[other] បានឈប់ខ្ទាស់កំណត់ចំណាំ { $count }
}
notes-colored-count = { $count ->
   *[other] បានប្ដូរពណ៌លើកំណត់ចំណាំ { $count }
}
notes-archived-count = { $count ->
   *[other] បានរក្សាកំណត់ចំណាំ { $count } ក្នុងប័ណ្ណសារ
}
notes-unarchived-count = { $count ->
   *[other] បានដកកំណត់ចំណាំ { $count } ចេញពីប័ណ្ណសារ
}
notes-trashed-count = { $count ->
   *[other] បានផ្លាស់ទីកំណត់ចំណាំ { $count } ទៅធុងសម្រាម
}
notes-restored-count = { $count ->
   *[other] បានស្ដារកំណត់ចំណាំ { $count }
}
notes-copied-count = { $count ->
   *[other] បានធ្វើច្បាប់ចម្លង { $count }
}
notes-empty-discarded = បានលះបង់កំណត់ចំណាំទទេ
notes-mail-gone = សំបុត្រនោះលែងមាននៅទីនេះទៀតហើយ
notes-deleted-forever = { $count ->
   *[other] បានលុបកំណត់ចំណាំ { $count } ជាអចិន្ត្រៃយ៍
}
