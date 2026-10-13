# Katna Mail, Swahili (Kiswahili).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Folders & rules

settings-rules = Sheria
settings-rules-summary = Panga, weka lebo, sambaza au nyamazisha barua mpya zenyewe
settings-rules-intro = Sheria hupanga barua mpya zenyewe, kwa mpangilio huu. Buruta ili kupanga upya.
settings-rules-all-accounts = Akaunti zote
settings-rules-new = Sheria mpya
settings-rules-none = Bado hakuna sheria. Sheria hupanga barua mpya yenyewe: kwa mtumaji, mada au maneno.
settings-rules-none-account = Bado hakuna sheria za akaunti hii.
settings-rules-drag = Buruta ili kupanga upya
settings-rules-edit = Hariri sheria
settings-rules-turn-off = Zima sheria hii
settings-rules-turn-on = Washa sheria hii

## Starter rules: offered under the user's own rules, switched off.
## Turning one on makes it one of the user's rules.

settings-rules-starters = Sheria za kuanzia
settings-rules-starters-intro = Zimezimwa hadi utakapowasha moja. Zinafanya kazi kwa akaunti zako zote; hariri moja ili kuibadilisha.
settings-rules-starter-turning-on = Inawasha “{ $name }”…
settings-rules-starter-failed = Imeshindwa kuwasha “{ $name }”: { $error }
rules-starter-promotions = Nyamazisha matangazo
rules-starter-newsletters = Majarida kwenda Kusoma
rules-starter-receipts = Risiti na ankara
rules-starter-deliveries = Usafirishaji wa vifurushi
rules-starter-train = Tiketi za treni
rules-starter-flight = Tiketi za ndege
rules-starter-codes = Misimbo ya mara moja
rules-starter-security = Tahadhari za usalama
rules-starter-social = Barua za mitandao ya kijamii
rules-starter-invites = Mialiko ya kalenda
rules-starter-folder-reading = Kusoma
rules-starter-folder-receipts = Risiti
rules-starter-folder-deliveries = Usafirishaji
rules-starter-folder-travel = Safari
rules-starter-folder-social = Mitandao ya kijamii
rules-runs-katna = Inaendeshwa kwenye Katna
rules-runs-gmail = Inaendeshwa kwenye Gmail
rules-runs-sieve = Inaendeshwa kwenye seva
rules-stopped = Imesimamishwa
rules-error-folder-gone = Folda ambayo sheria hii hutumia haipo tena. Hariri sheria ili uchague nyingine.
rules-error-no-archive = Akaunti hii haina folda ya kumbukumbu. Hariri sheria ili ifanye kitu kingine.
rules-error-no-trash = Akaunti hii haina folda ya Tupio. Hariri sheria ili ifanye kitu kingine.
rules-error-cannot-send = Akaunti hii haiwezi kutuma barua, hivyo sheria haiwezi kuisambaza.
rules-error-other = { $error }. Hariri sheria na uiwashe tena.

settings-folders = Folda
settings-folders-summary = Idadi ya ambazo hazijasomwa kwenye kidirisha cha folda
settings-folders-unread-counts = Idadi ya ambazo hazijasomwa kwenye kila folda
settings-folders-unread-counts-detail = Imezimwa: Kikasha pekee huonyesha ngapi hazijasomwa

## A rule in one line, on its row: "From contains substack.com → skip the
## inbox, label Reading".

rules-summary = { $when } → { $then }
rules-summary-and = { $first } na { $next }
rules-summary-or = { $first } au { $next }
rules-summary-more = { $count } zaidi
rules-summary-list = { $first }, { $next }
rules-summary-condition = { $field } { $comparator } { $value }
rules-summary-has-attachment = Ina kiambatisho
rules-summary-no-attachment = Haina kiambatisho
rules-summary-mailing-list = Kutoka kwenye orodha ya barua
rules-summary-not-mailing-list = Si kutoka kwenye orodha ya barua
rules-summary-tab = Kwenye kichupo cha { $tab }
rules-summary-not-tab = Si kwenye kichupo cha { $tab }
rules-summary-move = hamishia { $folder }
rules-summary-archive = ruka kikasha
rules-summary-trash = hamishia kwenye tupio
rules-summary-mark-read = tia alama kuwa imesomwa
rules-summary-star = weka nyota
rules-summary-important = tia alama kuwa muhimu
rules-summary-label = weka lebo { $label }
rules-summary-forward = sambaza kwa { $address }
rules-summary-dont-notify = usiarifu
rules-summary-read-after = { $count ->
    [one] tia alama kuwa imesomwa baada ya siku { $count }
   *[other] tia alama kuwa imesomwa baada ya siku { $count }
}
rules-summary-folder-gone = folda ambayo haipo tena

## The rule editor

rules-editor-new-title = Sheria mpya
rules-editor-edit-title = Hariri sheria
rules-editor-name-hint = Jina la sheria
rules-editor-when = Barua mpya inapolingana na
rules-editor-of-these = ya haya:
rules-mode-all = yote
rules-mode-any = yoyote
rules-field-from = Kutoka
rules-field-to = Kwa
rules-field-cc = Cc
rules-field-any-recipient = Kwa au Cc
rules-field-reply-to = Jibu kwa
rules-field-subject = Mada
rules-field-body = Maandishi
rules-field-attachment-name = Jina la kiambatisho
rules-field-has-attachment = Ina kiambatisho
rules-field-mailing-list = Kutoka kwenye orodha ya barua
rules-field-tab = Kichupo cha kikasha
rules-comparator-contains = ina
rules-comparator-not-contains = haina
rules-comparator-begins-with = inaanza na
rules-comparator-ends-with = inaishia na
rules-comparator-equals = ni hasa
rules-comparator-matches = inalingana na ruwaza
rules-has-yes = ndiyo
rules-has-no = hapana
rules-editor-value-hint = Maneno au anwani
rules-editor-add-condition = Ongeza sharti
rules-editor-remove = Ondoa
rules-editor-then = Kisha:
rules-action-move = Hamishia
rules-action-archive = Ruka kikasha (weka kwenye kumbukumbu)
rules-action-trash = Hamishia kwenye tupio
rules-action-mark-read = Tia alama kuwa imesomwa
rules-action-star = Weka nyota
rules-action-important = Tia alama kuwa muhimu
rules-action-label = Ongeza lebo
rules-action-forward = Sambaza kwa
rules-action-dont-notify = Usiarifu
rules-action-read-after = Tia alama kuwa imesomwa baada ya
rules-editor-choose-folder = Chagua folda
rules-editor-choose-label = Chagua lebo
rules-editor-new-folder = Mpya: { $name }
rules-editor-folder-of = { $folder } ({ $account })
rules-editor-forward-hint = Anwani ya barua pepe
rules-editor-days = siku
rules-editor-add-action = Ongeza kitendo
rules-editor-stop = Simama hapa: sheria zinazofuata hazitumiki kwenye barua hii
rules-editor-accounts = Akaunti:
rules-editor-accounts-none = Chagua akaunti
rules-editor-accounts-many = { $count ->
    [one] Akaunti { $count }
   *[other] Akaunti { $count }
}
rules-editor-matches = Inalingana na { $mails } kutoka siku { $days } zilizopita
rules-editor-mails = { $count ->
    [one] barua { $count }
   *[other] barua { $count }
}
rules-editor-counting = Inahesabu barua zinazolingana nayo…
rules-editor-show = Zionyeshe
rules-editor-also-apply = Itumie pia kwa hizi { $count }
rules-editor-runs-katna = Inaendeshwa kwenye Katna, wakati kompyuta hii imewashwa.
rules-editor-runs-gmail = Inaendeshwa kwenye Gmail, hivyo inafanya kazi pia kwenye simu yako na kompyuta hii ikiwa imezimwa.
rules-editor-runs-sieve = Inaendeshwa kwenye seva yako ya barua, hivyo inafanya kazi pia kwenye simu yako na kompyuta hii ikiwa imezimwa.
rules-note-gmail-action = Inaendeshwa kwenye Katna: vichujio vya Gmail haviwezi kufanya “{ $action }”.
rules-note-sieve-action = Inaendeshwa kwenye Katna: sheria za seva yako ya barua haziwezi kufanya “{ $action }”.
rules-note-test = { $field } { $comparator }
rules-note-gmail-condition = Inaendeshwa kwenye Katna: vichujio vya Gmail haviwezi kupima “{ $test }” kama Katna inavyofanya.
rules-note-sieve-condition = Inaendeshwa kwenye Katna: sheria za seva yako ya barua haziwezi kupima “{ $test }” kama Katna inavyofanya.
rules-note-order = Inaendeshwa kwenye Katna, kama sheria ya awali ya akaunti inavyofanya: sheria huendeshwa kwa mpangilio wa orodha.
rules-note-gmail-stop = Inaendeshwa kwenye Katna: vichujio vya Gmail haviwezi kuzuia sheria zinazofuata zisiendeshwe.
rules-note-gmail-forward = Inaendeshwa kwenye Katna: Gmail husambaza tu kwa anwani zilizothibitishwa kwenye mipangilio yake, na { $address } si mojawapo.
rules-note-gmail-folder = Inaendeshwa kwenye Katna: Gmail haina lebo ya folda ambayo sheria hii hutumia.
rules-note-sieve-folder = Inaendeshwa kwenye Katna: seva yako ya barua haina folda ambayo sheria hii hutumia.
rules-note-gmail-sign-in = Inaendeshwa kwenye Katna hadi utakapoingia tena kwenye Google na kuiruhusu Katna kuunda vichujio vya Gmail.
rules-note-sieve-other-script = Inaendeshwa kwenye Katna: hati nyingine ya sheria (“{ $name }”) inatumika kwenye seva yako ya barua.
rules-note-gmail-failed = Inaendeshwa kwenye Katna: Gmail haikuipokea ({ $error }).
rules-note-sieve-failed = Inaendeshwa kwenye Katna: seva yako ya barua haikuipokea ({ $error }).
rules-editor-cancel = Ghairi
rules-editor-save = Hifadhi
rules-editor-saving = Inahifadhi…
rules-editor-delete = Futa sheria
rules-editor-delete-ask = Futa sheria hii?
rules-editor-delete-keep = Iache
rules-editor-delete-confirm = Futa
rules-editor-needs-folder = Chagua folda kwa kila “Hamishia” na lebo kwa kila “Ongeza lebo”.
rules-editor-needs-days = “Tia alama kuwa imesomwa baada ya” inahitaji idadi ya siku, kuanzia 1 hadi 3650.
rules-saved = Sheria imehifadhiwa
rules-saved-applied = { $count ->
    [one] Sheria imehifadhiwa na kutumika kwa barua { $count }
   *[other] Sheria imehifadhiwa na kutumika kwa barua { $count }
}
rules-apply-failed = Sheria imehifadhiwa, lakini kuitumia kumeshindikana: { $error }
rules-deleted = Sheria imefutwa
rules-delete-failed = Imeshindwa kufuta sheria: { $error }
rules-change-failed = Imeshindwa kubadilisha sheria: { $error }
