# Katna Mail, Igbo (Igbo): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = Kọntaktị
contacts-frequent = Ndị a na-akpọ ugboro ugboro
contacts-other = Kọntaktị ndị ọzọ
contacts-other-about = Ndị i zigara ozi site na Gmail ma ị chekwabeghị
contacts-other-email = Zipu ozi ịntanetị
contacts-other-empty = Enwere kọntaktị ndị ọzọ. Ndị ị na-ezigara ozi site na Gmail ma ị naghị echekwa ga-apụta ebe a.
contacts-other-allow = Ka i hụ kọntaktị ndị ọzọ, banye n'akaụntụ Gmail gị ọzọ ma kwe ka Katna hụ ha.
contacts-labels = Leebụl
contacts-label-options = Nhọrọ leebụl
contacts-label-rename = Megharịa aha leebụl
contacts-label-email = Ziga ndị niile ozi
contacts-label-delete = Hichapụ leebụl
contacts-label-new = Leebụl ọhụrụ
contacts-label-name = Aha leebụl
contacts-label-button = Leebụl
contacts-label-menu = Tinye leebụl dị ka:
contacts-label-added = Agbakwunyere na { $name }
contacts-label-removed = Wepụrụ na { $name }
contacts-label-renamed = Agbanwela aha leebụl ka ọ bụrụ { $name }
contacts-label-deleted = Ehichapụla leebụl { $name }
contacts-label-no-email = Ọ dịghị onye nọ na leebụl a nwere adreesị ozi
# The column's list of mail accounts, each with the people saved in it.
contacts-accounts = Akaụntụ
# The line under an account whose contacts did not come: why, and the one
# click that fixes it.
contacts-account-sign-in = Banye ọzọ iji gosi kọntaktị
contacts-account-signed-in = Abanyela ọzọ na { $address }. Na-enweta kọntaktị gị…
contacts-account-sign-in-refused = { $provider } ekweghị ka Katna banye. Nwaa ọzọ, ma kwe ka ọ nweta kọntaktị gị.
contacts-account-password = Sava ahụ anabataghị okwuntughe ahụ. Yahoo, iCloud, Zoho na ndị ọzọ chọrọ okwuntughe ngwa.
contacts-account-change-password = Gbanwee okwuntughe
contacts-account-change-password-tooltip = Mepee Ntọala > Akaụntụ
contacts-account-failed = Enweghị ike ịgụ kọntaktị.
# $reason is the server's own words, in English.
contacts-account-error = Enweghị ike ịgụ kọntaktị: { $reason }
contacts-account-none = Ahụghị akwụkwọ adreesị ọ bụla
contacts-account-looking = Na-achọ kọntaktị…
contacts-account-try-again = Nwaa ọzọ
contacts-account-try-again-tooltip = Lelee kọntaktị akaụntụ a ọzọ ugbu a
contacts-account-fixing = Na-arụ ọrụ na ya…
contacts-manage = Dozie ma jikwaa
contacts-merge = Jikọta ma dozie
contacts-merge-about = { $count ->
   *[other] Aro { $count }: kọntaktị ndị yiri otu onye
}
contacts-merge-none = Enweghị ihe myirịta. Kọntaktị ndị nwere otu aha ma ọ bụ otu ọnụọgụ ekwentị ga-apụta ebe a.
contacts-merge-count = { $count ->
   *[other] Kọntaktị { $count }
}
contacts-merge-all = Jikọta ha niile
contacts-merge-button = Jikọta
contacts-merge-dismiss = Chefuo
contacts-merged = { $count ->
    [1] Ejikọtara kọntaktị
   *[other] Ejikọtala { $count }
}
contacts-import = Bubata
contacts-export = Bupụ
contacts-import-file = Bubata kọntaktị site na faịlụ vCard ma ọ bụ CSV
contacts-imported = { $count ->
   *[other] E bubatara kọntaktị { $count } na { $place }
}
contacts-imported-some = { $count ->
   *[other] E bubatara kọntaktị { $count } na { $place }; ahapụrụ { $skipped } echekwarala
}
contacts-import-none = Ahụghị kọntaktị ọ bụla na { $name }
contacts-import-all-saved = Onye ọ bụla nọ na { $name } echekwarala
contacts-import-failed = Enweghị ike ịgụ { $name }: { $error }
contacts-exported = { $count ->
   *[other] E bupụrụ kọntaktị { $count } na { $path }
}
contacts-export-none = Enweghị kọntaktị ị ga-ebupụ
contacts-export-failed = Enweghị ike ibupụ kọntaktị: { $error }
contacts-print = Bipụta
contacts-print-title = Kọntaktị
contacts-print-none = Enweghị kọntaktị ị ga-ebipụta
contacts-print-typed = { $value } ({ $kind })
contacts-print-birthday = Ụbọchị ọmụmụ: { $day }
contacts-print-nickname = Aha ọkpụkpọ: { $name }
contacts-create = Mepụta kọntaktị

## Search and the list

contacts-search = Chọọ kọntaktị
contacts-loading = Kọntaktị na-ebugo…
contacts-empty = Enwebeghị kọntaktị echekwara. Kọntaktị ị chekwara na Gmail, Outlook ma ọ bụ ọrụ ozi gị ga-apụta ebe a.
contacts-empty-no-books = Kọntaktị si n'akaụntụ gị ga-apụta ebe a ozugbo e sinkrọnaịzị ha.
contacts-none-found = Enweghị kọntaktị dabara na ọchụchọ gị.
contacts-starred = { $count ->
   *[other] Kọntaktị nwere kpakpando ({ $count })
}
contacts-count = Kọntaktị ({ $count })
contacts-col-name = Aha
contacts-col-email = Imeel
contacts-col-phone = Nọmba ekwentị
contacts-col-job = Aha ọrụ na ụlọ ọrụ
contacts-col-labels = Leebụl

## Asking to allow contacts, for accounts signed in before Katna read them

contacts-allow = Kwe ka Katna gụọ kọntaktị nke { $address }.
contacts-allow-many = { $more ->
   *[other] Kwe ka Katna gụọ kọntaktị nke { $address } na akaụntụ { $more } ọzọ.
}
contacts-allow-button = Kwe

## A contact's page

contacts-back = Laghachi na kọntaktị
contacts-edit = Dezie
contacts-delete = Hichapụ
contacts-qr = Kekọrịta dị ka koodu QR
contacts-qr-about = Were igwefoto foonu nyochaa nke a ka ịchekwa kọntaktị ahụ.
contacts-qr-too-long = Kọntaktị a nwere nkọwa dị ukwuu nke koodu QR nweghị ike ijide.
contacts-qr-done = Emechala
contacts-deleted = Ehichapụla { $name }
contacts-added = Agbakwunyere { $name } na kọntaktị
contacts-find-mail = Ozi
contacts-details = Nkọwa kọntaktị
contacts-saved-in = Echekwara na
contacts-notes = Ndetu
contacts-birthday = Ụbọchị ọmụmụ
contacts-nickname = Aha ọkpụkpọ
contacts-this-computer = Kọmputa a
contacts-kind-home = Ụlọ
contacts-kind-work = Ọrụ
contacts-kind-mobile = Ekwentị mkpanaaka
contacts-kind-other = Ndị ọzọ
contacts-source-google = Kọntaktị Google
contacts-source-microsoft = Kọntaktị Outlook
contacts-source-carddav = CardDAV

## Creating and changing a contact

contacts-edit-new-title = Mepụta kọntaktị
contacts-edit-title = Dezie kọntaktị
contacts-edit-save = Chekwaa
contacts-edit-saving = Na-echekwa…
contacts-edit-cancel = Kagbuo
contacts-saved = Echekwara kọntaktị
contacts-edit-save-to = Chekwaa na
contacts-edit-changes-go-to = A na-echekwa mgbanwe na { $place }.
contacts-edit-given = Aha mbụ
contacts-edit-family = Aha ikpeazụ
contacts-edit-company = Ụlọ ọrụ
contacts-edit-job = Aha ọrụ
contacts-edit-email = Imeel
contacts-edit-phone = Ekwentị
contacts-edit-with-kind = { $field } ({ $kind })
contacts-edit-add-email = Tinye imeel
contacts-edit-add-phone = Tinye ekwentị
contacts-edit-street = Adreesị okporo ámá
contacts-edit-city = Obodo
contacts-edit-postcode = Koodu nzipu ozi
contacts-edit-country = Mba
contacts-edit-birthday = Ụbọchị ọmụmụ (YYYY-MM-DD)
contacts-edit-empty = Buru ụzọ tinye aha, imeel ma ọ bụ nọmba ekwentị.
