# Katna Mail, Khmer (ខ្មែរ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = ចម្បង
tab-promotions = ការផ្សព្វផ្សាយ
tab-social = សង្គម
tab-updates = បច្ចុប្បន្នភាព
tab-forums = វេទិកា
tab-focused = បានផ្ដោត
tab-other = ផ្សេងៗ
tab-inbox = ប្រអប់ទទួល
tab-newsletters = ព្រឹត្តិបត្រ
tab-notifications = ការជូនដំណឹង
tab-new = ថ្មី { $count }
tab-provider-other = តម្រៀបដោយ Katna

## Mail list: toolbar

list-select = ជ្រើសរើស
list-refresh = ផ្ទុកឡើងវិញ
list-checking = កំពុងពិនិត្យរកសំបុត្រថ្មី…
list-more = ច្រើនទៀត
list-mark-read = សម្គាល់ថាបានអាន
list-mark-unread = សម្គាល់ថាមិនទាន់អាន
list-move-to = ផ្លាស់ទីទៅ
list-archive = ទុកក្នុងបណ្ណសារ
list-spam = រាយការណ៍ថាជាសារឥតបានការ
list-delete = លុប
list-snooze = ពន្យារពេល
list-unsnooze = ឈប់ពន្យារពេល
list-newer = ថ្មីជាង
list-older = ចាស់ជាង
list-range = { $first }–{ $last } នៃ { $total }
list-range-about = { $first }–{ $last } នៃប្រហែល { $total }
list-results = លទ្ធផលសម្រាប់ “{ $query }”
list-results-corrected = កំពុងបង្ហាញលទ្ធផលសម្រាប់ “{ $query }”
list-search-instead = ស្វែងរក “{ $query }” ជំនួសវិញ
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = ទាំងអស់
list-pick-none = គ្មាន
list-pick-read = បានអាន
list-pick-unread = មិនទាន់អាន
list-pick-starred = មានផ្កាយ
list-pick-unstarred = គ្មានផ្កាយ

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] ការសន្ទនាទាំង { $count } ត្រូវបានជ្រើសរើស។
   *[message] សារទាំង { $count } ត្រូវបានជ្រើសរើស។
}
list-selected-all-in = { $kind ->
    [conversation] ការសន្ទនាទាំង { $count } ក្នុង { $folder } ត្រូវបានជ្រើសរើស។
   *[message] សារទាំង { $count } ក្នុង { $folder } ត្រូវបានជ្រើសរើស។
}
list-selected-screen = { $kind ->
    [conversation] ការសន្ទនាទាំង { $count } នៅលើអេក្រង់ត្រូវបានជ្រើសរើស។
   *[message] សារទាំង { $count } នៅលើអេក្រង់ត្រូវបានជ្រើសរើស។
}
list-select-all = { $kind ->
    [conversation] ជ្រើសរើសការសន្ទនាទាំង { $count }
   *[message] ជ្រើសរើសសារទាំង { $count }
}
list-select-all-in = { $kind ->
    [conversation] ជ្រើសរើសការសន្ទនាទាំង { $count } ក្នុង { $folder }
   *[message] ជ្រើសរើសសារទាំង { $count } ក្នុង { $folder }
}
list-selected-picked-screen = { $pick ->
    [read] { $kind ->
        [conversation] ការសន្ទនាដែលបានអានទាំង { $count } នៅលើអេក្រង់ត្រូវបានជ្រើសរើស។
       *[message] សារដែលបានអានទាំង { $count } នៅលើអេក្រង់ត្រូវបានជ្រើសរើស។
    }
   *[unread] { $kind ->
        [conversation] ការសន្ទនាដែលមិនទាន់អានទាំង { $count } នៅលើអេក្រង់ត្រូវបានជ្រើសរើស។
       *[message] សារដែលមិនទាន់អានទាំង { $count } នៅលើអេក្រង់ត្រូវបានជ្រើសរើស។
    }
    [starred] { $kind ->
        [conversation] ការសន្ទនាដែលមានផ្កាយទាំង { $count } នៅលើអេក្រង់ត្រូវបានជ្រើសរើស។
       *[message] សារដែលមានផ្កាយទាំង { $count } នៅលើអេក្រង់ត្រូវបានជ្រើសរើស។
    }
    [unstarred] { $kind ->
        [conversation] ការសន្ទនាដែលគ្មានផ្កាយទាំង { $count } នៅលើអេក្រង់ត្រូវបានជ្រើសរើស។
       *[message] សារដែលគ្មានផ្កាយទាំង { $count } នៅលើអេក្រង់ត្រូវបានជ្រើសរើស។
    }
}
list-select-picked = { $pick ->
    [read] { $kind ->
        [conversation] ជ្រើសរើសការសន្ទនាដែលបានអានទាំង { $count }
       *[message] ជ្រើសរើសសារដែលបានអានទាំង { $count }
    }
   *[unread] { $kind ->
        [conversation] ជ្រើសរើសការសន្ទនាដែលមិនទាន់អានទាំង { $count }
       *[message] ជ្រើសរើសសារដែលមិនទាន់អានទាំង { $count }
    }
    [starred] { $kind ->
        [conversation] ជ្រើសរើសការសន្ទនាដែលមានផ្កាយទាំង { $count }
       *[message] ជ្រើសរើសសារដែលមានផ្កាយទាំង { $count }
    }
    [unstarred] { $kind ->
        [conversation] ជ្រើសរើសការសន្ទនាដែលគ្មានផ្កាយទាំង { $count }
       *[message] ជ្រើសរើសសារដែលគ្មានផ្កាយទាំង { $count }
    }
}
list-select-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] ជ្រើសរើសការសន្ទនាដែលបានអានទាំង { $count } ក្នុង { $folder }
       *[message] ជ្រើសរើសសារដែលបានអានទាំង { $count } ក្នុង { $folder }
    }
   *[unread] { $kind ->
        [conversation] ជ្រើសរើសការសន្ទនាដែលមិនទាន់អានទាំង { $count } ក្នុង { $folder }
       *[message] ជ្រើសរើសសារដែលមិនទាន់អានទាំង { $count } ក្នុង { $folder }
    }
    [starred] { $kind ->
        [conversation] ជ្រើសរើសការសន្ទនាដែលមានផ្កាយទាំង { $count } ក្នុង { $folder }
       *[message] ជ្រើសរើសសារដែលមានផ្កាយទាំង { $count } ក្នុង { $folder }
    }
    [unstarred] { $kind ->
        [conversation] ជ្រើសរើសការសន្ទនាដែលគ្មានផ្កាយទាំង { $count } ក្នុង { $folder }
       *[message] ជ្រើសរើសសារដែលគ្មានផ្កាយទាំង { $count } ក្នុង { $folder }
    }
}
list-selected-picked = { $pick ->
    [read] { $kind ->
        [conversation] ការសន្ទនាដែលបានអានទាំង { $count } ត្រូវបានជ្រើសរើស។
       *[message] សារដែលបានអានទាំង { $count } ត្រូវបានជ្រើសរើស។
    }
   *[unread] { $kind ->
        [conversation] ការសន្ទនាដែលមិនទាន់អានទាំង { $count } ត្រូវបានជ្រើសរើស។
       *[message] សារដែលមិនទាន់អានទាំង { $count } ត្រូវបានជ្រើសរើស។
    }
    [starred] { $kind ->
        [conversation] ការសន្ទនាដែលមានផ្កាយទាំង { $count } ត្រូវបានជ្រើសរើស។
       *[message] សារដែលមានផ្កាយទាំង { $count } ត្រូវបានជ្រើសរើស។
    }
    [unstarred] { $kind ->
        [conversation] ការសន្ទនាដែលគ្មានផ្កាយទាំង { $count } ត្រូវបានជ្រើសរើស។
       *[message] សារដែលគ្មានផ្កាយទាំង { $count } ត្រូវបានជ្រើសរើស។
    }
}
list-selected-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] ការសន្ទនាដែលបានអានទាំង { $count } ក្នុង { $folder } ត្រូវបានជ្រើសរើស។
       *[message] សារដែលបានអានទាំង { $count } ក្នុង { $folder } ត្រូវបានជ្រើសរើស។
    }
   *[unread] { $kind ->
        [conversation] ការសន្ទនាដែលមិនទាន់អានទាំង { $count } ក្នុង { $folder } ត្រូវបានជ្រើសរើស។
       *[message] សារដែលមិនទាន់អានទាំង { $count } ក្នុង { $folder } ត្រូវបានជ្រើសរើស។
    }
    [starred] { $kind ->
        [conversation] ការសន្ទនាដែលមានផ្កាយទាំង { $count } ក្នុង { $folder } ត្រូវបានជ្រើសរើស។
       *[message] សារដែលមានផ្កាយទាំង { $count } ក្នុង { $folder } ត្រូវបានជ្រើសរើស។
    }
    [unstarred] { $kind ->
        [conversation] ការសន្ទនាដែលគ្មានផ្កាយទាំង { $count } ក្នុង { $folder } ត្រូវបានជ្រើសរើស។
       *[message] សារដែលគ្មានផ្កាយទាំង { $count } ក្នុង { $folder } ត្រូវបានជ្រើសរើស។
    }
}
list-picked-none = { $pick ->
    [read] { $kind ->
        [conversation] គ្មានការសន្ទនាដែលបានអាននៅទីនេះទេ។
       *[message] គ្មានសារដែលបានអាននៅទីនេះទេ។
    }
   *[unread] { $kind ->
        [conversation] គ្មានការសន្ទនាដែលមិនទាន់អាននៅទីនេះទេ។
       *[message] គ្មានសារដែលមិនទាន់អាននៅទីនេះទេ។
    }
    [starred] { $kind ->
        [conversation] គ្មានការសន្ទនាដែលមានផ្កាយនៅទីនេះទេ។
       *[message] គ្មានសារដែលមានផ្កាយនៅទីនេះទេ។
    }
    [unstarred] { $kind ->
        [conversation] គ្មានការសន្ទនាដែលគ្មានផ្កាយនៅទីនេះទេ។
       *[message] គ្មានសារដែលគ្មានផ្កាយនៅទីនេះទេ។
    }
}
list-clear-selection = សម្អាតការជ្រើសរើស

## Mail list: empty states

list-empty-search = គ្មានសារដែលត្រូវនឹងការស្វែងរករបស់អ្នកទេ។
list-empty-tab = គ្មានសំបុត្រក្នុង { $tab } ទេ។
list-empty-tab-unknown = គ្មានសំបុត្រក្នុងផ្ទាំងនេះទេ។
list-empty-folder = គ្មានសារក្នុង { $folder } ទេ។
list-empty-folder-unknown = គ្មានសារក្នុងថតនេះទេ។
list-first-sync = កំពុងទទួលសំបុត្ររបស់អ្នក…
list-first-sync-detail = សំបុត្រនឹងបង្ហាញនៅទីនេះ នៅពេលវាមកដល់។

## Mail list: lines

row-removed = សារនេះត្រូវបានដកចេញ។
row-starred = មានផ្កាយ
row-not-starred = គ្មានផ្កាយ
row-important = សំខាន់។ ចុចដើម្បីសម្គាល់ថាមិនសំខាន់។
row-mark-important = សម្គាល់ថាសំខាន់
row-pinned = បានខ្ទាស់នៅខាងលើ
row-tracking-none = បានតាមដាន។ មិនទាន់បានបើកនៅឡើយ
row-tracking-opened = បានបើកដោយ { $opened } នាក់ ក្នុងចំណោម { $recipients }
row-tracking-clicked = បានបើកដោយ { $opened } នាក់ ក្នុងចំណោម { $recipients } ហើយ { $clicked } នាក់បានចុចតំណ
row-pin = ខ្ទាស់នៅខាងលើ
row-unpin = ឈប់ខ្ទាស់
row-snoozed-until = បានពន្យារពេលរហូតដល់ { $when }

## Mail list: More menu and right-click menu

menu-reply = ឆ្លើយតប
menu-reply-all = ឆ្លើយតបទាំងអស់
menu-forward = បញ្ជូនបន្ត
menu-archive = ទុកក្នុងបណ្ណសារ
menu-delete = លុប
menu-delete-forever = លុបជារៀងរហូត
menu-move-to-inbox = ផ្លាស់ទីទៅប្រអប់ទទួល
menu-spam = រាយការណ៍ថាជាសារឥតបានការ
menu-not-spam = មិនមែនសារឥតបានការ
menu-mark-read = សម្គាល់ថាបានអាន
menu-mark-unread = សម្គាល់ថាមិនទាន់អាន
menu-mark-all-read = សម្គាល់ទាំងអស់ថាបានអាន
menu-star = ដាក់ផ្កាយ
menu-unstar = ដកផ្កាយចេញ
menu-important = សម្គាល់ថាសំខាន់
menu-not-important = សម្គាល់ថាមិនសំខាន់
menu-pin = ខ្ទាស់នៅខាងលើ
menu-unpin = ឈប់ខ្ទាស់
menu-snooze = ពន្យារពេល
menu-unsnooze = ឈប់ពន្យារពេល
menu-add-to-tasks = បន្ថែមទៅកិច្ចការ
menu-schedule-meeting = កំណត់ពេលប្រជុំ
menu-add-note = បន្ថែមកំណត់ចំណាំ
menu-print-all = បោះពុម្ពទាំងអស់
menu-new-window = បើកក្នុងបង្អួចថ្មី
menu-move-to = ផ្លាស់ទីទៅ
menu-move-to-heading = ផ្លាស់ទីទៅ៖
menu-find-from = ស្វែងរកសំបុត្រពី { $name }

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] បានទុកការសន្ទនា { $count } ក្នុងបណ្ណសារ។
   *[message] បានទុកសារ { $count } ក្នុងបណ្ណសារ។
}
toast-trashed = { $kind ->
    [conversation] បានផ្លាស់ទីការសន្ទនា { $count } ទៅធុងសំរាម។
   *[message] បានផ្លាស់ទីសារ { $count } ទៅធុងសំរាម។
}
toast-moved = { $kind ->
    [conversation] បានផ្លាស់ទីការសន្ទនា { $count }។
   *[message] បានផ្លាស់ទីសារ { $count }។
}
toast-starred = { $kind ->
    [conversation] បានដាក់ផ្កាយលើការសន្ទនា { $count }។
   *[message] បានដាក់ផ្កាយលើសារ { $count }។
}
toast-unstarred = { $kind ->
    [conversation] បានដកផ្កាយចេញពីការសន្ទនា { $count }។
   *[message] បានដកផ្កាយចេញពីសារ { $count }។
}
toast-important = { $kind ->
    [conversation] បានសម្គាល់ការសន្ទនា { $count } ថាសំខាន់។
   *[message] បានសម្គាល់សារ { $count } ថាសំខាន់។
}
toast-not-important = { $kind ->
    [conversation] បានសម្គាល់ការសន្ទនា { $count } ថាមិនសំខាន់។
   *[message] បានសម្គាល់សារ { $count } ថាមិនសំខាន់។
}
toast-pinned = { $kind ->
    [conversation] បានខ្ទាស់ការសន្ទនា { $count } នៅខាងលើ។
   *[message] បានខ្ទាស់សារ { $count } នៅខាងលើ។
}
toast-unpinned = { $kind ->
    [conversation] បានឈប់ខ្ទាស់ការសន្ទនា { $count }។
   *[message] បានឈប់ខ្ទាស់សារ { $count }។
}
toast-snoozed = { $kind ->
    [conversation] បានពន្យារពេលការសន្ទនា { $count } រហូតដល់ { $when }។
   *[message] បានពន្យារពេលសារ { $count } រហូតដល់ { $when }។
}
toast-unsnoozed = { $kind ->
    [conversation] ការសន្ទនា { $count } បានត្រឡប់មកប្រអប់ទទួលវិញ។
   *[message] សារ { $count } បានត្រឡប់មកប្រអប់ទទួលវិញ។
}
toast-spam = { $kind ->
    [conversation] បានរាយការណ៍ការសន្ទនា { $count } ថាជាសារឥតបានការ។
   *[message] បានរាយការណ៍សារ { $count } ថាជាសារឥតបានការ។
}
toast-not-spam = { $kind ->
    [conversation] បានសម្គាល់ការសន្ទនា { $count } ថាមិនមែនសារឥតបានការ ហើយផ្លាស់ទីទៅប្រអប់ទទួល។
   *[message] បានសម្គាល់សារ { $count } ថាមិនមែនសារឥតបានការ ហើយផ្លាស់ទីទៅប្រអប់ទទួល។
}
toast-deleted-forever = { $kind ->
    [conversation] បានលុបការសន្ទនា { $count } ជារៀងរហូត។
   *[message] បានលុបសារ { $count } ជារៀងរហូត។
}
toast-marked-read = { $kind ->
    [conversation] បានសម្គាល់ការសន្ទនា { $count } ថាបានអាន។
   *[message] បានសម្គាល់សារ { $count } ថាបានអាន។
}
toast-marked-unread = { $kind ->
    [conversation] បានសម្គាល់ការសន្ទនា { $count } ថាមិនទាន់អាន។
   *[message] បានសម្គាល់សារ { $count } ថាមិនទាន់អាន។
}
toast-undone = បានត្រឡប់សកម្មភាពវិញ។
toast-nothing-to-undo = គ្មានអ្វីត្រូវមិនធ្វើវិញទេ។
toast-cannot-undo-delete-forever = សំបុត្រដែលបានលុបជារៀងរហូត មិនអាចយកមកវិញបានទេ។
toast-send-undone = បានមិនធ្វើការផ្ញើវិញ។
toast-too-late-to-undo-send = យឺតពេលហើយក្នុងការមិនធ្វើវិញ៖ សារត្រូវបានផ្ញើរួចហើយ។
toast-undo = មិនធ្វើវិញ
toast-close = បិទ
toast-no-spam-folder = គណនីនេះគ្មានថតសារឥតបានការទេ។
