# Katna Mail, Filipino (Filipino): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = Mga Contact
contacts-frequent = Madalas
contacts-other = Iba pang contact
contacts-other-about = Mga taong na-email mo mula sa Gmail pero hindi mo pa nase-save
contacts-other-email = Magpadala ng email
contacts-other-empty = Walang iba pang contact. Lalabas dito ang mga taong ine-email mo mula sa Gmail pero hindi mo sine-save.
contacts-other-allow = Para makita ang iba pang contact, mag-sign in ulit sa iyong Gmail account at payagan ang Katna na makita ang mga ito.
contacts-labels = Mga Label
contacts-label-options = Mga opsyon ng label
contacts-label-rename = Palitan ang pangalan ng label
contacts-label-email = I-email ang lahat
contacts-label-delete = I-delete ang label
contacts-label-new = Bagong label
contacts-label-name = Pangalan ng label
contacts-label-button = Label
contacts-label-menu = I-label bilang:
contacts-label-added = Naidagdag sa { $name }
contacts-label-removed = Naalis sa { $name }
contacts-label-renamed = Pinalitan ang pangalan ng label sa { $name }
contacts-label-deleted = Na-delete ang label na { $name }
contacts-label-no-email = Walang may email address sa label na ito
contacts-accounts = Mga Account
contacts-account-sign-in = Mag-sign in muli para ipakita ang mga contact
contacts-account-signed-in = Naka-sign in muli sa { $address }. Kinukuha ang iyong mga contact…
contacts-account-sign-in-refused = Hindi pinapasok ng { $provider } ang Katna. Subukang muli, at payagan ang access sa iyong mga contact.
contacts-account-password = Hindi tinanggap ng server ang password. Kailangan ng Yahoo, iCloud, Zoho at iba pa ng app password.
contacts-account-change-password = Palitan ang password
contacts-account-change-password-tooltip = I-type ang bagong password; susuriin ito ng Katna sa server
contacts-account-failed = Hindi mabasa ang mga contact.
contacts-account-error = Hindi mabasa ang mga contact: { $reason }
contacts-account-none = Walang nakitang address book
contacts-account-none-why = Walang nakitang address book: { $reason }
contacts-account-use-sign-in = Ipinapakita lang ng { $provider } ang mga contact sa Katna kapag naka-sign in gamit ang { $provider }.
contacts-account-sign-in-with = Mag-sign in gamit ang { $provider }
contacts-account-looking = Naghahanap ng mga contact…
contacts-account-try-again = Subukang muli
contacts-account-try-again-tooltip = Suriin muli ngayon ang mga contact ng account na ito
contacts-account-fixing = Inaayos na…
contacts-manage = Ayusin at pamahalaan
contacts-merge = Pagsamahin at ayusin
contacts-merge-about = { $count ->
    [one] { $count } suhestiyon: mga contact na mukhang iisang tao
   *[other] { $count } suhestiyon: mga contact na mukhang iisang tao
}
contacts-merge-none = Walang duplicate. Dito lalabas ang mga contact na may parehong pangalan o numero ng telepono.
contacts-merge-count = { $count ->
    [one] { $count } contact
   *[other] { $count } contact
}
contacts-merge-all = Pagsamahin lahat
contacts-merge-button = Pagsamahin
contacts-merge-dismiss = Balewalain
contacts-merged = { $count ->
    [1] Napagsama ang mga contact
    [one] { $count } pagsasama ang natapos
   *[other] { $count } pagsasama ang natapos
}
contacts-import = I-import
contacts-export = I-export
contacts-import-file = Mag-import ng mga contact mula sa vCard o CSV file
contacts-imported = { $count ->
    [one] { $count } contact ang na-import sa { $place }
   *[other] { $count } contact ang na-import sa { $place }
}
contacts-imported-some = { $count ->
    [one] { $count } contact ang na-import sa { $place }; { $skipped } na naka-save na, hindi isinama
   *[other] { $count } contact ang na-import sa { $place }; { $skipped } na naka-save na, hindi isinama
}
contacts-import-none = Walang nakitang contact sa { $name }
contacts-import-all-saved = Naka-save na ang lahat sa { $name }
contacts-import-failed = Hindi mabasa ang { $name }: { $error }
contacts-exported = { $count ->
    [one] { $count } contact ang na-export sa { $path }
   *[other] { $count } contact ang na-export sa { $path }
}
contacts-export-none = Walang mga contact na ie-export
contacts-export-failed = Hindi ma-export ang mga contact: { $error }
contacts-print = I-print
contacts-print-title = Mga Contact
contacts-print-none = Walang contact na ipi-print
contacts-print-typed = { $value } ({ $kind })
contacts-print-birthday = Kaarawan: { $day }
contacts-print-nickname = Palayaw: { $name }
contacts-create = Bagong contact

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
contacts-qr = I-share bilang QR code
contacts-qr-about = I-scan ito gamit ang camera ng telepono para ma-save ang contact.
contacts-qr-too-long = Masyadong marami ang detalye ng contact na ito para magkasya sa QR code.
contacts-qr-done = Tapos na
contacts-deleted = Na-delete: { $name }
contacts-added = Naidagdag ang { $name } sa mga contact
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
