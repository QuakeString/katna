# Katna Mail, Dutch (Nederlands).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The signature being edited

signature-placeholder = Je naam, en eventueel iets daaronder

## Its formatting bar

signature-bold = Vet
signature-italic = Cursief
signature-underline = Onderstrepen
signature-link = Link
signature-link-apply = Toepassen
signature-picture = Afbeelding invoegen
signature-align-left = Links uitlijnen
signature-align-center = Centreren
signature-align-right = Rechts uitlijnen
signature-numbered-list = Genummerde lijst
signature-bulleted-list = Lijst met opsommingstekens
signature-remove-formatting = Opmaak wissen

## Adding a picture

signature-picture-choose = Invoegen
signature-picture-too-big = Afbeeldingen in een handtekening mogen maximaal { $size } zijn.
signature-picture-kind = Kies een PNG-, JPEG-, GIF- of WebP-afbeelding.
signature-picture-unreadable = { $name }: { $error }

## Layouts

signature-layout = Indeling
signature-layout-own = Je eigen
signature-layout-classic = Klassiek
signature-layout-logo-left = Logo links
signature-layout-photo = Foto
signature-layout-band = Kleurband
signature-layout-one-line = Eén regel
signature-layout-centred = Gecentreerd
signature-layout-banner = Met banner
signature-layout-underline = Onderstreept
signature-layout-side-bar = Zijbalk
signature-layout-card = Kaart
signature-layout-monogram = Monogram
signature-layout-plain = Platte tekst
signature-layout-mobile-label = M:
signature-layout-office-label = T:
signature-layout-email-label = E:
signature-layout-name = Naam
signature-layout-job = Functie
signature-layout-company = Bedrijf
signature-layout-mobile = Mobiel
signature-layout-office = Kantoor
signature-layout-email = E-mail
signature-layout-website = Website
signature-layout-address = Adres
signature-layout-pictures = Afbeeldingen
signature-layout-logo = Logo
signature-layout-photo-picture = Foto
signature-layout-banner-picture = Banner
signature-layout-remove-picture = Verwijderen
signature-layout-pages = Pagina’s
signature-layout-page-placeholder = Het adres van een pagina toevoegen
signature-layout-colour = Kleur
signature-layout-picture-failed = { $name } kan niet als afbeelding worden gebruikt.
signature-layout-preview = Zo ziet de lezer het
signature-layout-light = Licht
signature-layout-dark = Donker
signature-layout-text = Platte tekst
signature-layout-inside = Afbeeldingen worden in de e-mail zelf meegestuurd, zodat ze ook zichtbaar zijn waar afbeeldingen van internet uit staan. Deze voegt { $size } toe aan elke e-mail.
signature-layout-edit = Handmatig bewerken
signature-layout-edit-confirm = Handmatig bewerken? De velden en indeling verdwijnen, en het uiterlijk blijft zo goed als de editor het kan vasthouden.
signature-layout-use-confirm = De indeling { $layout } gebruiken? Die vervangt deze handtekening, ingevuld met de gegevens ervan.
signature-layout-use = Indeling gebruiken
signature-layout-cancel = Annuleren

## Paste HTML

signature-html-title = HTML plakken
signature-html-subtitle = Voor een handtekening die je elders hebt ontworpen
signature-html-placeholder = Plak hier de HTML van de handtekening
signature-html-name = Geplakt
signature-html-new = Opgeslagen als nieuwe handtekening, “{ $name }”
signature-html-replaces = Overschrijft “{ $name }”
signature-html-cancel = Annuleren
signature-html-save = Opslaan
signature-html-fetching = De afbeeldingen worden gedownload…
signature-html-pictures-inside = { $count ->
    [one] { $count } afbeelding gedownload en in de e-mail zelf gezet ({ $size })
   *[other] { $count } afbeeldingen gedownload en in de e-mail zelf gezet ({ $size })
}
signature-html-pictures-web = { $count ->
    [one] { $count } afbeelding kon niet worden gedownload, dus lezers laden die van internet
   *[other] { $count } afbeeldingen konden niet worden gedownload, dus lezers laden die van internet
}
signature-html-removed = Scripts, formulieren en trackingpixels verwijderd, die e-mailapps toch blokkeren
signature-html-style-sheet = Een stylesheet weggelaten: e-mail houdt alleen de stijlen die bij elk onderdeel zelf staan
signature-html-links = Links verwijderd die niet naar een website, een adres of een telefoonnummer gingen
signature-html-plain-text = Er is een versie in platte tekst van gemaakt, voor e-mailapps die alleen tekst tonen

## Import

signature-import-title = Importeren
signature-import-subtitle = Uit Gmail, Thunderbird, Evolution en KMail
signature-import-looking = Handtekeningen zoeken…
signature-import-none = Geen handtekeningen gevonden. Kopieer voor een andere app de HTML van de handtekening en gebruik HTML plakken.
signature-import-from = Uit { $app }
signature-import-already = al in Katna
signature-import-gmail-sign-in = { $address }: meld je opnieuw aan in Instellingen > Accounts, zodat Katna de handtekeningen van Gmail mag lezen.
signature-import-gmail-failed = { $address }: { $error }
signature-import-cancel = Annuleren
signature-import-do = { $count ->
    [one] { $count } handtekening importeren
   *[other] { $count } handtekeningen importeren
}
signature-import-name = { $name } ({ $app })
