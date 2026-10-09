# Katna Mail, English: the signature editor in Settings > Compose.
# Guide: i18n/README.md. Keep ids stable; change the text freely.
# In plural forms, write { $count } rather than the digit, so languages
# with their own digits show them.

## The signature being edited

# Shown in an empty signature.
signature-placeholder = Your name, and anything to add below it

## Its formatting bar
# Font, size, text color, default color, table and the link's address
# field use the compose window's messages (compose-tools.ftl).

signature-bold = Bold
signature-italic = Italic
signature-underline = Underline
signature-link = Link
# Sets the link typed in the field under the Link button.
signature-link-apply = Apply
signature-picture = Insert picture
signature-align-left = Align left
signature-align-center = Align center
signature-align-right = Align right
signature-numbered-list = Numbered list
signature-bulleted-list = Bulleted list
signature-remove-formatting = Remove formatting

## Adding a picture

# The button of the file chooser.
signature-picture-choose = Insert
# $size: the largest size, such as 512 KB.
signature-picture-too-big = Pictures in a signature can be up to { $size }.
signature-picture-kind = Pick a PNG, JPEG, GIF or WebP picture.
# The file could not be read. $name: the file's name; $error: why, from the system.
signature-picture-unreadable = { $name }: { $error }

## Layouts
# Ready-made signatures: the fields are filled in once, a layout is
# picked, and Katna writes mail-safe HTML with a plain text twin.

signature-layout = Layout
# The choice that keeps a signature written by hand, not from a layout.
signature-layout-own = Your own
signature-layout-classic = Classic
signature-layout-logo-left = Logo left
signature-layout-photo = Photo
signature-layout-band = Colour band
signature-layout-one-line = One line
signature-layout-centred = Centred
signature-layout-banner = With banner
signature-layout-underline = Underline
signature-layout-side-bar = Side bar
signature-layout-card = Card
signature-layout-monogram = Monogram
signature-layout-plain = Plain text
# Labels written before numbers in the signature itself. Keep them short:
# Katna's person card reads "M:" and "O:".
signature-layout-mobile-label = M:
signature-layout-office-label = O:
signature-layout-email-label = E:
# The fields.
signature-layout-name = Name
signature-layout-job = Title
signature-layout-company = Company
signature-layout-mobile = Mobile
signature-layout-office = Office
signature-layout-email = Email
signature-layout-website = Website
signature-layout-address = Address
signature-layout-pictures = Pictures
signature-layout-logo = Logo
signature-layout-photo-picture = Photo
signature-layout-banner-picture = Banner
signature-layout-remove-picture = Remove
signature-layout-pages = Pages
# The field to add a page: a LinkedIn profile, a YouTube channel.
signature-layout-page-placeholder = Add a page's address
signature-layout-colour = Colour
# A picture that could not be read. $name: the file's name.
signature-layout-picture-failed = { $name } couldn't be used as a picture.
# Over the preview.
signature-layout-preview = How the reader sees it
signature-layout-light = Light
signature-layout-dark = Dark
signature-layout-text = Plain text
# $size: such as 23 KB.
signature-layout-inside = Pictures are sent inside the mail, so they show even where remote images are off. This one adds { $size } to each mail.
# The button that turns a signature made from a layout into one edited
# freely, like any text.
signature-layout-edit = Customise
signature-layout-edit-confirm = Customise it freely? Its fields and layout go, and it keeps its look as far as the editor can hold it.
# $layout: the layout's name.
signature-layout-use-confirm = Use the { $layout } layout? It replaces this signature, filled in from it.
signature-layout-use = Use layout
signature-layout-cancel = Cancel

## Paste HTML
# A signature designed elsewhere (a signature website, another mail app),
# pasted as HTML. It is sent as it is, with its pictures inside the mail.

signature-html-title = Paste HTML
signature-html-subtitle = For a signature you designed elsewhere
# Shown in the empty box.
signature-html-placeholder = Paste the signature's HTML here
# The name given to a signature made from pasted HTML.
signature-html-name = Pasted
# Under the box. $name: the name the new signature gets.
signature-html-new = Saved as a new signature, “{ $name }”
# Under the box, when editing a signature's HTML. $name: its name.
signature-html-replaces = Saves over “{ $name }”
signature-html-cancel = Cancel
signature-html-save = Save
signature-html-fetching = Downloading its pictures…
# $size: their total size, such as 9 KB.
signature-html-pictures-inside = { $count ->
    [one] { $count } picture downloaded and put inside the mail ({ $size })
   *[other] { $count } pictures downloaded and put inside the mail ({ $size })
}
signature-html-pictures-web = { $count ->
    [one] { $count } picture couldn't be downloaded, so readers load it from the web
   *[other] { $count } pictures couldn't be downloaded, so readers load them from the web
}
signature-html-removed = Removed scripts, forms and tracking pixels, which mail apps block anyway
signature-html-style-sheet = Left out a style sheet: mail keeps only the styles written on each part
signature-html-links = Removed links that went somewhere other than a website, an address or a phone
signature-html-plain-text = Plain text version made from it, for mail apps that show only text

## Import
# Signatures brought in from Gmail, and from Thunderbird, Evolution and
# KMail on this computer. App names stay as their makers write them.

signature-import-title = Import
signature-import-subtitle = From Gmail, Thunderbird, Evolution and KMail
signature-import-looking = Looking for signatures…
signature-import-none = No signatures found. For another app, copy its signature's HTML and use Paste HTML.
# Under a signature found. $app: Gmail, Thunderbird, Evolution or KMail.
signature-import-from = From { $app }
# Under a signature found that Katna has already.
signature-import-already = already in Katna
# A Gmail account whose sign-in doesn't let Katna read its signatures. $address: the account.
signature-import-gmail-sign-in = { $address }: sign in again in Settings > Accounts so Katna may read Gmail's signatures.
# Reading a Gmail account's signatures failed. $address: the account; $error: why, from Gmail or the system.
signature-import-gmail-failed = { $address }: { $error }
signature-import-cancel = Cancel
signature-import-do = { $count ->
    [one] Import { $count } signature
   *[other] Import { $count } signatures
}
# The name an imported signature gets. $name: its name there, or the address it signs; $app: the app.
signature-import-name = { $name } ({ $app })
