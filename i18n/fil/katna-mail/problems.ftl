# Katna Mail, Filipino (Filipino).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = Ang mail server
problems-signed-out = Na-sign out ng { $provider } ang Katna sa { $address }. Huminto ang pag-sync ng mail.
problems-password-refused = Tinanggihan ng { $provider } ang password para sa { $address }. Baka napalitan ito.
problems-no-answer = Hindi sumasagot ang { $provider } para sa { $address }. Patuloy na sumusubok ang Katna.
problems-offline = Offline ka. Narito pa rin ang iyong mail, at maghihintay ang mail na ipapadala mo hanggang makabalik ka.
problems-accounts-need-you = { $count ->
    [one] { $count } account ang nangangailangan sa iyo
   *[other] { $count } account ang nangangailangan sa iyo
}
problems-show = Ipakita
problems-later = Mamaya
problems-new-password = Bagong password
problems-try-again = Subukang muli

## The New password card

problems-password-title = Bagong password
problems-password-detail = Tinanggihan ng { $provider } ang naka-save na password para sa { $address }. I-type ang bago; susuriin ito ng Katna bago itago.
problems-password-placeholder = Password
problems-password-show = Ipakita ang password
problems-password-hide = Itago ang password
problems-password-cancel = Kanselahin
problems-password-save = I-save
problems-password-checking = Sinusuri…
problems-password-refused-again = Tinanggihan din ng { $provider } ang password na ito. Suriin ito at subukang muli.
problems-password-saved = Na-save ang password para sa { $address }. Kinukuha ang iyong mail…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = Hindi tinanggap ng mail server ng { $address } ang paglipat ng { $count ->
    [one] isang mensahe, kaya ibinalik ito kung saan ito dati.
   *[other] { $count } mensahe, kaya ibinalik ang mga ito kung saan sila dati.
}
problems-refused-flags = Hindi tinanggap ng mail server ng { $address } ang pagmarka sa { $count ->
    [one] isang mensahe (nabasa, naka-star…), kaya ibinalik ito sa dati.
   *[other] { $count } mensahe (nabasa, naka-star…), kaya ibinalik ang mga ito sa dati.
}
problems-refused-label = Hindi tinanggap ng mail server ng { $address } ang pagpapalit ng mga label ng { $count ->
    [one] isang mensahe, kaya ibinalik ito sa dati.
   *[other] { $count } mensahe, kaya ibinalik ang mga ito sa dati.
}
problems-refused-delete = Hindi tinanggap ng mail server ng { $address } ang pag-delete ng { $count ->
    [one] isang mensahe, kaya ibinalik ito.
   *[other] { $count } mensahe, kaya ibinalik ang mga ito.
}
problems-refused-other = Hindi tinanggap ng mail server ng { $address } ang { $count ->
    [one] isang pagbabago, kaya ibinalik ito ng Katna sa dati.
   *[other] { $count } pagbabago, kaya ibinalik ng Katna ang mga ito sa dati.
}
problems-details = Mga detalye

## Katna's background service (katna-daemon) isn't running

service-starting = Sinisimulan ang background service ng Katna…
service-failed = Ayaw magsimula ng background service ng Katna, kaya hindi nagsi-sync ang mail.
service-start-again = Simulan muli
service-started-again = Huminto ang background service ng Katna at sinimulan itong muli.
service-details-title = Kung bakit ayaw magsimula ng service
service-details-body = Kopyahin ito at ipadala kasama ng iyong ulat. Wala itong mail o password.
service-details-copy = Kopyahin
service-details-close = Isara
service-not-running = Hindi tumatakbo ang serbisyo ng Katna sa background.
service-no-answer = Hindi sumagot ang serbisyo ng Katna sa background: { $error }
service-no-session = Walang D-Bus session: { $error }
