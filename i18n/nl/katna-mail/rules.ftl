# Katna Mail, Dutch (Nederlands).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Folders & rules

settings-rules = Regels
settings-rules-summary = Nieuwe e-mail vanzelf sorteren, labelen, doorsturen of stil houden
settings-rules-intro = Regels sorteren nieuwe e-mail vanzelf, in deze volgorde. Sleep om de volgorde te wijzigen.
settings-rules-all-accounts = Alle accounts
settings-rules-new = Nieuwe regel
settings-rules-none = Nog geen regels. Een regel sorteert nieuwe e-mail vanzelf: op afzender, onderwerp of woorden.
settings-rules-none-account = Nog geen regels voor dit account.
settings-rules-drag = Sleep om de volgorde te wijzigen
settings-rules-edit = Regel bewerken
settings-rules-turn-off = Deze regel uitzetten
settings-rules-turn-on = Deze regel aanzetten

## Starter rules: offered under the user's own rules, switched off.
## Turning one on makes it one of the user's rules.

settings-rules-starters = Startregels
settings-rules-starters-intro = Uit totdat je er een aanzet. Ze werken voor al je accounts; bewerk er een om hem te wijzigen.
settings-rules-starter-turning-on = “{ $name }” wordt aangezet…
settings-rules-starter-failed = Kan “{ $name }” niet aanzetten: { $error }
rules-starter-promotions = Reclame stil houden
rules-starter-newsletters = Nieuwsbrieven naar Lezen
rules-starter-receipts = Bonnen en facturen
rules-starter-deliveries = Bezorgingen
rules-starter-train = Treinkaartjes
rules-starter-flight = Vliegtickets
rules-starter-codes = Eenmalige codes
rules-starter-security = Beveiligingsmeldingen
rules-starter-social = Sociale e-mail
rules-starter-invites = Agenda-uitnodigingen
rules-starter-folder-reading = Lezen
rules-starter-folder-receipts = Bonnen
rules-starter-folder-deliveries = Bezorgingen
rules-starter-folder-travel = Reizen
rules-starter-folder-social = Sociaal
rules-runs-katna = Draait in Katna
rules-runs-gmail = Draait op Gmail
rules-runs-sieve = Draait op de server
rules-stopped = Gestopt
rules-error-folder-gone = De map die deze regel gebruikt, bestaat niet meer. Bewerk de regel om een andere te kiezen.
rules-error-no-archive = Dit account heeft geen archiefmap. Bewerk de regel om iets anders te doen.
rules-error-no-trash = Dit account heeft geen Prullenbak. Bewerk de regel om iets anders te doen.
rules-error-cannot-send = Dit account kan geen e-mail versturen, dus de regel kan niet doorsturen.
rules-error-other = { $error }. Bewerk de regel en zet hem weer aan.

settings-folders = Mappen
settings-folders-summary = Aantallen ongelezen in het mappenvenster
settings-folders-unread-counts = Aantal ongelezen bij elke map
settings-folders-unread-counts-detail = Uit: alleen Inbox toont hoeveel er ongelezen zijn

## A rule in one line, on its row: "From contains substack.com → skip the
## inbox, label Reading".

rules-summary = { $when } → { $then }
rules-summary-and = { $first } en { $next }
rules-summary-or = { $first } of { $next }
rules-summary-more = nog { $count }
rules-summary-list = { $first }, { $next }
rules-summary-condition = { $field } { $comparator } { $value }
rules-summary-has-attachment = Heeft een bijlage
rules-summary-no-attachment = Heeft geen bijlage
rules-summary-mailing-list = Van een mailinglijst
rules-summary-not-mailing-list = Niet van een mailinglijst
rules-summary-tab = In het tabblad { $tab }
rules-summary-not-tab = Niet in het tabblad { $tab }
rules-summary-move = verplaatsen naar { $folder }
rules-summary-archive = inbox overslaan
rules-summary-trash = naar de Prullenbak verplaatsen
rules-summary-mark-read = als gelezen markeren
rules-summary-star = ster geven
rules-summary-important = als belangrijk markeren
rules-summary-label = label { $label } geven
rules-summary-forward = doorsturen naar { $address }
rules-summary-dont-notify = geen melding geven
rules-summary-read-after = { $count ->
    [one] als gelezen markeren na { $count } dag
   *[other] als gelezen markeren na { $count } dagen
}
rules-summary-folder-gone = een map die niet meer bestaat

## The rule editor

rules-editor-new-title = Nieuwe regel
rules-editor-edit-title = Regel bewerken
rules-editor-name-hint = Naam van de regel
rules-editor-when = Als een nieuwe e-mail voldoet aan
rules-editor-of-these = van deze:
rules-mode-all = alle
rules-mode-any = een
rules-field-from = Van
rules-field-to = Aan
rules-field-cc = Cc
rules-field-any-recipient = Aan of Cc
rules-field-reply-to = Antwoord aan
rules-field-subject = Onderwerp
rules-field-body = Tekst
rules-field-attachment-name = Naam van bijlage
rules-field-has-attachment = Heeft bijlage
rules-field-mailing-list = Van een mailinglijst
rules-field-tab = Inbox-tabblad
rules-comparator-contains = bevat
rules-comparator-not-contains = bevat niet
rules-comparator-begins-with = begint met
rules-comparator-ends-with = eindigt op
rules-comparator-equals = is precies
rules-comparator-matches = komt overeen met het patroon
rules-has-yes = ja
rules-has-no = nee
rules-editor-value-hint = Woorden of een adres
rules-editor-add-condition = Een voorwaarde toevoegen
rules-editor-remove = Verwijderen
rules-editor-then = Dan:
rules-action-move = Verplaatsen naar
rules-action-archive = Inbox overslaan (archiveren)
rules-action-trash = Naar de Prullenbak verplaatsen
rules-action-mark-read = Als gelezen markeren
rules-action-star = Ster geven
rules-action-important = Als belangrijk markeren
rules-action-label = Label toevoegen
rules-action-forward = Doorsturen naar
rules-action-dont-notify = Geen melding geven
rules-action-read-after = Als gelezen markeren na
rules-editor-choose-folder = Kies een map
rules-editor-choose-label = Kies een label
rules-editor-new-folder = Nieuw: { $name }
rules-editor-folder-of = { $folder } ({ $account })
rules-editor-forward-hint = E-mailadres
rules-editor-days = dagen
rules-editor-add-action = Een actie toevoegen
rules-editor-stop = Hier stoppen: latere regels worden niet op deze e-mail uitgevoerd
rules-editor-accounts = Accounts:
rules-editor-accounts-none = Kies accounts
rules-editor-accounts-many = { $count ->
    [one] { $count } account
   *[other] { $count } accounts
}
rules-editor-matches = Komt overeen met { $mails } van de afgelopen { $days } dagen
rules-editor-mails = { $count ->
    [one] { $count } e-mail
   *[other] { $count } e-mails
}
rules-editor-counting = De e-mail tellen die overeenkomt…
rules-editor-show = Tonen
rules-editor-also-apply = Ook toepassen op deze { $count }
rules-editor-runs-katna = Draait in Katna, zolang deze computer aan staat.
rules-editor-runs-gmail = Draait op Gmail, dus werkt ook op je telefoon en als deze computer uit staat.
rules-editor-runs-sieve = Draait op je mailserver, dus werkt ook op je telefoon en als deze computer uit staat.
rules-note-gmail-action = Draait in Katna: Gmail-filters kunnen “{ $action }” niet uitvoeren.
rules-note-sieve-action = Draait in Katna: de regels van je mailserver kunnen “{ $action }” niet uitvoeren.
rules-note-test = { $field } { $comparator }
rules-note-gmail-condition = Draait in Katna: Gmail-filters kunnen “{ $test }” niet testen zoals Katna dat doet.
rules-note-sieve-condition = Draait in Katna: de regels van je mailserver kunnen “{ $test }” niet testen zoals Katna dat doet.
rules-note-order = Draait in Katna, net als een eerdere regel van het account: regels worden in de volgorde van de lijst uitgevoerd.
rules-note-gmail-stop = Draait in Katna: Gmail-filters kunnen latere regels niet tegenhouden.
rules-note-gmail-forward = Draait in Katna: Gmail stuurt alleen door naar adressen die in de instellingen van Gmail zijn geverifieerd, en { $address } is daar niet een van.
rules-note-gmail-folder = Draait in Katna: Gmail heeft geen label voor een map die deze regel gebruikt.
rules-note-sieve-folder = Draait in Katna: je mailserver heeft geen map die deze regel gebruikt.
rules-note-gmail-sign-in = Draait in Katna totdat je opnieuw bij Google inlogt en Katna Gmail-filters laat maken.
rules-note-sieve-other-script = Draait in Katna: een ander regelscript (“{ $name }”) is actief op je mailserver.
rules-note-gmail-failed = Draait in Katna: Gmail heeft hem niet geaccepteerd ({ $error }).
rules-note-sieve-failed = Draait in Katna: je mailserver heeft hem niet geaccepteerd ({ $error }).
rules-editor-cancel = Annuleren
rules-editor-save = Opslaan
rules-editor-saving = Opslaan…
rules-editor-delete = Regel verwijderen
rules-editor-delete-ask = Deze regel verwijderen?
rules-editor-delete-keep = Behouden
rules-editor-delete-confirm = Verwijderen
rules-editor-needs-folder = Kies een map voor elke “Verplaatsen naar” en een label voor elke “Label toevoegen”.
rules-editor-needs-days = “Als gelezen markeren na” vraagt een aantal dagen, van 1 tot 3650.
rules-saved = Regel opgeslagen
rules-saved-applied = { $count ->
    [one] Regel opgeslagen en toegepast op { $count } e-mail
   *[other] Regel opgeslagen en toegepast op { $count } e-mails
}
rules-apply-failed = Regel opgeslagen, maar toepassen is mislukt: { $error }
rules-deleted = Regel verwijderd
rules-delete-failed = Kan de regel niet verwijderen: { $error }
rules-change-failed = Kan de regels niet wijzigen: { $error }
