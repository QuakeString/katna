# Katna Mail, Tamil (தமிழ்).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = ஃபைல்களில் தேடு

## Left side (and chips on a phone)

files-all = எல்லா ஃபைல்களும்
files-pictures = படங்கள்
files-pdfs = PDF-கள்
files-documents = ஆவணங்கள்
files-sheets = விரிதாள்கள்
files-slides = ஸ்லைடுகள்
files-other = மற்றவை
files-accounts = கணக்குகள்
files-drives = டிரைவ்கள்
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = என்னுடன் பகிரப்பட்டவை
files-shown = காட்டப்படுபவை
files-received = பெற்றவை
files-sent = நான் அனுப்பியவை

## Over the files

files-count = { $count ->
    [one] { $count } ஃபைல் · { $size }
   *[other] { $count } ஃபைல்கள் · { $size }
}
files-anyone = யாரும்
files-from-person = { $name } இடமிருந்து
files-time-any = எந்த நேரமும்
files-time-today = இன்று
files-time-yesterday = நேற்று
files-time-this-week = இந்த வாரம்
files-time-last-week = கடந்த வாரம்
files-time-this-month = இந்த மாதம்
files-time-last-month = கடந்த மாதம்
files-time-between = { $first } – { $last }
files-time-hint = ஒரு நாளைக் கிளிக் செய்யுங்கள், அல்லது நாட்களின் குறுக்கே இழுங்கள்
files-time-summary = { $count ->
    [one] { $days } · { $count } ஃபைல்
   *[other] { $days } · { $count } ஃபைல்கள்
}
files-time-clear = அழி
files-time-month-back = முந்தைய மாதம்
files-time-month-on = அடுத்த மாதம்
files-time-wheel = இந்தத் தேதிகளின் நீளம் மாறாமல் அவற்றை நகர்த்த உருட்டுங்கள்
files-sort-newest = புதியவை முதலில்
files-sort-oldest = பழையவை முதலில்
files-sort-largest = பெரியவை முதலில்
files-sort-name = பெயர்படி
files-grid = கார்டுகள்
files-list = பட்டியல்
files-this-week = இந்த வாரம்
files-undated = தேதி இல்லை
files-me = நான்
files-no-subject = (பொருள் இல்லை)
files-loading = உங்கள் அஞ்சலிலிருந்து ஃபைல்களைச் சேகரிக்கிறது…
files-empty = உங்கள் அஞ்சலில் உள்ள ஃபைல்கள் இங்கே காட்டப்படும்.
files-none-match = பொருந்தும் ஃபைல்கள் இல்லை.
files-load-failed = ஃபைல்களைப் படிக்க முடியவில்லை: { $error }

## A file's menu and buttons

files-open = திற
files-open-with = இதில் திற…
files-save = சேமி…
files-show-mail = அஞ்சலைக் காட்டு
files-mail-window = அஞ்சலைப் புதிய சாளரத்தில் திற
files-forward = ஃபைலை முன்னனுப்பு
files-from-them = { $name } இடமிருந்து வந்த ஃபைல்கள்
files-copy-name = ஃபைல் பெயரை நகலெடு
files-name-copied = ஃபைல் பெயர் நகலெடுக்கப்பட்டது
files-downloading = அஞ்சலைப் பதிவிறக்குகிறது…
files-download-failed = இந்த அஞ்சலைப் பதிவிறக்க முடியவில்லை.

## A cloud drive in place of the mail files

files-drive-mine = எனது டிரைவ்
files-drive-mine-onedrive = எனது ஃபைல்கள்
files-drive-results = “{ $words }”
files-drive-count = { $folders ->
    [0] { $files ->
        [one] { $files } ஃபைல்
       *[other] { $files } ஃபைல்கள்
    }
    [one] { $folders } ஃபோல்டர் · { $files ->
        [one] { $files } ஃபைல்
       *[other] { $files } ஃபைல்கள்
    }
   *[other] { $folders } ஃபோல்டர்கள் · { $files ->
        [one] { $files } ஃபைல்
       *[other] { $files } ஃபைல்கள்
    }
}
files-drive-folders = ஃபோல்டர்கள்
files-drive-files = ஃபைல்கள்
files-drive-folder = ஃபோல்டர்
files-drive-meta = { $what } · { $date } அன்று திருத்தப்பட்டது
files-drive-as-link = { $what } · இணைப்பாக
files-drive-google-doc = Google Doc
files-drive-google-sheet = Google Sheet
files-drive-google-slides = Google Slides
files-drive-google-drawing = Google Drawing
files-drive-fetching = பெறுகிறது…
files-drive-loading = டிரைவைத் திறக்கிறது…
files-drive-empty = இந்த ஃபோல்டர் காலியாக உள்ளது.
files-drive-unreachable = { $drive } ஐ அணுக முடியவில்லை.
files-drive-try-again = மீண்டும் முயல்
files-drive-needs-permission = இந்த டிரைவைக் காட்ட Katna-வுக்கு ஒருமுறை உங்கள் அனுமதி தேவை. மீண்டும் உள்நுழைந்து, உங்கள் ஃபைல்களைப் பார்க்க Katna-வை அனுமதியுங்கள்.
files-drive-allow = அனுமதி
files-drive-allow-failed = உள்நுழைவு முடியவில்லை, எனவே டிரைவ் மூடியே இருக்கும்.
files-drive-attach = இணை
files-drive-more = மேலும்
files-drive-download = பதிவிறக்கு…
files-drive-open-web = { $drive } இல் திற
files-drive-copy-link = இணைப்பை நகலெடு
files-drive-link-copied = இணைப்பு நகலெடுக்கப்பட்டது
files-drive-share = பகிர்…
files-drive-rename = பெயர் மாற்று
files-drive-trash = குப்பைக்கு நகர்த்து
files-drive-trashed = “{ $name }” { $drive } குப்பையில் உள்ளது
files-drive-renamed = “{ $name }” எனப் பெயர் மாற்றப்பட்டது
files-drive-getting = { $drive } இலிருந்து { $name } ஐப் பெறுகிறது…
files-drive-get-failed = { $name } ஐப் பெற முடியவில்லை: { $error }
files-drive-upload = பதிவேற்று
files-drive-upload-files = ஃபைல்களைப் பதிவேற்று
files-drive-upload-folder = ஃபோல்டரைப் பதிவேற்று
files-drive-upload-failed = { $name } ஐப் பதிவேற்ற முடியவில்லை: { $error }
files-drive-upload-needs = பதிவேற்ற, Katna-வுக்கு ஒருமுறை உங்கள் அனுமதி தேவை: அமைப்புகள் › இயல்புநிலை ஆப்ஸ் › ஃபைல்கள் பக்கம் என்பதில் அனுமதி என்பதை அழுத்துங்கள்.

## The Share dialog of a drive file or folder

files-share-title = “{ $name }” ஐப் பகிர்
files-share-add = பெயர் அல்லது முகவரி மூலம் நபர்களைச் சேர்
files-share-not-address = “{ $text }” ஒரு மின்னஞ்சல் முகவரி அல்ல
files-share-notify = { $drive } அவர்களுக்கும் மின்னஞ்சல் அனுப்பட்டும்
files-share-people = அணுகல் உள்ள நபர்கள்
files-share-general = பொது அணுகல்
files-share-loading = யாருக்கு அணுகல் உள்ளது என்று படிக்கிறது…
files-share-restricted = வரம்பிடப்பட்டது
files-share-restricted-about = அணுகல் உள்ள நபர்கள் மட்டுமே இணைப்பின் மூலம் இதைத் திறக்க முடியும்
files-share-anyone = இணைப்பு உள்ள எவரும்
files-share-anyone-can = { $role ->
    [editor] இணைப்பு உள்ள எவரும் திருத்தலாம்
    [commenter] இணைப்பு உள்ள எவரும் கருத்துத் தெரிவிக்கலாம்
   *[viewer] இணைப்பு உள்ள எவரும் பார்க்கலாம்
}
files-share-anyone-about = { $role ->
    [editor] இணையத்தில் இணைப்பு உள்ள எவரும் திருத்தலாம்
    [commenter] இணையத்தில் இணைப்பு உள்ள எவரும் கருத்துத் தெரிவிக்கலாம்
   *[viewer] இணையத்தில் இணைப்பு உள்ள எவரும் பார்க்கலாம்
}
files-share-role-owner = உரிமையாளர்
files-share-role-editor = எடிட்டர்
files-share-role-commenter = கருத்துரைப்பவர்
files-share-role-viewer = பார்வையாளர்
files-share-you = { $name } (நீங்கள்)
files-share-domain = { $domain } இல் உள்ள அனைவரும்
files-share-inherited = இது உள்ள ஃபோல்டரிலிருந்து அணுகல்
files-share-remove = அணுகலை அகற்று
files-share-copy-link = இணைப்பை நகலெடு
files-share-share = பகிர்
files-share-done = முடிந்தது
files-share-close = மூடு
files-share-sharing = பகிர்கிறது…
files-share-shared = { $count ->
    [one] { $count } நபருடன் பகிரப்பட்டது
   *[other] { $count } நபர்களுடன் பகிரப்பட்டது
}
files-share-refused = { $drive } ஆல் { $addresses } உடன் பகிர முடியவில்லை
files-share-failed = பகிர்வை மாற்ற முடியவில்லை: { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
    [one] { $count } உருப்படியைப் பதிவேற்றுகிறது
   *[other] { $count } உருப்படிகளைப் பதிவேற்றுகிறது
}
files-tray-done = { $count ->
    [one] { $count } பதிவேற்றம் முடிந்தது
   *[other] { $count } பதிவேற்றங்கள் முடிந்தன
}
files-tray-some-failed = { $done } பதிவேற்றப்பட்டன, { $failed } தோல்வியடைந்தன
files-tray-minutes-left = { $minutes ->
    [one] சுமார் { $minutes } நிமிடம் மீதமுள்ளது
   *[other] சுமார் { $minutes } நிமிடங்கள் மீதமுள்ளன
}
files-tray-seconds-left = ஒரு நிமிடத்துக்கும் குறைவாக மீதமுள்ளது
files-tray-starting = தொடங்குகிறது…
files-tray-cancel-all = அனைத்தையும் ரத்துசெய்
files-tray-cancel = ரத்துசெய்
files-tray-fold = பட்டியலை மறை
files-tray-unfold = பட்டியலைக் காட்டு
files-tray-close = மூடு
files-tray-progress = { $place } · { $size } இல் { $sent }
files-tray-in = { $place } இல்
files-tray-cancelled = ரத்துசெய்யப்பட்டது
