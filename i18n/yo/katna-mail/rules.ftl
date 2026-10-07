# Katna Mail, Yoruba (Yorùbá).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Folders & rules

settings-rules = Àwọn òfin
settings-rules-summary = Ṣètò, fi àmì sí, fi ránṣẹ́ síwájú tàbí pa ohùn lẹ́tà tuntun fúnra rẹ̀
settings-rules-intro = Àwọn òfin ń ṣètò lẹ́tà tuntun fúnra wọn, ní ìtò yìí. Fà á láti tún ìtò ṣe.
settings-rules-all-accounts = Gbogbo àkáǹtì
settings-rules-new = Òfin tuntun
settings-rules-none = Kò sí òfin kankan síbẹ̀. Òfin kan ń ṣètò lẹ́tà tuntun fúnra rẹ̀: nípa olùfiránṣẹ́, àkọlé tàbí ọ̀rọ̀.
settings-rules-none-account = Kò sí òfin kankan fún àkáǹtì yìí síbẹ̀.
settings-rules-drag = Fà á láti tún ìtò ṣe
settings-rules-edit = Ṣàtúnṣe òfin
settings-rules-turn-off = Pa òfin yìí
settings-rules-turn-on = Tan òfin yìí

## Starter rules: offered under the user's own rules, switched off.
## Turning one on makes it one of the user's rules.

settings-rules-starters = Àwọn òfin ìbẹ̀rẹ̀
settings-rules-starters-intro = Wọ́n wà ní pípa títí o fi tan ọ̀kan. Wọ́n ń ṣiṣẹ́ fún gbogbo àkáǹtì rẹ; ṣàtúnṣe ọ̀kan láti yí i padà.
settings-rules-starter-turning-on = Ń tan “{ $name }”…
settings-rules-starter-failed = A kò lè tan “{ $name }”: { $error }
rules-starter-promotions = Pa ohùn ìpolówó
rules-starter-newsletters = Ìwé ìròyìn sí Kíkà
rules-starter-receipts = Ìwé ẹ̀rí ìsanwó àti ìwé owó
rules-starter-deliveries = Ìfijíṣẹ́
rules-starter-train = Tíkẹ́ẹ̀tì ọkọ̀ ojú irin
rules-starter-flight = Tíkẹ́ẹ̀tì ọkọ̀ òfurufú
rules-starter-codes = Kóòdù ìlò ẹ̀ẹ̀kan
rules-starter-security = Ìkìlọ̀ ààbò
rules-starter-social = Lẹ́tà àwùjọ
rules-starter-invites = Ìkésíni kàlẹ́ńdà
rules-starter-folder-reading = Kíkà
rules-starter-folder-receipts = Ìwé ẹ̀rí ìsanwó
rules-starter-folder-deliveries = Ìfijíṣẹ́
rules-starter-folder-travel = Ìrìnàjò
rules-starter-folder-social = Àwùjọ
rules-runs-katna = Ń ṣiṣẹ́ nínú Katna
rules-runs-gmail = Ń ṣiṣẹ́ lórí Gmail
rules-runs-sieve = Ń ṣiṣẹ́ lórí sáfà
rules-stopped = Ti dúró
rules-error-folder-gone = Fódà tí òfin yìí ń lò kò sí mọ́. Ṣàtúnṣe òfin náà láti yan òmíràn.
rules-error-no-archive = Àkáǹtì yìí kò ní fódà ìpamọ́. Ṣàtúnṣe òfin náà láti ṣe nǹkan mìíràn.
rules-error-no-trash = Àkáǹtì yìí kò ní fódà Ìdọ̀tí. Ṣàtúnṣe òfin náà láti ṣe nǹkan mìíràn.
rules-error-cannot-send = Àkáǹtì yìí kò lè fi lẹ́tà ránṣẹ́, nítorí náà òfin náà kò lè fi í ránṣẹ́ síwájú.
rules-error-other = { $error }. Ṣàtúnṣe òfin náà kí o sì tún tàn án.

settings-folders = Àwọn fódà
settings-folders-summary = Iye àìkà nínú pánẹ́ẹ̀lì fódà
settings-folders-unread-counts = Iye àìkà lórí gbogbo fódà
settings-folders-unread-counts-detail = Ní pípa: Àpótí-ìwọlé nìkan ló ń fi iye àìkà hàn

## A rule in one line, on its row: "From contains substack.com → skip the
## inbox, label Reading".

rules-summary = { $when } → { $then }
rules-summary-and = { $first } àti { $next }
rules-summary-or = { $first } tàbí { $next }
rules-summary-more = { $count } mìíràn
rules-summary-list = { $first }, { $next }
rules-summary-condition = { $field } { $comparator } { $value }
rules-summary-has-attachment = Ó ní àfikún
rules-summary-no-attachment = Kò ní àfikún
rules-summary-mailing-list = Láti àkójọ lẹ́tà
rules-summary-not-mailing-list = Kì í ṣe láti àkójọ lẹ́tà
rules-summary-tab = Nínú táàbù { $tab }
rules-summary-not-tab = Kì í ṣe nínú táàbù { $tab }
rules-summary-move = gbé lọ sí { $folder }
rules-summary-archive = fo àpótí-ìwọlé
rules-summary-trash = gbé lọ sí ìdọ̀tí
rules-summary-mark-read = sàmì sí bí kíkà
rules-summary-star = fi ìràwọ̀ sí i
rules-summary-important = sàmì sí bí pàtàkì
rules-summary-label = fi àmì { $label } sí i
rules-summary-forward = fi ránṣẹ́ síwájú sí { $address }
rules-summary-dont-notify = má ṣe fi tó mi létí
rules-summary-read-after = { $count ->
   *[other] sàmì sí bí kíkà lẹ́yìn ọjọ́ { $count }
}
rules-summary-folder-gone = fódà kan tí kò sí mọ́

## The rule editor

rules-editor-new-title = Òfin tuntun
rules-editor-edit-title = Ṣàtúnṣe òfin
rules-editor-name-hint = Orúkọ òfin
rules-editor-when = Nígbà tí lẹ́tà tuntun bá bá
rules-editor-of-these = nínú ìwọ̀nyí mu:
rules-mode-all = gbogbo
rules-mode-any = èyíkéyìí
rules-field-from = Láti
rules-field-to = Sí
rules-field-cc = Ẹ̀dà
rules-field-any-recipient = Sí tàbí Ẹ̀dà
rules-field-reply-to = Fèsì sí
rules-field-subject = Àkọlé
rules-field-body = Ọ̀rọ̀
rules-field-attachment-name = Orúkọ àfikún
rules-field-has-attachment = Ó ní àfikún
rules-field-mailing-list = Láti àkójọ lẹ́tà
rules-field-tab = Táàbù àpótí-ìwọlé
rules-comparator-contains = ní nínú
rules-comparator-not-contains = kò ní nínú
rules-comparator-begins-with = bẹ̀rẹ̀ pẹ̀lú
rules-comparator-ends-with = parí pẹ̀lú
rules-comparator-equals = jẹ́ gẹ́lẹ́
rules-comparator-matches = bá àpẹẹrẹ mu
rules-has-yes = bẹ́ẹ̀ ni
rules-has-no = rárá
rules-editor-value-hint = Ọ̀rọ̀ tàbí àdírẹ́sì
rules-editor-add-condition = Fi àdéhùn kan kún un
rules-editor-remove = Yọ kúrò
rules-editor-then = Lẹ́yìn náà:
rules-action-move = Gbé lọ sí
rules-action-archive = Fo àpótí-ìwọlé (fi pamọ́)
rules-action-trash = Gbé lọ sí ìdọ̀tí
rules-action-mark-read = Sàmì sí bí kíkà
rules-action-star = Fi ìràwọ̀ sí i
rules-action-important = Sàmì sí bí pàtàkì
rules-action-label = Fi àmì kún un
rules-action-forward = Fi ránṣẹ́ síwájú sí
rules-action-dont-notify = Má ṣe fi tó mi létí
rules-action-read-after = Sàmì sí bí kíkà lẹ́yìn
rules-editor-choose-folder = Yan fódà kan
rules-editor-choose-label = Yan àmì kan
rules-editor-new-folder = Tuntun: { $name }
rules-editor-folder-of = { $folder } ({ $account })
rules-editor-forward-hint = Àdírẹ́sì ímeèlì
rules-editor-days = ọjọ́
rules-editor-add-action = Fi ìgbésẹ̀ kan kún un
rules-editor-stop = Dúró síbí: àwọn òfin tó tẹ̀lé kò ní ṣiṣẹ́ lórí lẹ́tà yìí
rules-editor-accounts = Àwọn àkáǹtì:
rules-editor-accounts-none = Yan àwọn àkáǹtì
rules-editor-accounts-many = { $count ->
   *[other] Àkáǹtì { $count }
}
rules-editor-matches = Ó bá { $mails } mu láti ọjọ́ { $days } sẹ́yìn
rules-editor-mails = { $count ->
   *[other] lẹ́tà { $count }
}
rules-editor-counting = Ń ka àwọn lẹ́tà tó bá mu…
rules-editor-show = Fi wọ́n hàn
rules-editor-also-apply = Lò ó fún àwọn { $count } yìí pẹ̀lú
rules-editor-runs-katna = Ń ṣiṣẹ́ nínú Katna, nígbà tí kọ̀ǹpútà yìí bá wà ní títàn.
rules-editor-runs-gmail = Ń ṣiṣẹ́ lórí Gmail, nítorí náà ó tún ń ṣiṣẹ́ lórí fóònù rẹ àti nígbà tí kọ̀ǹpútà yìí bá wà ní pípa.
rules-editor-runs-sieve = Ń ṣiṣẹ́ lórí sáfà lẹ́tà rẹ, nítorí náà ó tún ń ṣiṣẹ́ lórí fóònù rẹ àti nígbà tí kọ̀ǹpútà yìí bá wà ní pípa.
rules-note-gmail-action = Ń ṣiṣẹ́ nínú Katna: àwọn àlẹ̀mọ́ Gmail kò lè ṣe “{ $action }”.
rules-note-sieve-action = Ń ṣiṣẹ́ nínú Katna: àwọn òfin sáfà lẹ́tà rẹ kò lè ṣe “{ $action }”.
rules-note-test = { $field } { $comparator }
rules-note-gmail-condition = Ń ṣiṣẹ́ nínú Katna: àwọn àlẹ̀mọ́ Gmail kò lè dán “{ $test }” wò bí Katna ṣe ń ṣe.
rules-note-sieve-condition = Ń ṣiṣẹ́ nínú Katna: àwọn òfin sáfà lẹ́tà rẹ kò lè dán “{ $test }” wò bí Katna ṣe ń ṣe.
rules-note-order = Ń ṣiṣẹ́ nínú Katna, bí òfin àkáǹtì náà tó ṣáájú ṣe ń ṣe: àwọn òfin ń ṣiṣẹ́ ní ìtò àkójọ.
rules-note-gmail-stop = Ń ṣiṣẹ́ nínú Katna: àwọn àlẹ̀mọ́ Gmail kò lè dá àwọn òfin tó tẹ̀lé dúró.
rules-note-gmail-forward = Ń ṣiṣẹ́ nínú Katna: Gmail ń fi ránṣẹ́ síwájú kìkì sí àwọn àdírẹ́sì tí a ti fìdí wọn múlẹ̀ nínú ètò rẹ̀, { $address } kò sì sí lára wọn.
rules-note-gmail-folder = Ń ṣiṣẹ́ nínú Katna: Gmail kò ní àmì fún fódà kan tí òfin yìí ń lò.
rules-note-sieve-folder = Ń ṣiṣẹ́ nínú Katna: sáfà lẹ́tà rẹ kò ní fódà kan tí òfin yìí ń lò.
rules-note-gmail-sign-in = Ń ṣiṣẹ́ nínú Katna títí o fi wọlé sí Google lẹ́ẹ̀kan sí i tí o sì jẹ́ kí Katna ṣe àwọn àlẹ̀mọ́ Gmail.
rules-note-sieve-other-script = Ń ṣiṣẹ́ nínú Katna: ìwé òfin mìíràn (“{ $name }”) ń ṣiṣẹ́ lórí sáfà lẹ́tà rẹ.
rules-note-gmail-failed = Ń ṣiṣẹ́ nínú Katna: Gmail kò gbà á ({ $error }).
rules-note-sieve-failed = Ń ṣiṣẹ́ nínú Katna: sáfà lẹ́tà rẹ kò gbà á ({ $error }).
rules-editor-cancel = Fagilé
rules-editor-save = Fi pamọ́
rules-editor-saving = Ń fi pamọ́…
rules-editor-delete = Pa òfin rẹ́
rules-editor-delete-ask = Ṣé kí a pa òfin yìí rẹ́?
rules-editor-delete-keep = Pa á mọ́
rules-editor-delete-confirm = Pa rẹ́
rules-editor-needs-folder = Yan fódà kan fún “Gbé lọ sí” kọ̀ọ̀kan àti àmì kan fún “Fi àmì kún un” kọ̀ọ̀kan.
rules-editor-needs-days = “Sàmì sí bí kíkà lẹ́yìn” nílò iye ọjọ́, láti 1 sí 3650.
rules-saved = A ti fi òfin pamọ́
rules-saved-applied = { $count ->
   *[other] A ti fi òfin pamọ́, a sì ti lò ó fún lẹ́tà { $count }
}
rules-apply-failed = A ti fi òfin pamọ́, ṣùgbọ́n lílò rẹ̀ kùnà: { $error }
rules-deleted = A ti pa òfin rẹ́
rules-delete-failed = A kò lè pa òfin náà rẹ́: { $error }
rules-change-failed = A kò lè yí àwọn òfin padà: { $error }
