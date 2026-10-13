# Katna Mail, Hausa (Hausa).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = Sabar wasiƙa

problems-signed-out = { $provider } ta fitar da Katna daga { $address }. Wasiƙu sun daina daidaitawa.
problems-password-refused = { $provider } ta ƙi kalmar sirrin { $address }. Wataƙila an canza ta.
problems-no-answer = { $provider } ba ta amsawa ga { $address }. Katna tana ci gaba da gwadawa.
problems-offline = Ba ku da haɗi. Wasiƙunku suna nan, kuma wasiƙun da kuka aika za su jira har ku dawo.
problems-accounts-need-you = { $count ->
    [one] Asusu 1 yana buƙatar ku
   *[other] Asusu { $count } suna buƙatar ku
}
problems-show = Nuna
problems-later = Daga baya
problems-new-password = Sabuwar kalmar sirri
problems-try-again = Sake gwadawa

## The New password card

problems-password-title = Sabuwar kalmar sirri
problems-password-detail = { $provider } ta ƙi kalmar sirrin da aka ajiye don { $address }. Rubuta sabuwar; Katna za ta duba ta kafin ta ajiye.
problems-password-placeholder = Kalmar sirri
problems-password-show = Nuna kalmar sirri
problems-password-hide = Ɓoye kalmar sirri
problems-password-cancel = Soke
problems-password-save = Ajiye
problems-password-checking = Ana dubawa…
problems-password-refused-again = { $provider } ta ƙi wannan kalmar sirrin ma. Ku duba ta ku sake gwadawa.
problems-password-saved = An ajiye kalmar sirri don { $address }. Ana karɓo wasiƙunku…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = Sabar wasiƙar { $address } ba ta karɓi matsar da { $count ->
    [one] saƙo ba, don haka ya koma inda yake.
   *[other] saƙonni { $count } ba, don haka sun koma inda suke.
}
problems-refused-flags = Sabar wasiƙar { $address } ba ta karɓi yi wa { $count ->
    [one] saƙo alama ba (an karanta, tauraro…), don haka ya koma yadda yake.
   *[other] saƙonni { $count } alama ba (an karanta, tauraro…), don haka sun koma yadda suke.
}
problems-refused-label = Sabar wasiƙar { $address } ba ta karɓi canza lakabobin { $count ->
    [one] saƙo ba, don haka ya koma yadda yake.
   *[other] saƙonni { $count } ba, don haka sun koma yadda suke.
}
problems-refused-delete = Sabar wasiƙar { $address } ba ta karɓi share { $count ->
    [one] saƙo ba, don haka ya dawo.
   *[other] saƙonni { $count } ba, don haka sun dawo.
}
problems-refused-other = Sabar wasiƙar { $address } ba ta karɓi { $count ->
    [one] wani canji ba, don haka Katna ta mayar da shi yadda yake.
   *[other] canje-canje { $count } ba, don haka Katna ta mayar da su yadda suke.
}
problems-details = Cikakkun bayanai

## Katna's background service (katna-daemon) isn't running

service-starting = Ana fara sabis na bango na Katna…
service-failed = Sabis na bango na Katna ya ƙi farawa, don haka wasiƙu ba sa daidaitawa.
service-start-again = Sake farawa
service-started-again = Sabis na bango na Katna ya tsaya kuma an sake fara shi.
service-details-title = Dalilin da sabis ɗin ya ƙi farawa
service-details-body = Kwafi wannan ku aika shi tare da rahotonku. Babu wasiƙa ko kalmomin sirri a cikinsa.
service-details-copy = Kwafi
service-details-close = Rufe
service-not-running = Sabis na bango na Katna ba ya aiki.
service-no-answer = Sabis na bango na Katna bai amsa ba: { $error }
service-no-session = Babu zaman D-Bus: { $error }

## Safe mode: an update left Katna's background service unable to start

safe-line = Katna tana yanayin tsaro saboda matsala da sabuntawa, don haka wasiƙu ba sa daidaitawa.
safe-try-again = Sake gwadawa
safe-restore = Maido
safe-restoring = Ana maido da bayananku daga { $when }…
safe-restored = An maido da bayananku daga { $when }. Abin da yake nan a da an ajiye shi a cikin wata foldar.
safe-show-folder = Nuna foldar
safe-restore-failed = Ba a iya maido da bayananku ba: { $error }
safe-restore-title = A maido da bayananku daga kafin sabuntawa?
safe-restore-body = Katna tana komawa kwafin da kuka zaɓa. Wasiƙun da suka iso bayansa za a sake sauke su daga asusunku.
safe-restore-none = Babu kwafi tukuna. Katna tana yin ɗaya kafin kowace sabuntawa ta canza bayananku.
safe-restore-keep = Abin da yake nan yanzu, har da wasiƙun da ba a aika ba, zayyanai da canje-canjen da ba a daidaita ba tukuna, ana fara ajiye shi a cikin wata foldar, don haka babu abin da zai ɓace.
safe-restore-cancel = Soke
safe-restore-mail = Wasiƙu
safe-restore-pim = Asusu da lambobin sadarwa
safe-restore-blobs = Abubuwan haɗawa
safe-report-title = Rahoton matsala
safe-report-body = Ku kwafi wannan ku haɗa shi da rahotonku na matsala. Babu wasiƙa, adireshi ko kalmomin sirri a cikinsa.
safe-report-restore = Maido…
safe-report-copied = An kwafi rahoton matsala
