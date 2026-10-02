# Katna Mail, Khmer (ខ្មែរ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = ស្វែងរកឯកសារ

## Left side (and chips on a phone)

files-all = ឯកសារទាំងអស់
files-pictures = រូបភាព
files-pdfs = PDF
files-documents = ឯកសារអត្ថបទ
files-sheets = សៀវភៅបញ្ជី
files-slides = ស្លាយ
files-other = ផ្សេងៗ
files-accounts = គណនី
files-drives = ដ្រាយ
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = បានចែករំលែកជាមួយខ្ញុំ
files-shown = បង្ហាញ
files-received = បានទទួល
files-sent = ផ្ញើដោយខ្ញុំ

## Over the files

files-count = { $count ->
   *[other] ឯកសារ { $count } · { $size }
}
files-anyone = អ្នកណាក៏បាន
files-from-person = ពី { $name }
files-time-any = គ្រប់ពេល
files-time-today = ថ្ងៃនេះ
files-time-yesterday = ម្សិលមិញ
files-time-this-week = សប្ដាហ៍នេះ
files-time-last-week = សប្ដាហ៍មុន
files-time-this-month = ខែនេះ
files-time-last-month = ខែមុន
files-time-between = { $first } – { $last }
files-time-hint = ចុចលើថ្ងៃមួយ ឬអូសកាត់ថ្ងៃច្រើន
files-time-summary = { $count ->
   *[other] { $days } · ឯកសារ { $count }
}
files-time-clear = សម្អាត
files-time-month-back = ខែមុន
files-time-month-on = ខែបន្ទាប់
files-time-wheel = រមូលដើម្បីផ្លាស់ទីកាលបរិច្ឆេទទាំងនេះ ដោយរក្សាប្រវែងដដែល
files-sort-newest = ថ្មីបំផុតមុនគេ
files-sort-oldest = ចាស់បំផុតមុនគេ
files-sort-largest = ធំបំផុតមុនគេ
files-sort-name = តាមឈ្មោះ
files-grid = កាត
files-list = បញ្ជី
files-this-week = សប្ដាហ៍នេះ
files-undated = គ្មានកាលបរិច្ឆេទ
files-me = ខ្ញុំ
files-no-subject = (គ្មានប្រធានបទ)
files-loading = កំពុងប្រមូលឯកសារពីសំបុត្ររបស់អ្នក…
files-empty = ឯកសារពីសំបុត្ររបស់អ្នកបង្ហាញនៅទីនេះ។
files-none-match = គ្មានឯកសារដែលត្រូវគ្នាទេ។
files-load-failed = ការអានឯកសារបានបរាជ័យ៖ { $error }

## A file's menu and buttons

files-open = បើក
files-open-with = បើកជាមួយ…
files-save = រក្សាទុក…
files-show-mail = បង្ហាញសំបុត្រ
files-mail-window = បើកសំបុត្រក្នុងបង្អួចថ្មី
files-forward = បញ្ជូនបន្តឯកសារ
files-from-them = ឯកសារពី { $name }
files-copy-name = ចម្លងឈ្មោះឯកសារ
files-name-copied = បានចម្លងឈ្មោះឯកសារ
files-downloading = កំពុងទាញយកសំបុត្រ…
files-download-failed = មិនអាចទាញយកសំបុត្រនេះបានទេ។

## A cloud drive in place of the mail files

files-drive-mine = ដ្រាយរបស់ខ្ញុំ
files-drive-mine-onedrive = ឯកសាររបស់ខ្ញុំ
files-drive-results = “{ $words }”
files-drive-count = { $folders ->
    [0] { $files ->
        [one] ឯកសារ 1
       *[other] ឯកសារ { $files }
    }
    [one] ថត 1 · { $files ->
        [one] ឯកសារ 1
       *[other] ឯកសារ { $files }
    }
   *[other] ថត { $folders } · { $files ->
        [one] ឯកសារ 1
       *[other] ឯកសារ { $files }
    }
}
files-drive-folders = ថត
files-drive-files = ឯកសារ
files-drive-folder = ថត
files-drive-meta = { $what } · បានកែ { $date }
files-drive-as-link = { $what } · ជាតំណ
files-drive-google-doc = Google Doc
files-drive-google-sheet = Google Sheet
files-drive-google-slides = Google Slides
files-drive-google-drawing = Google Drawing
files-drive-fetching = កំពុងទាញយក…
files-drive-loading = កំពុងបើកដ្រាយ…
files-drive-empty = ថតនេះទទេ។
files-drive-unreachable = មិនអាចភ្ជាប់ទៅ { $drive } បានទេ។
files-drive-try-again = សាកម្ដងទៀត
files-drive-needs-permission = Katna ត្រូវការការអនុញ្ញាតពីអ្នកម្ដង ដើម្បីបង្ហាញដ្រាយនេះ។ ចូលម្ដងទៀត ហើយអនុញ្ញាតឱ្យ Katna មើលឯកសាររបស់អ្នក។
files-drive-allow = អនុញ្ញាត
files-drive-allow-failed = ការចូលមិនបានបញ្ចប់ ដូច្នេះដ្រាយនៅតែបិទ។
files-drive-attach = ភ្ជាប់
files-drive-more = ច្រើនទៀត
files-drive-download = ទាញយក…
files-drive-open-web = បើកក្នុង { $drive }
files-drive-copy-link = ចម្លងតំណ
files-drive-link-copied = បានចម្លងតំណ
files-drive-share = ចែករំលែក…
files-drive-rename = ប្ដូរឈ្មោះ
files-drive-trash = ផ្លាស់ទីទៅធុងសំរាម
files-drive-trashed = “{ $name }” នៅក្នុងធុងសំរាមរបស់ { $drive }
files-drive-renamed = បានប្ដូរឈ្មោះទៅជា “{ $name }”
files-drive-getting = កំពុងទាញយក { $name } ពី { $drive }…
files-drive-get-failed = មិនអាចទាញយក { $name } បានទេ៖ { $error }
files-drive-upload = ផ្ទុកឡើង
files-drive-upload-files = ផ្ទុកឯកសារឡើង
files-drive-upload-folder = ផ្ទុកថតឡើង
files-drive-upload-failed = មិនអាចផ្ទុក { $name } ឡើងបានទេ៖ { $error }
files-drive-upload-needs = ដើម្បីផ្ទុកឡើង Katna ត្រូវការការអនុញ្ញាតពីអ្នកម្ដង៖ ចុច អនុញ្ញាត នៅក្នុង ការកំណត់ › កម្មវិធីលំនាំដើម › ទំព័រឯកសារ។

## The Share dialog of a drive file or folder

files-share-title = ចែករំលែក “{ $name }”
files-share-add = បន្ថែមមនុស្សតាមឈ្មោះ ឬអាសយដ្ឋាន
files-share-not-address = “{ $text }” មិនមែនជាអាសយដ្ឋានអ៊ីមែលទេ
files-share-notify = ឱ្យ { $drive } ផ្ញើអ៊ីមែលទៅពួកគេដែរ
files-share-people = មនុស្សដែលមានសិទ្ធិចូលប្រើ
files-share-general = សិទ្ធិចូលប្រើទូទៅ
files-share-loading = កំពុងអានថាអ្នកណាមានសិទ្ធិចូលប្រើ…
files-share-restricted = បានដាក់កម្រិត
files-share-restricted-about = មានតែមនុស្សដែលមានសិទ្ធិចូលប្រើទេ ទើបអាចបើកវាដោយតំណបាន
files-share-anyone = អ្នកណាក៏បានដែលមានតំណ
files-share-anyone-can = { $role ->
    [editor] អ្នកណាក៏បានដែលមានតំណ អាចកែបាន
    [commenter] អ្នកណាក៏បានដែលមានតំណ អាចផ្ដល់មតិបាន
   *[viewer] អ្នកណាក៏បានដែលមានតំណ អាចមើលបាន
}
files-share-anyone-about = { $role ->
    [editor] អ្នកណាក៏បាននៅលើអ៊ីនធឺណិតដែលមានតំណ អាចកែបាន
    [commenter] អ្នកណាក៏បាននៅលើអ៊ីនធឺណិតដែលមានតំណ អាចផ្ដល់មតិបាន
   *[viewer] អ្នកណាក៏បាននៅលើអ៊ីនធឺណិតដែលមានតំណ អាចមើលបាន
}
files-share-role-owner = ម្ចាស់
files-share-role-editor = អ្នកកែ
files-share-role-commenter = អ្នកផ្ដល់មតិ
files-share-role-viewer = អ្នកមើល
files-share-you = { $name } (អ្នក)
files-share-domain = មនុស្សទាំងអស់នៅ { $domain }
files-share-inherited = សិទ្ធិចូលប្រើពីថតដែលវានៅក្នុង
files-share-remove = ដកសិទ្ធិចូលប្រើ
files-share-copy-link = ចម្លងតំណ
files-share-share = ចែករំលែក
files-share-done = រួចរាល់
files-share-sharing = កំពុងចែករំលែក…
files-share-shared = { $count ->
   *[other] បានចែករំលែកជាមួយ { $count } នាក់
}
files-share-refused = { $drive } មិនអាចចែករំលែកជាមួយ { $addresses } បានទេ
files-share-failed = មិនអាចប្ដូរការចែករំលែកបានទេ៖ { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
   *[other] កំពុងផ្ទុកឡើង { $count } ធាតុ
}
files-tray-done = { $count ->
   *[other] បានផ្ទុកឡើងរួច { $count }
}
files-tray-some-failed = បានផ្ទុកឡើង { $done } បរាជ័យ { $failed }
files-tray-minutes-left = { $minutes ->
   *[other] នៅសល់ប្រហែល { $minutes } នាទី
}
files-tray-seconds-left = នៅសល់តិចជាងមួយនាទី
files-tray-starting = កំពុងចាប់ផ្ដើម…
files-tray-cancel-all = បោះបង់ទាំងអស់
files-tray-cancel = បោះបង់
files-tray-fold = លាក់បញ្ជី
files-tray-unfold = បង្ហាញបញ្ជី
files-tray-close = បិទ
files-tray-progress = { $place } · { $sent } នៃ { $size }
files-tray-in = នៅក្នុង { $place }
files-tray-cancelled = បានបោះបង់
