# Katna Mail, Swahili (Kiswahili): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = Anwani
contacts-frequent = Zinazotumika mara kwa mara
contacts-other = Anwani nyingine
contacts-other-about = Watu ambao umewatumia barua pepe kutoka Gmail lakini hujawahifadhi
contacts-other-email = Tuma barua pepe
contacts-other-empty = Hakuna anwani nyingine. Watu unaowatumia barua pepe kutoka Gmail lakini huwahifadhi huonekana hapa.
contacts-other-allow = Ili kuona anwani nyingine, ingia tena kwenye akaunti yako ya Gmail na uruhusu Katna kuziona.
contacts-labels = Lebo
contacts-label-options = Chaguo za lebo
contacts-label-rename = Badilisha jina la lebo
contacts-label-email = Tuma barua pepe kwa wote
contacts-label-delete = Futa lebo
contacts-label-new = Lebo mpya
contacts-label-name = Jina la lebo
contacts-label-button = Weka lebo
contacts-label-menu = Weka kwenye lebo:
contacts-label-added = Imeongezwa kwenye { $name }
contacts-label-removed = Imeondolewa kutoka { $name }
contacts-label-renamed = Lebo imepewa jina jipya: { $name }
contacts-label-deleted = Lebo { $name } imefutwa
contacts-label-no-email = Hakuna mtu kwenye lebo hii mwenye anwani ya barua pepe
contacts-manage = Rekebisha na udhibiti
contacts-merge = Unganisha na urekebishe
contacts-merge-about = { $count ->
    [one] Pendekezo { $count }: anwani zinazoonekana kuwa mtu yuleyule
   *[other] Mapendekezo { $count }: anwani zinazoonekana kuwa mtu yuleyule
}
contacts-merge-none = Hakuna nakala rudufu. Anwani zenye jina au nambari ya simu sawa huonekana hapa.
contacts-merge-count = { $count ->
   *[other] Anwani { $count }
}
contacts-merge-all = Unganisha zote
contacts-merge-button = Unganisha
contacts-merge-dismiss = Ondoa
contacts-merged = { $count ->
    [1] Anwani zimeunganishwa
    [one] Muungano { $count } umekamilika
   *[other] Miungano { $count } imekamilika
}
contacts-import = Leta
contacts-export = Hamisha
contacts-import-file = Leta anwani kutoka faili ya vCard au CSV
contacts-imported = { $count ->
   *[other] Umeleta anwani { $count } kwenye { $place }
}
contacts-imported-some = { $count ->
   *[other] Umeleta anwani { $count } kwenye { $place }; { $skipped } zilizohifadhiwa tayari zimeachwa
}
contacts-import-none = Hakuna anwani zilizopatikana kwenye { $name }
contacts-import-all-saved = Kila mtu kwenye { $name } amehifadhiwa tayari
contacts-import-failed = Imeshindwa kusoma { $name }: { $error }
contacts-exported = { $count ->
   *[other] Umehamisha anwani { $count } hadi { $path }
}
contacts-export-none = Hakuna anwani za kuhamisha
contacts-export-failed = Imeshindwa kuhamisha anwani: { $error }
contacts-print = Chapisha
contacts-print-title = Anwani
contacts-print-none = Hakuna anwani za kuchapisha
contacts-print-typed = { $value } ({ $kind })
contacts-print-birthday = Siku ya kuzaliwa: { $day }
contacts-print-nickname = Jina la utani: { $name }
contacts-create = Unda anwani

## Search and the list

contacts-search = Tafuta anwani
contacts-loading = Inapakia anwani…
contacts-empty = Bado hakuna anwani zilizohifadhiwa. Anwani unazohifadhi kwenye Gmail, Outlook au huduma yako ya barua huonekana hapa.
contacts-empty-no-books = Anwani kutoka kwenye akaunti zako zitaonekana hapa mara zitakaposawazishwa.
contacts-none-found = Hakuna anwani zinazolingana na utafutaji wako.
contacts-starred = { $count ->
    [one] Anwani yenye nyota ({ $count })
   *[other] Anwani zenye nyota ({ $count })
}
contacts-count = Anwani ({ $count })
contacts-col-name = Jina
contacts-col-email = Barua pepe
contacts-col-phone = Nambari ya simu
contacts-col-job = Cheo na kampuni
contacts-col-labels = Lebo

## Asking to allow contacts, for accounts signed in before Katna read them

contacts-allow = Ruhusu Katna isome anwani za { $address }.
contacts-allow-many = { $more ->
    [one] Ruhusu Katna isome anwani za { $address } na akaunti { $more } nyingine.
   *[other] Ruhusu Katna isome anwani za { $address } na akaunti { $more } nyingine.
}
contacts-allow-button = Ruhusu

## A contact's page

contacts-back = Rudi kwenye anwani
contacts-edit = Hariri
contacts-delete = Futa
contacts-qr = Shiriki kama msimbo wa QR
contacts-qr-about = Changanua hii kwa kamera ya simu ili kuhifadhi anwani.
contacts-qr-too-long = Anwani hii ina maelezo mengi mno kutoshea kwenye msimbo wa QR.
contacts-qr-done = Nimemaliza
contacts-deleted = Imefutwa: { $name }
contacts-added = Imeongezwa kwenye anwani: { $name }
contacts-find-mail = Barua
contacts-details = Maelezo ya anwani
contacts-saved-in = Imehifadhiwa katika
contacts-notes = Madokezo
contacts-birthday = Siku ya kuzaliwa
contacts-nickname = Jina la utani
contacts-this-computer = Kompyuta hii
contacts-kind-home = Nyumbani
contacts-kind-work = Kazini
contacts-kind-mobile = Simu ya mkononi
contacts-kind-other = Nyingine
contacts-source-google = Anwani za Google
contacts-source-microsoft = Anwani za Outlook
contacts-source-carddav = CardDAV

## Creating and changing a contact

contacts-edit-new-title = Unda anwani
contacts-edit-title = Hariri anwani
contacts-edit-save = Hifadhi
contacts-edit-saving = Inahifadhi…
contacts-edit-cancel = Ghairi
contacts-saved = Anwani imehifadhiwa
contacts-edit-save-to = Hifadhi kwenye
contacts-edit-changes-go-to = Mabadiliko yanahifadhiwa kwenye { $place }.
contacts-edit-given = Jina la kwanza
contacts-edit-family = Jina la ukoo
contacts-edit-company = Kampuni
contacts-edit-job = Cheo cha kazi
contacts-edit-email = Barua pepe
contacts-edit-phone = Simu
contacts-edit-with-kind = { $field } ({ $kind })
contacts-edit-add-email = Ongeza barua pepe
contacts-edit-add-phone = Ongeza simu
contacts-edit-street = Anwani ya mtaa
contacts-edit-city = Jiji
contacts-edit-postcode = Msimbo wa posta
contacts-edit-country = Nchi
contacts-edit-birthday = Siku ya kuzaliwa (YYYY-MM-DD)
contacts-edit-empty = Ongeza jina, barua pepe au nambari ya simu kwanza.
