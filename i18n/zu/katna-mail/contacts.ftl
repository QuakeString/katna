# Katna Mail, Zulu (isiZulu): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = Oxhumana nabo
contacts-frequent = Ababuthakathaka
contacts-other = Abanye oxhumana nabo
contacts-other-about = Abantu obabhalele nge-Gmail kodwa ongabalondolozanga
contacts-other-email = Thumela i-imeyili
contacts-other-empty = Abekho abanye oxhumana nabo. Abantu obabhalela nge-Gmail kodwa ongabalondolozi bavela lapha.
contacts-other-allow = Ukuze ubone abanye oxhumana nabo, ngena futhi ku-akhawunti yakho ye-Gmail bese uvumela i-Katna ukuthi ibabone.
contacts-labels = Amalebula
contacts-label-options = Izinketho zelebula
contacts-label-rename = Qamba kabusha ilebula
contacts-label-email = Thumela imeyili kubo bonke
contacts-label-delete = Susa ilebula
contacts-label-new = Ilebula elisha
contacts-label-name = Igama lelebula
contacts-label-button = Ilebula
contacts-label-menu = Faka ilebula njengo:
contacts-label-added = Kwengezwe ku-{ $name }
contacts-label-removed = Kususwe ku-{ $name }
contacts-label-renamed = Ilebula lifakwe igama elisha elithi { $name }
contacts-label-deleted = Kususiwe ilebula { $name }
contacts-label-no-email = Akekho kule lebula onekheli le-imeyili
# The column's list of mail accounts, each with the people saved in it.
contacts-accounts = Ama-akhawunti
# The line under an account whose contacts did not come: why, and the one
# click that fixes it.
contacts-account-sign-in = Ngena futhi ukuze ubonise oxhumana nabo
contacts-account-signed-in = Ungene futhi ku-{ $address }. Kutholwa oxhumana nabo…
contacts-account-sign-in-refused = I-{ $provider } ayizange ivumele i-Katna ingene. Zama futhi, bese uvumela ukufinyelela koxhumana nabo.
contacts-account-password = Iseva ayizange yamukele iphasiwedi. I-Yahoo, i-iCloud, i-Zoho nabanye badinga iphasiwedi yohlelo lokusebenza.
contacts-account-change-password = Shintsha iphasiwedi
contacts-account-change-password-tooltip = Vula Izilungiselelo > Ama-akhawunti
contacts-account-failed = Oxhumana nabo abakwazanga ukufundwa.
# $reason is the server's own words, in English.
contacts-account-error = Oxhumana nabo abakwazanga ukufundwa: { $reason }
contacts-account-none = Ayikho incwadi yamakheli etholakele
# $reason is what the server answered, in English: "CardDAV https://dav.example.in/: status 404".
contacts-account-none-why = Ayikho incwadi yamakheli etholakele: { $reason }
# A Gmail or Outlook account added with a password: its contacts need the
# provider's sign-in.
contacts-account-use-sign-in = I-{ $provider } ibonisa oxhumana nabo kuphela ku-Katna engene nge-{ $provider }.
contacts-account-sign-in-with = Ngena nge-{ $provider }
contacts-account-looking = Kufunwa oxhumana nabo…
contacts-account-try-again = Zama futhi
contacts-account-try-again-tooltip = Hlola oxhumana nabo bale akhawunti futhi manje
contacts-account-fixing = Kuyasebenzwa kukho…
contacts-manage = Lungisa futhi uphathe
contacts-merge = Hlanganisa futhi ulungise
contacts-merge-about = { $count ->
    [one] Iziphakamiso ezingu-{ $count }: oxhumana nabo ababukeka njengomuntu oyedwa
   *[other] Iziphakamiso ezingu-{ $count }: oxhumana nabo ababukeka njengomuntu oyedwa
}
contacts-merge-none = Awekho amaduplicate. Oxhumana nabo abanegama noma inombolo yefoni efanayo bavela lapha.
contacts-merge-count = { $count ->
    [one] Oxhumana nabo abangu-{ $count }
   *[other] Oxhumana nabo abangu-{ $count }
}
contacts-merge-all = Hlanganisa bonke
contacts-merge-button = Hlanganisa
contacts-merge-dismiss = Cashisa
contacts-merged = { $count ->
    [1] Oxhumana nabo bahlanganisiwe
    [one] Ukuhlanganisa okungu-{ $count } kuqediwe
   *[other] Ukuhlanganisa okungu-{ $count } kuqediwe
}
contacts-import = Ngenisa
contacts-export = Thumela
contacts-import-file = Ngenisa oxhumana nabo kufayela le-vCard noma le-CSV
contacts-imported = { $count ->
    [one] Kungeniswe oxhumana nabo abangu-{ $count } ku-{ $place }
   *[other] Kungeniswe oxhumana nabo abangu-{ $count } ku-{ $place }
}
contacts-imported-some = { $count ->
    [one] Kungeniswe oxhumana nabo abangu-{ $count } ku-{ $place }; abangu-{ $skipped } abagcinwe kakade bashiywe
   *[other] Kungeniswe oxhumana nabo abangu-{ $count } ku-{ $place }; abangu-{ $skipped } abagcinwe kakade bashiywe
}
contacts-import-none = Akukho oxhumana nabo otholakele ku-{ $name }
contacts-import-all-saved = Wonke umuntu ku-{ $name } usegciniwe kakade
contacts-import-failed = Ayikwazanga ukufunda i-{ $name }: { $error }
contacts-exported = { $count ->
    [one] Kuthunyelwe oxhumana nabo abangu-{ $count } ku-{ $path }
   *[other] Kuthunyelwe oxhumana nabo abangu-{ $count } ku-{ $path }
}
contacts-export-none = Akukho oxhumana nabo okufanele bathunyelwe
contacts-export-failed = Ayikwazanga ukuthumela oxhumana nabo: { $error }
contacts-print = Phrinta
contacts-print-title = Oxhumana nabo
contacts-print-none = Akukho oxhumana nabo abazophrintwa
contacts-print-typed = { $value } ({ $kind })
contacts-print-birthday = Usuku lokuzalwa: { $day }
contacts-print-nickname = Isidlaliso: { $name }
contacts-create = Dala oxhumana naye

## Search and the list

contacts-search = Sesha oxhumana nabo
contacts-loading = Iyalayisha oxhumana nabo…
contacts-empty = Awukabi nabo oxhumana nabo abagcinwe. Oxhumana nabo obagcina ku-Gmail, ku-Outlook noma kusevisi yakho yemeyili bavela lapha.
contacts-empty-no-books = Oxhumana nabo bama-akhawunti akho bazovela lapha uma sebevumelanisiwe.
contacts-none-found = Alukho oxhumana naye ofana nosesho lwakho.
contacts-starred = { $count ->
    [one] Oxhumana naye onenkanyezi ({ $count })
   *[other] Oxhumana nabo abanenkanyezi ({ $count })
}
contacts-count = Oxhumana nabo ({ $count })
contacts-col-name = Igama
contacts-col-email = I-imeyili
contacts-col-phone = Inombolo yefoni
contacts-col-job = Isihloko somsebenzi nenkampani
contacts-col-labels = Amalebula

## Asking to allow contacts, for accounts signed in before Katna read them

contacts-allow = Vumela i-Katna ifunde oxhumana nabo be-{ $address }.
contacts-allow-many = { $more ->
    [one] Vumela i-Katna ifunde oxhumana nabo be-{ $address } nakwenye i-akhawunti engu-{ $more }.
   *[other] Vumela i-Katna ifunde oxhumana nabo be-{ $address } nakwezinye ama-akhawunti angu-{ $more }.
}
contacts-allow-button = Vumela

## A contact's page

contacts-back = Buyela koxhumana nabo
contacts-edit = Hlela
contacts-delete = Susa
contacts-qr = Yabelana njengekhodi ye-QR
contacts-qr-about = Skena lokhu ngekhamera yefoni ukuze ugcine oxhumana naye.
contacts-qr-too-long = Lo oxhumana naye unemininingwane eminingi kakhulu engangena ekhodini ye-QR.
contacts-qr-done = Kwenziwe
contacts-deleted = Kususiwe { $name }
contacts-added = Kwengezwe { $name } koxhumana nabo
contacts-find-mail = Imeyili
contacts-details = Imininingwane yoxhumana naye
contacts-saved-in = Kugcinwe ku-
contacts-notes = Amanothi
contacts-birthday = Usuku lokuzalwa
contacts-nickname = Isidlaliso
contacts-this-computer = Le khompyutha
contacts-kind-home = Ekhaya
contacts-kind-work = Emsebenzini
contacts-kind-mobile = Ifoni
contacts-kind-other = Okunye
contacts-source-google = Oxhumana nabo be-Google
contacts-source-microsoft = Oxhumana nabo be-Outlook
contacts-source-carddav = CardDAV

## Creating and changing a contact

contacts-edit-new-title = Dala oxhumana naye
contacts-edit-title = Hlela oxhumana naye
contacts-edit-save = Londoloza
contacts-edit-saving = Iyalondoloza…
contacts-edit-cancel = Khansela
contacts-saved = Oxhumana naye ulondolozwe
contacts-edit-save-to = Londoloza ku-
contacts-edit-changes-go-to = Izinguquko zilondolozwa ku-{ $place }.
contacts-edit-given = Igama lokuqala
contacts-edit-family = Isibongo
contacts-edit-company = Inkampani
contacts-edit-job = Isihloko somsebenzi
contacts-edit-email = I-imeyili
contacts-edit-phone = Ifoni
contacts-edit-with-kind = { $field } ({ $kind })
contacts-edit-add-email = Engeza i-imeyili
contacts-edit-add-phone = Engeza ifoni
contacts-edit-street = Ikheli lomgwaqo
contacts-edit-city = Idolobha
contacts-edit-postcode = Ikhodi yeposi
contacts-edit-country = Izwe
contacts-edit-birthday = Usuku lokuzalwa (YYYY-MM-DD)
contacts-edit-empty = Qala ngokwengeza igama, i-imeyili noma inombolo yefoni.
