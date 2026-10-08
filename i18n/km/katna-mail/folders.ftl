# Katna Mail, Khmer (ខ្មែរ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Navigation (the folders pane)

nav-labels = ស្លាក
nav-folders = ថត
nav-label-new = បង្កើតស្លាកថ្មី
nav-folder-new = បង្កើតថតថ្មី
nav-menu-check-mail = ពិនិត្យរកសំបុត្រថ្មី
nav-menu-check-inbox = ពិនិត្យប្រអប់ទទួលនេះ
nav-unified-leave-out = មិនដាក់ក្នុងប្រអប់ទទួលរួម
nav-unified-bring-back = ដាក់វិញក្នុងប្រអប់ទទួលរួម
nav-menu-sign-in-again = ចូលម្ដងទៀត
nav-menu-new-mail = សំបុត្រថ្មីពីគណនីនេះ
nav-menu-account-settings = ការកំណត់គណនី
nav-account-checked = បានធ្វើសមកាលកម្ម · បានពិនិត្យ { $ago }
nav-account-in-sync = បានធ្វើសមកាលកម្ម
nav-account-connecting = កំពុងភ្ជាប់…
nav-account-offline = គ្មានអ៊ីនធឺណិត កំពុងសាកម្ដងទៀត
nav-account-signed-out = ការចូល { $provider } បានផុតកំណត់
nav-account-password-refused = ពាក្យសម្ងាត់ត្រូវបានបដិសេធ
nav-account-storage = បានប្រើ { $used } នៃ { $total }
nav-menu-new-subfolder = ថតថ្មីនៅខាងក្នុង
nav-menu-new-sublabel = ស្លាកថ្មីនៅខាងក្នុង
nav-menu-rename = ប្ដូរឈ្មោះ
nav-menu-delete = លុប
nav-menu-empty-trash = សម្អាតធុងសំរាម
nav-account-unnamed = គណនី { $number }
nav-all-accounts = គណនីទាំងអស់
nav-expand = បង្ហាញថត
nav-collapse = លាក់ថត
storage-used = បានប្រើ { $percent }% នៃ { $total }
storage-used-detail = { $address }៖ បានប្រើ { $used } នៃ { $total }

## Special folders (the user's own folders keep their names)

folder-inbox = ប្រអប់ទទួល
folder-starred = មានផ្កាយ
folder-snoozed = បានពន្យារពេល
folder-unread = មិនទាន់អាន
folder-important = សំខាន់
folder-drafts = សេចក្ដីព្រាង
folder-sent = បានផ្ញើ
folder-archive = បណ្ណសារ
folder-spam = សារឥតបានការ
folder-trash = ធុងសំរាម
folder-all-mail = សំបុត្រទាំងអស់
folder-scheduled = បានកំណត់ពេល
folder-waiting = រង់ចាំការឆ្លើយតប
folder-waiting-short = កំពុងរង់ចាំ
folder-reminders = ការរំលឹក
folder-outbox = ប្រអប់ចេញ
folder-activity = សកម្មភាព

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = ស្លាកថ្មី
label-folder-new-title = ថតថ្មី
label-prompt = សូមបញ្ចូលឈ្មោះស្លាកថ្មី៖
label-folder-prompt = សូមបញ្ចូលឈ្មោះថតថ្មី៖
label-name-hint = ឈ្មោះស្លាក
label-folder-name-hint = ឈ្មោះថត
label-nest = ដាក់ស្លាកនៅក្រោម៖
label-folder-nest = ដាក់ថតនៅក្រោម៖
label-cancel = បោះបង់
label-create = បង្កើត
label-creating = កំពុងបង្កើត…
label-created = បានបង្កើតស្លាក “{ $name }”។
label-folder-created = បានបង្កើតថត “{ $name }”។
label-rename-title = ប្ដូរឈ្មោះស្លាក
label-folder-rename-title = ប្ដូរឈ្មោះថត
label-rename = ប្ដូរឈ្មោះ
label-renaming = កំពុងប្ដូរឈ្មោះ…
label-renamed = បានប្ដូរឈ្មោះស្លាកទៅ “{ $name }”។
label-folder-renamed = បានប្ដូរឈ្មោះថតទៅ “{ $name }”។

## Deleting a folder or label (asked first)

folder-delete-title = លុប “{ $name }” ឬ?
folder-delete-body = { $count ->
    [0] វាគ្មានសំបុត្រទេ។ ថតនេះត្រូវបានដកចេញពីម៉ាស៊ីនមេ ដូច្នេះ webmail និងទូរសព្ទរបស់អ្នកក៏បាត់វាដែរ។
   *[other] { $kind ->
        [conversation] { $count ->
            [one] ការសន្ទនា { $count } របស់វាទៅធុងសំរាម ដូច្នេះអ្នកនៅតែអាចយកវាមកវិញបាន។
           *[other] ការសន្ទនា { $count } របស់វាទៅធុងសំរាម ដូច្នេះអ្នកនៅតែអាចយកវាមកវិញបាន។
        }
       *[message] { $count ->
            [one] សារ { $count } របស់វាទៅធុងសំរាម ដូច្នេះអ្នកនៅតែអាចយកវាមកវិញបាន។
           *[other] សារ { $count } របស់វាទៅធុងសំរាម ដូច្នេះអ្នកនៅតែអាចយកវាមកវិញបាន។
        }
    } ថតនេះត្រូវបានដកចេញពីម៉ាស៊ីនមេ ដូច្នេះ webmail និងទូរសព្ទរបស់អ្នកក៏បាត់វាដែរ។
}
folder-delete-forever-body = { $count ->
    [0] វាគ្មានសំបុត្រទេ។ ថតនេះត្រូវបានដកចេញពីម៉ាស៊ីនមេ ដូច្នេះ webmail និងទូរសព្ទរបស់អ្នកក៏បាត់វាដែរ។
   *[other] { $kind ->
        [conversation] { $count ->
            [one] ការសន្ទនា { $count } របស់វានឹងត្រូវលុបជារៀងរហូត ព្រោះគណនីនេះគ្មានធុងសំរាម។
           *[other] ការសន្ទនា { $count } របស់វានឹងត្រូវលុបជារៀងរហូត ព្រោះគណនីនេះគ្មានធុងសំរាម។
        }
       *[message] { $count ->
            [one] សារ { $count } របស់វានឹងត្រូវលុបជារៀងរហូត ព្រោះគណនីនេះគ្មានធុងសំរាម។
           *[other] សារ { $count } របស់វានឹងត្រូវលុបជារៀងរហូត ព្រោះគណនីនេះគ្មានធុងសំរាម។
        }
    } ថតនេះត្រូវបានដកចេញពីម៉ាស៊ីនមេ ដូច្នេះ webmail និងទូរសព្ទរបស់អ្នកក៏បាត់វាដែរ។
}
folder-delete-label-body = ស្លាកនេះត្រូវបានដកចេញ។ សំបុត្ររបស់វានៅតែមានក្នុង សំបុត្រទាំងអស់ និងក្នុងស្លាកផ្សេងទៀតរបស់វា។
folder-delete-confirm = លុបថត
folder-delete-label-confirm = លុបស្លាក
folder-deleted = បានលុបថត “{ $name }”
label-deleted = បានលុបស្លាក “{ $name }”
