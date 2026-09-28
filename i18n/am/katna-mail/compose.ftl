# Katna Mail, Amharic (አማርኛ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Compose window: title bar

compose-new-message = አዲስ መልዕክት
compose-restore = ወደ ነበረበት መልስ
compose-minimize = አሳንስ
compose-exit-full-screen = ከሙሉ ማያ ገጽ ውጣ
compose-open-window = በአዲስ መስኮት ክፈት
compose-save-close = አስቀምጥ እና ዝጋ
compose-back-to-mail = ወደ ደብዳቤ መስኮቱ ተመለስ
compose-pop-out-reply = ምላሹን በተለየ መስኮት ክፈት
compose-edit-recipients = ተቀባዮችን አርትዕ
compose-summary-cc = Cc: { $names }
compose-summary-bcc = Bcc: { $names }
compose-more-recipients = { $count } ተጨማሪ
compose-show-trimmed = የተከረከመውን ይዘት አሳይ
compose-hide-trimmed = የተከረከመውን ይዘት ደብቅ
compose-remove-trimmed = የተጠቀሰውን ጽሑፍ አስወግድ
compose-trimmed-removed = የተጠቀሰው ጽሑፍ ተወግዷል

## Recipients and subject

compose-to = ለ
compose-cc = Cc
compose-bcc = Bcc
compose-from = ከ
compose-from-choose = ከሌላ መለያ ላክ
compose-recipients = ተቀባዮች
compose-subject = ርዕሰ ጉዳይ

## Sending (the notes at the bottom of the window)

compose-open-elsewhere = መጀመሪያ የተከፈተውን መልዕክት ይላኩ ወይም ይጣሉ።
compose-bad-address = «{ $address }» የኢሜይል አድራሻ አይደለም።
compose-no-recipients = ቢያንስ አንድ ተቀባይ ያክሉ።
compose-attachments-too-large = አባሪዎቹ { $size } ናቸው፤ የደብዳቤ አገልጋዮች እስከ { $limit } ይቀበላሉ።
compose-no-account = ደብዳቤ የሚላክበት መለያ ያክሉ።
compose-past-time = ወደፊት ያለ ጊዜ ይምረጡ።
compose-scheduling = መርሐግብር በማስያዝ ላይ…
compose-sending = በመላክ ላይ…
compose-scheduled = ለ{ $when } እንዲላክ መርሐግብር ተይዟል
compose-sent-archived = ተልኳል እና ወደ ማህደር ተቀምጧል
compose-sent = መልዕክቱ ተልኳል
compose-discarded = ረቂቁ ተጥሏል
compose-draft-saved = ረቂቁ ተቀምጧል
compose-draft-failed = ረቂቁን ማስቀመጥ አልተቻለም፦ { $error }
compose-draft-not-opened = ረቂቁን መክፈት አልተቻለም።

## Attachments

compose-picker-insert = አስገባ
compose-picker-attach = አያይዝ
compose-file-too-large = { $name } በጣም ትልቅ ነው፦ አንድ መልዕክት እስከ { $limit } መያዝ ይችላል።
compose-attachment-size = ({ $size })
compose-remove-attachment = አባሪውን አስወግድ
compose-attachments-total = { $count ->
    [one] { $count } ፋይል፣ { $size }
   *[other] { $count } ፋይሎች፣ { $size }
}
compose-drive-note = { $name } ከ{ $limit } ስለሚበልጥ ወደ Google Drive ይሄዳል፣ መልዕክቱም አገናኝ ይይዛል።
compose-drive-tip = በእርስዎ Google Drive ውስጥ፤ መልዕክቱ አገናኝ ይይዛል
compose-drive-uploading = በመስቀል ላይ { $percent }%
compose-drive-allow = Driveን ፍቀድ
compose-drive-allow-tip = Katna ትላልቅ ፋይሎችን በDrive እንዲያስቀምጥ በGoogle እንደገና ይግቡ
compose-drive-retry = እንደገና ሞክር
compose-drive-sends-when-uploaded = { $name } ከተሰቀለ በኋላ ይላካል
compose-drive-not-uploaded = { $name } ገና በGoogle Drive ውስጥ የለም
compose-drive-share-failed = ፋይሎቹን በGoogle Drive ማጋራት አልተቻለም፦ { $error }
compose-drive-share-title = ፋይሎቹን ለሁሉም ይጋሩ?
compose-drive-share-text = { $count ->
    [one] Google Drive ፋይሎቹን የGoogle መለያ ከሌለው ከ{ $addresses } ጋር ማጋራት አይችልም። በምትኩ አገናኙ ያለው ማንኛውም ሰው ሊከፍታቸው ይችላል።
   *[other] Google Drive ፋይሎቹን የGoogle መለያ ከሌላቸው ከ{ $addresses } ጋር ማጋራት አይችልም። በምትኩ አገናኙ ያለው ማንኛውም ሰው ሊከፍታቸው ይችላል።
}
compose-drive-share-link = በአገናኝ አጋራ
compose-drive-send-without = ሳያጋሩ ላክ
compose-drive-share-cancel = ይቅር
compose-drive-card-detail = { $size } · Google Drive
compose-onedrive-note = { $name } ከ{ $limit } ስለሚበልጥ ወደ OneDrive ይሄዳል፣ መልዕክቱም አገናኝ ይይዛል።
compose-onedrive-tip = በእርስዎ OneDrive ውስጥ፤ መልዕክቱ አገናኝ ይይዛል
compose-onedrive-allow = OneDriveን ፍቀድ
compose-onedrive-allow-tip = Katna ትላልቅ ፋይሎችን በOneDrive እንዲያስቀምጥ በMicrosoft እንደገና ይግቡ
compose-onedrive-not-uploaded = { $name } ገና በOneDrive ውስጥ የለም
compose-onedrive-share-failed = ፋይሎቹን በOneDrive ማጋራት አልተቻለም፦ { $error }
compose-onedrive-share-text = { $count ->
    [one] OneDrive ፋይሎቹን ከ{ $addresses } ጋር ማጋራት አይችልም። በምትኩ አገናኙ ያለው ማንኛውም ሰው ሊከፍታቸው ይችላል።
   *[other] OneDrive ፋይሎቹን ከ{ $addresses } ጋር ማጋራት አይችልም። በምትኩ አገናኙ ያለው ማንኛውም ሰው ሊከፍታቸው ይችላል።
}
compose-onedrive-card-detail = { $size } · OneDrive
compose-drop-files = ፋይሎችን እዚህ ይጣሉ
compose-drop-here = እዚህ ይጣሉ
compose-paste-keep-formatting = ቅርጸቱን አቆይ
compose-paste-table = ሰንጠረዥ
compose-paste-picture = ሥዕል
compose-paste-plain-text = ግልጽ ጽሑፍ
compose-paste-inline = በጽሑፉ ውስጥ
compose-paste-attachment = አባሪ

## Encryption and signing (the toggles by the recipients)

compose-encrypt = አመስጥር
compose-encrypted = ተመስጥሯል፦ ሊያነቡት የሚችሉት ተቀባዮቹ ብቻ ናቸው
compose-sign = ፈርም
compose-signed = ተፈርሟል፦ ተቀባዮች ከእርስዎ መሆኑን ማረጋገጥ ይችላሉ
compose-track = መከፈትን እና ጠቅታዎችን ተከታተል
compose-tracked = ክትትል ይደረግበታል፦ እያንዳንዱ ተቀባይ ሲከፍተው ወይም አገናኝ ሲከተል ያያሉ
compose-track-clicks = የአገናኝ ጠቅታዎችን ተከታተል (ግልጽ ጽሑፍ መከፈትን ማሳየት አይችልም)
compose-tracked-clicks = ክትትል ይደረግበታል፦ እያንዳንዱ ተቀባይ አገናኝ ሲከተል ያያሉ
compose-track-sign-in = መከፈትን እና ጠቅታዎችን ለመከታተል ወደ Katna መለያ ይግቡ
compose-receipt = የንባብ ማረጋገጫ ጠይቅ
compose-receipt-on = የንባብ ማረጋገጫ ተጠይቋል፦ የተቀባዩ መተግበሪያ እንዲልኩ ሊጠይቃቸው ይችላል
compose-delivery = የመድረስ ማረጋገጫ ጠይቅ
compose-delivery-on = የመድረስ ማረጋገጫ ተጠይቋል፦ የእያንዳንዱ ተቀባይ አገልጋይ ሲቀበለው የደብዳቤ አገልጋይዎ ኢሜይል ይልክልዎታል
compose-delivery-unavailable = የደብዳቤ አገልጋይዎ የመድረስ ማረጋገጫዎችን አይልክም

## Spelling

spell-no-dictionary = ለ{ $language } ምንም የፊደል አጻጻፍ መዝገበ ቃላት አልተጫነም (ለምሳሌ hunspell-en_us)።
spell-dictionary-error = የፊደል አጻጻፍ መዝገበ ቃላት፦ { $error }

## Grammar checking (the right-click menu on a grammar mistake)

grammar-replace = «{ $words }»
grammar-add = «{ $words }» አክል
grammar-remove = «{ $words }» አስወግድ
grammar-ignore = ችላ በል

## Send checks (asked before a message goes out)

send-check-attachment-title = ፋይሎችን ለማያያዝ አስበው ነበር?
send-check-attachment-text = ስለ አባሪ ጽፈዋል፣ ግን ምንም አልተያያዘም።
send-check-attach = ፋይል አያይዝ
send-check-subject-title = ያለ ርዕሰ ጉዳይ ይላክ?
send-check-subject-text = ይህ መልዕክት ርዕሰ ጉዳይ የለውም።
send-check-add-subject = ርዕሰ ጉዳይ አክል
send-check-send-anyway = ቢሆንም ላክ
recipient-not-valid = ትክክለኛ የኢሜይል አድራሻ አይደለም
recipient-show-address = አድራሻ አሳይ
recipient-remove = አስወግድ
recipient-bad-title = አድራሻውን ያረጋግጡ
recipient-bad-text = «{ $address }» ትክክለኛ የኢሜይል አድራሻ አይደለም። ከመላክዎ በፊት ያስተካክሉት ወይም ያስወግዱት።
recipient-bad-fix = አስተካክል
