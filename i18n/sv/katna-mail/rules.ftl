# Katna Mail, Swedish (Svenska).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Folders & rules

settings-rules = Regler
settings-rules-summary = Sortera, etikettera, vidarebefordra eller tysta ny e-post automatiskt
settings-rules-intro = Regler sorterar ny e-post automatiskt, i den här ordningen. Dra för att ändra ordning.
settings-rules-all-accounts = Alla konton
settings-rules-new = Ny regel
settings-rules-none = Inga regler än. En regel sorterar ny e-post automatiskt: efter avsändare, ämne eller ord.
settings-rules-none-account = Inga regler för det här kontot än.
settings-rules-drag = Dra för att ändra ordning
settings-rules-edit = Redigera regel
settings-rules-turn-off = Stäng av den här regeln
settings-rules-turn-on = Slå på den här regeln

## Starter rules: offered under the user's own rules, switched off.
## Turning one on makes it one of the user's rules.

settings-rules-starters = Färdiga regler
settings-rules-starters-intro = Av tills du slår på en. De fungerar för alla dina konton; redigera en för att ändra den.
settings-rules-starter-turning-on = Slår på ”{ $name }”…
settings-rules-starter-failed = Det gick inte att slå på ”{ $name }”: { $error }
rules-starter-promotions = Tysta kampanjer
rules-starter-newsletters = Nyhetsbrev till Läsning
rules-starter-receipts = Kvitton och fakturor
rules-starter-deliveries = Leveranser
rules-starter-train = Tågbiljetter
rules-starter-flight = Flygbiljetter
rules-starter-codes = Engångskoder
rules-starter-security = Säkerhetsvarningar
rules-starter-social = Social e-post
rules-starter-invites = Kalenderinbjudningar
rules-starter-folder-reading = Läsning
rules-starter-folder-receipts = Kvitton
rules-starter-folder-deliveries = Leveranser
rules-starter-folder-travel = Resor
rules-starter-folder-social = Socialt
rules-runs-katna = Körs i Katna
rules-runs-gmail = Körs i Gmail
rules-runs-sieve = Körs på servern
rules-stopped = Stoppad
rules-error-folder-gone = Mappen som den här regeln använder finns inte längre. Redigera regeln och välj en annan.
rules-error-no-archive = Det här kontot har ingen arkivmapp. Redigera regeln så att den gör något annat.
rules-error-no-trash = Det här kontot har ingen papperskorg. Redigera regeln så att den gör något annat.
rules-error-cannot-send = Det här kontot kan inte skicka e-post, så regeln kan inte vidarebefordra den.
rules-error-other = { $error }. Redigera regeln och slå på den igen.

settings-folders = Mappar
settings-folders-summary = Antal olästa i mappfönstret
settings-folders-unread-counts = Antal olästa på varje mapp
settings-folders-unread-counts-detail = Av: bara Inkorgen visar hur många som är olästa

## A rule in one line, on its row: "From contains substack.com → skip the
## inbox, label Reading".

rules-summary = { $when } → { $then }
rules-summary-and = { $first } och { $next }
rules-summary-or = { $first } eller { $next }
rules-summary-more = { $count } till
rules-summary-list = { $first }, { $next }
rules-summary-condition = { $field } { $comparator } { $value }
rules-summary-has-attachment = Har en bilaga
rules-summary-no-attachment = Har ingen bilaga
rules-summary-mailing-list = Från en e-postlista
rules-summary-not-mailing-list = Inte från en e-postlista
rules-summary-tab = I fliken { $tab }
rules-summary-not-tab = Inte i fliken { $tab }
rules-summary-move = flytta till { $folder }
rules-summary-archive = hoppa över inkorgen
rules-summary-trash = flytta till papperskorgen
rules-summary-mark-read = markera som läst
rules-summary-star = stjärnmärk
rules-summary-important = markera som viktigt
rules-summary-label = etikettera { $label }
rules-summary-forward = vidarebefordra till { $address }
rules-summary-dont-notify = avisera inte
rules-summary-read-after = { $count ->
    [one] markera som läst efter { $count } dag
   *[other] markera som läst efter { $count } dagar
}
rules-summary-folder-gone = en mapp som inte finns längre

## The rule editor

rules-editor-new-title = Ny regel
rules-editor-edit-title = Redigera regel
rules-editor-name-hint = Regelns namn
rules-editor-when = När ett nytt mejl matchar
rules-editor-of-these = av dessa:
rules-mode-all = alla
rules-mode-any = något
rules-field-from = Från
rules-field-to = Till
rules-field-cc = Kopia
rules-field-any-recipient = Till eller Kopia
rules-field-reply-to = Svara till
rules-field-subject = Ämne
rules-field-body = Text
rules-field-attachment-name = Bilagans namn
rules-field-has-attachment = Har bilaga
rules-field-mailing-list = Från en e-postlista
rules-field-tab = Inkorgsflik
rules-comparator-contains = innehåller
rules-comparator-not-contains = innehåller inte
rules-comparator-begins-with = börjar med
rules-comparator-ends-with = slutar med
rules-comparator-equals = är exakt
rules-comparator-matches = matchar mönstret
rules-has-yes = ja
rules-has-no = nej
rules-editor-value-hint = Ord eller en adress
rules-editor-add-condition = Lägg till ett villkor
rules-editor-remove = Ta bort
rules-editor-then = Gör sedan:
rules-action-move = Flytta till
rules-action-archive = Hoppa över inkorgen (arkivera)
rules-action-trash = Flytta till papperskorgen
rules-action-mark-read = Markera som läst
rules-action-star = Stjärnmärk
rules-action-important = Markera som viktigt
rules-action-label = Lägg till etikett
rules-action-forward = Vidarebefordra till
rules-action-dont-notify = Avisera inte
rules-action-read-after = Markera som läst efter
rules-editor-choose-folder = Välj en mapp
rules-editor-choose-label = Välj en etikett
rules-editor-new-folder = Ny: { $name }
rules-editor-folder-of = { $folder } ({ $account })
rules-editor-forward-hint = E-postadress
rules-editor-days = dagar
rules-editor-add-action = Lägg till en åtgärd
rules-editor-stop = Stanna här: senare regler körs inte på det här mejlet
rules-editor-accounts = Konton:
rules-editor-accounts-none = Välj konton
rules-editor-accounts-many = { $count ->
    [one] { $count } konto
   *[other] { $count } konton
}
rules-editor-matches = Matchar { $mails } från de senaste { $days } dagarna
rules-editor-mails = { $count ->
    [one] { $count } mejl
   *[other] { $count } mejl
}
rules-editor-counting = Räknar e-posten som matchar…
rules-editor-show = Visa dem
rules-editor-also-apply = Tillämpa även på dessa { $count }
rules-editor-runs-katna = Körs i Katna, medan den här datorn är på.
rules-editor-runs-gmail = Körs i Gmail, så den fungerar även i din telefon och när den här datorn är av.
rules-editor-runs-sieve = Körs på din e-postserver, så den fungerar även i din telefon och när den här datorn är av.
rules-note-gmail-action = Körs i Katna: Gmail-filter kan inte ”{ $action }”.
rules-note-sieve-action = Körs i Katna: din e-postservers regler kan inte ”{ $action }”.
rules-note-test = { $field } { $comparator }
rules-note-gmail-condition = Körs i Katna: Gmail-filter kan inte testa ”{ $test }” som Katna gör.
rules-note-sieve-condition = Körs i Katna: din e-postservers regler kan inte testa ”{ $test }” som Katna gör.
rules-note-order = Körs i Katna, liksom en tidigare regel för kontot: regler körs i listans ordning.
rules-note-gmail-stop = Körs i Katna: Gmail-filter kan inte hindra senare regler från att köras.
rules-note-gmail-forward = Körs i Katna: Gmail vidarebefordrar bara till adresser som har verifierats i dess inställningar, och { $address } är inte en av dem.
rules-note-gmail-folder = Körs i Katna: Gmail har ingen etikett för en mapp som den här regeln använder.
rules-note-sieve-folder = Körs i Katna: din e-postserver har ingen mapp som den här regeln använder.
rules-note-gmail-sign-in = Körs i Katna tills du loggar in på Google igen och låter Katna skapa Gmail-filter.
rules-note-sieve-other-script = Körs i Katna: ett annat regelskript (”{ $name }”) är aktivt på din e-postserver.
rules-note-gmail-failed = Körs i Katna: Gmail godtog den inte ({ $error }).
rules-note-sieve-failed = Körs i Katna: din e-postserver godtog den inte ({ $error }).
rules-editor-cancel = Avbryt
rules-editor-save = Spara
rules-editor-saving = Sparar…
rules-editor-delete = Radera regel
rules-editor-delete-ask = Radera den här regeln?
rules-editor-delete-keep = Behåll den
rules-editor-delete-confirm = Radera
rules-editor-needs-folder = Välj en mapp för varje ”Flytta till” och en etikett för varje ”Lägg till etikett”.
rules-editor-needs-days = ”Markera som läst efter” kräver ett antal dagar, från 1 till 3650.
rules-saved = Regeln har sparats
rules-saved-applied = { $count ->
    [one] Regeln har sparats och tillämpats på { $count } mejl
   *[other] Regeln har sparats och tillämpats på { $count } mejl
}
rules-apply-failed = Regeln har sparats, men det gick inte att tillämpa den: { $error }
rules-deleted = Regeln har raderats
rules-delete-failed = Det gick inte att radera regeln: { $error }
rules-change-failed = Det gick inte att ändra reglerna: { $error }
