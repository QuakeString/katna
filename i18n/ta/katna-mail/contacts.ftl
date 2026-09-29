# Katna Mail, Tamil (தமிழ்): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = தொடர்புகள்
contacts-frequent = அடிக்கடி
contacts-other = பிற தொடர்புகள்
contacts-other-about = நீங்கள் Gmail மூலம் அஞ்சல் அனுப்பியும் சேமிக்காதவர்கள்
contacts-other-email = மின்னஞ்சல் அனுப்பு
contacts-other-empty = பிற தொடர்புகள் இல்லை. Gmail மூலம் நீங்கள் அஞ்சல் அனுப்பியும் சேமிக்காதவர்கள் இங்கே தோன்றுவார்கள்.
contacts-other-allow = பிற தொடர்புகளைப் பார்க்க, உங்கள் Gmail கணக்கில் மீண்டும் உள்நுழைந்து, அவற்றைப் பார்க்க Katna-வை அனுமதிக்கவும்.
contacts-labels = லேபிள்கள்
contacts-label-options = லேபிள் விருப்பங்கள்
contacts-label-rename = லேபிள் பெயரை மாற்று
contacts-label-email = அனைவருக்கும் மெயில் அனுப்பு
contacts-label-delete = லேபிளை நீக்கு
contacts-label-new = புதிய லேபிள்
contacts-label-name = லேபிள் பெயர்
contacts-label-button = லேபிள்
contacts-label-menu = இவ்வாறு லேபிளிடு:
contacts-label-added = { $name } இல் சேர்க்கப்பட்டது
contacts-label-removed = { $name } இலிருந்து நீக்கப்பட்டது
contacts-label-renamed = லேபிள் பெயர் { $name } என மாற்றப்பட்டது
contacts-label-deleted = லேபிள் { $name } நீக்கப்பட்டது
contacts-label-no-email = இந்த லேபிளில் உள்ள யாருக்கும் மின்னஞ்சல் முகவரி இல்லை
# The column's list of mail accounts, each with the people saved in it.
contacts-accounts = கணக்குகள்
# The line under an account whose contacts did not come: why, and the one
# click that fixes it.
contacts-account-sign-in = தொடர்புகளைக் காட்ட மீண்டும் உள்நுழை
contacts-account-signed-in = { $address } இல் மீண்டும் உள்நுழைந்தது. உங்கள் தொடர்புகளைப் பெறுகிறது…
contacts-account-sign-in-refused = { $provider } Katna-வை உள்ளே அனுமதிக்கவில்லை. மீண்டும் முயன்று, உங்கள் தொடர்புகளுக்கான அணுகலை அனுமதிக்கவும்.
contacts-account-password = சர்வர் கடவுச்சொல்லை ஏற்கவில்லை. Yahoo, iCloud, Zoho போன்றவற்றுக்கு ஆப் கடவுச்சொல் தேவை.
contacts-account-change-password = கடவுச்சொல்லை மாற்று
contacts-account-change-password-tooltip = அமைப்புகள் > கணக்குகள் என்பதைத் திற
contacts-account-failed = தொடர்புகளைப் படிக்க முடியவில்லை.
# $reason is the server's own words, in English.
contacts-account-error = தொடர்புகளைப் படிக்க முடியவில்லை: { $reason }
contacts-account-none = முகவரிப் புத்தகம் எதுவும் இல்லை
# $reason is what the server answered, in English: "CardDAV https://dav.example.in/: status 404".
contacts-account-none-why = முகவரிப் புத்தகம் எதுவும் இல்லை: { $reason }
# A Gmail or Outlook account added with a password: its contacts need the
# provider's sign-in.
contacts-account-use-sign-in = { $provider } மூலம் உள்நுழைந்த Katna-வுக்கு மட்டுமே { $provider } தொடர்புகளைக் காட்டும்.
contacts-account-sign-in-with = { $provider } மூலம் உள்நுழை
contacts-account-looking = தொடர்புகளைத் தேடுகிறது…
contacts-account-try-again = மீண்டும் முயல்க
contacts-account-try-again-tooltip = இந்தக் கணக்கின் தொடர்புகளை இப்போது மீண்டும் சரிபார்
contacts-account-fixing = சரிசெய்கிறது…
contacts-manage = சரிசெய்து நிர்வகி
contacts-merge = ஒன்றிணைத்துச் சரிசெய்
contacts-merge-about = { $count ->
    [one] { $count } பரிந்துரை: ஒரே நபராகத் தோன்றும் தொடர்புகள்
   *[other] { $count } பரிந்துரை: ஒரே நபராகத் தோன்றும் தொடர்புகள்
}
contacts-merge-none = நகல்கள் இல்லை. ஒரே பெயர் அல்லது ஃபோன் எண் கொண்ட தொடர்புகள் இங்கே தோன்றும்.
contacts-merge-count = { $count ->
    [one] { $count } தொடர்புகள்
   *[other] { $count } தொடர்புகள்
}
contacts-merge-all = அனைத்தையும் ஒன்றிணை
contacts-merge-button = ஒன்றிணை
contacts-merge-dismiss = நிராகரி
contacts-merged = { $count ->
    [1] தொடர்புகள் ஒன்றிணைக்கப்பட்டன
    [one] { $count } ஒன்றிணைப்புகள் முடிந்தன
   *[other] { $count } ஒன்றிணைப்புகள் முடிந்தன
}
contacts-import = இறக்குமதி செய்
contacts-export = ஏற்றுமதி செய்
contacts-import-file = vCard அல்லது CSV கோப்பிலிருந்து தொடர்புகளை இறக்குமதி செய்
contacts-imported = { $count ->
    [one] { $place } இல் { $count } தொடர்புகள் இறக்குமதி செய்யப்பட்டன
   *[other] { $place } இல் { $count } தொடர்புகள் இறக்குமதி செய்யப்பட்டன
}
contacts-imported-some = { $count ->
    [one] { $place } இல் { $count } தொடர்புகள் இறக்குமதி செய்யப்பட்டன; ஏற்கெனவே சேமிக்கப்பட்ட { $skipped } தவிர்க்கப்பட்டன
   *[other] { $place } இல் { $count } தொடர்புகள் இறக்குமதி செய்யப்பட்டன; ஏற்கெனவே சேமிக்கப்பட்ட { $skipped } தவிர்க்கப்பட்டன
}
contacts-import-none = { $name } இல் தொடர்புகள் எதுவும் இல்லை
contacts-import-all-saved = { $name } இல் உள்ள அனைவரும் ஏற்கெனவே சேமிக்கப்பட்டுள்ளனர்
contacts-import-failed = { $name } ஐப் படிக்க முடியவில்லை: { $error }
contacts-exported = { $count ->
    [one] { $path } இல் { $count } தொடர்புகள் ஏற்றுமதி செய்யப்பட்டன
   *[other] { $path } இல் { $count } தொடர்புகள் ஏற்றுமதி செய்யப்பட்டன
}
contacts-export-none = ஏற்றுமதி செய்ய தொடர்புகள் இல்லை
contacts-export-failed = தொடர்புகளை ஏற்றுமதி செய்ய முடியவில்லை: { $error }
contacts-print = அச்சிடு
contacts-print-title = தொடர்புகள்
contacts-print-none = அச்சிட தொடர்புகள் இல்லை
contacts-print-typed = { $value } ({ $kind })
contacts-print-birthday = பிறந்தநாள்: { $day }
contacts-print-nickname = செல்லப்பெயர்: { $name }
contacts-create = தொடர்பை உருவாக்கு

## Search and the list

contacts-search = தொடர்புகளைத் தேடு
contacts-loading = தொடர்புகளை ஏற்றுகிறது…
contacts-empty = சேமித்த தொடர்புகள் இன்னும் இல்லை. Gmail, Outlook அல்லது உங்கள் அஞ்சல் சேவையில் சேமிக்கும் தொடர்புகள் இங்கே தோன்றும்.
contacts-empty-no-books = உங்கள் கணக்குகளின் தொடர்புகள் ஒத்திசைந்ததும் இங்கே தோன்றும்.
contacts-none-found = உங்கள் தேடலுக்குப் பொருந்தும் தொடர்புகள் இல்லை.
contacts-starred = { $count ->
    [one] நட்சத்திரமிட்ட தொடர்பு ({ $count })
   *[other] நட்சத்திரமிட்ட தொடர்புகள் ({ $count })
}
contacts-count = தொடர்புகள் ({ $count })
contacts-col-name = பெயர்
contacts-col-email = மின்னஞ்சல்
contacts-col-phone = தொலைபேசி எண்
contacts-col-job = பணிப் பெயர் & நிறுவனம்
contacts-col-labels = லேபிள்கள்

## Asking to allow contacts, for accounts signed in before Katna read them

contacts-allow = { $address } இன் தொடர்புகளைப் படிக்க Katna-வை அனுமதிக்கவும்.
contacts-allow-many = { $more ->
    [one] { $address } மற்றும் மேலும் { $more } கணக்கின் தொடர்புகளைப் படிக்க Katna-வை அனுமதிக்கவும்.
   *[other] { $address } மற்றும் மேலும் { $more } கணக்குகளின் தொடர்புகளைப் படிக்க Katna-வை அனுமதிக்கவும்.
}
contacts-allow-button = அனுமதி

## A contact's page

contacts-back = தொடர்புகளுக்குத் திரும்பு
contacts-edit = திருத்து
contacts-delete = நீக்கு
contacts-qr = QR குறியீடாகப் பகிர்
contacts-qr-about = தொடர்பைச் சேமிக்க, இதை ஃபோனின் கேமராவால் ஸ்கேன் செய்.
contacts-qr-too-long = QR குறியீட்டில் அடங்க இந்தத் தொடர்பில் விவரங்கள் மிக அதிகமாக உள்ளன.
contacts-qr-done = முடிந்தது
contacts-deleted = { $name } நீக்கப்பட்டது
contacts-added = { $name } தொடர்புகளில் சேர்க்கப்பட்டது
contacts-find-mail = அஞ்சல்
contacts-details = தொடர்பு விவரங்கள்
contacts-saved-in = சேமித்த இடம்
contacts-notes = குறிப்புகள்
contacts-birthday = பிறந்தநாள்
contacts-nickname = செல்லப்பெயர்
contacts-this-computer = இந்தக் கணினி
contacts-kind-home = வீடு
contacts-kind-work = பணியிடம்
contacts-kind-mobile = மொபைல்
contacts-kind-other = மற்றவை
contacts-source-google = Google தொடர்புகள்
contacts-source-microsoft = Outlook தொடர்புகள்
contacts-source-carddav = CardDAV

## Creating and changing a contact

contacts-edit-new-title = தொடர்பை உருவாக்கு
contacts-edit-title = தொடர்பைத் திருத்து
contacts-edit-save = சேமி
contacts-edit-saving = சேமிக்கிறது…
contacts-edit-cancel = ரத்துசெய்
contacts-saved = தொடர்பு சேமிக்கப்பட்டது
contacts-edit-save-to = இதில் சேமி
contacts-edit-changes-go-to = மாற்றங்கள் { $place } இல் சேமிக்கப்படும்.
contacts-edit-given = முதல் பெயர்
contacts-edit-family = கடைசி பெயர்
contacts-edit-company = நிறுவனம்
contacts-edit-job = பணிப் பெயர்
contacts-edit-email = மின்னஞ்சல்
contacts-edit-phone = தொலைபேசி
contacts-edit-with-kind = { $field } ({ $kind })
contacts-edit-add-email = மின்னஞ்சலைச் சேர்
contacts-edit-add-phone = தொலைபேசியைச் சேர்
contacts-edit-street = தெரு முகவரி
contacts-edit-city = நகரம்
contacts-edit-postcode = அஞ்சல் குறியீடு
contacts-edit-country = நாடு
contacts-edit-birthday = பிறந்தநாள் (YYYY-MM-DD)
contacts-edit-empty = முதலில் பெயர், மின்னஞ்சல் அல்லது தொலைபேசி எண்ணைச் சேர்க்கவும்.
