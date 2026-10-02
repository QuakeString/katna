# Katna Mail, Amharic (አማርኛ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = ፋይሎችን ፈልግ

## Left side (and chips on a phone)

files-all = ሁሉም ፋይሎች
files-pictures = ሥዕሎች
files-pdfs = PDFዎች
files-documents = ሰነዶች
files-sheets = የተመን ሉሆች
files-slides = ስላይዶች
files-other = ሌላ
files-accounts = መለያዎች
files-drives = ድራይቮች
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = ከእኔ ጋር የተጋሩ
files-shown = የሚታዩ
files-received = የተቀበልኳቸው
files-sent = በእኔ የተላኩ

## Over the files

files-count = { $count ->
    [one] { $count } ፋይል · { $size }
   *[other] { $count } ፋይሎች · { $size }
}
files-anyone = ማንኛውም ሰው
files-from-person = ከ{ $name }
files-time-any = በማንኛውም ጊዜ
files-time-today = ዛሬ
files-time-yesterday = ትናንት
files-time-this-week = በዚህ ሳምንት
files-time-last-week = ባለፈው ሳምንት
files-time-this-month = በዚህ ወር
files-time-last-month = ባለፈው ወር
files-time-between = { $first } – { $last }
files-time-hint = አንድ ቀን ጠቅ ያድርጉ፣ ወይም በቀናት ላይ ይጎትቱ
files-time-summary = { $count ->
    [one] { $days } · { $count } ፋይል
   *[other] { $days } · { $count } ፋይሎች
}
files-time-clear = አጽዳ
files-time-month-back = ያለፈው ወር
files-time-month-on = ቀጣዩ ወር
files-time-wheel = ርዝመታቸውን ጠብቆ እነዚህን ቀኖች ለማንቀሳቀስ ያሸብልሉ
files-sort-newest = አዲሱ መጀመሪያ
files-sort-oldest = የቆየው መጀመሪያ
files-sort-largest = ትልቁ መጀመሪያ
files-sort-name = በስም
files-grid = ካርዶች
files-list = ዝርዝር
files-this-week = በዚህ ሳምንት
files-undated = ቀን የለውም
files-me = እኔ
files-no-subject = (ርዕሰ ጉዳይ የለም)
files-loading = ከደብዳቤዎ ፋይሎችን በመሰብሰብ ላይ…
files-empty = ከደብዳቤዎ የመጡ ፋይሎች እዚህ ይታያሉ።
files-none-match = የሚዛመድ ፋይል የለም።
files-load-failed = ፋይሎቹን ማንበብ አልተሳካም፦ { $error }

## A file's menu and buttons

files-open = ክፈት
files-open-with = በዚህ ክፈት…
files-save = አስቀምጥ…
files-show-mail = ደብዳቤውን አሳይ
files-mail-window = ደብዳቤውን በአዲስ መስኮት ክፈት
files-forward = ፋይሉን አስተላልፍ
files-from-them = ከ{ $name } የመጡ ፋይሎች
files-copy-name = የፋይል ስም ቅዳ
files-name-copied = የፋይል ስም ተቀድቷል
files-downloading = ደብዳቤውን በማውረድ ላይ…
files-download-failed = ይህን ደብዳቤ ማውረድ አልተቻለም።

## A cloud drive in place of the mail files

files-drive-mine = የእኔ ድራይቭ
files-drive-mine-onedrive = የእኔ ፋይሎች
files-drive-results = «{ $words }»
files-drive-count = { $folders ->
    [0] { $files ->
        [one] { $files } ፋይል
       *[other] { $files } ፋይሎች
    }
    [one] { $folders } አቃፊ · { $files ->
        [one] { $files } ፋይል
       *[other] { $files } ፋይሎች
    }
   *[other] { $folders } አቃፊዎች · { $files ->
        [one] { $files } ፋይል
       *[other] { $files } ፋይሎች
    }
}
files-drive-folders = አቃፊዎች
files-drive-files = ፋይሎች
files-drive-folder = አቃፊ
files-drive-meta = { $what } · የተስተካከለው { $date }
files-drive-as-link = { $what } · እንደ አገናኝ
files-drive-google-doc = Google Doc
files-drive-google-sheet = Google Sheet
files-drive-google-slides = Google Slides
files-drive-google-drawing = Google Drawing
files-drive-fetching = በማምጣት ላይ…
files-drive-loading = ድራይቩን በመክፈት ላይ…
files-drive-empty = ይህ አቃፊ ባዶ ነው።
files-drive-unreachable = { $drive }ን ማግኘት አልተቻለም።
files-drive-try-again = እንደገና ሞክር
files-drive-needs-permission = ይህን ድራይቭ ለማሳየት Katna አንድ ጊዜ የእርስዎን ፈቃድ ይፈልጋል። እንደገና ይግቡ እና Katna ፋይሎችዎን እንዲያይ ይፍቀዱ።
files-drive-allow = ፍቀድ
files-drive-allow-failed = መግቢያው ስላልተጠናቀቀ ድራይቩ እንደተዘጋ ይቆያል።
files-drive-attach = አያይዝ
files-drive-more = ተጨማሪ
files-drive-download = አውርድ…
files-drive-open-web = በ{ $drive } ውስጥ ክፈት
files-drive-copy-link = አገናኝ ቅዳ
files-drive-link-copied = አገናኙ ተቀድቷል
files-drive-share = አጋራ…
files-drive-rename = እንደገና ሰይም
files-drive-trash = ወደ መጣያ ውሰድ
files-drive-trashed = «{ $name }» በ{ $drive } መጣያ ውስጥ ነው
files-drive-renamed = ወደ «{ $name }» እንደገና ተሰይሟል
files-drive-getting = { $name }ን ከ{ $drive } በማምጣት ላይ…
files-drive-get-failed = { $name }ን ማምጣት አልተቻለም፦ { $error }
files-drive-upload = ስቀል
files-drive-upload-files = ፋይሎችን ስቀል
files-drive-upload-folder = አቃፊ ስቀል
files-drive-upload-failed = { $name }ን መስቀል አልተቻለም፦ { $error }
files-drive-upload-needs = ለመስቀል Katna አንድ ጊዜ የእርስዎን ፈቃድ ይፈልጋል፦ በቅንብሮች › ነባሪ መተግበሪያዎች › የፋይሎች ገጽ ውስጥ «ፍቀድ»ን ይጫኑ።

## The Share dialog of a drive file or folder

files-share-title = «{ $name }»ን አጋራ
files-share-add = ሰዎችን በስም ወይም በአድራሻ ያክሉ
files-share-not-address = «{ $text }» የኢሜይል አድራሻ አይደለም
files-share-notify = { $drive } ኢሜይልም ይላክላቸው
files-share-people = መዳረሻ ያላቸው ሰዎች
files-share-general = አጠቃላይ መዳረሻ
files-share-loading = መዳረሻ ያላቸውን በማንበብ ላይ…
files-share-restricted = የተገደበ
files-share-restricted-about = በአገናኙ ሊከፍቱት የሚችሉት መዳረሻ ያላቸው ሰዎች ብቻ ናቸው
files-share-anyone = አገናኙ ያለው ማንኛውም ሰው
files-share-anyone-can = { $role ->
    [editor] አገናኙ ያለው ማንኛውም ሰው ማርትዕ ይችላል
    [commenter] አገናኙ ያለው ማንኛውም ሰው አስተያየት መስጠት ይችላል
   *[viewer] አገናኙ ያለው ማንኛውም ሰው ማየት ይችላል
}
files-share-anyone-about = { $role ->
    [editor] በበይነመረብ ላይ አገናኙ ያለው ማንኛውም ሰው ማርትዕ ይችላል
    [commenter] በበይነመረብ ላይ አገናኙ ያለው ማንኛውም ሰው አስተያየት መስጠት ይችላል
   *[viewer] በበይነመረብ ላይ አገናኙ ያለው ማንኛውም ሰው ማየት ይችላል
}
files-share-role-owner = ባለቤት
files-share-role-editor = አርታዒ
files-share-role-commenter = አስተያየት ሰጪ
files-share-role-viewer = ተመልካች
files-share-you = { $name } (እርስዎ)
files-share-domain = በ{ $domain } ያሉ ሁሉም
files-share-inherited = ካለበት አቃፊ የመጣ መዳረሻ
files-share-remove = መዳረሻን አስወግድ
files-share-copy-link = አገናኝ ቅዳ
files-share-share = አጋራ
files-share-done = ተጠናቀቀ
files-share-sharing = በማጋራት ላይ…
files-share-shared = { $count ->
    [one] ከ{ $count } ሰው ጋር ተጋርቷል
   *[other] ከ{ $count } ሰዎች ጋር ተጋርቷል
}
files-share-refused = { $drive } ከ{ $addresses } ጋር ማጋራት አልቻለም
files-share-failed = ማጋራትን መቀየር አልተቻለም፦ { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
    [one] { $count } ንጥል በመስቀል ላይ
   *[other] { $count } ንጥሎችን በመስቀል ላይ
}
files-tray-done = { $count ->
    [one] { $count } መስቀል ተጠናቋል
   *[other] { $count } መስቀሎች ተጠናቀዋል
}
files-tray-some-failed = { $done } ተሰቅለዋል፣ { $failed } አልተሳኩም
files-tray-minutes-left = { $minutes ->
    [one] ወደ { $minutes } ደቂቃ ገደማ ቀርቷል
   *[other] ወደ { $minutes } ደቂቃዎች ገደማ ቀርተዋል
}
files-tray-seconds-left = ከአንድ ደቂቃ ያነሰ ቀርቷል
files-tray-starting = በመጀመር ላይ…
files-tray-cancel-all = ሁሉንም ይቅር
files-tray-cancel = ይቅር
files-tray-fold = ዝርዝሩን ደብቅ
files-tray-unfold = ዝርዝሩን አሳይ
files-tray-close = ዝጋ
files-tray-progress = { $place } · { $sent } ከ{ $size }
files-tray-in = በ{ $place } ውስጥ
files-tray-cancelled = ተሰርዟል
