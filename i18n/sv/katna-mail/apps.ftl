# Katna Mail, Swedish (Svenska).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

top-brand = Katna

## App rail (and the bottom bar on a phone)

rail-mail = E-post
rail-calendar = Kalender
rail-contacts = Kontakter
rail-tasks = Uppgifter
rail-notes = Anteckningar
rail-files = Filer

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = Kommer snart
app-calendar-promise = Dina CalDAV-kalendrar, mötesinbjudningar från din e-post och påminnelser, bredvid inkorgen.
app-tasks-promise = Att göra-listor som synkroniseras med CalDAV, och uppgifter som skapats från e-post.
app-notes-promise = Snabba anteckningar, och anteckningar om ett meddelande eller en konversation till senare.

## Contacts page

app-contacts-loading = Samlar personer från din e-post…
app-contacts-empty = Personer du skriver med visas här.
app-contacts-count = { $count ->
    [one] { $count } person från din e-post, de du skriver mest med först
   *[other] { $count } personer från din e-post, de du skriver mest med först
}
app-contacts-top = { $count ->
    [one] Personen du skriver mest med
   *[other] De { $count } personer du skriver mest med, flest först
}
app-contacts-messages = { $count ->
    [one] { $count } meddelande
   *[other] { $count } meddelanden
}
app-contacts-last = senast { $date }
