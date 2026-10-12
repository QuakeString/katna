# Katna Mail, German (Deutsch).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The signature being edited

signature-placeholder = Ihr Name und alles, was darunter stehen soll

## Its formatting bar

signature-bold = Fett
signature-italic = Kursiv
signature-underline = Unterstrichen
signature-link = Link
signature-link-apply = Übernehmen
signature-picture = Bild einfügen
signature-align-left = Linksbündig
signature-align-center = Zentriert
signature-align-right = Rechtsbündig
signature-numbered-list = Nummerierte Liste
signature-bulleted-list = Aufzählung
signature-remove-formatting = Formatierung entfernen

## Adding a picture

signature-picture-choose = Einfügen
signature-picture-too-big = Bilder in einer Signatur dürfen bis zu { $size } groß sein.
signature-picture-kind = Wählen Sie ein PNG-, JPEG-, GIF- oder WebP-Bild.
signature-picture-unreadable = { $name }: { $error }

## Layouts

signature-layout = Layout
signature-layout-own = Eigene
signature-layout-classic = Klassisch
signature-layout-logo-left = Logo links
signature-layout-photo = Foto
signature-layout-band = Farbband
signature-layout-one-line = Einzeilig
signature-layout-centred = Zentriert
signature-layout-banner = Mit Banner
signature-layout-underline = Unterstrichen
signature-layout-side-bar = Seitenleiste
signature-layout-card = Karte
signature-layout-monogram = Monogramm
signature-layout-plain = Nur Text
signature-layout-mobile-label = M:
signature-layout-office-label = T:
signature-layout-email-label = E:
signature-layout-name = Name
signature-layout-job = Position
signature-layout-company = Unternehmen
signature-layout-mobile = Mobil
signature-layout-office = Büro
signature-layout-email = E-Mail
signature-layout-website = Website
signature-layout-address = Adresse
signature-layout-pictures = Bilder
signature-layout-logo = Logo
signature-layout-photo-picture = Foto
signature-layout-banner-picture = Banner
signature-layout-remove-picture = Entfernen
signature-layout-pages = Seiten
signature-layout-page-placeholder = Adresse einer Seite hinzufügen
signature-layout-colour = Farbe
signature-layout-picture-failed = { $name } konnte nicht als Bild verwendet werden.
signature-layout-preview = So sieht es der Empfänger
signature-layout-light = Hell
signature-layout-dark = Dunkel
signature-layout-text = Nur Text
signature-layout-inside = Bilder werden in der E-Mail mitgesendet und erscheinen daher auch dort, wo Bilder aus dem Web aus sind. Dieses fügt jeder E-Mail { $size } hinzu.
signature-layout-edit = Von Hand bearbeiten
signature-layout-edit-confirm = Von Hand bearbeiten? Felder und Layout entfallen; das Aussehen bleibt erhalten, soweit der Editor es darstellen kann.
signature-layout-use-confirm = Das Layout { $layout } verwenden? Es ersetzt diese Signatur und wird aus ihr ausgefüllt.
signature-layout-use = Layout verwenden
signature-layout-cancel = Abbrechen

## Paste HTML

signature-html-title = HTML einfügen
signature-html-subtitle = Für eine anderswo gestaltete Signatur
signature-html-placeholder = Fügen Sie hier den HTML-Code der Signatur ein
signature-html-name = Eingefügt
signature-html-new = Wird als neue Signatur „{ $name }“ gespeichert
signature-html-replaces = Überschreibt „{ $name }“
signature-html-cancel = Abbrechen
signature-html-save = Speichern
signature-html-fetching = Bilder werden heruntergeladen…
signature-html-pictures-inside = { $count ->
    [one] { $count } Bild heruntergeladen und in die E-Mail eingebettet ({ $size })
   *[other] { $count } Bilder heruntergeladen und in die E-Mail eingebettet ({ $size })
}
signature-html-pictures-web = { $count ->
    [one] { $count } Bild konnte nicht heruntergeladen werden, daher laden Empfänger es aus dem Web
   *[other] { $count } Bilder konnten nicht heruntergeladen werden, daher laden Empfänger sie aus dem Web
}
signature-html-removed = Skripte, Formulare und Tracking-Pixel entfernt, die E-Mail-Apps ohnehin blockieren
signature-html-style-sheet = Ein Stylesheet weggelassen: E-Mails behalten nur die Stile, die direkt an jedem Element stehen
signature-html-links = Links entfernt, die nicht zu einer Website, einer E-Mail-Adresse oder einer Telefonnummer führten
signature-html-plain-text = Daraus eine Nur-Text-Version erstellt, für E-Mail-Apps, die nur Text anzeigen

## Import

signature-import-title = Importieren
signature-import-subtitle = Aus Gmail, Thunderbird, Evolution und KMail
signature-import-looking = Signaturen werden gesucht…
signature-import-none = Keine Signaturen gefunden. Kopieren Sie für eine andere App den HTML-Code ihrer Signatur und verwenden Sie „HTML einfügen“.
signature-import-from = Aus { $app }
signature-import-already = bereits in Katna
signature-import-gmail-sign-in = { $address }: Melden Sie sich unter Einstellungen > Konten erneut an, damit Katna die Gmail-Signaturen lesen darf.
signature-import-gmail-failed = { $address }: { $error }
signature-import-cancel = Abbrechen
signature-import-do = { $count ->
    [one] { $count } Signatur importieren
   *[other] { $count } Signaturen importieren
}
signature-import-name = { $name } ({ $app })
