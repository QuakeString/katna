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

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = இணைப்புகளைப் படிக்க இந்த மெசேஜைத் திறக்கவும்.
text-copy = நகலெடு
text-select-all = அனைத்தையும் தேர்ந்தெடு

## Settings page: its tabs

settings-tab-general = பொது
settings-tab-inbox = இன்பாக்ஸ்
settings-tab-accounts = கணக்குகள்
settings-tab-subscriptions = சந்தாக்கள்
settings-tab-appearance = தோற்றம்
settings-tab-shortcuts = ஷார்ட்கட்கள்
settings-tab-default-apps = இயல்புநிலை ஆப்ஸ்
settings-tab-folders-rules = ஃபோல்டர்கள் & விதிகள்
settings-tab-compose = எழுதுதல்
settings-tab-mcp-server = MCP சர்வர்
settings-tab-feedback = பயனர் கருத்து
settings-tab-experimental = சோதனை அம்சங்கள்

## Settings page: tabs still to come

settings-tab-subscriptions-coming = நீங்கள் பெறும் செய்திமடல்களையும் அஞ்சல் பட்டியல்களையும் பார்த்து, ஒரே கிளிக்கில் குழுவிலகலாம்.
settings-tab-folders-rules-coming = ஃபோல்டர்களையும் லேபிள்களையும் உருவாக்கலாம், மறுபெயரிடலாம், நகர்த்தலாம், மறைக்கலாம், எவை ஒத்திசைய வேண்டும் என்பதைத் தேர்வுசெய்யலாம். விதிகள் அனுப்புநர், பொருள் அல்லது சொற்களின்படி புதிய அஞ்சலைத் தானாகவே வரிசைப்படுத்தும், லேபிளிடும், முன்னனுப்பும் அல்லது நீக்கும்.
settings-tab-mcp-server-coming = இந்தக் கணினியில் உள்ள AI உதவியாளர்கள் உங்கள் ஒப்புதலுடன் உங்கள் அஞ்சலைத் தேட, படிக்க, வரைவு எழுத அனுமதிக்கலாம்.

## Settings > General

settings-general-conversations = உரையாடல் காட்சி
settings-general-conversations-group = ஒரே அஞ்சலுக்கான பதில்களைக் குழுவாக்கு
settings-general-conversations-group-detail = பட்டியலில் ஓர் உரையாடலுக்கு ஒரு வரி
settings-general-reading = படித்தல்
settings-general-newest-first = புதிய மெசேஜ் முதலில்
settings-general-newest-first-detail = உரையாடல் அதன் சமீபத்திய பதிலுடன் தொடங்கும்
settings-general-full-headers = முழுத் தலைப்புகளைக் காட்டு
settings-general-full-headers-detail = ஒவ்வொரு மெசேஜிலும் அனுப்புநர், பெறுநர், cc, தேதி, பொருள் ஆகியவை திறந்தே இருக்கும்
settings-general-full-names = பெறுநர்களின் முழுப் பெயர்கள்
settings-general-full-names-detail = “எனக்கு, Ada” என்பதற்குப் பதிலாக “எனக்கு, Ada Lovelace”
settings-general-mark-read = படித்ததாகக் குறித்தல்
settings-general-mark-read-now = திறந்தவுடன்
settings-general-mark-read-1s = 1 விநாடி திறந்திருந்த பிறகு
settings-general-mark-read-3s = 3 விநாடிகள் திறந்திருந்த பிறகு
settings-general-mark-read-never = நான் குறிக்கும்போது மட்டும்
settings-general-reply-button = பதில் பட்டன்
settings-general-reply-all = அனைவருக்கும் பதிலளி
settings-general-reply-all-detail = ஒவ்வொரு மெசேஜுக்கும் அருகிலுள்ள பதில் பட்டன் அனுப்புநருக்கு மட்டுமல்லாமல் அனைவருக்கும் பதிலளிக்கும்
settings-general-remote-images = இணையத்திலிருந்து படங்கள்
settings-general-remote-images-detail = ஒரு மெசேஜின் படங்களை ஏற்றினால், நீங்கள் அதைத் திறந்ததையும், எப்போது, தோராயமாக எங்கிருந்து என்பதையும் அனுப்புநர் அறிந்துகொள்வார். முடக்கத்தில் இருந்தால், ஒவ்வொரு மெசேஜும் முதலில் கேட்கும்; ஓர் அனுப்புநரின் படங்களை எப்போது வேண்டுமானாலும் காட்டலாம்.
settings-general-remote-images-always = படங்களை எப்போதும் காட்டு
settings-general-remote-images-always-detail = நீங்கள் நம்பும் அனுப்புநர்களிடமிருந்து மட்டுமல்ல, ஒவ்வொரு மெசேஜிலும்
settings-general-sending = அனுப்புதல்
settings-general-sending-detail = அனுப்பிய மெசேஜைத் திரும்பப் பெற முடியும்படி, அது எவ்வளவு நேரம் காத்திருக்கும்.
settings-general-offline = ஆஃப்லைன் அஞ்சல்
settings-general-offline-detail = இணைப்பு இல்லாமல் படிக்க, சமீபத்திய அஞ்சல் முழுமையாகப் பதிவிறக்கப்படும். பழைய அஞ்சல் நீங்கள் திறக்கும்போது பதிவிறக்கப்படும்.
settings-general-offline-days = { $count ->
    [one] { $count } நாள்
   *[other] { $count } நாட்கள்
}
settings-general-offline-years = { $count ->
    [one] { $count } ஆண்டு
   *[other] { $count } ஆண்டுகள்
}
settings-general-offline-all = எல்லா அஞ்சல்களும்
settings-general-offline-note = குறைவான நாட்களைத் தேர்ந்தெடுத்தாலும், ஏற்கெனவே பதிவிறக்கிய அஞ்சல் அப்படியே இருக்கும். சர்வரில் எதுவும் மாறாது.
settings-general-notifications = அறிவிப்புகள்
settings-general-notifications-detail = இன்பாக்ஸில் வரும் புதிய அஞ்சலுக்கு, Katna Mail மூடியிருக்கும்போதும்.
settings-general-new-mail = புதிய அஞ்சல் குறித்து எனக்கு அறிவி
settings-general-new-mail-detail = அனைவருக்கும் பதிலளி, படித்ததாகக் குறி, காப்பகப்படுத்து ஆகியவற்றுடன்
settings-general-new-mail-sound = ஒலியை இயக்கு
settings-general-new-mail-sound-detail = டெஸ்க்டாப்பின் புதிய அஞ்சல் ஒலி
settings-general-desktop = டெஸ்க்டாப்
settings-general-open-at-login = உள்நுழையும்போது Katna Mail ஐத் திற
settings-general-open-at-login-detail = சேவை இயங்கும் வரை, எப்படியிருந்தாலும் உள்நுழையும்போது அஞ்சல் ஒத்திசைக்கப்படும்
settings-general-tray = சிஸ்டம் ட்ரேயில் Katna ஐக் காட்டு
settings-general-tray-detail = படிக்காதவற்றின் எண்ணிக்கையுடனும் ஒரு மெனுவுடனும்
settings-general-unread-badge = டாஸ்க்பார் ஐகானில் படிக்காதவற்றின் எண்ணிக்கை
settings-general-unread-badge-detail = இன்பாக்ஸில் எத்தனை மெசேஜ்கள் படிக்கப்படவில்லை

## Settings > Inbox

settings-inbox-tabs = இன்பாக்ஸ் தாவல்கள்
settings-inbox-tabs-detail = உங்கள் அஞ்சல் வழங்குநரின் இணையதளத்தைப் போலவே, இன்பாக்ஸைத் தாவல்களாகப் பிரிக்கும்.
settings-inbox-tabs-show = இன்பாக்ஸ் தாவல்களைக் காட்டு
settings-inbox-tabs-show-detail = முடக்கினால், ஒவ்வொரு கணக்குக்கும் ஒரே பட்டியல்
settings-inbox-no-accounts = தாவல்களைத் தேர்வுசெய்ய ஒரு கணக்கைச் சேர்க்கவும்.
settings-inbox-tabs-automatic = தானியங்கு: { $tabs } ({ $provider })
settings-inbox-tabs-off = தாவல்கள் இல்லை
settings-inbox-tabs-gmail = முதன்மை, விளம்பரங்கள், சமூகம், புதுப்பிப்புகள், மன்றங்கள்
settings-inbox-tabs-focused = கவனத்திற்குரியவை, மற்றவை
settings-inbox-tabs-zoho = இன்பாக்ஸ், செய்திமடல்கள், அறிவிப்புகள்
settings-inbox-tabs-shown = காட்டப்படும் தாவல்கள். நீங்கள் முடக்கும் தாவலின் அஞ்சல் { $tab } இல் இருக்கும்.

## Settings > Appearance

settings-appearance-reading-pane = படிக்கும் பலகம்
settings-appearance-reading-pane-detail = திறந்த உரையாடல் எங்கே காட்டப்படும்.
settings-appearance-pane-right = பட்டியலின் வலதுபுறம்
settings-appearance-pane-none = பிரிப்பு இல்லை
settings-appearance-density = அடர்த்தி
settings-appearance-density-default = இயல்புநிலை
settings-appearance-density-compact = கச்சிதமானது
settings-appearance-scaling = அளவிடுதல்
settings-appearance-scaling-detail = டெஸ்க்டாப்பின் சொந்த அளவுக்கு மேல், Katna Mail இல் உள்ள அனைத்தையும் பெரிதாகவோ சிறிதாகவோ ஆக்கும்: உரை, ஐகான்கள், இடைவெளி, பிரிப்புக் கோடுகள். நீங்கள் அனுப்பும் அஞ்சல் அதன் சொந்த எழுத்துரு அளவைக் கொண்டிருக்கும். மிகச் சிறிய அளவுகளில் ஐகான்களைக் கிளிக் செய்வது கடினமாகலாம்.
settings-appearance-theme = தீம்
settings-appearance-theme-system = டெஸ்க்டாப்பைப் போலவே
settings-appearance-theme-light = லைட்
settings-appearance-theme-dark = டார்க்
settings-appearance-desktop-colors = டெஸ்க்டாப் வண்ணங்கள்
settings-appearance-desktop-colors-use = டெஸ்க்டாப்பின் வண்ணங்களைப் பயன்படுத்து
settings-appearance-desktop-colors-use-detail = டெஸ்க்டாப்பின் வண்ணத் திட்டமும் அக்சென்ட் வண்ணமும்
settings-appearance-app-names = ஆப்ஸ் பெயர்கள்
settings-appearance-app-names-show = ஆப்ஸ் பெயர்களைக் காட்டு
settings-appearance-app-names-show-detail = இடது ஓரத்தில் உள்ள ஆப்ஸ் ஐகான்களுக்குக் கீழே பெயர்கள்
settings-appearance-sender-pictures = அனுப்புநர் படங்கள்
settings-appearance-sender-pictures-show = நிறுவன லோகோக்களைக் காட்டு
settings-appearance-sender-pictures-show-detail = மெசேஜைக் கொண்டு ஒருபோதும் அல்ல, அனுப்புநரின் டொமைனைக் கொண்டு தேடப்பட்டு, ஒரு வாரத்துக்கு வைத்திருக்கப்படும்
settings-appearance-important = முக்கியக் குறிப்பான்கள்
settings-appearance-important-show = முக்கியக் குறிப்பான்களைக் காட்டு
settings-appearance-important-show-detail = பட்டியலில் ஒவ்வொரு மெசேஜுக்கும் அருகில்
settings-appearance-message-width = மெசேஜ் அகலம்
settings-appearance-message-width-limit = மெசேஜ்களின் அகலத்தைக் கட்டுப்படுத்து
settings-appearance-message-width-limit-detail = அகலமான சாளரத்தில் நீண்ட வரிகளைப் படிப்பது எளிதாகும்
settings-appearance-mail-colors = அஞ்சல் வண்ணங்கள்
settings-appearance-mail-colors-detail = பெரும்பாலான அஞ்சல்கள் வெள்ளைப் பக்கத்துக்காக வடிவமைக்கப்பட்டவை. டார்க் தீமில், அவற்றின் வண்ணங்கள் நன்றாகப் படிக்கக்கூடிய அடர் வண்ணங்களாக மாற்றப்படும்; முடக்கினால், வெளிர் பக்கத்தில் அனுப்புநரின் வண்ணங்களே இருக்கும்.
settings-appearance-dark-mail = அஞ்சலுக்கும் அடர் வண்ணங்கள்
settings-appearance-dark-mail-detail = தீம் டார்க்காக இருக்கும்போது மட்டும்
settings-appearance-attachment-previews = இணைப்பு முன்னோட்டங்கள்
settings-appearance-attachment-previews-show = இணைப்புகளின் முன்னோட்டங்களைக் காட்டு
settings-appearance-attachment-previews-show-detail = ஒவ்வொரு ஃபைலின் உள்ளடக்கத்தின் சிறிய படம், அதன் கார்டில்

## Settings > Default apps

settings-default-apps-intro = இணைப்புகளைக் கிளிக் செய்யும்போது அவை எங்கே திறக்கும். வியூவரிலிருந்து ஒரு ஃபைலை வேறொரு ஆப்ஸிலும் எப்போதும் திறக்கலாம். டெஸ்க்டாப்பின் இயல்புநிலை ஆப்ஸ் அதன் சொந்த அமைப்புகளில் அமைக்கப்படும்.
settings-default-apps-pdf = PDF ஃபைல்கள்
settings-default-apps-pdf-detail = பக்கங்கள், பெரிதாக்கும் வசதியுடன்.
settings-default-apps-pictures = படங்கள்
settings-default-apps-pictures-detail = புகைப்படங்கள் (நேராகத் திருப்பப்பட்டவை), PNG, GIF, WebP, BMP, TIFF, SVG.
settings-default-apps-text = உரை ஃபைல்கள்
settings-default-apps-text-detail = வெற்று உரை, பதிவுகள், நிரல் குறியீடு, பிற உரை.
settings-default-apps-sheets = விரிதாள்கள்
settings-default-apps-sheets-detail = Excel (xlsx, xls), OpenDocument (ods), CSV.
settings-default-apps-documents = ஆவணங்கள்
settings-default-apps-documents-detail = Word (docx), OpenDocument உரை (odt).
settings-default-apps-katna = Katna Mail இன் வியூவர்
settings-default-apps-system = டெஸ்க்டாப்பின் இயல்புநிலை ஆப்ஸ்
settings-default-apps-ask = ஒவ்வொரு முறையும் எந்த ஆப்ஸ் எனக் கேள்
settings-default-apps-after-saving = சேமித்த பிறகு
settings-default-apps-show-folder = சேமித்த ஃபைல்களை அவற்றின் ஃபோல்டரில் காட்டு
settings-default-apps-show-folder-detail = சேமித்த இணைப்புகள் தேர்ந்தெடுக்கப்பட்ட நிலையில் ஃபைல் மேனேஜரைத் திறக்கும்

## Settings > Compose

settings-compose-send-from = புதிய மெசேஜ்களை இதிலிருந்து அனுப்பு
settings-compose-send-from-detail = பதில்களும் முன்னனுப்பல்களும் எப்போதும் நீங்கள் இருக்கும் கணக்கிலிருந்தே செல்லும்.
settings-compose-send-from-current = நீங்கள் இருக்கும் கணக்கு
settings-compose-send-on-replies = பதில்களில் அனுப்பு பட்டன்
settings-compose-send-on-replies-detail = பதிலிலோ முன்னனுப்பலிலோ அனுப்பு பட்டன் என்ன செய்யும். அனுப்பு பட்டனுக்கு அருகிலுள்ள மெனுவில் மற்றொன்று இருக்கும்.
settings-compose-send-plain = அனுப்பு
settings-compose-send-archive = அனுப்பிக் காப்பகப்படுத்து
settings-compose-signatures = கையொப்பங்கள்
settings-compose-signatures-detail = உங்கள் மெசேஜுக்குக் கீழே, “--” வரிக்குப் பிறகு சேர்க்கப்படும். எழுதும் சாளரத்தில் வேறொன்றைத் தேர்ந்தெடுக்கலாம்.
settings-compose-untitled = பெயரிடப்படாதது
settings-compose-signature-name = பெயர், எ.கா. பணி
settings-compose-signature-first = எனது கையொப்பம்
settings-compose-signature-numbered = கையொப்பம் { $number }
settings-compose-signature-delete = நீக்கு
settings-compose-signature-deleted = கையொப்பம் நீக்கப்பட்டது
settings-compose-signature-new = புதிதாக உருவாக்கு
settings-compose-no-signatures = இன்னும் கையொப்பங்கள் இல்லை.
settings-compose-no-signature = கையொப்பம் இல்லை
settings-compose-for-new-mail = புதிய அஞ்சலுக்கு
settings-compose-for-replies = பதில்கள், முன்னனுப்பல்களுக்கு
settings-compose-for-replies-detail = நீங்கள் ஒரு மெசேஜில் கையொப்பமிட்ட உரையாடலில், பதில் அதற்குப் பதிலாக அந்தக் கையொப்பத்துடன் தொடங்கும்.
settings-compose-format = வடிவம்
settings-compose-plain-text = வெற்று உரையில் எழுது
settings-compose-plain-text-detail = புதிய அஞ்சல் வடிவமைப்பு இல்லாமல் தொடங்கும்; எழுதும் சாளரத்தில் மாற்றலாம்
settings-compose-spelling = எழுத்துப்பிழை
settings-compose-spell-check = எழுதும்போது எழுத்துப்பிழையைச் சரிபார்
settings-compose-spell-check-detail = தவறான சொற்கள் அடிக்கோடிடப்படும், வலது கிளிக்கில் பரிந்துரைகளுடன்
settings-compose-spell-desktop = டெஸ்க்டாப்பின் மொழி ({ $language })
settings-compose-templates = டெம்ப்ளேட்கள்
settings-compose-templates-detail = அடிக்கடி எழுதும் அஞ்சலைச் சேமித்து, அதிலிருந்து புதிய அஞ்சலையோ பதிலையோ தொடங்குங்கள்.

## Settings > Shortcuts

settings-shortcuts-set = ஷார்ட்கட் தொகுப்பு
settings-shortcuts-set-detail = உங்களுக்குப் பழக்கமான அஞ்சல் ஆப்ஸின் கீகளிலிருந்து தொடங்குங்கள். இங்கே Cmd என்பது Ctrl. உங்கள் சொந்த மாற்றங்கள் தொகுப்பின் மேல் இருக்கும்; இயல்புநிலைகளை மீட்டமை தொகுப்பின் கீகளுக்குத் திரும்பும்.
settings-shortcuts-single = ஒற்றை-கீ ஷார்ட்கட்கள்
settings-shortcuts-single-detail = வெப்மெயிலைப் போல, Ctrl அல்லது Alt இல்லாத கீகள்: e காப்பகப்படுத்தும், j, k நகர்த்தும், / தேடும். இவை பட்டியலிலும் திறந்த உரையாடலிலும் வேலை செய்யும், தட்டச்சு செய்யும்போது ஒருபோதும் இல்லை.
settings-shortcuts-single-use = ஒற்றை-கீ ஷார்ட்கட்களைப் பயன்படுத்து
settings-shortcuts-single-use-detail = Ctrl ஷார்ட்கட்கள் எப்போதும் வேலை செய்யும்
settings-shortcuts-how = ஒரு கீயை மாற்ற அதைக் கிளிக் செய்யவும், அல்லது புதியதைச் சேர்க்க + ஐக் கிளிக் செய்யவும், பின்னர் புதிய கீகளை அழுத்தவும். Esc ரத்துசெய்யும்.
settings-shortcuts-restore = இயல்புநிலைகளை மீட்டமை
settings-shortcuts-no-key = கீ இல்லை
settings-shortcuts-press = கீகளை அழுத்தவும்…
settings-shortcuts-then = { $keys } பிறகு…
settings-shortcuts-moved = { $keys } இப்போது “{ $previous }” என்பதற்குப் பதிலாக “{ $action }” செய்யும்.
settings-shortcuts-single-off = ஒற்றை-கீ ஷார்ட்கட்கள் முடக்கத்தில் உள்ளன, எனவே அவற்றை இயக்கிய பிறகே இந்தக் கீ வேலை செய்யும்.
settings-shortcuts-restored = ஒவ்வொரு ஷார்ட்கட்டும் மீண்டும் அதன் தொகுப்பின் கீகளைப் பெற்றுள்ளது.

## Settings search: the line under a result

settings-general-language-summary = ஆப்ஸ், தேதிகள், எண்கள் ஆகியவற்றின் மொழி
settings-general-reading-summary = புதிய மெசேஜ் முதலில், முழுத் தலைப்புகள், பெறுநர்களின் முழுப் பெயர்கள்
settings-general-mark-read-summary = திறந்த உரையாடல் எப்போது படித்ததாகக் குறிக்கப்படும்: உடனே, 1 அல்லது 3 விநாடிகளுக்குப் பிறகு, அல்லது கைமுறையாக
settings-general-reply-button-summary = ஒவ்வொரு மெசேஜுக்கும் அருகிலுள்ள பதில் பட்டன் அனைவருக்கும் பதிலளிக்கும்
settings-general-remote-images-summary = ஒவ்வொரு மெசேஜின் படங்களையும் எப்போதும் காட்டு
settings-general-sending-summary = அனுப்புவதைச் செயல்தவிர்: அனுப்பிய மெசேஜைத் திரும்பப் பெற முடியும்படி அது எவ்வளவு நேரம் காத்திருக்கும்
settings-general-offline-summary = இணைப்பு இல்லாமல் படிக்க, எத்தனை நாட்களின் சமீபத்திய அஞ்சல் முழுமையாகப் பதிவிறக்கப்படும்
settings-general-notifications-summary = புதிய அஞ்சல் அறிவிப்புகளும் அவற்றின் ஒலியும்
settings-general-desktop-summary = உள்நுழையும்போது Katna Mail ஐத் திறத்தல், சிஸ்டம் ட்ரே ஐகான், டாஸ்க்பார் ஐகானில் படிக்காதவற்றின் எண்ணிக்கை
settings-accounts-accounts-summary = கணக்கைச் சேர்க்கவும் அல்லது அகற்றவும், அல்லது அதன் படத்தை மாற்றவும்
settings-appearance-density-summary = பட்டியலில் இயல்புநிலை அல்லது கச்சிதமான வரிகள்
settings-appearance-scaling-summary = அனைத்தையும் பெரிதாகவோ சிறிதாகவோ ஆக்கு: உரை, ஐகான்கள், இடைவெளி, பிரிப்புக் கோடுகள்
settings-appearance-theme-summary = டெஸ்க்டாப்பைப் போலவே, லைட் அல்லது டார்க்
settings-appearance-sender-pictures-summary = அனுப்புநரின் டொமைனைக் கொண்டு தேடப்படும் நிறுவன லோகோக்கள்
settings-appearance-important-summary = பட்டியலில் ஒவ்வொரு மெசேஜுக்கும் அருகிலுள்ள முக்கியக் குறிப்பான்
settings-appearance-mail-colors-summary = டார்க் தீமில் HTML அஞ்சலுக்கு அடர் வண்ணங்கள், அல்லது அனுப்புநரின் வண்ணங்கள்
settings-appearance-attachment-previews-summary = ஒவ்வொரு இணைப்பின் உள்ளடக்கத்தின் சிறிய படம்
settings-shortcuts-set-summary = Gmail, Inbox by Gmail, Apple Mail, Outlook அல்லது Thunderbird இன் கீகளிலிருந்து தொடங்குங்கள்
settings-shortcuts-single-summary = வெப்மெயிலைப் போல, Ctrl அல்லது Alt இல்லாத கீகள்
settings-default-apps-pdf-summary = PDF இணைப்புகள் எங்கே திறக்கும்
settings-default-apps-pictures-summary = புகைப்படங்களும் படங்களும் எங்கே திறக்கும்
settings-default-apps-text-summary = வெற்று உரை, பதிவுகள், நிரல் குறியீடு எங்கே திறக்கும்
settings-default-apps-sheets-summary = Excel, OpenDocument, CSV ஃபைல்கள் எங்கே திறக்கும்
settings-default-apps-documents-summary = Word, OpenDocument உரை எங்கே திறக்கும்
settings-default-apps-after-saving-summary = சேமித்த இணைப்புகளை அவற்றின் ஃபோல்டரில் காட்டு
settings-compose-send-from-summary = புதிய அஞ்சல் அனுப்பப்படும் கணக்கு: நீங்கள் இருக்கும் கணக்கு, அல்லது எப்போதும் ஒரே கணக்கு
settings-compose-send-on-replies-summary = பதில்களிலும் முன்னனுப்பல்களிலும் அனுப்பு, அல்லது அனுப்பி உரையாடலைக் காப்பகப்படுத்து
settings-compose-signatures-summary = உங்கள் மெசேஜுக்குக் கீழே, “--” வரிக்குப் பிறகு சேர்க்கப்படும்
settings-compose-for-new-mail-summary = புதிய அஞ்சல் தொடங்கும் கையொப்பம்
settings-compose-for-replies-summary = பதில்களும் முன்னனுப்பல்களும் தொடங்கும் கையொப்பம்
settings-compose-format-summary = புதிய அஞ்சலை வெற்று உரையில் எழுது
settings-compose-spelling-summary = எழுதும்போது எழுத்துப்பிழை சரிபார்ப்பு, அகராதியின் மொழி
settings-compose-templates-summary = விரைவில்: அடிக்கடி எழுதும் அஞ்சலைச் சேமித்து, அதிலிருந்து புதிய அஞ்சலையோ பதிலையோ தொடங்குங்கள்
settings-feedback-crash-reports-summary = Katna Mail அல்லது அதன் பின்னணிச் சேவை செயலிழக்கும்போது செயலிழப்பு அறிக்கைகளை இந்தக் கணினியில் சேமி
settings-feedback-saved-summary = இந்தக் கணினியில் சேமித்த செயலிழப்பு அறிக்கைகளைப் பார், நகலெடு அல்லது நீக்கு
settings-feedback-help-improve-summary = என்ன தவறு நடந்தது என்பதைச் சரிசெய்ய உதவ, செயலிழப்பு அறிக்கைகளை அனுப்பு; நீங்கள் இயக்கும் வரை முடக்கத்தில் இருக்கும்
settings-experimental-blur-summary = மேல் பட்டியின் வழியே டெஸ்க்டாப் மங்கலாகத் தெரியும், மெனுக்கள் பனிமூட்டக் கண்ணாடி போல் இருக்கும்
settings-search-shortcut = கீபோர்டு ஷார்ட்கட்
settings-search-tab = அமைப்புகள் தாவல்
settings-search-none = “{ $query }” உடன் பொருந்தும் அமைப்புகள் எதுவுமில்லை.
settings-search-results = “{ $query }” உடன் பொருந்தும் அமைப்புகள்

## Quick settings (the panel that slides in from the right)

quick-title = விரைவு அமைப்புகள்
quick-see-all = எல்லா அமைப்புகளையும் காட்டு
quick-reading-pane = படிக்கும் பலகம்
quick-pane-right = பட்டியலின் வலதுபுறம்
quick-pane-none = பிரிப்பு இல்லை
quick-density = அடர்த்தி
quick-density-default = இயல்புநிலை
quick-density-compact = கச்சிதமானது
quick-theme = தீம்
quick-theme-system = டெஸ்க்டாப்பைப் போலவே
quick-theme-light = லைட்
quick-theme-dark = டார்க்
quick-desktop-colors = டெஸ்க்டாப் வண்ணங்கள்
quick-desktop-colors-detail = டெஸ்க்டாப்பின் வண்ணத் திட்டமும் அக்சென்ட் வண்ணமும்
quick-app-names = ஆப்ஸ் பெயர்கள்
quick-app-names-detail = இடது ஓரத்தில் உள்ள ஆப்ஸ் ஐகான்களுக்குக் கீழே பெயர்கள்
quick-inbox-tabs = இன்பாக்ஸ் தாவல்கள்
quick-inbox-tabs-detail = ஒவ்வொரு கணக்கின் அஞ்சல் வழங்குநரின் தாவல்கள்
quick-choose-tabs = தாவல்களைத் தேர்வுசெய்
quick-choose-tabs-detail = ஒவ்வொரு கணக்குக்கும், அமைப்புகளில்
quick-sending = அனுப்புதல்
quick-undo-send = அனுப்புவதைச் செயல்தவிர்
quick-undo-send-off = முடக்கம்
quick-undo-send-seconds = { $seconds } வி.
quick-signatures = கையொப்பங்கள்
quick-signatures-none = இன்னும் இல்லை
quick-signatures-one = { $name }, இயல்பாகப் பயன்படுத்தப்படும்
quick-signatures-many = { $count ->
    [one] { $count } கையொப்பம்; இயல்பாக { $name }
   *[other] { $count } கையொப்பங்கள்; இயல்பாக { $name }
}
quick-signatures-no-default = { $count ->
    [one] { $count }, இயல்பாக எதுவுமில்லை
   *[other] { $count }, இயல்பாக எதுவுமில்லை
}
quick-signature-untitled = பெயரிடப்படாதது
quick-threading = அஞ்சல் தொடரிழை
quick-conversation-view = உரையாடல் காட்சி
quick-conversation-view-detail = ஒரே அஞ்சலுக்கான பதில்களைக் குழுவாக்கு
quick-help = உதவி
quick-tour = அறிமுகச் சுற்றைத் தொடங்கு
quick-whats-new = புதிதாக என்ன உள்ளது
quick-about = Katna பற்றி

## Settings: opening at login

settings-open-at-login-failed = உள்நுழையும்போது திறப்பதை மாற்ற முடியவில்லை: { $error }

## Settings > Appearance > Scaling

scale-letter = அ
scale-percent = { $percent }%
scale-reset = { $percent }% க்கு மீட்டமை

## Settings > Experimental > Look & Feel

look-intro = இன்னும் சோதிக்கப்பட்டு வரும் அம்சங்கள். இவை மாறலாம் அல்லது நீக்கப்படலாம்.
look-heading = தோற்றமும் உணர்வும்
look-window-frame = சாளரச் சட்டகம்
look-window-frame-detail = தலைப்புப் பட்டி, சாளர பட்டன்கள், மூலைகள், நிழல் ஆகியவற்றை யார் வரைகிறார்கள்.
look-frame-native-kde = நேட்டிவ்: உங்கள் Plasma தீமில் KDE இன் சட்டகம்
look-frame-native = நேட்டிவ்: டெஸ்க்டாப்பின் சட்டகம்
look-frame-katna = Katna: மேல் பட்டியே தலைப்புப் பட்டியாகும்
look-frame-katna-note-named = Katna வட்டமான மூலைகளையும் தன் சொந்த நிழலையும் வரைகிறது. சட்டகம் இனி { $desktop } தீமைப் பின்பற்றாது; சாளர விதிகள் இன்னும் பொருந்தும்.
look-frame-katna-note = Katna வட்டமான மூலைகளையும் தன் சொந்த நிழலையும் வரைகிறது. சட்டகம் இனி டெஸ்க்டாப் தீமைப் பின்பற்றாது; சாளர விதிகள் இன்னும் பொருந்தும்.
look-frame-client-side = உங்கள் டெஸ்க்டாப் சட்டகத்தை ஒவ்வொரு ஆப்ஸிடமும் விட்டுவிடுகிறது, எனவே Katna ஏற்கெனவே தன் சொந்தச் சட்டகத்தை வரைகிறது.
look-blurred-background = மங்கலான பின்னணி
look-blurred-background-detail = மேல் பட்டி, ஃபோல்டர்கள் வழியே டெஸ்க்டாப் மங்கலாகத் தெரியும்; மெனுக்களும் பாப்ஓவர்களும் பனிமூட்டக் கண்ணாடி போல் இருக்கும்.
look-blur = சாளரத்துக்குப் பின்னால் உள்ளதை மங்கலாக்கு
look-blur-detail = அஞ்சல் திடமான கார்டுகளிலேயே இருக்கும், எனவே உரையின் மாறுபாடு குறையாது
look-blur-off-kde = KDE இன் மங்கல் விளைவு முடக்கத்தில் உள்ளது. சிஸ்டம் அமைப்புகள், சாளர நிர்வாகம், டெஸ்க்டாப் விளைவுகள் என்பதில் மங்கல் என்பதை இயக்கி, பின்னர் Katna Mail ஐ மீண்டும் திறக்கவும்.
look-blur-none-gnome = GNOME சாளரங்களுக்குப் பின்னால் உள்ளதை மங்கலாக்குவதில்லை.
look-blur-none-x11 = உங்கள் சாளர மேலாளர் சாளரங்களுக்குப் பின்னால் உள்ளதை மங்கலாக்குவதில்லை.
look-blur-none-wayland = உங்கள் காம்போசிட்டர் சாளரங்களுக்குப் பின்னால் உள்ளதை மங்கலாக்குவதில்லை.

## Settings > User feedback (crash reports)

feedback-intro-sending = என்ன தவறு நடந்தது என்பதைச் சரிசெய்ய உதவ, புதிய செயலிழப்பு அறிக்கைகள் அனுப்பப்படும். வேறு எதுவும் இந்தக் கணினியை விட்டு வெளியேறாது.
feedback-intro-local = Katna எதையும் எங்கும் அனுப்புவதில்லை. செயலிழப்பு அறிக்கைகள் இந்தக் கணினியிலேயே இருக்கும், நீங்கள் பார்க்கவோ பிழை அறிக்கையில் இணைக்கவோ.
feedback-crash-reports = செயலிழப்பு அறிக்கைகள்
feedback-crash-reports-detail = Katna Mail அல்லது அதன் பின்னணிச் சேவை செயலிழக்கும்போது எழுதப்படும்.
feedback-save = செயலிழப்பு அறிக்கைகளை இந்தக் கணினியில் சேமி
feedback-save-detail = உங்கள் ஹோம் ஃபோல்டர், பயனர், கணினிப் பெயர்கள், மின்னஞ்சல் முகவரிகள் ஆகியவை சேர்க்கப்படாது
feedback-saved = சேமித்த செயலிழப்பு அறிக்கைகள்
feedback-saved-detail = { $count ->
    [one] சமீபத்திய { $count } அறிக்கை வைத்திருக்கப்படும்.
   *[other] சமீபத்திய { $count } அறிக்கைகள் வைத்திருக்கப்படும்.
}
feedback-help-improve = Katna ஐ மேம்படுத்த உதவுங்கள்
feedback-help-improve-detail = நீங்கள் இயக்கும் வரை முடக்கத்தில் இருக்கும், எப்போது வேண்டுமானாலும் இங்கே முடக்கலாம்.
feedback-send = செயலிழப்பு அறிக்கைகளை அனுப்பு
feedback-send-detail = சேமித்த அறிக்கை, நீங்கள் இங்கே பார்ப்பது போலவே, Katna இன் செயலிழப்பு டிராக்கருக்கு (Sentry, EU இல்) செல்லும். IP முகவரி, மெசேஜ்கள், மின்னஞ்சல் முகவரிகள் எதுவும் இல்லை
feedback-none-saved = செயலிழப்பு அறிக்கைகள் எதுவும் சேமிக்கப்படவில்லை.
feedback-delete-all = அனைத்தையும் நீக்கு
feedback-app-daemon = பின்னணிச் சேவை
feedback-report-sent = { $date } · அனுப்பப்பட்டது
feedback-view = பார்
feedback-view-tooltip = அறிக்கையைத் திற
feedback-copy-tooltip = பிழை அறிக்கையில் ஒட்ட இதை நகலெடு
feedback-copied = செயலிழப்பு அறிக்கை நகலெடுக்கப்பட்டது.
feedback-deleted-all = செயலிழப்பு அறிக்கைகள் நீக்கப்பட்டன.
feedback-read-failed = செயலிழப்பு அறிக்கையைப் படிக்க முடியவில்லை: { $error }
feedback-delete-failed = செயலிழப்பு அறிக்கையை நீக்க முடியவில்லை: { $error }
feedback-delete-all-failed = செயலிழப்பு அறிக்கைகளை நீக்க முடியவில்லை: { $error }

## Menu bar (the KDE global menu)

desktop-menu-file = _ஃபைல்
desktop-menu-new-message = _புதிய மெசேஜ்
desktop-menu-quit = _வெளியேறு
desktop-menu-edit = _திருத்து
desktop-menu-undo = _செயல்தவிர்
desktop-menu-select-all = _அனைத்தையும் தேர்ந்தெடு
desktop-menu-select-none = _எதையும் தேர்ந்தெடுக்காதே
desktop-menu-find = _கண்டறி…
desktop-menu-view = _காட்சி
desktop-menu-folder-list = _ஃபோல்டர் பட்டியலைக் காட்டு
desktop-menu-refresh = _புதுப்பி
desktop-menu-go = _செல்
desktop-menu-inbox = _இன்பாக்ஸ்
desktop-menu-starred = _நட்சத்திரமிட்டவை
desktop-menu-sent = _அனுப்பியவை
desktop-menu-drafts = _வரைவுகள்
desktop-menu-all-mail = _எல்லா அஞ்சல்களும்
desktop-menu-next = _அடுத்த உரையாடல்
desktop-menu-previous = _முந்தைய உரையாடல்
desktop-menu-message = _மெசேஜ்
desktop-menu-open = _திற
desktop-menu-reply = _பதிலளி
desktop-menu-reply-all = _அனைவருக்கும் பதிலளி
desktop-menu-forward = _முன்னனுப்பு
desktop-menu-archive = _காப்பகப்படுத்து
desktop-menu-delete = _நீக்கு
desktop-menu-spam = _ஸ்பேம் எனப் புகாரளி
desktop-menu-move-to = _இதற்கு நகர்த்து…
desktop-menu-mark-read = _படித்ததாகக் குறி
desktop-menu-mark-unread = _படிக்காததாகக் குறி
desktop-menu-star = _நட்சத்திரமிடு
desktop-menu-important = _முக்கியமானது எனக் குறி
desktop-menu-not-important = _முக்கியமில்லாதது எனக் குறி
desktop-menu-settings = _அமைப்புகள்
desktop-menu-quick-settings = _விரைவு அமைப்புகள்
desktop-menu-configure = _Katna Mail ஐ உள்ளமை…
desktop-menu-help = _உதவி
desktop-menu-shortcuts = _கீபோர்டு ஷார்ட்கட்கள்
desktop-menu-whats-new = _புதிதாக என்ன உள்ளது
desktop-menu-about = _Katna பற்றி

## Settings > Keyboard shortcuts: the groups of the list

shortcut-group-moving = நகர்தல்
shortcut-group-actions = செயல்கள்
shortcut-group-go-to = இதற்குச் செல்
shortcut-group-app = பயன்பாடு

## Settings > Keyboard shortcuts: what each shortcut does

shortcut-next = அடுத்த உரையாடல்
shortcut-previous = முந்தைய உரையாடல்
shortcut-down = பட்டியலில் கீழே நகர்
shortcut-up = பட்டியலில் மேலே நகர்
shortcut-first = பட்டியலில் முதலாவது
shortcut-last = பட்டியலில் கடைசி
shortcut-page-down = பட்டியலில் ஒரு பக்கம் கீழே
shortcut-page-up = பட்டியலில் ஒரு பக்கம் மேலே
shortcut-open = உரையாடலைத் திற
shortcut-back = பட்டியலுக்குத் திரும்பு
shortcut-scroll-down = கீழே உருட்டு
shortcut-scroll-up = மேலே உருட்டு
shortcut-scroll-page-down = ஒரு பக்கம் கீழே உருட்டு
shortcut-scroll-page-up = ஒரு பக்கம் மேலே உருட்டு
shortcut-compose = எழுது
shortcut-reply = பதிலளி
shortcut-reply-all = அனைவருக்கும் பதிலளி
shortcut-forward = முன்னனுப்பு
shortcut-archive = காப்பகப்படுத்து
shortcut-delete = நீக்கு
shortcut-spam = ஸ்பேம் எனப் புகாரளி
shortcut-move-to = இதற்கு நகர்த்து
shortcut-mark-read = படித்ததாகக் குறி
shortcut-mark-unread = படிக்காததாகக் குறி
shortcut-star = நட்சத்திரமிடு அல்லது அகற்று
shortcut-important = முக்கியமானது எனக் குறி
shortcut-not-important = முக்கியமில்லாதது எனக் குறி
shortcut-check = உரையாடலைத் தேர்வுசெய்
shortcut-select-all = எல்லா உரையாடல்களையும் தேர்வுசெய்
shortcut-select-none = எல்லா உரையாடல்களின் தேர்வையும் நீக்கு
shortcut-undo = கடைசிச் செயலைச் செயல்தவிர்
shortcut-go-inbox = இன்பாக்ஸ்
shortcut-go-starred = நட்சத்திரமிட்டவை
shortcut-go-sent = அனுப்பியவை
shortcut-go-drafts = வரைவுகள்
shortcut-go-all = எல்லா அஞ்சல்களும்
shortcut-search = அஞ்சலில் தேடு
shortcut-navigation = மெனுவைக் காட்டு அல்லது சுருக்கு
shortcut-quick-settings = விரைவு அமைப்புகள்
shortcut-settings = எல்லா அமைப்புகளும்
shortcut-shortcuts = கீபோர்டு ஷார்ட்கட்கள்
shortcut-reload = புதிய அஞ்சலைச் சரிபார்
shortcut-quit = வெளியேறு

## Keys pressed one after another, as a shortcut shows them ("G then I")

shortcut-sequence = { $first } பிறகு { $second }

## Settings > Accounts

accounts-folder-pane = ஃபோல்டர் பலகம்
accounts-folder-pane-detail = இடதுபுறப் பலகம் எந்தக் கணக்குகளின் ஃபோல்டர்களைக் காட்டும்.
accounts-shown-one = ஒரு நேரத்தில் ஒரு கணக்கு; கணக்கு கார்டில் மாற்றலாம்
accounts-shown-all = எல்லாக் கணக்குகளும், ஒன்றன்பின் ஒன்றாக
accounts-row = கணக்குகள்
accounts-row-detail = ஒரு கணக்கை அகற்றினால், இந்தக் கணினியில் உள்ள அதன் அஞ்சலின் Katna நகல் நீக்கப்படும். அஞ்சல் சர்வரில் அப்படியே இருக்கும்.
accounts-none = இன்னும் கணக்குகள் இல்லை.
accounts-kind-imported = இம்போர்ட் செய்யப்பட்டது
accounts-picture-reset = டெஸ்க்டாப் படத்தைப் பயன்படுத்து
accounts-picture-change = படத்தை மாற்று
accounts-remove = அகற்று
accounts-delete-all-row = எல்லாத் தரவையும் நீக்கு
accounts-delete-all-row-detail = புதிதாக நிறுவியது போல, மீண்டும் தொடங்குங்கள்.
accounts-delete-all-about = ஒவ்வொரு கணக்கு, சேமித்த எல்லா அஞ்சல்கள், தொடர்புகள், கேலெண்டர்கள், தேடல் அட்டவணை, உங்கள் அமைப்புகள், சேமித்த கடவுச்சொற்கள் ஆகியவற்றை இந்தக் கணினியிலிருந்து நீக்கும். உங்கள் அஞ்சல் சர்வர்களில் எதுவும் மாறாது.
accounts-delete-all-open = எல்லா Katna தரவையும் நீக்கு

## Settings > Accounts: snackbars after deleting

accounts-removed-local = { $address } Katna இலிருந்து அகற்றப்பட்டது.
accounts-removed = { $address } Katna இலிருந்து அகற்றப்பட்டது. அதன் அஞ்சல் இன்னும் சர்வரில் உள்ளது.
accounts-all-deleted = எல்லா Katna தரவும் இந்தக் கணினியிலிருந்து நீக்கப்பட்டது.

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = { $address } ஐ அகற்றவா?
accounts-remove-confirm = கணக்கை அகற்று
accounts-removing = அகற்றுகிறது…
accounts-remove-local-mail = { $folders ->
    [0] இந்தக் கணக்கில் இம்போர்ட் செய்யப்பட்ட எல்லா அஞ்சல்களும்
    [one] இந்தக் கணக்கில் இம்போர்ட் செய்யப்பட்ட, அதன் ஃபோல்டரில் உள்ள எல்லா அஞ்சல்களும்
   *[other] இந்தக் கணக்கில் இம்போர்ட் செய்யப்பட்ட, அதன் { $folders } ஃபோல்டர்களில் உள்ள எல்லா அஞ்சல்களும்
}
accounts-remove-local-settings = அதன் Katna அமைப்புகள்
accounts-remove-mail = { $folders ->
    [0] Katna சேமித்த இந்தக் கணக்கின் எல்லா அஞ்சல்களும்
    [one] Katna அதன் ஃபோல்டரில் சேமித்த இந்தக் கணக்கின் எல்லா அஞ்சல்களும்
   *[other] Katna அதன் { $folders } ஃபோல்டர்களில் சேமித்த இந்தக் கணக்கின் எல்லா அஞ்சல்களும்
}
accounts-remove-outbox = அவுட்பாக்ஸில் காத்திருக்கும் அதன் மெசேஜ்கள்
accounts-remove-settings = அதன் சேமித்த கடவுச்சொல்லும் Katna அமைப்புகளும்
accounts-delete-all-title = எல்லா Katna தரவையும் நீக்கவா?
accounts-delete-all-confirm = அனைத்தையும் நீக்கு
accounts-deleting = நீக்குகிறது…
accounts-delete-all-accounts = ஒவ்வொரு கணக்கும், Katna சேமித்த எல்லா அஞ்சல்களும் இணைப்புகளும்
accounts-delete-all-contacts = தொடர்புகள், கேலெண்டர்கள், தேடல் அட்டவணை
accounts-delete-all-settings = எல்லா அமைப்புகள், கையொப்பங்கள், கீபோர்டு ஷார்ட்கட்கள்
accounts-delete-all-passwords = சேமித்த ஒவ்வொரு கடவுச்சொல்லும்
accounts-deleted-heading = இந்தக் கணினியிலிருந்து நீக்கப்படுபவை:
accounts-cannot-undo = இதைச் செயல்தவிர்க்க முடியாது.
accounts-server-delete-all = உங்கள் அஞ்சல் சர்வர்களில் எதுவும் மாறாது: உங்கள் அஞ்சல் அங்கேயே இருக்கும், கணக்கை மீண்டும் சேர்த்தால் அது மீண்டும் பதிவிறக்கப்படும். ஃபைல்களிலிருந்து இம்போர்ட் செய்த அஞ்சல் Katna இல் மட்டுமே உள்ளது; அந்த ஃபைல்கள் தொடப்படாது.
accounts-server-local = இந்த அஞ்சல் ஃபைல்களிலிருந்து இம்போர்ட் செய்யப்பட்டது, எனவே அதன் ஒரே நகல் Katna இடம் மட்டுமே உள்ளது. அது வந்த ஃபைல்கள் தொடப்படாது; திரும்பப் பெற அவற்றை மீண்டும் இம்போர்ட் செய்யவும்.
accounts-server-remove = அஞ்சல் சர்வரில் எதுவும் மாறாது: உங்கள் அஞ்சல் அங்கேயே இருக்கும், கணக்கை மீண்டும் சேர்த்தால் அது மீண்டும் பதிவிறக்கப்படும்.
accounts-confirm-word = நீக்கு
accounts-confirm-placeholder = “{ accounts-confirm-word }” என டைப் செய்யவும்
accounts-confirm-prompt = உறுதிப்படுத்த, “{ accounts-confirm-word }” என டைப் செய்யவும்:
accounts-cancel = ரத்துசெய்
