# Katna Mail, Khmer (ខ្មែរ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = បញ្ចូលគណនីសំបុត្រ
add-account-looking = កំពុងរកម៉ាស៊ីនមេសំបុត្ររបស់ { $address }…
add-account-address-intro = បញ្ចូលអាសយដ្ឋានអ៊ីមែលរបស់អ្នក។ Katna នឹងរកម៉ាស៊ីនមេឱ្យអ្នក។
add-account-servers-title = ការកំណត់ម៉ាស៊ីនមេ
add-account-servers-intro = កន្លែងដែល Katna អាន និងផ្ញើសំបុត្រសម្រាប់ { $address }។
add-account-password-title = បញ្ចូលពាក្យសម្ងាត់របស់អ្នក
add-account-signing-in = កំពុងចូល…

## Add a mail account: fields

add-account-field-address = អាសយដ្ឋានអ៊ីមែល
add-account-incoming = សំបុត្រចូល ({ $protocol })
add-account-outgoing = សំបុត្រចេញ ({ $protocol })
add-account-field-server = ម៉ាស៊ីនមេ
add-account-field-port = ច្រក
add-account-security-none = គ្មាន
add-account-field-username = ឈ្មោះអ្នកប្រើ
add-account-field-password = ពាក្យសម្ងាត់
add-account-show-password = បង្ហាញពាក្យសម្ងាត់
add-account-app-password-hint = { $provider } ត្រូវការពាក្យសម្ងាត់កម្មវិធីនៅទីនេះ មិនមែនពាក្យសម្ងាត់ដែលអ្នកប្រើនៅលើវេបទេ។ បង្កើតមួយនៅក្នុងការកំណត់សុវត្ថិភាពនៃគណនី { $provider } របស់អ្នក។
add-account-field-name = ឈ្មោះរបស់អ្នក (ស្រេចចិត្ត)
add-account-name-hint = បង្ហាញដល់មនុស្សដែលអ្នកសរសេរទៅ។
add-account-servers-pair = { $imap } និង { $smtp }
add-account-servers-found = { $source ->
    [built-in] ម៉ាស៊ីនមេ៖ { $servers } រកឃើញនៅក្នុងបញ្ជីអ្នកផ្ដល់សេវារបស់ Katna។
    [provider] ម៉ាស៊ីនមេ៖ { $servers } រកឃើញនៅក្នុងការកំណត់របស់អ្នកផ្ដល់សេវារបស់អ្នក។
    [ispdb] ម៉ាស៊ីនមេ៖ { $servers } រកឃើញនៅក្នុងបញ្ជីអ្នកផ្ដល់សេវារបស់ Thunderbird។
    [dns] ម៉ាស៊ីនមេ៖ { $servers } រកឃើញនៅក្នុងកំណត់ត្រា DNS នៃដែនរបស់អ្នក។
   *[other] ម៉ាស៊ីនមេ៖ { $servers } ដោយការស្មាន សូមពិនិត្យវា បើការចូលបរាជ័យ។
}
add-account-servers-entered = ម៉ាស៊ីនមេ៖ { $servers } ដូចដែលបានបញ្ចូល។

## Add a mail account: buttons

add-account-servers-button = ការកំណត់ម៉ាស៊ីនមេ
add-account-back = ថយក្រោយ
add-account-add = បញ្ចូលគណនី
add-account-next = បន្ទាប់
add-account-cancel = បោះបង់

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] បញ្ចូលម៉ាស៊ីនមេសំបុត្រចូល។
   *[outgoing] បញ្ចូលម៉ាស៊ីនមេសំបុត្រចេញ។
}
add-account-server-space = { $kind ->
    [incoming] ឈ្មោះម៉ាស៊ីនមេសំបុត្រចូលមានដកឃ្លា។
   *[outgoing] ឈ្មោះម៉ាស៊ីនមេសំបុត្រចេញមានដកឃ្លា។
}
add-account-port-invalid = { $kind ->
    [incoming] ច្រកសំបុត្រចូលត្រូវតែជាលេខពី { $min } ដល់ { $max }។
   *[outgoing] ច្រកសំបុត្រចេញត្រូវតែជាលេខពី { $min } ដល់ { $max }។
}
add-account-address-empty = បញ្ចូលអាសយដ្ឋានអ៊ីមែល។
add-account-address-invalid = បញ្ចូលអាសយដ្ឋានអ៊ីមែលដូចជា { $example }។
add-account-not-found = Katna រកមិនឃើញម៉ាស៊ីនមេសម្រាប់ { $address } ទេ ដូច្នេះវាបានបំពេញឈ្មោះដែលប្រើជាទូទៅ។ សូមពិនិត្យវាជាមួយអ្នកផ្ដល់សេវារបស់អ្នក។
add-account-password-empty = បញ្ចូលពាក្យសម្ងាត់។
add-account-added = បានបញ្ចូល { $address }។ កំពុងទាញយកសំបុត្ររបស់អ្នក…
add-account-app-password-refused = { $provider } បានបដិសេធពាក្យសម្ងាត់។ វាត្រូវការពាក្យសម្ងាត់កម្មវិធី មិនមែនពាក្យសម្ងាត់ដែលអ្នកប្រើនៅលើវេបទេ។
add-account-password-refused = ម៉ាស៊ីនមេបានបដិសេធពាក្យសម្ងាត់។ សូមពិនិត្យវា ហើយព្យាយាមម្ដងទៀត។

## The account menu (from the account button on the top bar)

add-account-menu-another = បញ្ចូលគណនីមួយទៀត
add-account-menu-manage = គ្រប់គ្រងគណនី
