# Katna Mail, Odia (ଓଡ଼ିଆ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = ଫାଇଲ ସନ୍ଧାନ କରନ୍ତୁ

## Left side (and chips on a phone)

files-all = ସମସ୍ତ ଫାଇଲ
files-pictures = ଛବି
files-pdfs = PDF
files-documents = ଡକ୍ୟୁମେଣ୍ଟ
files-sheets = ସ୍ପ୍ରେଡସିଟ
files-slides = ସ୍ଲାଇଡ
files-other = ଅନ୍ୟ
files-accounts = ଆକାଉଣ୍ଟ
files-drives = ଡ୍ରାଇଭ
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = ମୋ ସହ ସେୟାର କରାଯାଇଛି
files-shown = ଦେଖାଯାଉଛି
files-received = ପ୍ରାପ୍ତ
files-sent = ମୋ ଦ୍ୱାରା ପଠାଯାଇଛି

## Over the files

files-count = { $count ->
    [one] { $count }ଟି ଫାଇଲ · { $size }
   *[other] { $count }ଟି ଫାଇଲ · { $size }
}
files-anyone = ଯେକେହି
files-from-person = { $name }ଙ୍କଠାରୁ
files-time-any = ଯେକୌଣସି ସମୟ
files-time-today = ଆଜି
files-time-yesterday = ଗତକାଲି
files-time-this-week = ଏହି ସପ୍ତାହ
files-time-last-week = ଗତ ସପ୍ତାହ
files-time-this-month = ଏହି ମାସ
files-time-last-month = ଗତ ମାସ
files-time-between = { $first } – { $last }
files-time-hint = ଏକ ଦିନ କ୍ଲିକ କରନ୍ତୁ, କିମ୍ବା ଦିନଗୁଡ଼ିକ ଉପରେ ଡ୍ରାଗ କରନ୍ତୁ
files-time-summary = { $count ->
    [one] { $days } · { $count }ଟି ଫାଇଲ
   *[other] { $days } · { $count }ଟି ଫାଇଲ
}
files-time-clear = ଖାଲି କରନ୍ତୁ
files-time-month-back = ପୂର୍ବ ମାସ
files-time-month-on = ପରବର୍ତ୍ତୀ ମାସ
files-time-wheel = ଅବଧି ସମାନ ରଖି ଏହି ତାରିଖଗୁଡ଼ିକୁ ଘୁଞ୍ଚାଇବାକୁ ସ୍କ୍ରୋଲ କରନ୍ତୁ
files-sort-newest = ସବୁଠୁ ନୂଆ ପ୍ରଥମେ
files-sort-oldest = ସବୁଠୁ ପୁରୁଣା ପ୍ରଥମେ
files-sort-largest = ସବୁଠୁ ବଡ଼ ପ୍ରଥମେ
files-sort-name = ନାମ ଅନୁସାରେ
files-grid = କାର୍ଡ
files-list = ତାଲିକା
files-this-week = ଏହି ସପ୍ତାହ
files-undated = ତାରିଖ ନାହିଁ
files-me = ମୁଁ
files-no-subject = (ବିଷୟ ନାହିଁ)
files-loading = ଆପଣଙ୍କ ମେଲରୁ ଫାଇଲ ସଂଗ୍ରହ କରାଯାଉଛି…
files-empty = ଆପଣଙ୍କ ମେଲର ଫାଇଲ ଏଠାରେ ଦେଖାଯାଏ।
files-none-match = କୌଣସି ଫାଇଲ ମେଳ ଖାଉନାହିଁ।
files-load-failed = ଫାଇଲ ପଢ଼ିବା ବିଫଳ ହେଲା: { $error }

## A file's menu and buttons

files-open = ଖୋଲନ୍ତୁ
files-open-with = ଏଥିରେ ଖୋଲନ୍ତୁ…
files-save = ସେଭ କରନ୍ତୁ…
files-show-mail = ମେଲ ଦେଖାନ୍ତୁ
files-mail-window = ମେଲକୁ ନୂଆ ୱିଣ୍ଡୋରେ ଖୋଲନ୍ତୁ
files-forward = ଫାଇଲ ଫରୱାର୍ଡ କରନ୍ତୁ
files-from-them = { $name }ଙ୍କ ଫାଇଲ
files-copy-name = ଫାଇଲ ନାମ କପି କରନ୍ତୁ
files-name-copied = ଫାଇଲ ନାମ କପି ହେଲା
files-downloading = ମେଲ ଡାଉନଲୋଡ ହେଉଛି…
files-download-failed = ଏହି ମେଲ ଡାଉନଲୋଡ କରିହେଲା ନାହିଁ।

## A cloud drive in place of the mail files

files-drive-mine = ମୋ Drive
files-drive-mine-onedrive = ମୋ ଫାଇଲ
files-drive-results = “{ $words }”
files-drive-count = { $folders ->
    [0] { $files ->
        [one] 1ଟି ଫାଇଲ
       *[other] { $files }ଟି ଫାଇଲ
    }
    [one] 1ଟି ଫୋଲ୍ଡର · { $files ->
        [one] 1ଟି ଫାଇଲ
       *[other] { $files }ଟି ଫାଇଲ
    }
   *[other] { $folders }ଟି ଫୋଲ୍ଡର · { $files ->
        [one] 1ଟି ଫାଇଲ
       *[other] { $files }ଟି ଫାଇଲ
    }
}
files-drive-folders = ଫୋଲ୍ଡର
files-drive-files = ଫାଇଲ
files-drive-folder = ଫୋଲ୍ଡର
files-drive-meta = { $what } · { $date }ରେ ସମ୍ପାଦିତ
files-drive-as-link = { $what } · ଲିଙ୍କ ଭାବେ
files-drive-google-doc = Google Doc
files-drive-google-sheet = Google Sheet
files-drive-google-slides = Google Slides
files-drive-google-drawing = Google Drawing
files-drive-fetching = ଅଣାଯାଉଛି…
files-drive-loading = ଡ୍ରାଇଭ ଖୋଲାଯାଉଛି…
files-drive-empty = ଏହି ଫୋଲ୍ଡର ଖାଲି ଅଛି।
files-drive-unreachable = { $drive } ସହ ସଂଯୋଗ ହୋଇପାରୁନାହିଁ।
files-drive-try-again = ପୁଣି ଚେଷ୍ଟା କରନ୍ତୁ
files-drive-needs-permission = ଏହି ଡ୍ରାଇଭ ଦେଖାଇବା ପାଇଁ Katnaକୁ ଥରେ ଆପଣଙ୍କ ଅନୁମତି ଦରକାର। ପୁଣି ସାଇନ ଇନ କରନ୍ତୁ ଏବଂ Katnaକୁ ଆପଣଙ୍କ ଫାଇଲ ଦେଖିବାକୁ ଅନୁମତି ଦିଅନ୍ତୁ।
files-drive-allow = ଅନୁମତି ଦିଅନ୍ତୁ
files-drive-allow-failed = ସାଇନ ଇନ ସମ୍ପୂର୍ଣ୍ଣ ହେଲା ନାହିଁ, ତେଣୁ ଡ୍ରାଇଭ ବନ୍ଦ ରହିଲା।
files-drive-attach = ଆଟାଚ କରନ୍ତୁ
files-drive-more = ଅଧିକ
files-drive-download = ଡାଉନଲୋଡ କରନ୍ତୁ…
files-drive-open-web = { $drive }ରେ ଖୋଲନ୍ତୁ
files-drive-copy-link = ଲିଙ୍କ କପି କରନ୍ତୁ
files-drive-link-copied = ଲିଙ୍କ କପି ହେଲା
files-drive-share = ସେୟାର କରନ୍ତୁ…
files-drive-rename = ନାମ ବଦଳାନ୍ତୁ
files-drive-trash = ବିନକୁ ଘୁଞ୍ଚାନ୍ତୁ
files-drive-trashed = “{ $name }” { $drive } ବିନରେ ଅଛି
files-drive-renamed = “{ $name }” ନାମରେ ବଦଳାଗଲା
files-drive-getting = { $drive }ରୁ { $name } ଅଣାଯାଉଛି…
files-drive-get-failed = { $name } ଅଣାଯାଇପାରିଲା ନାହିଁ: { $error }
files-drive-upload = ଅପଲୋଡ କରନ୍ତୁ
files-drive-upload-files = ଫାଇଲ ଅପଲୋଡ କରନ୍ତୁ
files-drive-upload-folder = ଫୋଲ୍ଡର ଅପଲୋଡ କରନ୍ତୁ
files-drive-upload-failed = { $name } ଅପଲୋଡ କରିହେଲା ନାହିଁ: { $error }
files-drive-upload-needs = ଅପଲୋଡ କରିବାକୁ, Katnaକୁ ଥରେ ଆପଣଙ୍କ ଅନୁମତି ଦରକାର: ସେଟିଂସ › ଡିଫଲ୍ଟ ଆପ › ଫାଇଲ ପୃଷ୍ଠାରେ ଅନୁମତି ଦିଅନ୍ତୁ ଦବାନ୍ତୁ।

## The Share dialog of a drive file or folder

files-share-title = “{ $name }” ସେୟାର କରନ୍ତୁ
files-share-add = ନାମ କିମ୍ବା ଠିକଣା ଦ୍ୱାରା ଲୋକଙ୍କୁ ଯୋଗ କରନ୍ତୁ
files-share-not-address = “{ $text }” ଏକ ଇମେଲ ଠିକଣା ନୁହେଁ
files-share-notify = { $drive } ମଧ୍ୟ ସେମାନଙ୍କୁ ଇମେଲ କରୁ
files-share-people = ଆକ୍ସେସ ଥିବା ଲୋକ
files-share-general = ସାଧାରଣ ଆକ୍ସେସ
files-share-loading = କାହାର ଆକ୍ସେସ ଅଛି ପଢ଼ାଯାଉଛି…
files-share-restricted = ସୀମିତ
files-share-restricted-about = କେବଳ ଆକ୍ସେସ ଥିବା ଲୋକ ଲିଙ୍କ ସହ ଏହାକୁ ଖୋଲିପାରିବେ
files-share-anyone = ଲିଙ୍କ ଥିବା ଯେକେହି
files-share-anyone-can = { $role ->
    [editor] ଲିଙ୍କ ଥିବା ଯେକେହି ସମ୍ପାଦନ କରିପାରିବେ
    [commenter] ଲିଙ୍କ ଥିବା ଯେକେହି ମନ୍ତବ୍ୟ କରିପାରିବେ
   *[viewer] ଲିଙ୍କ ଥିବା ଯେକେହି ଦେଖିପାରିବେ
}
files-share-anyone-about = { $role ->
    [editor] ଇଣ୍ଟରନେଟରେ ଲିଙ୍କ ଥିବା ଯେକେହି ସମ୍ପାଦନ କରିପାରିବେ
    [commenter] ଇଣ୍ଟରନେଟରେ ଲିଙ୍କ ଥିବା ଯେକେହି ମନ୍ତବ୍ୟ କରିପାରିବେ
   *[viewer] ଇଣ୍ଟରନେଟରେ ଲିଙ୍କ ଥିବା ଯେକେହି ଦେଖିପାରିବେ
}
files-share-role-owner = ମାଲିକ
files-share-role-editor = ସମ୍ପାଦକ
files-share-role-commenter = ମନ୍ତବ୍ୟକାରୀ
files-share-role-viewer = ଦର୍ଶକ
files-share-you = { $name } (ଆପଣ)
files-share-domain = { $domain }ର ସମସ୍ତେ
files-share-inherited = ଏହା ଥିବା ଫୋଲ୍ଡରରୁ ଆକ୍ସେସ
files-share-remove = ଆକ୍ସେସ କାଢ଼ନ୍ତୁ
files-share-copy-link = ଲିଙ୍କ କପି କରନ୍ତୁ
files-share-share = ସେୟାର କରନ୍ତୁ
files-share-done = ହୋଇଗଲା
files-share-sharing = ସେୟାର କରାଯାଉଛି…
files-share-shared = { $count ->
    [one] 1 ଜଣଙ୍କ ସହ ସେୟାର କରାଗଲା
   *[other] { $count } ଜଣଙ୍କ ସହ ସେୟାର କରାଗଲା
}
files-share-refused = { $drive } { $addresses } ସହ ସେୟାର କରିପାରିଲା ନାହିଁ
files-share-failed = ସେୟାରିଂ ବଦଳାଇହେଲା ନାହିଁ: { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
    [one] 1ଟି ଆଇଟମ ଅପଲୋଡ ହେଉଛି
   *[other] { $count }ଟି ଆଇଟମ ଅପଲୋଡ ହେଉଛି
}
files-tray-done = { $count ->
    [one] 1ଟି ଅପଲୋଡ ସମ୍ପୂର୍ଣ୍ଣ
   *[other] { $count }ଟି ଅପଲୋଡ ସମ୍ପୂର୍ଣ୍ଣ
}
files-tray-some-failed = { $done }ଟି ଅପଲୋଡ ହେଲା, { $failed }ଟି ବିଫଳ
files-tray-minutes-left = { $minutes ->
    [one] ପ୍ରାୟ ଏକ ମିନିଟ ବାକି
   *[other] ପ୍ରାୟ { $minutes } ମିନିଟ ବାକି
}
files-tray-seconds-left = ଏକ ମିନିଟରୁ କମ୍ ବାକି
files-tray-starting = ଆରମ୍ଭ ହେଉଛି…
files-tray-cancel-all = ସବୁ ବାତିଲ କରନ୍ତୁ
files-tray-cancel = ବାତିଲ କରନ୍ତୁ
files-tray-fold = ତାଲିକା ଲୁଚାନ୍ତୁ
files-tray-unfold = ତାଲିକା ଦେଖାନ୍ତୁ
files-tray-close = ବନ୍ଦ କରନ୍ତୁ
files-tray-progress = { $place } · { $size }ରୁ { $sent }
files-tray-in = { $place }ରେ
files-tray-cancelled = ବାତିଲ ହେଲା
