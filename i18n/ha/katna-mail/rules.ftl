# Katna Mail, Hausa (Hausa).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Folders & rules

settings-rules = Ƙa'idoji
settings-rules-summary = Tsara, sanya lakabi, tura ko yi shiru da sababbin wasiƙu da kansu
settings-rules-intro = Ƙa'idoji suna tsara sababbin wasiƙu da kansu, a wannan tsari. Ja don sake tsarawa.
settings-rules-all-accounts = Duk asusu
settings-rules-new = Sabuwar ƙa'ida
settings-rules-none = Babu ƙa'idoji tukuna. Ƙa'ida tana tsara sababbin wasiƙu da kanta: ta mai aikawa, jigo ko kalmomi.
settings-rules-none-account = Babu ƙa'idoji ga wannan asusun tukuna.
settings-rules-drag = Ja don sake tsarawa
settings-rules-edit = Gyara ƙa'ida
settings-rules-turn-off = Kashe wannan ƙa'ida
settings-rules-turn-on = Kunna wannan ƙa'ida

## Starter rules: offered under the user's own rules, switched off.
## Turning one on makes it one of the user's rules.

settings-rules-starters = Ƙa'idojin farawa
settings-rules-starters-intro = A kashe suke har sai kun kunna ɗaya. Suna aiki ga duk asusunku; ku gyara ɗaya don canza ta.
settings-rules-starter-turning-on = Ana kunna “{ $name }”…
settings-rules-starter-failed = Ba a iya kunna “{ $name }” ba: { $error }
rules-starter-promotions = Yi shiru da tallace-tallace
rules-starter-newsletters = Wasiƙun labarai zuwa Karatu
rules-starter-receipts = Rasidai da takardun biya
rules-starter-deliveries = Isar da kaya
rules-starter-train = Tikitin jirgin ƙasa
rules-starter-flight = Tikitin jirgin sama
rules-starter-codes = Lambobin amfani sau ɗaya
rules-starter-security = Faɗakarwar tsaro
rules-starter-social = Wasiƙun zamantakewa
rules-starter-invites = Gayyatar kalanda
rules-starter-folder-reading = Karatu
rules-starter-folder-receipts = Rasidai
rules-starter-folder-deliveries = Isar da kaya
rules-starter-folder-travel = Tafiya
rules-starter-folder-social = Zamantakewa
rules-runs-katna = Yana aiki a Katna
rules-runs-gmail = Yana aiki a Gmail
rules-runs-sieve = Yana aiki a sabar
rules-stopped = An tsayar
rules-error-folder-gone = Foldar da wannan ƙa'ida ke amfani da ita babu ita kuma. Ku gyara ƙa'idar don zaɓar wata.
rules-error-no-archive = Wannan asusun ba shi da foldar ma'ajiya. Ku gyara ƙa'idar don yin wani abu dabam.
rules-error-no-trash = Wannan asusun ba shi da Kwandon shara. Ku gyara ƙa'idar don yin wani abu dabam.
rules-error-cannot-send = Wannan asusun ba zai iya aika wasiƙa ba, don haka ƙa'idar ba za ta iya tura ta ba.
rules-error-other = { $error }. Ku gyara ƙa'idar ku sake kunna ta.

settings-folders = Folda
settings-folders-summary = Adadin waɗanda ba a karanta ba a sashen folda
settings-folders-unread-counts = Adadin waɗanda ba a karanta ba a kowace folda
settings-folders-unread-counts-detail = A kashe: Akwatin saƙo kaɗai ke nuna yawan waɗanda ba a karanta ba

## A rule in one line, on its row: "From contains substack.com → skip the
## inbox, label Reading".

rules-summary = { $when } → { $then }
rules-summary-and = { $first } da { $next }
rules-summary-or = { $first } ko { $next }
rules-summary-more = ƙarin { $count }
rules-summary-list = { $first }, { $next }
rules-summary-condition = { $field } { $comparator } { $value }
rules-summary-has-attachment = Yana da abin haɗawa
rules-summary-no-attachment = Ba shi da abin haɗawa
rules-summary-mailing-list = Daga jerin aikawa
rules-summary-not-mailing-list = Ba daga jerin aikawa ba
rules-summary-tab = A shafin { $tab }
rules-summary-not-tab = Ba a shafin { $tab } ba
rules-summary-move = matsar zuwa { $folder }
rules-summary-archive = tsallake akwatin saƙo
rules-summary-trash = matsar zuwa kwandon shara
rules-summary-mark-read = yi alama an karanta
rules-summary-star = sanya tauraro
rules-summary-important = yi alama muhimmi
rules-summary-label = sanya lakabi { $label }
rules-summary-forward = tura zuwa { $address }
rules-summary-dont-notify = kada a sanar
rules-summary-read-after = { $count ->
    [one] yi alama an karanta bayan kwana { $count }
   *[other] yi alama an karanta bayan kwanaki { $count }
}
rules-summary-folder-gone = foldar da ta ɓace

## The rule editor

rules-editor-new-title = Sabuwar ƙa'ida
rules-editor-edit-title = Gyara ƙa'ida
rules-editor-name-hint = Sunan ƙa'ida
rules-editor-when = Idan sabuwar wasiƙa ta dace da
rules-editor-of-these = na waɗannan:
rules-mode-all = duka
rules-mode-any = kowanne
rules-field-from = Daga
rules-field-to = Zuwa
rules-field-cc = Cc
rules-field-any-recipient = Zuwa ko Cc
rules-field-reply-to = Amsa-zuwa
rules-field-subject = Jigo
rules-field-body = Rubutu
rules-field-attachment-name = Sunan abin haɗawa
rules-field-has-attachment = Yana da abin haɗawa
rules-field-mailing-list = Daga jerin aikawa
rules-field-tab = Shafin akwatin saƙo
rules-comparator-contains = yana ƙunshe da
rules-comparator-not-contains = ba ya ƙunshe da
rules-comparator-begins-with = yana farawa da
rules-comparator-ends-with = yana ƙarewa da
rules-comparator-equals = daidai yake da
rules-comparator-matches = ya dace da tsarin
rules-has-yes = eh
rules-has-no = a'a
rules-editor-value-hint = Kalmomi ko adireshi
rules-editor-add-condition = Ƙara sharaɗi
rules-editor-remove = Cire
rules-editor-then = Sannan:
rules-action-move = Matsar zuwa
rules-action-archive = Tsallake akwatin saƙo (ma'ajiya)
rules-action-trash = Matsar zuwa kwandon shara
rules-action-mark-read = Yi alama an karanta
rules-action-star = Sanya tauraro
rules-action-important = Yi alama muhimmi
rules-action-label = Ƙara lakabi
rules-action-forward = Tura zuwa
rules-action-dont-notify = Kada a sanar
rules-action-read-after = Yi alama an karanta bayan
rules-editor-choose-folder = Zaɓi folda
rules-editor-choose-label = Zaɓi lakabi
rules-editor-new-folder = Sabo: { $name }
rules-editor-folder-of = { $folder } ({ $account })
rules-editor-forward-hint = Adireshin imel
rules-editor-days = kwanaki
rules-editor-add-action = Ƙara aiki
rules-editor-stop = Tsaya a nan: ƙa'idojin baya ba za su yi aiki a kan wannan wasiƙa ba
rules-editor-accounts = Asusu:
rules-editor-accounts-none = Zaɓi asusu
rules-editor-accounts-many = { $count ->
    [one] asusu { $count }
   *[other] asusu { $count }
}
rules-editor-matches = Ya dace da { $mails } daga kwanaki { $days } da suka wuce
rules-editor-mails = { $count ->
    [one] wasiƙa { $count }
   *[other] wasiƙu { $count }
}
rules-editor-counting = Ana ƙirga wasiƙun da ta dace da su…
rules-editor-show = Nuna su
rules-editor-also-apply = Yi amfani da ita a kan waɗannan { $count } ma
rules-editor-runs-katna = Yana aiki a Katna, yayin da wannan kwamfuta ke kunne.
rules-editor-runs-gmail = Yana aiki a Gmail, don haka yana aiki a wayarku ma kuma idan wannan kwamfuta a kashe take.
rules-editor-runs-sieve = Yana aiki a sabar wasiƙarku, don haka yana aiki a wayarku ma kuma idan wannan kwamfuta a kashe take.
rules-note-gmail-action = Yana aiki a Katna: matatun Gmail ba za su iya “{ $action }” ba.
rules-note-sieve-action = Yana aiki a Katna: ƙa'idojin sabar wasiƙarku ba za su iya “{ $action }” ba.
rules-note-test = { $field } { $comparator }
rules-note-gmail-condition = Yana aiki a Katna: matatun Gmail ba za su iya gwada “{ $test }” kamar yadda Katna ke yi ba.
rules-note-sieve-condition = Yana aiki a Katna: ƙa'idojin sabar wasiƙarku ba za su iya gwada “{ $test }” kamar yadda Katna ke yi ba.
rules-note-order = Yana aiki a Katna, kamar yadda wata ƙa'idar asusun ta baya ke yi: ƙa'idoji suna aiki bisa tsarin jeri.
rules-note-gmail-stop = Yana aiki a Katna: matatun Gmail ba za su iya hana ƙa'idojin baya aiki ba.
rules-note-gmail-forward = Yana aiki a Katna: Gmail tana tura wa adireshin da aka tabbatar a saitunanta kaɗai, kuma { $address } ba ɗaya daga cikinsu ba ne.
rules-note-gmail-folder = Yana aiki a Katna: Gmail ba ta da lakabi ga wata folda da wannan ƙa'ida ke amfani da ita.
rules-note-sieve-folder = Yana aiki a Katna: sabar wasiƙarku ba ta da wata folda da wannan ƙa'ida ke amfani da ita.
rules-note-gmail-sign-in = Yana aiki a Katna har sai kun sake shiga Google kuma kun bar Katna ta yi matatun Gmail.
rules-note-sieve-other-script = Yana aiki a Katna: wani rubutun ƙa'idoji (“{ $name }”) yana aiki a sabar wasiƙarku.
rules-note-gmail-failed = Yana aiki a Katna: Gmail ba ta karɓe shi ba ({ $error }).
rules-note-sieve-failed = Yana aiki a Katna: sabar wasiƙarku ba ta karɓe shi ba ({ $error }).
rules-editor-cancel = Soke
rules-editor-save = Ajiye
rules-editor-saving = Ana ajiyewa…
rules-editor-delete = Share ƙa'ida
rules-editor-delete-ask = A share wannan ƙa'ida?
rules-editor-delete-keep = Bar ta
rules-editor-delete-confirm = Share
rules-editor-needs-folder = Zaɓi folda ga kowane “Matsar zuwa” da lakabi ga kowane “Ƙara lakabi”.
rules-editor-needs-days = “Yi alama an karanta bayan” yana buƙatar adadin kwanaki, daga 1 zuwa 3650.
rules-saved = An ajiye ƙa'ida
rules-saved-applied = { $count ->
    [one] An ajiye ƙa'ida kuma an yi amfani da ita a kan wasiƙa { $count }
   *[other] An ajiye ƙa'ida kuma an yi amfani da ita a kan wasiƙu { $count }
}
rules-apply-failed = An ajiye ƙa'ida, amma amfani da ita bai yi nasara ba: { $error }
rules-deleted = An share ƙa'ida
rules-delete-failed = Ba a iya share ƙa'idar ba: { $error }
rules-change-failed = Ba a iya canza ƙa'idojin ba: { $error }
