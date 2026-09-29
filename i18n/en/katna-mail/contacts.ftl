# SPDX-License-Identifier: GPL-3.0-or-later
# The Contacts page: saved contacts from every account.

## The column at the left

contacts-all = Contacts
contacts-frequent = Frequent
contacts-labels = Labels
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
# $name: the person's name.
contacts-deleted = Deleted { $name }
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
