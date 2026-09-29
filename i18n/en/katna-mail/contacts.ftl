# SPDX-License-Identifier: GPL-3.0-or-later
# The Contacts page: saved contacts from every account.

## The column at the left

contacts-all = Contacts
contacts-frequent = Frequent
# Google's "Other contacts": people you mailed but never saved.
contacts-other = Other contacts
contacts-other-about = People you've mailed from Gmail but haven't saved
contacts-other-email = Send email
contacts-other-empty = No other contacts. People you mail from Gmail but don't save show up here.
contacts-other-allow = To see other contacts, sign in to your Gmail account again and allow Katna to see them.
contacts-labels = Labels
# The ⋮ beside a label in the column, and on a label's own page.
contacts-label-options = Label options
contacts-label-rename = Rename label
contacts-label-email = Email everyone
contacts-label-delete = Delete label
contacts-label-new = New label
contacts-label-name = Label name
# After a person's labels on their page: opens the menu that ticks labels.
contacts-label-button = Label
contacts-label-menu = Label as:
contacts-label-added = Added to { $name }
contacts-label-removed = Removed from { $name }
contacts-label-renamed = Label renamed to { $name }
contacts-label-deleted = Deleted label { $name }
contacts-label-no-email = Nobody on this label has an email address
# Under the labels: tools for the whole address book, as in Google Contacts.
# The column's list of mail accounts, each with the people saved in it.
contacts-accounts = Accounts
# The line under an account whose contacts did not come: why, and the one
# click that fixes it.
contacts-account-sign-in = Sign in again to show contacts
contacts-account-signed-in = Signed in to { $address } again. Getting your contacts…
contacts-account-sign-in-refused = { $provider } did not let Katna in. Try again, and allow access to your contacts.
contacts-account-password = The server did not accept the password. Yahoo, iCloud, Zoho and others need an app password.
contacts-account-change-password = Change password
contacts-account-change-password-tooltip = Open Settings > Accounts
contacts-account-failed = The contacts could not be read.
# $reason is the server's own words, in English.
contacts-account-error = The contacts could not be read: { $reason }
contacts-account-none = No address book found
contacts-account-looking = Looking for contacts…
contacts-account-try-again = Try again
contacts-account-try-again-tooltip = Check this account's contacts again now
contacts-account-fixing = Working on it…
contacts-manage = Fix and manage
# Suggested duplicates, as in Google Contacts.
contacts-merge = Merge and fix
contacts-merge-about = { $count ->
    [one] 1 suggestion: contacts that look like the same person
   *[other] { $count } suggestions: contacts that look like the same person
}
contacts-merge-none = No duplicates. Contacts with the same name or phone number show up here.
contacts-merge-count = { $count } contacts
contacts-merge-all = Merge all
contacts-merge-button = Merge
contacts-merge-dismiss = Dismiss
contacts-merged = { $count ->
    [one] Contacts merged
   *[other] { $count } merges done
}
contacts-import = Import
contacts-export = Export
contacts-import-file = Import contacts from a vCard or CSV file
contacts-imported = { $count ->
    [one] Imported 1 contact to { $place }
   *[other] Imported { $count } contacts to { $place }
}
contacts-imported-some = { $count ->
    [one] Imported 1 contact to { $place }; { $skipped } already saved left out
   *[other] Imported { $count } contacts to { $place }; { $skipped } already saved left out
}
contacts-import-none = No contacts found in { $name }
contacts-import-all-saved = Everyone in { $name } is already saved
contacts-import-failed = Couldn't read { $name }: { $error }
contacts-exported = { $count ->
    [one] Exported 1 contact to { $path }
   *[other] Exported { $count } contacts to { $path }
}
contacts-export-none = No contacts to export
contacts-export-failed = Couldn't export contacts: { $error }
contacts-print = Print
# The title of the printed list when no label is on show.
contacts-print-title = Contacts
contacts-print-none = No contacts to print
# A printed detail and what it is: "+91 98765 43210 (Mobile)".
contacts-print-typed = { $value } ({ $kind })
contacts-print-birthday = Birthday: { $day }
contacts-print-nickname = Nickname: { $name }
contacts-create = Create contact

## Search and the list

contacts-search = Search contacts
contacts-loading = Loading contacts…
contacts-empty = No saved contacts yet. Contacts you save in Gmail, Outlook or your mail service show up here.
contacts-empty-no-books = Contacts from your accounts show up here once they are synced.
contacts-none-found = No contacts match your search.
# The heading over the starred contacts.
contacts-starred = { $count ->
    [one] Starred contact ({ $count })
   *[other] Starred contacts ({ $count })
}
# The heading over all contacts.
contacts-count = Contacts ({ $count })
contacts-col-name = Name
contacts-col-email = Email
contacts-col-phone = Phone number
contacts-col-job = Job title & company
contacts-col-labels = Labels

## Asking to allow contacts, for accounts signed in before Katna read them

# $address: the account's address.
contacts-allow = Allow Katna to read the contacts of { $address }.
# $more: how many other accounts also need it.
contacts-allow-many = { $more ->
    [one] Allow Katna to read the contacts of { $address } and { $more } more account.
   *[other] Allow Katna to read the contacts of { $address } and { $more } more accounts.
}
contacts-allow-button = Allow

## A contact's page

contacts-back = Back to contacts
contacts-edit = Edit
contacts-delete = Delete
contacts-qr = Share as QR code
contacts-qr-about = Scan this with a phone's camera to save the contact.
contacts-qr-too-long = This contact has too many details to fit in a QR code.
contacts-qr-done = Done
# $name: the person's name.
contacts-deleted = Deleted { $name }
contacts-added = Added { $name } to contacts
contacts-find-mail = Mail
contacts-details = Contact details
contacts-saved-in = Saved in
contacts-notes = Notes
contacts-birthday = Birthday
contacts-nickname = Nickname
contacts-this-computer = This computer
contacts-kind-home = Home
contacts-kind-work = Work
contacts-kind-mobile = Mobile
contacts-kind-other = Other
contacts-source-google = Google Contacts
contacts-source-microsoft = Outlook contacts
contacts-source-carddav = CardDAV

## Creating and changing a contact

contacts-edit-new-title = Create contact
contacts-edit-title = Edit contact
contacts-edit-save = Save
contacts-edit-saving = Saving…
contacts-edit-cancel = Cancel
contacts-saved = Contact saved
contacts-edit-save-to = Save to
# $place: the account address, or "This computer".
contacts-edit-changes-go-to = Changes are saved to { $place }.
contacts-edit-given = First name
contacts-edit-family = Last name
contacts-edit-company = Company
contacts-edit-job = Job title
contacts-edit-email = Email
contacts-edit-phone = Phone
# A field with its kind, e.g. "Email (Work)".
contacts-edit-with-kind = { $field } ({ $kind })
contacts-edit-add-email = Add email
contacts-edit-add-phone = Add phone
contacts-edit-street = Street address
contacts-edit-city = City
contacts-edit-postcode = Postal code
contacts-edit-country = Country
# How the date is typed: YYYY-MM-DD, or MM-DD without a year. Keep the letters as they are.
contacts-edit-birthday = Birthday (YYYY-MM-DD)
contacts-edit-empty = Add a name, an email or a phone number first.
