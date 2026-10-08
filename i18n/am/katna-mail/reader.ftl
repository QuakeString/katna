# Katna Mail, Amharic (አማርኛ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = ዝጋ
reader-back = ተመለስ
reader-mark-unread = እንዳልተነበበ ምልክት አድርግ
reader-move-to = ውሰድ ወደ
reader-snooze = አሸልብ
reader-remind = አስታውሰኝ
reader-more = ተጨማሪ
reader-original-colors = የመጀመሪያዎቹን ቀለሞች አሳይ
reader-dark-colors = በጨለማ ቀለሞች አሳይ
reader-print-all = ሁሉንም አትም
reader-new-window = በአዲስ መስኮት
reader-position = { $position } ከ{ $total }
reader-newer = አዲስ
reader-older = የቆየ

## Reading pane: the conversation

reader-removed = ይህ ውይይት ተወግዷል።
reader-no-subject = (ርዕሰ ጉዳይ የለም)
reader-collapse-all = ሁሉንም ሰብስብ
reader-expand-all = ሁሉንም ዘርጋ
reader-unknown-sender = (ያልታወቀ ላኪ)
reader-date-ago = { $date } ({ $ago })
reader-sending = በመላክ ላይ…
reader-me = እኔ
reader-to = ለ{ $names }
reader-to-label = ወደ
reader-tick-delivered = ደርሷል { $when }
reader-tick-no-bounce = ተልኳል { $when }፤ የመላክ ስህተት መልስ አልተመለሰም፣ ስለዚህ ምናልባት ደርሷል
reader-tick-bounced = አልደረሰም፦ { $when } ተመልሷል
reader-tick-read = ተነቧል { $when } (የንባብ ማረጋገጫ)
reader-tick-opened = ተከፍቷል፣ መጨረሻ { $when } (የመከፈት ክትትል)
reader-starred = ኮከብ የተደረገበት
reader-chip-remove = { $label }ን አስወግድ
reader-not-starred = ኮከብ ያልተደረገበት
reader-too-long = መልዕክቱ ሙሉ በሙሉ ለማሳየት በጣም ረጅም ነው።
reader-encrypted-images = በተመሰጠረ ደብዳቤ ውስጥ ከድር የሚመጡ ምስሎች በጭራሽ አይጫኑም።
reader-window-failed = አዲስ መስኮት መክፈት አልተቻለም።

## Reading pane: message details (opened from "to me")

reader-details-from = ከ፦
reader-details-to = ለ፦
reader-details-cc = ግልባጭ፦
reader-details-date = ቀን፦
reader-details-subject = ርዕሰ ጉዳይ፦

## Reading pane: downloading a message

reader-downloading = ይህን መልዕክት ከአገልጋዩ በማውረድ ላይ…
reader-download-failed = ይህን መልዕክት ማውረድ አልተቻለም።
reader-download-failed-reason = ይህን መልዕክት ማውረድ አልተቻለም። { $reason }
reader-download-offline = ይህ መለያ ከመስመር ውጭ ነው። ይህን መልዕክት ለማውረድ መስመር ላይ ይሁኑ።
reader-try-again = እንደገና ሞክር

## Reply row

reply-reply = መልስ
reply-reply-all = ለሁሉም መልስ
reply-forward = አስተላልፍ

## Encrypted and signed mail

security-decrypting = ምስጠራውን በመፍታት ላይ…
security-checking = ፊርማውን በማረጋገጥ ላይ…
security-partly-encrypted = የዚህ መልዕክት ክፍል ብቻ ነው የተመሰጠረው። የተቀረው ከጥበቃው ውጭ የተጨመረ ሲሆን ከማንኛውም ሰው ሊመጣ ይችላል።
security-partly-signed = የዚህ መልዕክት ክፍል ብቻ ነው የተፈረመው። የተቀረው ከጥበቃው ውጭ የተጨመረ ሲሆን ከማንኛውም ሰው ሊመጣ ይችላል።
security-encrypted = የተመሰጠረ መልዕክት
security-encrypted-smime = የተመሰጠረ መልዕክት (S/MIME)
security-no-key = ይህን መልዕክት መፍታት አይቻልም፦ የተመሰጠረው እርስዎ በሌለዎት ቁልፍ ነው።
security-cancelled = መፍታቱ ተሰርዟል።
security-damaged = ይህን መልዕክት መፍታት አይቻልም፦ የተመሰጠረው ውሂብ ተበላሽቷል ወይም ተቀይሯል።
security-decrypt-unavailable = ይህን መልዕክት መፍታት አይቻልም፦ የተመሰጠረ ደብዳቤ ለማንበብ { $tool }ን ይጫኑ።
security-decrypt-failed = ይህን መልዕክት መፍታት አይቻልም፦ { $reason }
security-unknown-signer = ያልታወቀ ፈራሚ
security-signed-verified = በ{ $signer } የተፈረመ · የተረጋገጠ
security-signed-not-sender = ላኪው ባልሆነው በ{ $signer } የተፈረመ
security-signed-untrusted = እምነት የማይጣልበት ብለው ምልክት ባደረጉበት ቁልፍ በ{ $signer } የተፈረመ
security-signed-unverified = በ{ $signer } የተፈረመ · ቁልፉ አልተረጋገጠም
security-bad-signature = መጥፎ ፊርማ፦ ይህ መልዕክት ከተፈረመ በኋላ ተቀይሯል፣ ወይም ፊርማው የተጭበረበረ ነው።
security-signature-expired = በ{ $signer } የተፈረመ · የፊርማው ጊዜ አልፏል
security-key-expired = በ{ $signer } የተፈረመ · የቁልፉ ጊዜ ከዚያ ወዲህ አልፏል
security-key-revoked = በተሻረ ቁልፍ በ{ $signer } የተፈረመ
security-missing-key = በሌለዎት ቁልፍ የተፈረመ ስለሆነ ሊረጋገጥ አይችልም
security-missing-key-id = በሌለዎት ቁልፍ ({ $key }) የተፈረመ ስለሆነ ሊረጋገጥ አይችልም
security-signature-unavailable = የተፈረመ፤ ፊርማውን ለማረጋገጥ { $tool }ን ይጫኑ
security-signature-error = ፊርማው ሊረጋገጥ አልቻለም።
security-look-up-key = ቁልፉን ፈልግ

## The key popover: the details of the key a message was signed with, and
## a key to import (looked up, or attached to the message)

key-card-verified = የተረጋገጠ ፊርማ
key-card-verified-detail = ፊርማው ትክክል ነው፣ ይህን ቁልፍም ያምኑታል።
key-card-unverified = ፊርማው አልተረጋገጠም
key-card-unverified-detail = ፊርማው ትክክል ነው፣ ነገር ግን ቁልፉ የእነሱ መሆኑን የሚያረጋግጥ ነገር የለም። የጣት አሻራውን ከእነሱ ጋር ያወዳድሩ፣ ከዚያም ቁልፉን በGnuPG ውስጥ ይመኑት (Kleopatra ወይም gpg --edit-key)።
key-card-not-sender = በሌላ ሰው የተፈረመ
key-card-not-sender-detail = ፊርማው ትክክል ነው፣ ነገር ግን ቁልፉ የላኪው አይደለም።
key-card-untrusted = ቁልፉ የሚታመን አይደለም
key-card-untrusted-detail = ይህን ቁልፍ በGnuPG ውስጥ እምነት የማይጣልበት ብለው ምልክት አድርገውበታል።
key-card-signature-expired = የፊርማው ጊዜ አልፏል
key-card-signature-expired-detail = ፊርማው ትክክል ነበር፣ ነገር ግን ጊዜው አልፏል።
key-card-key-expired = የቁልፉ ጊዜ አልፏል
key-card-key-expired-detail = ፊርማው ትክክል ነው፣ ነገር ግን የቁልፉ ጊዜ ከዚያ ወዲህ አልፏል።
key-card-key-revoked = ቁልፉ ተሽሯል
key-card-key-revoked-detail = ባለቤቱ ይህን ቁልፍ ስለሻረው ፊርማው ሊታመን አይችልም።
key-card-bad = መጥፎ ፊርማ
key-card-bad-detail = ይህ መልዕክት ከተፈረመ በኋላ ተቀይሯል፣ ወይም ፊርማው የተጭበረበረ ነው።
key-card-signed-by = የፈረመው
key-card-belongs-to = ባለቤቱ
key-card-fingerprint = የጣት አሻራ
key-card-signed = የተፈረመበት
key-card-key = ቁልፍ
key-card-kind = { $standard }፣ { $algorithm }
key-card-created = የተፈጠረበት
key-card-expires = የሚያበቃበት
key-card-never = በጭራሽ
key-card-issued-by = የሰጠው
key-card-found-in = የተገኘበት
key-card-keyring = የእርስዎ GnuPG የቁልፍ ቀለበት
key-card-copy = የጣት አሻራውን ቅዳ
key-card-import-title = ይህን ቁልፍ ላስገባ?
key-card-from-directory = በ{ $domain } የቁልፍ ማውጫ ውስጥ ተገኝቷል።
key-card-from-attachment = ከአባሪው { $name }።
key-card-import-note = ከዚያ Katna የዚህን ሰው ፊርማዎች ማረጋገጥ እና ወደ እነሱ የሚላክ ደብዳቤ ማመስጠር ይችላል። ቁልፉን ሙሉ በሙሉ ለማመን የጣት አሻራውን ከእነሱ ጋር ያወዳድሩ።
key-card-cancel = ይቅር
key-card-import = ቁልፉን አስገባ
key-card-looking-up = ቁልፉን በመፈለግ ላይ…
key-card-looking-up-detail = የ{ $domain } የቁልፍ ማውጫን በመጠየቅ ላይ።
key-card-not-found = ምንም ቁልፍ አልተገኘም
key-card-not-found-detail = { $domain } ለዚህ አድራሻ ቁልፍ አያትምም። ላኪው የራሱን እንዲልክልዎ ይጠይቁ።
key-card-not-kept = የተገኘው ቁልፍ ጥቅም ላይ ሊውል አይችልም።
key-card-failed = ቁልፉን ማግኘት አልተቻለም
tracking-opened = { $who } { $count ->
    [one] አንድ ጊዜ
   *[other] { $count } ጊዜ
} ከፍቶታል፣ መጨረሻ { $when }
tracking-opens-clicks = { $who } { $opens ->
    [one] አንድ ጊዜ
   *[other] { $opens } ጊዜ
} ከፍቶታል፣ አገናኝም { $clicks ->
    [one] አንድ ጊዜ
   *[other] { $clicks } ጊዜ
} ተከትሏል፣ መጨረሻ { $when }
tracking-clicked = { $who } አገናኝ { $clicks ->
    [one] አንድ ጊዜ
   *[other] { $clicks } ጊዜ
} ተከትሏል፣ መጨረሻ { $when }
tracking-maybe-opened = { $who } ከፍቶት ሊሆን ይችላል (Apple Mail ለግላዊነት ሲባል ምስሎችን ይጭናል)
tracking-seen-none = እስካሁን ማንም አልከፈተውም ወይም አገናኝ አልተከተለም
tracking-receipt = { $who } የንባብ ማረጋገጫ ልኳል
tracking-receipt-read = { $who } አንብቦታል (የንባብ ማረጋገጫ)፣ { $when }
tracking-receipt-displayed = የንባብ ማረጋገጫ፦ { $who } መልዕክትዎን ከፍቷል
tracking-receipt-other = የንባብ ማረጋገጫ፦ { $who } መልዕክትዎን ሳይከፍት ሰርዞታል ወይም አስተናግዶታል

## Remote images and pictures

remote-hidden = በዚህ መልዕክት ውስጥ ያሉ ምስሎች ተደብቀዋል።
remote-hidden-unconfirmed = ምስሎች ተደብቀዋል፤ ላኪው ሊረጋገጥ አልቻለም።
remote-show = ምስሎችን አሳይ
remote-always-show = ከዚህ ላኪ ሁልጊዜ አሳይ
remote-picture-use = ተጠቀም
remote-picture-too-big = 8 MB ወይም ከዚያ ያነሰ ሥዕል ይምረጡ።
remote-picture-type = የPNG፣ JPEG፣ GIF፣ WebP ወይም SVG ሥዕል ይምረጡ።
remote-picture-read-failed = ሥዕሉን ማንበብ አይቻልም፦ { $error }
remote-picture-keep-failed = ሥዕሉን ማስቀመጥ አይቻልም፦ { $error }
remote-picture-remove-failed = ሥዕሉን ማስወገድ አይቻልም፦ { $error }

## Attachments

attachment-count = { $count ->
    [one] አንድ አባሪ
   *[other] { $count } አባሪዎች
}
attachment-save = አስቀምጥ
attachment-forward = አስተላልፍ
attachment-save-all = ሁሉንም አስቀምጥ
attachment-save-all-tooltip = እያንዳንዱን አባሪ ወደ አቃፊ አስቀምጥ
attachment-save-here = እዚህ አስቀምጥ
attachment-not-downloaded = ይህ መልዕክት አልወረደም።
attachment-not-found = ይህ አባሪ በመልዕክቱ ውስጥ ሊገኝ አልቻለም።
attachment-read-failed = { $name }ን ማንበብ አልተቻለም
attachment-numbered = አባሪ { $number }
attachment-saved-all = { $count ->
    [one] { $count } ፋይል ወደ { $place } ተቀምጧል
   *[other] { $count } ፋይሎች ወደ { $place } ተቀምጠዋል
}
attachment-saved-some = { $total ->
    [one] ከ{ $total } ፋይል { $saved } ወደ { $place } ተቀምጧል። { $failed }ን ማስቀመጥ አልተቻለም
   *[other] ከ{ $total } ፋይሎች { $saved } ወደ { $place } ተቀምጠዋል። { $failed }ን ማስቀመጥ አልተቻለም
}
attachment-saved-to = ወደ { $path } ተቀምጧል
attachment-save-failed = { $name }ን ማስቀመጥ አልተቻለም፦ { $error }
attachment-open-failed = { $name }ን መክፈት አልተቻለም፦ { $error }
attachment-risky = ይህ ፋይል ፕሮግራም ሊያሄድ ስለሚችል Katna አይከፍተውም። በምትኩ ያስቀምጡት።
attachment-encrypted-open = ይህ ፋይል ተመስጥሮ ነው የመጣው። ሌላ ቦታ ለመክፈት ያስቀምጡት።

## Printing

print-failed = ማተም አልተቻለም፦ { $error }
print-no-font = ምንም ቅርጸ-ቁምፊ አልተገኘም
print-opened-as-pdf = ከዚያ ለማተም እንደ PDF ተከፍቷል።
print-preview-title = የህትመት ቅድመ እይታ
print-preview-laying-out = ገጾቹን በማዘጋጀት ላይ…
print-preview-pages = { $count ->
    [one] { $count } ገጽ
   *[other] { $count } ገጾች
}
print-preview-more = { $count ->
    [one] እና ተጨማሪ { $count } ገጽ
   *[other] እና ተጨማሪ { $count } ገጾች
}
print-preview-failed = ገጾቹ ሊታዩ አልቻሉም
print-preview-paper = ወረቀት
print-preview-a4 = A4
print-preview-letter = US Letter
print-preview-layout = አቀማመጥ
print-preview-as-shown = እንደሚታየው
print-preview-simple = ተራ ጽሑፍ
print-preview-backgrounds = ዳራዎች
print-preview-cancel = ይቅር
print-preview-print = አትም
print-not-downloaded = (ገና አልወረደም።)
print-encrypted = (የተመሰጠረ። ጽሑፉን ለማተም በKatna Mail ውስጥ ይክፈቱት።)
print-to = ለ፦ { $addresses }
print-cc = ግልባጭ፦ { $addresses }

## Message text (right-click menu in the reading pane)

text-pin = ከላይ ሰካ
text-copy-address = አድራሻ ቅዳ

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = አባሪዎቹን ለማንበብ ይህን መልዕክት ይክፈቱ።
text-copy = ቅዳ
text-select-all = ሁሉንም ምረጥ
