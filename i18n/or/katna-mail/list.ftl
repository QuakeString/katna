# Katna Mail, Odia (ଓଡ଼ିଆ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = ପ୍ରାଥମିକ
tab-promotions = ପ୍ରଚାର
tab-social = ସାମାଜିକ
tab-updates = ଅପଡେଟ
tab-forums = ଫୋରମ
tab-focused = ଫୋକସ୍‌ଡ
tab-other = ଅନ୍ୟ
tab-inbox = ଇନବକ୍ସ
tab-newsletters = ନ୍ୟୁଜଲେଟର
tab-notifications = ବିଜ୍ଞପ୍ତି
tab-new = { $count }ଟି ନୂଆ
tab-provider-other = Katna ଦ୍ୱାରା ସଜାଯାଇଛି

## Mail list: toolbar

list-select = ଚୟନ କରନ୍ତୁ
list-refresh = ରିଫ୍ରେସ କରନ୍ତୁ
list-checking = ନୂଆ ମେଲ ଯାଞ୍ଚ କରାଯାଉଛି…
list-more = ଅଧିକ
list-mark-read = ପଢ଼ାଯାଇଛି ଭାବେ ଚିହ୍ନିତ କରନ୍ତୁ
list-mark-unread = ପଢ଼ାଯାଇନାହିଁ ଭାବେ ଚିହ୍ନିତ କରନ୍ତୁ
list-move-to = ଏଠାକୁ ଘୁଞ୍ଚାନ୍ତୁ
list-archive = ଆର୍କାଇଭ କରନ୍ତୁ
list-spam = ସ୍ପାମ ଭାବେ ରିପୋର୍ଟ କରନ୍ତୁ
list-delete = ଡିଲିଟ କରନ୍ତୁ
list-snooze = ସ୍ନୁଜ କରନ୍ତୁ
list-unsnooze = ସ୍ନୁଜ ହଟାନ୍ତୁ
list-newer = ନୂଆ
list-older = ପୁରୁଣା
list-range = { $total }ରୁ { $first }–{ $last }
list-range-about = ପ୍ରାୟ { $total }ରୁ { $first }–{ $last }
list-results = “{ $query }” ପାଇଁ ଫଳାଫଳ
list-results-corrected = “{ $query }” ପାଇଁ ଫଳାଫଳ ଦେଖାଯାଉଛି
list-search-instead = ଏହା ବଦଳରେ “{ $query }” ସନ୍ଧାନ କରନ୍ତୁ
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = ସମସ୍ତ
list-pick-none = କୌଣସିଟି ନୁହେଁ
list-pick-read = ପଢ଼ାଯାଇଛି
list-pick-unread = ପଢ଼ାଯାଇନାହିଁ
list-pick-starred = ତାରାଙ୍କିତ
list-pick-unstarred = ତାରାଙ୍କିତ ନୁହେଁ

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] ସମସ୍ତ { $count }ଟି ବାର୍ତ୍ତାଳାପ ଚୟନ କରାଯାଇଛି।
       *[other] ସମସ୍ତ { $count }ଟି ବାର୍ତ୍ତାଳାପ ଚୟନ କରାଯାଇଛି।
    }
   *[message] { $count ->
        [one] ସମସ୍ତ { $count }ଟି ମେସେଜ ଚୟନ କରାଯାଇଛି।
       *[other] ସମସ୍ତ { $count }ଟି ମେସେଜ ଚୟନ କରାଯାଇଛି।
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder }ରେ ଥିବା ସମସ୍ତ { $count }ଟି ବାର୍ତ୍ତାଳାପ ଚୟନ କରାଯାଇଛି।
       *[other] { $folder }ରେ ଥିବା ସମସ୍ତ { $count }ଟି ବାର୍ତ୍ତାଳାପ ଚୟନ କରାଯାଇଛି।
    }
   *[message] { $count ->
        [one] { $folder }ରେ ଥିବା ସମସ୍ତ { $count }ଟି ମେସେଜ ଚୟନ କରାଯାଇଛି।
       *[other] { $folder }ରେ ଥିବା ସମସ୍ତ { $count }ଟି ମେସେଜ ଚୟନ କରାଯାଇଛି।
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] ସ୍କ୍ରିନରେ ଥିବା ସମସ୍ତ { $count }ଟି ବାର୍ତ୍ତାଳାପ ଚୟନ କରାଯାଇଛି।
       *[other] ସ୍କ୍ରିନରେ ଥିବା ସମସ୍ତ { $count }ଟି ବାର୍ତ୍ତାଳାପ ଚୟନ କରାଯାଇଛି।
    }
   *[message] { $count ->
        [one] ସ୍କ୍ରିନରେ ଥିବା ସମସ୍ତ { $count }ଟି ମେସେଜ ଚୟନ କରାଯାଇଛି।
       *[other] ସ୍କ୍ରିନରେ ଥିବା ସମସ୍ତ { $count }ଟି ମେସେଜ ଚୟନ କରାଯାଇଛି।
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] ସମସ୍ତ { $count }ଟି ବାର୍ତ୍ତାଳାପ ଚୟନ କରନ୍ତୁ
       *[other] ସମସ୍ତ { $count }ଟି ବାର୍ତ୍ତାଳାପ ଚୟନ କରନ୍ତୁ
    }
   *[message] { $count ->
        [one] ସମସ୍ତ { $count }ଟି ମେସେଜ ଚୟନ କରନ୍ତୁ
       *[other] ସମସ୍ତ { $count }ଟି ମେସେଜ ଚୟନ କରନ୍ତୁ
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder }ରେ ଥିବା ସମସ୍ତ { $count }ଟି ବାର୍ତ୍ତାଳାପ ଚୟନ କରନ୍ତୁ
       *[other] { $folder }ରେ ଥିବା ସମସ୍ତ { $count }ଟି ବାର୍ତ୍ତାଳାପ ଚୟନ କରନ୍ତୁ
    }
   *[message] { $count ->
        [one] { $folder }ରେ ଥିବା ସମସ୍ତ { $count }ଟି ମେସେଜ ଚୟନ କରନ୍ତୁ
       *[other] { $folder }ରେ ଥିବା ସମସ୍ତ { $count }ଟି ମେସେଜ ଚୟନ କରନ୍ତୁ
    }
}
list-selected-picked-screen = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] ସ୍କ୍ରିନରେ ଥିବା { $count }ଟି ପଢ଼ାଯାଇଥିବା ବାର୍ତ୍ତାଳାପ ଚୟନ କରାଯାଇଛି।
           *[other] ସ୍କ୍ରିନରେ ଥିବା ସମସ୍ତ { $count }ଟି ପଢ଼ାଯାଇଥିବା ବାର୍ତ୍ତାଳାପ ଚୟନ କରାଯାଇଛି।
        }
       *[message] { $count ->
            [one] ସ୍କ୍ରିନରେ ଥିବା { $count }ଟି ପଢ଼ାଯାଇଥିବା ମେସେଜ ଚୟନ କରାଯାଇଛି।
           *[other] ସ୍କ୍ରିନରେ ଥିବା ସମସ୍ତ { $count }ଟି ପଢ଼ାଯାଇଥିବା ମେସେଜ ଚୟନ କରାଯାଇଛି।
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] ସ୍କ୍ରିନରେ ଥିବା { $count }ଟି ପଢ଼ାଯାଇନଥିବା ବାର୍ତ୍ତାଳାପ ଚୟନ କରାଯାଇଛି।
           *[other] ସ୍କ୍ରିନରେ ଥିବା ସମସ୍ତ { $count }ଟି ପଢ଼ାଯାଇନଥିବା ବାର୍ତ୍ତାଳାପ ଚୟନ କରାଯାଇଛି।
        }
       *[message] { $count ->
            [one] ସ୍କ୍ରିନରେ ଥିବା { $count }ଟି ପଢ଼ାଯାଇନଥିବା ମେସେଜ ଚୟନ କରାଯାଇଛି।
           *[other] ସ୍କ୍ରିନରେ ଥିବା ସମସ୍ତ { $count }ଟି ପଢ଼ାଯାଇନଥିବା ମେସେଜ ଚୟନ କରାଯାଇଛି।
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] ସ୍କ୍ରିନରେ ଥିବା { $count }ଟି ତାରାଙ୍କିତ ବାର୍ତ୍ତାଳାପ ଚୟନ କରାଯାଇଛି।
           *[other] ସ୍କ୍ରିନରେ ଥିବା ସମସ୍ତ { $count }ଟି ତାରାଙ୍କିତ ବାର୍ତ୍ତାଳାପ ଚୟନ କରାଯାଇଛି।
        }
       *[message] { $count ->
            [one] ସ୍କ୍ରିନରେ ଥିବା { $count }ଟି ତାରାଙ୍କିତ ମେସେଜ ଚୟନ କରାଯାଇଛି।
           *[other] ସ୍କ୍ରିନରେ ଥିବା ସମସ୍ତ { $count }ଟି ତାରାଙ୍କିତ ମେସେଜ ଚୟନ କରାଯାଇଛି।
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] ସ୍କ୍ରିନରେ ଥିବା { $count }ଟି ତାରାଙ୍କିତ ନଥିବା ବାର୍ତ୍ତାଳାପ ଚୟନ କରାଯାଇଛି।
           *[other] ସ୍କ୍ରିନରେ ଥିବା ସମସ୍ତ { $count }ଟି ତାରାଙ୍କିତ ନଥିବା ବାର୍ତ୍ତାଳାପ ଚୟନ କରାଯାଇଛି।
        }
       *[message] { $count ->
            [one] ସ୍କ୍ରିନରେ ଥିବା { $count }ଟି ତାରାଙ୍କିତ ନଥିବା ମେସେଜ ଚୟନ କରାଯାଇଛି।
           *[other] ସ୍କ୍ରିନରେ ଥିବା ସମସ୍ତ { $count }ଟି ତାରାଙ୍କିତ ନଥିବା ମେସେଜ ଚୟନ କରାଯାଇଛି।
        }
    }
}
list-select-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $count }ଟି ପଢ଼ାଯାଇଥିବା ବାର୍ତ୍ତାଳାପ ଚୟନ କରନ୍ତୁ
           *[other] ସମସ୍ତ { $count }ଟି ପଢ଼ାଯାଇଥିବା ବାର୍ତ୍ତାଳାପ ଚୟନ କରନ୍ତୁ
        }
       *[message] { $count ->
            [one] { $count }ଟି ପଢ଼ାଯାଇଥିବା ମେସେଜ ଚୟନ କରନ୍ତୁ
           *[other] ସମସ୍ତ { $count }ଟି ପଢ଼ାଯାଇଥିବା ମେସେଜ ଚୟନ କରନ୍ତୁ
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $count }ଟି ପଢ଼ାଯାଇନଥିବା ବାର୍ତ୍ତାଳାପ ଚୟନ କରନ୍ତୁ
           *[other] ସମସ୍ତ { $count }ଟି ପଢ଼ାଯାଇନଥିବା ବାର୍ତ୍ତାଳାପ ଚୟନ କରନ୍ତୁ
        }
       *[message] { $count ->
            [one] { $count }ଟି ପଢ଼ାଯାଇନଥିବା ମେସେଜ ଚୟନ କରନ୍ତୁ
           *[other] ସମସ୍ତ { $count }ଟି ପଢ଼ାଯାଇନଥିବା ମେସେଜ ଚୟନ କରନ୍ତୁ
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $count }ଟି ତାରାଙ୍କିତ ବାର୍ତ୍ତାଳାପ ଚୟନ କରନ୍ତୁ
           *[other] ସମସ୍ତ { $count }ଟି ତାରାଙ୍କିତ ବାର୍ତ୍ତାଳାପ ଚୟନ କରନ୍ତୁ
        }
       *[message] { $count ->
            [one] { $count }ଟି ତାରାଙ୍କିତ ମେସେଜ ଚୟନ କରନ୍ତୁ
           *[other] ସମସ୍ତ { $count }ଟି ତାରାଙ୍କିତ ମେସେଜ ଚୟନ କରନ୍ତୁ
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $count }ଟି ତାରାଙ୍କିତ ନଥିବା ବାର୍ତ୍ତାଳାପ ଚୟନ କରନ୍ତୁ
           *[other] ସମସ୍ତ { $count }ଟି ତାରାଙ୍କିତ ନଥିବା ବାର୍ତ୍ତାଳାପ ଚୟନ କରନ୍ତୁ
        }
       *[message] { $count ->
            [one] { $count }ଟି ତାରାଙ୍କିତ ନଥିବା ମେସେଜ ଚୟନ କରନ୍ତୁ
           *[other] ସମସ୍ତ { $count }ଟି ତାରାଙ୍କିତ ନଥିବା ମେସେଜ ଚୟନ କରନ୍ତୁ
        }
    }
}
list-select-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $folder }ରେ ଥିବା { $count }ଟି ପଢ଼ାଯାଇଥିବା ବାର୍ତ୍ତାଳାପ ଚୟନ କରନ୍ତୁ
           *[other] { $folder }ରେ ଥିବା ସମସ୍ତ { $count }ଟି ପଢ଼ାଯାଇଥିବା ବାର୍ତ୍ତାଳାପ ଚୟନ କରନ୍ତୁ
        }
       *[message] { $count ->
            [one] { $folder }ରେ ଥିବା { $count }ଟି ପଢ଼ାଯାଇଥିବା ମେସେଜ ଚୟନ କରନ୍ତୁ
           *[other] { $folder }ରେ ଥିବା ସମସ୍ତ { $count }ଟି ପଢ଼ାଯାଇଥିବା ମେସେଜ ଚୟନ କରନ୍ତୁ
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $folder }ରେ ଥିବା { $count }ଟି ପଢ଼ାଯାଇନଥିବା ବାର୍ତ୍ତାଳାପ ଚୟନ କରନ୍ତୁ
           *[other] { $folder }ରେ ଥିବା ସମସ୍ତ { $count }ଟି ପଢ଼ାଯାଇନଥିବା ବାର୍ତ୍ତାଳାପ ଚୟନ କରନ୍ତୁ
        }
       *[message] { $count ->
            [one] { $folder }ରେ ଥିବା { $count }ଟି ପଢ଼ାଯାଇନଥିବା ମେସେଜ ଚୟନ କରନ୍ତୁ
           *[other] { $folder }ରେ ଥିବା ସମସ୍ତ { $count }ଟି ପଢ଼ାଯାଇନଥିବା ମେସେଜ ଚୟନ କରନ୍ତୁ
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $folder }ରେ ଥିବା { $count }ଟି ତାରାଙ୍କିତ ବାର୍ତ୍ତାଳାପ ଚୟନ କରନ୍ତୁ
           *[other] { $folder }ରେ ଥିବା ସମସ୍ତ { $count }ଟି ତାରାଙ୍କିତ ବାର୍ତ୍ତାଳାପ ଚୟନ କରନ୍ତୁ
        }
       *[message] { $count ->
            [one] { $folder }ରେ ଥିବା { $count }ଟି ତାରାଙ୍କିତ ମେସେଜ ଚୟନ କରନ୍ତୁ
           *[other] { $folder }ରେ ଥିବା ସମସ୍ତ { $count }ଟି ତାରାଙ୍କିତ ମେସେଜ ଚୟନ କରନ୍ତୁ
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $folder }ରେ ଥିବା { $count }ଟି ତାରାଙ୍କିତ ନଥିବା ବାର୍ତ୍ତାଳାପ ଚୟନ କରନ୍ତୁ
           *[other] { $folder }ରେ ଥିବା ସମସ୍ତ { $count }ଟି ତାରାଙ୍କିତ ନଥିବା ବାର୍ତ୍ତାଳାପ ଚୟନ କରନ୍ତୁ
        }
       *[message] { $count ->
            [one] { $folder }ରେ ଥିବା { $count }ଟି ତାରାଙ୍କିତ ନଥିବା ମେସେଜ ଚୟନ କରନ୍ତୁ
           *[other] { $folder }ରେ ଥିବା ସମସ୍ତ { $count }ଟି ତାରାଙ୍କିତ ନଥିବା ମେସେଜ ଚୟନ କରନ୍ତୁ
        }
    }
}
list-selected-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $count }ଟି ପଢ଼ାଯାଇଥିବା ବାର୍ତ୍ତାଳାପ ଚୟନ କରାଯାଇଛି।
           *[other] ସମସ୍ତ { $count }ଟି ପଢ଼ାଯାଇଥିବା ବାର୍ତ୍ତାଳାପ ଚୟନ କରାଯାଇଛି।
        }
       *[message] { $count ->
            [one] { $count }ଟି ପଢ଼ାଯାଇଥିବା ମେସେଜ ଚୟନ କରାଯାଇଛି।
           *[other] ସମସ୍ତ { $count }ଟି ପଢ଼ାଯାଇଥିବା ମେସେଜ ଚୟନ କରାଯାଇଛି।
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $count }ଟି ପଢ଼ାଯାଇନଥିବା ବାର୍ତ୍ତାଳାପ ଚୟନ କରାଯାଇଛି।
           *[other] ସମସ୍ତ { $count }ଟି ପଢ଼ାଯାଇନଥିବା ବାର୍ତ୍ତାଳାପ ଚୟନ କରାଯାଇଛି।
        }
       *[message] { $count ->
            [one] { $count }ଟି ପଢ଼ାଯାଇନଥିବା ମେସେଜ ଚୟନ କରାଯାଇଛି।
           *[other] ସମସ୍ତ { $count }ଟି ପଢ଼ାଯାଇନଥିବା ମେସେଜ ଚୟନ କରାଯାଇଛି।
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $count }ଟି ତାରାଙ୍କିତ ବାର୍ତ୍ତାଳାପ ଚୟନ କରାଯାଇଛି।
           *[other] ସମସ୍ତ { $count }ଟି ତାରାଙ୍କିତ ବାର୍ତ୍ତାଳାପ ଚୟନ କରାଯାଇଛି।
        }
       *[message] { $count ->
            [one] { $count }ଟି ତାରାଙ୍କିତ ମେସେଜ ଚୟନ କରାଯାଇଛି।
           *[other] ସମସ୍ତ { $count }ଟି ତାରାଙ୍କିତ ମେସେଜ ଚୟନ କରାଯାଇଛି।
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $count }ଟି ତାରାଙ୍କିତ ନଥିବା ବାର୍ତ୍ତାଳାପ ଚୟନ କରାଯାଇଛି।
           *[other] ସମସ୍ତ { $count }ଟି ତାରାଙ୍କିତ ନଥିବା ବାର୍ତ୍ତାଳାପ ଚୟନ କରାଯାଇଛି।
        }
       *[message] { $count ->
            [one] { $count }ଟି ତାରାଙ୍କିତ ନଥିବା ମେସେଜ ଚୟନ କରାଯାଇଛି।
           *[other] ସମସ୍ତ { $count }ଟି ତାରାଙ୍କିତ ନଥିବା ମେସେଜ ଚୟନ କରାଯାଇଛି।
        }
    }
}
list-selected-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $folder }ରେ ଥିବା { $count }ଟି ପଢ଼ାଯାଇଥିବା ବାର୍ତ୍ତାଳାପ ଚୟନ କରାଯାଇଛି।
           *[other] { $folder }ରେ ଥିବା ସମସ୍ତ { $count }ଟି ପଢ଼ାଯାଇଥିବା ବାର୍ତ୍ତାଳାପ ଚୟନ କରାଯାଇଛି।
        }
       *[message] { $count ->
            [one] { $folder }ରେ ଥିବା { $count }ଟି ପଢ଼ାଯାଇଥିବା ମେସେଜ ଚୟନ କରାଯାଇଛି।
           *[other] { $folder }ରେ ଥିବା ସମସ୍ତ { $count }ଟି ପଢ଼ାଯାଇଥିବା ମେସେଜ ଚୟନ କରାଯାଇଛି।
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $folder }ରେ ଥିବା { $count }ଟି ପଢ଼ାଯାଇନଥିବା ବାର୍ତ୍ତାଳାପ ଚୟନ କରାଯାଇଛି।
           *[other] { $folder }ରେ ଥିବା ସମସ୍ତ { $count }ଟି ପଢ଼ାଯାଇନଥିବା ବାର୍ତ୍ତାଳାପ ଚୟନ କରାଯାଇଛି।
        }
       *[message] { $count ->
            [one] { $folder }ରେ ଥିବା { $count }ଟି ପଢ଼ାଯାଇନଥିବା ମେସେଜ ଚୟନ କରାଯାଇଛି।
           *[other] { $folder }ରେ ଥିବା ସମସ୍ତ { $count }ଟି ପଢ଼ାଯାଇନଥିବା ମେସେଜ ଚୟନ କରାଯାଇଛି।
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $folder }ରେ ଥିବା { $count }ଟି ତାରାଙ୍କିତ ବାର୍ତ୍ତାଳାପ ଚୟନ କରାଯାଇଛି।
           *[other] { $folder }ରେ ଥିବା ସମସ୍ତ { $count }ଟି ତାରାଙ୍କିତ ବାର୍ତ୍ତାଳାପ ଚୟନ କରାଯାଇଛି।
        }
       *[message] { $count ->
            [one] { $folder }ରେ ଥିବା { $count }ଟି ତାରାଙ୍କିତ ମେସେଜ ଚୟନ କରାଯାଇଛି।
           *[other] { $folder }ରେ ଥିବା ସମସ୍ତ { $count }ଟି ତାରାଙ୍କିତ ମେସେଜ ଚୟନ କରାଯାଇଛି।
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $folder }ରେ ଥିବା { $count }ଟି ତାରାଙ୍କିତ ନଥିବା ବାର୍ତ୍ତାଳାପ ଚୟନ କରାଯାଇଛି।
           *[other] { $folder }ରେ ଥିବା ସମସ୍ତ { $count }ଟି ତାରାଙ୍କିତ ନଥିବା ବାର୍ତ୍ତାଳାପ ଚୟନ କରାଯାଇଛି।
        }
       *[message] { $count ->
            [one] { $folder }ରେ ଥିବା { $count }ଟି ତାରାଙ୍କିତ ନଥିବା ମେସେଜ ଚୟନ କରାଯାଇଛି।
           *[other] { $folder }ରେ ଥିବା ସମସ୍ତ { $count }ଟି ତାରାଙ୍କିତ ନଥିବା ମେସେଜ ଚୟନ କରାଯାଇଛି।
        }
    }
}
list-picked-none = { $pick ->
    [read] { $kind ->
        [conversation] ଏଠାରେ କୌଣସି ପଢ଼ାଯାଇଥିବା ବାର୍ତ୍ତାଳାପ ନାହିଁ।
       *[message] ଏଠାରେ କୌଣସି ପଢ଼ାଯାଇଥିବା ମେସେଜ ନାହିଁ।
    }
   *[unread] { $kind ->
        [conversation] ଏଠାରେ କୌଣସି ପଢ଼ାଯାଇନଥିବା ବାର୍ତ୍ତାଳାପ ନାହିଁ।
       *[message] ଏଠାରେ କୌଣସି ପଢ଼ାଯାଇନଥିବା ମେସେଜ ନାହିଁ।
    }
    [starred] { $kind ->
        [conversation] ଏଠାରେ କୌଣସି ତାରାଙ୍କିତ ବାର୍ତ୍ତାଳାପ ନାହିଁ।
       *[message] ଏଠାରେ କୌଣସି ତାରାଙ୍କିତ ମେସେଜ ନାହିଁ।
    }
    [unstarred] { $kind ->
        [conversation] ଏଠାରେ କୌଣସି ତାରାଙ୍କିତ ନଥିବା ବାର୍ତ୍ତାଳାପ ନାହିଁ।
       *[message] ଏଠାରେ କୌଣସି ତାରାଙ୍କିତ ନଥିବା ମେସେଜ ନାହିଁ।
    }
}
list-clear-selection = ଚୟନ ଖାଲି କରନ୍ତୁ

## Mail list: empty states

list-empty-search = ଆପଣଙ୍କ ସନ୍ଧାନ ସହ କୌଣସି ମେସେଜ ମେଳ ଖାଇଲା ନାହିଁ।
list-empty-tab = { $tab }ରେ କୌଣସି ମେଲ ନାହିଁ।
list-empty-tab-unknown = ଏହି ଟାବରେ କୌଣସି ମେଲ ନାହିଁ।
list-empty-folder = { $folder }ରେ କୌଣସି ମେସେଜ ନାହିଁ।
list-empty-folder-unknown = ଏହି ଫୋଲ୍ଡରରେ କୌଣସି ମେସେଜ ନାହିଁ।
list-first-sync = ଆପଣଙ୍କ ମେଲ ଅଣାଯାଉଛି…
list-first-sync-detail = ଏହା ଆସିବା ସହ ଏଠାରେ ଦେଖାଯିବ।

## Mail list: lines

row-removed = ଏହି ମେସେଜକୁ କାଢ଼ି ଦିଆଯାଇଛି।
row-starred = ତାରାଙ୍କିତ
row-not-starred = ତାରାଙ୍କିତ ନୁହେଁ
row-important = ଗୁରୁତ୍ୱପୂର୍ଣ୍ଣ। ଗୁରୁତ୍ୱପୂର୍ଣ୍ଣ ନୁହେଁ ଭାବେ ଚିହ୍ନିତ କରିବାକୁ କ୍ଲିକ କରନ୍ତୁ।
row-mark-important = ଗୁରୁତ୍ୱପୂର୍ଣ୍ଣ ଭାବେ ଚିହ୍ନିତ କରନ୍ତୁ
row-pinned = ଉପରେ ପିନ କରାଯାଇଛି
row-tracking-none = ଟ୍ରାକ କରାଯାଇଛି। ଏପର୍ଯ୍ୟନ୍ତ ଖୋଲାଯାଇନାହିଁ
row-tracking-opened = { $recipients } ଜଣଙ୍କ ମଧ୍ୟରୁ { $opened } ଜଣ ଖୋଲିଛନ୍ତି
row-tracking-clicked = { $recipients } ଜଣଙ୍କ ମଧ୍ୟରୁ { $opened } ଜଣ ଖୋଲିଛନ୍ତି, { $clicked } ଜଣ ଲିଙ୍କ ଖୋଲିଛନ୍ତି
row-pin = ଉପରେ ପିନ କରନ୍ତୁ
row-unpin = ଅନପିନ କରନ୍ତୁ
row-snoozed-until = { $when } ପର୍ଯ୍ୟନ୍ତ ସ୍ନୁଜ କରାଯାଇଛି

## Mail list: More menu and right-click menu

menu-reply = ଉତ୍ତର ଦିଅନ୍ତୁ
menu-reply-all = ସମସ୍ତଙ୍କୁ ଉତ୍ତର ଦିଅନ୍ତୁ
menu-forward = ଫରୱାର୍ଡ କରନ୍ତୁ
menu-archive = ଆର୍କାଇଭ କରନ୍ତୁ
menu-delete = ଡିଲିଟ କରନ୍ତୁ
menu-delete-forever = ସ୍ଥାୟୀ ଭାବେ ଡିଲିଟ କରନ୍ତୁ
menu-move-to-inbox = ଇନବକ୍ସକୁ ଘୁଞ୍ଚାନ୍ତୁ
menu-spam = ସ୍ପାମ ଭାବେ ରିପୋର୍ଟ କରନ୍ତୁ
menu-not-spam = ସ୍ପାମ ନୁହେଁ
menu-mark-read = ପଢ଼ାଯାଇଛି ଭାବେ ଚିହ୍ନିତ କରନ୍ତୁ
menu-mark-unread = ପଢ଼ାଯାଇନାହିଁ ଭାବେ ଚିହ୍ନିତ କରନ୍ତୁ
menu-mark-all-read = ସବୁକୁ ପଢ଼ାଯାଇଛି ଭାବେ ଚିହ୍ନିତ କରନ୍ତୁ
menu-star = ତାରା ଯୋଗ କରନ୍ତୁ
menu-unstar = ତାରା କାଢ଼ନ୍ତୁ
menu-important = ଗୁରୁତ୍ୱପୂର୍ଣ୍ଣ ଭାବେ ଚିହ୍ନିତ କରନ୍ତୁ
menu-not-important = ଗୁରୁତ୍ୱପୂର୍ଣ୍ଣ ନୁହେଁ ଭାବେ ଚିହ୍ନିତ କରନ୍ତୁ
menu-pin = ଉପରେ ପିନ କରନ୍ତୁ
menu-unpin = ଅନପିନ କରନ୍ତୁ
menu-snooze = ସ୍ନୁଜ କରନ୍ତୁ
menu-unsnooze = ସ୍ନୁଜ ହଟାନ୍ତୁ
menu-add-to-tasks = କାର୍ଯ୍ୟରେ ଯୋଗ କରନ୍ତୁ
menu-add-note = ନୋଟ ଯୋଗ କରନ୍ତୁ
menu-print-all = ସବୁ ପ୍ରିଣ୍ଟ କରନ୍ତୁ
menu-new-window = ନୂଆ ୱିଣ୍ଡୋରେ ଖୋଲନ୍ତୁ
menu-move-to = ଏଠାକୁ ଘୁଞ୍ଚାନ୍ତୁ
menu-move-to-heading = ଏଠାକୁ ଘୁଞ୍ଚାନ୍ତୁ:
menu-find-from = { $name }ଙ୍କଠାରୁ ଇମେଲ ଖୋଜନ୍ତୁ

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] ବାର୍ତ୍ତାଳାପ ଆର୍କାଇଭ କରାଗଲା।
       *[other] { $count }ଟି ବାର୍ତ୍ତାଳାପ ଆର୍କାଇଭ କରାଗଲା।
    }
   *[message] { $count ->
        [one] ମେସେଜ ଆର୍କାଇଭ କରାଗଲା।
       *[other] { $count }ଟି ମେସେଜ ଆର୍କାଇଭ କରାଗଲା।
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] ବାର୍ତ୍ତାଳାପ ଟ୍ରାସକୁ ଘୁଞ୍ଚାଗଲା।
       *[other] { $count }ଟି ବାର୍ତ୍ତାଳାପ ଟ୍ରାସକୁ ଘୁଞ୍ଚାଗଲା।
    }
   *[message] { $count ->
        [one] ମେସେଜ ଟ୍ରାସକୁ ଘୁଞ୍ଚାଗଲା।
       *[other] { $count }ଟି ମେସେଜ ଟ୍ରାସକୁ ଘୁଞ୍ଚାଗଲା।
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] ବାର୍ତ୍ତାଳାପ ଘୁଞ୍ଚାଗଲା।
       *[other] { $count }ଟି ବାର୍ତ୍ତାଳାପ ଘୁଞ୍ଚାଗଲା।
    }
   *[message] { $count ->
        [one] ମେସେଜ ଘୁଞ୍ଚାଗଲା।
       *[other] { $count }ଟି ମେସେଜ ଘୁଞ୍ଚାଗଲା।
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] ବାର୍ତ୍ତାଳାପ ତାରାଙ୍କିତ କରାଗଲା।
       *[other] { $count }ଟି ବାର୍ତ୍ତାଳାପ ତାରାଙ୍କିତ କରାଗଲା।
    }
   *[message] { $count ->
        [one] ମେସେଜ ତାରାଙ୍କିତ କରାଗଲା।
       *[other] { $count }ଟି ମେସେଜ ତାରାଙ୍କିତ କରାଗଲା।
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] ବାର୍ତ୍ତାଳାପରୁ ତାରା କଢ଼ାଗଲା।
       *[other] { $count }ଟି ବାର୍ତ୍ତାଳାପରୁ ତାରା କଢ଼ାଗଲା।
    }
   *[message] { $count ->
        [one] ମେସେଜରୁ ତାରା କଢ଼ାଗଲା।
       *[other] { $count }ଟି ମେସେଜରୁ ତାରା କଢ଼ାଗଲା।
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] ବାର୍ତ୍ତାଳାପ ଗୁରୁତ୍ୱପୂର୍ଣ୍ଣ ଭାବେ ଚିହ୍ନିତ କରାଗଲା।
       *[other] { $count }ଟି ବାର୍ତ୍ତାଳାପ ଗୁରୁତ୍ୱପୂର୍ଣ୍ଣ ଭାବେ ଚିହ୍ନିତ କରାଗଲା।
    }
   *[message] { $count ->
        [one] ମେସେଜ ଗୁରୁତ୍ୱପୂର୍ଣ୍ଣ ଭାବେ ଚିହ୍ନିତ କରାଗଲା।
       *[other] { $count }ଟି ମେସେଜ ଗୁରୁତ୍ୱପୂର୍ଣ୍ଣ ଭାବେ ଚିହ୍ନିତ କରାଗଲା।
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] ବାର୍ତ୍ତାଳାପ ଗୁରୁତ୍ୱପୂର୍ଣ୍ଣ ନୁହେଁ ଭାବେ ଚିହ୍ନିତ କରାଗଲା।
       *[other] { $count }ଟି ବାର୍ତ୍ତାଳାପ ଗୁରୁତ୍ୱପୂର୍ଣ୍ଣ ନୁହେଁ ଭାବେ ଚିହ୍ନିତ କରାଗଲା।
    }
   *[message] { $count ->
        [one] ମେସେଜ ଗୁରୁତ୍ୱପୂର୍ଣ୍ଣ ନୁହେଁ ଭାବେ ଚିହ୍ନିତ କରାଗଲା।
       *[other] { $count }ଟି ମେସେଜ ଗୁରୁତ୍ୱପୂର୍ଣ୍ଣ ନୁହେଁ ଭାବେ ଚିହ୍ନିତ କରାଗଲା।
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] ବାର୍ତ୍ତାଳାପ ଉପରେ ପିନ କରାଗଲା।
       *[other] { $count }ଟି ବାର୍ତ୍ତାଳାପ ଉପରେ ପିନ କରାଗଲା।
    }
   *[message] { $count ->
        [one] ମେସେଜ ଉପରେ ପିନ କରାଗଲା।
       *[other] { $count }ଟି ମେସେଜ ଉପରେ ପିନ କରାଗଲା।
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] ବାର୍ତ୍ତାଳାପ ଅନପିନ କରାଗଲା।
       *[other] { $count }ଟି ବାର୍ତ୍ତାଳାପ ଅନପିନ କରାଗଲା।
    }
   *[message] { $count ->
        [one] ମେସେଜ ଅନପିନ କରାଗଲା।
       *[other] { $count }ଟି ମେସେଜ ଅନପିନ କରାଗଲା।
    }
}
toast-snoozed = { $kind ->
    [conversation] { $count ->
        [one] ବାର୍ତ୍ତାଳାପ { $when } ପର୍ଯ୍ୟନ୍ତ ସ୍ନୁଜ କରାଗଲା।
       *[other] { $count }ଟି ବାର୍ତ୍ତାଳାପ { $when } ପର୍ଯ୍ୟନ୍ତ ସ୍ନୁଜ କରାଗଲା।
    }
   *[message] { $count ->
        [one] ମେସେଜ { $when } ପର୍ଯ୍ୟନ୍ତ ସ୍ନୁଜ କରାଗଲା।
       *[other] { $count }ଟି ମେସେଜ { $when } ପର୍ଯ୍ୟନ୍ତ ସ୍ନୁଜ କରାଗଲା।
    }
}
toast-unsnoozed = { $kind ->
    [conversation] { $count ->
        [one] ବାର୍ତ୍ତାଳାପ ଇନବକ୍ସକୁ ଫେରିଆସିଲା।
       *[other] { $count }ଟି ବାର୍ତ୍ତାଳାପ ଇନବକ୍ସକୁ ଫେରିଆସିଲା।
    }
   *[message] { $count ->
        [one] ମେସେଜ ଇନବକ୍ସକୁ ଫେରିଆସିଲା।
       *[other] { $count }ଟି ମେସେଜ ଇନବକ୍ସକୁ ଫେରିଆସିଲା।
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] ବାର୍ତ୍ତାଳାପ ସ୍ପାମ ଭାବେ ରିପୋର୍ଟ କରାଗଲା।
       *[other] { $count }ଟି ବାର୍ତ୍ତାଳାପ ସ୍ପାମ ଭାବେ ରିପୋର୍ଟ କରାଗଲା।
    }
   *[message] { $count ->
        [one] ମେସେଜ ସ୍ପାମ ଭାବେ ରିପୋର୍ଟ କରାଗଲା।
       *[other] { $count }ଟି ମେସେଜ ସ୍ପାମ ଭାବେ ରିପୋର୍ଟ କରାଗଲା।
    }
}
toast-not-spam = { $kind ->
    [conversation] { $count ->
        [one] ବାର୍ତ୍ତାଳାପକୁ ସ୍ପାମ ନୁହେଁ ଭାବେ ଚିହ୍ନିତ କରି ଇନବକ୍ସକୁ ଘୁଞ୍ଚାଯାଇଛି।
       *[other] { $count }ଟି ବାର୍ତ୍ତାଳାପକୁ ସ୍ପାମ ନୁହେଁ ଭାବେ ଚିହ୍ନିତ କରି ଇନବକ୍ସକୁ ଘୁଞ୍ଚାଯାଇଛି।
    }
   *[message] { $count ->
        [one] ମେସେଜକୁ ସ୍ପାମ ନୁହେଁ ଭାବେ ଚିହ୍ନିତ କରି ଇନବକ୍ସକୁ ଘୁଞ୍ଚାଯାଇଛି।
       *[other] { $count }ଟି ମେସେଜକୁ ସ୍ପାମ ନୁହେଁ ଭାବେ ଚିହ୍ନିତ କରି ଇନବକ୍ସକୁ ଘୁଞ୍ଚାଯାଇଛି।
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] ବାର୍ତ୍ତାଳାପ ସ୍ଥାୟୀ ଭାବେ ଡିଲିଟ କରାଗଲା।
       *[other] { $count }ଟି ବାର୍ତ୍ତାଳାପ ସ୍ଥାୟୀ ଭାବେ ଡିଲିଟ କରାଗଲା।
    }
   *[message] { $count ->
        [one] ମେସେଜ ସ୍ଥାୟୀ ଭାବେ ଡିଲିଟ କରାଗଲା।
       *[other] { $count }ଟି ମେସେଜ ସ୍ଥାୟୀ ଭାବେ ଡିଲିଟ କରାଗଲା।
    }
}
toast-marked-read = { $kind ->
    [conversation] { $count ->
        [one] ବାର୍ତ୍ତାଳାପ ପଢ଼ାଯାଇଛି ଭାବେ ଚିହ୍ନିତ କରାଗଲା।
       *[other] { $count }ଟି ବାର୍ତ୍ତାଳାପ ପଢ଼ାଯାଇଛି ଭାବେ ଚିହ୍ନିତ କରାଗଲା।
    }
   *[message] { $count ->
        [one] ମେସେଜ ପଢ଼ାଯାଇଛି ଭାବେ ଚିହ୍ନିତ କରାଗଲା।
       *[other] { $count }ଟି ମେସେଜ ପଢ଼ାଯାଇଛି ଭାବେ ଚିହ୍ନିତ କରାଗଲା।
    }
}
toast-marked-unread = { $kind ->
    [conversation] { $count ->
        [one] ବାର୍ତ୍ତାଳାପ ପଢ଼ାଯାଇନାହିଁ ଭାବେ ଚିହ୍ନିତ କରାଗଲା।
       *[other] { $count }ଟି ବାର୍ତ୍ତାଳାପ ପଢ଼ାଯାଇନାହିଁ ଭାବେ ଚିହ୍ନିତ କରାଗଲା।
    }
   *[message] { $count ->
        [one] ମେସେଜ ପଢ଼ାଯାଇନାହିଁ ଭାବେ ଚିହ୍ନିତ କରାଗଲା।
       *[other] { $count }ଟି ମେସେଜ ପଢ଼ାଯାଇନାହିଁ ଭାବେ ଚିହ୍ନିତ କରାଗଲା।
    }
}
toast-undone = କାର୍ଯ୍ୟ ପୂର୍ବବତ୍ କରାଗଲା।
toast-nothing-to-undo = ପୂର୍ବବତ୍ କରିବାକୁ କିଛି ନାହିଁ।
toast-cannot-undo-delete-forever = ସ୍ଥାୟୀ ଭାବେ ଡିଲିଟ ହୋଇଥିବା ମେଲ ଫେରାଇ ଅଣାଯାଇପାରିବ ନାହିଁ।
toast-send-undone = ପଠାଇବା ପୂର୍ବବତ୍ କରାଗଲା।
toast-too-late-to-undo-send = ପୂର୍ବବତ୍ କରିବାକୁ ବହୁତ ଡେରି ହୋଇଗଲା: ମେସେଜଟି ପୂର୍ବରୁ ପଠାଯାଇସାରିଛି।
toast-undo = ପୂର୍ବବତ୍ କରନ୍ତୁ
toast-close = ବନ୍ଦ କରନ୍ତୁ
toast-no-spam-folder = ଏହି ଆକାଉଣ୍ଟରେ କୌଣସି ସ୍ପାମ ଫୋଲ୍ଡର ନାହିଁ।
