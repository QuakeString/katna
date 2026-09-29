# Katna Mail, Punjabi (ਪੰਜਾਬੀ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = ਮੁੱਖ
tab-promotions = ਪ੍ਰੋਮੋਸ਼ਨ
tab-social = ਸੋਸ਼ਲ
tab-updates = ਅੱਪਡੇਟ
tab-forums = ਫੋਰਮ
tab-focused = ਕੇਂਦਰਿਤ
tab-other = ਹੋਰ
tab-inbox = ਇਨਬਾਕਸ
tab-newsletters = ਨਿਊਜ਼ਲੈਟਰ
tab-notifications = ਸੂਚਨਾਵਾਂ
tab-new = { $count } ਨਵੇਂ
tab-provider-other = Katna ਵੱਲੋਂ ਛਾਂਟਿਆ ਗਿਆ

## Mail list: toolbar

list-select = ਚੁਣੋ
list-refresh = ਤਾਜ਼ਾ ਕਰੋ
list-checking = ਨਵੀਂ ਮੇਲ ਜਾਂਚੀ ਜਾ ਰਹੀ ਹੈ…
list-more = ਹੋਰ
list-mark-read = ਪੜ੍ਹਿਆ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕਰੋ
list-mark-unread = ਅਣਪੜ੍ਹਿਆ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕਰੋ
list-move-to = ਇੱਥੇ ਭੇਜੋ
list-archive = ਪੁਰਾਲੇਖਬੱਧ ਕਰੋ
list-spam = ਸਪੈਮ ਦੀ ਰਿਪੋਰਟ ਕਰੋ
list-delete = ਮਿਟਾਓ
list-snooze = ਸਨੂਜ਼ ਕਰੋ
list-unsnooze = ਸਨੂਜ਼ ਹਟਾਓ
list-newer = ਨਵੀਆਂ
list-older = ਪੁਰਾਣੀਆਂ
list-range = { $total } ਵਿੱਚੋਂ { $first }–{ $last }
list-range-about = ਲਗਭਗ { $total } ਵਿੱਚੋਂ { $first }–{ $last }
list-results = “{ $query }” ਲਈ ਨਤੀਜੇ
list-results-corrected = “{ $query }” ਲਈ ਨਤੀਜੇ ਦਿਖਾਏ ਜਾ ਰਹੇ ਹਨ
list-search-instead = ਇਸਦੀ ਬਜਾਏ “{ $query }” ਲਈ ਖੋਜੋ
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = ਸਭ
list-pick-none = ਕੋਈ ਨਹੀਂ
list-pick-read = ਪੜ੍ਹੀਆਂ
list-pick-unread = ਅਣਪੜ੍ਹੀਆਂ
list-pick-starred = ਤਾਰਾਬੱਧ
list-pick-unstarred = ਤਾਰਾ-ਰਹਿਤ

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] ਸਾਰੀ { $count } ਗੱਲਬਾਤ ਚੁਣੀ ਗਈ ਹੈ।
       *[other] ਸਾਰੀਆਂ { $count } ਗੱਲਬਾਤਾਂ ਚੁਣੀਆਂ ਗਈਆਂ ਹਨ।
    }
   *[message] { $count ->
        [one] { $count } ਸੁਨੇਹਾ ਚੁਣਿਆ ਗਿਆ ਹੈ।
       *[other] ਸਾਰੇ { $count } ਸੁਨੇਹੇ ਚੁਣੇ ਗਏ ਹਨ।
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder } ਵਿਚਲੀ { $count } ਗੱਲਬਾਤ ਚੁਣੀ ਗਈ ਹੈ।
       *[other] { $folder } ਵਿਚਲੀਆਂ ਸਾਰੀਆਂ { $count } ਗੱਲਬਾਤਾਂ ਚੁਣੀਆਂ ਗਈਆਂ ਹਨ।
    }
   *[message] { $count ->
        [one] { $folder } ਵਿਚਲਾ { $count } ਸੁਨੇਹਾ ਚੁਣਿਆ ਗਿਆ ਹੈ।
       *[other] { $folder } ਵਿਚਲੇ ਸਾਰੇ { $count } ਸੁਨੇਹੇ ਚੁਣੇ ਗਏ ਹਨ।
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] ਸਕ੍ਰੀਨ ਉੱਤੇ { $count } ਗੱਲਬਾਤ ਚੁਣੀ ਗਈ ਹੈ।
       *[other] ਸਕ੍ਰੀਨ ਉੱਤੇ ਸਾਰੀਆਂ { $count } ਗੱਲਬਾਤਾਂ ਚੁਣੀਆਂ ਗਈਆਂ ਹਨ।
    }
   *[message] { $count ->
        [one] ਸਕ੍ਰੀਨ ਉੱਤੇ { $count } ਸੁਨੇਹਾ ਚੁਣਿਆ ਗਿਆ ਹੈ।
       *[other] ਸਕ੍ਰੀਨ ਉੱਤੇ ਸਾਰੇ { $count } ਸੁਨੇਹੇ ਚੁਣੇ ਗਏ ਹਨ।
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] { $count } ਗੱਲਬਾਤ ਚੁਣੋ
       *[other] ਸਾਰੀਆਂ { $count } ਗੱਲਬਾਤਾਂ ਚੁਣੋ
    }
   *[message] { $count ->
        [one] { $count } ਸੁਨੇਹਾ ਚੁਣੋ
       *[other] ਸਾਰੇ { $count } ਸੁਨੇਹੇ ਚੁਣੋ
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder } ਵਿਚਲੀ { $count } ਗੱਲਬਾਤ ਚੁਣੋ
       *[other] { $folder } ਵਿਚਲੀਆਂ ਸਾਰੀਆਂ { $count } ਗੱਲਬਾਤਾਂ ਚੁਣੋ
    }
   *[message] { $count ->
        [one] { $folder } ਵਿਚਲਾ { $count } ਸੁਨੇਹਾ ਚੁਣੋ
       *[other] { $folder } ਵਿਚਲੇ ਸਾਰੇ { $count } ਸੁਨੇਹੇ ਚੁਣੋ
    }
}
list-selected-picked-screen = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] ਸਕ੍ਰੀਨ ਉੱਤੇ { $count } ਪੜ੍ਹੀ ਗੱਲਬਾਤ ਚੁਣੀ ਗਈ ਹੈ।
           *[other] ਸਕ੍ਰੀਨ ਉੱਤੇ ਸਾਰੀਆਂ { $count } ਪੜ੍ਹੀਆਂ ਗੱਲਬਾਤਾਂ ਚੁਣੀਆਂ ਗਈਆਂ ਹਨ।
        }
       *[message] { $count ->
            [one] ਸਕ੍ਰੀਨ ਉੱਤੇ { $count } ਪੜ੍ਹਿਆ ਸੁਨੇਹਾ ਚੁਣਿਆ ਗਿਆ ਹੈ।
           *[other] ਸਕ੍ਰੀਨ ਉੱਤੇ ਸਾਰੇ { $count } ਪੜ੍ਹੇ ਸੁਨੇਹੇ ਚੁਣੇ ਗਏ ਹਨ।
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] ਸਕ੍ਰੀਨ ਉੱਤੇ { $count } ਅਣਪੜ੍ਹੀ ਗੱਲਬਾਤ ਚੁਣੀ ਗਈ ਹੈ।
           *[other] ਸਕ੍ਰੀਨ ਉੱਤੇ ਸਾਰੀਆਂ { $count } ਅਣਪੜ੍ਹੀਆਂ ਗੱਲਬਾਤਾਂ ਚੁਣੀਆਂ ਗਈਆਂ ਹਨ।
        }
       *[message] { $count ->
            [one] ਸਕ੍ਰੀਨ ਉੱਤੇ { $count } ਅਣਪੜ੍ਹਿਆ ਸੁਨੇਹਾ ਚੁਣਿਆ ਗਿਆ ਹੈ।
           *[other] ਸਕ੍ਰੀਨ ਉੱਤੇ ਸਾਰੇ { $count } ਅਣਪੜ੍ਹੇ ਸੁਨੇਹੇ ਚੁਣੇ ਗਏ ਹਨ।
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] ਸਕ੍ਰੀਨ ਉੱਤੇ { $count } ਤਾਰਾਬੱਧ ਗੱਲਬਾਤ ਚੁਣੀ ਗਈ ਹੈ।
           *[other] ਸਕ੍ਰੀਨ ਉੱਤੇ ਸਾਰੀਆਂ { $count } ਤਾਰਾਬੱਧ ਗੱਲਬਾਤਾਂ ਚੁਣੀਆਂ ਗਈਆਂ ਹਨ।
        }
       *[message] { $count ->
            [one] ਸਕ੍ਰੀਨ ਉੱਤੇ { $count } ਤਾਰਾਬੱਧ ਸੁਨੇਹਾ ਚੁਣਿਆ ਗਿਆ ਹੈ।
           *[other] ਸਕ੍ਰੀਨ ਉੱਤੇ ਸਾਰੇ { $count } ਤਾਰਾਬੱਧ ਸੁਨੇਹੇ ਚੁਣੇ ਗਏ ਹਨ।
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] ਸਕ੍ਰੀਨ ਉੱਤੇ { $count } ਤਾਰਾ-ਰਹਿਤ ਗੱਲਬਾਤ ਚੁਣੀ ਗਈ ਹੈ।
           *[other] ਸਕ੍ਰੀਨ ਉੱਤੇ ਸਾਰੀਆਂ { $count } ਤਾਰਾ-ਰਹਿਤ ਗੱਲਬਾਤਾਂ ਚੁਣੀਆਂ ਗਈਆਂ ਹਨ।
        }
       *[message] { $count ->
            [one] ਸਕ੍ਰੀਨ ਉੱਤੇ { $count } ਤਾਰਾ-ਰਹਿਤ ਸੁਨੇਹਾ ਚੁਣਿਆ ਗਿਆ ਹੈ।
           *[other] ਸਕ੍ਰੀਨ ਉੱਤੇ ਸਾਰੇ { $count } ਤਾਰਾ-ਰਹਿਤ ਸੁਨੇਹੇ ਚੁਣੇ ਗਏ ਹਨ।
        }
    }
}
list-select-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $count } ਪੜ੍ਹੀ ਗੱਲਬਾਤ ਚੁਣੋ
           *[other] ਸਾਰੀਆਂ { $count } ਪੜ੍ਹੀਆਂ ਗੱਲਬਾਤਾਂ ਚੁਣੋ
        }
       *[message] { $count ->
            [one] { $count } ਪੜ੍ਹਿਆ ਸੁਨੇਹਾ ਚੁਣੋ
           *[other] ਸਾਰੇ { $count } ਪੜ੍ਹੇ ਸੁਨੇਹੇ ਚੁਣੋ
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $count } ਅਣਪੜ੍ਹੀ ਗੱਲਬਾਤ ਚੁਣੋ
           *[other] ਸਾਰੀਆਂ { $count } ਅਣਪੜ੍ਹੀਆਂ ਗੱਲਬਾਤਾਂ ਚੁਣੋ
        }
       *[message] { $count ->
            [one] { $count } ਅਣਪੜ੍ਹਿਆ ਸੁਨੇਹਾ ਚੁਣੋ
           *[other] ਸਾਰੇ { $count } ਅਣਪੜ੍ਹੇ ਸੁਨੇਹੇ ਚੁਣੋ
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $count } ਤਾਰਾਬੱਧ ਗੱਲਬਾਤ ਚੁਣੋ
           *[other] ਸਾਰੀਆਂ { $count } ਤਾਰਾਬੱਧ ਗੱਲਬਾਤਾਂ ਚੁਣੋ
        }
       *[message] { $count ->
            [one] { $count } ਤਾਰਾਬੱਧ ਸੁਨੇਹਾ ਚੁਣੋ
           *[other] ਸਾਰੇ { $count } ਤਾਰਾਬੱਧ ਸੁਨੇਹੇ ਚੁਣੋ
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $count } ਤਾਰਾ-ਰਹਿਤ ਗੱਲਬਾਤ ਚੁਣੋ
           *[other] ਸਾਰੀਆਂ { $count } ਤਾਰਾ-ਰਹਿਤ ਗੱਲਬਾਤਾਂ ਚੁਣੋ
        }
       *[message] { $count ->
            [one] { $count } ਤਾਰਾ-ਰਹਿਤ ਸੁਨੇਹਾ ਚੁਣੋ
           *[other] ਸਾਰੇ { $count } ਤਾਰਾ-ਰਹਿਤ ਸੁਨੇਹੇ ਚੁਣੋ
        }
    }
}
list-select-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $folder } ਵਿਚਲੀ { $count } ਪੜ੍ਹੀ ਗੱਲਬਾਤ ਚੁਣੋ
           *[other] { $folder } ਵਿਚਲੀਆਂ ਸਾਰੀਆਂ { $count } ਪੜ੍ਹੀਆਂ ਗੱਲਬਾਤਾਂ ਚੁਣੋ
        }
       *[message] { $count ->
            [one] { $folder } ਵਿਚਲਾ { $count } ਪੜ੍ਹਿਆ ਸੁਨੇਹਾ ਚੁਣੋ
           *[other] { $folder } ਵਿਚਲੇ ਸਾਰੇ { $count } ਪੜ੍ਹੇ ਸੁਨੇਹੇ ਚੁਣੋ
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $folder } ਵਿਚਲੀ { $count } ਅਣਪੜ੍ਹੀ ਗੱਲਬਾਤ ਚੁਣੋ
           *[other] { $folder } ਵਿਚਲੀਆਂ ਸਾਰੀਆਂ { $count } ਅਣਪੜ੍ਹੀਆਂ ਗੱਲਬਾਤਾਂ ਚੁਣੋ
        }
       *[message] { $count ->
            [one] { $folder } ਵਿਚਲਾ { $count } ਅਣਪੜ੍ਹਿਆ ਸੁਨੇਹਾ ਚੁਣੋ
           *[other] { $folder } ਵਿਚਲੇ ਸਾਰੇ { $count } ਅਣਪੜ੍ਹੇ ਸੁਨੇਹੇ ਚੁਣੋ
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $folder } ਵਿਚਲੀ { $count } ਤਾਰਾਬੱਧ ਗੱਲਬਾਤ ਚੁਣੋ
           *[other] { $folder } ਵਿਚਲੀਆਂ ਸਾਰੀਆਂ { $count } ਤਾਰਾਬੱਧ ਗੱਲਬਾਤਾਂ ਚੁਣੋ
        }
       *[message] { $count ->
            [one] { $folder } ਵਿਚਲਾ { $count } ਤਾਰਾਬੱਧ ਸੁਨੇਹਾ ਚੁਣੋ
           *[other] { $folder } ਵਿਚਲੇ ਸਾਰੇ { $count } ਤਾਰਾਬੱਧ ਸੁਨੇਹੇ ਚੁਣੋ
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $folder } ਵਿਚਲੀ { $count } ਤਾਰਾ-ਰਹਿਤ ਗੱਲਬਾਤ ਚੁਣੋ
           *[other] { $folder } ਵਿਚਲੀਆਂ ਸਾਰੀਆਂ { $count } ਤਾਰਾ-ਰਹਿਤ ਗੱਲਬਾਤਾਂ ਚੁਣੋ
        }
       *[message] { $count ->
            [one] { $folder } ਵਿਚਲਾ { $count } ਤਾਰਾ-ਰਹਿਤ ਸੁਨੇਹਾ ਚੁਣੋ
           *[other] { $folder } ਵਿਚਲੇ ਸਾਰੇ { $count } ਤਾਰਾ-ਰਹਿਤ ਸੁਨੇਹੇ ਚੁਣੋ
        }
    }
}
list-selected-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $count } ਪੜ੍ਹੀ ਗੱਲਬਾਤ ਚੁਣੀ ਗਈ ਹੈ।
           *[other] ਸਾਰੀਆਂ { $count } ਪੜ੍ਹੀਆਂ ਗੱਲਬਾਤਾਂ ਚੁਣੀਆਂ ਗਈਆਂ ਹਨ।
        }
       *[message] { $count ->
            [one] { $count } ਪੜ੍ਹਿਆ ਸੁਨੇਹਾ ਚੁਣਿਆ ਗਿਆ ਹੈ।
           *[other] ਸਾਰੇ { $count } ਪੜ੍ਹੇ ਸੁਨੇਹੇ ਚੁਣੇ ਗਏ ਹਨ।
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $count } ਅਣਪੜ੍ਹੀ ਗੱਲਬਾਤ ਚੁਣੀ ਗਈ ਹੈ।
           *[other] ਸਾਰੀਆਂ { $count } ਅਣਪੜ੍ਹੀਆਂ ਗੱਲਬਾਤਾਂ ਚੁਣੀਆਂ ਗਈਆਂ ਹਨ।
        }
       *[message] { $count ->
            [one] { $count } ਅਣਪੜ੍ਹਿਆ ਸੁਨੇਹਾ ਚੁਣਿਆ ਗਿਆ ਹੈ।
           *[other] ਸਾਰੇ { $count } ਅਣਪੜ੍ਹੇ ਸੁਨੇਹੇ ਚੁਣੇ ਗਏ ਹਨ।
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $count } ਤਾਰਾਬੱਧ ਗੱਲਬਾਤ ਚੁਣੀ ਗਈ ਹੈ।
           *[other] ਸਾਰੀਆਂ { $count } ਤਾਰਾਬੱਧ ਗੱਲਬਾਤਾਂ ਚੁਣੀਆਂ ਗਈਆਂ ਹਨ।
        }
       *[message] { $count ->
            [one] { $count } ਤਾਰਾਬੱਧ ਸੁਨੇਹਾ ਚੁਣਿਆ ਗਿਆ ਹੈ।
           *[other] ਸਾਰੇ { $count } ਤਾਰਾਬੱਧ ਸੁਨੇਹੇ ਚੁਣੇ ਗਏ ਹਨ।
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $count } ਤਾਰਾ-ਰਹਿਤ ਗੱਲਬਾਤ ਚੁਣੀ ਗਈ ਹੈ।
           *[other] ਸਾਰੀਆਂ { $count } ਤਾਰਾ-ਰਹਿਤ ਗੱਲਬਾਤਾਂ ਚੁਣੀਆਂ ਗਈਆਂ ਹਨ।
        }
       *[message] { $count ->
            [one] { $count } ਤਾਰਾ-ਰਹਿਤ ਸੁਨੇਹਾ ਚੁਣਿਆ ਗਿਆ ਹੈ।
           *[other] ਸਾਰੇ { $count } ਤਾਰਾ-ਰਹਿਤ ਸੁਨੇਹੇ ਚੁਣੇ ਗਏ ਹਨ।
        }
    }
}
list-selected-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $folder } ਵਿਚਲੀ { $count } ਪੜ੍ਹੀ ਗੱਲਬਾਤ ਚੁਣੀ ਗਈ ਹੈ।
           *[other] { $folder } ਵਿਚਲੀਆਂ ਸਾਰੀਆਂ { $count } ਪੜ੍ਹੀਆਂ ਗੱਲਬਾਤਾਂ ਚੁਣੀਆਂ ਗਈਆਂ ਹਨ।
        }
       *[message] { $count ->
            [one] { $folder } ਵਿਚਲਾ { $count } ਪੜ੍ਹਿਆ ਸੁਨੇਹਾ ਚੁਣਿਆ ਗਿਆ ਹੈ।
           *[other] { $folder } ਵਿਚਲੇ ਸਾਰੇ { $count } ਪੜ੍ਹੇ ਸੁਨੇਹੇ ਚੁਣੇ ਗਏ ਹਨ।
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $folder } ਵਿਚਲੀ { $count } ਅਣਪੜ੍ਹੀ ਗੱਲਬਾਤ ਚੁਣੀ ਗਈ ਹੈ।
           *[other] { $folder } ਵਿਚਲੀਆਂ ਸਾਰੀਆਂ { $count } ਅਣਪੜ੍ਹੀਆਂ ਗੱਲਬਾਤਾਂ ਚੁਣੀਆਂ ਗਈਆਂ ਹਨ।
        }
       *[message] { $count ->
            [one] { $folder } ਵਿਚਲਾ { $count } ਅਣਪੜ੍ਹਿਆ ਸੁਨੇਹਾ ਚੁਣਿਆ ਗਿਆ ਹੈ।
           *[other] { $folder } ਵਿਚਲੇ ਸਾਰੇ { $count } ਅਣਪੜ੍ਹੇ ਸੁਨੇਹੇ ਚੁਣੇ ਗਏ ਹਨ।
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $folder } ਵਿਚਲੀ { $count } ਤਾਰਾਬੱਧ ਗੱਲਬਾਤ ਚੁਣੀ ਗਈ ਹੈ।
           *[other] { $folder } ਵਿਚਲੀਆਂ ਸਾਰੀਆਂ { $count } ਤਾਰਾਬੱਧ ਗੱਲਬਾਤਾਂ ਚੁਣੀਆਂ ਗਈਆਂ ਹਨ।
        }
       *[message] { $count ->
            [one] { $folder } ਵਿਚਲਾ { $count } ਤਾਰਾਬੱਧ ਸੁਨੇਹਾ ਚੁਣਿਆ ਗਿਆ ਹੈ।
           *[other] { $folder } ਵਿਚਲੇ ਸਾਰੇ { $count } ਤਾਰਾਬੱਧ ਸੁਨੇਹੇ ਚੁਣੇ ਗਏ ਹਨ।
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $folder } ਵਿਚਲੀ { $count } ਤਾਰਾ-ਰਹਿਤ ਗੱਲਬਾਤ ਚੁਣੀ ਗਈ ਹੈ।
           *[other] { $folder } ਵਿਚਲੀਆਂ ਸਾਰੀਆਂ { $count } ਤਾਰਾ-ਰਹਿਤ ਗੱਲਬਾਤਾਂ ਚੁਣੀਆਂ ਗਈਆਂ ਹਨ।
        }
       *[message] { $count ->
            [one] { $folder } ਵਿਚਲਾ { $count } ਤਾਰਾ-ਰਹਿਤ ਸੁਨੇਹਾ ਚੁਣਿਆ ਗਿਆ ਹੈ।
           *[other] { $folder } ਵਿਚਲੇ ਸਾਰੇ { $count } ਤਾਰਾ-ਰਹਿਤ ਸੁਨੇਹੇ ਚੁਣੇ ਗਏ ਹਨ।
        }
    }
}
list-picked-none = { $pick ->
    [read] { $kind ->
        [conversation] ਇੱਥੇ ਕੋਈ ਪੜ੍ਹੀ ਗੱਲਬਾਤ ਨਹੀਂ ਹੈ।
       *[message] ਇੱਥੇ ਕੋਈ ਪੜ੍ਹਿਆ ਸੁਨੇਹਾ ਨਹੀਂ ਹੈ।
    }
   *[unread] { $kind ->
        [conversation] ਇੱਥੇ ਕੋਈ ਅਣਪੜ੍ਹੀ ਗੱਲਬਾਤ ਨਹੀਂ ਹੈ।
       *[message] ਇੱਥੇ ਕੋਈ ਅਣਪੜ੍ਹਿਆ ਸੁਨੇਹਾ ਨਹੀਂ ਹੈ।
    }
    [starred] { $kind ->
        [conversation] ਇੱਥੇ ਕੋਈ ਤਾਰਾਬੱਧ ਗੱਲਬਾਤ ਨਹੀਂ ਹੈ।
       *[message] ਇੱਥੇ ਕੋਈ ਤਾਰਾਬੱਧ ਸੁਨੇਹਾ ਨਹੀਂ ਹੈ।
    }
    [unstarred] { $kind ->
        [conversation] ਇੱਥੇ ਕੋਈ ਤਾਰਾ-ਰਹਿਤ ਗੱਲਬਾਤ ਨਹੀਂ ਹੈ।
       *[message] ਇੱਥੇ ਕੋਈ ਤਾਰਾ-ਰਹਿਤ ਸੁਨੇਹਾ ਨਹੀਂ ਹੈ।
    }
}
list-clear-selection = ਚੋਣ ਸਾਫ਼ ਕਰੋ

## Mail list: empty states

list-empty-search = ਤੁਹਾਡੀ ਖੋਜ ਨਾਲ ਕੋਈ ਸੁਨੇਹਾ ਮੇਲ ਨਹੀਂ ਖਾਂਦਾ।
list-empty-tab = { $tab } ਵਿੱਚ ਕੋਈ ਮੇਲ ਨਹੀਂ।
list-empty-tab-unknown = ਇਸ ਟੈਬ ਵਿੱਚ ਕੋਈ ਮੇਲ ਨਹੀਂ।
list-empty-folder = { $folder } ਵਿੱਚ ਕੋਈ ਸੁਨੇਹਾ ਨਹੀਂ।
list-empty-folder-unknown = ਇਸ ਫੋਲਡਰ ਵਿੱਚ ਕੋਈ ਸੁਨੇਹਾ ਨਹੀਂ।
list-first-sync = ਤੁਹਾਡੀ ਮੇਲ ਲਿਆਂਦੀ ਜਾ ਰਹੀ ਹੈ…
list-first-sync-detail = ਜਿਵੇਂ-ਜਿਵੇਂ ਇਹ ਆਵੇਗੀ, ਇੱਥੇ ਦਿਖਾਈ ਦੇਵੇਗੀ।

## Mail list: lines

row-removed = ਇਹ ਸੁਨੇਹਾ ਹਟਾ ਦਿੱਤਾ ਗਿਆ ਸੀ।
row-starred = ਤਾਰਾਬੱਧ
row-not-starred = ਤਾਰਾਬੱਧ ਨਹੀਂ
row-important = ਮਹੱਤਵਪੂਰਨ। ਗੈਰ-ਮਹੱਤਵਪੂਰਨ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕਰਨ ਲਈ ਕਲਿੱਕ ਕਰੋ।
row-mark-important = ਮਹੱਤਵਪੂਰਨ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕਰੋ
row-pinned = ਸਿਖਰ ’ਤੇ ਪਿੰਨ ਕੀਤਾ ਗਿਆ
row-task = ਕਾਰਜ
row-task-open = ਕਾਰਜ ਖੋਲ੍ਹੋ: { $title }
row-tracking-none = ਟ੍ਰੈਕ ਕੀਤਾ ਗਿਆ। ਅਜੇ ਖੋਲ੍ਹਿਆ ਨਹੀਂ ਗਿਆ
row-tracking-opened = { $recipients } ਵਿੱਚੋਂ { $opened } ਨੇ ਖੋਲ੍ਹਿਆ
row-tracking-clicked = { $recipients } ਵਿੱਚੋਂ { $opened } ਨੇ ਖੋਲ੍ਹਿਆ, { $clicked } ਨੇ ਲਿੰਕ ਖੋਲ੍ਹਿਆ
row-pin = ਸਿਖਰ ’ਤੇ ਪਿੰਨ ਕਰੋ
row-unpin = ਅਣਪਿੰਨ ਕਰੋ
row-snoozed-until = { $when } ਤੱਕ ਸਨੂਜ਼ ਕੀਤੀ ਗਈ

## Mail list: More menu and right-click menu

menu-reply = ਜਵਾਬ ਦਿਓ
menu-reply-all = ਸਭ ਨੂੰ ਜਵਾਬ ਦਿਓ
menu-forward = ਅੱਗੇ ਭੇਜੋ
menu-archive = ਪੁਰਾਲੇਖਬੱਧ ਕਰੋ
menu-delete = ਮਿਟਾਓ
menu-delete-forever = ਹਮੇਸ਼ਾ ਲਈ ਮਿਟਾਓ
menu-move-to-inbox = ਇਨਬਾਕਸ ਵਿੱਚ ਭੇਜੋ
menu-spam = ਸਪੈਮ ਦੀ ਰਿਪੋਰਟ ਕਰੋ
menu-not-spam = ਸਪੈਮ ਨਹੀਂ
menu-mark-read = ਪੜ੍ਹਿਆ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕਰੋ
menu-mark-unread = ਅਣਪੜ੍ਹਿਆ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕਰੋ
menu-mark-all-read = ਸਭ ਨੂੰ ਪੜ੍ਹਿਆ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕਰੋ
menu-star = ਤਾਰਾ ਲਗਾਓ
menu-unstar = ਤਾਰਾ ਹਟਾਓ
menu-important = ਮਹੱਤਵਪੂਰਨ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕਰੋ
menu-not-important = ਗੈਰ-ਮਹੱਤਵਪੂਰਨ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕਰੋ
menu-pin = ਸਿਖਰ ’ਤੇ ਪਿੰਨ ਕਰੋ
menu-unpin = ਅਣਪਿੰਨ ਕਰੋ
menu-snooze = ਸਨੂਜ਼ ਕਰੋ
menu-unsnooze = ਸਨੂਜ਼ ਹਟਾਓ
menu-add-to-tasks = ਕਾਰਜਾਂ ਵਿੱਚ ਜੋੜੋ
menu-schedule-meeting = ਮੀਟਿੰਗ ਤਹਿ ਕਰੋ
menu-add-note = ਨੋਟ ਜੋੜੋ
menu-print-all = ਸਭ ਪ੍ਰਿੰਟ ਕਰੋ
menu-new-window = ਨਵੀਂ ਵਿੰਡੋ ਵਿੱਚ ਖੋਲ੍ਹੋ
menu-move-to = ਇੱਥੇ ਭੇਜੋ
menu-move-to-heading = ਇੱਥੇ ਭੇਜੋ:
menu-find-from = { $name } ਵੱਲੋਂ ਈਮੇਲਾਂ ਲੱਭੋ

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] ਗੱਲਬਾਤ ਪੁਰਾਲੇਖਬੱਧ ਕੀਤੀ ਗਈ।
       *[other] { $count } ਗੱਲਬਾਤਾਂ ਪੁਰਾਲੇਖਬੱਧ ਕੀਤੀਆਂ ਗਈਆਂ।
    }
   *[message] { $count ->
        [one] ਸੁਨੇਹਾ ਪੁਰਾਲੇਖਬੱਧ ਕੀਤਾ ਗਿਆ।
       *[other] { $count } ਸੁਨੇਹੇ ਪੁਰਾਲੇਖਬੱਧ ਕੀਤੇ ਗਏ।
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] ਗੱਲਬਾਤ ਰੱਦੀ ਵਿੱਚ ਭੇਜੀ ਗਈ।
       *[other] { $count } ਗੱਲਬਾਤਾਂ ਰੱਦੀ ਵਿੱਚ ਭੇਜੀਆਂ ਗਈਆਂ।
    }
   *[message] { $count ->
        [one] ਸੁਨੇਹਾ ਰੱਦੀ ਵਿੱਚ ਭੇਜਿਆ ਗਿਆ।
       *[other] { $count } ਸੁਨੇਹੇ ਰੱਦੀ ਵਿੱਚ ਭੇਜੇ ਗਏ।
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] ਗੱਲਬਾਤ ਭੇਜੀ ਗਈ।
       *[other] { $count } ਗੱਲਬਾਤਾਂ ਭੇਜੀਆਂ ਗਈਆਂ।
    }
   *[message] { $count ->
        [one] ਸੁਨੇਹਾ ਭੇਜਿਆ ਗਿਆ।
       *[other] { $count } ਸੁਨੇਹੇ ਭੇਜੇ ਗਏ।
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] ਗੱਲਬਾਤ ਨੂੰ ਤਾਰਾ ਲਗਾਇਆ ਗਿਆ।
       *[other] { $count } ਗੱਲਬਾਤਾਂ ਨੂੰ ਤਾਰਾ ਲਗਾਇਆ ਗਿਆ।
    }
   *[message] { $count ->
        [one] ਸੁਨੇਹੇ ਨੂੰ ਤਾਰਾ ਲਗਾਇਆ ਗਿਆ।
       *[other] { $count } ਸੁਨੇਹਿਆਂ ਨੂੰ ਤਾਰਾ ਲਗਾਇਆ ਗਿਆ।
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] ਗੱਲਬਾਤ ਤੋਂ ਤਾਰਾ ਹਟਾਇਆ ਗਿਆ।
       *[other] { $count } ਗੱਲਬਾਤਾਂ ਤੋਂ ਤਾਰਾ ਹਟਾਇਆ ਗਿਆ।
    }
   *[message] { $count ->
        [one] ਸੁਨੇਹੇ ਤੋਂ ਤਾਰਾ ਹਟਾਇਆ ਗਿਆ।
       *[other] { $count } ਸੁਨੇਹਿਆਂ ਤੋਂ ਤਾਰਾ ਹਟਾਇਆ ਗਿਆ।
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] ਗੱਲਬਾਤ ਮਹੱਤਵਪੂਰਨ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕੀਤੀ ਗਈ।
       *[other] { $count } ਗੱਲਬਾਤਾਂ ਮਹੱਤਵਪੂਰਨ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕੀਤੀਆਂ ਗਈਆਂ।
    }
   *[message] { $count ->
        [one] ਸੁਨੇਹਾ ਮਹੱਤਵਪੂਰਨ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕੀਤਾ ਗਿਆ।
       *[other] { $count } ਸੁਨੇਹੇ ਮਹੱਤਵਪੂਰਨ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕੀਤੇ ਗਏ।
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] ਗੱਲਬਾਤ ਗੈਰ-ਮਹੱਤਵਪੂਰਨ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕੀਤੀ ਗਈ।
       *[other] { $count } ਗੱਲਬਾਤਾਂ ਗੈਰ-ਮਹੱਤਵਪੂਰਨ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕੀਤੀਆਂ ਗਈਆਂ।
    }
   *[message] { $count ->
        [one] ਸੁਨੇਹਾ ਗੈਰ-ਮਹੱਤਵਪੂਰਨ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕੀਤਾ ਗਿਆ।
       *[other] { $count } ਸੁਨੇਹੇ ਗੈਰ-ਮਹੱਤਵਪੂਰਨ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕੀਤੇ ਗਏ।
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] ਗੱਲਬਾਤ ਸਿਖਰ ’ਤੇ ਪਿੰਨ ਕੀਤੀ ਗਈ।
       *[other] { $count } ਗੱਲਬਾਤਾਂ ਸਿਖਰ ’ਤੇ ਪਿੰਨ ਕੀਤੀਆਂ ਗਈਆਂ।
    }
   *[message] { $count ->
        [one] ਸੁਨੇਹਾ ਸਿਖਰ ’ਤੇ ਪਿੰਨ ਕੀਤਾ ਗਿਆ।
       *[other] { $count } ਸੁਨੇਹੇ ਸਿਖਰ ’ਤੇ ਪਿੰਨ ਕੀਤੇ ਗਏ।
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] ਗੱਲਬਾਤ ਅਣਪਿੰਨ ਕੀਤੀ ਗਈ।
       *[other] { $count } ਗੱਲਬਾਤਾਂ ਅਣਪਿੰਨ ਕੀਤੀਆਂ ਗਈਆਂ।
    }
   *[message] { $count ->
        [one] ਸੁਨੇਹਾ ਅਣਪਿੰਨ ਕੀਤਾ ਗਿਆ।
       *[other] { $count } ਸੁਨੇਹੇ ਅਣਪਿੰਨ ਕੀਤੇ ਗਏ।
    }
}
toast-snoozed = { $kind ->
    [conversation] { $count ->
        [one] ਗੱਲਬਾਤ { $when } ਤੱਕ ਸਨੂਜ਼ ਕੀਤੀ ਗਈ।
       *[other] { $count } ਗੱਲਬਾਤਾਂ { $when } ਤੱਕ ਸਨੂਜ਼ ਕੀਤੀਆਂ ਗਈਆਂ।
    }
   *[message] { $count ->
        [one] ਸੁਨੇਹਾ { $when } ਤੱਕ ਸਨੂਜ਼ ਕੀਤਾ ਗਿਆ।
       *[other] { $count } ਸੁਨੇਹੇ { $when } ਤੱਕ ਸਨੂਜ਼ ਕੀਤੇ ਗਏ।
    }
}
toast-unsnoozed = { $kind ->
    [conversation] { $count ->
        [one] ਗੱਲਬਾਤ ਇਨਬਾਕਸ ਵਿੱਚ ਵਾਪਸ ਆ ਗਈ।
       *[other] { $count } ਗੱਲਬਾਤਾਂ ਇਨਬਾਕਸ ਵਿੱਚ ਵਾਪਸ ਆ ਗਈਆਂ।
    }
   *[message] { $count ->
        [one] ਸੁਨੇਹਾ ਇਨਬਾਕਸ ਵਿੱਚ ਵਾਪਸ ਆ ਗਿਆ।
       *[other] { $count } ਸੁਨੇਹੇ ਇਨਬਾਕਸ ਵਿੱਚ ਵਾਪਸ ਆ ਗਏ।
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] ਗੱਲਬਾਤ ਦੀ ਸਪੈਮ ਵਜੋਂ ਰਿਪੋਰਟ ਕੀਤੀ ਗਈ।
       *[other] { $count } ਗੱਲਬਾਤਾਂ ਦੀ ਸਪੈਮ ਵਜੋਂ ਰਿਪੋਰਟ ਕੀਤੀ ਗਈ।
    }
   *[message] { $count ->
        [one] ਸੁਨੇਹੇ ਦੀ ਸਪੈਮ ਵਜੋਂ ਰਿਪੋਰਟ ਕੀਤੀ ਗਈ।
       *[other] { $count } ਸੁਨੇਹਿਆਂ ਦੀ ਸਪੈਮ ਵਜੋਂ ਰਿਪੋਰਟ ਕੀਤੀ ਗਈ।
    }
}
toast-not-spam = { $kind ->
    [conversation] { $count ->
        [one] ਗੱਲਬਾਤ ਨੂੰ ਸਪੈਮ ਨਹੀਂ ਵਜੋਂ ਨਿਸ਼ਾਨਬੱਧ ਕਰਕੇ ਇਨਬਾਕਸ ਵਿੱਚ ਭੇਜਿਆ ਗਿਆ।
       *[other] { $count } ਗੱਲਬਾਤਾਂ ਨੂੰ ਸਪੈਮ ਨਹੀਂ ਵਜੋਂ ਨਿਸ਼ਾਨਬੱਧ ਕਰਕੇ ਇਨਬਾਕਸ ਵਿੱਚ ਭੇਜਿਆ ਗਿਆ।
    }
   *[message] { $count ->
        [one] ਸੁਨੇਹੇ ਨੂੰ ਸਪੈਮ ਨਹੀਂ ਵਜੋਂ ਨਿਸ਼ਾਨਬੱਧ ਕਰਕੇ ਇਨਬਾਕਸ ਵਿੱਚ ਭੇਜਿਆ ਗਿਆ।
       *[other] { $count } ਸੁਨੇਹਿਆਂ ਨੂੰ ਸਪੈਮ ਨਹੀਂ ਵਜੋਂ ਨਿਸ਼ਾਨਬੱਧ ਕਰਕੇ ਇਨਬਾਕਸ ਵਿੱਚ ਭੇਜਿਆ ਗਿਆ।
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] ਗੱਲਬਾਤ ਹਮੇਸ਼ਾ ਲਈ ਮਿਟਾਈ ਗਈ।
       *[other] { $count } ਗੱਲਬਾਤਾਂ ਹਮੇਸ਼ਾ ਲਈ ਮਿਟਾਈਆਂ ਗਈਆਂ।
    }
   *[message] { $count ->
        [one] ਸੁਨੇਹਾ ਹਮੇਸ਼ਾ ਲਈ ਮਿਟਾਇਆ ਗਿਆ।
       *[other] { $count } ਸੁਨੇਹੇ ਹਮੇਸ਼ਾ ਲਈ ਮਿਟਾਏ ਗਏ।
    }
}
toast-marked-read = { $kind ->
    [conversation] { $count ->
        [one] ਗੱਲਬਾਤ ਪੜ੍ਹੀ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕੀਤੀ ਗਈ।
       *[other] { $count } ਗੱਲਬਾਤਾਂ ਪੜ੍ਹੀਆਂ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕੀਤੀਆਂ ਗਈਆਂ।
    }
   *[message] { $count ->
        [one] ਸੁਨੇਹਾ ਪੜ੍ਹਿਆ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕੀਤਾ ਗਿਆ।
       *[other] { $count } ਸੁਨੇਹੇ ਪੜ੍ਹੇ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕੀਤੇ ਗਏ।
    }
}
toast-marked-unread = { $kind ->
    [conversation] { $count ->
        [one] ਗੱਲਬਾਤ ਅਣਪੜ੍ਹੀ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕੀਤੀ ਗਈ।
       *[other] { $count } ਗੱਲਬਾਤਾਂ ਅਣਪੜ੍ਹੀਆਂ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕੀਤੀਆਂ ਗਈਆਂ।
    }
   *[message] { $count ->
        [one] ਸੁਨੇਹਾ ਅਣਪੜ੍ਹਿਆ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕੀਤਾ ਗਿਆ।
       *[other] { $count } ਸੁਨੇਹੇ ਅਣਪੜ੍ਹੇ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕੀਤੇ ਗਏ।
    }
}
toast-undone = ਕਾਰਵਾਈ ਅਣਕੀਤੀ ਕੀਤੀ ਗਈ।
toast-nothing-to-undo = ਅਣਕੀਤਾ ਕਰਨ ਲਈ ਕੁਝ ਨਹੀਂ ਹੈ।
toast-cannot-undo-delete-forever = ਹਮੇਸ਼ਾ ਲਈ ਮਿਟਾਈ ਗਈ ਮੇਲ ਵਾਪਸ ਨਹੀਂ ਲਿਆਂਦੀ ਜਾ ਸਕਦੀ।
toast-send-undone = ਭੇਜਣਾ ਅਣਕੀਤਾ ਕੀਤਾ ਗਿਆ।
toast-too-late-to-undo-send = ਅਣਕੀਤਾ ਕਰਨ ਲਈ ਬਹੁਤ ਦੇਰ ਹੋ ਗਈ: ਸੁਨੇਹਾ ਪਹਿਲਾਂ ਹੀ ਭੇਜਿਆ ਜਾ ਚੁੱਕਾ ਹੈ।
toast-undo = ਅਣਕੀਤਾ ਕਰੋ
toast-close = ਬੰਦ ਕਰੋ
toast-no-spam-folder = ਇਸ ਖਾਤੇ ਵਿੱਚ ਕੋਈ ਸਪੈਮ ਫੋਲਡਰ ਨਹੀਂ ਹੈ।
