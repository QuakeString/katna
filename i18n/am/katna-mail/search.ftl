# Katna Mail, Amharic (አማርኛ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Search options (the panel from the button at the right of the search box)

search-options = የፍለጋ አማራጮች
search-options-close = ዝጋ
search-from = ከ
search-to = ለ
search-subject = ርዕሰ ጉዳይ
search-has-words = ቃላቱን የያዘ
search-without = የማይይዘው
search-date-within = የጊዜ ክልል
search-has-attachment = አባሪ ያለው
search-attachment-custom = ብጁ
search-attachment-custom-hint = ቅጥያ ይተይቡ፣ ለምሳሌ png፣ ከዚያ Space ይጫኑ
search-attachment-remove = አስወግድ
search-clear-filter = ማጣሪያውን አጽዳ

## Search options: "Date within" choices

search-within-any = በማንኛውም ጊዜ
search-within-days = { $count ->
    [one] { $count } ቀን
   *[other] { $count } ቀናት
}
search-within-weeks = { $count ->
    [one] { $count } ሳምንት
   *[other] { $count } ሳምንታት
}
search-within-months = { $count ->
    [one] { $count } ወር
   *[other] { $count } ወራት
}
search-within-years = { $count ->
    [one] { $count } ዓመት
   *[other] { $count } ዓመታት
}
search-within-custom = ብጁ

## Search options: custom dates (the calendar popover)

search-dates-on = በ
search-dates-before = በፊት
search-dates-since = ጀምሮ
search-dates-between = መካከል
search-dates-from = ከ
search-dates-to = እስከ
search-dates-placeholder = ዓዓዓዓ-ወወ-ቀቀ
search-dates-missing = ቀን ይምረጡ
search-dates-unreadable = እንደ 2026-09-01 ያለ ቀን ይጠቀሙ
search-dates-out-of-range = ያ ቀን ከክልል ውጭ ነው
search-dates-chip-before = ከ{ $date } በፊት
search-dates-chip-since = ከ{ $date } ጀምሮ
search-dates-chip-between = { $first } – { $last }
search-dates-cancel = ይቅር
search-dates-done = ተጠናቋል
search-dates-month-back = ያለፈው ወር
search-dates-month-on = ቀጣዩ ወር
search-dates-year-back = ያለፈው ዓመት
search-dates-year-on = ቀጣዩ ዓመት
