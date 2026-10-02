# Katna Mail, Dzongkha (རྫོང་ཁ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = ཡིག་སྣོད་འཚོལ།

## Left side (and chips on a phone)

files-all = ཡིག་སྣོད་ཆ་མཉམ
files-pictures = པར་ཚུ
files-pdfs = PDF ཚུ
files-documents = ཡིག་ཆ་ཚུ
files-sheets = ཤོག་ཁྲམ་ཚུ
files-slides = བཤུད་བརྙན་ཚུ
files-other = གཞན
files-accounts = རྩིས་ཐོ་ཚུ
files-drives = ཌའིཝ་ཚུ
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = ང་དང་བརྗེ་སོར་འབད་ཡོདཔ
files-shown = སྟོན་ཡོདཔ
files-received = ཐོབ་ཡོདཔ
files-sent = ང་གིས་བཏང་ཡོདཔ

## Over the files

files-count = { $count ->
   *[other] ཡིག་སྣོད་ { $count } · { $size }
}
files-anyone = ག་ཨིན་རུང
files-from-person = { $name } ལས
files-time-any = ནམ་རང་ཨིན་རུང
files-time-today = ད་རིས
files-time-yesterday = ཁ་ཙ
files-time-this-week = བདུན་ཕྲག་འདི
files-time-last-week = བདུན་ཕྲག་ཧེ་མམ
files-time-this-month = ཟླ་ཝ་འདི
files-time-last-month = ཟླ་ཝ་ཧེ་མམ
files-time-between = { $first } – { $last }
files-time-hint = ཉིནམ་ཅིག་ཨེབ་ ཡང་ན་ ཉིནམ་ཚུ་གི་ཁ་ཐོག་ལས་འདྲུད།
files-time-summary = { $count ->
   *[other] { $days } · ཡིག་སྣོད་ { $count }
}
files-time-clear = བསལ།
files-time-month-back = ཟླ་ཝ་ཧེ་མམ
files-time-month-on = ཟླ་ཝ་ཤུལ་མམ
files-time-wheel = ཚེས་གྲངས་འདི་ཚུ་ རིང་ཚད་མ་བསྒྱུར་བར་ སྤོ་ནིའི་དོན་ལུ་ བཤུད་བྱིས།
files-sort-newest = གསར་ཤོས་ཧེ་མ
files-sort-oldest = རྙིང་ཤོས་ཧེ་མ
files-sort-largest = སྦོམ་ཤོས་ཧེ་མ
files-sort-name = མིང་ལྟར
files-grid = ཤོག་བྱང་ཚུ
files-list = ཐོ་ཡིག
files-this-week = བདུན་ཕྲག་འདི
files-undated = ཚེས་གྲངས་མེད
files-me = ང
files-no-subject = (དོན་ཚན་མེད)
files-loading = ཁྱོད་ཀྱི་གློག་འཕྲིན་ནང་ལས་ ཡིག་སྣོད་ཚུ་བསྡུ་དོ…
files-empty = ཁྱོད་ཀྱི་གློག་འཕྲིན་ནང་གི་ཡིག་སྣོད་ཚུ་ ནཱ་ལུ་སྟོནམ་ཨིན།
files-none-match = མཐུན་པའི་ཡིག་སྣོད་མིན་འདུག
files-load-failed = ཡིག་སྣོད་ཚུ་ལྷག་མ་ཚུགས: { $error }

## A file's menu and buttons

files-open = ཁ་ཕྱེ།
files-open-with = འདི་གིས་ཁ་ཕྱེ…
files-save = སྲུང་…
files-show-mail = གློག་འཕྲིན་སྟོན།
files-mail-window = གློག་འཕྲིན་འདི་ སྒོ་སྒྲིག་གསརཔ་ནང་ཁ་ཕྱེ།
files-forward = ཡིག་སྣོད་མདུན་སྐྱེལ་འབད།
files-from-them = { $name } ལས་ཡིག་སྣོད་ཚུ
files-copy-name = ཡིག་སྣོད་ཀྱི་མིང་འདྲ་བཤུས་རྐྱབ།
files-name-copied = ཡིག་སྣོད་ཀྱི་མིང་འདྲ་བཤུས་རྐྱབ་ཡི
files-downloading = གློག་འཕྲིན་ཕབ་ལེན་འབད་དོ…
files-download-failed = གློག་འཕྲིན་འདི་ཕབ་ལེན་འབད་མ་ཚུགས།

## A cloud drive in place of the mail files

files-drive-mine = ངེ་གི་ཌའིཝ
files-drive-mine-onedrive = ངེ་གི་ཡིག་སྣོད་ཚུ
files-drive-results = “{ $words }”
files-drive-count = { $folders ->
    [0] { $files ->
       *[other] ཡིག་སྣོད་ { $files }
    }
   *[other] སྣོད་འཛིན་ { $folders } · { $files ->
       *[other] ཡིག་སྣོད་ { $files }
    }
}
files-drive-folders = སྣོད་འཛིན་ཚུ
files-drive-files = ཡིག་སྣོད་ཚུ
files-drive-folder = སྣོད་འཛིན
files-drive-meta = { $what } · { $date } ལུ་ཞུན་དག་འབད་ཡི
files-drive-as-link = { $what } · འབྲེལ་མཐུད་སྦེ
files-drive-google-doc = Google Doc
files-drive-google-sheet = Google Sheet
files-drive-google-slides = Google Slides
files-drive-google-drawing = Google Drawing
files-drive-fetching = ལེན་དོ…
files-drive-loading = ཌའིཝ་ཁ་ཕྱེ་དོ…
files-drive-empty = སྣོད་འཛིན་འདི་སྟོངམ་ཨིན།
files-drive-unreachable = { $drive } ལུ་ལྷོད་མ་ཚུགས།
files-drive-try-again = ལོག་འབད་རྩོལ་བསྐྱེད།
files-drive-needs-permission = ཌའིཝ་འདི་སྟོན་ནིའི་དོན་ལུ་ Katna ལུ་ ཁྱོད་ཀྱི་ཆོག་ཐམ་ཚར་གཅིག་དགོ། ལོག་ནང་བསྐྱོད་འབད་ཞིནམ་ལས་ Katna ལུ་ ཁྱོད་ཀྱི་ཡིག་སྣོད་ཚུ་བལྟ་ནི་ལུ་ཆོག་ཐམ་བྱིན།
files-drive-allow = ཆོག་ཐམ་བྱིན།
files-drive-allow-failed = ནང་བསྐྱོད་འདི་མ་ཚར་བས་ ཌའིཝ་འདི་ཁ་བསྡམས་ཏེ་སྡོདཔ་ཨིན།
files-drive-attach = མཉམ་སྦྲགས།
files-drive-more = ཧེང་བཀལ
files-drive-download = ཕབ་ལེན་འབད…
files-drive-open-web = { $drive } ནང་ཁ་ཕྱེ།
files-drive-copy-link = འབྲེལ་མཐུད་འདྲ་བཤུས་རྐྱབ།
files-drive-link-copied = འབྲེལ་མཐུད་འདྲ་བཤུས་རྐྱབ་ཡི
files-drive-share = བརྗེ་སོར་འབད…
files-drive-rename = མིང་བསྒྱུར།
files-drive-trash = གད་སྙིགས་ནང་སྤོ།
files-drive-trashed = “{ $name }” འདི་ { $drive } གི་གད་སྙིགས་ནང་ཡོད
files-drive-renamed = “{ $name }” ལུ་མིང་བསྒྱུར་ཡི
files-drive-getting = { $drive } ལས་ { $name } ལེན་དོ…
files-drive-get-failed = { $name } ལེན་མ་ཚུགས: { $error }
files-drive-upload = ཡར་སྐྱེལ།
files-drive-upload-files = ཡིག་སྣོད་ཚུ་ཡར་སྐྱེལ་འབད།
files-drive-upload-folder = སྣོད་འཛིན་ཡར་སྐྱེལ་འབད།
files-drive-upload-failed = { $name } ཡར་སྐྱེལ་འབད་མ་ཚུགས: { $error }
files-drive-upload-needs = ཡར་སྐྱེལ་འབད་ནིའི་དོན་ལུ་ Katna ལུ་ ཁྱོད་ཀྱི་ཆོག་ཐམ་ཚར་གཅིག་དགོ: སྒྲིག་སྟངས › སྔོན་སྒྲིག་གློག་རིམ་ཚུ › ཡིག་སྣོད་ཚུ་གི་ཤོག་ལེབ་ ནང་ ཆོག་ཐམ་བྱིན་ ཨེབ།

## The Share dialog of a drive file or folder

files-share-title = “{ $name }” བརྗེ་སོར་འབད།
files-share-add = མིང་ ཡང་ན་ ཁ་བྱང་གིས་ མི་ཚུ་ཁ་སྐོང་འབད།
files-share-not-address = “{ $text }” འདི་ གློག་འཕྲིན་ཁ་བྱང་མེན།
files-share-notify = { $drive } གིས་ ཁོང་ལུ་ གློག་འཕྲིན་ཡང་གཏང་།
files-share-people = འཛུལ་སྤྱོད་ཡོད་མི་མི་ཚུ
files-share-general = སྤྱིར་བཏང་འཛུལ་སྤྱོད
files-share-loading = འཛུལ་སྤྱོད་ག་ལུ་ཡོདཔ་ཨིན་ན་ ལྷག་དོ…
files-share-restricted = བཀག་ཆ་ཡོདཔ
files-share-restricted-about = འཛུལ་སྤྱོད་ཡོད་མི་མི་ཚུ་གིས་རྐྱངམ་ཅིག་ འབྲེལ་མཐུད་དང་གཅིག་ཁར་ ཁ་ཕྱེ་ཚུགས
files-share-anyone = འབྲེལ་མཐུད་ཡོད་མི་ ག་ཨིན་རུང
files-share-anyone-can = { $role ->
    [editor] འབྲེལ་མཐུད་ཡོད་མི་ ག་ཨིན་རུང་གིས་ ཞུན་དག་འབད་ཚུགས
    [commenter] འབྲེལ་མཐུད་ཡོད་མི་ ག་ཨིན་རུང་གིས་ བསམ་བཀོད་བྲི་ཚུགས
   *[viewer] འབྲེལ་མཐུད་ཡོད་མི་ ག་ཨིན་རུང་གིས་ བལྟ་ཚུགས
}
files-share-anyone-about = { $role ->
    [editor] ཨིན་ཊར་ནེཊ་གུ་ འབྲེལ་མཐུད་ཡོད་མི་ ག་ཨིན་རུང་གིས་ ཞུན་དག་འབད་ཚུགས
    [commenter] ཨིན་ཊར་ནེཊ་གུ་ འབྲེལ་མཐུད་ཡོད་མི་ ག་ཨིན་རུང་གིས་ བསམ་བཀོད་བྲི་ཚུགས
   *[viewer] ཨིན་ཊར་ནེཊ་གུ་ འབྲེལ་མཐུད་ཡོད་མི་ ག་ཨིན་རུང་གིས་ བལྟ་ཚུགས
}
files-share-role-owner = ཇོ་བདག
files-share-role-editor = ཞུན་དག་པ
files-share-role-commenter = བསམ་བཀོད་པ
files-share-role-viewer = བལྟ་མི
files-share-you = { $name } (ཁྱོད)
files-share-domain = { $domain } ནང་གི་ མི་ག་ར
files-share-inherited = ཡོད་སའི་སྣོད་འཛིན་ལས་ འཛུལ་སྤྱོད
files-share-remove = འཛུལ་སྤྱོད་བཏོན།
files-share-copy-link = འབྲེལ་མཐུད་འདྲ་བཤུས་རྐྱབ།
files-share-share = བརྗེ་སོར་འབད།
files-share-done = ཚར་ཡི།
files-share-sharing = བརྗེ་སོར་འབད་དོ…
files-share-shared = { $count ->
   *[other] མི་ { $count } དང་ བརྗེ་སོར་འབད་ཡི
}
files-share-refused = { $drive } གིས་ { $addresses } དང་ བརྗེ་སོར་འབད་མ་ཚུགས
files-share-failed = བརྗེ་སོར་བསྒྱུར་མ་ཚུགས: { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
   *[other] རྣམ་གྲངས་ { $count } ཡར་སྐྱེལ་འབད་དོ
}
files-tray-done = { $count ->
   *[other] ཡར་སྐྱེལ་ { $count } ཚར་ཡི
}
files-tray-some-failed = { $done } ཡར་སྐྱེལ་འབད་ཡི། { $failed } འཐུས་ཤོར་བྱུང་ཡི
files-tray-minutes-left = { $minutes ->
   *[other] སྐར་མ་ { $minutes } དེ་ཅིག་ལུས་ཡོད
}
files-tray-seconds-left = སྐར་མ་གཅིག་ལས་ཉུང་བ་ལུས་ཡོད
files-tray-starting = འགོ་བཙུགས་དོ…
files-tray-cancel-all = ཆ་མཉམ་ཆ་མེད་གཏང་།
files-tray-cancel = ཆ་མེད་གཏང་།
files-tray-fold = ཐོ་ཡིག་སྦ།
files-tray-unfold = ཐོ་ཡིག་སྟོན།
files-tray-close = ཁ་བསྡམས།
files-tray-progress = { $place } · { $size } ལས་ { $sent }
files-tray-in = { $place } ནང་
files-tray-cancelled = ཆ་མེད་གཏང་ཡི
