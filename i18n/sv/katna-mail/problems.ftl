# Katna Mail, Swedish (Svenska).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = E-postservern

problems-signed-out = { $provider } loggade ut Katna från { $address }. E-posten synkroniseras inte längre.
problems-password-refused = { $provider } nekade lösenordet för { $address }. Det kan ha ändrats.
problems-no-answer = { $provider } svarar inte för { $address }. Katna fortsätter att försöka.
problems-offline = Du är offline. Din e-post finns kvar här, och e-post du skickar väntar tills du är tillbaka.
problems-accounts-need-you = { $count ->
    [one] 1 konto behöver dig
   *[other] { $count } konton behöver dig
}
problems-show = Visa
problems-later = Senare
problems-new-password = Nytt lösenord
problems-try-again = Försök igen

## The New password card

problems-password-title = Nytt lösenord
problems-password-detail = { $provider } nekade det sparade lösenordet för { $address }. Ange det nya; Katna kontrollerar det innan det sparas.
problems-password-placeholder = Lösenord
problems-password-show = Visa lösenord
problems-password-hide = Dölj lösenord
problems-password-cancel = Avbryt
problems-password-save = Spara
problems-password-checking = Kontrollerar…
problems-password-refused-again = { $provider } nekade det här lösenordet också. Kontrollera det och försök igen.
problems-password-saved = Lösenordet sparades för { $address }. Hämtar din e-post…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = E-postservern för { $address } godtog inte att { $count ->
    [one] ett meddelande flyttades, så det är tillbaka där det var.
   *[other] { $count } meddelanden flyttades, så de är tillbaka där de var.
}
problems-refused-flags = E-postservern för { $address } godtog inte att { $count ->
    [one] ett meddelande markerades (läst, stjärnmärkt…), så det är tillbaka som det var.
   *[other] { $count } meddelanden markerades (läst, stjärnmärkt…), så de är tillbaka som de var.
}
problems-refused-label = E-postservern för { $address } godtog inte att etiketterna ändrades på { $count ->
    [one] ett meddelande, så det är tillbaka som det var.
   *[other] { $count } meddelanden, så de är tillbaka som de var.
}
problems-refused-delete = E-postservern för { $address } godtog inte att { $count ->
    [one] ett meddelande raderades, så det är tillbaka.
   *[other] { $count } meddelanden raderades, så de är tillbaka.
}
problems-refused-other = E-postservern för { $address } godtog inte { $count ->
    [one] en ändring, så Katna återställde den.
   *[other] { $count } ändringar, så Katna återställde dem.
}
problems-details = Detaljer

## Katna's background service (katna-daemon) isn't running

service-starting = Startar Katnas bakgrundstjänst…
service-failed = Katnas bakgrundstjänst startar inte, så e-posten synkroniseras inte.
service-start-again = Starta igen
service-started-again = Katnas bakgrundstjänst stoppades och startades igen.
service-details-title = Varför tjänsten inte startar
service-details-body = Kopiera detta och skicka det med din rapport. Det innehåller ingen e-post och inga lösenord.
service-details-copy = Kopiera
service-details-close = Stäng
service-not-running = Katnas bakgrundstjänst körs inte.
service-no-answer = Katnas bakgrundstjänst svarade inte: { $error }
service-no-session = Ingen D-Bus-session: { $error }

## Safe mode: an update left Katna's background service unable to start

safe-line = Katna är i felsäkert läge efter ett problem med uppdateringen, så e-posten synkroniseras inte.
safe-try-again = Försök igen
safe-restore = Återställ
safe-restoring = Återställer dina data från { $when }…
safe-restored = Dina data har återställts från { $when }. Det som fanns där innan sparas i en mapp.
safe-show-folder = Visa mapp
safe-restore-failed = Det gick inte att återställa dina data: { $error }
safe-restore-title = Återställa dina data från före en uppdatering?
safe-restore-body = Katna går tillbaka till kopian du väljer. E-post som kom efter den hämtas igen från dina konton.
safe-restore-none = Det finns inga kopior än. Katna gör en innan varje uppdatering ändrar dina data.
safe-restore-keep = Det som finns där nu, inklusive oskickad e-post, utkast och ändringar som inte har synkroniserats än, sparas först i en mapp, så att ingenting går förlorat.
safe-restore-cancel = Avbryt
safe-restore-mail = E-post
safe-restore-pim = Konton och kontakter
safe-restore-blobs = Bilagor
safe-report-title = Felsökningsrapport
safe-report-body = Kopiera detta och bifoga det till din felrapport. Den innehåller ingen e-post, inga adresser och inga lösenord.
safe-report-restore = Återställ…
safe-report-copied = Felsökningsrapporten kopierades
