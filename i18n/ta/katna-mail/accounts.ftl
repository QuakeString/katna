# Katna Mail, Tamil (தமிழ்).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Accounts

accounts-folder-pane = ஃபோல்டர் பலகம்
accounts-folder-pane-detail = இடதுபுறப் பலகம் எந்தக் கணக்குகளின் ஃபோல்டர்களைக் காட்டும்.
accounts-shown-one = ஒரு நேரத்தில் ஒரு கணக்கு; கணக்கு கார்டில் மாற்றலாம்
accounts-shown-all = எல்லாக் கணக்குகளும், ஒன்றன்பின் ஒன்றாக
accounts-unified = ஒருங்கிணைந்த இன்பாக்ஸ்
accounts-unified-switch = எல்லாக் கணக்குகளின் அஞ்சலையும் ஒன்றாகக் காட்டு
accounts-unified-switch-detail = “எல்லாக் கணக்குகளும்” ஃபோல்டர் பலகத்தின் மேலே இருக்கும்; ஒவ்வொரு கணக்கின் இன்பாக்ஸ், அனுப்பிய அஞ்சல் மற்றும் பலவும் ஒரே பட்டியலில் இருக்கும். அதன் கீழுள்ள கணக்குகள் மடிக்கப்பட்ட நிலையில் தொடங்கும்.
accounts-row = கணக்குகள்
accounts-row-detail = ஃபோல்டர் பலகமும் கணக்கு மெனுவும் கணக்குகளை இந்த வரிசையில் காட்டும்; முதலாவது இயல்புநிலைக் கணக்கு. ஒரு கணக்கை அகற்றினால், இந்தக் கணினியில் உள்ள அதன் அஞ்சலின் Katna நகல் நீக்கப்படும். அஞ்சல் சர்வரில் அப்படியே இருக்கும்.
accounts-none = இன்னும் கணக்குகள் இல்லை.
accounts-pop3-row = சர்வரில் உள்ள அஞ்சல்
accounts-pop3-row-detail = POP3 கணக்குகள் அஞ்சலை இந்தக் கணினிக்குப் பதிவிறக்கும். அதன் பிறகு சர்வரில் உள்ள நகலுக்கு என்ன ஆக வேண்டும் என்பதைத் தேர்வுசெய்யுங்கள்.
accounts-pop3-with-katna = Katna-வில் நான் நீக்கும் வரை வைத்திரு
accounts-pop3-at-once = பதிவிறக்கியதும் நீக்கு
accounts-pop3-after-days = { $count ->
    [one] { $count } நாளுக்குப் பிறகு நீக்கு
   *[other] { $count } நாட்களுக்குப் பிறகு நீக்கு
}
accounts-pop3-never = ஒருபோதும் நீக்க வேண்டாம்
accounts-pop3-days-less = குறைவான நாட்கள்
accounts-pop3-days-more = அதிக நாட்கள்
accounts-kind-imported = இம்போர்ட் செய்யப்பட்டது
accounts-picture-reset = டெஸ்க்டாப் படத்தைப் பயன்படுத்து
accounts-picture-change = படத்தை மாற்று
accounts-picture-remove = படத்தை அகற்று
accounts-rename = பெயர் மாற்று
accounts-name-save = சேமி
accounts-name-cancel = ரத்துசெய்
accounts-name-placeholder = உங்கள் பெயர்
accounts-rename-failed = கணக்கின் பெயரை மாற்ற முடியவில்லை: { $error }
accounts-move-up = மேலே நகர்த்து
accounts-move-down = கீழே நகர்த்து
accounts-drag = வரிசையை மாற்ற இழுக்கவும்
accounts-remove = அகற்று
accounts-delete-all-row = எல்லாத் தரவையும் நீக்கு
accounts-delete-all-row-detail = புதிதாக நிறுவியது போல, மீண்டும் தொடங்குங்கள்.
accounts-delete-all-about = ஒவ்வொரு கணக்கு, சேமித்த எல்லா அஞ்சல்கள், தொடர்புகள், கேலெண்டர்கள், தேடல் அட்டவணை, உங்கள் அமைப்புகள், சேமித்த கடவுச்சொற்கள் ஆகியவற்றை இந்தக் கணினியிலிருந்து நீக்கும். உங்கள் அஞ்சல் சர்வர்களில் எதுவும் மாறாது.
accounts-delete-all-open = எல்லா Katna தரவையும் நீக்கு

## Settings > Accounts: snackbars after deleting

accounts-removed-local = { $address } Katna இலிருந்து அகற்றப்பட்டது.
accounts-removed = { $address } Katna இலிருந்து அகற்றப்பட்டது. அதன் அஞ்சல் இன்னும் சர்வரில் உள்ளது.
accounts-all-deleted = எல்லா Katna தரவும் இந்தக் கணினியிலிருந்து நீக்கப்பட்டது.

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = { $address } ஐ அகற்றவா?
accounts-remove-confirm = கணக்கை அகற்று
accounts-removing = அகற்றுகிறது…
accounts-remove-local-mail = { $folders ->
    [0] இந்தக் கணக்கில் இம்போர்ட் செய்யப்பட்ட எல்லா அஞ்சல்களும்
    [one] இந்தக் கணக்கில் இம்போர்ட் செய்யப்பட்ட, அதன் ஃபோல்டரில் உள்ள எல்லா அஞ்சல்களும்
   *[other] இந்தக் கணக்கில் இம்போர்ட் செய்யப்பட்ட, அதன் { $folders } ஃபோல்டர்களில் உள்ள எல்லா அஞ்சல்களும்
}
accounts-remove-local-settings = அதன் Katna அமைப்புகள்
accounts-remove-mail = { $folders ->
    [0] Katna சேமித்த இந்தக் கணக்கின் எல்லா அஞ்சல்களும்
    [one] Katna அதன் ஃபோல்டரில் சேமித்த இந்தக் கணக்கின் எல்லா அஞ்சல்களும்
   *[other] Katna அதன் { $folders } ஃபோல்டர்களில் சேமித்த இந்தக் கணக்கின் எல்லா அஞ்சல்களும்
}
accounts-remove-outbox = அவுட்பாக்ஸில் காத்திருக்கும் அதன் மெசேஜ்கள்
accounts-remove-settings = அதன் சேமித்த கடவுச்சொல்லும் Katna அமைப்புகளும்
accounts-delete-all-title = எல்லா Katna தரவையும் நீக்கவா?
accounts-delete-all-confirm = அனைத்தையும் நீக்கு
accounts-deleting = நீக்குகிறது…
accounts-delete-all-accounts = ஒவ்வொரு கணக்கும், Katna சேமித்த எல்லா அஞ்சல்களும் இணைப்புகளும்
accounts-delete-all-contacts = தொடர்புகள், கேலெண்டர்கள், தேடல் அட்டவணை
accounts-delete-all-settings = எல்லா அமைப்புகள், கையொப்பங்கள், கீபோர்டு ஷார்ட்கட்கள்
accounts-delete-all-passwords = சேமித்த ஒவ்வொரு கடவுச்சொல்லும்
accounts-deleted-heading = இந்தக் கணினியிலிருந்து நீக்கப்படுபவை:
accounts-cannot-undo = இதைச் செயல்தவிர்க்க முடியாது.
accounts-server-delete-all = உங்கள் அஞ்சல் சர்வர்களில் எதுவும் மாறாது: உங்கள் அஞ்சல் அங்கேயே இருக்கும், கணக்கை மீண்டும் சேர்த்தால் அது மீண்டும் பதிவிறக்கப்படும். ஃபைல்களிலிருந்து இம்போர்ட் செய்த அஞ்சல் Katna இல் மட்டுமே உள்ளது; அந்த ஃபைல்கள் தொடப்படாது.
accounts-server-local = இந்த அஞ்சல் ஃபைல்களிலிருந்து இம்போர்ட் செய்யப்பட்டது, எனவே அதன் ஒரே நகல் Katna இடம் மட்டுமே உள்ளது. அது வந்த ஃபைல்கள் தொடப்படாது; திரும்பப் பெற அவற்றை மீண்டும் இம்போர்ட் செய்யவும்.
accounts-server-remove = அஞ்சல் சர்வரில் எதுவும் மாறாது: உங்கள் அஞ்சல் அங்கேயே இருக்கும், கணக்கை மீண்டும் சேர்த்தால் அது மீண்டும் பதிவிறக்கப்படும்.
accounts-confirm-word = நீக்கு
accounts-confirm-placeholder = “{ accounts-confirm-word }” என டைப் செய்யவும்
accounts-confirm-prompt = உறுதிப்படுத்த, “{ accounts-confirm-word }” என டைப் செய்யவும்:
accounts-cancel = ரத்துசெய்

## Reset cache (Settings > General), in the same dialog

reset-cache-about = Katna பதிவிறக்கிய அஞ்சல்கள், இணைப்புகள், அனுப்புநர் படங்கள், தேடல் அட்டவணை ஆகியவற்றை நீக்கி, பின் சமீபத்திய அஞ்சலை மீண்டும் பதிவிறக்கும். கணக்குகள், அமைப்புகள், இந்தக் கணினியில் மட்டுமே உள்ள அஞ்சல் ஆகியவை அப்படியே இருக்கும்.
reset-cache-button = தற்காலிகச் சேமிப்பை மீட்டமை
reset-cache-title = தற்காலிகச் சேமிப்பை மீட்டமைக்கவா?
reset-cache-deleted = நீக்கப்பட்டு, மீண்டும் பதிவிறக்கப்படுபவை:
reset-cache-mail = உங்கள் IMAP சர்வர்களிலிருந்து பதிவிறக்கிய அஞ்சல்களும் இணைப்புகளும்: சமீபத்திய அஞ்சல் இப்போதே மீண்டும் பதிவிறக்கப்படும், பழைய அஞ்சல் நீங்கள் திறக்கும்போது
reset-cache-index = தேடல் அட்டவணை, இது உடனே மீண்டும் உருவாக்கப்படும்
reset-cache-pictures = அனுப்புநர் படங்கள்
reset-cache-kept = வைக்கப்படுபவை: உங்கள் கணக்குகள், கடவுச்சொற்கள், அமைப்புகள்; நட்சத்திரங்கள், லேபிள்கள், படித்த குறிகள், பின்கள்; வரைவுகள், அவுட்பாக்ஸ், இன்னும் சர்வரை அடையாத மாற்றங்கள்; POP3 கணக்குகளிலிருந்தோ இம்போர்ட் செய்த ஃபைல்களிலிருந்தோ வந்த அஞ்சல், அதற்கு வேறு நகல் இல்லாமல் இருக்கலாம். உங்கள் அஞ்சல் சர்வர்களில் எதுவும் மாறாது.
reset-cache-confirm = தற்காலிகச் சேமிப்பை மீட்டமை
reset-cache-busy = மீட்டமைக்கிறது…
reset-cache-done = தற்காலிகச் சேமிப்பு மீட்டமைக்கப்பட்டது. சமீபத்திய அஞ்சல் மீண்டும் பதிவிறக்கப்படுகிறது.
reset-cache-done-freed = தற்காலிகச் சேமிப்பு மீட்டமைக்கப்பட்டு { $size } இடம் விடுவிக்கப்பட்டது. சமீபத்திய அஞ்சல் மீண்டும் பதிவிறக்கப்படுகிறது.
