# Katna Mail, Dzongkha (རྫོང་ཁ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = གཙོ་བོ
tab-promotions = ཁྱབ་བསྒྲགས
tab-social = མི་སྡེ
tab-updates = གསར་བསྒྱུར
tab-forums = གྲོས་བསྡུར་ས་སྒོ
tab-focused = དམིགས་གཏད
tab-other = གཞན
tab-inbox = ནང་འབྱོར་སྒྲོམ
tab-newsletters = གསར་ཤོག
tab-notifications = བརྡ་བསྐུལ
tab-new = གསརཔ་ { $count }
tab-provider-other = Katna གིས་དབྱེ་སེལ་འབད་ཡོདཔ

## Mail list: toolbar

list-select = གདམ།
list-refresh = གསར་བཟོ།
list-checking = གློག་འཕྲིན་གསརཔ་ཞིབ་དཔྱད་འབད་དོ…
list-more = གཞན་ཡང་།
list-mark-read = ལྷག་ཡོདཔ་སྦེ་རྟགས་བཀལ།
list-mark-unread = མ་ལྷག་པ་སྦེ་རྟགས་བཀལ།
list-move-to = ལུ་སྤོ།
list-archive = ཡིག་མཛོད་ནང་བཙུགས།
list-spam = སྤེམ་སྦེ་སྙན་ཞུ་འབད།
list-delete = བཏོན་གཏང་།
list-snooze = ཤུལ་མར་བཞག།
list-unsnooze = ཤུལ་མར་བཞག་མི་བཏོན།
list-newer = དེ་ལས་གསརཔ།
list-older = དེ་ལས་རྙིངམ།
list-range = { $total } ལས་ { $first }–{ $last }
list-range-about = ཧ་ལམ་ { $total } ལས་ { $first }–{ $last }
list-results = “{ $query }” གི་གྲུབ་འབྲས་ཚུ
list-results-corrected = “{ $query }” གི་གྲུབ་འབྲས་ཚུ་སྟོན་དོ
list-search-instead = དེ་གི་ཚབ་ལུ་ “{ $query }” འཚོལ།
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = ཆ་མཉམ
list-pick-none = ག་ནི་ཡང་མེད
list-pick-read = ལྷག་ཡོདཔ
list-pick-unread = མ་ལྷག་པ
list-pick-starred = སྐར་མ་བཀལ་ཡོདཔ
list-pick-unstarred = སྐར་མ་མ་བཀལ་བ

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] གླེང་མོལ་ { $count } ཆ་མཉམ་གདམ་ཡོད།
   *[message] འཕྲིན་དོན་ { $count } ཆ་མཉམ་གདམ་ཡོད།
}
list-selected-all-in = { $kind ->
    [conversation] { $folder } ནང་གི་གླེང་མོལ་ { $count } ཆ་མཉམ་གདམ་ཡོད།
   *[message] { $folder } ནང་གི་འཕྲིན་དོན་ { $count } ཆ་མཉམ་གདམ་ཡོད།
}
list-selected-screen = { $kind ->
    [conversation] གསལ་གཞི་གུ་ཡོད་པའི་གླེང་མོལ་ { $count } ཆ་མཉམ་གདམ་ཡོད།
   *[message] གསལ་གཞི་གུ་ཡོད་པའི་འཕྲིན་དོན་ { $count } ཆ་མཉམ་གདམ་ཡོད།
}
list-select-all = { $kind ->
    [conversation] གླེང་མོལ་ { $count } ཆ་མཉམ་གདམ།
   *[message] འཕྲིན་དོན་ { $count } ཆ་མཉམ་གདམ།
}
list-select-all-in = { $kind ->
    [conversation] { $folder } ནང་གི་གླེང་མོལ་ { $count } ཆ་མཉམ་གདམ།
   *[message] { $folder } ནང་གི་འཕྲིན་དོན་ { $count } ཆ་མཉམ་གདམ།
}
list-selected-picked-screen = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
           *[other] གསལ་གཞི་གུ་ ལྷག་ཡོད་པའི་གླེང་མོལ་ { $count } ཆ་མཉམ་གདམ་ཡོད།
        }
       *[message] { $count ->
           *[other] གསལ་གཞི་གུ་ ལྷག་ཡོད་པའི་འཕྲིན་དོན་ { $count } ཆ་མཉམ་གདམ་ཡོད།
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
           *[other] གསལ་གཞི་གུ་ མ་ལྷག་པའི་གླེང་མོལ་ { $count } ཆ་མཉམ་གདམ་ཡོད།
        }
       *[message] { $count ->
           *[other] གསལ་གཞི་གུ་ མ་ལྷག་པའི་འཕྲིན་དོན་ { $count } ཆ་མཉམ་གདམ་ཡོད།
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
           *[other] གསལ་གཞི་གུ་ སྐར་མ་བཀལ་ཡོད་པའི་གླེང་མོལ་ { $count } ཆ་མཉམ་གདམ་ཡོད།
        }
       *[message] { $count ->
           *[other] གསལ་གཞི་གུ་ སྐར་མ་བཀལ་ཡོད་པའི་འཕྲིན་དོན་ { $count } ཆ་མཉམ་གདམ་ཡོད།
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
           *[other] གསལ་གཞི་གུ་ སྐར་མ་མ་བཀལ་བའི་གླེང་མོལ་ { $count } ཆ་མཉམ་གདམ་ཡོད།
        }
       *[message] { $count ->
           *[other] གསལ་གཞི་གུ་ སྐར་མ་མ་བཀལ་བའི་འཕྲིན་དོན་ { $count } ཆ་མཉམ་གདམ་ཡོད།
        }
    }
}
list-select-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
           *[other] ལྷག་ཡོད་པའི་གླེང་མོལ་ { $count } ཆ་མཉམ་གདམ།
        }
       *[message] { $count ->
           *[other] ལྷག་ཡོད་པའི་འཕྲིན་དོན་ { $count } ཆ་མཉམ་གདམ།
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
           *[other] མ་ལྷག་པའི་གླེང་མོལ་ { $count } ཆ་མཉམ་གདམ།
        }
       *[message] { $count ->
           *[other] མ་ལྷག་པའི་འཕྲིན་དོན་ { $count } ཆ་མཉམ་གདམ།
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
           *[other] སྐར་མ་བཀལ་ཡོད་པའི་གླེང་མོལ་ { $count } ཆ་མཉམ་གདམ།
        }
       *[message] { $count ->
           *[other] སྐར་མ་བཀལ་ཡོད་པའི་འཕྲིན་དོན་ { $count } ཆ་མཉམ་གདམ།
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
           *[other] སྐར་མ་མ་བཀལ་བའི་གླེང་མོལ་ { $count } ཆ་མཉམ་གདམ།
        }
       *[message] { $count ->
           *[other] སྐར་མ་མ་བཀལ་བའི་འཕྲིན་དོན་ { $count } ཆ་མཉམ་གདམ།
        }
    }
}
list-select-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
           *[other] { $folder } ནང་གི་ལྷག་ཡོད་པའི་གླེང་མོལ་ { $count } ཆ་མཉམ་གདམ།
        }
       *[message] { $count ->
           *[other] { $folder } ནང་གི་ལྷག་ཡོད་པའི་འཕྲིན་དོན་ { $count } ཆ་མཉམ་གདམ།
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
           *[other] { $folder } ནང་གི་མ་ལྷག་པའི་གླེང་མོལ་ { $count } ཆ་མཉམ་གདམ།
        }
       *[message] { $count ->
           *[other] { $folder } ནང་གི་མ་ལྷག་པའི་འཕྲིན་དོན་ { $count } ཆ་མཉམ་གདམ།
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
           *[other] { $folder } ནང་གི་སྐར་མ་བཀལ་ཡོད་པའི་གླེང་མོལ་ { $count } ཆ་མཉམ་གདམ།
        }
       *[message] { $count ->
           *[other] { $folder } ནང་གི་སྐར་མ་བཀལ་ཡོད་པའི་འཕྲིན་དོན་ { $count } ཆ་མཉམ་གདམ།
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
           *[other] { $folder } ནང་གི་སྐར་མ་མ་བཀལ་བའི་གླེང་མོལ་ { $count } ཆ་མཉམ་གདམ།
        }
       *[message] { $count ->
           *[other] { $folder } ནང་གི་སྐར་མ་མ་བཀལ་བའི་འཕྲིན་དོན་ { $count } ཆ་མཉམ་གདམ།
        }
    }
}
list-selected-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
           *[other] ལྷག་ཡོད་པའི་གླེང་མོལ་ { $count } ཆ་མཉམ་གདམ་ཡོད།
        }
       *[message] { $count ->
           *[other] ལྷག་ཡོད་པའི་འཕྲིན་དོན་ { $count } ཆ་མཉམ་གདམ་ཡོད།
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
           *[other] མ་ལྷག་པའི་གླེང་མོལ་ { $count } ཆ་མཉམ་གདམ་ཡོད།
        }
       *[message] { $count ->
           *[other] མ་ལྷག་པའི་འཕྲིན་དོན་ { $count } ཆ་མཉམ་གདམ་ཡོད།
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
           *[other] སྐར་མ་བཀལ་ཡོད་པའི་གླེང་མོལ་ { $count } ཆ་མཉམ་གདམ་ཡོད།
        }
       *[message] { $count ->
           *[other] སྐར་མ་བཀལ་ཡོད་པའི་འཕྲིན་དོན་ { $count } ཆ་མཉམ་གདམ་ཡོད།
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
           *[other] སྐར་མ་མ་བཀལ་བའི་གླེང་མོལ་ { $count } ཆ་མཉམ་གདམ་ཡོད།
        }
       *[message] { $count ->
           *[other] སྐར་མ་མ་བཀལ་བའི་འཕྲིན་དོན་ { $count } ཆ་མཉམ་གདམ་ཡོད།
        }
    }
}
list-selected-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
           *[other] { $folder } ནང་གི་ལྷག་ཡོད་པའི་གླེང་མོལ་ { $count } ཆ་མཉམ་གདམ་ཡོད།
        }
       *[message] { $count ->
           *[other] { $folder } ནང་གི་ལྷག་ཡོད་པའི་འཕྲིན་དོན་ { $count } ཆ་མཉམ་གདམ་ཡོད།
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
           *[other] { $folder } ནང་གི་མ་ལྷག་པའི་གླེང་མོལ་ { $count } ཆ་མཉམ་གདམ་ཡོད།
        }
       *[message] { $count ->
           *[other] { $folder } ནང་གི་མ་ལྷག་པའི་འཕྲིན་དོན་ { $count } ཆ་མཉམ་གདམ་ཡོད།
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
           *[other] { $folder } ནང་གི་སྐར་མ་བཀལ་ཡོད་པའི་གླེང་མོལ་ { $count } ཆ་མཉམ་གདམ་ཡོད།
        }
       *[message] { $count ->
           *[other] { $folder } ནང་གི་སྐར་མ་བཀལ་ཡོད་པའི་འཕྲིན་དོན་ { $count } ཆ་མཉམ་གདམ་ཡོད།
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
           *[other] { $folder } ནང་གི་སྐར་མ་མ་བཀལ་བའི་གླེང་མོལ་ { $count } ཆ་མཉམ་གདམ་ཡོད།
        }
       *[message] { $count ->
           *[other] { $folder } ནང་གི་སྐར་མ་མ་བཀལ་བའི་འཕྲིན་དོན་ { $count } ཆ་མཉམ་གདམ་ཡོད།
        }
    }
}
list-picked-none = { $pick ->
    [read] { $kind ->
        [conversation] ནཱ་ལུ་ ལྷག་ཡོད་པའི་གླེང་མོལ་མེད།
       *[message] ནཱ་ལུ་ ལྷག་ཡོད་པའི་འཕྲིན་དོན་མེད།
    }
   *[unread] { $kind ->
        [conversation] ནཱ་ལུ་ མ་ལྷག་པའི་གླེང་མོལ་མེད།
       *[message] ནཱ་ལུ་ མ་ལྷག་པའི་འཕྲིན་དོན་མེད།
    }
    [starred] { $kind ->
        [conversation] ནཱ་ལུ་ སྐར་མ་བཀལ་ཡོད་པའི་གླེང་མོལ་མེད།
       *[message] ནཱ་ལུ་ སྐར་མ་བཀལ་ཡོད་པའི་འཕྲིན་དོན་མེད།
    }
    [unstarred] { $kind ->
        [conversation] ནཱ་ལུ་ སྐར་མ་མ་བཀལ་བའི་གླེང་མོལ་མེད།
       *[message] ནཱ་ལུ་ སྐར་མ་མ་བཀལ་བའི་འཕྲིན་དོན་མེད།
    }
}
list-clear-selection = གདམ་ཁ་བསལ།

## Mail list: empty states

list-empty-search = ཁྱོད་ཀྱི་འཚོལ་ཞིབ་དང་མཐུན་པའི་འཕྲིན་དོན་མིན་འདུག
list-empty-tab = { $tab } ནང་གློག་འཕྲིན་མིན་འདུག
list-empty-tab-unknown = ཤོག་མཚན་འདི་ནང་གློག་འཕྲིན་མིན་འདུག
list-empty-folder = { $folder } ནང་འཕྲིན་དོན་མིན་འདུག
list-empty-folder-unknown = སྣོད་འཛིན་འདི་ནང་འཕྲིན་དོན་མིན་འདུག
list-first-sync = ཁྱོད་ཀྱི་གློག་འཕྲིན་ལེན་དོ…
list-first-sync-detail = གློག་འཕྲིན་འབྱོར་བའི་བསྒང་ ནཱ་ལུ་སྟོནམ་ཨིན།

## Mail list: lines

row-removed = འཕྲིན་དོན་འདི་བཏོན་གཏང་ཡི།
row-starred = སྐར་མ་བཀལ་ཡོདཔ
row-not-starred = སྐར་མ་མ་བཀལ་བ
row-important = གལ་ཅན། གལ་ཅན་མེན་པ་སྦེ་རྟགས་བཀལ་ནི་ལུ་ ཨེབ་གཏང་འབད།
row-mark-important = གལ་ཅན་སྦེ་རྟགས་བཀལ།
row-pinned = ཡར་སྟོད་ལུ་བཙུགས་ཡོདཔ
row-task = ལཱ
row-task-open = ལཱ་ཁ་ཕྱེ་བ: { $title }
row-tracking-none = རྗེས་འཚོལ་འབད་དོ། ད་ཚུན་ ཁ་མ་ཕྱེ་བས
row-tracking-opened = { $recipients } ལས་ { $opened } གིས་ཁ་ཕྱེ་ཡོདཔ
row-tracking-clicked = { $recipients } ལས་ { $opened } གིས་ཁ་ཕྱེ་ཡོདཔ། { $clicked } གིས་ འབྲེལ་མཐུད་ཁ་ཕྱེ་ཡོདཔ
row-pin = ཡར་སྟོད་ལུ་བཙུགས།
row-unpin = བཙུགས་མི་བཏོན།
row-snoozed-until = { $when } ཚུན་ཚོད་ ཤུལ་མར་བཞག་ཡོདཔ

## Mail list: More menu and right-click menu

menu-reply = ལན་སློག
menu-reply-all = ཆ་མཉམ་ལུ་ལན་སློག
menu-forward = མདུན་སྐྱེལ་འབད།
menu-archive = ཡིག་མཛོད་ནང་བཙུགས།
menu-delete = བཏོན་གཏང་།
menu-delete-forever = ཨ་རྟག་གི་དོན་ལུ་བཏོན་གཏང་།
menu-move-to-inbox = ནང་འབྱོར་སྒྲོམ་ལུ་སྤོ།
menu-spam = སྤེམ་སྦེ་སྙན་ཞུ་འབད།
menu-not-spam = སྤེམ་མེན།
menu-mark-read = ལྷག་ཡོདཔ་སྦེ་རྟགས་བཀལ།
menu-mark-unread = མ་ལྷག་པ་སྦེ་རྟགས་བཀལ།
menu-mark-all-read = ཆ་མཉམ་ལྷག་ཡོདཔ་སྦེ་རྟགས་བཀལ།
menu-star = སྐར་མ་བཀལ།
menu-unstar = སྐར་མ་བཏོན།
menu-important = གལ་ཅན་སྦེ་རྟགས་བཀལ།
menu-not-important = གལ་ཅན་མེན་པ་སྦེ་རྟགས་བཀལ།
menu-pin = ཡར་སྟོད་ལུ་བཙུགས།
menu-unpin = བཙུགས་མི་བཏོན།
menu-snooze = ཤུལ་མར་བཞག།
menu-unsnooze = ཤུལ་མར་བཞག་མི་བཏོན།
menu-add-to-tasks = ལཱ་ནང་ཁ་སྣོན་འབད།
menu-schedule-meeting = ཞལ་འཛོམས་ཅིག་ལུ་ དུས་ཚོད་བཞག།
menu-start-call = བརྙན་ཁ་པར་འགོ་བཙུགས།
menu-add-note = དྲན་ཐོ་ཅིག་ཁ་སྣོན་འབད།
menu-print-all = ཆ་མཉམ་དཔར་བསྐྲུན་འབད།
menu-new-window = སྒོ་སྒྲིག་གསརཔ་ནང་ཁ་ཕྱེ།
menu-move-to = ལུ་སྤོ།
menu-move-to-heading = ལུ་སྤོ:
menu-find-from = { $name } ལས་འོང་མི་གློག་འཕྲིན་ཚུ་འཚོལ།

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] གླེང་མོལ་ { $count } ཡིག་མཛོད་ནང་བཙུགས་ཡི།
   *[message] འཕྲིན་དོན་ { $count } ཡིག་མཛོད་ནང་བཙུགས་ཡི།
}
toast-trashed = { $kind ->
    [conversation] གླེང་མོལ་ { $count } གད་སྙིགས་ནང་སྤོ་ཡི།
   *[message] འཕྲིན་དོན་ { $count } གད་སྙིགས་ནང་སྤོ་ཡི།
}
toast-moved = { $kind ->
    [conversation] གླེང་མོལ་ { $count } སྤོ་ཡི།
   *[message] འཕྲིན་དོན་ { $count } སྤོ་ཡི།
}
toast-starred = { $kind ->
    [conversation] གླེང་མོལ་ { $count } ལུ་སྐར་མ་བཀལ་ཡི།
   *[message] འཕྲིན་དོན་ { $count } ལུ་སྐར་མ་བཀལ་ཡི།
}
toast-unstarred = { $kind ->
    [conversation] གླེང་མོལ་ { $count } ལས་སྐར་མ་བཏོན་ཡི།
   *[message] འཕྲིན་དོན་ { $count } ལས་སྐར་མ་བཏོན་ཡི།
}
toast-important = { $kind ->
    [conversation] གླེང་མོལ་ { $count } གལ་ཅན་སྦེ་རྟགས་བཀལ་ཡི།
   *[message] འཕྲིན་དོན་ { $count } གལ་ཅན་སྦེ་རྟགས་བཀལ་ཡི།
}
toast-not-important = { $kind ->
    [conversation] གླེང་མོལ་ { $count } གལ་ཅན་མེན་པ་སྦེ་རྟགས་བཀལ་ཡི།
   *[message] འཕྲིན་དོན་ { $count } གལ་ཅན་མེན་པ་སྦེ་རྟགས་བཀལ་ཡི།
}
toast-pinned = { $kind ->
    [conversation] གླེང་མོལ་ { $count } ཡར་སྟོད་ལུ་བཙུགས་ཡི།
   *[message] འཕྲིན་དོན་ { $count } ཡར་སྟོད་ལུ་བཙུགས་ཡི།
}
toast-unpinned = { $kind ->
    [conversation] གླེང་མོལ་ { $count } གི་བཙུགས་མི་བཏོན་ཡི།
   *[message] འཕྲིན་དོན་ { $count } གི་བཙུགས་མི་བཏོན་ཡི།
}
toast-snoozed = { $kind ->
    [conversation] གླེང་མོལ་ { $count } { $when } ཚུན་ཚོད་ ཤུལ་མར་བཞག་ཡི།
   *[message] འཕྲིན་དོན་ { $count } { $when } ཚུན་ཚོད་ ཤུལ་མར་བཞག་ཡི།
}
toast-unsnoozed = { $kind ->
    [conversation] གླེང་མོལ་ { $count } ནང་འབྱོར་སྒྲོམ་ལུ་ ལོག་འོང་ཡི།
   *[message] འཕྲིན་དོན་ { $count } ནང་འབྱོར་སྒྲོམ་ལུ་ ལོག་འོང་ཡི།
}
toast-spam = { $kind ->
    [conversation] གླེང་མོལ་ { $count } སྤེམ་སྦེ་སྙན་ཞུ་འབད་ཡི།
   *[message] འཕྲིན་དོན་ { $count } སྤེམ་སྦེ་སྙན་ཞུ་འབད་ཡི།
}
toast-not-spam = { $kind ->
    [conversation] གླེང་མོལ་ { $count } སྤེམ་མེན་པའི་རྟགས་བཀལ་ཏེ་ ནང་འབྱོར་སྒྲོམ་ལུ་སྤོ་ཡི།
   *[message] འཕྲིན་དོན་ { $count } སྤེམ་མེན་པའི་རྟགས་བཀལ་ཏེ་ ནང་འབྱོར་སྒྲོམ་ལུ་སྤོ་ཡི།
}
toast-deleted-forever = { $kind ->
    [conversation] གླེང་མོལ་ { $count } ཨ་རྟག་གི་དོན་ལུ་བཏོན་གཏང་ཡི།
   *[message] འཕྲིན་དོན་ { $count } ཨ་རྟག་གི་དོན་ལུ་བཏོན་གཏང་ཡི།
}
toast-marked-read = { $kind ->
    [conversation] { $count ->
       *[other] གླེང་མོལ་ { $count } ལྷག་ཡོདཔ་སྦེ་རྟགས་བཀལ་ཡི།
    }
   *[message] { $count ->
       *[other] འཕྲིན་དོན་ { $count } ལྷག་ཡོདཔ་སྦེ་རྟགས་བཀལ་ཡི།
    }
}
toast-marked-unread = { $kind ->
    [conversation] { $count ->
       *[other] གླེང་མོལ་ { $count } མ་ལྷག་པ་སྦེ་རྟགས་བཀལ་ཡི།
    }
   *[message] { $count ->
       *[other] འཕྲིན་དོན་ { $count } མ་ལྷག་པ་སྦེ་རྟགས་བཀལ་ཡི།
    }
}
toast-undone = བྱ་བ་འབད་བཤོལ་འབད་ཡི།
toast-nothing-to-undo = འབད་བཤོལ་ནི་ག་ནི་ཡང་མེད།
toast-cannot-undo-delete-forever = ཨ་རྟག་གི་དོན་ལུ་བཏོན་གཏང་ཡོད་པའི་གློག་འཕྲིན་ ལོག་ལེན་མི་ཚུགས།
toast-send-undone = གཏང་ནི་ འབད་བཤོལ་འབད་ཡི།
toast-too-late-to-undo-send = འབད་བཤོལ་ནི་ལུ་ ཕྱི་རུ་སོང་ཡི: འཕྲིན་དོན་འདི་ ཧེ་མ་ལས་རང་ གཏང་ཚར་ཡི།
toast-undo = འབད་བཤོལ།
toast-close = ཁ་བསྡམས།
toast-no-spam-folder = རྩིས་ཐོ་འདི་ལུ་ སྤེམ་སྣོད་འཛིན་མིན་འདུག
