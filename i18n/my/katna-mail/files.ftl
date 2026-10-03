# Katna Mail, Burmese (မြန်မာ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = ဖိုင်များ ရှာရန်

## Left side (and chips on a phone)

files-all = ဖိုင်အားလုံး
files-pictures = ပုံများ
files-pdfs = PDF များ
files-documents = စာရွက်စာတမ်းများ
files-sheets = စာရင်းဇယားများ
files-slides = ဆလိုက်များ
files-other = အခြား
files-accounts = အကောင့်များ
files-drives = Drive များ
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = ကျွန်ုပ်နှင့် မျှဝေထားသည်
files-shown = ပြထားသည်
files-received = လက်ခံရရှိ
files-sent = ကျွန်ုပ် ပို့ထားသည်

## Over the files

files-count = { $count ->
   *[other] ဖိုင် { $count } ခု · { $size }
}
files-anyone = မည်သူမဆို
files-from-person = { $name } ထံမှ
files-time-any = အချိန်မရွေး
files-time-today = ယနေ့
files-time-yesterday = မနေ့က
files-time-this-week = ဤအပတ်
files-time-last-week = ပြီးခဲ့သော အပတ်
files-time-this-month = ဤလ
files-time-last-month = ပြီးခဲ့သော လ
files-time-between = { $first } – { $last }
files-time-hint = ရက်တစ်ရက်ကို နှိပ်ပါ၊ သို့မဟုတ် ရက်များကို ဖြတ်ဆွဲပါ
files-time-summary = { $count ->
   *[other] { $days } · ဖိုင် { $count } ခု
}
files-time-clear = ရှင်းရန်
files-time-month-back = ယခင် လ
files-time-month-on = နောက် လ
files-time-wheel = ဤရက်များကို ရွှေ့ရန် လှိမ့်ပါ၊ ကြာချိန် မပြောင်းပါ
files-sort-newest = အသစ်ဆုံး အရင်
files-sort-oldest = အဟောင်းဆုံး အရင်
files-sort-largest = အကြီးဆုံး အရင်
files-sort-name = အမည်အလိုက်
files-grid = ကတ်များ
files-list = စာရင်း
files-this-week = ဤအပတ်
files-undated = ရက်စွဲ မရှိ
files-me = ကျွန်ုပ်
files-no-subject = (ခေါင်းစဉ် မရှိ)
files-loading = သင့်မေးလ်မှ ဖိုင်များကို စုဆောင်းနေသည်…
files-empty = သင့်မေးလ်မှ ဖိုင်များ ဤနေရာတွင် ပေါ်လာမည်။
files-none-match = ကိုက်ညီသော ဖိုင် မရှိပါ။
files-load-failed = ဖိုင်များကို ဖတ်၍ မရပါ- { $error }

## A file's menu and buttons

files-open = ဖွင့်ရန်
files-open-with = ဖြင့် ဖွင့်ရန်…
files-save = သိမ်းရန်…
files-show-mail = မေးလ်ကို ပြရန်
files-mail-window = မေးလ်ကို ဝင်းဒိုးအသစ်တွင် ဖွင့်ရန်
files-forward = ဖိုင်ကို ထပ်ဆင့်ပို့ရန်
files-from-them = { $name } ထံမှ ဖိုင်များ
files-copy-name = ဖိုင်အမည် ကူးရန်
files-name-copied = ဖိုင်အမည် ကူးပြီး
files-downloading = မေးလ်ကို ဒေါင်းလုဒ်လုပ်နေသည်…
files-download-failed = ဤမေးလ်ကို ဒေါင်းလုဒ်လုပ်၍ မရပါ။

## A cloud drive in place of the mail files

files-drive-mine = ကျွန်ုပ်၏ Drive
files-drive-mine-onedrive = ကျွန်ုပ်၏ ဖိုင်များ
files-drive-results = “{ $words }”
files-drive-count = { $folders ->
    [0] { $files ->
        [one] ဖိုင် { $files } ခု
       *[other] ဖိုင် { $files } ခု
    }
    [one] ဖိုင်တွဲ { $folders } ခု · { $files ->
        [one] ဖိုင် { $files } ခု
       *[other] ဖိုင် { $files } ခု
    }
   *[other] ဖိုင်တွဲ { $folders } ခု · { $files ->
        [one] ဖိုင် { $files } ခု
       *[other] ဖိုင် { $files } ခု
    }
}
files-drive-folders = ဖိုင်တွဲများ
files-drive-files = ဖိုင်များ
files-drive-folder = ဖိုင်တွဲ
files-drive-meta = { $what } · { $date } တွင် ပြင်ထားသည်
files-drive-as-link = { $what } · လင့်ခ်အဖြစ်
files-drive-google-doc = Google Doc
files-drive-google-sheet = Google Sheet
files-drive-google-slides = Google Slides
files-drive-google-drawing = Google Drawing
files-drive-fetching = ရယူနေသည်…
files-drive-loading = drive ကို ဖွင့်နေသည်…
files-drive-empty = ဤဖိုင်တွဲ ဗလာဖြစ်နေသည်။
files-drive-unreachable = { $drive } ကို ဆက်သွယ်၍ မရပါ။
files-drive-try-again = ထပ်စမ်းကြည့်ရန်
files-drive-needs-permission = ဤ drive ကို ပြရန် Katna သည် သင့်ခွင့်ပြုချက် တစ်ကြိမ် လိုအပ်သည်။ ထပ်မံ ဝင်ရောက်ပြီး သင့်ဖိုင်များကို ကြည့်ရန် Katna ကို ခွင့်ပြုပါ။
files-drive-allow = ခွင့်ပြုရန်
files-drive-allow-failed = ဝင်ရောက်ခြင်း မပြီးဆုံးသဖြင့် drive ကို ပိတ်ထားဆဲ ဖြစ်သည်။
files-drive-attach = ပူးတွဲရန်
files-drive-more = နောက်ထပ်
files-drive-download = ဒေါင်းလုဒ်လုပ်ရန်…
files-drive-open-web = { $drive } တွင် ဖွင့်ရန်
files-drive-copy-link = လင့်ခ် ကူးရန်
files-drive-link-copied = လင့်ခ် ကူးပြီး
files-drive-share = မျှဝေရန်…
files-drive-rename = အမည်ပြောင်းရန်
files-drive-trash = အမှိုက်ပုံးသို့ ရွှေ့ရန်
files-drive-trashed = “{ $name }” သည် { $drive } အမှိုက်ပုံးထဲတွင် ရှိသည်
files-drive-renamed = “{ $name }” သို့ အမည်ပြောင်းပြီး
files-drive-getting = { $drive } မှ { $name } ကို ရယူနေသည်…
files-drive-get-failed = { $name } ကို ရယူ၍ မရပါ- { $error }
files-drive-upload = အပ်လုဒ်
files-drive-upload-files = ဖိုင်များ အပ်လုဒ်လုပ်ရန်
files-drive-upload-folder = ဖိုင်တွဲ အပ်လုဒ်လုပ်ရန်
files-drive-upload-failed = { $name } ကို အပ်လုဒ်လုပ်၍ မရပါ- { $error }
files-drive-upload-needs = အပ်လုဒ်လုပ်ရန် Katna သည် သင့်ခွင့်ပြုချက် တစ်ကြိမ် လိုအပ်သည်- ဆက်တင်များ › မူရင်း အက်ပ်များ › ဖိုင်များ စာမျက်နှာ တွင် ခွင့်ပြုရန် ကို နှိပ်ပါ။

## The Share dialog of a drive file or folder

files-share-title = “{ $name }” ကို မျှဝေရန်
files-share-add = အမည် သို့မဟုတ် လိပ်စာဖြင့် လူများ ထည့်ရန်
files-share-not-address = “{ $text }” သည် အီးမေးလ်လိပ်စာ မဟုတ်ပါ
files-share-notify = { $drive } ကလည်း ၎င်းတို့ထံ အီးမေးလ် ပို့ပါစေ
files-share-people = အသုံးပြုခွင့်ရှိသူများ
files-share-general = အထွေထွေ အသုံးပြုခွင့်
files-share-loading = အသုံးပြုခွင့်ရှိသူများကို ဖတ်နေသည်…
files-share-restricted = ကန့်သတ်ထားသည်
files-share-restricted-about = အသုံးပြုခွင့်ရှိသူများသာ လင့်ခ်ဖြင့် ဖွင့်နိုင်သည်
files-share-anyone = လင့်ခ်ရှိသူ မည်သူမဆို
files-share-anyone-can = { $role ->
    [editor] လင့်ခ်ရှိသူ မည်သူမဆို ပြင်ဆင်နိုင်သည်
    [commenter] လင့်ခ်ရှိသူ မည်သူမဆို မှတ်ချက်ပေးနိုင်သည်
   *[viewer] လင့်ခ်ရှိသူ မည်သူမဆို ကြည့်နိုင်သည်
}
files-share-anyone-about = { $role ->
    [editor] အင်တာနက်ပေါ်ရှိ လင့်ခ်ရှိသူ မည်သူမဆို ပြင်ဆင်နိုင်သည်
    [commenter] အင်တာနက်ပေါ်ရှိ လင့်ခ်ရှိသူ မည်သူမဆို မှတ်ချက်ပေးနိုင်သည်
   *[viewer] အင်တာနက်ပေါ်ရှိ လင့်ခ်ရှိသူ မည်သူမဆို ကြည့်နိုင်သည်
}
files-share-role-owner = ပိုင်ရှင်
files-share-role-editor = တည်းဖြတ်သူ
files-share-role-commenter = မှတ်ချက်ပေးသူ
files-share-role-viewer = ကြည့်ရှုသူ
files-share-you = { $name } (သင်)
files-share-domain = { $domain } ရှိ လူတိုင်း
files-share-inherited = ၎င်းပါဝင်သော ဖိုင်တွဲမှ အသုံးပြုခွင့်
files-share-remove = အသုံးပြုခွင့် ဖယ်ရန်
files-share-copy-link = လင့်ခ် ကူးရန်
files-share-share = မျှဝေရန်
files-share-done = ပြီးပါပြီ
files-share-close = ပိတ်ရန်
files-share-sharing = မျှဝေနေသည်…
files-share-shared = { $count ->
   *[other] လူ { $count } ယောက်နှင့် မျှဝေပြီး
}
files-share-refused = { $drive } သည် { $addresses } နှင့် မျှဝေ၍ မရပါ
files-share-failed = မျှဝေမှုကို ပြောင်း၍ မရပါ- { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
   *[other] { $count } ခု အပ်လုဒ်လုပ်နေသည်
}
files-tray-done = { $count ->
   *[other] အပ်လုဒ် { $count } ခု ပြီးပါပြီ
}
files-tray-some-failed = { $done } ခု အပ်လုဒ်ပြီး၊ { $failed } ခု မအောင်မြင်ပါ
files-tray-minutes-left = { $minutes ->
   *[other] { $minutes } မိနစ်ခန့် ကျန်သည်
}
files-tray-seconds-left = တစ်မိနစ်အောက် ကျန်သည်
files-tray-starting = စတင်နေသည်…
files-tray-cancel-all = အားလုံး မလုပ်တော့ပါ
files-tray-cancel = မလုပ်တော့ပါ
files-tray-fold = စာရင်း ဖျောက်ရန်
files-tray-unfold = စာရင်း ပြရန်
files-tray-close = ပိတ်ရန်
files-tray-progress = { $place } · { $size } အနက် { $sent }
files-tray-in = { $place } တွင်
files-tray-cancelled = ပယ်ဖျက်ပြီး
