# Katna Mail, Malay (Bahasa Melayu).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Navigation (the folders pane)

nav-labels = Label
nav-folders = Folder
nav-label-new = Cipta label baharu
nav-folder-new = Cipta folder baharu
nav-menu-check-mail = Semak mel baharu
nav-menu-check-inbox = Semak peti masuk ini
nav-unified-leave-out = Kecualikan daripada Peti Masuk bersatu
nav-unified-bring-back = Kembalikan ke Peti Masuk bersatu
nav-menu-sign-in-again = Log masuk semula
nav-menu-new-mail = Mel baharu daripada akaun ini
nav-menu-account-settings = Tetapan akaun
nav-account-checked = Disegerakkan · disemak { $ago }
nav-account-in-sync = Disegerakkan
nav-account-connecting = Menyambung…
nav-account-offline = Luar talian, mencuba lagi
nav-account-signed-out = Log masuk { $provider } telah tamat tempoh
nav-account-password-refused = Kata laluan ditolak
nav-account-storage = { $used } daripada { $total } digunakan
nav-menu-new-subfolder = Folder baharu di dalamnya
nav-menu-new-sublabel = Label baharu di dalamnya
nav-menu-rename = Namakan semula
nav-menu-delete = Padam
nav-menu-empty-trash = Kosongkan Sampah
nav-account-unnamed = Akaun { $number }
nav-all-accounts = Semua Akaun
nav-expand = Tunjukkan folder
nav-collapse = Sembunyikan folder
storage-used = { $percent }% daripada { $total } digunakan
storage-used-detail = { $address }: { $used } daripada { $total } digunakan

## Special folders (the user's own folders keep their names)

folder-inbox = Peti Masuk
folder-starred = Dibintangi
folder-snoozed = Ditunda
folder-unread = Belum dibaca
folder-important = Penting
folder-drafts = Draf
folder-sent = Dihantar
folder-archive = Arkib
folder-spam = Spam
folder-trash = Sampah
folder-all-mail = Semua Mel
folder-scheduled = Dijadualkan
folder-waiting = Menunggu balasan
folder-waiting-short = Menunggu
folder-reminders = Peringatan
folder-outbox = Peti Keluar
folder-activity = Aktiviti
folder-not-on-account = Akaun ini tiada folder sedemikian.

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = Label baharu
label-folder-new-title = Folder baharu
label-prompt = Sila masukkan nama label baharu:
label-folder-prompt = Sila masukkan nama folder baharu:
label-name-hint = Nama label
label-folder-name-hint = Nama folder
label-nest = Sarangkan label di bawah:
label-folder-nest = Sarangkan folder di bawah:
label-cancel = Batal
label-create = Cipta
label-creating = Mencipta…
label-created = Label “{ $name }” dicipta.
label-folder-created = Folder “{ $name }” dicipta.
label-rename-title = Namakan semula label
label-folder-rename-title = Namakan semula folder
label-rename = Namakan semula
label-renaming = Menamakan semula…
label-renamed = Label dinamakan semula kepada “{ $name }”.
label-folder-renamed = Folder dinamakan semula kepada “{ $name }”.

## Deleting a folder or label (asked first)

folder-delete-title = Padam “{ $name }”?
folder-delete-body = { $count ->
    [0] Folder ini tiada mel. Folder dialih keluar daripada pelayan, jadi mel web dan telefon anda juga kehilangannya.
   *[other] { $kind ->
        [conversation] { $count ->
           *[other] { $count } perbualan di dalamnya masuk ke Sampah, jadi anda masih boleh mendapatkannya semula.
        }
       *[message] { $count ->
           *[other] { $count } mesej di dalamnya masuk ke Sampah, jadi anda masih boleh mendapatkannya semula.
        }
    } Folder dialih keluar daripada pelayan, jadi mel web dan telefon anda juga kehilangannya.
}
folder-delete-forever-body = { $count ->
    [0] Folder ini tiada mel. Folder dialih keluar daripada pelayan, jadi mel web dan telefon anda juga kehilangannya.
   *[other] { $kind ->
        [conversation] { $count ->
           *[other] { $count } perbualan di dalamnya dipadam selama-lamanya; akaun ini tiada Sampah.
        }
       *[message] { $count ->
           *[other] { $count } mesej di dalamnya dipadam selama-lamanya; akaun ini tiada Sampah.
        }
    } Folder dialih keluar daripada pelayan, jadi mel web dan telefon anda juga kehilangannya.
}
folder-delete-label-body = Label dialih keluar. Melnya kekal dalam Semua Mel dan dalam label lainnya.
folder-delete-confirm = Padam folder
folder-delete-label-confirm = Padam label
folder-deleted = Folder “{ $name }” dipadam
label-deleted = Label “{ $name }” dipadam
