# Katna Mail, Kannada (ಕನ್ನಡ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = ಪ್ರಾಥಮಿಕ
tab-promotions = ಪ್ರಚಾರಗಳು
tab-social = ಸಾಮಾಜಿಕ
tab-updates = ಅಪ್‌ಡೇಟ್‌ಗಳು
tab-forums = ಫೋರಮ್‌ಗಳು
tab-focused = ಕೇಂದ್ರೀಕೃತ
tab-other = ಇತರೆ
tab-inbox = ಇನ್‌ಬಾಕ್ಸ್
tab-newsletters = ಸುದ್ದಿಪತ್ರಗಳು
tab-notifications = ಅಧಿಸೂಚನೆಗಳು
tab-new = { $count } ಹೊಸವು
tab-provider-other = Katna ವಿಂಗಡಿಸಿದ್ದು

## Mail list: toolbar

list-select = ಆಯ್ಕೆಮಾಡಿ
list-refresh = ರಿಫ್ರೆಶ್ ಮಾಡಿ
list-more = ಇನ್ನಷ್ಟು
list-mark-read = ಓದಲಾಗಿದೆ ಎಂದು ಗುರುತಿಸಿ
list-mark-unread = ಓದಿಲ್ಲ ಎಂದು ಗುರುತಿಸಿ
list-move-to = ಇಲ್ಲಿಗೆ ಸರಿಸಿ
list-archive = ಆರ್ಕೈವ್ ಮಾಡಿ
list-spam = ಸ್ಪ್ಯಾಮ್ ಎಂದು ವರದಿ ಮಾಡಿ
list-delete = ಅಳಿಸಿ
list-newer = ಹೊಸದು
list-older = ಹಳೆಯದು
list-range = { $total } ರಲ್ಲಿ { $first }–{ $last }
list-range-about = ಸುಮಾರು { $total } ರಲ್ಲಿ { $first }–{ $last }
list-results = “{ $query }” ಗಾಗಿ ಫಲಿತಾಂಶಗಳು
list-results-corrected = “{ $query }” ಗಾಗಿ ಫಲಿತಾಂಶಗಳನ್ನು ತೋರಿಸಲಾಗುತ್ತಿದೆ
list-search-instead = ಬದಲಿಗೆ “{ $query }” ಗಾಗಿ ಹುಡುಕಿ
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = ಎಲ್ಲಾ
list-pick-none = ಯಾವುದೂ ಇಲ್ಲ
list-pick-read = ಓದಿರುವುದು
list-pick-unread = ಓದದಿರುವುದು
list-pick-starred = ನಕ್ಷತ್ರ ಹಾಕಿರುವುದು
list-pick-unstarred = ನಕ್ಷತ್ರ ಹಾಕದಿರುವುದು

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] ಎಲ್ಲಾ { $count } ಸಂವಾದವನ್ನು ಆಯ್ಕೆಮಾಡಲಾಗಿದೆ.
       *[other] ಎಲ್ಲಾ { $count } ಸಂವಾದಗಳನ್ನು ಆಯ್ಕೆಮಾಡಲಾಗಿದೆ.
    }
   *[message] { $count ->
        [one] ಎಲ್ಲಾ { $count } ಸಂದೇಶವನ್ನು ಆಯ್ಕೆಮಾಡಲಾಗಿದೆ.
       *[other] ಎಲ್ಲಾ { $count } ಸಂದೇಶಗಳನ್ನು ಆಯ್ಕೆಮಾಡಲಾಗಿದೆ.
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder } ನಲ್ಲಿರುವ ಎಲ್ಲಾ { $count } ಸಂವಾದವನ್ನು ಆಯ್ಕೆಮಾಡಲಾಗಿದೆ.
       *[other] { $folder } ನಲ್ಲಿರುವ ಎಲ್ಲಾ { $count } ಸಂವಾದಗಳನ್ನು ಆಯ್ಕೆಮಾಡಲಾಗಿದೆ.
    }
   *[message] { $count ->
        [one] { $folder } ನಲ್ಲಿರುವ ಎಲ್ಲಾ { $count } ಸಂದೇಶವನ್ನು ಆಯ್ಕೆಮಾಡಲಾಗಿದೆ.
       *[other] { $folder } ನಲ್ಲಿರುವ ಎಲ್ಲಾ { $count } ಸಂದೇಶಗಳನ್ನು ಆಯ್ಕೆಮಾಡಲಾಗಿದೆ.
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] ಪರದೆಯಲ್ಲಿರುವ ಎಲ್ಲಾ { $count } ಸಂವಾದವನ್ನು ಆಯ್ಕೆಮಾಡಲಾಗಿದೆ.
       *[other] ಪರದೆಯಲ್ಲಿರುವ ಎಲ್ಲಾ { $count } ಸಂವಾದಗಳನ್ನು ಆಯ್ಕೆಮಾಡಲಾಗಿದೆ.
    }
   *[message] { $count ->
        [one] ಪರದೆಯಲ್ಲಿರುವ ಎಲ್ಲಾ { $count } ಸಂದೇಶವನ್ನು ಆಯ್ಕೆಮಾಡಲಾಗಿದೆ.
       *[other] ಪರದೆಯಲ್ಲಿರುವ ಎಲ್ಲಾ { $count } ಸಂದೇಶಗಳನ್ನು ಆಯ್ಕೆಮಾಡಲಾಗಿದೆ.
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] ಎಲ್ಲಾ { $count } ಸಂವಾದವನ್ನು ಆಯ್ಕೆಮಾಡಿ
       *[other] ಎಲ್ಲಾ { $count } ಸಂವಾದಗಳನ್ನು ಆಯ್ಕೆಮಾಡಿ
    }
   *[message] { $count ->
        [one] ಎಲ್ಲಾ { $count } ಸಂದೇಶವನ್ನು ಆಯ್ಕೆಮಾಡಿ
       *[other] ಎಲ್ಲಾ { $count } ಸಂದೇಶಗಳನ್ನು ಆಯ್ಕೆಮಾಡಿ
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder } ನಲ್ಲಿರುವ ಎಲ್ಲಾ { $count } ಸಂವಾದವನ್ನು ಆಯ್ಕೆಮಾಡಿ
       *[other] { $folder } ನಲ್ಲಿರುವ ಎಲ್ಲಾ { $count } ಸಂವಾದಗಳನ್ನು ಆಯ್ಕೆಮಾಡಿ
    }
   *[message] { $count ->
        [one] { $folder } ನಲ್ಲಿರುವ ಎಲ್ಲಾ { $count } ಸಂದೇಶವನ್ನು ಆಯ್ಕೆಮಾಡಿ
       *[other] { $folder } ನಲ್ಲಿರುವ ಎಲ್ಲಾ { $count } ಸಂದೇಶಗಳನ್ನು ಆಯ್ಕೆಮಾಡಿ
    }
}
list-clear-selection = ಆಯ್ಕೆಯನ್ನು ತೆರವುಗೊಳಿಸಿ

## Mail list: empty states

list-empty-search = ನಿಮ್ಮ ಹುಡುಕಾಟಕ್ಕೆ ಹೊಂದಿಕೆಯಾಗುವ ಯಾವುದೇ ಸಂದೇಶಗಳಿಲ್ಲ.
list-empty-tab = { $tab } ನಲ್ಲಿ ಯಾವುದೇ ಮೇಲ್ ಇಲ್ಲ.
list-empty-tab-unknown = ಈ ಟ್ಯಾಬ್‌ನಲ್ಲಿ ಯಾವುದೇ ಮೇಲ್ ಇಲ್ಲ.
list-empty-folder = { $folder } ನಲ್ಲಿ ಯಾವುದೇ ಸಂದೇಶಗಳಿಲ್ಲ.
list-empty-folder-unknown = ಈ ಫೋಲ್ಡರ್‌ನಲ್ಲಿ ಯಾವುದೇ ಸಂದೇಶಗಳಿಲ್ಲ.
list-first-sync = ನಿಮ್ಮ ಮೇಲ್ ಪಡೆಯಲಾಗುತ್ತಿದೆ…
list-first-sync-detail = ಮೇಲ್ ಬಂದಂತೆ ಇಲ್ಲಿ ಕಾಣಿಸುತ್ತದೆ.

## Mail list: lines

row-removed = ಈ ಸಂದೇಶವನ್ನು ತೆಗೆದುಹಾಕಲಾಗಿದೆ.
row-starred = ನಕ್ಷತ್ರ ಹಾಕಲಾಗಿದೆ
row-not-starred = ನಕ್ಷತ್ರ ಹಾಕಿಲ್ಲ
row-important = ಪ್ರಮುಖ. ಪ್ರಮುಖವಲ್ಲ ಎಂದು ಗುರುತಿಸಲು ಕ್ಲಿಕ್ ಮಾಡಿ.
row-mark-important = ಪ್ರಮುಖ ಎಂದು ಗುರುತಿಸಿ
row-pinned = ಮೇಲ್ಭಾಗಕ್ಕೆ ಪಿನ್ ಮಾಡಲಾಗಿದೆ
row-pin = ಮೇಲ್ಭಾಗಕ್ಕೆ ಪಿನ್ ಮಾಡಿ
row-unpin = ಅನ್‌ಪಿನ್ ಮಾಡಿ

## Mail list: More menu and right-click menu

menu-reply = ಪ್ರತ್ಯುತ್ತರಿಸಿ
menu-reply-all = ಎಲ್ಲರಿಗೂ ಪ್ರತ್ಯುತ್ತರಿಸಿ
menu-forward = ಫಾರ್ವರ್ಡ್ ಮಾಡಿ
menu-archive = ಆರ್ಕೈವ್ ಮಾಡಿ
menu-delete = ಅಳಿಸಿ
menu-spam = ಸ್ಪ್ಯಾಮ್ ಎಂದು ವರದಿ ಮಾಡಿ
menu-mark-read = ಓದಲಾಗಿದೆ ಎಂದು ಗುರುತಿಸಿ
menu-mark-unread = ಓದಿಲ್ಲ ಎಂದು ಗುರುತಿಸಿ
menu-mark-all-read = ಎಲ್ಲವನ್ನೂ ಓದಲಾಗಿದೆ ಎಂದು ಗುರುತಿಸಿ
menu-star = ನಕ್ಷತ್ರ ಸೇರಿಸಿ
menu-unstar = ನಕ್ಷತ್ರ ತೆಗೆದುಹಾಕಿ
menu-important = ಪ್ರಮುಖ ಎಂದು ಗುರುತಿಸಿ
menu-not-important = ಪ್ರಮುಖವಲ್ಲ ಎಂದು ಗುರುತಿಸಿ
menu-pin = ಮೇಲ್ಭಾಗಕ್ಕೆ ಪಿನ್ ಮಾಡಿ
menu-unpin = ಅನ್‌ಪಿನ್ ಮಾಡಿ
menu-print-all = ಎಲ್ಲವನ್ನೂ ಮುದ್ರಿಸಿ
menu-new-window = ಹೊಸ ವಿಂಡೋದಲ್ಲಿ ತೆರೆಯಿರಿ
menu-move-to = ಇಲ್ಲಿಗೆ ಸರಿಸಿ
menu-move-to-heading = ಇಲ್ಲಿಗೆ ಸರಿಸಿ:
menu-find-from = { $name } ಅವರಿಂದ ಬಂದ ಇಮೇಲ್‌ಗಳನ್ನು ಹುಡುಕಿ

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] ಸಂವಾದವನ್ನು ಆರ್ಕೈವ್ ಮಾಡಲಾಗಿದೆ.
       *[other] { $count } ಸಂವಾದಗಳನ್ನು ಆರ್ಕೈವ್ ಮಾಡಲಾಗಿದೆ.
    }
   *[message] { $count ->
        [one] ಸಂದೇಶವನ್ನು ಆರ್ಕೈವ್ ಮಾಡಲಾಗಿದೆ.
       *[other] { $count } ಸಂದೇಶಗಳನ್ನು ಆರ್ಕೈವ್ ಮಾಡಲಾಗಿದೆ.
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] ಸಂವಾದವನ್ನು ಅನುಪಯುಕ್ತಕ್ಕೆ ಸರಿಸಲಾಗಿದೆ.
       *[other] { $count } ಸಂವಾದಗಳನ್ನು ಅನುಪಯುಕ್ತಕ್ಕೆ ಸರಿಸಲಾಗಿದೆ.
    }
   *[message] { $count ->
        [one] ಸಂದೇಶವನ್ನು ಅನುಪಯುಕ್ತಕ್ಕೆ ಸರಿಸಲಾಗಿದೆ.
       *[other] { $count } ಸಂದೇಶಗಳನ್ನು ಅನುಪಯುಕ್ತಕ್ಕೆ ಸರಿಸಲಾಗಿದೆ.
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] ಸಂವಾದವನ್ನು ಸರಿಸಲಾಗಿದೆ.
       *[other] { $count } ಸಂವಾದಗಳನ್ನು ಸರಿಸಲಾಗಿದೆ.
    }
   *[message] { $count ->
        [one] ಸಂದೇಶವನ್ನು ಸರಿಸಲಾಗಿದೆ.
       *[other] { $count } ಸಂದೇಶಗಳನ್ನು ಸರಿಸಲಾಗಿದೆ.
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] ಸಂವಾದಕ್ಕೆ ನಕ್ಷತ್ರ ಹಾಕಲಾಗಿದೆ.
       *[other] { $count } ಸಂವಾದಗಳಿಗೆ ನಕ್ಷತ್ರ ಹಾಕಲಾಗಿದೆ.
    }
   *[message] { $count ->
        [one] ಸಂದೇಶಕ್ಕೆ ನಕ್ಷತ್ರ ಹಾಕಲಾಗಿದೆ.
       *[other] { $count } ಸಂದೇಶಗಳಿಗೆ ನಕ್ಷತ್ರ ಹಾಕಲಾಗಿದೆ.
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] ಸಂವಾದದಿಂದ ನಕ್ಷತ್ರ ತೆಗೆದುಹಾಕಲಾಗಿದೆ.
       *[other] { $count } ಸಂವಾದಗಳಿಂದ ನಕ್ಷತ್ರ ತೆಗೆದುಹಾಕಲಾಗಿದೆ.
    }
   *[message] { $count ->
        [one] ಸಂದೇಶದಿಂದ ನಕ್ಷತ್ರ ತೆಗೆದುಹಾಕಲಾಗಿದೆ.
       *[other] { $count } ಸಂದೇಶಗಳಿಂದ ನಕ್ಷತ್ರ ತೆಗೆದುಹಾಕಲಾಗಿದೆ.
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] ಸಂವಾದವನ್ನು ಪ್ರಮುಖ ಎಂದು ಗುರುತಿಸಲಾಗಿದೆ.
       *[other] { $count } ಸಂವಾದಗಳನ್ನು ಪ್ರಮುಖ ಎಂದು ಗುರುತಿಸಲಾಗಿದೆ.
    }
   *[message] { $count ->
        [one] ಸಂದೇಶವನ್ನು ಪ್ರಮುಖ ಎಂದು ಗುರುತಿಸಲಾಗಿದೆ.
       *[other] { $count } ಸಂದೇಶಗಳನ್ನು ಪ್ರಮುಖ ಎಂದು ಗುರುತಿಸಲಾಗಿದೆ.
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] ಸಂವಾದವನ್ನು ಪ್ರಮುಖವಲ್ಲ ಎಂದು ಗುರುತಿಸಲಾಗಿದೆ.
       *[other] { $count } ಸಂವಾದಗಳನ್ನು ಪ್ರಮುಖವಲ್ಲ ಎಂದು ಗುರುತಿಸಲಾಗಿದೆ.
    }
   *[message] { $count ->
        [one] ಸಂದೇಶವನ್ನು ಪ್ರಮುಖವಲ್ಲ ಎಂದು ಗುರುತಿಸಲಾಗಿದೆ.
       *[other] { $count } ಸಂದೇಶಗಳನ್ನು ಪ್ರಮುಖವಲ್ಲ ಎಂದು ಗುರುತಿಸಲಾಗಿದೆ.
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] ಸಂವಾದವನ್ನು ಮೇಲ್ಭಾಗಕ್ಕೆ ಪಿನ್ ಮಾಡಲಾಗಿದೆ.
       *[other] { $count } ಸಂವಾದಗಳನ್ನು ಮೇಲ್ಭಾಗಕ್ಕೆ ಪಿನ್ ಮಾಡಲಾಗಿದೆ.
    }
   *[message] { $count ->
        [one] ಸಂದೇಶವನ್ನು ಮೇಲ್ಭಾಗಕ್ಕೆ ಪಿನ್ ಮಾಡಲಾಗಿದೆ.
       *[other] { $count } ಸಂದೇಶಗಳನ್ನು ಮೇಲ್ಭಾಗಕ್ಕೆ ಪಿನ್ ಮಾಡಲಾಗಿದೆ.
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] ಸಂವಾದವನ್ನು ಅನ್‌ಪಿನ್ ಮಾಡಲಾಗಿದೆ.
       *[other] { $count } ಸಂವಾದಗಳನ್ನು ಅನ್‌ಪಿನ್ ಮಾಡಲಾಗಿದೆ.
    }
   *[message] { $count ->
        [one] ಸಂದೇಶವನ್ನು ಅನ್‌ಪಿನ್ ಮಾಡಲಾಗಿದೆ.
       *[other] { $count } ಸಂದೇಶಗಳನ್ನು ಅನ್‌ಪಿನ್ ಮಾಡಲಾಗಿದೆ.
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] ಸಂವಾದವನ್ನು ಸ್ಪ್ಯಾಮ್ ಎಂದು ವರದಿ ಮಾಡಲಾಗಿದೆ.
       *[other] { $count } ಸಂವಾದಗಳನ್ನು ಸ್ಪ್ಯಾಮ್ ಎಂದು ವರದಿ ಮಾಡಲಾಗಿದೆ.
    }
   *[message] { $count ->
        [one] ಸಂದೇಶವನ್ನು ಸ್ಪ್ಯಾಮ್ ಎಂದು ವರದಿ ಮಾಡಲಾಗಿದೆ.
       *[other] { $count } ಸಂದೇಶಗಳನ್ನು ಸ್ಪ್ಯಾಮ್ ಎಂದು ವರದಿ ಮಾಡಲಾಗಿದೆ.
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] ಸಂವಾದವನ್ನು ಶಾಶ್ವತವಾಗಿ ಅಳಿಸಲಾಗಿದೆ.
       *[other] { $count } ಸಂವಾದಗಳನ್ನು ಶಾಶ್ವತವಾಗಿ ಅಳಿಸಲಾಗಿದೆ.
    }
   *[message] { $count ->
        [one] ಸಂದೇಶವನ್ನು ಶಾಶ್ವತವಾಗಿ ಅಳಿಸಲಾಗಿದೆ.
       *[other] { $count } ಸಂದೇಶಗಳನ್ನು ಶಾಶ್ವತವಾಗಿ ಅಳಿಸಲಾಗಿದೆ.
    }
}
toast-undone = ಕ್ರಿಯೆಯನ್ನು ರದ್ದುಗೊಳಿಸಲಾಗಿದೆ.
toast-undo = ರದ್ದುಗೊಳಿಸಿ
toast-no-spam-folder = ಈ ಖಾತೆಯಲ್ಲಿ ಸ್ಪ್ಯಾಮ್ ಫೋಲ್ಡರ್ ಇಲ್ಲ.
