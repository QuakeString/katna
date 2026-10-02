# Katna Mail, Punjabi (ਪੰਜਾਬੀ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = ਫ਼ਾਈਲਾਂ ਖੋਜੋ

## Left side (and chips on a phone)

files-all = ਸਾਰੀਆਂ ਫ਼ਾਈਲਾਂ
files-pictures = ਤਸਵੀਰਾਂ
files-pdfs = PDF
files-documents = ਦਸਤਾਵੇਜ਼
files-sheets = ਸਪ੍ਰੈਡਸ਼ੀਟਾਂ
files-slides = ਸਲਾਈਡਾਂ
files-other = ਹੋਰ
files-accounts = ਖਾਤੇ
files-drives = ਡਰਾਈਵਾਂ
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = ਮੇਰੇ ਨਾਲ ਸਾਂਝੀਆਂ
files-shown = ਦਿਖਾਈਆਂ
files-received = ਪ੍ਰਾਪਤ
files-sent = ਮੇਰੇ ਵੱਲੋਂ ਭੇਜੀਆਂ

## Over the files

files-count = { $count ->
    [one] { $count } ਫ਼ਾਈਲ · { $size }
   *[other] { $count } ਫ਼ਾਈਲਾਂ · { $size }
}
files-anyone = ਕੋਈ ਵੀ
files-from-person = { $name } ਵੱਲੋਂ
files-time-any = ਕਿਸੇ ਵੀ ਸਮੇਂ
files-time-today = ਅੱਜ
files-time-yesterday = ਕੱਲ੍ਹ
files-time-this-week = ਇਸ ਹਫ਼ਤੇ
files-time-last-week = ਪਿਛਲੇ ਹਫ਼ਤੇ
files-time-this-month = ਇਸ ਮਹੀਨੇ
files-time-last-month = ਪਿਛਲੇ ਮਹੀਨੇ
files-time-between = { $first } – { $last }
files-time-hint = ਕਿਸੇ ਦਿਨ ’ਤੇ ਕਲਿੱਕ ਕਰੋ, ਜਾਂ ਦਿਨਾਂ ’ਤੇ ਖਿੱਚੋ
files-time-summary = { $count ->
    [one] { $days } · { $count } ਫ਼ਾਈਲ
   *[other] { $days } · { $count } ਫ਼ਾਈਲਾਂ
}
files-time-clear = ਸਾਫ਼ ਕਰੋ
files-time-month-back = ਪਿਛਲਾ ਮਹੀਨਾ
files-time-month-on = ਅਗਲਾ ਮਹੀਨਾ
files-time-wheel = ਇਹਨਾਂ ਤਾਰੀਖਾਂ ਨੂੰ ਉਹੀ ਲੰਬਾਈ ਰੱਖਦੇ ਹੋਏ ਹਿਲਾਉਣ ਲਈ ਸਕ੍ਰੌਲ ਕਰੋ
files-sort-newest = ਸਭ ਤੋਂ ਨਵੀਆਂ ਪਹਿਲਾਂ
files-sort-oldest = ਸਭ ਤੋਂ ਪੁਰਾਣੀਆਂ ਪਹਿਲਾਂ
files-sort-largest = ਸਭ ਤੋਂ ਵੱਡੀਆਂ ਪਹਿਲਾਂ
files-sort-name = ਨਾਮ ਮੁਤਾਬਕ
files-grid = ਕਾਰਡ
files-list = ਸੂਚੀ
files-this-week = ਇਸ ਹਫ਼ਤੇ
files-undated = ਕੋਈ ਤਾਰੀਖ ਨਹੀਂ
files-me = ਮੈਂ
files-no-subject = (ਕੋਈ ਵਿਸ਼ਾ ਨਹੀਂ)
files-loading = ਤੁਹਾਡੀ ਮੇਲ ਤੋਂ ਫ਼ਾਈਲਾਂ ਇਕੱਠੀਆਂ ਕੀਤੀਆਂ ਜਾ ਰਹੀਆਂ ਹਨ…
files-empty = ਤੁਹਾਡੀ ਮੇਲ ਦੀਆਂ ਫ਼ਾਈਲਾਂ ਇੱਥੇ ਦਿਖਾਈ ਦਿੰਦੀਆਂ ਹਨ।
files-none-match = ਕੋਈ ਫ਼ਾਈਲ ਮੇਲ ਨਹੀਂ ਖਾਂਦੀ।
files-load-failed = ਫ਼ਾਈਲਾਂ ਪੜ੍ਹਨਾ ਅਸਫਲ ਰਿਹਾ: { $error }

## A file's menu and buttons

files-open = ਖੋਲ੍ਹੋ
files-open-with = ਇਸ ਨਾਲ ਖੋਲ੍ਹੋ…
files-save = ਰੱਖਿਅਤ ਕਰੋ…
files-show-mail = ਮੇਲ ਦਿਖਾਓ
files-mail-window = ਮੇਲ ਨਵੀਂ ਵਿੰਡੋ ਵਿੱਚ ਖੋਲ੍ਹੋ
files-forward = ਫ਼ਾਈਲ ਅੱਗੇ ਭੇਜੋ
files-from-them = { $name } ਦੀਆਂ ਫ਼ਾਈਲਾਂ
files-copy-name = ਫ਼ਾਈਲ ਦਾ ਨਾਮ ਕਾਪੀ ਕਰੋ
files-name-copied = ਫ਼ਾਈਲ ਦਾ ਨਾਮ ਕਾਪੀ ਕੀਤਾ ਗਿਆ
files-downloading = ਮੇਲ ਡਾਊਨਲੋਡ ਹੋ ਰਹੀ ਹੈ…
files-download-failed = ਇਹ ਮੇਲ ਡਾਊਨਲੋਡ ਨਹੀਂ ਹੋ ਸਕੀ।

## A cloud drive in place of the mail files

files-drive-mine = ਮੇਰੀ ਡਰਾਈਵ
files-drive-mine-onedrive = ਮੇਰੀਆਂ ਫ਼ਾਈਲਾਂ
files-drive-results = “{ $words }”
files-drive-count = { $folders ->
    [0] { $files ->
        [one] 1 ਫ਼ਾਈਲ
       *[other] { $files } ਫ਼ਾਈਲਾਂ
    }
    [one] 1 ਫੋਲਡਰ · { $files ->
        [one] 1 ਫ਼ਾਈਲ
       *[other] { $files } ਫ਼ਾਈਲਾਂ
    }
   *[other] { $folders } ਫੋਲਡਰ · { $files ->
        [one] 1 ਫ਼ਾਈਲ
       *[other] { $files } ਫ਼ਾਈਲਾਂ
    }
}
files-drive-folders = ਫੋਲਡਰ
files-drive-files = ਫ਼ਾਈਲਾਂ
files-drive-folder = ਫੋਲਡਰ
files-drive-meta = { $what } · { $date } ਨੂੰ ਸੋਧਿਆ
files-drive-as-link = { $what } · ਲਿੰਕ ਵਜੋਂ
files-drive-google-doc = Google Doc
files-drive-google-sheet = Google Sheet
files-drive-google-slides = Google Slides
files-drive-google-drawing = Google Drawing
files-drive-fetching = ਲਿਆਂਦੀ ਜਾ ਰਹੀ ਹੈ…
files-drive-loading = ਡਰਾਈਵ ਖੁੱਲ੍ਹ ਰਹੀ ਹੈ…
files-drive-empty = ਇਹ ਫੋਲਡਰ ਖਾਲੀ ਹੈ।
files-drive-unreachable = { $drive } ਤੱਕ ਪਹੁੰਚ ਨਹੀਂ ਹੋ ਸਕੀ।
files-drive-try-again = ਦੁਬਾਰਾ ਕੋਸ਼ਿਸ਼ ਕਰੋ
files-drive-needs-permission = ਇਹ ਡਰਾਈਵ ਦਿਖਾਉਣ ਲਈ Katna ਨੂੰ ਇੱਕ ਵਾਰ ਤੁਹਾਡੀ ਇਜਾਜ਼ਤ ਚਾਹੀਦੀ ਹੈ। ਦੁਬਾਰਾ ਸਾਈਨ ਇਨ ਕਰੋ ਅਤੇ Katna ਨੂੰ ਆਪਣੀਆਂ ਫ਼ਾਈਲਾਂ ਦੇਖਣ ਦਿਓ।
files-drive-allow = ਇਜਾਜ਼ਤ ਦਿਓ
files-drive-allow-failed = ਸਾਈਨ ਇਨ ਪੂਰਾ ਨਹੀਂ ਹੋਇਆ, ਇਸ ਲਈ ਡਰਾਈਵ ਬੰਦ ਰਹਿੰਦੀ ਹੈ।
files-drive-attach = ਨੱਥੀ ਕਰੋ
files-drive-more = ਹੋਰ
files-drive-download = ਡਾਊਨਲੋਡ ਕਰੋ…
files-drive-open-web = { $drive } ਵਿੱਚ ਖੋਲ੍ਹੋ
files-drive-copy-link = ਲਿੰਕ ਕਾਪੀ ਕਰੋ
files-drive-link-copied = ਲਿੰਕ ਕਾਪੀ ਕੀਤਾ ਗਿਆ
files-drive-share = ਸਾਂਝਾ ਕਰੋ…
files-drive-rename = ਨਾਮ ਬਦਲੋ
files-drive-trash = ਰੱਦੀ ਵਿੱਚ ਭੇਜੋ
files-drive-trashed = “{ $name }” { $drive } ਦੀ ਰੱਦੀ ਵਿੱਚ ਹੈ
files-drive-renamed = ਨਾਮ ਬਦਲ ਕੇ “{ $name }” ਕੀਤਾ ਗਿਆ
files-drive-getting = { $drive } ਤੋਂ { $name } ਲਿਆਂਦੀ ਜਾ ਰਹੀ ਹੈ…
files-drive-get-failed = { $name } ਨਹੀਂ ਲਿਆਂਦੀ ਜਾ ਸਕੀ: { $error }
files-drive-upload = ਅੱਪਲੋਡ ਕਰੋ
files-drive-upload-files = ਫ਼ਾਈਲਾਂ ਅੱਪਲੋਡ ਕਰੋ
files-drive-upload-folder = ਫੋਲਡਰ ਅੱਪਲੋਡ ਕਰੋ
files-drive-upload-failed = { $name } ਅੱਪਲੋਡ ਨਹੀਂ ਹੋ ਸਕੀ: { $error }
files-drive-upload-needs = ਅੱਪਲੋਡ ਕਰਨ ਲਈ Katna ਨੂੰ ਇੱਕ ਵਾਰ ਤੁਹਾਡੀ ਇਜਾਜ਼ਤ ਚਾਹੀਦੀ ਹੈ: ਸੈਟਿੰਗਾਂ › ਪੂਰਵ-ਨਿਰਧਾਰਿਤ ਐਪਾਂ › ਫ਼ਾਈਲਾਂ ਪੰਨਾ ਵਿੱਚ ਇਜਾਜ਼ਤ ਦਿਓ ਦਬਾਓ।

## The Share dialog of a drive file or folder

files-share-title = “{ $name }” ਸਾਂਝਾ ਕਰੋ
files-share-add = ਨਾਮ ਜਾਂ ਪਤੇ ਨਾਲ ਲੋਕ ਸ਼ਾਮਲ ਕਰੋ
files-share-not-address = “{ $text }” ਈਮੇਲ ਪਤਾ ਨਹੀਂ ਹੈ
files-share-notify = { $drive } ਨੂੰ ਉਨ੍ਹਾਂ ਨੂੰ ਈਮੇਲ ਵੀ ਭੇਜਣ ਦਿਓ
files-share-people = ਪਹੁੰਚ ਵਾਲੇ ਲੋਕ
files-share-general = ਆਮ ਪਹੁੰਚ
files-share-loading = ਪੜ੍ਹਿਆ ਜਾ ਰਿਹਾ ਹੈ ਕਿ ਕਿਸ ਕੋਲ ਪਹੁੰਚ ਹੈ…
files-share-restricted = ਸੀਮਤ
files-share-restricted-about = ਸਿਰਫ਼ ਪਹੁੰਚ ਵਾਲੇ ਲੋਕ ਹੀ ਇਸਨੂੰ ਲਿੰਕ ਨਾਲ ਖੋਲ੍ਹ ਸਕਦੇ ਹਨ
files-share-anyone = ਲਿੰਕ ਵਾਲਾ ਕੋਈ ਵੀ
files-share-anyone-can = { $role ->
    [editor] ਲਿੰਕ ਵਾਲਾ ਕੋਈ ਵੀ ਸੋਧ ਸਕਦਾ ਹੈ
    [commenter] ਲਿੰਕ ਵਾਲਾ ਕੋਈ ਵੀ ਟਿੱਪਣੀ ਕਰ ਸਕਦਾ ਹੈ
   *[viewer] ਲਿੰਕ ਵਾਲਾ ਕੋਈ ਵੀ ਦੇਖ ਸਕਦਾ ਹੈ
}
files-share-anyone-about = { $role ->
    [editor] ਇੰਟਰਨੈੱਟ ’ਤੇ ਲਿੰਕ ਵਾਲਾ ਕੋਈ ਵੀ ਸੋਧ ਸਕਦਾ ਹੈ
    [commenter] ਇੰਟਰਨੈੱਟ ’ਤੇ ਲਿੰਕ ਵਾਲਾ ਕੋਈ ਵੀ ਟਿੱਪਣੀ ਕਰ ਸਕਦਾ ਹੈ
   *[viewer] ਇੰਟਰਨੈੱਟ ’ਤੇ ਲਿੰਕ ਵਾਲਾ ਕੋਈ ਵੀ ਦੇਖ ਸਕਦਾ ਹੈ
}
files-share-role-owner = ਮਾਲਕ
files-share-role-editor = ਸੰਪਾਦਕ
files-share-role-commenter = ਟਿੱਪਣੀਕਾਰ
files-share-role-viewer = ਦਰਸ਼ਕ
files-share-you = { $name } (ਤੁਸੀਂ)
files-share-domain = { $domain } ਦੇ ਸਾਰੇ ਲੋਕ
files-share-inherited = ਜਿਸ ਫੋਲਡਰ ਵਿੱਚ ਇਹ ਹੈ ਉਸ ਤੋਂ ਪਹੁੰਚ
files-share-remove = ਪਹੁੰਚ ਹਟਾਓ
files-share-copy-link = ਲਿੰਕ ਕਾਪੀ ਕਰੋ
files-share-share = ਸਾਂਝਾ ਕਰੋ
files-share-done = ਹੋ ਗਿਆ
files-share-sharing = ਸਾਂਝਾ ਕੀਤਾ ਜਾ ਰਿਹਾ ਹੈ…
files-share-shared = { $count ->
    [one] 1 ਵਿਅਕਤੀ ਨਾਲ ਸਾਂਝਾ ਕੀਤਾ
   *[other] { $count } ਲੋਕਾਂ ਨਾਲ ਸਾਂਝਾ ਕੀਤਾ
}
files-share-refused = { $drive } { $addresses } ਨਾਲ ਸਾਂਝਾ ਨਹੀਂ ਕਰ ਸਕਿਆ
files-share-failed = ਸਾਂਝਾਕਰਨ ਬਦਲਿਆ ਨਹੀਂ ਜਾ ਸਕਿਆ: { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
    [one] 1 ਆਈਟਮ ਅੱਪਲੋਡ ਹੋ ਰਹੀ ਹੈ
   *[other] { $count } ਆਈਟਮਾਂ ਅੱਪਲੋਡ ਹੋ ਰਹੀਆਂ ਹਨ
}
files-tray-done = { $count ->
    [one] 1 ਅੱਪਲੋਡ ਪੂਰਾ
   *[other] { $count } ਅੱਪਲੋਡ ਪੂਰੇ
}
files-tray-some-failed = { $done } ਅੱਪਲੋਡ ਹੋਈਆਂ, { $failed } ਅਸਫਲ
files-tray-minutes-left = { $minutes ->
    [one] ਲਗਭਗ ਇੱਕ ਮਿੰਟ ਬਾਕੀ
   *[other] ਲਗਭਗ { $minutes } ਮਿੰਟ ਬਾਕੀ
}
files-tray-seconds-left = ਇੱਕ ਮਿੰਟ ਤੋਂ ਘੱਟ ਬਾਕੀ
files-tray-starting = ਸ਼ੁਰੂ ਹੋ ਰਿਹਾ ਹੈ…
files-tray-cancel-all = ਸਭ ਰੱਦ ਕਰੋ
files-tray-cancel = ਰੱਦ ਕਰੋ
files-tray-fold = ਸੂਚੀ ਲੁਕਾਓ
files-tray-unfold = ਸੂਚੀ ਦਿਖਾਓ
files-tray-close = ਬੰਦ ਕਰੋ
files-tray-progress = { $place } · { $size } ਵਿੱਚੋਂ { $sent }
files-tray-in = { $place } ਵਿੱਚ
files-tray-cancelled = ਰੱਦ ਕੀਤਾ ਗਿਆ
