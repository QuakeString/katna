# Katna Mail, Tamil (தமிழ்).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Language picker (top bar and Settings > General)

language-tooltip = மொழி: { $language }
language-tooltip-system = மொழி: { $language }, சிஸ்டத்தைப் பின்பற்றுகிறது
language-search = மொழியைத் தேடு
language-system-default = சிஸ்டம் இயல்புநிலை
language-system-now = இப்போது { $language }
language-no-match = “{ $query }” உடன் பொருந்தும் மொழி எதுவுமில்லை
language-machine = இயந்திர மொழிபெயர்ப்பு. மேம்படுத்த உதவுங்கள்
language-setting = மொழி
language-setting-detail = மெனுக்கள், பட்டன்கள், மெசேஜ்கள் ஆகியவற்றின் மொழியும் தேதிகள், எண்கள் ஆகியவற்றின் வடிவமும். சிஸ்டம் இயல்புநிலை டெஸ்க்டாப்பைப் பின்பற்றும்.

## Dates and sizes

ago-just-now = சற்றுமுன்
ago-minutes = { $count ->
    [one] { $count } நிமிடத்திற்கு முன்
   *[other] { $count } நிமிடங்களுக்கு முன்
}
ago-hours = { $count ->
    [one] { $count } மணிநேரத்திற்கு முன்
   *[other] { $count } மணிநேரத்திற்கு முன்
}
ago-days = { $count ->
    [one] { $count } நாளுக்கு முன்
   *[other] { $count } நாட்களுக்கு முன்
}
size-bytes = { $count ->
    [one] { $count } பைட்
   *[other] { $count } பைட்கள்
}
size-kb = { $size } KB
size-mb = { $size } MB
size-gb = { $size } GB
size-tb = { $size } TB

## Top bar

folders-hide = ஃபோல்டர்களை மறை
folders-show = ஃபோல்டர்களைக் காட்டு
compose = எழுது
search = தேடு
search-mail = அஞ்சலில் தேடு
search-settings = அமைப்புகளில் தேடு
search-clear = தேடலை அழி
search-options-show = தேடல் விருப்பங்களைக் காட்டு
settings = அமைப்புகள்
account-add = கணக்கைச் சேர்

## App rail (and the bottom bar on a phone)

rail-mail = அஞ்சல்
rail-calendar = கேலெண்டர்
rail-contacts = தொடர்புகள்
rail-tasks = பணிகள்
rail-notes = குறிப்புகள்
rail-feeds = ஊட்டங்கள்

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = விரைவில் வருகிறது
app-calendar-promise = உங்கள் CalDAV கேலெண்டர்கள், அஞ்சலில் வந்த மீட்டிங் அழைப்புகள், நினைவூட்டல்கள் ஆகியவை உங்கள் இன்பாக்ஸுக்கு அருகிலேயே.
app-tasks-promise = CalDAV உடன் ஒத்திசைக்கப்படும் செய்ய வேண்டியவை பட்டியல்களும், அஞ்சலிலிருந்து உருவாக்கப்பட்ட பணிகளும்.
app-notes-promise = விரைவுக் குறிப்புகளும், பின்னர் பார்க்க ஓர் அஞ்சல் அல்லது உரையாடலில் குறிப்புகளும்.
app-feeds-promise = உங்கள் அஞ்சலுக்கு அருகிலேயே RSS, Atom ஊட்டங்களைப் படியுங்கள்.

## Contacts page

app-contacts-loading = உங்கள் அஞ்சலிலிருந்து நபர்களைச் சேகரிக்கிறது…
app-contacts-empty = நீங்கள் அஞ்சல் பரிமாறிக்கொள்ளும் நபர்கள் இங்கே காட்டப்படுவார்கள்.
app-contacts-count = { $count ->
    [one] உங்கள் அஞ்சலிலிருந்து { $count } நபர், அதிகம் தொடர்புகொண்டவர் முதலில்
   *[other] உங்கள் அஞ்சலிலிருந்து { $count } நபர்கள், அதிகம் தொடர்புகொண்டவர்கள் முதலில்
}
app-contacts-top = { $count ->
    [one] உங்கள் அஞ்சலிலிருந்து முதல் { $count } நபர், அதிகம் தொடர்புகொண்டவர் முதலில்
   *[other] உங்கள் அஞ்சலிலிருந்து முதல் { $count } நபர்கள், அதிகம் தொடர்புகொண்டவர்கள் முதலில்
}
app-contacts-messages = { $count ->
    [one] { $count } மெசேஜ்
   *[other] { $count } மெசேஜ்கள்
}
app-contacts-last = கடைசியாக { $date }

## Navigation (the folders pane)

nav-labels = லேபிள்கள்
nav-folders = ஃபோல்டர்கள்
nav-label-new = புதிய லேபிளை உருவாக்கு
nav-folder-new = புதிய ஃபோல்டரை உருவாக்கு
nav-account-unnamed = கணக்கு { $number }
nav-tab-new = { $count ->
    [one] { $count } புதியது
   *[other] { $count } புதியவை
}

## Special folders (the user's own folders keep their names)

folder-inbox = இன்பாக்ஸ்
folder-starred = நட்சத்திரமிட்டவை
folder-drafts = வரைவுகள்
folder-sent = அனுப்பியவை
folder-archive = காப்பகம்
folder-spam = ஸ்பேம்
folder-trash = நீக்கியவை
folder-all-mail = எல்லா அஞ்சல்களும்
folder-scheduled = திட்டமிடப்பட்டவை

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

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = முதன்மை
tab-promotions = விளம்பரங்கள்
tab-social = சமூகம்
tab-updates = புதுப்பிப்புகள்
tab-forums = மன்றங்கள்
tab-focused = கவனத்திற்குரியவை
tab-other = மற்றவை
tab-inbox = இன்பாக்ஸ்
tab-newsletters = செய்திமடல்கள்
tab-notifications = அறிவிப்புகள்
tab-new = { $count } புதியவை
tab-provider-other = Katna வரிசைப்படுத்தியது

## Mail list: toolbar

list-select = தேர்ந்தெடு
list-refresh = புதுப்பி
list-more = மேலும்
list-mark-read = படித்ததாகக் குறி
list-mark-unread = படிக்காததாகக் குறி
list-move-to = இதற்கு நகர்த்து
list-archive = காப்பகப்படுத்து
list-spam = ஸ்பேம் எனப் புகாரளி
list-delete = நீக்கு
list-newer = புதியவை
list-older = பழையவை
list-range = { $total } இல் { $first }–{ $last }
list-range-about = சுமார் { $total } இல் { $first }–{ $last }
list-results = “{ $query }” க்கான முடிவுகள்
list-results-corrected = “{ $query }” க்கான முடிவுகள் காட்டப்படுகின்றன
list-search-instead = அதற்குப் பதிலாக “{ $query }” என்று தேடு
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = அனைத்தும்
list-pick-none = எதுவுமில்லை
list-pick-read = படித்தவை
list-pick-unread = படிக்காதவை
list-pick-starred = நட்சத்திரமிட்டவை
list-pick-unstarred = நட்சத்திரமிடாதவை

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] { $count } உரையாடல் தேர்ந்தெடுக்கப்பட்டுள்ளது.
       *[other] { $count } உரையாடல்களும் தேர்ந்தெடுக்கப்பட்டுள்ளன.
    }
   *[message] { $count ->
        [one] { $count } மெசேஜ் தேர்ந்தெடுக்கப்பட்டுள்ளது.
       *[other] { $count } மெசேஜ்களும் தேர்ந்தெடுக்கப்பட்டுள்ளன.
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder } இல் உள்ள { $count } உரையாடல் தேர்ந்தெடுக்கப்பட்டுள்ளது.
       *[other] { $folder } இல் உள்ள { $count } உரையாடல்களும் தேர்ந்தெடுக்கப்பட்டுள்ளன.
    }
   *[message] { $count ->
        [one] { $folder } இல் உள்ள { $count } மெசேஜ் தேர்ந்தெடுக்கப்பட்டுள்ளது.
       *[other] { $folder } இல் உள்ள { $count } மெசேஜ்களும் தேர்ந்தெடுக்கப்பட்டுள்ளன.
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] திரையில் உள்ள { $count } உரையாடல் தேர்ந்தெடுக்கப்பட்டுள்ளது.
       *[other] திரையில் உள்ள { $count } உரையாடல்களும் தேர்ந்தெடுக்கப்பட்டுள்ளன.
    }
   *[message] { $count ->
        [one] திரையில் உள்ள { $count } மெசேஜ் தேர்ந்தெடுக்கப்பட்டுள்ளது.
       *[other] திரையில் உள்ள { $count } மெசேஜ்களும் தேர்ந்தெடுக்கப்பட்டுள்ளன.
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] { $count } உரையாடலைத் தேர்ந்தெடு
       *[other] { $count } உரையாடல்களையும் தேர்ந்தெடு
    }
   *[message] { $count ->
        [one] { $count } மெசேஜைத் தேர்ந்தெடு
       *[other] { $count } மெசேஜ்களையும் தேர்ந்தெடு
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder } இல் உள்ள { $count } உரையாடலைத் தேர்ந்தெடு
       *[other] { $folder } இல் உள்ள { $count } உரையாடல்களையும் தேர்ந்தெடு
    }
   *[message] { $count ->
        [one] { $folder } இல் உள்ள { $count } மெசேஜைத் தேர்ந்தெடு
       *[other] { $folder } இல் உள்ள { $count } மெசேஜ்களையும் தேர்ந்தெடு
    }
}
list-clear-selection = தேர்வை அழி

## Mail list: empty states

list-empty-search = உங்கள் தேடலுடன் பொருந்தும் மெசேஜ்கள் எதுவுமில்லை.
list-empty-tab = { $tab } இல் அஞ்சல் எதுவுமில்லை.
list-empty-tab-unknown = இந்தத் தாவலில் அஞ்சல் எதுவுமில்லை.
list-empty-folder = { $folder } இல் மெசேஜ்கள் எதுவுமில்லை.
list-empty-folder-unknown = இந்த ஃபோல்டரில் மெசேஜ்கள் எதுவுமில்லை.
list-first-sync = உங்கள் அஞ்சலைப் பெறுகிறது…
list-first-sync-detail = அஞ்சல் வர வர இங்கே காட்டப்படும்.

## Mail list: lines

row-removed = இந்த மெசேஜ் அகற்றப்பட்டது.
row-starred = நட்சத்திரமிட்டது
row-not-starred = நட்சத்திரமிடவில்லை
row-important = முக்கியமானது. முக்கியமில்லாதது எனக் குறிக்கக் கிளிக் செய்யவும்.
row-mark-important = முக்கியமானது எனக் குறி
row-pinned = மேலே பின் செய்யப்பட்டது
row-pin = மேலே பின் செய்
row-unpin = பின்னை அகற்று

## Mail list: More menu and right-click menu

menu-reply = பதிலளி
menu-reply-all = அனைவருக்கும் பதிலளி
menu-forward = முன்னனுப்பு
menu-archive = காப்பகப்படுத்து
menu-delete = நீக்கு
menu-spam = ஸ்பேம் எனப் புகாரளி
menu-mark-read = படித்ததாகக் குறி
menu-mark-unread = படிக்காததாகக் குறி
menu-mark-all-read = அனைத்தையும் படித்ததாகக் குறி
menu-star = நட்சத்திரமிடு
menu-unstar = நட்சத்திரத்தை அகற்று
menu-important = முக்கியமானது எனக் குறி
menu-not-important = முக்கியமில்லாதது எனக் குறி
menu-pin = மேலே பின் செய்
menu-unpin = பின்னை அகற்று
menu-print-all = அனைத்தையும் அச்சிடு
menu-new-window = புதிய சாளரத்தில் திற
menu-move-to = இதற்கு நகர்த்து
menu-move-to-heading = இதற்கு நகர்த்து:
menu-find-from = { $name } அனுப்பிய மின்னஞ்சல்களைக் கண்டறி

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] உரையாடல் காப்பகப்படுத்தப்பட்டது.
       *[other] { $count } உரையாடல்கள் காப்பகப்படுத்தப்பட்டன.
    }
   *[message] { $count ->
        [one] மெசேஜ் காப்பகப்படுத்தப்பட்டது.
       *[other] { $count } மெசேஜ்கள் காப்பகப்படுத்தப்பட்டன.
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] உரையாடல் நீக்கியவை ஃபோல்டருக்கு நகர்த்தப்பட்டது.
       *[other] { $count } உரையாடல்கள் நீக்கியவை ஃபோல்டருக்கு நகர்த்தப்பட்டன.
    }
   *[message] { $count ->
        [one] மெசேஜ் நீக்கியவை ஃபோல்டருக்கு நகர்த்தப்பட்டது.
       *[other] { $count } மெசேஜ்கள் நீக்கியவை ஃபோல்டருக்கு நகர்த்தப்பட்டன.
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] உரையாடல் நகர்த்தப்பட்டது.
       *[other] { $count } உரையாடல்கள் நகர்த்தப்பட்டன.
    }
   *[message] { $count ->
        [one] மெசேஜ் நகர்த்தப்பட்டது.
       *[other] { $count } மெசேஜ்கள் நகர்த்தப்பட்டன.
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] உரையாடல் நட்சத்திரமிடப்பட்டது.
       *[other] { $count } உரையாடல்கள் நட்சத்திரமிடப்பட்டன.
    }
   *[message] { $count ->
        [one] மெசேஜ் நட்சத்திரமிடப்பட்டது.
       *[other] { $count } மெசேஜ்கள் நட்சத்திரமிடப்பட்டன.
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] உரையாடலிலிருந்து நட்சத்திரம் அகற்றப்பட்டது.
       *[other] { $count } உரையாடல்களிலிருந்து நட்சத்திரம் அகற்றப்பட்டது.
    }
   *[message] { $count ->
        [one] மெசேஜிலிருந்து நட்சத்திரம் அகற்றப்பட்டது.
       *[other] { $count } மெசேஜ்களிலிருந்து நட்சத்திரம் அகற்றப்பட்டது.
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] உரையாடல் முக்கியமானது எனக் குறிக்கப்பட்டது.
       *[other] { $count } உரையாடல்கள் முக்கியமானவை எனக் குறிக்கப்பட்டன.
    }
   *[message] { $count ->
        [one] மெசேஜ் முக்கியமானது எனக் குறிக்கப்பட்டது.
       *[other] { $count } மெசேஜ்கள் முக்கியமானவை எனக் குறிக்கப்பட்டன.
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] உரையாடல் முக்கியமில்லாதது எனக் குறிக்கப்பட்டது.
       *[other] { $count } உரையாடல்கள் முக்கியமில்லாதவை எனக் குறிக்கப்பட்டன.
    }
   *[message] { $count ->
        [one] மெசேஜ் முக்கியமில்லாதது எனக் குறிக்கப்பட்டது.
       *[other] { $count } மெசேஜ்கள் முக்கியமில்லாதவை எனக் குறிக்கப்பட்டன.
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] உரையாடல் மேலே பின் செய்யப்பட்டது.
       *[other] { $count } உரையாடல்கள் மேலே பின் செய்யப்பட்டன.
    }
   *[message] { $count ->
        [one] மெசேஜ் மேலே பின் செய்யப்பட்டது.
       *[other] { $count } மெசேஜ்கள் மேலே பின் செய்யப்பட்டன.
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] உரையாடலின் பின் அகற்றப்பட்டது.
       *[other] { $count } உரையாடல்களின் பின் அகற்றப்பட்டது.
    }
   *[message] { $count ->
        [one] மெசேஜின் பின் அகற்றப்பட்டது.
       *[other] { $count } மெசேஜ்களின் பின் அகற்றப்பட்டது.
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] உரையாடல் ஸ்பேம் எனப் புகாரளிக்கப்பட்டது.
       *[other] { $count } உரையாடல்கள் ஸ்பேம் எனப் புகாரளிக்கப்பட்டன.
    }
   *[message] { $count ->
        [one] மெசேஜ் ஸ்பேம் எனப் புகாரளிக்கப்பட்டது.
       *[other] { $count } மெசேஜ்கள் ஸ்பேம் எனப் புகாரளிக்கப்பட்டன.
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] உரையாடல் நிரந்தரமாக நீக்கப்பட்டது.
       *[other] { $count } உரையாடல்கள் நிரந்தரமாக நீக்கப்பட்டன.
    }
   *[message] { $count ->
        [one] மெசேஜ் நிரந்தரமாக நீக்கப்பட்டது.
       *[other] { $count } மெசேஜ்கள் நிரந்தரமாக நீக்கப்பட்டன.
    }
}
toast-undone = செயல் செயல்தவிர்க்கப்பட்டது.
toast-undo = செயல்தவிர்
toast-no-spam-folder = இந்தக் கணக்கில் ஸ்பேம் ஃபோல்டர் இல்லை.

## Reading pane: toolbar

reader-close = மூடு
reader-back = பின்செல்
reader-mark-unread = படிக்காததாகக் குறி
reader-move-to = இதற்கு நகர்த்து
reader-more = மேலும்
reader-print-all = அனைத்தையும் அச்சிடு
reader-new-window = புதிய சாளரத்தில்
reader-position = { $total } இல் { $position }
reader-newer = புதியது
reader-older = பழையது

## Reading pane: the conversation

reader-removed = இந்த உரையாடல் அகற்றப்பட்டது.
reader-no-subject = (பொருள் இல்லை)
reader-collapse-all = அனைத்தையும் சுருக்கு
reader-expand-all = அனைத்தையும் விரி
reader-unknown-sender = (அறியாத அனுப்புநர்)
reader-date-ago = { $date } ({ $ago })
reader-me = எனக்கு
reader-to = பெறுநர்: { $names }
reader-starred = நட்சத்திரமிட்டது
reader-not-starred = நட்சத்திரமிடவில்லை
reader-too-long = மெசேஜ் மிக நீளமாக இருப்பதால் முழுவதுமாகக் காட்ட முடியாது.
reader-encrypted-images = என்க்ரிப்ட் செய்யப்பட்ட அஞ்சலில் இணையப் படங்கள் ஒருபோதும் ஏற்றப்படாது.
reader-window-failed = புதிய சாளரத்தைத் திறக்க முடியவில்லை.

## Reading pane: message details (opened from "to me")

reader-details-from = அனுப்புநர்:
reader-details-to = பெறுநர்:
reader-details-cc = cc:
reader-details-date = தேதி:
reader-details-subject = பொருள்:

## Reading pane: downloading a message

reader-downloading = இந்த மெசேஜை சர்வரிலிருந்து பதிவிறக்குகிறது…
reader-download-failed = இந்த மெசேஜைப் பதிவிறக்க முடியவில்லை.
reader-try-again = மீண்டும் முயல்க

## Reply row

reply-reply = பதிலளி
reply-reply-all = அனைவருக்கும் பதிலளி
reply-forward = முன்னனுப்பு

## Encrypted and signed mail

security-decrypting = டீக்ரிப்ட் செய்கிறது…
security-checking = கையொப்பம் சரிபார்க்கப்படுகிறது…
security-partly-encrypted = இந்த மெசேஜின் ஒரு பகுதி மட்டுமே என்க்ரிப்ட் செய்யப்பட்டுள்ளது. மீதி பாதுகாப்புக்கு வெளியே சேர்க்கப்பட்டது, யார் வேண்டுமானாலும் அனுப்பியிருக்கலாம்.
security-partly-signed = இந்த மெசேஜின் ஒரு பகுதி மட்டுமே கையொப்பமிடப்பட்டுள்ளது. மீதி பாதுகாப்புக்கு வெளியே சேர்க்கப்பட்டது, யார் வேண்டுமானாலும் அனுப்பியிருக்கலாம்.
security-encrypted = என்க்ரிப்ட் செய்யப்பட்ட மெசேஜ்
security-encrypted-smime = என்க்ரிப்ட் செய்யப்பட்ட மெசேஜ் (S/MIME)
security-no-key = இந்த மெசேஜை டீக்ரிப்ட் செய்ய முடியாது: உங்களிடம் இல்லாத ஒரு கீக்காக இது என்க்ரிப்ட் செய்யப்பட்டுள்ளது.
security-cancelled = டீக்ரிப்ட் செய்வது ரத்துசெய்யப்பட்டது.
security-damaged = இந்த மெசேஜை டீக்ரிப்ட் செய்ய முடியாது: என்க்ரிப்ட் செய்யப்பட்ட தரவு சேதமடைந்துள்ளது அல்லது மாற்றப்பட்டுள்ளது.
security-decrypt-unavailable = இந்த மெசேஜை டீக்ரிப்ட் செய்ய முடியாது: என்க்ரிப்ட் செய்யப்பட்ட அஞ்சலைப் படிக்க { $tool } ஐ நிறுவவும்.
security-decrypt-failed = இந்த மெசேஜை டீக்ரிப்ட் செய்ய முடியாது: { $reason }
security-unknown-signer = அறியாத கையொப்பமிட்டவர்
security-signed-verified = { $signer } கையொப்பமிட்டது · சரிபார்க்கப்பட்டது
security-signed-not-sender = { $signer } கையொப்பமிட்டது, இவர் அனுப்புநர் அல்ல
security-signed-untrusted = { $signer } கையொப்பமிட்டது, நீங்கள் நம்பகமற்றது எனக் குறித்த கீயைக் கொண்டு
security-signed-unverified = { $signer } கையொப்பமிட்டது · கீ சரிபார்க்கப்படவில்லை
security-bad-signature = தவறான கையொப்பம்: கையொப்பமிட்ட பிறகு இந்த மெசேஜ் மாற்றப்பட்டுள்ளது, அல்லது கையொப்பம் போலியானது.
security-signature-expired = { $signer } கையொப்பமிட்டது · கையொப்பம் காலாவதியாகிவிட்டது
security-key-expired = { $signer } கையொப்பமிட்டது · அதன் பிறகு கீ காலாவதியாகிவிட்டது
security-key-revoked = { $signer } கையொப்பமிட்டது, திரும்பப்பெறப்பட்ட கீயைக் கொண்டு
security-missing-key = உங்களிடம் இல்லாத கீயைக் கொண்டு கையொப்பமிடப்பட்டுள்ளது, எனவே சரிபார்க்க முடியாது
security-missing-key-id = உங்களிடம் இல்லாத கீயைக் ({ $key }) கொண்டு கையொப்பமிடப்பட்டுள்ளது, எனவே சரிபார்க்க முடியாது
security-signature-unavailable = கையொப்பமிடப்பட்டது; கையொப்பத்தைச் சரிபார்க்க { $tool } ஐ நிறுவவும்
security-signature-error = கையொப்பத்தைச் சரிபார்க்க முடியவில்லை.

## Remote images and pictures

remote-hidden = இந்த மெசேஜில் உள்ள படங்கள் மறைக்கப்பட்டுள்ளன.
remote-show = படங்களைக் காட்டு
remote-always-show = இந்த அனுப்புநரிடமிருந்து எப்போதும் காட்டு
remote-picture-use = பயன்படுத்து
remote-picture-too-big = 8 MB அல்லது அதற்குக் குறைவான படத்தைத் தேர்ந்தெடுக்கவும்.
remote-picture-type = PNG, JPEG, GIF, WebP அல்லது SVG படத்தைத் தேர்ந்தெடுக்கவும்.
remote-picture-read-failed = படத்தைப் படிக்க முடியவில்லை: { $error }
remote-picture-keep-failed = படத்தைச் சேமிக்க முடியவில்லை: { $error }
remote-picture-remove-failed = படத்தை அகற்ற முடியவில்லை: { $error }

## Attachments

attachment-count = { $count ->
    [one] ஒரு இணைப்பு
   *[other] { $count } இணைப்புகள்
}
attachment-save = சேமி
attachment-save-all = அனைத்தையும் சேமி
attachment-save-all-tooltip = எல்லா இணைப்புகளையும் ஒரு ஃபோல்டரில் சேமி
attachment-save-here = இங்கே சேமி
attachment-not-downloaded = இந்த மெசேஜ் பதிவிறக்கப்படவில்லை.
attachment-not-found = இந்த இணைப்பு மெசேஜில் கிடைக்கவில்லை.
attachment-read-failed = { $name } ஐப் படிக்க முடியவில்லை
attachment-numbered = இணைப்பு { $number }
attachment-saved-all = { $count ->
    [one] { $count } ஃபைல் { $place } இல் சேமிக்கப்பட்டது
   *[other] { $count } ஃபைல்கள் { $place } இல் சேமிக்கப்பட்டன
}
attachment-saved-some = { $total ->
    [one] { $total } ஃபைலில் { $saved } { $place } இல் சேமிக்கப்பட்டது. { $failed } ஐச் சேமிக்க முடியவில்லை
   *[other] { $total } ஃபைல்களில் { $saved } { $place } இல் சேமிக்கப்பட்டன. { $failed } ஐச் சேமிக்க முடியவில்லை
}
attachment-saved-to = { $path } இல் சேமிக்கப்பட்டது
attachment-save-failed = { $name } ஐச் சேமிக்க முடியவில்லை: { $error }
attachment-open-failed = { $name } ஐத் திறக்க முடியவில்லை: { $error }
attachment-risky = இந்த ஃபைல் ஒரு புரோகிராமை இயக்கக்கூடும், எனவே Katna இதைத் திறக்காது. பதிலாக இதைச் சேமிக்கவும்.
attachment-encrypted-open = இந்த ஃபைல் என்க்ரிப்ட் செய்யப்பட்டு வந்தது. வேறு இடத்தில் திறக்க இதைச் சேமிக்கவும்.

## Printing

print-failed = அச்சிட முடியவில்லை: { $error }
print-no-font = எழுத்துரு எதுவும் கிடைக்கவில்லை
print-opened-as-pdf = அங்கிருந்து அச்சிட PDF ஆகத் திறக்கப்பட்டது.
print-not-downloaded = (இன்னும் பதிவிறக்கப்படவில்லை.)
print-encrypted = (என்க்ரிப்ட் செய்யப்பட்டது. இதன் உரையை அச்சிட Katna Mail இல் திறக்கவும்.)
print-to = பெறுநர்: { $addresses }
print-cc = Cc: { $addresses }
