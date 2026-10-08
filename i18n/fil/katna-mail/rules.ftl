# Katna Mail, Filipino (Filipino).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Folders & rules

settings-rules = Mga Panuntunan
settings-rules-summary = Kusang ayusin, lagyan ng label, ipasa o patahimikin ang bagong mail
settings-rules-intro = Kusang inaayos ng mga panuntunan ang bagong mail, sa ganitong pagkakasunod. I-drag para ayusin ang pagkakasunod.
settings-rules-all-accounts = Lahat ng account
settings-rules-new = Bagong panuntunan
settings-rules-none = Wala pang panuntunan. Kusang inaayos ng panuntunan ang bagong mail: ayon sa nagpadala, subject o mga salita.
settings-rules-none-account = Wala pang panuntunan para sa account na ito.
settings-rules-drag = I-drag para ayusin ang pagkakasunod
settings-rules-edit = I-edit ang panuntunan
settings-rules-turn-off = I-off ang panuntunang ito
settings-rules-turn-on = I-on ang panuntunang ito

## Starter rules: offered under the user's own rules, switched off.
## Turning one on makes it one of the user's rules.

settings-rules-starters = Mga panimulang panuntunan
settings-rules-starters-intro = Naka-off hanggang i-on mo ang isa. Gumagana ang mga ito para sa lahat ng iyong account; i-edit ang isa para baguhin ito.
settings-rules-starter-turning-on = Ino-on ang “{ $name }”…
settings-rules-starter-failed = Hindi ma-on ang “{ $name }”: { $error }
rules-starter-promotions = Patahimikin ang mga promosyon
rules-starter-newsletters = Mga newsletter sa Babasahin
rules-starter-receipts = Mga resibo at invoice
rules-starter-deliveries = Mga delivery
rules-starter-train = Mga tiket sa tren
rules-starter-flight = Mga tiket sa eroplano
rules-starter-codes = Mga one-time code
rules-starter-security = Mga alerto sa seguridad
rules-starter-social = Social na mail
rules-starter-invites = Mga imbitasyon sa kalendaryo
rules-starter-folder-reading = Babasahin
rules-starter-folder-receipts = Mga Resibo
rules-starter-folder-deliveries = Mga Delivery
rules-starter-folder-travel = Biyahe
rules-starter-folder-social = Social
rules-runs-katna = Tumatakbo sa Katna
rules-runs-gmail = Tumatakbo sa Gmail
rules-runs-sieve = Tumatakbo sa server
rules-stopped = Huminto
rules-error-folder-gone = Wala na ang folder na ginagamit ng panuntunang ito. I-edit ang panuntunan para pumili ng iba.
rules-error-no-archive = Walang archive folder ang account na ito. I-edit ang panuntunan para gumawa ng iba.
rules-error-no-trash = Walang Basurahan ang account na ito. I-edit ang panuntunan para gumawa ng iba.
rules-error-cannot-send = Hindi makapagpadala ng mail ang account na ito, kaya hindi ito maipapasa ng panuntunan.
rules-error-other = { $error }. I-edit ang panuntunan at i-on itong muli.
settings-folders = Mga Folder
settings-folders-summary = Bilang ng hindi pa nabasa sa folder pane
settings-folders-unread-counts = Bilang ng hindi pa nabasa sa bawat folder
settings-folders-unread-counts-detail = Naka-off: Inbox lang ang nagpapakita kung ilan ang hindi pa nabasa

## A rule in one line, on its row: "From contains substack.com → skip the
## inbox, label Reading".

rules-summary = { $when } → { $then }
rules-summary-and = { $first } at { $next }
rules-summary-or = { $first } o { $next }
rules-summary-more = { $count } pa
rules-summary-list = { $first }, { $next }
rules-summary-condition = { $field } { $comparator } { $value }
rules-summary-has-attachment = May attachment
rules-summary-no-attachment = Walang attachment
rules-summary-mailing-list = Mula sa isang mailing list
rules-summary-not-mailing-list = Hindi mula sa isang mailing list
rules-summary-tab = Nasa tab na { $tab }
rules-summary-not-tab = Wala sa tab na { $tab }
rules-summary-move = ilipat sa { $folder }
rules-summary-archive = laktawan ang inbox
rules-summary-trash = ilipat sa basurahan
rules-summary-mark-read = markahan bilang nabasa na
rules-summary-star = lagyan ng star
rules-summary-important = markahan bilang mahalaga
rules-summary-label = lagyan ng label na { $label }
rules-summary-forward = ipasa kay { $address }
rules-summary-dont-notify = huwag mag-abiso
rules-summary-read-after = { $count ->
    [one] markahan bilang nabasa pagkalipas ng { $count } araw
   *[other] markahan bilang nabasa pagkalipas ng { $count } araw
}
rules-summary-folder-gone = isang folder na wala na

## The rule editor

rules-editor-new-title = Bagong panuntunan
rules-editor-edit-title = I-edit ang panuntunan
rules-editor-name-hint = Pangalan ng panuntunan
rules-editor-when = Kapag tumugma ang bagong mail sa
rules-editor-of-these = sa mga ito:
rules-mode-all = lahat
rules-mode-any = alinman
rules-field-from = Mula kay
rules-field-to = Para kay
rules-field-cc = Cc
rules-field-any-recipient = Para kay o Cc
rules-field-reply-to = Reply-to
rules-field-subject = Subject
rules-field-body = Teksto
rules-field-attachment-name = Pangalan ng attachment
rules-field-has-attachment = May attachment
rules-field-mailing-list = Mula sa isang mailing list
rules-field-tab = Tab ng Inbox
rules-comparator-contains = naglalaman ng
rules-comparator-not-contains = hindi naglalaman ng
rules-comparator-begins-with = nagsisimula sa
rules-comparator-ends-with = nagtatapos sa
rules-comparator-equals = eksaktong
rules-comparator-matches = tumutugma sa pattern na
rules-has-yes = oo
rules-has-no = hindi
rules-editor-value-hint = Mga salita o address
rules-editor-add-condition = Magdagdag ng kondisyon
rules-editor-remove = Alisin
rules-editor-then = Pagkatapos:
rules-action-move = Ilipat sa
rules-action-archive = Laktawan ang inbox (i-archive)
rules-action-trash = Ilipat sa basurahan
rules-action-mark-read = Markahan bilang nabasa na
rules-action-star = Lagyan ng star
rules-action-important = Markahan bilang mahalaga
rules-action-label = Magdagdag ng label
rules-action-forward = Ipasa kay
rules-action-dont-notify = Huwag mag-abiso
rules-action-read-after = Markahan bilang nabasa pagkalipas ng
rules-editor-choose-folder = Pumili ng folder
rules-editor-choose-label = Pumili ng label
rules-editor-new-folder = Bago: { $name }
rules-editor-folder-of = { $folder } ({ $account })
rules-editor-forward-hint = Email address
rules-editor-days = araw
rules-editor-add-action = Magdagdag ng aksyon
rules-editor-stop = Huminto rito: hindi na tatakbo sa mail na ito ang mga susunod na panuntunan
rules-editor-accounts = Mga Account:
rules-editor-accounts-none = Pumili ng mga account
rules-editor-accounts-many = { $count ->
    [one] { $count } account
   *[other] { $count } account
}
rules-editor-matches = Tumutugma sa { $mails } mula sa nakaraang { $days } araw
rules-editor-mails = { $count ->
    [one] { $count } mail
   *[other] { $count } mail
}
rules-editor-counting = Binibilang ang mail na tumutugma rito…
rules-editor-show = Ipakita ang mga ito
rules-editor-also-apply = Ilapat din sa { $count } na ito
rules-editor-runs-katna = Tumatakbo sa Katna, habang naka-on ang computer na ito.
rules-editor-runs-gmail = Tumatakbo sa Gmail, kaya gumagana rin ito sa iyong phone at kahit nakapatay ang computer na ito.
rules-editor-runs-sieve = Tumatakbo sa iyong mail server, kaya gumagana rin ito sa iyong phone at kahit nakapatay ang computer na ito.
rules-note-gmail-action = Tumatakbo sa Katna: hindi kayang gawin ng mga filter ng Gmail ang “{ $action }”.
rules-note-sieve-action = Tumatakbo sa Katna: hindi kayang gawin ng mga panuntunan ng iyong mail server ang “{ $action }”.
rules-note-test = { $field } { $comparator }
rules-note-gmail-condition = Tumatakbo sa Katna: hindi kayang suriin ng mga filter ng Gmail ang “{ $test }” gaya ng ginagawa ng Katna.
rules-note-sieve-condition = Tumatakbo sa Katna: hindi kayang suriin ng mga panuntunan ng iyong mail server ang “{ $test }” gaya ng ginagawa ng Katna.
rules-note-order = Tumatakbo sa Katna, gaya ng isang naunang panuntunan ng account: tumatakbo ang mga panuntunan ayon sa pagkakasunod sa listahan.
rules-note-gmail-stop = Tumatakbo sa Katna: hindi kayang pigilan ng mga filter ng Gmail ang pagtakbo ng mga susunod na panuntunan.
rules-note-gmail-forward = Tumatakbo sa Katna: nagpapasa lang ang Gmail sa mga address na na-verify sa mga setting nito, at hindi kabilang ang { $address }.
rules-note-gmail-folder = Tumatakbo sa Katna: walang label ang Gmail para sa isang folder na ginagamit ng panuntunang ito.
rules-note-sieve-folder = Tumatakbo sa Katna: walang folder sa iyong mail server na ginagamit ng panuntunang ito.
rules-note-gmail-sign-in = Tumatakbo sa Katna hanggang mag-sign in ka muli sa Google at payagan ang Katna na gumawa ng mga filter sa Gmail.
rules-note-sieve-other-script = Tumatakbo sa Katna: may ibang rule script (“{ $name }”) na aktibo sa iyong mail server.
rules-note-gmail-failed = Tumatakbo sa Katna: hindi ito tinanggap ng Gmail ({ $error }).
rules-note-sieve-failed = Tumatakbo sa Katna: hindi ito tinanggap ng iyong mail server ({ $error }).
rules-editor-cancel = Kanselahin
rules-editor-save = I-save
rules-editor-saving = Sine-save…
rules-editor-delete = I-delete ang panuntunan
rules-editor-delete-ask = I-delete ang panuntunang ito?
rules-editor-delete-keep = Panatilihin
rules-editor-delete-confirm = I-delete
rules-editor-needs-folder = Pumili ng folder para sa bawat “Ilipat sa” at ng label para sa bawat “Magdagdag ng label”.
rules-editor-needs-days = Ang “Markahan bilang nabasa pagkalipas ng” ay nangangailangan ng bilang ng araw, mula 1 hanggang 3650.
rules-saved = Na-save ang panuntunan
rules-saved-applied = { $count ->
    [one] Na-save ang panuntunan at inilapat sa { $count } mail
   *[other] Na-save ang panuntunan at inilapat sa { $count } mail
}
rules-apply-failed = Na-save ang panuntunan, pero nabigo ang paglalapat nito: { $error }
rules-deleted = Na-delete ang panuntunan
rules-delete-failed = Hindi ma-delete ang panuntunan: { $error }
rules-change-failed = Hindi mabago ang mga panuntunan: { $error }
