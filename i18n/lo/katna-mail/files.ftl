# Katna Mail, Lao (ລາວ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = ຊອກຫາໄຟລ໌

## Left side (and chips on a phone)

files-all = ໄຟລ໌ທັງໝົດ
files-pictures = ຮູບພາບ
files-pdfs = PDF
files-documents = ເອກະສານ
files-sheets = ສະເປຣດຊີດ
files-slides = ສະໄລ້
files-other = ອື່ນໆ
files-accounts = ບັນຊີ
files-drives = ໄດຣຟ໌
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = ແບ່ງປັນກັບຂ້ອຍ
files-shown = ສະແດງ
files-received = ໄດ້ຮັບ
files-sent = ຂ້ອຍສົ່ງ

## Over the files

files-count = { $count ->
   *[other] { $count } ໄຟລ໌ · { $size }
}
files-anyone = ທຸກຄົນ
files-from-person = ຈາກ { $name }
files-time-any = ທຸກເວລາ
files-time-today = ມື້ນີ້
files-time-yesterday = ມື້ວານນີ້
files-time-this-week = ອາທິດນີ້
files-time-last-week = ອາທິດແລ້ວ
files-time-this-month = ເດືອນນີ້
files-time-last-month = ເດືອນແລ້ວ
files-time-between = { $first } – { $last }
files-time-hint = ຄລິກມື້ໜຶ່ງ, ຫຼື ລາກຂ້າມຫຼາຍມື້
files-time-summary = { $count ->
   *[other] { $days } · { $count } ໄຟລ໌
}
files-time-clear = ລຶບລ້າງ
files-time-month-back = ເດືອນກ່ອນ
files-time-month-on = ເດືອນຖັດໄປ
files-time-wheel = ເລື່ອນເພື່ອຍ້າຍວັນທີເຫຼົ່ານີ້, ໂດຍຮັກສາຄວາມຍາວໄວ້
files-sort-newest = ໃໝ່ສຸດກ່ອນ
files-sort-oldest = ເກົ່າສຸດກ່ອນ
files-sort-largest = ໃຫຍ່ສຸດກ່ອນ
files-sort-name = ຕາມຊື່
files-grid = ບັດ
files-list = ລາຍການ
files-this-week = ອາທິດນີ້
files-undated = ບໍ່ມີວັນທີ
files-me = ຂ້ອຍ
files-no-subject = (ບໍ່ມີຫົວເລື່ອງ)
files-loading = ກຳລັງລວບລວມໄຟລ໌ຈາກອີເມວຂອງທ່ານ…
files-empty = ໄຟລ໌ຈາກອີເມວຂອງທ່ານຈະສະແດງຢູ່ບ່ອນນີ້.
files-none-match = ບໍ່ມີໄຟລ໌ທີ່ກົງກັນ.
files-load-failed = ອ່ານໄຟລ໌ບໍ່ສຳເລັດ: { $error }

## A file's menu and buttons

files-open = ເປີດ
files-open-with = ເປີດດ້ວຍ…
files-save = ບັນທຶກ…
files-show-mail = ສະແດງອີເມວ
files-mail-window = ເປີດອີເມວໃນໜ້າຕ່າງໃໝ່
files-forward = ສົ່ງຕໍ່ໄຟລ໌
files-from-them = ໄຟລ໌ຈາກ { $name }
files-copy-name = ສຳເນົາຊື່ໄຟລ໌
files-name-copied = ສຳເນົາຊື່ໄຟລ໌ແລ້ວ
files-downloading = ກຳລັງດາວໂຫຼດອີເມວ…
files-download-failed = ບໍ່ສາມາດດາວໂຫຼດອີເມວນີ້ໄດ້.

## A cloud drive in place of the mail files

files-drive-mine = ໄດຣຟ໌ຂອງຂ້ອຍ
files-drive-mine-onedrive = ໄຟລ໌ຂອງຂ້ອຍ
files-drive-results = “{ $words }”
files-drive-count = { $folders ->
    [0] { $files ->
        [one] 1 ໄຟລ໌
       *[other] { $files } ໄຟລ໌
    }
    [one] 1 ໂຟນເດີ · { $files ->
        [one] 1 ໄຟລ໌
       *[other] { $files } ໄຟລ໌
    }
   *[other] { $folders } ໂຟນເດີ · { $files ->
        [one] 1 ໄຟລ໌
       *[other] { $files } ໄຟລ໌
    }
}
files-drive-folders = ໂຟນເດີ
files-drive-files = ໄຟລ໌
files-drive-folder = ໂຟນເດີ
files-drive-meta = { $what } · ແກ້ໄຂເມື່ອ { $date }
files-drive-as-link = { $what } · ເປັນລິ້ງ
files-drive-google-doc = Google Doc
files-drive-google-sheet = Google Sheet
files-drive-google-slides = Google Slides
files-drive-google-drawing = Google Drawing
files-drive-fetching = ກຳລັງດຶງ…
files-drive-loading = ກຳລັງເປີດໄດຣຟ໌…
files-drive-empty = ໂຟນເດີນີ້ຫວ່າງເປົ່າ.
files-drive-unreachable = ບໍ່ສາມາດເຊື່ອມຕໍ່ { $drive } ໄດ້.
files-drive-try-again = ລອງໃໝ່
files-drive-needs-permission = Katna ຕ້ອງການການອະນຸຍາດຈາກທ່ານເທື່ອດຽວ ເພື່ອສະແດງໄດຣຟ໌ນີ້. ເຂົ້າສູ່ລະບົບອີກຄັ້ງ ແລະ ອະນຸຍາດໃຫ້ Katna ເບິ່ງໄຟລ໌ຂອງທ່ານ.
files-drive-allow = ອະນຸຍາດ
files-drive-allow-failed = ການເຂົ້າສູ່ລະບົບບໍ່ສຳເລັດ, ສະນັ້ນໄດຣຟ໌ຍັງປິດຢູ່.
files-drive-attach = ແນບ
files-drive-more = ເພີ່ມເຕີມ
files-drive-download = ດາວໂຫຼດ…
files-drive-open-web = ເປີດໃນ { $drive }
files-drive-copy-link = ສຳເນົາລິ້ງ
files-drive-link-copied = ສຳເນົາລິ້ງແລ້ວ
files-drive-share = ແບ່ງປັນ…
files-drive-rename = ປ່ຽນຊື່
files-drive-trash = ຍ້າຍໄປຖັງຂີ້ເຫຍື້ອ
files-drive-trashed = “{ $name }” ຢູ່ໃນຖັງຂີ້ເຫຍື້ອຂອງ { $drive } ແລ້ວ
files-drive-renamed = ປ່ຽນຊື່ເປັນ “{ $name }” ແລ້ວ
files-drive-getting = ກຳລັງດຶງ { $name } ຈາກ { $drive }…
files-drive-get-failed = ບໍ່ສາມາດດຶງ { $name } ໄດ້: { $error }
files-drive-upload = ອັບໂຫຼດ
files-drive-upload-files = ອັບໂຫຼດໄຟລ໌
files-drive-upload-folder = ອັບໂຫຼດໂຟນເດີ
files-drive-upload-failed = ບໍ່ສາມາດອັບໂຫຼດ { $name } ໄດ້: { $error }
files-drive-upload-needs = ເພື່ອອັບໂຫຼດ, Katna ຕ້ອງການການອະນຸຍາດຈາກທ່ານເທື່ອດຽວ: ກົດ ອະນຸຍາດ ທີ່ ການຕັ້ງຄ່າ › ແອັບເລີ່ມຕົ້ນ › ໜ້າໄຟລ໌.

## The Share dialog of a drive file or folder

files-share-title = ແບ່ງປັນ “{ $name }”
files-share-add = ເພີ່ມຄົນດ້ວຍຊື່ ຫຼື ທີ່ຢູ່
files-share-not-address = “{ $text }” ບໍ່ແມ່ນທີ່ຢູ່ອີເມວ
files-share-notify = ໃຫ້ { $drive } ສົ່ງອີເມວແຈ້ງເຂົາເຈົ້ານຳ
files-share-people = ຄົນທີ່ມີສິດເຂົ້າເຖິງ
files-share-general = ການເຂົ້າເຖິງທົ່ວໄປ
files-share-loading = ກຳລັງອ່ານວ່າໃຜມີສິດເຂົ້າເຖິງ…
files-share-restricted = ຈຳກັດ
files-share-restricted-about = ສະເພາະຄົນທີ່ມີສິດເຂົ້າເຖິງເທົ່ານັ້ນທີ່ເປີດດ້ວຍລິ້ງໄດ້
files-share-anyone = ທຸກຄົນທີ່ມີລິ້ງ
files-share-anyone-can = { $role ->
    [editor] ທຸກຄົນທີ່ມີລິ້ງສາມາດແກ້ໄຂໄດ້
    [commenter] ທຸກຄົນທີ່ມີລິ້ງສາມາດສະແດງຄຳເຫັນໄດ້
   *[viewer] ທຸກຄົນທີ່ມີລິ້ງສາມາດເບິ່ງໄດ້
}
files-share-anyone-about = { $role ->
    [editor] ທຸກຄົນໃນອິນເຕີເນັດທີ່ມີລິ້ງສາມາດແກ້ໄຂໄດ້
    [commenter] ທຸກຄົນໃນອິນເຕີເນັດທີ່ມີລິ້ງສາມາດສະແດງຄຳເຫັນໄດ້
   *[viewer] ທຸກຄົນໃນອິນເຕີເນັດທີ່ມີລິ້ງສາມາດເບິ່ງໄດ້
}
files-share-role-owner = ເຈົ້າຂອງ
files-share-role-editor = ຜູ້ແກ້ໄຂ
files-share-role-commenter = ຜູ້ສະແດງຄຳເຫັນ
files-share-role-viewer = ຜູ້ເບິ່ງ
files-share-you = { $name } (ທ່ານ)
files-share-domain = ທຸກຄົນທີ່ { $domain }
files-share-inherited = ສິດເຂົ້າເຖິງຈາກໂຟນເດີທີ່ມັນຢູ່
files-share-remove = ເອົາສິດເຂົ້າເຖິງອອກ
files-share-copy-link = ສຳເນົາລິ້ງ
files-share-share = ແບ່ງປັນ
files-share-done = ແລ້ວໆ
files-share-close = ປິດ
files-share-sharing = ກຳລັງແບ່ງປັນ…
files-share-shared = { $count ->
   *[other] ແບ່ງປັນກັບ { $count } ຄົນແລ້ວ
}
files-share-refused = { $drive } ບໍ່ສາມາດແບ່ງປັນກັບ { $addresses } ໄດ້
files-share-failed = ບໍ່ສາມາດປ່ຽນການແບ່ງປັນໄດ້: { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
   *[other] ກຳລັງອັບໂຫຼດ { $count } ລາຍການ
}
files-tray-done = { $count ->
   *[other] ອັບໂຫຼດສຳເລັດ { $count } ລາຍການ
}
files-tray-some-failed = ອັບໂຫຼດແລ້ວ { $done }, ລົ້ມເຫຼວ { $failed }
files-tray-minutes-left = { $minutes ->
   *[other] ເຫຼືອປະມານ { $minutes } ນາທີ
}
files-tray-seconds-left = ເຫຼືອບໍ່ຮອດໜຶ່ງນາທີ
files-tray-starting = ກຳລັງເລີ່ມ…
files-tray-cancel-all = ຍົກເລີກທັງໝົດ
files-tray-cancel = ຍົກເລີກ
files-tray-fold = ເຊື່ອງລາຍການ
files-tray-unfold = ສະແດງລາຍການ
files-tray-close = ປິດ
files-tray-progress = { $place } · { $sent } ຈາກ { $size }
files-tray-in = ໃນ { $place }
files-tray-cancelled = ຍົກເລີກແລ້ວ
