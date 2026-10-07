# Katna Mail, Afrikaans (Afrikaans).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Folders & rules

settings-rules = Reëls
settings-rules-summary = Sorteer, etiketteer, stuur aan of demp nuwe e-pos vanself
settings-rules-intro = Reëls sorteer nuwe e-pos vanself, in hierdie volgorde. Sleep om te herrangskik.
settings-rules-all-accounts = Alle rekeninge
settings-rules-new = Nuwe reël
settings-rules-none = Nog geen reëls nie. 'n Reël sorteer nuwe e-pos vanself: volgens sender, onderwerp of woorde.
settings-rules-none-account = Nog geen reëls vir hierdie rekening nie.
settings-rules-drag = Sleep om te herrangskik
settings-rules-edit = Wysig reël
settings-rules-turn-off = Skakel hierdie reël af
settings-rules-turn-on = Skakel hierdie reël aan

## Starter rules: offered under the user's own rules, switched off.
## Turning one on makes it one of the user's rules.

settings-rules-starters = Beginreëls
settings-rules-starters-intro = Af totdat jy een aanskakel. Hulle werk vir al jou rekeninge; wysig een om dit te verander.
settings-rules-starter-turning-on = Skakel tans “{ $name }” aan…
settings-rules-starter-failed = Kon nie “{ $name }” aanskakel nie: { $error }
rules-starter-promotions = Stil promosies
rules-starter-newsletters = Nuusbriewe na Leesstof
rules-starter-receipts = Kwitansies en fakture
rules-starter-deliveries = Aflewerings
rules-starter-train = Treinkaartjies
rules-starter-flight = Vliegkaartjies
rules-starter-codes = Eenmalige kodes
rules-starter-security = Sekuriteitswaarskuwings
rules-starter-social = Sosiale e-pos
rules-starter-invites = Kalenderuitnodigings
rules-starter-folder-reading = Leesstof
rules-starter-folder-receipts = Kwitansies
rules-starter-folder-deliveries = Aflewerings
rules-starter-folder-travel = Reis
rules-starter-folder-social = Sosiaal
rules-runs-katna = Loop in Katna
rules-runs-gmail = Loop op Gmail
rules-runs-sieve = Loop op die bediener
rules-stopped = Gestop
rules-error-folder-gone = Die vouer wat hierdie reël gebruik, bestaan nie meer nie. Wysig die reël om 'n ander te kies.
rules-error-no-archive = Hierdie rekening het geen argiefvouer nie. Wysig die reël om iets anders te doen.
rules-error-no-trash = Hierdie rekening het geen Asblik-vouer nie. Wysig die reël om iets anders te doen.
rules-error-cannot-send = Hierdie rekening kan nie e-pos stuur nie, so die reël kan dit nie aanstuur nie.
rules-error-other = { $error }. Wysig die reël en skakel dit weer aan.

settings-folders = Vouers
settings-folders-summary = Ongeleesde tellings in die vouerpaneel
settings-folders-unread-counts = Ongeleesde telling op elke vouer
settings-folders-unread-counts-detail = Af: net Inkassie wys hoeveel ongelees is

## A rule in one line, on its row: "From contains substack.com → skip the
## inbox, label Reading".

rules-summary = { $when } → { $then }
rules-summary-and = { $first } en { $next }
rules-summary-or = { $first } of { $next }
rules-summary-more = { $count } meer
rules-summary-list = { $first }, { $next }
rules-summary-condition = { $field } { $comparator } { $value }
rules-summary-has-attachment = Het 'n aanhegsel
rules-summary-no-attachment = Het geen aanhegsel nie
rules-summary-mailing-list = Van 'n poslys
rules-summary-not-mailing-list = Nie van 'n poslys nie
rules-summary-tab = In die { $tab }-oortjie
rules-summary-not-tab = Nie in die { $tab }-oortjie nie
rules-summary-move = skuif na { $folder }
rules-summary-archive = slaan die inkassie oor
rules-summary-trash = skuif na die asblik
rules-summary-mark-read = merk as gelees
rules-summary-star = gee 'n ster
rules-summary-important = merk as belangrik
rules-summary-label = etiketteer { $label }
rules-summary-forward = stuur aan na { $address }
rules-summary-dont-notify = moenie kennis gee nie
rules-summary-read-after = { $count ->
    [one] merk as gelees na { $count } dag
   *[other] merk as gelees na { $count } dae
}
rules-summary-folder-gone = 'n vouer wat weg is

## The rule editor

rules-editor-new-title = Nuwe reël
rules-editor-edit-title = Wysig reël
rules-editor-name-hint = Reëlnaam
rules-editor-when = Wanneer 'n nuwe e-pos ooreenstem met
rules-editor-of-these = van hierdie:
rules-mode-all = al
rules-mode-any = enige
rules-field-from = Van
rules-field-to = Aan
rules-field-cc = Cc
rules-field-any-recipient = Aan of Cc
rules-field-reply-to = Antwoord-aan
rules-field-subject = Onderwerp
rules-field-body = Teks
rules-field-attachment-name = Aanhegselnaam
rules-field-has-attachment = Het aanhegsel
rules-field-mailing-list = Van 'n poslys
rules-field-tab = Inkassie-oortjie
rules-comparator-contains = bevat
rules-comparator-not-contains = bevat nie
rules-comparator-begins-with = begin met
rules-comparator-ends-with = eindig met
rules-comparator-equals = is presies
rules-comparator-matches = pas by die patroon
rules-has-yes = ja
rules-has-no = nee
rules-editor-value-hint = Woorde of 'n adres
rules-editor-add-condition = Voeg 'n voorwaarde by
rules-editor-remove = Verwyder
rules-editor-then = Dan:
rules-action-move = Skuif na
rules-action-archive = Slaan die inkassie oor (argiveer)
rules-action-trash = Skuif na die asblik
rules-action-mark-read = Merk as gelees
rules-action-star = Gee 'n ster
rules-action-important = Merk as belangrik
rules-action-label = Voeg etiket by
rules-action-forward = Stuur aan na
rules-action-dont-notify = Moenie kennis gee nie
rules-action-read-after = Merk as gelees na
rules-editor-choose-folder = Kies 'n vouer
rules-editor-choose-label = Kies 'n etiket
rules-editor-new-folder = Nuut: { $name }
rules-editor-folder-of = { $folder } ({ $account })
rules-editor-forward-hint = E-posadres
rules-editor-days = dae
rules-editor-add-action = Voeg 'n aksie by
rules-editor-stop = Stop hier: latere reëls loop nie op hierdie e-pos nie
rules-editor-accounts = Rekeninge:
rules-editor-accounts-none = Kies rekeninge
rules-editor-accounts-many = { $count ->
    [one] { $count } rekening
   *[other] { $count } rekeninge
}
rules-editor-matches = Pas by { $mails } van die afgelope { $days } dae
rules-editor-mails = { $count ->
    [one] { $count } e-pos
   *[other] { $count } e-posse
}
rules-editor-counting = Tel tans die e-pos wat dit pas…
rules-editor-show = Wys hulle
rules-editor-also-apply = Pas ook toe op hierdie { $count }
rules-editor-runs-katna = Loop in Katna, terwyl hierdie rekenaar aan is.
rules-editor-runs-gmail = Loop op Gmail, so dit werk ook op jou foon en met hierdie rekenaar af.
rules-editor-runs-sieve = Loop op jou e-posbediener, so dit werk ook op jou foon en met hierdie rekenaar af.
rules-note-gmail-action = Loop in Katna: Gmail-filters kan nie “{ $action }” doen nie.
rules-note-sieve-action = Loop in Katna: jou e-posbediener se reëls kan nie “{ $action }” doen nie.
rules-note-test = { $field } { $comparator }
rules-note-gmail-condition = Loop in Katna: Gmail-filters kan nie “{ $test }” toets soos Katna doen nie.
rules-note-sieve-condition = Loop in Katna: jou e-posbediener se reëls kan nie “{ $test }” toets soos Katna doen nie.
rules-note-order = Loop in Katna, soos 'n vroeëre reël van die rekening: reëls loop in lysvolgorde.
rules-note-gmail-stop = Loop in Katna: Gmail-filters kan nie keer dat latere reëls loop nie.
rules-note-gmail-forward = Loop in Katna: Gmail stuur net aan na adresse wat in sy instellings bevestig is, en { $address } is nie een nie.
rules-note-gmail-folder = Loop in Katna: Gmail het geen etiket vir 'n vouer wat hierdie reël gebruik nie.
rules-note-sieve-folder = Loop in Katna: jou e-posbediener het nie 'n vouer wat hierdie reël gebruik nie.
rules-note-gmail-sign-in = Loop in Katna totdat jy weer by Google aanmeld en Katna toelaat om Gmail-filters te maak.
rules-note-sieve-other-script = Loop in Katna: 'n ander reëlskrip (“{ $name }”) is aktief op jou e-posbediener.
rules-note-gmail-failed = Loop in Katna: Gmail het dit nie aanvaar nie ({ $error }).
rules-note-sieve-failed = Loop in Katna: jou e-posbediener het dit nie aanvaar nie ({ $error }).
rules-editor-cancel = Kanselleer
rules-editor-save = Stoor
rules-editor-saving = Stoor tans…
rules-editor-delete = Vee reël uit
rules-editor-delete-ask = Vee hierdie reël uit?
rules-editor-delete-keep = Hou dit
rules-editor-delete-confirm = Vee uit
rules-editor-needs-folder = Kies 'n vouer vir elke “Skuif na” en 'n etiket vir elke “Voeg etiket by”.
rules-editor-needs-days = “Merk as gelees na” neem 'n aantal dae, van 1 tot 3650.
rules-saved = Reël gestoor
rules-saved-applied = { $count ->
    [one] Reël gestoor en op { $count } e-pos toegepas
   *[other] Reël gestoor en op { $count } e-posse toegepas
}
rules-apply-failed = Reël gestoor, maar om dit toe te pas, het misluk: { $error }
rules-deleted = Reël uitgevee
rules-delete-failed = Kon nie die reël uitvee nie: { $error }
rules-change-failed = Kon nie die reëls verander nie: { $error }
