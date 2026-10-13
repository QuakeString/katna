# Katna Mail, Amharic (አማርኛ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.
snooze-until = አሸልብ እስከ…
snooze-later-today = ዛሬ ቆይቶ
snooze-tomorrow = ነገ
snooze-this-weekend = በዚህ ሳምንት መጨረሻ
snooze-next-week = በሚቀጥለው ሳምንት
snooze-pick = ቀን እና ሰዓት ምረጥ
snooze-back = ወደ ሰዓቶቹ ተመለስ
snooze-type-placeholder = ሰዓት ይተይቡ
snooze-type-hint = ለምሳሌ «tue 3pm»፣ «tomorrow» ወይም «in 2 hours»
snooze-type-hint-unclear = Katna ያንን እንደ ሰዓት ማንበብ አይችልም
snooze-type-unclear = «{ $text }» Katna የሚያውቀው ሰዓት አይደለም

## Snooze and Remind me on mail: one menu, switched at its top (B and H)

snooze-tab = አሸልብ
remind-tab = አስታውሰኝ
snooze-says = እስከዚያ ድረስ ይደብቀዋል
remind-says = ባለበት ያቆየዋል እና ያሳውቅዎታል
remind-before-due = ጊዜው ከመድረሱ በፊት
remind-note = ማስታወሻ (አማራጭ)
remind-note-placeholder = ባዶ ከተተወ፣ ርዕሰ ጉዳዩ
toast-remind-set = አስታዋሽ ለ{ $date } ተዘጋጅቷል
remind-chat-line = አስታዋሽ { $date } · { $title }
remind-done = ተጠናቋል
toast-remind-done = አስታዋሹ ተጠናቋል
snooze-chat-line = እስከ { $date } አሸልቧል
snooze-chat-change = ቀይር
snooze-cancel = ይቅር
snooze-save = አስቀምጥ
snooze-in-the-past = ከአሁን በኋላ ያለ ሰዓት ይምረጡ።

## Follow up if no reply: compose's send menu, its popover and the chip
## beside Send

follow-up-menu = ምላሽ ካልመጣ ክትትል አድርግ…
follow-up-title = ምላሽ ካልመጣ ክትትል አድርግ
follow-up-off = ጠፍቷል
follow-up-days = { $days ->
    [one] { $days } ቀን
   *[other] { $days } ቀናት
}
follow-up-weeks = { $weeks ->
    [one] { $weeks } ሳምንት
   *[other] { $weeks } ሳምንታት
}
follow-up-pick = ምረጥ…
follow-up-pick-title = እስከዚህ ጊዜ ምላሽ ካልመጣ ክትትል አድርግ
follow-up-remind = አስታውሰኝ
follow-up-remind-note = ውይይቱ ወደ ገቢ መልዕክት ሳጥንዎ አናት ይመለሳል
follow-up-send = ለእኔ የክትትል መልዕክት ላክ
follow-up-send-note = ለተመሳሳይ ሰዎች፣ በተመሳሳይ ውይይት ውስጥ
follow-up-send-encrypted = ለተመሰጠረ ደብዳቤ አይሠራም
follow-up-text-placeholder = ምን እንደሚጻፍ
follow-up-text-named = ሰላም { $name }፣ ከታች ያለውን መልዕክቴን እንዳዩት ለማረጋገጥ ብቻ ነው።
follow-up-text = ሰላም፣ ከታች ያለውን መልዕክቴን እንዳዩት ለማረጋገጥ ብቻ ነው።
follow-up-template = አብነት ተጠቀም
follow-up-signature = ፊርማዎ ይታከላል
follow-up-again = አሁንም ምላሽ ካልመጣ፣ እንደገና ክትትል አድርግ ከዚህ በኋላ፦
follow-up-note = በውይይቱ ውስጥ ማንም ሰው እንደመለሰ ወዲያውኑ ይቆማል። ራስ-ሰር ምላሾች አይቆጠሩም።
follow-up-note-send = በውይይቱ ውስጥ ማንም ሰው እንደመለሰ ወዲያውኑ ይቆማል። በሥራ ቀናት ከ{ $start } እስከ { $end } ይላካል፣ ከአንድ ቀን በላይም በጭራሽ አይዘገይም።
follow-up-cancel = ይቅር
follow-up-done = ተጠናቋል
follow-up-chip-send = ክትትል በ{ $time } ውስጥ
follow-up-chip-remind = አስታዋሽ በ{ $time } ውስጥ
follow-up-chip-send-on = ክትትል { $date }
follow-up-chip-remind-on = አስታዋሽ { $date }

## The card over an open conversation a follow-up waits on.

follow-up-card-title = እስካሁን ምላሽ የለም
follow-up-card-title-waiting = የክትትል መልዕክትዎ በመጠበቅ ላይ ነው
follow-up-card-send = Katna የክትትል መልዕክትዎን በ{ $date } ይልካል። ማንም ሲመልስ ይቆማል።
follow-up-card-send-twice = Katna የክትትል መልዕክትዎን በ{ $date } ይልካል፣ ከዚያ በኋላ አንድ ጊዜ ተጨማሪ። ማንም ሲመልስ ይቆማል።
follow-up-card-remind = ማንም ካልመለሰ፣ ይህ ውይይት በ{ $date } ወደ ገቢ መልዕክት ሳጥንዎ ይመለሳል።
follow-up-card-waiting = ጊዜው የደረሰው ኮምፒውተርዎ ጠፍቶ እያለ ስለነበር ዘግይቶ አልተላከም። አሁን ይላኩት፣ አዲስ ሰዓት ይምረጡ ወይም ያቁሙት።
follow-up-card-edit = አርትዕ
follow-up-card-edit-title = ክትትል አድርግ በ
follow-up-card-send-now = አሁን ላክ
follow-up-card-stop = አቁም
follow-up-chat-send = ክትትል · ማንም ካልመለሰ { $date }
follow-up-chat-step = ክትትል { $step } ከ{ $steps } · ማንም ካልመለሰ { $date }
follow-up-chat-waiting = ክትትል በመጠበቅ ላይ · ጊዜው የደረሰው ኮምፒውተርዎ ጠፍቶ እያለ ነበር
follow-up-chat-remind = ምላሽ ካልመጣ { $date } ወደ ገቢ መልዕክት ሳጥን ይመለሳል
toast-follow-up-sent = የክትትል መልዕክቱ ተልኳል
toast-follow-up-stopped = ክትትሉ ቆሟል
toast-follow-up-moved = ክትትሉ ወደ { $date } ተዛውሯል
nudge-row = { $days ->
    [one] ከ{ $days } ቀን በፊት
   *[other] ከ{ $days } ቀናት በፊት
} ተልኳል። ክትትል ይደረግ?
nudge-row-tip = በውስጡ ላሉት ሁሉ የክትትል መልዕክት ጻፍ
nudge-follow-up = ክትትል አድርግ
nudge-dismiss = አሰናብት
nudge-card-title = እስካሁን ምላሽ የለም
nudge-card-text = { $days ->
    [one] ከ{ $days } ቀን በፊት
   *[other] ከ{ $days } ቀናት በፊት
} አንድ ነገር ጠይቀው ነበር፣ ማንም አልመለሰም።
nudge-chat-line = { $days ->
    [one] ከ{ $days } ቀን በፊት
   *[other] ከ{ $days } ቀናት በፊት
} ተልኳል፣ እስካሁን ምላሽ የለም
toast-nudge-dismissed = ማንቂያው ተሰናብቷል
