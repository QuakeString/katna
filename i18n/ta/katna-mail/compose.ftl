# Katna Mail, Tamil (தமிழ்).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Compose window: title bar

compose-new-message = புதிய மெசேஜ்
compose-restore = மீட்டமை
compose-minimize = சிறிதாக்கு
compose-exit-full-screen = முழுத்திரையிலிருந்து வெளியேறு
compose-open-window = புதிய சாளரத்தில் திற
compose-save-close = சேமித்து மூடு
compose-back-to-mail = அஞ்சல் சாளரத்துக்குத் திரும்பு
compose-pop-out-reply = பதிலைத் தனியாகத் திற
compose-edit-recipients = பெறுநர்களைத் திருத்து
compose-summary-cc = Cc: { $names }
compose-summary-bcc = Bcc: { $names }
compose-more-recipients = மேலும் { $count }
compose-show-trimmed = சுருக்கிய உள்ளடக்கத்தைக் காட்டு
compose-hide-trimmed = சுருக்கிய உள்ளடக்கத்தை மறை
compose-remove-trimmed = மேற்கோள் உரையை நீக்கு
compose-trimmed-removed = மேற்கோள் உரை நீக்கப்பட்டது

## Recipients and subject

compose-to = பெறுநர்
compose-cc = Cc
compose-bcc = Bcc
compose-from = அனுப்புநர்
compose-from-choose = வேறு கணக்கிலிருந்து அனுப்பு
compose-recipients = பெறுநர்கள்
compose-subject = பொருள்

## Sending (the notes at the bottom of the window)

compose-open-elsewhere = திறந்திருக்கும் மெசேஜை முதலில் அனுப்பவும் அல்லது நிராகரிக்கவும்.
compose-bad-address = “{ $address }” மின்னஞ்சல் முகவரி அல்ல.
compose-no-recipients = குறைந்தது ஒரு பெறுநரைச் சேர்க்கவும்.
compose-attachments-too-large = இணைப்புகள் { $size }; அஞ்சல் சர்வர்கள் { $limit } வரை மட்டுமே ஏற்கும்.
compose-no-account = அஞ்சல் அனுப்ப ஒரு கணக்கைச் சேர்க்கவும்.
compose-past-time = எதிர்கால நேரத்தைத் தேர்ந்தெடுக்கவும்.
compose-scheduling = திட்டமிடுகிறது…
compose-sending = அனுப்புகிறது…
compose-scheduled = { $when } அன்று அனுப்பத் திட்டமிடப்பட்டது
compose-sent-archived = அனுப்பிக் காப்பகப்படுத்தப்பட்டது
compose-sent = மெசேஜ் அனுப்பப்பட்டது
compose-discarded = வரைவு நிராகரிக்கப்பட்டது
compose-draft-saved = வரைவு சேமிக்கப்பட்டது
compose-draft-failed = வரைவைச் சேமிக்க முடியவில்லை: { $error }
compose-draft-not-opened = வரைவைத் திறக்க முடியவில்லை.

## Attachments

compose-picker-insert = செருகு
compose-picker-attach = இணை
compose-file-too-large = { $name } மிகப் பெரியது: ஒரு மெசேஜ் { $limit } வரை மட்டுமே கொண்டு செல்லும்.
compose-attachment-size = ({ $size })
compose-remove-attachment = இணைப்பை அகற்று
compose-attachments-total = { $count ->
    [one] { $count } கோப்பு, { $size }
   *[other] { $count } கோப்புகள், { $size }
}
compose-drive-note = { $name } { $limit } அளவைத் தாண்டுவதால், அது உங்கள் Google Drive-க்குச் செல்கிறது; மெசேஜில் அதன் லிங்க் இருக்கும்.
compose-drive-tip = உங்கள் Google Drive-இல்; மெசேஜில் லிங்க் இருக்கும்
compose-drive-uploading = பதிவேற்றுகிறது { $percent }%
compose-drive-allow = Drive-ஐ அனுமதி
compose-drive-allow-tip = பெரிய கோப்புகளை உங்கள் Drive-இல் Katna சேமிக்க Google மூலம் மீண்டும் உள்நுழையுங்கள்
compose-drive-retry = மீண்டும் முயல்க
compose-drive-sends-when-uploaded = { $name } பதிவேற்றப்பட்டதும் அனுப்பப்படும்
compose-drive-not-uploaded = { $name } இன்னும் Google Drive-இல் இல்லை
compose-drive-share-failed = Google Drive-இல் கோப்புகளைப் பகிர முடியவில்லை: { $error }
compose-drive-share-title = கோப்புகளை அனைவருடனும் பகிரவா?
compose-drive-share-text = { $count ->
    [one] Google கணக்கு இல்லாத { $addresses } உடன் Google Drive கோப்புகளைப் பகிர முடியாது. அதற்குப் பதிலாக, லிங்க் உள்ள எவரும் அவற்றைத் திறக்கலாம்.
   *[other] Google கணக்கு இல்லாத { $addresses } உடன் Google Drive கோப்புகளைப் பகிர முடியாது. அதற்குப் பதிலாக, லிங்க் உள்ள எவரும் அவற்றைத் திறக்கலாம்.
}
compose-drive-share-link = லிங்குடன் பகிர்
compose-drive-send-without = பகிராமல் அனுப்பு
compose-drive-share-cancel = ரத்துசெய்
compose-drive-card-detail = { $size } · Google Drive
compose-onedrive-note = { $name } { $limit } அளவைத் தாண்டுவதால், அது உங்கள் OneDrive-க்குச் செல்கிறது; மெசேஜில் அதன் லிங்க் இருக்கும்.
compose-onedrive-tip = உங்கள் OneDrive-இல்; மெசேஜில் லிங்க் இருக்கும்
compose-onedrive-allow = OneDrive-ஐ அனுமதி
compose-onedrive-allow-tip = பெரிய கோப்புகளை உங்கள் OneDrive-இல் Katna சேமிக்க Microsoft மூலம் மீண்டும் உள்நுழையுங்கள்
compose-onedrive-not-uploaded = { $name } இன்னும் OneDrive-இல் இல்லை
compose-onedrive-share-failed = OneDrive-இல் கோப்புகளைப் பகிர முடியவில்லை: { $error }
compose-onedrive-share-text = { $count ->
    [one] OneDrive-ஆல் { $addresses } உடன் கோப்புகளைப் பகிர முடியாது. அதற்குப் பதிலாக, லிங்க் உள்ள எவரும் அவற்றைத் திறக்கலாம்.
   *[other] OneDrive-ஆல் { $addresses } உடன் கோப்புகளைப் பகிர முடியாது. அதற்குப் பதிலாக, லிங்க் உள்ள எவரும் அவற்றைத் திறக்கலாம்.
}
compose-onedrive-card-detail = { $size } · OneDrive
compose-drop-files = கோப்புகளை இங்கே விடவும்
compose-drop-here = இங்கே விடவும்
compose-paste-keep-formatting = வடிவமைப்பை வைத்திரு
compose-paste-table = அட்டவணை
compose-paste-picture = படம்
compose-paste-plain-text = வெற்று உரை
compose-paste-inline = உரையில்
compose-paste-attachment = இணைப்பு

## Encryption and signing (the toggles by the recipients)

compose-encrypt = என்க்ரிப்ட் செய்
compose-encrypted = என்க்ரிப்ட் செய்யப்பட்டது: பெறுநர்கள் மட்டுமே படிக்க முடியும்
compose-sign = கையொப்பமிடு
compose-signed = கையொப்பமிடப்பட்டது: இது உங்களிடமிருந்து வந்தது என்று பெறுநர்கள் சரிபார்க்கலாம்
compose-track = திறப்புகளையும் கிளிக்குகளையும் கண்காணி
compose-tracked = கண்காணிக்கப்படுகிறது: ஒவ்வொரு பெறுநரும் இதைத் திறக்கும்போதோ லிங்க்கைத் திறக்கும்போதோ நீங்கள் பார்க்கலாம்
compose-track-clicks = லிங்க் கிளிக்குகளைக் கண்காணி (வெற்று உரையில் திறப்புகளைக் காட்ட முடியாது)
compose-tracked-clicks = கண்காணிக்கப்படுகிறது: ஒவ்வொரு பெறுநரும் லிங்க்கைத் திறக்கும்போது நீங்கள் பார்க்கலாம்
compose-track-sign-in = திறப்புகளையும் கிளிக்குகளையும் கண்காணிக்க Katna கணக்கில் உள்நுழையுங்கள்
compose-receipt = படித்த ரசீதைக் கோரு
compose-receipt-on = படித்த ரசீது கோரப்பட்டது: பெறுநரின் ஆப் அதை அனுப்பும்படி அவரிடம் கேட்கலாம்
compose-delivery = டெலிவரி ரசீதைக் கோரு
compose-delivery-on = டெலிவரி ரசீது கோரப்பட்டது: ஒவ்வொரு பெறுநரின் சர்வரும் மெசேஜை ஏற்கும்போது உங்கள் அஞ்சல் சர்வர் உங்களுக்கு மின்னஞ்சல் அனுப்பும்
compose-delivery-unavailable = உங்கள் அஞ்சல் சர்வர் டெலிவரி ரசீதுகளை அனுப்புவதில்லை

## Spelling

spell-no-dictionary = { $language } க்கான எழுத்துப்பிழை அகராதி நிறுவப்படவில்லை (எ.கா. hunspell-en_us).
spell-dictionary-error = எழுத்துப்பிழை அகராதி: { $error }

## Grammar checking (the right-click menu on a grammar mistake)

grammar-replace = “{ $words }”
grammar-add = “{ $words }” ஐச் சேர்
grammar-remove = “{ $words }” ஐ அகற்று
grammar-ignore = புறக்கணி

## Send checks (asked before a message goes out)

send-check-attachment-title = கோப்புகளை இணைக்க நினைத்தீர்களா?
send-check-attachment-text = நீங்கள் ஒரு இணைப்பைப் பற்றி எழுதினீர்கள், ஆனால் எதுவும் இணைக்கப்படவில்லை.
send-check-attach = கோப்பை இணை
send-check-subject-title = பொருள் இல்லாமல் அனுப்பவா?
send-check-subject-text = இந்த மெசேஜுக்குப் பொருள் இல்லை.
send-check-add-subject = பொருளைச் சேர்
send-check-send-anyway = பரவாயில்லை, அனுப்பு
recipient-not-valid = சரியான மின்னஞ்சல் முகவரி அல்ல
recipient-show-address = முகவரியைக் காட்டு
recipient-remove = அகற்று
recipient-bad-title = முகவரியைச் சரிபார்க்கவும்
recipient-bad-text = “{ $address }” சரியான மின்னஞ்சல் முகவரி அல்ல. அனுப்பும் முன் அதைச் சரிசெய்யவும் அல்லது அகற்றவும்.
recipient-bad-fix = சரிசெய்
