# Katna Mail, Tamil (தமிழ்).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Navigation (the folders pane)

nav-labels = லேபிள்கள்
nav-folders = ஃபோல்டர்கள்
nav-label-new = புதிய லேபிளை உருவாக்கு
nav-folder-new = புதிய ஃபோல்டரை உருவாக்கு
nav-menu-check-mail = புதிய அஞ்சலைச் சரிபார்
nav-menu-check-inbox = இந்த இன்பாக்ஸைச் சரிபார்
nav-unified-leave-out = ஒருங்கிணைந்த இன்பாக்ஸிலிருந்து விலக்கு
nav-unified-bring-back = ஒருங்கிணைந்த இன்பாக்ஸில் மீண்டும் சேர்
nav-menu-sign-in-again = மீண்டும் உள்நுழை
nav-menu-new-mail = இந்தக் கணக்கிலிருந்து புதிய அஞ்சல்
nav-menu-account-settings = கணக்கு அமைப்புகள்
nav-account-checked = ஒத்திசைவில் உள்ளது · { $ago } சரிபார்க்கப்பட்டது
nav-account-in-sync = ஒத்திசைவில் உள்ளது
nav-account-connecting = இணைக்கிறது…
nav-account-offline = ஆஃப்லைன், மீண்டும் முயல்கிறது
nav-account-signed-out = { $provider } உள்நுழைவு காலாவதியானது
nav-account-password-refused = கடவுச்சொல் ஏற்கப்படவில்லை
nav-account-storage = { $total } இல் { $used } பயன்படுத்தப்பட்டது
nav-menu-new-subfolder = உள்ளே புதிய ஃபோல்டர்
nav-menu-new-sublabel = உள்ளே புதிய லேபிள்
nav-menu-rename = மறுபெயரிடு
nav-menu-delete = நீக்கு
nav-menu-empty-trash = நீக்கியவற்றைக் காலி செய்
nav-account-unnamed = கணக்கு { $number }
nav-all-accounts = எல்லாக் கணக்குகளும்
nav-expand = ஃபோல்டர்களைக் காட்டு
nav-collapse = ஃபோல்டர்களை மறை
storage-used = { $total } இல் { $percent }% பயன்படுத்தப்பட்டுள்ளது
storage-used-detail = { $address }: { $total } இல் { $used } பயன்படுத்தப்பட்டுள்ளது

## Special folders (the user's own folders keep their names)

folder-inbox = இன்பாக்ஸ்
folder-starred = நட்சத்திரமிட்டவை
folder-snoozed = உறக்கநிலையில் உள்ளவை
folder-unread = படிக்காதவை
folder-important = முக்கியமானவை
folder-drafts = வரைவுகள்
folder-sent = அனுப்பியவை
folder-archive = காப்பகம்
folder-spam = ஸ்பேம்
folder-trash = நீக்கியவை
folder-all-mail = எல்லா அஞ்சல்களும்
folder-scheduled = திட்டமிடப்பட்டவை
folder-waiting = பதிலுக்காகக் காத்திருப்பவை
folder-waiting-short = காத்திருப்பவை
folder-reminders = நினைவூட்டல்கள்
folder-outbox = அவுட்பாக்ஸ்
folder-activity = செயல்பாடு

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = புதிய லேபிள்
label-folder-new-title = புதிய ஃபோல்டர்
label-prompt = புதிய லேபிளின் பெயரை உள்ளிடவும்:
label-folder-prompt = புதிய ஃபோல்டரின் பெயரை உள்ளிடவும்:
label-name-hint = லேபிள் பெயர்
label-folder-name-hint = ஃபோல்டர் பெயர்
label-nest = இதன் கீழ் லேபிளை வை:
label-folder-nest = இதன் கீழ் ஃபோல்டரை வை:
label-cancel = ரத்துசெய்
label-create = உருவாக்கு
label-creating = உருவாக்குகிறது…
label-created = “{ $name }” லேபிள் உருவாக்கப்பட்டது.
label-folder-created = “{ $name }” ஃபோல்டர் உருவாக்கப்பட்டது.
label-rename-title = லேபிளுக்கு மறுபெயரிடு
label-folder-rename-title = ஃபோல்டருக்கு மறுபெயரிடு
label-rename = மறுபெயரிடு
label-renaming = மறுபெயரிடுகிறது…
label-renamed = லேபிள் “{ $name }” என மறுபெயரிடப்பட்டது.
label-folder-renamed = ஃபோல்டர் “{ $name }” என மறுபெயரிடப்பட்டது.

## Deleting a folder or label (asked first)

folder-delete-title = “{ $name }” ஐ நீக்கவா?
folder-delete-body = { $count ->
    [0] இதில் அஞ்சல் எதுவும் இல்லை. ஃபோல்டர் சர்வரிலிருந்து அகற்றப்படும், எனவே வெப்மெயிலிலும் உங்கள் ஃபோனிலும் அது இருக்காது.
   *[other] { $kind ->
        [conversation] { $count ->
            [one] இதிலுள்ள { $count } உரையாடல் நீக்கியவைக்குச் செல்லும், எனவே அதை இன்னும் திரும்பப் பெறலாம்.
           *[other] இதிலுள்ள { $count } உரையாடல்களும் நீக்கியவைக்குச் செல்லும், எனவே அவற்றை இன்னும் திரும்பப் பெறலாம்.
        }
       *[message] { $count ->
            [one] இதிலுள்ள { $count } மெசேஜ் நீக்கியவைக்குச் செல்லும், எனவே அதை இன்னும் திரும்பப் பெறலாம்.
           *[other] இதிலுள்ள { $count } மெசேஜ்களும் நீக்கியவைக்குச் செல்லும், எனவே அவற்றை இன்னும் திரும்பப் பெறலாம்.
        }
    } ஃபோல்டர் சர்வரிலிருந்து அகற்றப்படும், எனவே வெப்மெயிலிலும் உங்கள் ஃபோனிலும் அது இருக்காது.
}
folder-delete-forever-body = { $count ->
    [0] இதில் அஞ்சல் எதுவும் இல்லை. ஃபோல்டர் சர்வரிலிருந்து அகற்றப்படும், எனவே வெப்மெயிலிலும் உங்கள் ஃபோனிலும் அது இருக்காது.
   *[other] { $kind ->
        [conversation] { $count ->
            [one] இதிலுள்ள { $count } உரையாடல் நிரந்தரமாக நீக்கப்படும்; இந்தக் கணக்கில் நீக்கியவை ஃபோல்டர் இல்லை.
           *[other] இதிலுள்ள { $count } உரையாடல்களும் நிரந்தரமாக நீக்கப்படும்; இந்தக் கணக்கில் நீக்கியவை ஃபோல்டர் இல்லை.
        }
       *[message] { $count ->
            [one] இதிலுள்ள { $count } மெசேஜ் நிரந்தரமாக நீக்கப்படும்; இந்தக் கணக்கில் நீக்கியவை ஃபோல்டர் இல்லை.
           *[other] இதிலுள்ள { $count } மெசேஜ்களும் நிரந்தரமாக நீக்கப்படும்; இந்தக் கணக்கில் நீக்கியவை ஃபோல்டர் இல்லை.
        }
    } ஃபோல்டர் சர்வரிலிருந்து அகற்றப்படும், எனவே வெப்மெயிலிலும் உங்கள் ஃபோனிலும் அது இருக்காது.
}
folder-delete-label-body = லேபிள் அகற்றப்படும். அதன் அஞ்சல் எல்லா அஞ்சல்களும் பகுதியிலும் அதன் மற்ற லேபிள்களிலும் அப்படியே இருக்கும்.
folder-delete-confirm = ஃபோல்டரை நீக்கு
folder-delete-label-confirm = லேபிளை நீக்கு
folder-deleted = “{ $name }” ஃபோல்டர் நீக்கப்பட்டது
label-deleted = “{ $name }” லேபிள் நீக்கப்பட்டது
