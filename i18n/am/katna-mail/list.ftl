# Katna Mail, Amharic (አማርኛ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = ዋና
tab-promotions = ማስተዋወቂያዎች
tab-social = ማህበራዊ
tab-updates = ዝማኔዎች
tab-forums = መድረኮች
tab-focused = ትኩረት የተሰጣቸው
tab-other = ሌሎች
tab-inbox = ገቢ መልዕክት ሳጥን
tab-newsletters = ጋዜጣዎች
tab-notifications = ማሳወቂያዎች
tab-provider-other = በKatna የተደረደሩ

## Mail list: toolbar

list-select = ምረጥ
list-refresh = አድስ
list-checking = አዲስ ደብዳቤ በመፈተሽ ላይ…
list-more = ተጨማሪ
list-mark-read = እንደተነበበ ምልክት አድርግ
list-mark-unread = እንዳልተነበበ ምልክት አድርግ
list-move-to = ውሰድ ወደ
list-archive = ወደ ማህደር አስቀምጥ
list-spam = አይፈለጌ መልዕክት ሪፖርት አድርግ
list-delete = ሰርዝ
list-snooze = አሸልብ
list-unsnooze = ማሸለብ ሰርዝ
list-newer = አዲስ
list-older = የቆየ
list-range = { $first }–{ $last } ከ{ $total }
list-range-about = { $first }–{ $last } ከ{ $total } ገደማ
list-results = የ«{ $query }» ውጤቶች
list-results-corrected = የ«{ $query }» ውጤቶችን በማሳየት ላይ
list-search-instead = በምትኩ «{ $query }»ን ፈልግ
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = ሁሉም
list-pick-none = ምንም
list-pick-read = የተነበቡ
list-pick-unread = ያልተነበቡ
list-pick-starred = ኮከብ የተደረገባቸው
list-pick-unstarred = ኮከብ ያልተደረገባቸው

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] ሁሉም { $count } ውይይት ተመርጧል።
       *[other] ሁሉም { $count } ውይይቶች ተመርጠዋል።
    }
   *[message] { $count ->
        [one] ሁሉም { $count } መልዕክት ተመርጧል።
       *[other] ሁሉም { $count } መልዕክቶች ተመርጠዋል።
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] በ{ $folder } ውስጥ ያለው ሁሉም { $count } ውይይት ተመርጧል።
       *[other] በ{ $folder } ውስጥ ያሉት ሁሉም { $count } ውይይቶች ተመርጠዋል።
    }
   *[message] { $count ->
        [one] በ{ $folder } ውስጥ ያለው ሁሉም { $count } መልዕክት ተመርጧል።
       *[other] በ{ $folder } ውስጥ ያሉት ሁሉም { $count } መልዕክቶች ተመርጠዋል።
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] በማያ ገጹ ላይ ያለው ሁሉም { $count } ውይይት ተመርጧል።
       *[other] በማያ ገጹ ላይ ያሉት ሁሉም { $count } ውይይቶች ተመርጠዋል።
    }
   *[message] { $count ->
        [one] በማያ ገጹ ላይ ያለው ሁሉም { $count } መልዕክት ተመርጧል።
       *[other] በማያ ገጹ ላይ ያሉት ሁሉም { $count } መልዕክቶች ተመርጠዋል።
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] ሁሉንም { $count } ውይይት ምረጥ
       *[other] ሁሉንም { $count } ውይይቶች ምረጥ
    }
   *[message] { $count ->
        [one] ሁሉንም { $count } መልዕክት ምረጥ
       *[other] ሁሉንም { $count } መልዕክቶች ምረጥ
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] በ{ $folder } ውስጥ ያለውን ሁሉንም { $count } ውይይት ምረጥ
       *[other] በ{ $folder } ውስጥ ያሉትን ሁሉንም { $count } ውይይቶች ምረጥ
    }
   *[message] { $count ->
        [one] በ{ $folder } ውስጥ ያለውን ሁሉንም { $count } መልዕክት ምረጥ
       *[other] በ{ $folder } ውስጥ ያሉትን ሁሉንም { $count } መልዕክቶች ምረጥ
    }
}
list-selected-picked-screen = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] በማያ ገጹ ላይ ያለው { $count } የተነበበ ውይይት ተመርጧል።
           *[other] በማያ ገጹ ላይ ያሉት ሁሉም { $count } የተነበቡ ውይይቶች ተመርጠዋል።
        }
       *[message] { $count ->
            [one] በማያ ገጹ ላይ ያለው { $count } የተነበበ መልዕክት ተመርጧል።
           *[other] በማያ ገጹ ላይ ያሉት ሁሉም { $count } የተነበቡ መልዕክቶች ተመርጠዋል።
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] በማያ ገጹ ላይ ያለው { $count } ያልተነበበ ውይይት ተመርጧል።
           *[other] በማያ ገጹ ላይ ያሉት ሁሉም { $count } ያልተነበቡ ውይይቶች ተመርጠዋል።
        }
       *[message] { $count ->
            [one] በማያ ገጹ ላይ ያለው { $count } ያልተነበበ መልዕክት ተመርጧል።
           *[other] በማያ ገጹ ላይ ያሉት ሁሉም { $count } ያልተነበቡ መልዕክቶች ተመርጠዋል።
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] በማያ ገጹ ላይ ያለው { $count } ኮከብ የተደረገበት ውይይት ተመርጧል።
           *[other] በማያ ገጹ ላይ ያሉት ሁሉም { $count } ኮከብ የተደረገባቸው ውይይቶች ተመርጠዋል።
        }
       *[message] { $count ->
            [one] በማያ ገጹ ላይ ያለው { $count } ኮከብ የተደረገበት መልዕክት ተመርጧል።
           *[other] በማያ ገጹ ላይ ያሉት ሁሉም { $count } ኮከብ የተደረገባቸው መልዕክቶች ተመርጠዋል።
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] በማያ ገጹ ላይ ያለው { $count } ኮከብ ያልተደረገበት ውይይት ተመርጧል።
           *[other] በማያ ገጹ ላይ ያሉት ሁሉም { $count } ኮከብ ያልተደረገባቸው ውይይቶች ተመርጠዋል።
        }
       *[message] { $count ->
            [one] በማያ ገጹ ላይ ያለው { $count } ኮከብ ያልተደረገበት መልዕክት ተመርጧል።
           *[other] በማያ ገጹ ላይ ያሉት ሁሉም { $count } ኮከብ ያልተደረገባቸው መልዕክቶች ተመርጠዋል።
        }
    }
}
list-select-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $count } የተነበበ ውይይት ምረጥ
           *[other] ሁሉንም { $count } የተነበቡ ውይይቶች ምረጥ
        }
       *[message] { $count ->
            [one] { $count } የተነበበ መልዕክት ምረጥ
           *[other] ሁሉንም { $count } የተነበቡ መልዕክቶች ምረጥ
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $count } ያልተነበበ ውይይት ምረጥ
           *[other] ሁሉንም { $count } ያልተነበቡ ውይይቶች ምረጥ
        }
       *[message] { $count ->
            [one] { $count } ያልተነበበ መልዕክት ምረጥ
           *[other] ሁሉንም { $count } ያልተነበቡ መልዕክቶች ምረጥ
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $count } ኮከብ የተደረገበት ውይይት ምረጥ
           *[other] ሁሉንም { $count } ኮከብ የተደረገባቸው ውይይቶች ምረጥ
        }
       *[message] { $count ->
            [one] { $count } ኮከብ የተደረገበት መልዕክት ምረጥ
           *[other] ሁሉንም { $count } ኮከብ የተደረገባቸው መልዕክቶች ምረጥ
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $count } ኮከብ ያልተደረገበት ውይይት ምረጥ
           *[other] ሁሉንም { $count } ኮከብ ያልተደረገባቸው ውይይቶች ምረጥ
        }
       *[message] { $count ->
            [one] { $count } ኮከብ ያልተደረገበት መልዕክት ምረጥ
           *[other] ሁሉንም { $count } ኮከብ ያልተደረገባቸው መልዕክቶች ምረጥ
        }
    }
}
list-select-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] በ{ $folder } ውስጥ ያለውን { $count } የተነበበ ውይይት ምረጥ
           *[other] በ{ $folder } ውስጥ ያሉትን ሁሉንም { $count } የተነበቡ ውይይቶች ምረጥ
        }
       *[message] { $count ->
            [one] በ{ $folder } ውስጥ ያለውን { $count } የተነበበ መልዕክት ምረጥ
           *[other] በ{ $folder } ውስጥ ያሉትን ሁሉንም { $count } የተነበቡ መልዕክቶች ምረጥ
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] በ{ $folder } ውስጥ ያለውን { $count } ያልተነበበ ውይይት ምረጥ
           *[other] በ{ $folder } ውስጥ ያሉትን ሁሉንም { $count } ያልተነበቡ ውይይቶች ምረጥ
        }
       *[message] { $count ->
            [one] በ{ $folder } ውስጥ ያለውን { $count } ያልተነበበ መልዕክት ምረጥ
           *[other] በ{ $folder } ውስጥ ያሉትን ሁሉንም { $count } ያልተነበቡ መልዕክቶች ምረጥ
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] በ{ $folder } ውስጥ ያለውን { $count } ኮከብ የተደረገበት ውይይት ምረጥ
           *[other] በ{ $folder } ውስጥ ያሉትን ሁሉንም { $count } ኮከብ የተደረገባቸው ውይይቶች ምረጥ
        }
       *[message] { $count ->
            [one] በ{ $folder } ውስጥ ያለውን { $count } ኮከብ የተደረገበት መልዕክት ምረጥ
           *[other] በ{ $folder } ውስጥ ያሉትን ሁሉንም { $count } ኮከብ የተደረገባቸው መልዕክቶች ምረጥ
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] በ{ $folder } ውስጥ ያለውን { $count } ኮከብ ያልተደረገበት ውይይት ምረጥ
           *[other] በ{ $folder } ውስጥ ያሉትን ሁሉንም { $count } ኮከብ ያልተደረገባቸው ውይይቶች ምረጥ
        }
       *[message] { $count ->
            [one] በ{ $folder } ውስጥ ያለውን { $count } ኮከብ ያልተደረገበት መልዕክት ምረጥ
           *[other] በ{ $folder } ውስጥ ያሉትን ሁሉንም { $count } ኮከብ ያልተደረገባቸው መልዕክቶች ምረጥ
        }
    }
}
list-selected-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $count } የተነበበ ውይይት ተመርጧል።
           *[other] ሁሉም { $count } የተነበቡ ውይይቶች ተመርጠዋል።
        }
       *[message] { $count ->
            [one] { $count } የተነበበ መልዕክት ተመርጧል።
           *[other] ሁሉም { $count } የተነበቡ መልዕክቶች ተመርጠዋል።
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $count } ያልተነበበ ውይይት ተመርጧል።
           *[other] ሁሉም { $count } ያልተነበቡ ውይይቶች ተመርጠዋል።
        }
       *[message] { $count ->
            [one] { $count } ያልተነበበ መልዕክት ተመርጧል።
           *[other] ሁሉም { $count } ያልተነበቡ መልዕክቶች ተመርጠዋል።
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $count } ኮከብ የተደረገበት ውይይት ተመርጧል።
           *[other] ሁሉም { $count } ኮከብ የተደረገባቸው ውይይቶች ተመርጠዋል።
        }
       *[message] { $count ->
            [one] { $count } ኮከብ የተደረገበት መልዕክት ተመርጧል።
           *[other] ሁሉም { $count } ኮከብ የተደረገባቸው መልዕክቶች ተመርጠዋል።
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $count } ኮከብ ያልተደረገበት ውይይት ተመርጧል።
           *[other] ሁሉም { $count } ኮከብ ያልተደረገባቸው ውይይቶች ተመርጠዋል።
        }
       *[message] { $count ->
            [one] { $count } ኮከብ ያልተደረገበት መልዕክት ተመርጧል።
           *[other] ሁሉም { $count } ኮከብ ያልተደረገባቸው መልዕክቶች ተመርጠዋል።
        }
    }
}
list-selected-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] በአቃፊ { $folder } ውስጥ { $count } የተነበበ ውይይት ተመርጧል።
           *[other] በአቃፊ { $folder } ውስጥ ሁሉም { $count } የተነበቡ ውይይቶች ተመርጠዋል።
        }
       *[message] { $count ->
            [one] በአቃፊ { $folder } ውስጥ { $count } የተነበበ መልዕክት ተመርጧል።
           *[other] በአቃፊ { $folder } ውስጥ ሁሉም { $count } የተነበቡ መልዕክቶች ተመርጠዋል።
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] በአቃፊ { $folder } ውስጥ { $count } ያልተነበበ ውይይት ተመርጧል።
           *[other] በአቃፊ { $folder } ውስጥ ሁሉም { $count } ያልተነበቡ ውይይቶች ተመርጠዋል።
        }
       *[message] { $count ->
            [one] በአቃፊ { $folder } ውስጥ { $count } ያልተነበበ መልዕክት ተመርጧል።
           *[other] በአቃፊ { $folder } ውስጥ ሁሉም { $count } ያልተነበቡ መልዕክቶች ተመርጠዋል።
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] በአቃፊ { $folder } ውስጥ { $count } ኮከብ የተደረገበት ውይይት ተመርጧል።
           *[other] በአቃፊ { $folder } ውስጥ ሁሉም { $count } ኮከብ የተደረገባቸው ውይይቶች ተመርጠዋል።
        }
       *[message] { $count ->
            [one] በአቃፊ { $folder } ውስጥ { $count } ኮከብ የተደረገበት መልዕክት ተመርጧል።
           *[other] በአቃፊ { $folder } ውስጥ ሁሉም { $count } ኮከብ የተደረገባቸው መልዕክቶች ተመርጠዋል።
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] በአቃፊ { $folder } ውስጥ { $count } ኮከብ ያልተደረገበት ውይይት ተመርጧል።
           *[other] በአቃፊ { $folder } ውስጥ ሁሉም { $count } ኮከብ ያልተደረገባቸው ውይይቶች ተመርጠዋል።
        }
       *[message] { $count ->
            [one] በአቃፊ { $folder } ውስጥ { $count } ኮከብ ያልተደረገበት መልዕክት ተመርጧል።
           *[other] በአቃፊ { $folder } ውስጥ ሁሉም { $count } ኮከብ ያልተደረገባቸው መልዕክቶች ተመርጠዋል።
        }
    }
}
list-picked-none = { $pick ->
    [read] { $kind ->
        [conversation] እዚህ ምንም የተነበቡ ውይይቶች የሉም።
       *[message] እዚህ ምንም የተነበቡ መልዕክቶች የሉም።
    }
   *[unread] { $kind ->
        [conversation] እዚህ ምንም ያልተነበቡ ውይይቶች የሉም።
       *[message] እዚህ ምንም ያልተነበቡ መልዕክቶች የሉም።
    }
    [starred] { $kind ->
        [conversation] እዚህ ምንም ኮከብ የተደረገባቸው ውይይቶች የሉም።
       *[message] እዚህ ምንም ኮከብ የተደረገባቸው መልዕክቶች የሉም።
    }
    [unstarred] { $kind ->
        [conversation] እዚህ ምንም ኮከብ ያልተደረገባቸው ውይይቶች የሉም።
       *[message] እዚህ ምንም ኮከብ ያልተደረገባቸው መልዕክቶች የሉም።
    }
}
list-clear-selection = ምርጫን አጽዳ

## Mail list: empty states

list-empty-search = ከፍለጋዎ ጋር የሚዛመድ መልዕክት የለም።
list-empty-tab = በ{ $tab } ውስጥ ምንም ደብዳቤ የለም።
list-empty-tab-unknown = በዚህ ትር ውስጥ ምንም ደብዳቤ የለም።
list-empty-folder = በ{ $folder } ውስጥ ምንም መልዕክት የለም።
list-empty-folder-unknown = በዚህ አቃፊ ውስጥ ምንም መልዕክት የለም።
list-first-sync = ደብዳቤዎን በማምጣት ላይ…
list-first-sync-detail = ሲደርስ እዚህ ይታያል።

## Mail list: lines

row-removed = ይህ መልዕክት ተወግዷል።
row-starred = ኮከብ የተደረገበት
row-not-starred = ኮከብ ያልተደረገበት
row-important = አስፈላጊ። አስፈላጊ እንዳልሆነ ምልክት ለማድረግ ጠቅ ያድርጉ።
row-mark-important = እንደ አስፈላጊ ምልክት አድርግ
row-pinned = ከላይ ተሰክቷል
row-task = ተግባር
row-task-open = ተግባሩን ክፈት፦ { $title }
row-tracking-none = ክትትል ይደረግበታል። እስካሁን አልተከፈተም
row-tracking-opened = ከ{ $recipients } ውስጥ በ{ $opened } ተከፍቷል
row-tracking-clicked = ከ{ $recipients } ውስጥ በ{ $opened } ተከፍቷል፣ { $clicked } አገናኝ ተከትለዋል
row-pin = ከላይ ሰካ
row-unpin = ንቀል
row-snoozed-until = እስከ { $when } አሸልቧል

## Mail list: More menu and right-click menu

menu-reply = መልስ
menu-reply-all = ለሁሉም መልስ
menu-forward = አስተላልፍ
menu-archive = ወደ ማህደር አስቀምጥ
menu-delete = ሰርዝ
menu-delete-forever = እስከመጨረሻው ሰርዝ
menu-move-to-inbox = ወደ ገቢ መልዕክት ሳጥን ውሰድ
menu-spam = አይፈለጌ መልዕክት ሪፖርት አድርግ
menu-not-spam = አይፈለጌ መልዕክት አይደለም
menu-mark-read = እንደተነበበ ምልክት አድርግ
menu-mark-unread = እንዳልተነበበ ምልክት አድርግ
menu-mark-all-read = ሁሉንም እንደተነበቡ ምልክት አድርግ
menu-star = ኮከብ አክል
menu-unstar = ኮከብ አስወግድ
menu-important = እንደ አስፈላጊ ምልክት አድርግ
menu-not-important = አስፈላጊ እንዳልሆነ ምልክት አድርግ
menu-pin = ከላይ ሰካ
menu-unpin = ንቀል
menu-snooze = አሸልብ
menu-unsnooze = ማሸለብ ሰርዝ
menu-add-to-tasks = ወደ ተግባራት አክል
menu-schedule-meeting = ስብሰባ መርሐግብር አውጣ
menu-start-call = የቪዲዮ ጥሪ ጀምር
menu-add-note = ማስታወሻ አክል
menu-print-all = ሁሉንም አትም
menu-new-window = በአዲስ መስኮት ክፈት
menu-move-to = ውሰድ ወደ
# Opens a submenu: Add to Tasks, Add a note, Schedule a meeting and Start a
# video call.
menu-follow-up = ክትትል
# Opens a submenu of the rarer actions: Report spam, Mark as important and
# Pin to top.
menu-more = ተጨማሪ
menu-move-to-heading = ውሰድ ወደ፦
menu-find-from = ከ{ $name } የመጡ ኢሜይሎችን ፈልግ

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] ውይይቱ ወደ ማህደር ተቀምጧል።
       *[other] { $count } ውይይቶች ወደ ማህደር ተቀምጠዋል።
    }
   *[message] { $count ->
        [one] መልዕክቱ ወደ ማህደር ተቀምጧል።
       *[other] { $count } መልዕክቶች ወደ ማህደር ተቀምጠዋል።
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] ውይይቱ ወደ መጣያ ተወስዷል።
       *[other] { $count } ውይይቶች ወደ መጣያ ተወስደዋል።
    }
   *[message] { $count ->
        [one] መልዕክቱ ወደ መጣያ ተወስዷል።
       *[other] { $count } መልዕክቶች ወደ መጣያ ተወስደዋል።
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] ውይይቱ ተወስዷል።
       *[other] { $count } ውይይቶች ተወስደዋል።
    }
   *[message] { $count ->
        [one] መልዕክቱ ተወስዷል።
       *[other] { $count } መልዕክቶች ተወስደዋል።
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] ውይይቱ ኮከብ ተደርጎበታል።
       *[other] { $count } ውይይቶች ኮከብ ተደርጎባቸዋል።
    }
   *[message] { $count ->
        [one] መልዕክቱ ኮከብ ተደርጎበታል።
       *[other] { $count } መልዕክቶች ኮከብ ተደርጎባቸዋል።
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] ከውይይቱ ኮከብ ተወግዷል።
       *[other] ከ{ $count } ውይይቶች ኮከብ ተወግዷል።
    }
   *[message] { $count ->
        [one] ከመልዕክቱ ኮከብ ተወግዷል።
       *[other] ከ{ $count } መልዕክቶች ኮከብ ተወግዷል።
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] ውይይቱ አስፈላጊ ተብሎ ምልክት ተደርጎበታል።
       *[other] { $count } ውይይቶች አስፈላጊ ተብለው ምልክት ተደርጎባቸዋል።
    }
   *[message] { $count ->
        [one] መልዕክቱ አስፈላጊ ተብሎ ምልክት ተደርጎበታል።
       *[other] { $count } መልዕክቶች አስፈላጊ ተብለው ምልክት ተደርጎባቸዋል።
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] ውይይቱ አስፈላጊ አይደለም ተብሎ ምልክት ተደርጎበታል።
       *[other] { $count } ውይይቶች አስፈላጊ አይደሉም ተብለው ምልክት ተደርጎባቸዋል።
    }
   *[message] { $count ->
        [one] መልዕክቱ አስፈላጊ አይደለም ተብሎ ምልክት ተደርጎበታል።
       *[other] { $count } መልዕክቶች አስፈላጊ አይደሉም ተብለው ምልክት ተደርጎባቸዋል።
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] ውይይቱ ከላይ ተሰክቷል።
       *[other] { $count } ውይይቶች ከላይ ተሰክተዋል።
    }
   *[message] { $count ->
        [one] መልዕክቱ ከላይ ተሰክቷል።
       *[other] { $count } መልዕክቶች ከላይ ተሰክተዋል።
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] ውይይቱ ተነቅሏል።
       *[other] { $count } ውይይቶች ተነቅለዋል።
    }
   *[message] { $count ->
        [one] መልዕክቱ ተነቅሏል።
       *[other] { $count } መልዕክቶች ተነቅለዋል።
    }
}
toast-snoozed = { $kind ->
    [conversation] { $count ->
        [one] ውይይቱ እስከ { $when } አሸልቧል።
       *[other] { $count } ውይይቶች እስከ { $when } አሸልበዋል።
    }
   *[message] { $count ->
        [one] መልዕክቱ እስከ { $when } አሸልቧል።
       *[other] { $count } መልዕክቶች እስከ { $when } አሸልበዋል።
    }
}
toast-unsnoozed = { $kind ->
    [conversation] { $count ->
        [one] ውይይቱ ወደ ገቢ መልዕክት ሳጥን ተመልሷል።
       *[other] { $count } ውይይቶች ወደ ገቢ መልዕክት ሳጥን ተመልሰዋል።
    }
   *[message] { $count ->
        [one] መልዕክቱ ወደ ገቢ መልዕክት ሳጥን ተመልሷል።
       *[other] { $count } መልዕክቶች ወደ ገቢ መልዕክት ሳጥን ተመልሰዋል።
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] ውይይቱ አይፈለጌ መልዕክት ተብሎ ሪፖርት ተደርጓል።
       *[other] { $count } ውይይቶች አይፈለጌ መልዕክት ተብለው ሪፖርት ተደርገዋል።
    }
   *[message] { $count ->
        [one] መልዕክቱ አይፈለጌ መልዕክት ተብሎ ሪፖርት ተደርጓል።
       *[other] { $count } መልዕክቶች አይፈለጌ መልዕክት ተብለው ሪፖርት ተደርገዋል።
    }
}
toast-not-spam = { $kind ->
    [conversation] { $count ->
        [one] ውይይቱ አይፈለጌ መልዕክት አይደለም ተብሎ ወደ ገቢ መልዕክት ሳጥን ተወስዷል።
       *[other] { $count } ውይይቶች አይፈለጌ መልዕክት አይደሉም ተብለው ወደ ገቢ መልዕክት ሳጥን ተወስደዋል።
    }
   *[message] { $count ->
        [one] መልዕክቱ አይፈለጌ መልዕክት አይደለም ተብሎ ወደ ገቢ መልዕክት ሳጥን ተወስዷል።
       *[other] { $count } መልዕክቶች አይፈለጌ መልዕክት አይደሉም ተብለው ወደ ገቢ መልዕክት ሳጥን ተወስደዋል።
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] ውይይቱ እስከመጨረሻው ተሰርዟል።
       *[other] { $count } ውይይቶች እስከመጨረሻው ተሰርዘዋል።
    }
   *[message] { $count ->
        [one] መልዕክቱ እስከመጨረሻው ተሰርዟል።
       *[other] { $count } መልዕክቶች እስከመጨረሻው ተሰርዘዋል።
    }
}
toast-marked-read = { $kind ->
    [conversation] { $count ->
        [one] ውይይቱ እንደተነበበ ምልክት ተደርጓል።
       *[other] { $count } ውይይቶች እንደተነበቡ ምልክት ተደርጎባቸዋል።
    }
   *[message] { $count ->
        [one] መልዕክቱ እንደተነበበ ምልክት ተደርጓል።
       *[other] { $count } መልዕክቶች እንደተነበቡ ምልክት ተደርጎባቸዋል።
    }
}
toast-marked-unread = { $kind ->
    [conversation] { $count ->
        [one] ውይይቱ እንዳልተነበበ ምልክት ተደርጓል።
       *[other] { $count } ውይይቶች እንዳልተነበቡ ምልክት ተደርጎባቸዋል።
    }
   *[message] { $count ->
        [one] መልዕክቱ እንዳልተነበበ ምልክት ተደርጓል።
       *[other] { $count } መልዕክቶች እንዳልተነበቡ ምልክት ተደርጎባቸዋል።
    }
}
toast-undone = እርምጃው ተቀልብሷል።
toast-nothing-to-undo = የሚቀለበስ ምንም ነገር የለም።
toast-cannot-undo-delete-forever = እስከመጨረሻው የተሰረዘ ደብዳቤ ሊመለስ አይችልም።
toast-send-undone = መላኩ ተቀልብሷል።
toast-too-late-to-undo-send = ለመቀልበስ ዘግይቷል፦ መልዕክቱ አስቀድሞ ተልኳል።
toast-undo = ቀልብስ
toast-close = ዝጋ
toast-no-spam-folder = ይህ መለያ የአይፈለጌ መልዕክት አቃፊ የለውም።
