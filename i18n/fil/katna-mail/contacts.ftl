# Katna Mail, Filipino (Filipino): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = Mga Contact
contacts-frequent = Madalas
contacts-labels = Mga Label
contacts-create = Gumawa ng contact

## Search and the list

contacts-search = Maghanap ng mga contact
contacts-loading = Nilo-load ang mga contact…
contacts-empty = Wala pang naka-save na contact. Lalabas dito ang mga contact na sine-save mo sa Gmail, Outlook, o sa iyong mail service.
contacts-empty-no-books = Lalabas dito ang mga contact mula sa iyong mga account kapag na-sync na ang mga ito.
contacts-none-found = Walang contact na tumutugma sa iyong paghahanap.
contacts-starred = { $count ->
    [one] Naka-star na contact ({ $count })
   *[other] Mga naka-star na contact ({ $count })
}
contacts-count = Mga Contact ({ $count })
contacts-col-name = Pangalan
contacts-col-email = Email
contacts-col-phone = Numero ng telepono
contacts-col-job = Titulo ng trabaho at kumpanya
contacts-col-labels = Mga Label

## Asking to allow contacts, for accounts signed in before Katna read them

contacts-allow = Payagan ang Katna na basahin ang mga contact ng { $address }.
contacts-allow-many = { $more ->
    [one] Payagan ang Katna na basahin ang mga contact ng { $address } at { $more } pang account.
   *[other] Payagan ang Katna na basahin ang mga contact ng { $address } at { $more } pang account.
}
contacts-allow-button = Payagan

## A contact's page

contacts-back = Bumalik sa mga contact
contacts-edit = I-edit
contacts-delete = I-delete
contacts-deleted = Na-delete: { $name }
contacts-find-mail = Mail
contacts-details = Mga detalye ng contact
contacts-saved-in = Naka-save sa
contacts-notes = Mga Tala
contacts-birthday = Kaarawan
contacts-nickname = Palayaw
contacts-this-computer = Computer na ito
contacts-kind-home = Bahay
contacts-kind-work = Trabaho
contacts-kind-mobile = Mobile
contacts-kind-other = Iba pa
contacts-source-google = Google Contacts
contacts-source-microsoft = Mga contact sa Outlook
contacts-source-carddav = CardDAV

## Creating and changing a contact

contacts-edit-new-title = Gumawa ng contact
contacts-edit-title = I-edit ang contact
contacts-edit-save = I-save
contacts-edit-saving = Sine-save…
contacts-edit-cancel = Kanselahin
contacts-saved = Na-save ang contact
contacts-edit-save-to = I-save sa
contacts-edit-changes-go-to = Sine-save ang mga pagbabago sa { $place }.
contacts-edit-given = Unang pangalan
contacts-edit-family = Apelyido
contacts-edit-company = Kumpanya
contacts-edit-job = Titulo ng trabaho
contacts-edit-email = Email
contacts-edit-phone = Telepono
contacts-edit-with-kind = { $field } ({ $kind })
contacts-edit-add-email = Magdagdag ng email
contacts-edit-add-phone = Magdagdag ng telepono
contacts-edit-street = Address ng kalye
contacts-edit-city = Lungsod
contacts-edit-postcode = Postal code
contacts-edit-country = Bansa
contacts-edit-birthday = Kaarawan (YYYY-MM-DD)
contacts-edit-empty = Magdagdag muna ng pangalan, email, o numero ng telepono.
