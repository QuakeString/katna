# Katna Mail, Khmer (ខ្មែរ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification (the desktop shows it, even with Katna Mail closed)

notify-new-emails = អ៊ីមែលថ្មី { $count }
notify-and-more = និង { $count } ទៀត
notify-no-subject = (គ្មានប្រធានបទ)
notify-unknown-sender = មិនស្គាល់អ្នកផ្ញើ

## Reminders the user asked for (same buttons)

notify-snooze-back = ត្រឡប់មកពីការពន្យារពេល
notify-no-reply = មិនទាន់មានការឆ្លើយតប
notify-no-reply-to = គ្មាននរណាបានឆ្លើយតប “{ $subject }” ទេ។
notify-follow-up-sent = បានផ្ញើសារតាមដាន
notify-follow-up-sent-to = គ្មាននរណាបានឆ្លើយតប “{ $subject }” ទេ ដូច្នេះ Katna បានផ្ញើសារតាមដាន។
notify-follow-up-waiting = មិនបានផ្ញើសារតាមដាន
notify-follow-up-waiting-to = វាដល់ពេលកំណត់ ខណៈកុំព្យូទ័រនេះបិទ។ “{ $subject }” បានត្រឡប់មកប្រអប់ទទួលរបស់អ្នកវិញ។

## Open and click tracking (only for mail sent with "Track opens and clicks")

notify-tracking-opened = { $who } បានបើក { $subject }
notify-tracking-clicked = { $who } បានចុចតំណក្នុង { $subject }

## An update of Katna is downloaded and ready to install

notify-update-ready = អាចធ្វើបច្ចុប្បន្នភាព Katna Mail បាន
notify-update-ready-body = កំណែ { $version } ត្រូវបានទាញយករួចហើយ។ ចុចធ្វើបច្ចុប្បន្នភាពដើម្បីដំឡើង ហើយចាប់ផ្ដើម Katna Mail ឡើងវិញ។
notify-update = ធ្វើបច្ចុប្បន្នភាព

## Something needs the user, shown once per problem

notify-signed-out = ចូលម្ដងទៀត
notify-signed-out-body = { $provider } បានចាកចេញ Katna ពី { $address }។ សំបុត្របានឈប់ធ្វើសមកាលកម្ម។
notify-sign-in = ចូល
notify-password-refused = ពាក្យសម្ងាត់ត្រូវបានបដិសេធ
notify-password-refused-body = ម៉ាស៊ីនមេសំបុត្របានបដិសេធពាក្យសម្ងាត់សម្រាប់ { $address }។ វាប្រហែលជាបានផ្លាស់ប្ដូរ។
notify-new-password = ពាក្យសម្ងាត់ថ្មី
notify-not-sent = “{ $subject }” មិនបានផ្ញើទេ
notify-not-sent-no-subject = សារមួយមិនបានផ្ញើទេ
notify-not-sent-body = វានៅក្នុងប្រអប់ចេញ ដែលប្រាប់មូលហេតុ។
notify-open-outbox = បើកប្រអប់ចេញ

## Reminders of calendar events

notify-event-now = ឥឡូវនេះ
notify-event-in-minutes = { $count ->
   *[other] ក្នុងរយៈពេល { $count } នាទី
}
notify-event-in-hours = { $count ->
   *[other] ក្នុងរយៈពេល { $count } ម៉ោង
}
notify-event-in-days = { $count ->
    [1] ថ្ងៃស្អែក
   *[other] ក្នុងរយៈពេល { $count } ថ្ងៃ
}
notify-event-all-day = ពេញមួយថ្ងៃ
notify-event-join = ចូលរួម
notify-event-snooze = ពន្យារពេល 5 នាទី
notify-task-done = សម្គាល់ថាបានធ្វើរួច

## The buttons of new-mail notifications and reminders

notify-open = បើក
notify-peek = មើលបន្តិច
notify-reply = ឆ្លើយតប
notify-reply-placeholder = ឆ្លើយតបទៅ { $name }…
notify-send = ផ្ញើ
notify-reply-quote-header = នៅ { $date } { $from } បានសរសេរ៖
notify-reply-quote-header-no-date = { $from } បានសរសេរ៖
notify-reply-all = ឆ្លើយតបទាំងអស់
notify-mark-read = សម្គាល់ថាបានអាន
notify-mark-all-read = សម្គាល់ទាំងអស់ថាបានអាន
notify-archive = ទុកក្នុងបណ្ណសារ
notify-snooze-hour = ពន្យារពេល 1 ម៉ោង
notify-snooze-tomorrow = ថ្ងៃស្អែក
notify-copy-code = ចម្លង { $code }
notify-link-verify = ផ្ទៀងផ្ទាត់នៅ { $domain }
notify-link-confirm = បញ្ជាក់នៅ { $domain }
notify-link-activate = ធ្វើឱ្យសកម្មនៅ { $domain }

## After Archive on a notification: a short note in the same place

notify-archived = បានទុកក្នុងបណ្ណសារ
notify-archived-count = { $count ->
   *[other] សារ { $count } ត្រូវបានផ្លាស់ចេញពីប្រអប់ទទួល
}
notify-undo = មិនធ្វើវិញ

## After Copy on a notification: a short note, with the code under the title

notify-code-copied = បានចម្លងលេខកូដ
notify-code-not-copied = មិនអាចចម្លងលេខកូដបានទេ

## it waits for the undo time

notify-reply-sent = បានផ្ញើការឆ្លើយតបទៅ { $name }
notify-open-in-katna = បើកក្នុង Katna
