# Katna Mail, Bengali (বাংলা).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Search options (the panel from the button at the right of the search box)

search-options = অনুসন্ধানের বিকল্প
search-options-close = বন্ধ করুন
search-from = প্রেরক
search-to = প্রাপক
search-subject = বিষয়
search-has-words = এই শব্দগুলো আছে
search-without = এগুলো নেই
search-date-within = তারিখের সময়সীমা
search-has-attachment = সংযুক্তি আছে
search-clear-filter = ফিল্টার মুছুন

## Search options: "Date within" choices

search-within-any = যেকোনো সময়
search-within-days = { $count ->
    [one] { $count } দিন
   *[other] { $count } দিন
}
search-within-weeks = { $count ->
    [one] { $count } সপ্তাহ
   *[other] { $count } সপ্তাহ
}
search-within-months = { $count ->
    [one] { $count } মাস
   *[other] { $count } মাস
}
search-within-years = { $count ->
    [one] { $count } বছর
   *[other] { $count } বছর
}
search-within-custom = নিজে বাছুন

## Search options: custom dates (the calendar popover)

search-dates-on = নির্দিষ্ট দিনে
search-dates-before = আগে
search-dates-since = থেকে
search-dates-between = মধ্যে
search-dates-from = শুরু
search-dates-to = শেষ
search-dates-placeholder = বববব-মম-দদ
search-dates-missing = একটি তারিখ বাছুন
search-dates-unreadable = 2026-09-01-এর মতো একটি তারিখ দিন
search-dates-out-of-range = তারিখটি সীমার বাইরে
search-dates-chip-before = { $date }-এর আগে
search-dates-chip-since = { $date } থেকে
search-dates-chip-between = { $first } – { $last }
search-dates-cancel = বাতিল করুন
search-dates-done = হয়ে গেছে
search-dates-month-back = আগের মাস
search-dates-month-on = পরের মাস
search-dates-year-back = আগের বছর
search-dates-year-on = পরের বছর
