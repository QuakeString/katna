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
