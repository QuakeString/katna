# Katna Mail, Swedish (Svenska).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Accounts

accounts-folder-pane = Mappfönster
accounts-folder-pane-detail = Vilka kontons mappar fönstret till vänster visar.
accounts-shown-one = Ett konto i taget; byt i kontokortet
accounts-shown-all = Alla konton, efter varandra
accounts-unified = Gemensam inkorg
accounts-unified-switch = Visa e-post från alla konton tillsammans
accounts-unified-switch-detail = ”Alla konton” står överst i mappfönstret, med inkorgen, skickad e-post och mer från varje konto i en lista. Kontona under börjar ihopfällda.
accounts-row = Konton
accounts-row-detail = Mappfönstret och kontomenyn visar kontona i den här ordningen; det första är standard. När du tar bort ett konto raderas Katnas kopia av dess e-post på den här datorn. E-posten finns kvar på servern.
accounts-none = Inga konton än.
accounts-pop3-row = E-post på servern
accounts-pop3-row-detail = POP3-konton hämtar e-post till den här datorn. Välj vad som sedan händer med kopian på servern.
accounts-pop3-with-katna = Behåll den tills jag raderar den i Katna
accounts-pop3-at-once = Radera den när den har hämtats
accounts-pop3-after-days = { $count ->
    [one] Radera den efter { $count } dag
   *[other] Radera den efter { $count } dagar
}
accounts-pop3-never = Radera den aldrig
accounts-pop3-days-less = Färre dagar
accounts-pop3-days-more = Fler dagar
accounts-kind-imported = Importerat
accounts-picture-reset = Använd skrivbordets bild
accounts-picture-change = Byt bild
accounts-picture-remove = Ta bort bild
accounts-rename = Byt namn
accounts-name-save = Spara
accounts-name-cancel = Avbryt
accounts-name-placeholder = Ditt namn
accounts-rename-failed = Det gick inte att byta namn på kontot: { $error }
accounts-move-up = Flytta upp
accounts-move-down = Flytta ned
accounts-drag = Dra för att ändra ordningen
accounts-remove = Ta bort
accounts-delete-all-row = Radera all data
accounts-delete-all-row-detail = Börja om, som vid en ny installation.
accounts-delete-all-about = Raderar alla konton, all sparad e-post, kontakter och kalendrar, sökindexet, dina inställningar och sparade lösenord från den här datorn. Ingenting ändras på dina e-postservrar.
accounts-delete-all-open = Radera all Katna-data

## Settings > Accounts: snackbars after deleting

accounts-removed-local = { $address } har tagits bort från Katna.
accounts-removed = { $address } har tagits bort från Katna. E-posten finns kvar på servern.
accounts-all-deleted = All Katna-data har raderats från den här datorn.

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = Ta bort { $address }?
accounts-remove-confirm = Ta bort konto
accounts-removing = Tar bort…
accounts-remove-local-mail = { $folders ->
    [0] All e-post som har importerats till kontot
    [one] All e-post som har importerats till kontot, i dess mapp
   *[other] All e-post som har importerats till kontot, i dess { $folders } mappar
}
accounts-remove-local-settings = Dess Katna-inställningar
accounts-remove-mail = { $folders ->
    [0] All e-post från kontot som Katna har sparat
    [one] All e-post från kontot som Katna har sparat, i dess mapp
   *[other] All e-post från kontot som Katna har sparat, i dess { $folders } mappar
}
accounts-remove-outbox = Dess meddelanden som väntar i utkorgen
accounts-remove-settings = Dess sparade lösenord och dess Katna-inställningar
accounts-delete-all-title = Radera all Katna-data?
accounts-delete-all-confirm = Radera allt
accounts-deleting = Raderar…
accounts-delete-all-accounts = Alla konton, och all e-post och alla bilagor som Katna har sparat
accounts-delete-all-contacts = Kontakter, kalendrar och sökindexet
accounts-delete-all-settings = Alla inställningar, signaturer och kortkommandon
accounts-delete-all-passwords = Alla sparade lösenord
accounts-deleted-heading = Raderas från den här datorn:
accounts-cannot-undo = Det går inte att ångra.
accounts-server-delete-all = Ingenting ändras på dina e-postservrar: din e-post finns kvar där, och om du lägger till ett konto igen hämtas den igen. E-post som har importerats från filer finns bara i Katna; filerna rörs inte.
accounts-server-local = E-posten har importerats från filer, så Katna har den enda kopian. Filerna den kom från rörs inte; importera dem igen för att få tillbaka den.
accounts-server-remove = Ingenting ändras på e-postservern: din e-post finns kvar där, och om du lägger till kontot igen hämtas den igen.
accounts-confirm-word = radera
accounts-confirm-placeholder = Skriv ”{ accounts-confirm-word }”
accounts-confirm-prompt = Bekräfta genom att skriva ”{ accounts-confirm-word }”:
accounts-cancel = Avbryt

## Reset cache (Settings > General), in the same dialog

reset-cache-about = Raderar e-posten och bilagorna som Katna har hämtat, avsändarbilder och sökindexet och hämtar sedan ny e-post igen. Konton, inställningar och e-post som bara finns på den här datorn blir kvar.
reset-cache-button = Återställ cache
reset-cache-title = Återställa cachen?
reset-cache-deleted = Raderas och hämtas sedan igen:
reset-cache-mail = E-post och bilagor som hämtats från dina IMAP-servrar: ny e-post hämtas igen direkt, äldre när du öppnar den
reset-cache-index = Sökindexet, som byggs upp igen direkt
reset-cache-pictures = Avsändarbilder
reset-cache-kept = Blir kvar: dina konton, lösenord och inställningar; stjärnor, etiketter, lästmarkeringar och fästa meddelanden; utkast, utkorgen och ändringar som ännu inte finns på servern; och e-post från POP3-konton eller importerade filer, som kanske inte har någon annan kopia. Ingenting ändras på dina e-postservrar.
reset-cache-confirm = Återställ cache
reset-cache-busy = Återställer…
reset-cache-done = Cachen har återställts. Ny e-post hämtas igen.
reset-cache-done-freed = Cachen har återställts och { $size } har frigjorts. Ny e-post hämtas igen.
