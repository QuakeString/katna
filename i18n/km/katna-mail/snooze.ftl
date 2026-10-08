# Katna Mail, Khmer (ខ្មែរ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The snooze menu (the clock on a line, or Snooze in the right-click menu)

snooze-until = ពន្យារពេលរហូតដល់…
snooze-later-today = ក្រោយបន្តិចទៀតនៅថ្ងៃនេះ
snooze-tomorrow = ថ្ងៃស្អែក
snooze-this-weekend = ចុងសប្ដាហ៍នេះ
snooze-next-week = សប្ដាហ៍ក្រោយ
snooze-pick = ជ្រើសរើសកាលបរិច្ឆេទ និងម៉ោង
snooze-back = ត្រឡប់ទៅពេលវេលា
snooze-type-placeholder = វាយពេលវេលា
snooze-type-hint = ដូចជា “tue 3pm”, “tomorrow” ឬ “in 2 hours”
snooze-type-hint-unclear = Katna មិនអាចអានវាជាពេលវេលាបានទេ
snooze-type-unclear = “{ $text }” មិនមែនជាពេលវេលាដែល Katna ស្គាល់ទេ

## Snooze and Remind me on mail: one menu, switched at its top (B and H)

snooze-tab = ពន្យារពេល
remind-tab = រំលឹកខ្ញុំ
snooze-says = លាក់វារហូតដល់ពេលនោះ
remind-says = ទុកវានៅកន្លែងដដែល ហើយជូនដំណឹងអ្នក
remind-before-due = មុនពេលផុតកំណត់
remind-note = កំណត់ចំណាំ (ស្រេចចិត្ត)
remind-note-placeholder = ប្រធានបទ បើទុកទទេ
toast-remind-set = បានកំណត់ការរំលឹកសម្រាប់ { $date }
remind-chat-line = រំលឹក { $date } · { $title }
remind-done = រួចរាល់
toast-remind-done = ការរំលឹកបានរួចរាល់
snooze-chat-line = បានពន្យារពេលរហូតដល់ { $date }
snooze-chat-change = ប្ដូរ

## The date and time picker

snooze-cancel = បោះបង់
snooze-save = រក្សាទុក
snooze-in-the-past = ជ្រើសរើសម៉ោងក្រោយពេលឥឡូវនេះ។

## beside Send

follow-up-menu = តាមដាន បើគ្មានការឆ្លើយតប…
follow-up-title = តាមដាន បើគ្មានការឆ្លើយតប
follow-up-off = បិទ
follow-up-days = { $days ->
   *[other] { $days } ថ្ងៃ
}
follow-up-weeks = { $weeks ->
   *[other] { $weeks } សប្ដាហ៍
}
follow-up-pick = ជ្រើសរើស…
follow-up-pick-title = តាមដាន បើគ្មានការឆ្លើយតបត្រឹម
follow-up-remind = រំលឹកខ្ញុំ
follow-up-remind-note = ការសន្ទនាត្រឡប់មកខាងលើប្រអប់ទទួលរបស់អ្នកវិញ
follow-up-send = ផ្ញើសារតាមដានជំនួសខ្ញុំ
follow-up-send-note = ទៅមនុស្សដដែល ក្នុងការសន្ទនាដដែល
follow-up-send-encrypted = មិនសម្រាប់សំបុត្រដែលបានអ៊ិនគ្រីបទេ
follow-up-text-placeholder = អ្វីដែលត្រូវសរសេរ
follow-up-text-named = សួស្ដី { $name } គ្រាន់តែសួរថា តើអ្នកបានឃើញសាររបស់ខ្ញុំខាងក្រោមហើយឬនៅ។
follow-up-text = សួស្ដី គ្រាន់តែសួរថា តើអ្នកបានឃើញសាររបស់ខ្ញុំខាងក្រោមហើយឬនៅ។
follow-up-template = ប្រើគំរូ
follow-up-signature = ហត្ថលេខារបស់អ្នកត្រូវបានបន្ថែម
follow-up-again = បើនៅតែគ្មានការឆ្លើយតប តាមដានម្ដងទៀតបន្ទាប់ពី
follow-up-note = ឈប់ភ្លាមៗ ពេលនរណាម្នាក់ក្នុងការសន្ទនាឆ្លើយតប។ ការឆ្លើយតបស្វ័យប្រវត្តិមិនរាប់ទេ។
follow-up-note-send = ឈប់ភ្លាមៗ ពេលនរណាម្នាក់ក្នុងការសន្ទនាឆ្លើយតប។ ផ្ញើចេញនៅថ្ងៃធ្វើការ ពី { $start } ដល់ { $end } ហើយមិនយឺតលើសពីមួយថ្ងៃទេ។
follow-up-cancel = បោះបង់
follow-up-done = រួចរាល់
follow-up-chip-send = តាមដានក្នុងរយៈពេល { $time }
follow-up-chip-remind = រំលឹកក្នុងរយៈពេល { $time }
follow-up-chip-send-on = តាមដាន { $date }
follow-up-chip-remind-on = រំលឹក { $date }

## The card over an open conversation a follow-up waits on.

follow-up-card-title = មិនទាន់មានការឆ្លើយតប
follow-up-card-title-waiting = សារតាមដានរបស់អ្នកកំពុងរង់ចាំ
follow-up-card-send = Katna នឹងផ្ញើសារតាមដានរបស់អ្នកនៅ { $date }។ វាឈប់ពេលនរណាម្នាក់ឆ្លើយតប។
follow-up-card-send-twice = Katna នឹងផ្ញើសារតាមដានរបស់អ្នកនៅ { $date } រួចម្ដងទៀតនៅពេលក្រោយ។ វាឈប់ពេលនរណាម្នាក់ឆ្លើយតប។
follow-up-card-remind = បើគ្មាននរណាឆ្លើយតប ការសន្ទនានេះនឹងត្រឡប់មកប្រអប់ទទួលរបស់អ្នកវិញនៅ { $date }។
follow-up-card-waiting = វាដល់ពេលកំណត់ ខណៈកុំព្យូទ័ររបស់អ្នកបិទ ដូច្នេះវាមិនត្រូវបានផ្ញើយឺតទេ។ ផ្ញើវាឥឡូវនេះ ជ្រើសពេលថ្មី ឬបញ្ឈប់វា។
follow-up-card-edit = កែសម្រួល
follow-up-card-edit-title = តាមដាននៅ
follow-up-card-send-now = ផ្ញើឥឡូវនេះ
follow-up-card-stop = បញ្ឈប់
follow-up-chat-send = តាមដាន · { $date } បើគ្មាននរណាឆ្លើយតប
follow-up-chat-step = ការតាមដាន { $step } នៃ { $steps } · { $date } បើគ្មាននរណាឆ្លើយតប
follow-up-chat-waiting = ការតាមដានកំពុងរង់ចាំ · វាដល់ពេលកំណត់ ខណៈកុំព្យូទ័ររបស់អ្នកបិទ
follow-up-chat-remind = ត្រឡប់មកប្រអប់ទទួល { $date } បើគ្មានការឆ្លើយតប
toast-follow-up-sent = បានផ្ញើសារតាមដាន
toast-follow-up-stopped = បានបញ្ឈប់ការតាមដាន
toast-follow-up-moved = បានប្ដូរការតាមដានទៅ { $date }
nudge-row = បានផ្ញើ { $days ->
   *[other] { $days } ថ្ងៃមុន
}។ តាមដានទេ?
nudge-row-tip = សរសេរសារតាមដានទៅគ្រប់គ្នាក្នុងការសន្ទនា
nudge-follow-up = តាមដាន
nudge-dismiss = បដិសេធ
nudge-card-title = មិនទាន់មានការឆ្លើយតប
nudge-card-text = អ្នកបានសួរអ្វីមួយ { $days ->
   *[other] { $days } ថ្ងៃមុន
} ហើយគ្មាននរណាឆ្លើយទេ។
nudge-chat-line = បានផ្ញើ { $days ->
   *[other] { $days } ថ្ងៃមុន
} មិនទាន់មានការឆ្លើយតប
toast-nudge-dismissed = បានបដិសេធការរំលឹក
