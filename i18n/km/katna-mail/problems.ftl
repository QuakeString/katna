# Katna Mail, Khmer (ខ្មែរ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = ម៉ាស៊ីនមេសំបុត្រ
problems-signed-out = { $provider } បានចាកចេញ Katna ពី { $address }។ សំបុត្របានឈប់ធ្វើសមកាលកម្ម។
problems-password-refused = { $provider } បានបដិសេធពាក្យសម្ងាត់សម្រាប់ { $address }។ វាប្រហែលជាបានផ្លាស់ប្ដូរ។
problems-no-answer = { $provider } មិនឆ្លើយតបសម្រាប់ { $address } ទេ។ Katna នៅតែព្យាយាម។
problems-offline = អ្នកស្ថិតនៅក្រៅបណ្ដាញ។ សំបុត្ររបស់អ្នកនៅតែមាននៅទីនេះ ហើយសំបុត្រដែលអ្នកផ្ញើនឹងរង់ចាំរហូតដល់អ្នកត្រឡប់មកវិញ។
problems-accounts-need-you = { $count ->
   *[other] គណនី { $count } ត្រូវការអ្នក
}
problems-show = បង្ហាញ
problems-later = ពេលក្រោយ
problems-new-password = ពាក្យសម្ងាត់ថ្មី
problems-try-again = ព្យាយាមម្ដងទៀត

## The New password card

problems-password-title = ពាក្យសម្ងាត់ថ្មី
problems-password-detail = { $provider } បានបដិសេធពាក្យសម្ងាត់ដែលបានរក្សាទុកសម្រាប់ { $address }។ វាយពាក្យសម្ងាត់ថ្មី Katna នឹងពិនិត្យវាមុនពេលរក្សាទុក។
problems-password-placeholder = ពាក្យសម្ងាត់
problems-password-show = បង្ហាញពាក្យសម្ងាត់
problems-password-hide = លាក់ពាក្យសម្ងាត់
problems-password-cancel = បោះបង់
problems-password-save = រក្សាទុក
problems-password-checking = កំពុងពិនិត្យ…
problems-password-refused-again = { $provider } ក៏បានបដិសេធពាក្យសម្ងាត់នេះដែរ។ សូមពិនិត្យវា ហើយព្យាយាមម្ដងទៀត។
problems-password-saved = បានរក្សាទុកពាក្យសម្ងាត់សម្រាប់ { $address }។ កំពុងទាញយកសំបុត្ររបស់អ្នក…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = ម៉ាស៊ីនមេសំបុត្ររបស់ { $address } មិនទទួលយកការផ្លាស់ទី{ $count ->
   *[other] សារ { $count } ទេ ដូច្នេះវាបានត្រឡប់ទៅកន្លែងដើមវិញ។
}
problems-refused-flags = ម៉ាស៊ីនមេសំបុត្ររបស់ { $address } មិនទទួលយកការសម្គាល់{ $count ->
   *[other] សារ { $count } (បានអាន ដាក់ផ្កាយ…) ទេ ដូច្នេះវាបានត្រឡប់ដូចដើមវិញ។
}
problems-refused-label = ម៉ាស៊ីនមេសំបុត្ររបស់ { $address } មិនទទួលយកការប្ដូរស្លាករបស់{ $count ->
   *[other] សារ { $count } ទេ ដូច្នេះវាបានត្រឡប់ដូចដើមវិញ។
}
problems-refused-delete = ម៉ាស៊ីនមេសំបុត្ររបស់ { $address } មិនទទួលយកការលុប{ $count ->
   *[other] សារ { $count } ទេ ដូច្នេះវាបានត្រឡប់មកវិញ។
}
problems-refused-other = ម៉ាស៊ីនមេសំបុត្ររបស់ { $address } មិនទទួលយក{ $count ->
   *[other] ការផ្លាស់ប្ដូរ { $count } ទេ ដូច្នេះ Katna បានដាក់វាវិញដូចដើម។
}
problems-details = ព័ត៌មានលម្អិត

## Katna's background service (katna-daemon) isn't running

service-starting = កំពុងចាប់ផ្ដើមសេវាផ្ទៃខាងក្រោយរបស់ Katna…
service-failed = សេវាផ្ទៃខាងក្រោយរបស់ Katna មិនព្រមចាប់ផ្ដើម ដូច្នេះសំបុត្រមិនកំពុងធ្វើសមកាលកម្មទេ។
service-start-again = ចាប់ផ្ដើមម្ដងទៀត
service-started-again = សេវាផ្ទៃខាងក្រោយរបស់ Katna បានឈប់ ហើយត្រូវបានចាប់ផ្ដើមម្ដងទៀត។
service-details-title = មូលហេតុដែលសេវាមិនព្រមចាប់ផ្ដើម
service-details-body = ចម្លងវា ហើយផ្ញើជាមួយរបាយការណ៍របស់អ្នក។ វាគ្មានសំបុត្រ ឬពាក្យសម្ងាត់ក្នុងនោះទេ។
service-details-copy = ចម្លង
service-details-close = បិទ
service-not-running = សេវាផ្ទៃខាងក្រោយរបស់ Katna មិនកំពុងដំណើរការទេ។
service-no-answer = សេវាផ្ទៃខាងក្រោយរបស់ Katna មិនបានឆ្លើយតប៖ { $error }
service-no-session = គ្មានវគ្គ D-Bus ទេ៖ { $error }

## Safe mode: an update left Katna's background service unable to start

safe-line = Katna ស្ថិតក្នុងរបៀបសុវត្ថិភាព ក្រោយមានបញ្ហាជាមួយបច្ចុប្បន្នភាព ដូច្នេះសំបុត្រមិនកំពុងធ្វើសមកាលកម្មទេ។
safe-try-again = ព្យាយាមម្ដងទៀត
safe-restore = ស្ដារឡើងវិញ
safe-restoring = កំពុងស្ដារទិន្នន័យរបស់អ្នកពី { $when }…
safe-restored = បានស្ដារទិន្នន័យរបស់អ្នកពី { $when }។ អ្វីដែលមានពីមុនត្រូវបានរក្សាទុកក្នុងថតមួយ។
safe-show-folder = បង្ហាញថត
safe-restore-failed = មិនអាចស្ដារទិន្នន័យរបស់អ្នកបានទេ៖ { $error }
safe-restore-title = ស្ដារទិន្នន័យរបស់អ្នកពីមុនបច្ចុប្បន្នភាពឬ?
safe-restore-body = Katna ត្រឡប់ទៅច្បាប់ចម្លងដែលអ្នកជ្រើស។ សំបុត្រដែលមកដល់ក្រោយនោះ នឹងទាញយកម្ដងទៀតពីគណនីរបស់អ្នក។
safe-restore-none = មិនទាន់មានច្បាប់ចម្លងនៅឡើយទេ។ Katna បង្កើតច្បាប់ចម្លងមួយ មុនពេលបច្ចុប្បន្នភាពនីមួយៗផ្លាស់ប្ដូរទិន្នន័យរបស់អ្នក។
safe-restore-keep = អ្វីដែលមាននៅពេលនេះ រួមទាំងសំបុត្រមិនទាន់ផ្ញើ សេចក្ដីព្រាង និងការផ្លាស់ប្ដូរដែលមិនទាន់ធ្វើសមកាលកម្ម ត្រូវបានរក្សាទុកក្នុងថតមួយជាមុនសិន ដូច្នេះគ្មានអ្វីបាត់បង់ទេ។
safe-restore-cancel = បោះបង់
safe-restore-mail = សំបុត្រ
safe-restore-pim = គណនី និងទំនាក់ទំនង
safe-restore-blobs = ឯកសារភ្ជាប់
safe-report-title = របាយការណ៍បំបាត់កំហុស
safe-report-body = ចម្លងវា ហើយភ្ជាប់វាទៅរបាយការណ៍កំហុសរបស់អ្នក។ វាគ្មានសំបុត្រ អាសយដ្ឋាន ឬពាក្យសម្ងាត់នៅក្នុងនោះទេ។
safe-report-restore = ស្ដារឡើងវិញ…
safe-report-copied = បានចម្លងរបាយការណ៍បំបាត់កំហុស
