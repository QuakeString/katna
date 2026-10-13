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

## Rail right-click menu

rail-menu-open = Öppna { $app }
rail-menu-settings = Inställningar för { $app }
rail-menu-turn-off = Stäng av { $app }…

## Turning an app off (Settings > Apps)

app-off-title = Stänga av { $app }?
app-off-body = Katna slutar synkronisera { $app } och tar bort det från:
app-off-keep = Behåll en kopia på den här datorn
app-off-keep-detail = Det går direkt att slå på det igen
app-off-remove = Ta bort kopian på den här datorn
app-off-remove-detail = Ingenting ändras på dina konton, och när du slår på det igen hämtas allt på nytt. Det som bara finns på den här datorn, eller inte har skickats än, blir kvar.
app-off-cancel = Avbryt
app-off-confirm = Stäng av
app-off-done = { $app } stängdes av
app-off-note = { $app } är avstängt
app-off-turn-on = Slå på
app-off-leaves-calendar-rail = Appfältet och Ctrl+2
app-off-leaves-calendar-agenda = Agendan bredvid din e-post
app-off-leaves-calendar-meeting = Boka möte, och Öppna i Kalender på inbjudningar
app-off-leaves-calendar-reminders = Påminnelser om händelser
app-off-leaves-calendar-desktop = Händelser i KRunner och skrivbordets klocka
app-off-leaves-contacts-rail = Appfältet och Ctrl+3
app-off-leaves-contacts-card = Lägg till i kontakter på en avsändares kort
app-off-leaves-contacts-birthdays = Födelsedagar i Kalender
app-off-leaves-tasks-rail = Appfältet och Ctrl+4
app-off-leaves-tasks-mail = Lägg till i Uppgifter på e-post, och Shift+T
app-off-leaves-tasks-calendar = Uppgifter i Kalender
app-off-leaves-tasks-tray = Ny uppgift i systemfältet, och Meta+Alt+T
app-off-leaves-tasks-reminders = Påminnelser om uppgifter
app-off-leaves-notes-rail = Appfältet och Ctrl+5
app-off-leaves-notes-mail = Lägg till en anteckning på e-post
app-off-leaves-notes-meetings = Mötesanteckningar på händelser
app-off-leaves-notes-tray = Ny anteckning i systemfältet, och Meta+Alt+N
app-off-leaves-notes-reminders = Påminnelser om anteckningar
app-off-leaves-files-rail = Appfältet och Ctrl+7
app-off-leaves-files-compose = Filer när du bifogar i Skriv

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
