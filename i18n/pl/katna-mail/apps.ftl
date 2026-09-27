# Katna Mail, Polish (Polski).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## App rail (and the bottom bar on a phone)

rail-mail = Poczta
rail-calendar = Kalendarz
rail-contacts = Kontakty
rail-tasks = Zadania
rail-notes = Notatki
rail-feeds = Kanały

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = Wkrótce
app-calendar-promise = Twoje kalendarze CalDAV, zaproszenia na spotkania z poczty i przypomnienia – obok skrzynki odbiorczej.
app-tasks-promise = Listy zadań synchronizowane przez CalDAV i zadania tworzone z wiadomości.
app-notes-promise = Szybkie notatki oraz notatki do wiadomości lub wątku na później.
app-feeds-promise = Czytaj kanały RSS i Atom obok poczty.

## Contacts page

app-contacts-loading = Zbieranie osób z poczty…
app-contacts-empty = Tutaj pojawią się osoby, z którymi korespondujesz.
app-contacts-count = { $count ->
    [one] { $count } osoba z poczty, najczęstsi rozmówcy na początku
    [few] { $count } osoby z poczty, najczęstsi rozmówcy na początku
    [many] { $count } osób z poczty, najczęstsi rozmówcy na początku
   *[other] { $count } osoby z poczty, najczęstsi rozmówcy na początku
}
app-contacts-top = { $count ->
    [one] Najczęstszy rozmówca z poczty
    [few] Pierwsze { $count } osoby z poczty, najczęstsi rozmówcy na początku
    [many] Pierwszych { $count } osób z poczty, najczęstsi rozmówcy na początku
   *[other] Pierwsze { $count } osoby z poczty, najczęstsi rozmówcy na początku
}
app-contacts-messages = { $count ->
    [one] { $count } wiadomość
    [few] { $count } wiadomości
    [many] { $count } wiadomości
   *[other] { $count } wiadomości
}
app-contacts-last = ostatnio { $date }
