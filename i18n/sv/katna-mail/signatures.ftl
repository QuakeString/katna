# Katna Mail, Swedish (Svenska).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The signature being edited

signature-placeholder = Ditt namn, och det du vill lägga till under det

## Its formatting bar

signature-bold = Fet
signature-italic = Kursiv
signature-underline = Understruken
signature-link = Länk
signature-link-apply = Använd
signature-picture = Infoga bild
signature-align-left = Vänsterjustera
signature-align-center = Centrera
signature-align-right = Högerjustera
signature-numbered-list = Numrerad lista
signature-bulleted-list = Punktlista
signature-remove-formatting = Ta bort formatering

## Adding a picture

signature-picture-choose = Infoga
signature-picture-too-big = Bilder i en signatur får vara upp till { $size }.
signature-picture-kind = Välj en PNG-, JPEG-, GIF- eller WebP-bild.
signature-picture-unreadable = { $name }: { $error }

## Layouts

signature-layout = Layout
signature-layout-own = Din egen
signature-layout-classic = Klassisk
signature-layout-logo-left = Logotyp till vänster
signature-layout-photo = Foto
signature-layout-band = Färgband
signature-layout-one-line = En rad
signature-layout-centred = Centrerad
signature-layout-banner = Med banner
signature-layout-underline = Understrykning
signature-layout-side-bar = Sidolist
signature-layout-card = Kort
signature-layout-monogram = Monogram
signature-layout-plain = Oformaterad text
signature-layout-mobile-label = M:
signature-layout-office-label = T:
signature-layout-email-label = E:
signature-layout-name = Namn
signature-layout-job = Titel
signature-layout-company = Företag
signature-layout-mobile = Mobil
signature-layout-office = Kontor
signature-layout-email = E-post
signature-layout-website = Webbplats
signature-layout-address = Adress
signature-layout-pictures = Bilder
signature-layout-logo = Logotyp
signature-layout-photo-picture = Foto
signature-layout-banner-picture = Banner
signature-layout-remove-picture = Ta bort
signature-layout-pages = Sidor
signature-layout-page-placeholder = Lägg till en sidas adress
signature-layout-colour = Färg
signature-layout-picture-failed = { $name } kunde inte användas som bild.
signature-layout-preview = Så ser mottagaren den
signature-layout-light = Ljus
signature-layout-dark = Mörk
signature-layout-text = Oformaterad text
signature-layout-inside = Bilderna skickas inuti mejlet, så de visas även där bilder från webben är avstängda. Den här lägger till { $size } i varje mejl.
signature-layout-edit = Redigera för hand
signature-layout-edit-confirm = Redigera den för hand? Dess fält och layout försvinner, och den behåller sitt utseende så långt redigeraren klarar.
signature-layout-use-confirm = Använda layouten { $layout }? Den ersätter den här signaturen och fylls i utifrån den.
signature-layout-use = Använd layout
signature-layout-cancel = Avbryt

## Paste HTML

signature-html-title = Klistra in HTML
signature-html-subtitle = För en signatur du har utformat någon annanstans
signature-html-placeholder = Klistra in signaturens HTML här
signature-html-name = Inklistrad
signature-html-new = Sparas som en ny signatur, ”{ $name }”
signature-html-replaces = Sparas över ”{ $name }”
signature-html-cancel = Avbryt
signature-html-save = Spara
signature-html-fetching = Hämtar dess bilder…
signature-html-pictures-inside = { $count ->
    [one] { $count } bild hämtades och lades inuti mejlet ({ $size })
   *[other] { $count } bilder hämtades och lades inuti mejlet ({ $size })
}
signature-html-pictures-web = { $count ->
    [one] { $count } bild kunde inte hämtas, så mottagarna läser in den från webben
   *[other] { $count } bilder kunde inte hämtas, så mottagarna läser in dem från webben
}
signature-html-removed = Skript, formulär och spårningspixlar har tagits bort, eftersom e-postappar blockerar dem ändå
signature-html-style-sheet = En stilmall utelämnades: e-post behåller bara stilarna som är skrivna på varje del
signature-html-links = Länkar som ledde någon annanstans än till en webbplats, en adress eller ett telefonnummer har tagits bort
signature-html-plain-text = En version med oformaterad text har skapats utifrån den, för e-postappar som bara visar text

## Import

signature-import-title = Importera
signature-import-subtitle = Från Gmail, Thunderbird, Evolution och KMail
signature-import-looking = Letar efter signaturer…
signature-import-none = Inga signaturer hittades. För en annan app, kopiera signaturens HTML och använd Klistra in HTML.
signature-import-from = Från { $app }
signature-import-already = finns redan i Katna
signature-import-gmail-sign-in = { $address }: logga in igen i Inställningar > Konton så att Katna får läsa Gmails signaturer.
signature-import-gmail-failed = { $address }: { $error }
signature-import-cancel = Avbryt
signature-import-do = { $count ->
    [one] Importera { $count } signatur
   *[other] Importera { $count } signaturer
}
signature-import-name = { $name } ({ $app })
