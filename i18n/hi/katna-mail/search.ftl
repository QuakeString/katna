# Katna Mail, Hindi (हिन्दी).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Search options (the panel from the button at the right of the search box)

search-options = खोज विकल्प
search-options-close = बंद करें
search-from = भेजने वाला
search-to = पाने वाला
search-subject = विषय
search-has-words = इन शब्दों वाले
search-without = इनके बिना
search-date-within = इतने समय के भीतर
search-has-attachment = अटैचमेंट वाले
search-attachment-custom = कस्टम
search-attachment-image = तस्वीर
search-attachment-custom-hint = कोई एक्सटेंशन टाइप करें, जैसे png, फिर Space दबाएं
search-attachment-remove = हटाएं
search-clear-filter = फ़िल्टर साफ़ करें

## Search options: "Date within" choices

search-within-any = कभी भी
search-within-days = { $count ->
    [one] { $count } दिन
   *[other] { $count } दिन
}
search-within-weeks = { $count ->
    [one] { $count } हफ़्ता
   *[other] { $count } हफ़्ते
}
search-within-months = { $count ->
    [one] { $count } महीना
   *[other] { $count } महीने
}
search-within-years = { $count ->
    [one] { $count } साल
   *[other] { $count } साल
}
search-within-custom = कस्टम

## Search options: custom dates (the calendar popover)

search-dates-on = इस दिन
search-dates-before = इससे पहले
search-dates-since = इसके बाद से
search-dates-between = इनके बीच
search-dates-from = से
search-dates-to = तक
search-dates-placeholder = वववव-मम-दद
search-dates-missing = तारीख चुनें
search-dates-unreadable = 2026-09-01 जैसी तारीख डालें
search-dates-out-of-range = वह तारीख सीमा से बाहर है
search-dates-chip-before = { $date } से पहले
search-dates-chip-since = { $date } से
search-dates-chip-between = { $first } – { $last }
search-dates-cancel = रद्द करें
search-dates-done = हो गया
search-dates-month-back = पिछला महीना
search-dates-month-on = अगला महीना
search-dates-year-back = पिछला साल
search-dates-year-on = अगला साल

## More results on server: under the results, mail found by asking the
## mail server, for mail that is not downloaded to this computer yet.

search-server-more = सर्वर पर और नतीजे
search-server-searching = सर्वर पर मेल खोजे जा रहे हैं…
search-server-empty-searching = अभी यहां कुछ नहीं है। सर्वर पर मेल खोजे जा रहे हैं…
search-server-nothing = सर्वर पर और कोई नतीजा नहीं
search-server-failed = सर्वर पर खोज नहीं हो सकी।
search-server-again = फिर से कोशिश करें
