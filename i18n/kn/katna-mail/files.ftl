# Katna Mail, Kannada (ಕನ್ನಡ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = ಫೈಲ್‌ಗಳನ್ನು ಹುಡುಕಿ

## Left side (and chips on a phone)

files-all = ಎಲ್ಲಾ ಫೈಲ್‌ಗಳು
files-pictures = ಚಿತ್ರಗಳು
files-pdfs = PDF ಗಳು
files-documents = ಡಾಕ್ಯುಮೆಂಟ್‌ಗಳು
files-sheets = ಸ್ಪ್ರೆಡ್‌ಶೀಟ್‌ಗಳು
files-slides = ಸ್ಲೈಡ್‌ಗಳು
files-other = ಇತರೆ
files-accounts = ಖಾತೆಗಳು
files-drives = ಡ್ರೈವ್‌ಗಳು
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = ನನ್ನೊಂದಿಗೆ ಹಂಚಿಕೊಂಡವು
files-shown = ತೋರಿಸುವುದು
files-received = ಸ್ವೀಕರಿಸಿದವು
files-sent = ನಾನು ಕಳುಹಿಸಿದವು

## Over the files

files-count = { $count ->
    [one] { $count } ಫೈಲ್ · { $size }
   *[other] { $count } ಫೈಲ್‌ಗಳು · { $size }
}
files-anyone = ಯಾರಾದರೂ
files-from-person = { $name } ಅವರಿಂದ
files-time-any = ಯಾವಾಗಲಾದರೂ
files-time-today = ಇಂದು
files-time-yesterday = ನಿನ್ನೆ
files-time-this-week = ಈ ವಾರ
files-time-last-week = ಕಳೆದ ವಾರ
files-time-this-month = ಈ ತಿಂಗಳು
files-time-last-month = ಕಳೆದ ತಿಂಗಳು
files-time-between = { $first } – { $last }
files-time-hint = ಒಂದು ದಿನದ ಮೇಲೆ ಕ್ಲಿಕ್ ಮಾಡಿ, ಅಥವಾ ದಿನಗಳ ಮೇಲೆ ಎಳೆಯಿರಿ
files-time-summary = { $count ->
    [one] { $days } · { $count } ಫೈಲ್
   *[other] { $days } · { $count } ಫೈಲ್‌ಗಳು
}
files-time-clear = ತೆರವುಗೊಳಿಸಿ
files-time-month-back = ಹಿಂದಿನ ತಿಂಗಳು
files-time-month-on = ಮುಂದಿನ ತಿಂಗಳು
files-time-wheel = ಅವಧಿಯನ್ನು ಹಾಗೆಯೇ ಇಟ್ಟು ಈ ದಿನಾಂಕಗಳನ್ನು ಸರಿಸಲು ಸ್ಕ್ರಾಲ್ ಮಾಡಿ
files-sort-newest = ಹೊಸದು ಮೊದಲು
files-sort-oldest = ಹಳೆಯದು ಮೊದಲು
files-sort-largest = ದೊಡ್ಡದು ಮೊದಲು
files-sort-name = ಹೆಸರಿನ ಪ್ರಕಾರ
files-grid = ಕಾರ್ಡ್‌ಗಳು
files-list = ಪಟ್ಟಿ
files-this-week = ಈ ವಾರ
files-undated = ದಿನಾಂಕವಿಲ್ಲ
files-me = ನಾನು
files-no-subject = (ವಿಷಯವಿಲ್ಲ)
files-loading = ನಿಮ್ಮ ಮೇಲ್‌ನಿಂದ ಫೈಲ್‌ಗಳನ್ನು ಸಂಗ್ರಹಿಸಲಾಗುತ್ತಿದೆ…
files-empty = ನಿಮ್ಮ ಮೇಲ್‌ನ ಫೈಲ್‌ಗಳು ಇಲ್ಲಿ ಕಾಣಿಸುತ್ತವೆ.
files-none-match = ಯಾವುದೇ ಫೈಲ್ ಹೊಂದಿಕೆಯಾಗುತ್ತಿಲ್ಲ.
files-load-failed = ಫೈಲ್‌ಗಳನ್ನು ಓದಲು ವಿಫಲವಾಯಿತು: { $error }

## A file's menu and buttons

files-open = ತೆರೆಯಿರಿ
files-open-with = ಇದರೊಂದಿಗೆ ತೆರೆಯಿರಿ…
files-save = ಉಳಿಸಿ…
files-show-mail = ಮೇಲ್ ತೋರಿಸಿ
files-mail-window = ಮೇಲ್ ಅನ್ನು ಹೊಸ ವಿಂಡೋದಲ್ಲಿ ತೆರೆಯಿರಿ
files-forward = ಫೈಲ್ ಫಾರ್ವರ್ಡ್ ಮಾಡಿ
files-from-them = { $name } ಅವರಿಂದ ಫೈಲ್‌ಗಳು
files-copy-name = ಫೈಲ್ ಹೆಸರು ನಕಲಿಸಿ
files-name-copied = ಫೈಲ್ ಹೆಸರು ನಕಲಿಸಲಾಗಿದೆ
files-downloading = ಮೇಲ್ ಡೌನ್‌ಲೋಡ್ ಮಾಡಲಾಗುತ್ತಿದೆ…
files-download-failed = ಈ ಮೇಲ್ ಡೌನ್‌ಲೋಡ್ ಮಾಡಲು ಸಾಧ್ಯವಾಗಲಿಲ್ಲ.

## A cloud drive in place of the mail files

files-drive-mine = ನನ್ನ ಡ್ರೈವ್
files-drive-mine-onedrive = ನನ್ನ ಫೈಲ್‌ಗಳು
files-drive-results = “{ $words }”
files-drive-count = { $folders ->
    [0] { $files ->
        [one] { $files } ಫೈಲ್
       *[other] { $files } ಫೈಲ್‌ಗಳು
    }
    [one] { $folders } ಫೋಲ್ಡರ್ · { $files ->
        [one] { $files } ಫೈಲ್
       *[other] { $files } ಫೈಲ್‌ಗಳು
    }
   *[other] { $folders } ಫೋಲ್ಡರ್‌ಗಳು · { $files ->
        [one] { $files } ಫೈಲ್
       *[other] { $files } ಫೈಲ್‌ಗಳು
    }
}
files-drive-folders = ಫೋಲ್ಡರ್‌ಗಳು
files-drive-files = ಫೈಲ್‌ಗಳು
files-drive-folder = ಫೋಲ್ಡರ್
files-drive-meta = { $what } · { $date } ರಂದು ಎಡಿಟ್ ಮಾಡಲಾಗಿದೆ
files-drive-as-link = { $what } · ಲಿಂಕ್ ಆಗಿ
files-drive-google-doc = Google Doc
files-drive-google-sheet = Google Sheet
files-drive-google-slides = Google Slides
files-drive-google-drawing = Google Drawing
files-drive-fetching = ಪಡೆಯಲಾಗುತ್ತಿದೆ…
files-drive-loading = ಡ್ರೈವ್ ತೆರೆಯಲಾಗುತ್ತಿದೆ…
files-drive-empty = ಈ ಫೋಲ್ಡರ್ ಖಾಲಿಯಾಗಿದೆ.
files-drive-unreachable = { $drive } ಅನ್ನು ತಲುಪಲು ಸಾಧ್ಯವಾಗುತ್ತಿಲ್ಲ.
files-drive-try-again = ಮತ್ತೆ ಪ್ರಯತ್ನಿಸಿ
files-drive-needs-permission = ಈ ಡ್ರೈವ್ ತೋರಿಸಲು Katna ಗೆ ಒಮ್ಮೆ ನಿಮ್ಮ ಅನುಮತಿ ಬೇಕು. ಮತ್ತೆ ಸೈನ್ ಇನ್ ಮಾಡಿ ಮತ್ತು ನಿಮ್ಮ ಫೈಲ್‌ಗಳನ್ನು ನೋಡಲು Katna ಗೆ ಅನುಮತಿಸಿ.
files-drive-allow = ಅನುಮತಿಸಿ
files-drive-allow-failed = ಸೈನ್ ಇನ್ ಪೂರ್ಣಗೊಳ್ಳಲಿಲ್ಲ, ಆದ್ದರಿಂದ ಡ್ರೈವ್ ಮುಚ್ಚಿಯೇ ಇರುತ್ತದೆ.
files-drive-attach = ಲಗತ್ತಿಸಿ
files-drive-more = ಇನ್ನಷ್ಟು
files-drive-download = ಡೌನ್‌ಲೋಡ್ ಮಾಡಿ…
files-drive-open-web = { $drive } ನಲ್ಲಿ ತೆರೆಯಿರಿ
files-drive-copy-link = ಲಿಂಕ್ ನಕಲಿಸಿ
files-drive-link-copied = ಲಿಂಕ್ ನಕಲಿಸಲಾಗಿದೆ
files-drive-share = ಹಂಚಿಕೊಳ್ಳಿ…
files-drive-rename = ಮರುಹೆಸರಿಸಿ
files-drive-trash = ಅನುಪಯುಕ್ತಕ್ಕೆ ಸರಿಸಿ
files-drive-trashed = “{ $name }” { $drive } ಅನುಪಯುಕ್ತದಲ್ಲಿದೆ
files-drive-renamed = “{ $name }” ಎಂದು ಮರುಹೆಸರಿಸಲಾಗಿದೆ
files-drive-getting = { $drive } ನಿಂದ { $name } ಪಡೆಯಲಾಗುತ್ತಿದೆ…
files-drive-get-failed = { $name } ಪಡೆಯಲಾಗಲಿಲ್ಲ: { $error }
files-drive-upload = ಅಪ್‌ಲೋಡ್ ಮಾಡಿ
files-drive-upload-files = ಫೈಲ್‌ಗಳನ್ನು ಅಪ್‌ಲೋಡ್ ಮಾಡಿ
files-drive-upload-folder = ಫೋಲ್ಡರ್ ಅಪ್‌ಲೋಡ್ ಮಾಡಿ
files-drive-upload-failed = { $name } ಅಪ್‌ಲೋಡ್ ಮಾಡಲಾಗಲಿಲ್ಲ: { $error }
files-drive-upload-needs = ಅಪ್‌ಲೋಡ್ ಮಾಡಲು Katna ಗೆ ಒಮ್ಮೆ ನಿಮ್ಮ ಅನುಮತಿ ಬೇಕು: ಸೆಟ್ಟಿಂಗ್‌ಗಳು › ಡೀಫಾಲ್ಟ್ ಆ್ಯಪ್‌ಗಳು › ಫೈಲ್‌ಗಳ ಪುಟ ಎಂಬಲ್ಲಿ ಅನುಮತಿಸಿ ಒತ್ತಿರಿ.

## The Share dialog of a drive file or folder

files-share-title = “{ $name }” ಹಂಚಿಕೊಳ್ಳಿ
files-share-add = ಹೆಸರು ಅಥವಾ ವಿಳಾಸದಿಂದ ಜನರನ್ನು ಸೇರಿಸಿ
files-share-not-address = “{ $text }” ಇಮೇಲ್ ವಿಳಾಸವಲ್ಲ
files-share-notify = { $drive } ಅವರಿಗೆ ಇಮೇಲ್ ಕೂಡ ಕಳುಹಿಸಲಿ
files-share-people = ಪ್ರವೇಶ ಇರುವ ಜನರು
files-share-general = ಸಾಮಾನ್ಯ ಪ್ರವೇಶ
files-share-loading = ಯಾರಿಗೆ ಪ್ರವೇಶ ಇದೆ ಎಂದು ಓದಲಾಗುತ್ತಿದೆ…
files-share-restricted = ನಿರ್ಬಂಧಿತ
files-share-restricted-about = ಪ್ರವೇಶ ಇರುವ ಜನರು ಮಾತ್ರ ಲಿಂಕ್ ಮೂಲಕ ಇದನ್ನು ತೆರೆಯಬಹುದು
files-share-anyone = ಲಿಂಕ್ ಇರುವ ಯಾರಾದರೂ
files-share-anyone-can = { $role ->
    [editor] ಲಿಂಕ್ ಇರುವ ಯಾರಾದರೂ ಎಡಿಟ್ ಮಾಡಬಹುದು
    [commenter] ಲಿಂಕ್ ಇರುವ ಯಾರಾದರೂ ಕಾಮೆಂಟ್ ಮಾಡಬಹುದು
   *[viewer] ಲಿಂಕ್ ಇರುವ ಯಾರಾದರೂ ವೀಕ್ಷಿಸಬಹುದು
}
files-share-anyone-about = { $role ->
    [editor] ಇಂಟರ್ನೆಟ್‌ನಲ್ಲಿ ಲಿಂಕ್ ಇರುವ ಯಾರಾದರೂ ಎಡಿಟ್ ಮಾಡಬಹುದು
    [commenter] ಇಂಟರ್ನೆಟ್‌ನಲ್ಲಿ ಲಿಂಕ್ ಇರುವ ಯಾರಾದರೂ ಕಾಮೆಂಟ್ ಮಾಡಬಹುದು
   *[viewer] ಇಂಟರ್ನೆಟ್‌ನಲ್ಲಿ ಲಿಂಕ್ ಇರುವ ಯಾರಾದರೂ ವೀಕ್ಷಿಸಬಹುದು
}
files-share-role-owner = ಮಾಲೀಕರು
files-share-role-editor = ಎಡಿಟರ್
files-share-role-commenter = ಕಾಮೆಂಟ್ ಮಾಡುವವರು
files-share-role-viewer = ವೀಕ್ಷಕರು
files-share-you = { $name } (ನೀವು)
files-share-domain = { $domain } ನಲ್ಲಿರುವ ಎಲ್ಲರೂ
files-share-inherited = ಇದು ಇರುವ ಫೋಲ್ಡರ್‌ನಿಂದ ಪ್ರವೇಶ
files-share-remove = ಪ್ರವೇಶ ತೆಗೆದುಹಾಕಿ
files-share-copy-link = ಲಿಂಕ್ ನಕಲಿಸಿ
files-share-share = ಹಂಚಿಕೊಳ್ಳಿ
files-share-done = ಮುಗಿದಿದೆ
files-share-sharing = ಹಂಚಿಕೊಳ್ಳಲಾಗುತ್ತಿದೆ…
files-share-shared = { $count ->
    [one] { $count } ವ್ಯಕ್ತಿಯೊಂದಿಗೆ ಹಂಚಿಕೊಳ್ಳಲಾಗಿದೆ
   *[other] { $count } ಜನರೊಂದಿಗೆ ಹಂಚಿಕೊಳ್ಳಲಾಗಿದೆ
}
files-share-refused = { $drive } ಗೆ { $addresses } ಜೊತೆ ಹಂಚಿಕೊಳ್ಳಲು ಸಾಧ್ಯವಾಗಲಿಲ್ಲ
files-share-failed = ಹಂಚಿಕೆಯನ್ನು ಬದಲಾಯಿಸಲಾಗಲಿಲ್ಲ: { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
    [one] { $count } ಐಟಂ ಅಪ್‌ಲೋಡ್ ಆಗುತ್ತಿದೆ
   *[other] { $count } ಐಟಂಗಳು ಅಪ್‌ಲೋಡ್ ಆಗುತ್ತಿವೆ
}
files-tray-done = { $count ->
    [one] { $count } ಅಪ್‌ಲೋಡ್ ಮುಗಿದಿದೆ
   *[other] { $count } ಅಪ್‌ಲೋಡ್‌ಗಳು ಮುಗಿದಿವೆ
}
files-tray-some-failed = { $done } ಅಪ್‌ಲೋಡ್ ಆಗಿವೆ, { $failed } ವಿಫಲವಾಗಿವೆ
files-tray-minutes-left = { $minutes ->
    [one] ಸುಮಾರು { $minutes } ನಿಮಿಷ ಉಳಿದಿದೆ
   *[other] ಸುಮಾರು { $minutes } ನಿಮಿಷಗಳು ಉಳಿದಿವೆ
}
files-tray-seconds-left = ಒಂದು ನಿಮಿಷಕ್ಕಿಂತ ಕಡಿಮೆ ಉಳಿದಿದೆ
files-tray-starting = ಆರಂಭಿಸಲಾಗುತ್ತಿದೆ…
files-tray-cancel-all = ಎಲ್ಲವನ್ನೂ ರದ್ದುಮಾಡಿ
files-tray-cancel = ರದ್ದುಮಾಡಿ
files-tray-fold = ಪಟ್ಟಿ ಮರೆಮಾಡಿ
files-tray-unfold = ಪಟ್ಟಿ ತೋರಿಸಿ
files-tray-close = ಮುಚ್ಚಿ
files-tray-progress = { $place } · { $size } ರಲ್ಲಿ { $sent }
files-tray-in = { $place } ನಲ್ಲಿ
files-tray-cancelled = ರದ್ದುಮಾಡಲಾಗಿದೆ
