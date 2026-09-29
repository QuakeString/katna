# Katna Mail, Urdu (اردو).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = بنیادی
tab-promotions = پروموشنز
tab-social = سوشل
tab-updates = اپ ڈیٹس
tab-forums = فورمز
tab-focused = مرکوز
tab-other = دیگر
tab-inbox = ان باکس
tab-newsletters = نیوز لیٹرز
tab-notifications = اطلاعات
tab-new = { $count } نئے
tab-provider-other = Katna کی ترتیب

## Mail list: toolbar

list-select = منتخب کریں
list-refresh = ریفریش کریں
list-checking = نئی میل چیک کی جا رہی ہے…
list-more = مزید
list-mark-read = بطور پڑھا ہوا نشان زد کریں
list-mark-unread = بطور ناخواندہ نشان زد کریں
list-move-to = یہاں منتقل کریں
list-archive = آرکائیو کریں
list-spam = سپام کی اطلاع دیں
list-delete = حذف کریں
list-snooze = اسنوز کریں
list-unsnooze = اسنوز ختم کریں
list-newer = نئی
list-older = پرانی
list-range = { $total } میں سے { $first }–{ $last }
list-range-about = تقریباً { $total } میں سے { $first }–{ $last }
list-results = ”{ $query }“ کے نتائج
list-results-corrected = ”{ $query }“ کے نتائج دکھائے جا رہے ہیں
list-search-instead = اس کے بجائے ”{ $query }“ تلاش کریں
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = تمام
list-pick-none = کوئی نہیں
list-pick-read = پڑھی ہوئی
list-pick-unread = ناخواندہ
list-pick-starred = ستارے والی
list-pick-unstarred = بغیر ستارے والی

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] { $count } گفتگو منتخب ہے۔
       *[other] تمام { $count } گفتگوئیں منتخب ہیں۔
    }
   *[message] { $count ->
        [one] { $count } پیغام منتخب ہے۔
       *[other] تمام { $count } پیغامات منتخب ہیں۔
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder } میں { $count } گفتگو منتخب ہے۔
       *[other] { $folder } میں تمام { $count } گفتگوئیں منتخب ہیں۔
    }
   *[message] { $count ->
        [one] { $folder } میں { $count } پیغام منتخب ہے۔
       *[other] { $folder } میں تمام { $count } پیغامات منتخب ہیں۔
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] اسکرین پر { $count } گفتگو منتخب ہے۔
       *[other] اسکرین پر تمام { $count } گفتگوئیں منتخب ہیں۔
    }
   *[message] { $count ->
        [one] اسکرین پر { $count } پیغام منتخب ہے۔
       *[other] اسکرین پر تمام { $count } پیغامات منتخب ہیں۔
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] { $count } گفتگو منتخب کریں
       *[other] تمام { $count } گفتگوئیں منتخب کریں
    }
   *[message] { $count ->
        [one] { $count } پیغام منتخب کریں
       *[other] تمام { $count } پیغامات منتخب کریں
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder } میں { $count } گفتگو منتخب کریں
       *[other] { $folder } میں تمام { $count } گفتگوئیں منتخب کریں
    }
   *[message] { $count ->
        [one] { $folder } میں { $count } پیغام منتخب کریں
       *[other] { $folder } میں تمام { $count } پیغامات منتخب کریں
    }
}
list-selected-picked-screen = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] اسکرین پر { $count } پڑھی ہوئی گفتگو منتخب ہے۔
           *[other] اسکرین پر تمام { $count } پڑھی ہوئی گفتگوئیں منتخب ہیں۔
        }
       *[message] { $count ->
            [one] اسکرین پر { $count } پڑھا ہوا پیغام منتخب ہے۔
           *[other] اسکرین پر تمام { $count } پڑھے ہوئے پیغامات منتخب ہیں۔
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] اسکرین پر { $count } ناخواندہ گفتگو منتخب ہے۔
           *[other] اسکرین پر تمام { $count } ناخواندہ گفتگوئیں منتخب ہیں۔
        }
       *[message] { $count ->
            [one] اسکرین پر { $count } ناخواندہ پیغام منتخب ہے۔
           *[other] اسکرین پر تمام { $count } ناخواندہ پیغامات منتخب ہیں۔
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] اسکرین پر { $count } ستارے والی گفتگو منتخب ہے۔
           *[other] اسکرین پر تمام { $count } ستارے والی گفتگوئیں منتخب ہیں۔
        }
       *[message] { $count ->
            [one] اسکرین پر { $count } ستارے والا پیغام منتخب ہے۔
           *[other] اسکرین پر تمام { $count } ستارے والے پیغامات منتخب ہیں۔
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] اسکرین پر { $count } بغیر ستارے والی گفتگو منتخب ہے۔
           *[other] اسکرین پر تمام { $count } بغیر ستارے والی گفتگوئیں منتخب ہیں۔
        }
       *[message] { $count ->
            [one] اسکرین پر { $count } بغیر ستارے والا پیغام منتخب ہے۔
           *[other] اسکرین پر تمام { $count } بغیر ستارے والے پیغامات منتخب ہیں۔
        }
    }
}
list-select-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $count } پڑھی ہوئی گفتگو منتخب کریں
           *[other] تمام { $count } پڑھی ہوئی گفتگوئیں منتخب کریں
        }
       *[message] { $count ->
            [one] { $count } پڑھا ہوا پیغام منتخب کریں
           *[other] تمام { $count } پڑھے ہوئے پیغامات منتخب کریں
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $count } ناخواندہ گفتگو منتخب کریں
           *[other] تمام { $count } ناخواندہ گفتگوئیں منتخب کریں
        }
       *[message] { $count ->
            [one] { $count } ناخواندہ پیغام منتخب کریں
           *[other] تمام { $count } ناخواندہ پیغامات منتخب کریں
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $count } ستارے والی گفتگو منتخب کریں
           *[other] تمام { $count } ستارے والی گفتگوئیں منتخب کریں
        }
       *[message] { $count ->
            [one] { $count } ستارے والا پیغام منتخب کریں
           *[other] تمام { $count } ستارے والے پیغامات منتخب کریں
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $count } بغیر ستارے والی گفتگو منتخب کریں
           *[other] تمام { $count } بغیر ستارے والی گفتگوئیں منتخب کریں
        }
       *[message] { $count ->
            [one] { $count } بغیر ستارے والا پیغام منتخب کریں
           *[other] تمام { $count } بغیر ستارے والے پیغامات منتخب کریں
        }
    }
}
list-select-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $folder } میں { $count } پڑھی ہوئی گفتگو منتخب کریں
           *[other] { $folder } میں تمام { $count } پڑھی ہوئی گفتگوئیں منتخب کریں
        }
       *[message] { $count ->
            [one] { $folder } میں { $count } پڑھا ہوا پیغام منتخب کریں
           *[other] { $folder } میں تمام { $count } پڑھے ہوئے پیغامات منتخب کریں
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $folder } میں { $count } ناخواندہ گفتگو منتخب کریں
           *[other] { $folder } میں تمام { $count } ناخواندہ گفتگوئیں منتخب کریں
        }
       *[message] { $count ->
            [one] { $folder } میں { $count } ناخواندہ پیغام منتخب کریں
           *[other] { $folder } میں تمام { $count } ناخواندہ پیغامات منتخب کریں
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $folder } میں { $count } ستارے والی گفتگو منتخب کریں
           *[other] { $folder } میں تمام { $count } ستارے والی گفتگوئیں منتخب کریں
        }
       *[message] { $count ->
            [one] { $folder } میں { $count } ستارے والا پیغام منتخب کریں
           *[other] { $folder } میں تمام { $count } ستارے والے پیغامات منتخب کریں
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $folder } میں { $count } بغیر ستارے والی گفتگو منتخب کریں
           *[other] { $folder } میں تمام { $count } بغیر ستارے والی گفتگوئیں منتخب کریں
        }
       *[message] { $count ->
            [one] { $folder } میں { $count } بغیر ستارے والا پیغام منتخب کریں
           *[other] { $folder } میں تمام { $count } بغیر ستارے والے پیغامات منتخب کریں
        }
    }
}
list-selected-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $count } پڑھی ہوئی گفتگو منتخب ہے۔
           *[other] تمام { $count } پڑھی ہوئی گفتگوئیں منتخب ہیں۔
        }
       *[message] { $count ->
            [one] { $count } پڑھا ہوا پیغام منتخب ہے۔
           *[other] تمام { $count } پڑھے ہوئے پیغامات منتخب ہیں۔
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $count } ناخواندہ گفتگو منتخب ہے۔
           *[other] تمام { $count } ناخواندہ گفتگوئیں منتخب ہیں۔
        }
       *[message] { $count ->
            [one] { $count } ناخواندہ پیغام منتخب ہے۔
           *[other] تمام { $count } ناخواندہ پیغامات منتخب ہیں۔
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $count } ستارے والی گفتگو منتخب ہے۔
           *[other] تمام { $count } ستارے والی گفتگوئیں منتخب ہیں۔
        }
       *[message] { $count ->
            [one] { $count } ستارے والا پیغام منتخب ہے۔
           *[other] تمام { $count } ستارے والے پیغامات منتخب ہیں۔
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $count } بغیر ستارے والی گفتگو منتخب ہے۔
           *[other] تمام { $count } بغیر ستارے والی گفتگوئیں منتخب ہیں۔
        }
       *[message] { $count ->
            [one] { $count } بغیر ستارے والا پیغام منتخب ہے۔
           *[other] تمام { $count } بغیر ستارے والے پیغامات منتخب ہیں۔
        }
    }
}
list-selected-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $folder } میں { $count } پڑھی ہوئی گفتگو منتخب ہے۔
           *[other] { $folder } میں تمام { $count } پڑھی ہوئی گفتگوئیں منتخب ہیں۔
        }
       *[message] { $count ->
            [one] { $folder } میں { $count } پڑھا ہوا پیغام منتخب ہے۔
           *[other] { $folder } میں تمام { $count } پڑھے ہوئے پیغامات منتخب ہیں۔
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $folder } میں { $count } ناخواندہ گفتگو منتخب ہے۔
           *[other] { $folder } میں تمام { $count } ناخواندہ گفتگوئیں منتخب ہیں۔
        }
       *[message] { $count ->
            [one] { $folder } میں { $count } ناخواندہ پیغام منتخب ہے۔
           *[other] { $folder } میں تمام { $count } ناخواندہ پیغامات منتخب ہیں۔
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $folder } میں { $count } ستارے والی گفتگو منتخب ہے۔
           *[other] { $folder } میں تمام { $count } ستارے والی گفتگوئیں منتخب ہیں۔
        }
       *[message] { $count ->
            [one] { $folder } میں { $count } ستارے والا پیغام منتخب ہے۔
           *[other] { $folder } میں تمام { $count } ستارے والے پیغامات منتخب ہیں۔
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $folder } میں { $count } بغیر ستارے والی گفتگو منتخب ہے۔
           *[other] { $folder } میں تمام { $count } بغیر ستارے والی گفتگوئیں منتخب ہیں۔
        }
       *[message] { $count ->
            [one] { $folder } میں { $count } بغیر ستارے والا پیغام منتخب ہے۔
           *[other] { $folder } میں تمام { $count } بغیر ستارے والے پیغامات منتخب ہیں۔
        }
    }
}
list-picked-none = { $pick ->
    [read] { $kind ->
        [conversation] یہاں کوئی پڑھی ہوئی گفتگو نہیں۔
       *[message] یہاں کوئی پڑھا ہوا پیغام نہیں۔
    }
   *[unread] { $kind ->
        [conversation] یہاں کوئی ناخواندہ گفتگو نہیں۔
       *[message] یہاں کوئی ناخواندہ پیغام نہیں۔
    }
    [starred] { $kind ->
        [conversation] یہاں کوئی ستارے والی گفتگو نہیں۔
       *[message] یہاں کوئی ستارے والا پیغام نہیں۔
    }
    [unstarred] { $kind ->
        [conversation] یہاں کوئی بغیر ستارے والی گفتگو نہیں۔
       *[message] یہاں کوئی بغیر ستارے والا پیغام نہیں۔
    }
}
list-clear-selection = انتخاب صاف کریں

## Mail list: empty states

list-empty-search = کوئی پیغام آپ کی تلاش سے مماثل نہیں۔
list-empty-tab = { $tab } میں کوئی میل نہیں۔
list-empty-tab-unknown = اس ٹیب میں کوئی میل نہیں۔
list-empty-folder = { $folder } میں کوئی پیغام نہیں۔
list-empty-folder-unknown = اس فولڈر میں کوئی پیغام نہیں۔
list-first-sync = آپ کی میل حاصل کی جا رہی ہے…
list-first-sync-detail = جیسے جیسے یہ آئے گی، یہاں نظر آئے گی۔

## Mail list: lines

row-removed = یہ پیغام ہٹا دیا گیا۔
row-starred = ستارے والا
row-not-starred = ستارہ نہیں لگا
row-important = اہم۔ غیر اہم کے بطور نشان زد کرنے کے لیے کلک کریں۔
row-mark-important = بطور اہم نشان زد کریں
row-pinned = سب سے اوپر پن کیا گیا
row-tracking-none = ٹریک ہو رہا ہے۔ ابھی کھولا نہیں گیا
row-tracking-opened = { $recipients } میں سے { $opened } نے کھولا
row-tracking-clicked = { $recipients } میں سے { $opened } نے کھولا، { $clicked } نے لنک کھولا
row-pin = سب سے اوپر پن کریں
row-unpin = پن ہٹائیں
row-snoozed-until = { $when } تک اسنوز شدہ

## Mail list: More menu and right-click menu

menu-reply = جواب دیں
menu-reply-all = سب کو جواب دیں
menu-forward = آگے بھیجیں
menu-archive = آرکائیو کریں
menu-delete = حذف کریں
menu-delete-forever = ہمیشہ کے لیے حذف کریں
menu-move-to-inbox = ان باکس میں منتقل کریں
menu-spam = سپام کی اطلاع دیں
menu-not-spam = سپام نہیں ہے
menu-mark-read = بطور پڑھا ہوا نشان زد کریں
menu-mark-unread = بطور ناخواندہ نشان زد کریں
menu-mark-all-read = سب کو بطور پڑھا ہوا نشان زد کریں
menu-star = ستارہ لگائیں
menu-unstar = ستارہ ہٹائیں
menu-important = بطور اہم نشان زد کریں
menu-not-important = بطور غیر اہم نشان زد کریں
menu-pin = سب سے اوپر پن کریں
menu-unpin = پن ہٹائیں
menu-snooze = اسنوز کریں
menu-unsnooze = اسنوز ختم کریں
menu-add-to-tasks = کاموں میں شامل کریں
menu-add-note = نوٹ شامل کریں
menu-print-all = سب پرنٹ کریں
menu-new-window = نئی ونڈو میں کھولیں
menu-move-to = یہاں منتقل کریں
menu-move-to-heading = یہاں منتقل کریں:
menu-find-from = { $name } کی ای میلز تلاش کریں

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] گفتگو آرکائیو کر دی گئی۔
       *[other] { $count } گفتگوئیں آرکائیو کر دی گئیں۔
    }
   *[message] { $count ->
        [one] پیغام آرکائیو کر دیا گیا۔
       *[other] { $count } پیغامات آرکائیو کر دیے گئے۔
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] گفتگو کوڑے دان میں منتقل کر دی گئی۔
       *[other] { $count } گفتگوئیں کوڑے دان میں منتقل کر دی گئیں۔
    }
   *[message] { $count ->
        [one] پیغام کوڑے دان میں منتقل کر دیا گیا۔
       *[other] { $count } پیغامات کوڑے دان میں منتقل کر دیے گئے۔
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] گفتگو منتقل کر دی گئی۔
       *[other] { $count } گفتگوئیں منتقل کر دی گئیں۔
    }
   *[message] { $count ->
        [one] پیغام منتقل کر دیا گیا۔
       *[other] { $count } پیغامات منتقل کر دیے گئے۔
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] گفتگو پر ستارہ لگا دیا گیا۔
       *[other] { $count } گفتگوئیں پر ستارہ لگا دیا گیا۔
    }
   *[message] { $count ->
        [one] پیغام پر ستارہ لگا دیا گیا۔
       *[other] { $count } پیغامات پر ستارہ لگا دیا گیا۔
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] گفتگو سے ستارہ ہٹا دیا گیا۔
       *[other] { $count } گفتگوئیں سے ستارہ ہٹا دیا گیا۔
    }
   *[message] { $count ->
        [one] پیغام سے ستارہ ہٹا دیا گیا۔
       *[other] { $count } پیغامات سے ستارہ ہٹا دیا گیا۔
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] گفتگو بطور اہم نشان زد کر دی گئی۔
       *[other] { $count } گفتگوئیں بطور اہم نشان زد کر دی گئیں۔
    }
   *[message] { $count ->
        [one] پیغام بطور اہم نشان زد کر دیا گیا۔
       *[other] { $count } پیغامات بطور اہم نشان زد کر دیے گئے۔
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] گفتگو بطور غیر اہم نشان زد کر دی گئی۔
       *[other] { $count } گفتگوئیں بطور غیر اہم نشان زد کر دی گئیں۔
    }
   *[message] { $count ->
        [one] پیغام بطور غیر اہم نشان زد کر دیا گیا۔
       *[other] { $count } پیغامات بطور غیر اہم نشان زد کر دیے گئے۔
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] گفتگو سب سے اوپر پن کر دی گئی۔
       *[other] { $count } گفتگوئیں سب سے اوپر پن کر دی گئیں۔
    }
   *[message] { $count ->
        [one] پیغام سب سے اوپر پن کر دیا گیا۔
       *[other] { $count } پیغامات سب سے اوپر پن کر دیے گئے۔
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] گفتگو سے پن ہٹا دیا گیا۔
       *[other] { $count } گفتگوئیں سے پن ہٹا دیا گیا۔
    }
   *[message] { $count ->
        [one] پیغام سے پن ہٹا دیا گیا۔
       *[other] { $count } پیغامات سے پن ہٹا دیا گیا۔
    }
}
toast-snoozed = { $kind ->
    [conversation] { $count ->
        [one] گفتگو { $when } تک اسنوز کر دی گئی۔
       *[other] { $count } گفتگوئیں { $when } تک اسنوز کر دی گئیں۔
    }
   *[message] { $count ->
        [one] پیغام { $when } تک اسنوز کر دیا گیا۔
       *[other] { $count } پیغامات { $when } تک اسنوز کر دیے گئے۔
    }
}
toast-unsnoozed = { $kind ->
    [conversation] { $count ->
        [one] گفتگو ان باکس میں واپس آ گئی۔
       *[other] { $count } گفتگوئیں ان باکس میں واپس آ گئیں۔
    }
   *[message] { $count ->
        [one] پیغام ان باکس میں واپس آ گیا۔
       *[other] { $count } پیغامات ان باکس میں واپس آ گئے۔
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] گفتگو کی بطور سپام اطلاع دے دی گئی۔
       *[other] { $count } گفتگوئیں کی بطور سپام اطلاع دے دی گئی۔
    }
   *[message] { $count ->
        [one] پیغام کی بطور سپام اطلاع دے دی گئی۔
       *[other] { $count } پیغامات کی بطور سپام اطلاع دے دی گئی۔
    }
}
toast-not-spam = { $kind ->
    [conversation] { $count ->
        [one] گفتگو کو سپام نہیں کے بطور نشان زد کر کے ان باکس میں منتقل کر دیا گیا۔
       *[other] { $count } گفتگوئیں سپام نہیں کے بطور نشان زد کر کے ان باکس میں منتقل کر دی گئیں۔
    }
   *[message] { $count ->
        [one] پیغام کو سپام نہیں کے بطور نشان زد کر کے ان باکس میں منتقل کر دیا گیا۔
       *[other] { $count } پیغامات کو سپام نہیں کے بطور نشان زد کر کے ان باکس میں منتقل کر دیا گیا۔
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] گفتگو ہمیشہ کے لیے حذف کر دی گئی۔
       *[other] { $count } گفتگوئیں ہمیشہ کے لیے حذف کر دی گئیں۔
    }
   *[message] { $count ->
        [one] پیغام ہمیشہ کے لیے حذف کر دیا گیا۔
       *[other] { $count } پیغامات ہمیشہ کے لیے حذف کر دیے گئے۔
    }
}
toast-marked-read = { $kind ->
    [conversation] { $count ->
        [one] گفتگو بطور پڑھی ہوئی نشان زد کر دی گئی۔
       *[other] { $count } گفتگوئیں بطور پڑھی ہوئی نشان زد کر دی گئیں۔
    }
   *[message] { $count ->
        [one] پیغام بطور پڑھا ہوا نشان زد کر دیا گیا۔
       *[other] { $count } پیغامات بطور پڑھے ہوئے نشان زد کر دیے گئے۔
    }
}
toast-marked-unread = { $kind ->
    [conversation] { $count ->
        [one] گفتگو بطور ناخواندہ نشان زد کر دی گئی۔
       *[other] { $count } گفتگوئیں بطور ناخواندہ نشان زد کر دی گئیں۔
    }
   *[message] { $count ->
        [one] پیغام بطور ناخواندہ نشان زد کر دیا گیا۔
       *[other] { $count } پیغامات بطور ناخواندہ نشان زد کر دیے گئے۔
    }
}
toast-undone = کارروائی کالعدم کر دی گئی۔
toast-nothing-to-undo = کالعدم کرنے کو کچھ نہیں۔
toast-cannot-undo-delete-forever = ہمیشہ کے لیے حذف کی گئی میل واپس نہیں لائی جا سکتی۔
toast-send-undone = بھیجنا کالعدم کر دیا گیا۔
toast-too-late-to-undo-send = کالعدم کرنے میں بہت دیر ہو گئی: پیغام پہلے ہی بھیجا جا چکا ہے۔
toast-undo = کالعدم کریں
toast-close = بند کریں
toast-no-spam-folder = اس اکاؤنٹ میں کوئی سپام فولڈر نہیں ہے۔
