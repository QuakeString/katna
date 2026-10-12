# Katna Mail, Tamil (தமிழ்).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

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
tab-provider-other = Katna வரிசைப்படுத்தியது

## Mail list: toolbar

list-select = தேர்ந்தெடு
list-refresh = புதுப்பி
list-back-to-top = மேலே செல்
list-checking = புதிய அஞ்சலைச் சரிபார்க்கிறது…
list-more = மேலும்
list-mark-read = படித்ததாகக் குறி
list-mark-unread = படிக்காததாகக் குறி
list-move-to = இதற்கு நகர்த்து
list-archive = காப்பகப்படுத்து
list-spam = ஸ்பேம் எனப் புகாரளி
list-delete = நீக்கு
list-snooze = உறக்கநிலையில் வை
list-unsnooze = உறக்கநிலையை நீக்கு
list-newer = புதியவை
list-older = பழையவை
list-range = { $total } இல் { $first }–{ $last }
list-range-about = சுமார் { $total } இல் { $first }–{ $last }
list-results = “{ $query }” க்கான முடிவுகள்
list-results-corrected = “{ $query }” க்கான முடிவுகள் காட்டப்படுகின்றன
list-search-instead = அதற்குப் பதிலாக “{ $query }” என்று தேடு
list-search-no-index = தேடல் தயாராக இல்லை: அட்டவணை இன்னும் உருவாக்கப்படவில்லை.
list-search-not-ready = தேடல் தயாராக இல்லை: { $error }
list-files-more = +{ $count }
list-replied = நீங்கள் பதிலளித்தீர்கள்

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
list-selected-picked-screen = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] திரையில் உள்ள படித்த { $count } உரையாடல் தேர்ந்தெடுக்கப்பட்டுள்ளது.
           *[other] திரையில் உள்ள படித்த { $count } உரையாடல்களும் தேர்ந்தெடுக்கப்பட்டுள்ளன.
        }
       *[message] { $count ->
            [one] திரையில் உள்ள படித்த { $count } மெசேஜ் தேர்ந்தெடுக்கப்பட்டுள்ளது.
           *[other] திரையில் உள்ள படித்த { $count } மெசேஜ்களும் தேர்ந்தெடுக்கப்பட்டுள்ளன.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] திரையில் உள்ள படிக்காத { $count } உரையாடல் தேர்ந்தெடுக்கப்பட்டுள்ளது.
           *[other] திரையில் உள்ள படிக்காத { $count } உரையாடல்களும் தேர்ந்தெடுக்கப்பட்டுள்ளன.
        }
       *[message] { $count ->
            [one] திரையில் உள்ள படிக்காத { $count } மெசேஜ் தேர்ந்தெடுக்கப்பட்டுள்ளது.
           *[other] திரையில் உள்ள படிக்காத { $count } மெசேஜ்களும் தேர்ந்தெடுக்கப்பட்டுள்ளன.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] திரையில் உள்ள நட்சத்திரமிட்ட { $count } உரையாடல் தேர்ந்தெடுக்கப்பட்டுள்ளது.
           *[other] திரையில் உள்ள நட்சத்திரமிட்ட { $count } உரையாடல்களும் தேர்ந்தெடுக்கப்பட்டுள்ளன.
        }
       *[message] { $count ->
            [one] திரையில் உள்ள நட்சத்திரமிட்ட { $count } மெசேஜ் தேர்ந்தெடுக்கப்பட்டுள்ளது.
           *[other] திரையில் உள்ள நட்சத்திரமிட்ட { $count } மெசேஜ்களும் தேர்ந்தெடுக்கப்பட்டுள்ளன.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] திரையில் உள்ள நட்சத்திரமிடாத { $count } உரையாடல் தேர்ந்தெடுக்கப்பட்டுள்ளது.
           *[other] திரையில் உள்ள நட்சத்திரமிடாத { $count } உரையாடல்களும் தேர்ந்தெடுக்கப்பட்டுள்ளன.
        }
       *[message] { $count ->
            [one] திரையில் உள்ள நட்சத்திரமிடாத { $count } மெசேஜ் தேர்ந்தெடுக்கப்பட்டுள்ளது.
           *[other] திரையில் உள்ள நட்சத்திரமிடாத { $count } மெசேஜ்களும் தேர்ந்தெடுக்கப்பட்டுள்ளன.
        }
    }
}
list-select-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] படித்த { $count } உரையாடலைத் தேர்ந்தெடு
           *[other] படித்த { $count } உரையாடல்களையும் தேர்ந்தெடு
        }
       *[message] { $count ->
            [one] படித்த { $count } மெசேஜைத் தேர்ந்தெடு
           *[other] படித்த { $count } மெசேஜ்களையும் தேர்ந்தெடு
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] படிக்காத { $count } உரையாடலைத் தேர்ந்தெடு
           *[other] படிக்காத { $count } உரையாடல்களையும் தேர்ந்தெடு
        }
       *[message] { $count ->
            [one] படிக்காத { $count } மெசேஜைத் தேர்ந்தெடு
           *[other] படிக்காத { $count } மெசேஜ்களையும் தேர்ந்தெடு
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] நட்சத்திரமிட்ட { $count } உரையாடலைத் தேர்ந்தெடு
           *[other] நட்சத்திரமிட்ட { $count } உரையாடல்களையும் தேர்ந்தெடு
        }
       *[message] { $count ->
            [one] நட்சத்திரமிட்ட { $count } மெசேஜைத் தேர்ந்தெடு
           *[other] நட்சத்திரமிட்ட { $count } மெசேஜ்களையும் தேர்ந்தெடு
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] நட்சத்திரமிடாத { $count } உரையாடலைத் தேர்ந்தெடு
           *[other] நட்சத்திரமிடாத { $count } உரையாடல்களையும் தேர்ந்தெடு
        }
       *[message] { $count ->
            [one] நட்சத்திரமிடாத { $count } மெசேஜைத் தேர்ந்தெடு
           *[other] நட்சத்திரமிடாத { $count } மெசேஜ்களையும் தேர்ந்தெடு
        }
    }
}
list-select-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $folder } இல் உள்ள படித்த { $count } உரையாடலைத் தேர்ந்தெடு
           *[other] { $folder } இல் உள்ள படித்த { $count } உரையாடல்களையும் தேர்ந்தெடு
        }
       *[message] { $count ->
            [one] { $folder } இல் உள்ள படித்த { $count } மெசேஜைத் தேர்ந்தெடு
           *[other] { $folder } இல் உள்ள படித்த { $count } மெசேஜ்களையும் தேர்ந்தெடு
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $folder } இல் உள்ள படிக்காத { $count } உரையாடலைத் தேர்ந்தெடு
           *[other] { $folder } இல் உள்ள படிக்காத { $count } உரையாடல்களையும் தேர்ந்தெடு
        }
       *[message] { $count ->
            [one] { $folder } இல் உள்ள படிக்காத { $count } மெசேஜைத் தேர்ந்தெடு
           *[other] { $folder } இல் உள்ள படிக்காத { $count } மெசேஜ்களையும் தேர்ந்தெடு
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $folder } இல் உள்ள நட்சத்திரமிட்ட { $count } உரையாடலைத் தேர்ந்தெடு
           *[other] { $folder } இல் உள்ள நட்சத்திரமிட்ட { $count } உரையாடல்களையும் தேர்ந்தெடு
        }
       *[message] { $count ->
            [one] { $folder } இல் உள்ள நட்சத்திரமிட்ட { $count } மெசேஜைத் தேர்ந்தெடு
           *[other] { $folder } இல் உள்ள நட்சத்திரமிட்ட { $count } மெசேஜ்களையும் தேர்ந்தெடு
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $folder } இல் உள்ள நட்சத்திரமிடாத { $count } உரையாடலைத் தேர்ந்தெடு
           *[other] { $folder } இல் உள்ள நட்சத்திரமிடாத { $count } உரையாடல்களையும் தேர்ந்தெடு
        }
       *[message] { $count ->
            [one] { $folder } இல் உள்ள நட்சத்திரமிடாத { $count } மெசேஜைத் தேர்ந்தெடு
           *[other] { $folder } இல் உள்ள நட்சத்திரமிடாத { $count } மெசேஜ்களையும் தேர்ந்தெடு
        }
    }
}
list-selected-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] படித்த { $count } உரையாடல் தேர்ந்தெடுக்கப்பட்டுள்ளது.
           *[other] படித்த { $count } உரையாடல்களும் தேர்ந்தெடுக்கப்பட்டுள்ளன.
        }
       *[message] { $count ->
            [one] படித்த { $count } மெசேஜ் தேர்ந்தெடுக்கப்பட்டுள்ளது.
           *[other] படித்த { $count } மெசேஜ்களும் தேர்ந்தெடுக்கப்பட்டுள்ளன.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] படிக்காத { $count } உரையாடல் தேர்ந்தெடுக்கப்பட்டுள்ளது.
           *[other] படிக்காத { $count } உரையாடல்களும் தேர்ந்தெடுக்கப்பட்டுள்ளன.
        }
       *[message] { $count ->
            [one] படிக்காத { $count } மெசேஜ் தேர்ந்தெடுக்கப்பட்டுள்ளது.
           *[other] படிக்காத { $count } மெசேஜ்களும் தேர்ந்தெடுக்கப்பட்டுள்ளன.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] நட்சத்திரமிட்ட { $count } உரையாடல் தேர்ந்தெடுக்கப்பட்டுள்ளது.
           *[other] நட்சத்திரமிட்ட { $count } உரையாடல்களும் தேர்ந்தெடுக்கப்பட்டுள்ளன.
        }
       *[message] { $count ->
            [one] நட்சத்திரமிட்ட { $count } மெசேஜ் தேர்ந்தெடுக்கப்பட்டுள்ளது.
           *[other] நட்சத்திரமிட்ட { $count } மெசேஜ்களும் தேர்ந்தெடுக்கப்பட்டுள்ளன.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] நட்சத்திரமிடாத { $count } உரையாடல் தேர்ந்தெடுக்கப்பட்டுள்ளது.
           *[other] நட்சத்திரமிடாத { $count } உரையாடல்களும் தேர்ந்தெடுக்கப்பட்டுள்ளன.
        }
       *[message] { $count ->
            [one] நட்சத்திரமிடாத { $count } மெசேஜ் தேர்ந்தெடுக்கப்பட்டுள்ளது.
           *[other] நட்சத்திரமிடாத { $count } மெசேஜ்களும் தேர்ந்தெடுக்கப்பட்டுள்ளன.
        }
    }
}
list-selected-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $folder } இல் உள்ள படித்த { $count } உரையாடல் தேர்ந்தெடுக்கப்பட்டுள்ளது.
           *[other] { $folder } இல் உள்ள படித்த { $count } உரையாடல்களும் தேர்ந்தெடுக்கப்பட்டுள்ளன.
        }
       *[message] { $count ->
            [one] { $folder } இல் உள்ள படித்த { $count } மெசேஜ் தேர்ந்தெடுக்கப்பட்டுள்ளது.
           *[other] { $folder } இல் உள்ள படித்த { $count } மெசேஜ்களும் தேர்ந்தெடுக்கப்பட்டுள்ளன.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $folder } இல் உள்ள படிக்காத { $count } உரையாடல் தேர்ந்தெடுக்கப்பட்டுள்ளது.
           *[other] { $folder } இல் உள்ள படிக்காத { $count } உரையாடல்களும் தேர்ந்தெடுக்கப்பட்டுள்ளன.
        }
       *[message] { $count ->
            [one] { $folder } இல் உள்ள படிக்காத { $count } மெசேஜ் தேர்ந்தெடுக்கப்பட்டுள்ளது.
           *[other] { $folder } இல் உள்ள படிக்காத { $count } மெசேஜ்களும் தேர்ந்தெடுக்கப்பட்டுள்ளன.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $folder } இல் உள்ள நட்சத்திரமிட்ட { $count } உரையாடல் தேர்ந்தெடுக்கப்பட்டுள்ளது.
           *[other] { $folder } இல் உள்ள நட்சத்திரமிட்ட { $count } உரையாடல்களும் தேர்ந்தெடுக்கப்பட்டுள்ளன.
        }
       *[message] { $count ->
            [one] { $folder } இல் உள்ள நட்சத்திரமிட்ட { $count } மெசேஜ் தேர்ந்தெடுக்கப்பட்டுள்ளது.
           *[other] { $folder } இல் உள்ள நட்சத்திரமிட்ட { $count } மெசேஜ்களும் தேர்ந்தெடுக்கப்பட்டுள்ளன.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $folder } இல் உள்ள நட்சத்திரமிடாத { $count } உரையாடல் தேர்ந்தெடுக்கப்பட்டுள்ளது.
           *[other] { $folder } இல் உள்ள நட்சத்திரமிடாத { $count } உரையாடல்களும் தேர்ந்தெடுக்கப்பட்டுள்ளன.
        }
       *[message] { $count ->
            [one] { $folder } இல் உள்ள நட்சத்திரமிடாத { $count } மெசேஜ் தேர்ந்தெடுக்கப்பட்டுள்ளது.
           *[other] { $folder } இல் உள்ள நட்சத்திரமிடாத { $count } மெசேஜ்களும் தேர்ந்தெடுக்கப்பட்டுள்ளன.
        }
    }
}
list-picked-none = { $pick ->
    [read] { $kind ->
        [conversation] இங்கே படித்த உரையாடல்கள் இல்லை.
       *[message] இங்கே படித்த மெசேஜ்கள் இல்லை.
    }
   *[unread] { $kind ->
        [conversation] இங்கே படிக்காத உரையாடல்கள் இல்லை.
       *[message] இங்கே படிக்காத மெசேஜ்கள் இல்லை.
    }
    [starred] { $kind ->
        [conversation] இங்கே நட்சத்திரமிட்ட உரையாடல்கள் இல்லை.
       *[message] இங்கே நட்சத்திரமிட்ட மெசேஜ்கள் இல்லை.
    }
    [unstarred] { $kind ->
        [conversation] இங்கே நட்சத்திரமிடாத உரையாடல்கள் இல்லை.
       *[message] இங்கே நட்சத்திரமிடாத மெசேஜ்கள் இல்லை.
    }
}
list-clear-selection = தேர்வை அழி

## Mail list: empty states

list-empty-search = உங்கள் தேடலுடன் பொருந்தும் மெசேஜ்கள் எதுவுமில்லை.
list-empty-tab = { $tab } இல் அஞ்சல் எதுவுமில்லை.
list-empty-tab-unknown = இந்தத் தாவலில் அஞ்சல் எதுவுமில்லை.
list-empty-folder = { $folder } இல் மெசேஜ்கள் எதுவுமில்லை.
list-empty-folder-unknown = இந்த ஃபோல்டரில் மெசேஜ்கள் எதுவுமில்லை.
list-empty-waiting = பதிலுக்காக எதுவும் காத்திருக்கவில்லை.
list-empty-reminders = நினைவூட்டல்கள் இல்லை. ஒன்றைச் சேர்க்க அஞ்சலில் H ஐ அழுத்துங்கள்.
list-first-sync = உங்கள் அஞ்சலைப் பெறுகிறது…
list-first-sync-detail = அஞ்சல் வர வர இங்கே காட்டப்படும்.
list-store-unreadable = அஞ்சல் சேமிப்பகத்தைத் திறக்க முடியவில்லை

## Mail list: lines

row-no-subject = (பொருள் இல்லை)
row-unknown-sender = (தெரியாத அனுப்புநர்)
row-to = பெறுநர்:
row-no-recipients = (பெறுநர்கள் இல்லை)
row-names-separator = {", "}
row-me = நான்
row-removed = இந்த மெசேஜ் அகற்றப்பட்டது.
row-starred = நட்சத்திரமிட்டது
row-not-starred = நட்சத்திரமிடவில்லை
row-important = முக்கியமானது. முக்கியமில்லாதது எனக் குறிக்கக் கிளிக் செய்யவும்.
row-mark-important = முக்கியமானது எனக் குறி
row-pinned = மேலே பின் செய்யப்பட்டது
row-task = பணி
row-task-open = பணியைத் திற: { $title }
row-tracking-none = கண்காணிக்கப்படுகிறது. இன்னும் திறக்கப்படவில்லை
row-tracking-opened = { $recipients } பேரில் { $opened } பேர் திறந்தனர்
row-tracking-clicked = { $recipients } பேரில் { $opened } பேர் திறந்தனர், { $clicked } பேர் லிங்க்கைத் திறந்தனர்
row-pin = மேலே பின் செய்
row-unpin = பின்னை அகற்று
row-snoozed-until = { $when } வரை உறக்கநிலையில்
row-snoozed-day-time = { $day } { $time }
snoozed-group-today = இன்று
snoozed-group-tomorrow = நாளை
snoozed-group-this-week = இந்த வாரம்
snoozed-group-later = பின்னர்
row-follow-up-step = ஃபாலோ-அப் { $steps } இல் { $step } · { $date }
row-follow-up-waiting = ஃபாலோ-அப் காத்திருக்கிறது
row-reminder = நினைவூட்டல் { $date }

## Mail list: More menu and right-click menu

menu-reply = பதிலளி
menu-reply-all = அனைவருக்கும் பதிலளி
menu-forward = முன்னனுப்பு
menu-archive = காப்பகப்படுத்து
menu-delete = நீக்கு
menu-delete-forever = நிரந்தரமாக நீக்கு
menu-move-to-inbox = இன்பாக்ஸுக்கு நகர்த்து
menu-spam = ஸ்பேம் எனப் புகாரளி
menu-not-spam = ஸ்பேம் அல்ல
menu-mark-read = படித்ததாகக் குறி
menu-mark-unread = படிக்காததாகக் குறி
menu-mark-all-read = அனைத்தையும் படித்ததாகக் குறி
menu-star = நட்சத்திரமிடு
menu-unstar = நட்சத்திரத்தை அகற்று
menu-important = முக்கியமானது எனக் குறி
menu-not-important = முக்கியமில்லாதது எனக் குறி
menu-pin = மேலே பின் செய்
menu-unpin = பின்னை அகற்று
menu-snooze = உறக்கநிலையில் வை
menu-remind = எனக்கு நினைவூட்டு
menu-unsnooze = உறக்கநிலையை நீக்கு
menu-add-to-tasks = பணிகளில் சேர்
menu-schedule-meeting = கூட்டத்தைத் திட்டமிடு
menu-start-call = வீடியோ அழைப்பைத் தொடங்கு
menu-add-note = குறிப்பைச் சேர்
menu-print-all = அனைத்தையும் அச்சிடு
menu-new-window = புதிய சாளரத்தில் திற
menu-move-to = இதற்கு நகர்த்து
menu-follow-up = பின்தொடர்
menu-more = மேலும்
menu-move-to-heading = இதற்கு நகர்த்து:
menu-move-to-search = இதற்கு நகர்த்து…
menu-label-as = லேபிளிடு
menu-label-as-search = லேபிளிடு…
menu-no-folder = “{ $name }” என்ற ஃபோல்டர் இல்லை
menu-no-label = “{ $name }” என்ற லேபிள் இல்லை
menu-create-folder = “{ $name }” ஐ உருவாக்கு
menu-always-move = { $name } இடமிருந்து வரும் அஞ்சலை எப்போதும் இங்கே நகர்த்து
toast-always-move-failed = அஞ்சல் நகர்த்தப்பட்டது, ஆனால் விதி உருவாக்கப்படவில்லை: { $error }
drag-mail = { $kind ->
    [conversation] { $count ->
        [one] { $count } உரையாடல்
       *[other] { $count } உரையாடல்கள்
    }
   *[message] { $count ->
        [one] { $count } மெசேஜ்
       *[other] { $count } மெசேஜ்கள்
    }
}
menu-find-from = { $name } அனுப்பிய மின்னஞ்சல்களைக் கண்டறி
menu-make-rule = விதியை உருவாக்கு…

## Snackbar after an action on mail in the list

toast-key-imported = கீ இறக்குமதி செய்யப்பட்டது
toast-key-updated = இந்தக் கீ ஏற்கனவே உங்களிடம் இருந்தது; இப்போது அது புதுப்பிக்கப்பட்டுள்ளது
toast-key-removed = கீ அகற்றப்பட்டது
toast-key-not-removed = கீயை அகற்ற முடியவில்லை
toast-fingerprint-copied = ஃபிங்கர்பிரிண்ட் நகலெடுக்கப்பட்டது
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
toast-label-added = “{ $label }” லேபிள் சேர்க்கப்பட்டது.
toast-label-removed = “{ $label }” லேபிள் அகற்றப்பட்டது.
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
toast-snoozed = { $kind ->
    [conversation] { $count ->
        [one] உரையாடல் { $when } வரை உறக்கநிலையில் வைக்கப்பட்டது.
       *[other] { $count } உரையாடல்கள் { $when } வரை உறக்கநிலையில் வைக்கப்பட்டன.
    }
   *[message] { $count ->
        [one] மெசேஜ் { $when } வரை உறக்கநிலையில் வைக்கப்பட்டது.
       *[other] { $count } மெசேஜ்கள் { $when } வரை உறக்கநிலையில் வைக்கப்பட்டன.
    }
}
toast-unsnoozed = { $kind ->
    [conversation] { $count ->
        [one] உரையாடல் இன்பாக்ஸுக்குத் திரும்பியது.
       *[other] { $count } உரையாடல்கள் இன்பாக்ஸுக்குத் திரும்பின.
    }
   *[message] { $count ->
        [one] மெசேஜ் இன்பாக்ஸுக்குத் திரும்பியது.
       *[other] { $count } மெசேஜ்கள் இன்பாக்ஸுக்குத் திரும்பின.
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
toast-not-spam = { $kind ->
    [conversation] { $count ->
        [one] உரையாடல் ஸ்பேம் அல்ல எனக் குறிக்கப்பட்டு இன்பாக்ஸுக்கு நகர்த்தப்பட்டது.
       *[other] { $count } உரையாடல்கள் ஸ்பேம் அல்ல எனக் குறிக்கப்பட்டு இன்பாக்ஸுக்கு நகர்த்தப்பட்டன.
    }
   *[message] { $count ->
        [one] மெசேஜ் ஸ்பேம் அல்ல எனக் குறிக்கப்பட்டு இன்பாக்ஸுக்கு நகர்த்தப்பட்டது.
       *[other] { $count } மெசேஜ்கள் ஸ்பேம் அல்ல எனக் குறிக்கப்பட்டு இன்பாக்ஸுக்கு நகர்த்தப்பட்டன.
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
toast-marked-read = { $kind ->
    [conversation] { $count ->
        [one] உரையாடல் படித்ததாகக் குறிக்கப்பட்டது.
       *[other] { $count } உரையாடல்கள் படித்ததாகக் குறிக்கப்பட்டன.
    }
   *[message] { $count ->
        [one] மெசேஜ் படித்ததாகக் குறிக்கப்பட்டது.
       *[other] { $count } மெசேஜ்கள் படித்ததாகக் குறிக்கப்பட்டன.
    }
}
toast-marked-unread = { $kind ->
    [conversation] { $count ->
        [one] உரையாடல் படிக்காததாகக் குறிக்கப்பட்டது.
       *[other] { $count } உரையாடல்கள் படிக்காததாகக் குறிக்கப்பட்டன.
    }
   *[message] { $count ->
        [one] மெசேஜ் படிக்காததாகக் குறிக்கப்பட்டது.
       *[other] { $count } மெசேஜ்கள் படிக்காததாகக் குறிக்கப்பட்டன.
    }
}
toast-undone = செயல் செயல்தவிர்க்கப்பட்டது.
toast-nothing-to-undo = செயல்தவிர்க்க எதுவும் இல்லை.
toast-cannot-undo-delete-forever = நிரந்தரமாக நீக்கப்பட்ட மெயிலை மீட்டெடுக்க முடியாது.
toast-send-undone = அனுப்புதல் செயல்தவிர்க்கப்பட்டது.
toast-too-late-to-undo-send = செயல்தவிர்க்கத் தாமதமாகிவிட்டது: மெசேஜ் ஏற்கனவே அனுப்பப்பட்டுவிட்டது.
toast-undo = செயல்தவிர்
toast-close = மூடு
toast-no-spam-folder = இந்தக் கணக்கில் ஸ்பேம் ஃபோல்டர் இல்லை.
