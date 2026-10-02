# Katna Mail, Malayalam (മലയാളം).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = ഫയലുകൾ തിരയുക

## Left side (and chips on a phone)

files-all = എല്ലാ ഫയലുകളും
files-pictures = ചിത്രങ്ങൾ
files-pdfs = PDF-കൾ
files-documents = ഡോക്യുമെന്റുകൾ
files-sheets = സ്‌പ്രെഡ്‌ഷീറ്റുകൾ
files-slides = സ്ലൈഡുകൾ
files-other = മറ്റുള്ളവ
files-accounts = അക്കൗണ്ടുകൾ
files-drives = ഡ്രൈവുകൾ
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = എന്നുമായി പങ്കിട്ടവ
files-shown = കാണിക്കുന്നത്
files-received = ലഭിച്ചവ
files-sent = ഞാൻ അയച്ചവ

## Over the files

files-count = { $count ->
    [one] { $count } ഫയൽ · { $size }
   *[other] { $count } ഫയലുകൾ · { $size }
}
files-anyone = ആരിൽ നിന്നും
files-from-person = { $name }-ൽ നിന്ന്
files-time-any = ഏത് സമയവും
files-time-today = ഇന്ന്
files-time-yesterday = ഇന്നലെ
files-time-this-week = ഈ ആഴ്‌ച
files-time-last-week = കഴിഞ്ഞ ആഴ്‌ച
files-time-this-month = ഈ മാസം
files-time-last-month = കഴിഞ്ഞ മാസം
files-time-between = { $first } – { $last }
files-time-hint = ഒരു ദിവസം ക്ലിക്ക് ചെയ്യുക, അല്ലെങ്കിൽ ദിവസങ്ങളിലൂടെ വലിക്കുക
files-time-summary = { $count ->
    [one] { $days } · { $count } ഫയൽ
   *[other] { $days } · { $count } ഫയലുകൾ
}
files-time-clear = മായ്‌ക്കുക
files-time-month-back = മുമ്പത്തെ മാസം
files-time-month-on = അടുത്ത മാസം
files-time-wheel = ദൈർഘ്യം മാറ്റാതെ ഈ തീയതികൾ നീക്കാൻ സ്ക്രോൾ ചെയ്യുക
files-sort-newest = ഏറ്റവും പുതിയത് ആദ്യം
files-sort-oldest = ഏറ്റവും പഴയത് ആദ്യം
files-sort-largest = ഏറ്റവും വലുത് ആദ്യം
files-sort-name = പേര് അനുസരിച്ച്
files-grid = കാർഡുകൾ
files-list = ലിസ്റ്റ്
files-this-week = ഈ ആഴ്‌ച
files-undated = തീയതിയില്ല
files-me = ഞാൻ
files-no-subject = (വിഷയമില്ല)
files-loading = നിങ്ങളുടെ മെയിലിൽ നിന്ന് ഫയലുകൾ ശേഖരിക്കുന്നു…
files-empty = നിങ്ങളുടെ മെയിലിലെ ഫയലുകൾ ഇവിടെ കാണാം.
files-none-match = പൊരുത്തപ്പെടുന്ന ഫയലുകളൊന്നുമില്ല.
files-load-failed = ഫയലുകൾ വായിക്കാനായില്ല: { $error }

## A file's menu and buttons

files-open = തുറക്കുക
files-open-with = ഇതുപയോഗിച്ച് തുറക്കുക…
files-save = സംരക്ഷിക്കുക…
files-show-mail = മെയിൽ കാണിക്കുക
files-mail-window = മെയിൽ പുതിയ വിൻഡോയിൽ തുറക്കുക
files-forward = ഫയൽ ഫോർവേഡ് ചെയ്യുക
files-from-them = { $name } അയച്ച ഫയലുകൾ
files-copy-name = ഫയലിന്റെ പേര് പകർത്തുക
files-name-copied = ഫയലിന്റെ പേര് പകർത്തി
files-downloading = മെയിൽ ഡൗൺലോഡ് ചെയ്യുന്നു…
files-download-failed = ഈ മെയിൽ ഡൗൺലോഡ് ചെയ്യാനായില്ല.

## A cloud drive in place of the mail files

files-drive-mine = എന്റെ ഡ്രൈവ്
files-drive-mine-onedrive = എന്റെ ഫയലുകൾ
files-drive-results = “{ $words }”
files-drive-count = { $folders ->
    [0] { $files ->
        [one] 1 ഫയൽ
       *[other] { $files } ഫയലുകൾ
    }
    [one] 1 ഫോൾഡർ · { $files ->
        [one] 1 ഫയൽ
       *[other] { $files } ഫയലുകൾ
    }
   *[other] { $folders } ഫോൾഡറുകൾ · { $files ->
        [one] 1 ഫയൽ
       *[other] { $files } ഫയലുകൾ
    }
}
files-drive-folders = ഫോൾഡറുകൾ
files-drive-files = ഫയലുകൾ
files-drive-folder = ഫോൾഡർ
files-drive-meta = { $what } · { $date }-ന് എഡിറ്റ് ചെയ്‌തു
files-drive-as-link = { $what } · ലിങ്കായി
files-drive-google-doc = Google Doc
files-drive-google-sheet = Google Sheet
files-drive-google-slides = Google Slides
files-drive-google-drawing = Google Drawing
files-drive-fetching = എടുക്കുന്നു…
files-drive-loading = ഡ്രൈവ് തുറക്കുന്നു…
files-drive-empty = ഈ ഫോൾഡർ ശൂന്യമാണ്.
files-drive-unreachable = { $drive }-ൽ എത്താനാകുന്നില്ല.
files-drive-try-again = വീണ്ടും ശ്രമിക്കുക
files-drive-needs-permission = ഈ ഡ്രൈവ് കാണിക്കാൻ Katna-യ്ക്ക് ഒരിക്കൽ നിങ്ങളുടെ അനുമതി വേണം. വീണ്ടും സൈൻ ഇൻ ചെയ്‌ത് നിങ്ങളുടെ ഫയലുകൾ കാണാൻ Katna-യെ അനുവദിക്കുക.
files-drive-allow = അനുവദിക്കുക
files-drive-allow-failed = സൈൻ ഇൻ പൂർത്തിയായില്ല, അതിനാൽ ഡ്രൈവ് അടഞ്ഞുതന്നെ കിടക്കും.
files-drive-attach = അറ്റാച്ച് ചെയ്യുക
files-drive-more = കൂടുതൽ
files-drive-download = ഡൗൺലോഡ് ചെയ്യുക…
files-drive-open-web = { $drive }-ൽ തുറക്കുക
files-drive-copy-link = ലിങ്ക് പകർത്തുക
files-drive-link-copied = ലിങ്ക് പകർത്തി
files-drive-share = പങ്കിടുക…
files-drive-rename = പേരുമാറ്റുക
files-drive-trash = ട്രാഷിലേക്ക് നീക്കുക
files-drive-trashed = “{ $name }” { $drive } ട്രാഷിലാണ്
files-drive-renamed = “{ $name }” എന്ന് പേരുമാറ്റി
files-drive-getting = { $drive }-ൽ നിന്ന് { $name } എടുക്കുന്നു…
files-drive-get-failed = { $name } എടുക്കാനായില്ല: { $error }
files-drive-upload = അപ്‌ലോഡ് ചെയ്യുക
files-drive-upload-files = ഫയലുകൾ അപ്‌ലോഡ് ചെയ്യുക
files-drive-upload-folder = ഫോൾഡർ അപ്‌ലോഡ് ചെയ്യുക
files-drive-upload-failed = { $name } അപ്‌ലോഡ് ചെയ്യാനായില്ല: { $error }
files-drive-upload-needs = അപ്‌ലോഡ് ചെയ്യാൻ Katna-യ്ക്ക് ഒരിക്കൽ നിങ്ങളുടെ അനുമതി വേണം: ക്രമീകരണം › ഡിഫോൾട്ട് ആപ്പുകൾ › ഫയലുകൾ പേജ് എന്നതിൽ അനുവദിക്കുക അമർത്തുക.

## The Share dialog of a drive file or folder

files-share-title = “{ $name }” പങ്കിടുക
files-share-add = പേരോ വിലാസമോ ഉപയോഗിച്ച് ആളുകളെ ചേർക്കുക
files-share-not-address = “{ $text }” ഒരു ഇമെയിൽ വിലാസമല്ല
files-share-notify = { $drive } അവർക്കും ഇമെയിൽ അയയ്ക്കട്ടെ
files-share-people = ആക്‌സസ് ഉള്ള ആളുകൾ
files-share-general = പൊതുവായ ആക്‌സസ്
files-share-loading = ആർക്കൊക്കെ ആക്‌സസ് ഉണ്ടെന്ന് വായിക്കുന്നു…
files-share-restricted = നിയന്ത്രിതം
files-share-restricted-about = ആക്‌സസ് ഉള്ളവർക്ക് മാത്രമേ ലിങ്ക് ഉപയോഗിച്ച് ഇത് തുറക്കാനാകൂ
files-share-anyone = ലിങ്ക് ഉള്ള ആർക്കും
files-share-anyone-can = { $role ->
    [editor] ലിങ്ക് ഉള്ള ആർക്കും എഡിറ്റ് ചെയ്യാം
    [commenter] ലിങ്ക് ഉള്ള ആർക്കും അഭിപ്രായമിടാം
   *[viewer] ലിങ്ക് ഉള്ള ആർക്കും കാണാം
}
files-share-anyone-about = { $role ->
    [editor] ഇന്റർനെറ്റിൽ ലിങ്ക് ഉള്ള ആർക്കും എഡിറ്റ് ചെയ്യാം
    [commenter] ഇന്റർനെറ്റിൽ ലിങ്ക് ഉള്ള ആർക്കും അഭിപ്രായമിടാം
   *[viewer] ഇന്റർനെറ്റിൽ ലിങ്ക് ഉള്ള ആർക്കും കാണാം
}
files-share-role-owner = ഉടമ
files-share-role-editor = എഡിറ്റർ
files-share-role-commenter = അഭിപ്രായമിടുന്നയാൾ
files-share-role-viewer = കാഴ്‌ചക്കാരൻ
files-share-you = { $name } (നിങ്ങൾ)
files-share-domain = { $domain }-ലെ എല്ലാവരും
files-share-inherited = ഉൾപ്പെടുന്ന ഫോൾഡറിൽ നിന്നുള്ള ആക്‌സസ്
files-share-remove = ആക്‌സസ് നീക്കം ചെയ്യുക
files-share-copy-link = ലിങ്ക് പകർത്തുക
files-share-share = പങ്കിടുക
files-share-done = പൂർത്തിയായി
files-share-sharing = പങ്കിടുന്നു…
files-share-shared = { $count ->
    [one] 1 ആളുമായി പങ്കിട്ടു
   *[other] { $count } പേരുമായി പങ്കിട്ടു
}
files-share-refused = { $drive }-ന് { $addresses } എന്നിവരുമായി പങ്കിടാനായില്ല
files-share-failed = പങ്കിടൽ മാറ്റാനായില്ല: { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
    [one] 1 ഇനം അപ്‌ലോഡ് ചെയ്യുന്നു
   *[other] { $count } ഇനങ്ങൾ അപ്‌ലോഡ് ചെയ്യുന്നു
}
files-tray-done = { $count ->
    [one] 1 അപ്‌ലോഡ് പൂർത്തിയായി
   *[other] { $count } അപ്‌ലോഡുകൾ പൂർത്തിയായി
}
files-tray-some-failed = { $done } അപ്‌ലോഡ് ചെയ്‌തു, { $failed } പരാജയപ്പെട്ടു
files-tray-minutes-left = { $minutes ->
    [one] ഏകദേശം ഒരു മിനിറ്റ് ബാക്കി
   *[other] ഏകദേശം { $minutes } മിനിറ്റ് ബാക്കി
}
files-tray-seconds-left = ഒരു മിനിറ്റിൽ താഴെ ബാക്കി
files-tray-starting = ആരംഭിക്കുന്നു…
files-tray-cancel-all = എല്ലാം റദ്ദാക്കുക
files-tray-cancel = റദ്ദാക്കുക
files-tray-fold = ലിസ്റ്റ് മറയ്ക്കുക
files-tray-unfold = ലിസ്റ്റ് കാണിക്കുക
files-tray-close = അടയ്ക്കുക
files-tray-progress = { $place } · { $size }-ൽ { $sent }
files-tray-in = { $place }-ൽ
files-tray-cancelled = റദ്ദാക്കി
