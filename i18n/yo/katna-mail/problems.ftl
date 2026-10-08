# Katna Mail, Yoruba (Yorùbá).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = Sáfà lẹ́tà

problems-signed-out = { $provider } ti mú Katna jáde kúrò nínú { $address }. Lẹ́tà ti dáwọ́ ìbámu dúró.
problems-password-refused = { $provider } kọ ọ̀rọ̀ aṣínà fún { $address }. Ó lè jẹ́ pé ó ti yí padà.
problems-no-answer = { $provider } kò dáhùn fún { $address }. Katna ń gbìyànjú síbẹ̀.
problems-offline = O kò sí lórí ayélujára. Lẹ́tà rẹ ṣì wà níbí, lẹ́tà tí o bá fi ránṣẹ́ yóò sì dúró títí o fi padà.
problems-accounts-need-you = { $count ->
   *[other] Àkáǹtì { $count } nílò rẹ
}
problems-show = Fi hàn
problems-later = Lẹ́yìn náà
problems-new-password = Ọ̀rọ̀ aṣínà tuntun
problems-try-again = Gbìyànjú lẹ́ẹ̀kan sí i

## The New password card

problems-password-title = Ọ̀rọ̀ aṣínà tuntun
problems-password-detail = { $provider } kọ ọ̀rọ̀ aṣínà tí a fi pamọ́ fún { $address }. Tẹ èyí tuntun; Katna yóò ṣàyẹ̀wò rẹ̀ kí ó tó pa á mọ́.
problems-password-placeholder = Ọ̀rọ̀ aṣínà
problems-password-show = Fi ọ̀rọ̀ aṣínà hàn
problems-password-hide = Fi ọ̀rọ̀ aṣínà pamọ́
problems-password-cancel = Fagilé
problems-password-save = Fi pamọ́
problems-password-checking = Ó ń ṣàyẹ̀wò…
problems-password-refused-again = { $provider } kọ ọ̀rọ̀ aṣínà yìí náà. Ṣàyẹ̀wò rẹ̀ kí o sì gbìyànjú lẹ́ẹ̀kan sí i.
problems-password-saved = A ti fi ọ̀rọ̀ aṣínà pamọ́ fún { $address }. Ń gba lẹ́tà rẹ…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = Sáfà lẹ́tà { $address } kò gbà láti gbé { $count ->
   *[other] ìfiránṣẹ́ { $count }, nítorí náà wọ́n ti padà sí ibi tí wọ́n wà.
}
problems-refused-flags = Sáfà lẹ́tà { $address } kò gbà láti sàmì sí { $count ->
   *[other] ìfiránṣẹ́ { $count } (kíkà, oní ìràwọ̀…), nítorí náà wọ́n ti padà sí bí wọ́n ṣe wà.
}
problems-refused-label = Sáfà lẹ́tà { $address } kò gbà láti yí àmì { $count ->
   *[other] ìfiránṣẹ́ { $count } padà, nítorí náà wọ́n ti padà sí bí wọ́n ṣe wà.
}
problems-refused-delete = Sáfà lẹ́tà { $address } kò gbà láti pa { $count ->
   *[other] ìfiránṣẹ́ { $count } rẹ́, nítorí náà wọ́n ti padà.
}
problems-refused-other = Sáfà lẹ́tà { $address } kò gba { $count ->
   *[other] àyípadà { $count }, nítorí náà Katna ti dá wọn padà sí bí wọ́n ṣe wà.
}
problems-details = Àlàyé

## Katna's background service (katna-daemon) isn't running

service-starting = Ń bẹ̀rẹ̀ iṣẹ́ ẹ̀yìn Katna…
service-failed = Iṣẹ́ ẹ̀yìn Katna kò bẹ̀rẹ̀, nítorí náà lẹ́tà kò ní ìbámu.
service-start-again = Bẹ̀rẹ̀ lẹ́ẹ̀kan sí i
service-started-again = Iṣẹ́ ẹ̀yìn Katna dúró, a sì ti tún un bẹ̀rẹ̀.
service-details-title = Ìdí tí iṣẹ́ náà kò fi bẹ̀rẹ̀
service-details-body = Ṣẹ̀dà èyí kí o sì fi ránṣẹ́ pẹ̀lú ìròyìn rẹ. Kò sí lẹ́tà tàbí ọ̀rọ̀ aṣínà kankan nínú rẹ̀.
service-details-copy = Ṣẹ̀dà
service-details-close = Pa á dé
service-not-running = Iṣẹ́ ẹ̀yìn Katna kò ṣiṣẹ́.
service-no-answer = Iṣẹ́ ẹ̀yìn Katna kò dáhùn: { $error }
service-no-session = Kò sí ìgbà D-Bus: { $error }

## Safe mode: an update left Katna's background service unable to start

safe-line = Katna wà ní ipò ààbò lẹ́yìn ìṣòro kan pẹ̀lú ìmúdójúìwọ̀n, nítorí náà lẹ́tà kò ní ìbámu.
safe-try-again = Gbìyànjú lẹ́ẹ̀kan sí i
safe-restore = Dá padà
safe-restoring = À ń dá dátà rẹ padà láti { $when }…
safe-restored = A ti dá dátà rẹ padà láti { $when }. Ohun tó wà níbẹ̀ tẹ́lẹ̀ wà nínú fódà kan.
safe-show-folder = Fi fódà hàn
safe-restore-failed = A kò lè dá dátà rẹ padà: { $error }
safe-restore-title = Dá dátà rẹ padà sí bó ṣe wà ṣáájú ìmúdójúìwọ̀n?
safe-restore-body = Katna ń padà sí ẹ̀dà tí o bá yàn. Lẹ́tà tó dé lẹ́yìn rẹ̀ yóò tún wá láti àwọn àkáǹtì rẹ.
safe-restore-none = Kò tíì sí ẹ̀dà kankan. Katna ń ṣe ọ̀kan kí ìmúdójúìwọ̀n kọ̀ọ̀kan tó yí dátà rẹ padà.
safe-restore-keep = Ohun tó wà níbẹ̀ báyìí, pẹ̀lú lẹ́tà tí a kò tíì fi ránṣẹ́, àwọn àkọ̀pamọ́ àti àwọn àyípadà tí kò tíì ní ìbámu, ni a ó kọ́kọ́ fi pamọ́ sínú fódà kan, nítorí náà kò sí ohun tó máa sọnù.
safe-restore-cancel = Fagilé
safe-restore-mail = Lẹ́tà
safe-restore-pim = Àwọn àkáǹtì àti olùbásọ̀rọ̀
safe-restore-blobs = Àwọn àfikún
safe-report-title = Ìròyìn àṣìṣe
safe-report-body = Ṣẹ̀dà èyí kí o sì so ó mọ́ ìròyìn àṣìṣe rẹ. Kò sí lẹ́tà, àdírẹ́sì tàbí ọ̀rọ̀ aṣínà kankan nínú rẹ̀.
safe-report-restore = Dá padà…
safe-report-copied = A ti ṣẹ̀dà ìròyìn àṣìṣe
