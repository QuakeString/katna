# Katna Mail, Khmer (ខ្មែរ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The tray icon's menu

tray-open-inbox = _បើកប្រអប់ទទួល
tray-new-message = _សារថ្មី
tray-new-task = _កិច្ចការថ្មី
tray-new-note = _កំណត់ចំណាំថ្មី
tray-preferences = _ការកំណត់
tray-quit = _ចាកចេញ

## The tray icon's tooltip, under "Katna Mail"

tray-unread = { $count ->
    [0] គ្មានសារមិនទាន់អានទេ
   *[other] សារមិនទាន់អាន { $count }
}
tray-password-refused = ត្រូវការពាក្យសម្ងាត់ថ្មីសម្រាប់ { $address }
tray-signed-out = ចូល { $address } ម្ដងទៀត
tray-accounts-need-you = គណនី { $count } ត្រូវការអ្នក
tray-not-sent = { $count ->
   *[other] សារ { $count } មិនបានផ្ញើទេ
}
