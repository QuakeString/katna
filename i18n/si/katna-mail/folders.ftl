# Katna Mail, Sinhala (සිංහල).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Navigation (the folders pane)

nav-labels = ලේබල
nav-folders = ෆෝල්ඩර
nav-label-new = නව ලේබලයක් සාදන්න
nav-folder-new = නව ෆෝල්ඩරයක් සාදන්න
nav-menu-check-mail = නව තැපැල් පරීක්ෂා කරන්න
nav-menu-check-inbox = මෙම එන ලිපි පරීක්ෂා කරන්න
nav-unified-leave-out = ඒකාබද්ධ එන ලිපිවලින් ඉවත් කරන්න
nav-unified-bring-back = ඒකාබද්ධ එන ලිපිවලට නැවත ගෙන එන්න
nav-menu-sign-in-again = නැවත පුරනය වන්න
nav-menu-new-mail = මෙම ගිණුමෙන් නව තැපැල
nav-menu-account-settings = ගිණුම් සැකසීම්
nav-account-checked = සමමුහුර්තයි · { $ago } පරීක්ෂා කළා
nav-account-in-sync = සමමුහුර්තයි
nav-account-connecting = සම්බන්ධ වෙමින්…
nav-account-offline = නොබැඳි, නැවත උත්සාහ කරමින්
nav-account-signed-out = { $provider } පුරනය කල් ඉකුත් විය
nav-account-password-refused = මුරපදය ප්‍රතික්ෂේප විය
nav-account-storage = { $total } න් { $used } භාවිතයි
nav-menu-new-subfolder = ඇතුළත නව ෆෝල්ඩරය
nav-menu-new-sublabel = ඇතුළත නව ලේබලය
nav-menu-rename = නැවත නම් කරන්න
nav-menu-delete = මකන්න
nav-menu-empty-trash = කුණු කූඩය හිස් කරන්න
nav-account-unnamed = ගිණුම { $number }
nav-all-accounts = සියලු ගිණුම්
nav-expand = ෆෝල්ඩර පෙන්වන්න
nav-collapse = ෆෝල්ඩර සඟවන්න
storage-used = { $total } න් { $percent }% භාවිත කර ඇත
storage-used-detail = { $address }: { $total } න් { $used } භාවිත කර ඇත

## Special folders (the user's own folders keep their names)

folder-inbox = එන ලිපි
folder-starred = තරු යෙදූ
folder-snoozed = කල් දැමූ
folder-unread = නොකියවූ
folder-important = වැදගත්
folder-drafts = කෙටුම්පත්
folder-sent = යැවූ
folder-archive = සංරක්ෂිත
folder-spam = අයාචිත තැපැල්
folder-trash = කුණු කූඩය
folder-all-mail = සියලු තැපැල්
folder-scheduled = උපලේඛනගත
folder-waiting = පිළිතුරක් බලාපොරොත්තුවෙන්
folder-waiting-short = බලාපොරොත්තුවෙන්
folder-reminders = සිහිකැඳවීම්
folder-outbox = පිටතට යන ලිපි
folder-activity = ක්‍රියාකාරකම්

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = නව ලේබලය
label-folder-new-title = නව ෆෝල්ඩරය
label-prompt = කරුණාකර නව ලේබල නාමයක් ඇතුළු කරන්න:
label-folder-prompt = කරුණාකර නව ෆෝල්ඩර නාමයක් ඇතුළු කරන්න:
label-name-hint = ලේබල නාමය
label-folder-name-hint = ෆෝල්ඩර නාමය
label-nest = ලේබලය මේ යටතේ තබන්න:
label-folder-nest = ෆෝල්ඩරය මේ යටතේ තබන්න:
label-cancel = අවලංගු කරන්න
label-create = සාදන්න
label-creating = සාදමින්…
label-created = “{ $name }” ලේබලය සාදන ලදී.
label-folder-created = “{ $name }” ෆෝල්ඩරය සාදන ලදී.
label-rename-title = ලේබලය නැවත නම් කරන්න
label-folder-rename-title = ෆෝල්ඩරය නැවත නම් කරන්න
label-rename = නැවත නම් කරන්න
label-renaming = නැවත නම් කරමින්…
label-renamed = ලේබලය “{ $name }” ලෙස නැවත නම් කළා.
label-folder-renamed = ෆෝල්ඩරය “{ $name }” ලෙස නැවත නම් කළා.

## Deleting a folder or label (asked first)

folder-delete-title = “{ $name }” මකන්නද?
folder-delete-body = { $count ->
    [0] එහි තැපැල් නැත. ෆෝල්ඩරය සේවාදායකයෙන් ඉවත් කෙරෙන නිසා, වෙබ් තැපැල් සහ ඔබේ දුරකථනයෙන්ද එය නැති වේ.
   *[other] { $kind ->
        [conversation] { $count ->
            [one] එහි සංවාද { $count } කුණු කූඩයට යන නිසා, ඔබට එය තවමත් ආපසු ලබා ගත හැක.
           *[other] එහි සංවාද { $count } කුණු කූඩයට යන නිසා, ඔබට ඒවා තවමත් ආපසු ලබා ගත හැක.
        }
       *[message] { $count ->
            [one] එහි පණිවිඩ { $count } කුණු කූඩයට යන නිසා, ඔබට එය තවමත් ආපසු ලබා ගත හැක.
           *[other] එහි පණිවිඩ { $count } කුණු කූඩයට යන නිසා, ඔබට ඒවා තවමත් ආපසු ලබා ගත හැක.
        }
    } ෆෝල්ඩරය සේවාදායකයෙන් ඉවත් කෙරෙන නිසා, වෙබ් තැපැල් සහ ඔබේ දුරකථනයෙන්ද එය නැති වේ.
}
folder-delete-forever-body = { $count ->
    [0] එහි තැපැල් නැත. ෆෝල්ඩරය සේවාදායකයෙන් ඉවත් කෙරෙන නිසා, වෙබ් තැපැල් සහ ඔබේ දුරකථනයෙන්ද එය නැති වේ.
   *[other] { $kind ->
        [conversation] { $count ->
            [one] එහි සංවාද { $count } සදහටම මැකේ; මෙම ගිණුමට කුණු කූඩයක් නැත.
           *[other] එහි සංවාද { $count } සදහටම මැකේ; මෙම ගිණුමට කුණු කූඩයක් නැත.
        }
       *[message] { $count ->
            [one] එහි පණිවිඩ { $count } සදහටම මැකේ; මෙම ගිණුමට කුණු කූඩයක් නැත.
           *[other] එහි පණිවිඩ { $count } සදහටම මැකේ; මෙම ගිණුමට කුණු කූඩයක් නැත.
        }
    } ෆෝල්ඩරය සේවාදායකයෙන් ඉවත් කෙරෙන නිසා, වෙබ් තැපැල් සහ ඔබේ දුරකථනයෙන්ද එය නැති වේ.
}
folder-delete-label-body = ලේබලය ඉවත් කෙරේ. එහි තැපැල් සියලු තැපැල් තුළ සහ එහි අනෙක් ලේබලවල පවතී.
folder-delete-confirm = ෆෝල්ඩරය මකන්න
folder-delete-label-confirm = ලේබලය මකන්න
folder-deleted = “{ $name }” ෆෝල්ඩරය මැකුවා
label-deleted = “{ $name }” ලේබලය මැකුවා
