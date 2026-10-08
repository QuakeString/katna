# Katna Mail, Amharic (አማርኛ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Navigation (the folders pane)

nav-labels = መሰየሚያዎች
nav-folders = አቃፊዎች
nav-label-new = አዲስ መሰየሚያ ፍጠር
nav-folder-new = አዲስ አቃፊ ፍጠር
nav-menu-check-mail = አዲስ ደብዳቤ ይፈትሹ
nav-menu-check-inbox = ይህን ገቢ መልዕክት ሳጥን ፈትሽ
nav-unified-leave-out = ከተዋሃደው ገቢ መልዕክት ሳጥን ውጭ አድርግ
nav-unified-bring-back = ወደ ተዋሃደው ገቢ መልዕክት ሳጥን መልስ
nav-menu-sign-in-again = እንደገና ግባ
nav-menu-new-mail = ከዚህ መለያ አዲስ ደብዳቤ
nav-menu-account-settings = የመለያ ቅንብሮች
nav-account-checked = ተመሳስሏል · የተፈተሸው { $ago }
nav-account-in-sync = ተመሳስሏል
nav-account-connecting = በመገናኘት ላይ…
nav-account-offline = ከመስመር ውጭ፣ እንደገና በመሞከር ላይ
nav-account-signed-out = የ{ $provider } መግቢያ ጊዜው አልፏል
nav-account-password-refused = የይለፍ ቃሉ ተቀባይነት አላገኘም
nav-account-storage = ከ{ $total } { $used } ጥቅም ላይ ውሏል
nav-menu-new-subfolder = በውስጡ አዲስ አቃፊ
nav-menu-new-sublabel = በውስጡ አዲስ መሰየሚያ
nav-menu-rename = እንደገና ሰይም
nav-menu-delete = ሰርዝ
nav-menu-empty-trash = መጣያውን ባዶ አድርግ
nav-account-unnamed = መለያ { $number }
nav-all-accounts = ሁሉም መለያዎች
nav-expand = አቃፊዎችን አሳይ
nav-collapse = አቃፊዎችን ደብቅ
storage-used = ከ{ $total } ውስጥ { $percent }% ጥቅም ላይ ውሏል
storage-used-detail = { $address }፦ ከ{ $total } ውስጥ { $used } ጥቅም ላይ ውሏል

## Special folders (the user's own folders keep their names)

folder-inbox = ገቢ መልዕክት ሳጥን
folder-starred = ኮከብ የተደረገባቸው
folder-snoozed = ያሸለቡ
folder-unread = ያልተነበቡ
folder-important = አስፈላጊ
folder-drafts = ረቂቆች
folder-sent = የተላኩ
folder-archive = ማህደር
folder-spam = አይፈለጌ መልዕክት
folder-trash = መጣያ
folder-all-mail = ሁሉም ደብዳቤ
folder-scheduled = መርሐግብር የተያዘላቸው
folder-waiting = ምላሽ በመጠበቅ ላይ
folder-waiting-short = በመጠበቅ ላይ
folder-reminders = አስታዋሾች
folder-outbox = የወጪ መልዕክት ሳጥን
folder-activity = እንቅስቃሴ

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = አዲስ መሰየሚያ
label-folder-new-title = አዲስ አቃፊ
label-prompt = እባክዎ አዲስ የመሰየሚያ ስም ያስገቡ፦
label-folder-prompt = እባክዎ አዲስ የአቃፊ ስም ያስገቡ፦
label-name-hint = የመሰየሚያ ስም
label-folder-name-hint = የአቃፊ ስም
label-nest = መሰየሚያውን ከዚህ ስር አስቀምጥ፦
label-folder-nest = አቃፊውን ከዚህ ስር አስቀምጥ፦
label-cancel = ይቅር
label-create = ፍጠር
label-creating = በመፍጠር ላይ…
label-created = መሰየሚያ «{ $name }» ተፈጥሯል።
label-folder-created = አቃፊ «{ $name }» ተፈጥሯል።
label-rename-title = መሰየሚያውን እንደገና ሰይም
label-folder-rename-title = አቃፊውን እንደገና ሰይም
label-rename = እንደገና ሰይም
label-renaming = እንደገና በመሰየም ላይ…
label-renamed = መሰየሚያው ወደ «{ $name }» ተቀይሯል።
label-folder-renamed = አቃፊው ወደ «{ $name }» ተቀይሯል።

## Deleting a folder or label (asked first)

folder-delete-title = «{ $name }» ይሰረዝ?
folder-delete-body = { $count ->
    [0] ምንም ደብዳቤ የለውም። አቃፊው ከአገልጋዩ ይወገዳል፣ ስለዚህ ከዌብሜይል እና ከስልክዎም ይጠፋል።
   *[other] { $kind ->
        [conversation] { $count ->
            [one] { $count } ውይይቱ ወደ መጣያ ይሄዳል፣ ስለዚህ አሁንም መልሰው ማግኘት ይችላሉ።
           *[other] { $count } ውይይቶቹ ወደ መጣያ ይሄዳሉ፣ ስለዚህ አሁንም መልሰው ማግኘት ይችላሉ።
        }
       *[message] { $count ->
            [one] { $count } መልዕክቱ ወደ መጣያ ይሄዳል፣ ስለዚህ አሁንም መልሰው ማግኘት ይችላሉ።
           *[other] { $count } መልዕክቶቹ ወደ መጣያ ይሄዳሉ፣ ስለዚህ አሁንም መልሰው ማግኘት ይችላሉ።
        }
    } አቃፊው ከአገልጋዩ ይወገዳል፣ ስለዚህ ከዌብሜይል እና ከስልክዎም ይጠፋል።
}
folder-delete-forever-body = { $count ->
    [0] ምንም ደብዳቤ የለውም። አቃፊው ከአገልጋዩ ይወገዳል፣ ስለዚህ ከዌብሜይል እና ከስልክዎም ይጠፋል።
   *[other] { $kind ->
        [conversation] { $count ->
            [one] { $count } ውይይቱ እስከመጨረሻው ይሰረዛል፤ ይህ መለያ መጣያ የለውም።
           *[other] { $count } ውይይቶቹ እስከመጨረሻው ይሰረዛሉ፤ ይህ መለያ መጣያ የለውም።
        }
       *[message] { $count ->
            [one] { $count } መልዕክቱ እስከመጨረሻው ይሰረዛል፤ ይህ መለያ መጣያ የለውም።
           *[other] { $count } መልዕክቶቹ እስከመጨረሻው ይሰረዛሉ፤ ይህ መለያ መጣያ የለውም።
        }
    } አቃፊው ከአገልጋዩ ይወገዳል፣ ስለዚህ ከዌብሜይል እና ከስልክዎም ይጠፋል።
}
folder-delete-label-body = መሰየሚያው ይወገዳል። ደብዳቤዎቹ በሁሉም ደብዳቤ እና በሌሎች መሰየሚያዎቻቸው ውስጥ ይቆያሉ።
folder-delete-confirm = አቃፊውን ሰርዝ
folder-delete-label-confirm = መሰየሚያውን ሰርዝ
folder-deleted = አቃፊ «{ $name }» ተሰርዟል
label-deleted = መሰየሚያ «{ $name }» ተሰርዟል
