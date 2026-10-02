# Katna Mail, Khmer (ខ្មែរ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Accounts

accounts-folder-pane = ផ្ទាំងថត
accounts-folder-pane-detail = ថតរបស់គណនីណាខ្លះ ដែលផ្ទាំងនៅខាងឆ្វេងបង្ហាញ។
accounts-shown-one = គណនីម្ដងមួយ ប្ដូរនៅក្នុងកាតគណនី
accounts-shown-all = គណនីទាំងអស់ មួយបន្ទាប់ពីមួយ
accounts-unified = ប្រអប់ទទួលរួម
accounts-unified-switch = បង្ហាញសំបុត្រនៃគណនីទាំងអស់ជាមួយគ្នា
accounts-unified-switch-detail = “គណនីទាំងអស់” នៅខាងលើគេនៃផ្ទាំងថត ដោយមានប្រអប់ទទួល សំបុត្របានផ្ញើ និងច្រើនទៀតនៃគណនីនីមួយៗ ក្នុងបញ្ជីតែមួយ។ គណនីនៅខាងក្រោមវា ចាប់ផ្ដើមដោយបត់ទុក។
accounts-row = គណនី
accounts-row-detail = ផ្ទាំងថត និងម៉ឺនុយគណនី រាយគណនីតាមលំដាប់នេះ។ គណនីទីមួយជាលំនាំដើម។ ការដកគណនីចេញ លុបច្បាប់ចម្លងសំបុត្ររបស់វាដែល Katna រក្សាទុកនៅលើកុំព្យូទ័រនេះ។ សំបុត្រនៅតែនៅលើម៉ាស៊ីនមេ។
accounts-none = មិនទាន់មានគណនីនៅឡើយទេ។
accounts-pop3-row = សំបុត្រនៅលើម៉ាស៊ីនមេ
accounts-pop3-row-detail = គណនី POP3 ទាញយកសំបុត្រមកកុំព្យូទ័រនេះ។ ជ្រើសរើសថាត្រូវធ្វើអ្វីបន្ទាប់ជាមួយច្បាប់ចម្លងនៅលើម៉ាស៊ីនមេ។
accounts-pop3-with-katna = ទុកវារហូតដល់ខ្ញុំលុបវានៅក្នុង Katna
accounts-pop3-at-once = លុបវាភ្លាមៗពេលទាញយករួច
accounts-pop3-after-days = { $count ->
   *[other] លុបវាបន្ទាប់ពី { $count } ថ្ងៃ
}
accounts-pop3-never = កុំលុបវាឡើយ
accounts-pop3-days-less = តិចថ្ងៃជាង
accounts-pop3-days-more = ច្រើនថ្ងៃជាង
accounts-kind-imported = បាននាំចូល
accounts-picture-reset = ប្រើរូបភាពផ្ទៃតុ
accounts-picture-change = ប្ដូររូបភាព
accounts-picture-remove = ដករូបភាពចេញ
accounts-rename = ប្ដូរឈ្មោះ
accounts-name-save = រក្សាទុក
accounts-name-cancel = បោះបង់
accounts-name-placeholder = ឈ្មោះរបស់អ្នក
accounts-rename-failed = មិនអាចប្ដូរឈ្មោះគណនីបានទេ៖ { $error }
accounts-move-up = ផ្លាស់ទីឡើងលើ
accounts-move-down = ផ្លាស់ទីចុះក្រោម
accounts-drag = អូសដើម្បីប្ដូរលំដាប់
accounts-remove = ដកចេញ
accounts-delete-all-row = លុបទិន្នន័យទាំងអស់
accounts-delete-all-row-detail = ចាប់ផ្ដើមម្ដងទៀត ដូចការដំឡើងថ្មី។
accounts-delete-all-about = លុបគណនីទាំងអស់ សំបុត្រដែលបានរក្សាទុកទាំងអស់ ទំនាក់ទំនង និងប្រតិទិន លិបិក្រមស្វែងរក ការកំណត់របស់អ្នក និងពាក្យសម្ងាត់ដែលបានរក្សាទុក ចេញពីកុំព្យូទ័រនេះ។ គ្មានអ្វីផ្លាស់ប្ដូរនៅលើម៉ាស៊ីនមេសំបុត្ររបស់អ្នកទេ។
accounts-delete-all-open = លុបទិន្នន័យ Katna ទាំងអស់

## Settings > Accounts: snackbars after deleting

accounts-removed-local = { $address } ត្រូវបានដកចេញពី Katna។
accounts-removed = { $address } ត្រូវបានដកចេញពី Katna។ សំបុត្ររបស់វានៅតែនៅលើម៉ាស៊ីនមេ។
accounts-all-deleted = ទិន្នន័យ Katna ទាំងអស់ត្រូវបានលុបចេញពីកុំព្យូទ័រនេះ។

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = ដក { $address } ចេញឬ?
accounts-remove-confirm = ដកគណនីចេញ
accounts-removing = កំពុងដកចេញ…
accounts-remove-local-mail = { $folders ->
    [0] សំបុត្រទាំងអស់ដែលបាននាំចូលទៅក្នុងគណនីនេះ
   *[other] សំបុត្រទាំងអស់ដែលបាននាំចូលទៅក្នុងគណនីនេះ ក្នុងថត { $folders } របស់វា
}
accounts-remove-local-settings = ការកំណត់ Katna របស់វា
accounts-remove-mail = { $folders ->
    [0] សំបុត្រទាំងអស់របស់គណនីនេះ ដែល Katna បានរក្សាទុក
   *[other] សំបុត្រទាំងអស់របស់គណនីនេះ ដែល Katna បានរក្សាទុក ក្នុងថត { $folders } របស់វា
}
accounts-remove-outbox = សាររបស់វាដែលកំពុងរង់ចាំក្នុងប្រអប់ចេញ
accounts-remove-settings = ពាក្យសម្ងាត់ដែលបានរក្សាទុក និងការកំណត់ Katna របស់វា
accounts-delete-all-title = លុបទិន្នន័យ Katna ទាំងអស់ឬ?
accounts-delete-all-confirm = លុបអ្វីៗទាំងអស់
accounts-deleting = កំពុងលុប…
accounts-delete-all-accounts = គណនីទាំងអស់ និងសំបុត្រ និងឯកសារភ្ជាប់ទាំងអស់ដែល Katna បានរក្សាទុក
accounts-delete-all-contacts = ទំនាក់ទំនង ប្រតិទិន និងលិបិក្រមស្វែងរក
accounts-delete-all-settings = ការកំណត់ ហត្ថលេខា និងផ្លូវកាត់ក្ដារចុចទាំងអស់
accounts-delete-all-passwords = ពាក្យសម្ងាត់ដែលបានរក្សាទុកទាំងអស់
accounts-deleted-heading = នឹងត្រូវលុបចេញពីកុំព្យូទ័រនេះ៖
accounts-cannot-undo = សកម្មភាពនេះមិនអាចត្រឡប់វិញបានទេ។
accounts-server-delete-all = គ្មានអ្វីផ្លាស់ប្ដូរនៅលើម៉ាស៊ីនមេសំបុត្ររបស់អ្នកទេ៖ សំបុត្ររបស់អ្នកនៅតែនៅទីនោះ ហើយការបញ្ចូលគណនីម្ដងទៀតនឹងទាញយកវាម្ដងទៀត។ សំបុត្រដែលបាននាំចូលពីឯកសារ មានតែនៅក្នុង Katna ប៉ុណ្ណោះ។ ឯកសារទាំងនោះមិនត្រូវបានប៉ះពាល់ទេ។
accounts-server-local = សំបុត្រនេះត្រូវបាននាំចូលពីឯកសារ ដូច្នេះ Katna មានច្បាប់ចម្លងតែមួយគត់។ ឯកសារដើមមិនត្រូវបានប៉ះពាល់ទេ។ នាំចូលវាម្ដងទៀត ដើម្បីយកវាមកវិញ។
accounts-server-remove = គ្មានអ្វីផ្លាស់ប្ដូរនៅលើម៉ាស៊ីនមេសំបុត្រទេ៖ សំបុត្ររបស់អ្នកនៅតែនៅទីនោះ ហើយការបញ្ចូលគណនីម្ដងទៀតនឹងទាញយកវាម្ដងទៀត។
accounts-confirm-word = លុប
accounts-confirm-placeholder = វាយ “{ accounts-confirm-word }”
accounts-confirm-prompt = ដើម្បីបញ្ជាក់ សូមវាយ “{ accounts-confirm-word }”៖
accounts-cancel = បោះបង់

## Reset cache (Settings > General), in the same dialog

reset-cache-about = លុបសំបុត្រ និងឯកសារភ្ជាប់ដែល Katna បានទាញយក រូបភាពអ្នកផ្ញើ និងលិបិក្រមស្វែងរក រួចទាញយកសំបុត្រថ្មីៗម្ដងទៀត។ គណនី ការកំណត់ និងសំបុត្រដែលមានតែនៅលើកុំព្យូទ័រនេះ នៅដដែល។
reset-cache-button = កំណត់ឃ្លាំងសម្ងាត់ឡើងវិញ
reset-cache-title = កំណត់ឃ្លាំងសម្ងាត់ឡើងវិញឬ?
reset-cache-deleted = ត្រូវលុប រួចទាញយកម្ដងទៀត៖
reset-cache-mail = សំបុត្រ និងឯកសារភ្ជាប់ដែលបានទាញយកពីម៉ាស៊ីនមេ IMAP របស់អ្នក៖ សំបុត្រថ្មីៗទាញយកម្ដងទៀតឥឡូវនេះ សំបុត្រចាស់ជាងនេះនៅពេលអ្នកបើកវា
reset-cache-index = លិបិក្រមស្វែងរក ដែលត្រូវបានបង្កើតឡើងវិញភ្លាមៗ
reset-cache-pictures = រូបភាពអ្នកផ្ញើ
reset-cache-kept = នៅដដែល៖ គណនី ពាក្យសម្ងាត់ និងការកំណត់របស់អ្នក ផ្កាយ ស្លាក សញ្ញាបានអាន និងការខ្ទាស់ សេចក្ដីព្រាង ប្រអប់ចេញ និងការផ្លាស់ប្ដូរដែលមិនទាន់ដល់ម៉ាស៊ីនមេ ព្រមទាំងសំបុត្រពីគណនី POP3 ឬឯកសារដែលបាននាំចូល ដែលប្រហែលជាគ្មានច្បាប់ចម្លងផ្សេងទៀត។ គ្មានអ្វីផ្លាស់ប្ដូរនៅលើម៉ាស៊ីនមេសំបុត្ររបស់អ្នកទេ។
reset-cache-confirm = កំណត់ឃ្លាំងសម្ងាត់ឡើងវិញ
reset-cache-busy = កំពុងកំណត់ឡើងវិញ…
reset-cache-done = ឃ្លាំងសម្ងាត់ត្រូវបានកំណត់ឡើងវិញ។ កំពុងទាញយកសំបុត្រថ្មីៗម្ដងទៀត។
reset-cache-done-freed = ឃ្លាំងសម្ងាត់ត្រូវបានកំណត់ឡើងវិញ ហើយបានទំនេរ { $size }។ កំពុងទាញយកសំបុត្រថ្មីៗម្ដងទៀត។
