# Katna Mail, Hausa (Hausa): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = Lambobin sadarwa
contacts-frequent = Masu yawa
contacts-other = Sauran lambobin sadarwa
contacts-other-about = Mutanen da ka aika wa imel daga Gmail amma ba ka adana su ba
contacts-other-email = Aika imel
contacts-other-empty = Babu sauran lambobin sadarwa. Mutanen da ka aika wa imel daga Gmail amma ba ka adana su ba suna bayyana a nan.
contacts-other-allow = Don ganin sauran lambobin sadarwa, sake shiga asusun Gmail ɗinka kuma ka ƙyale Katna ta gan su.
contacts-labels = Lakabobi
contacts-label-options = Zaɓuɓɓukan lakabi
contacts-label-rename = Sake sunan lakabi
contacts-label-email = Aika imel ga kowa
contacts-label-delete = Share lakabi
contacts-label-new = Sabon lakabi
contacts-label-name = Sunan lakabi
contacts-label-button = Lakabi
contacts-label-menu = Sanya lakabi:
contacts-label-added = An ƙara zuwa { $name }
contacts-label-removed = An cire daga { $name }
contacts-label-renamed = An sake wa lakabin suna: { $name }
contacts-label-deleted = An share lakabin { $name }
contacts-label-no-email = Babu wanda ke da adireshin imel a wannan lakabin
# The column's list of mail accounts, each with the people saved in it.
contacts-accounts = Asusu
# The line under an account whose contacts did not come: why, and the one
# click that fixes it.
contacts-account-sign-in = Sake shiga don nuna lambobin sadarwa
contacts-account-signed-in = An sake shiga { $address }. Ana samo lambobin sadarwarku…
contacts-account-sign-in-refused = { $provider } bai bar Katna ya shiga ba. Ku sake gwadawa, kuma ku ba da izinin shiga lambobin sadarwarku.
contacts-account-password = Sabar ba ta karɓi kalmar sirrin ba. Yahoo, iCloud, Zoho da wasu suna buƙatar kalmar sirrin manhaja.
contacts-account-change-password = Canza kalmar sirri
contacts-account-change-password-tooltip = Buɗe Saituna > Asusu
contacts-account-failed = Ba a iya karanta lambobin sadarwar ba.
# $reason is the server's own words, in English.
contacts-account-error = Ba a iya karanta lambobin sadarwar ba: { $reason }
contacts-account-none = Ba a sami littafin adireshi ba
# $reason is what the server answered, in English: "CardDAV https://dav.example.in/: status 404".
contacts-account-none-why = Ba a sami littafin adireshi ba: { $reason }
# A Gmail or Outlook account added with a password: its contacts need the
# provider's sign-in.
contacts-account-use-sign-in = { $provider } yana nuna lambobin sadarwa ga Katna ne kawai idan ya shiga da { $provider }.
contacts-account-sign-in-with = Shiga da { $provider }
contacts-account-looking = Ana neman lambobin sadarwa…
contacts-account-try-again = Sake gwadawa
contacts-account-try-again-tooltip = Sake duba lambobin sadarwar wannan asusun yanzu
contacts-account-fixing = Ana aiki a kai…
contacts-manage = Gyara da sarrafa
contacts-merge = Haɗa da gyara
contacts-merge-about = { $count ->
    [one] Shawara { $count }: lambobin sadarwa da suke kama da mutum ɗaya
   *[other] Shawarwari { $count }: lambobin sadarwa da suke kama da mutum ɗaya
}
contacts-merge-none = Babu kwafi. Lambobin sadarwa masu suna ko lambar waya iri ɗaya suna bayyana a nan.
contacts-merge-count = { $count ->
    [one] Lambar sadarwa { $count }
   *[other] Lambobin sadarwa { $count }
}
contacts-merge-all = Haɗa duka
contacts-merge-button = Haɗa
contacts-merge-dismiss = Yi watsi
contacts-merged = { $count ->
    [1] An haɗa lambobin sadarwa
    [one] An yi haɗewa { $count }
   *[other] An yi haɗe-haɗe { $count }
}
contacts-import = Shigo da
contacts-export = Fitar da
contacts-import-file = Shigo da lambobin sadarwa daga fayil na vCard ko CSV
contacts-imported = { $count ->
    [one] An shigo da lambar sadarwa { $count } cikin { $place }
   *[other] An shigo da lambobin sadarwa { $count } cikin { $place }
}
contacts-imported-some = { $count ->
    [one] An shigo da lambar sadarwa { $count } cikin { $place }; an bar { $skipped } da aka riga aka ajiye
   *[other] An shigo da lambobin sadarwa { $count } cikin { $place }; an bar { $skipped } da aka riga aka ajiye
}
contacts-import-none = Ba a sami lambar sadarwa a cikin { $name } ba
contacts-import-all-saved = Kowa a cikin { $name } an riga an ajiye shi
contacts-import-failed = Ba a iya karanta { $name } ba: { $error }
contacts-exported = { $count ->
    [one] An fitar da lambar sadarwa { $count } zuwa { $path }
   *[other] An fitar da lambobin sadarwa { $count } zuwa { $path }
}
contacts-export-none = Babu lambobin sadarwa da za a fitar
contacts-export-failed = Ba a iya fitar da lambobin sadarwa ba: { $error }
contacts-print = Buga
contacts-print-title = Lambobin sadarwa
contacts-print-none = Babu lambobin sadarwa da za a buga
contacts-print-typed = { $value } ({ $kind })
contacts-print-birthday = Ranar haihuwa: { $day }
contacts-print-nickname = Sunan barkwanci: { $name }
contacts-create = Sabuwar lambar sadarwa

## Search and the list

contacts-search = Nemo lambobin sadarwa
contacts-loading = Ana loda lambobin sadarwa…
contacts-empty = Babu wasu lambobin sadarwa da aka adana tukuna. Lambobin sadarwar da ka adana a Gmail, Outlook ko sabis ɗin wasiƙarka za su bayyana a nan.
contacts-empty-no-books = Lambobin sadarwa daga asusunka za su bayyana a nan da zarar an daidaita su.
contacts-none-found = Babu lambobin sadarwa da suka dace da bincikenka.
contacts-starred = { $count ->
    [one] Lambar sadarwa mai tauraro ({ $count })
   *[other] Lambobin sadarwa masu tauraro ({ $count })
}
contacts-count = Lambobin sadarwa ({ $count })
contacts-col-name = Suna
contacts-col-email = Imel
contacts-col-phone = Lambar waya
contacts-col-job = Matsayin aiki da kamfani
contacts-col-labels = Lakabobi

## Asking to allow contacts, for accounts signed in before Katna read them

contacts-allow = Ƙyale Katna ta karanta lambobin sadarwa na { $address }.
contacts-allow-many = { $more ->
    [one] Ƙyale Katna ta karanta lambobin sadarwa na { $address } da wani asusu { $more }.
   *[other] Ƙyale Katna ta karanta lambobin sadarwa na { $address } da wasu asusu { $more }.
}
contacts-allow-button = Ƙyale

## A contact's page

contacts-back = Koma zuwa lambobin sadarwa
contacts-edit = Gyara
contacts-delete = Share
contacts-qr = Raba a matsayin lambar QR
contacts-qr-about = Duba wannan da kyamarar waya don ajiye lambar sadarwa.
contacts-qr-too-long = Wannan lambar sadarwa tana da bayanai da yawa da ba za su shiga cikin lambar QR ba.
contacts-qr-done = An gama
contacts-deleted = An share { $name }
contacts-added = An ƙara { $name } cikin lambobin sadarwa
contacts-find-mail = Wasiƙu
contacts-details = Bayanan lamba
contacts-saved-in = An adana a
contacts-notes = Bayanai
contacts-birthday = Ranar haihuwa
contacts-nickname = Suna na wasa
contacts-this-computer = Wannan kwamfutar
contacts-kind-home = Gida
contacts-kind-work = Aiki
contacts-kind-mobile = Wayar hannu
contacts-kind-other = Wani
contacts-source-google = Lambobin sadarwa na Google
contacts-source-microsoft = Lambobin sadarwa na Outlook
contacts-source-carddav = CardDAV

## Creating and changing a contact

contacts-edit-new-title = Ƙirƙiri lambar sadarwa
contacts-edit-title = Gyara lambar sadarwa
contacts-edit-save = Ajiye
contacts-edit-saving = Ana ajiyewa…
contacts-edit-cancel = Soke
contacts-saved = An ajiye lambar sadarwa
contacts-edit-save-to = Ajiye a
contacts-edit-changes-go-to = Ana ajiye canje-canje a { $place }.
contacts-edit-given = Sunan farko
contacts-edit-family = Sunan ƙarshe
contacts-edit-company = Kamfani
contacts-edit-job = Matsayin aiki
contacts-edit-email = Imel
contacts-edit-phone = Waya
contacts-edit-with-kind = { $field } ({ $kind })
contacts-edit-add-email = Ƙara imel
contacts-edit-add-phone = Ƙara waya
contacts-edit-street = Adireshin titi
contacts-edit-city = Birni
contacts-edit-postcode = Lambar gidan waya
contacts-edit-country = Ƙasa
contacts-edit-birthday = Ranar haihuwa (YYYY-MM-DD)
contacts-edit-empty = Da farko ƙara suna, imel ko lambar waya.
