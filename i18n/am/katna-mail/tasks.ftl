# Katna Mail, Amharic (አማርኛ): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = ፍጠር
tasks-all = ሁሉም ተግባራት
tasks-today = ዛሬ
tasks-starred = ኮከብ የተደረገባቸው
tasks-new-list = አዲስ ዝርዝር ፍጠር
tasks-on-this-computer = በዚህ ኮምፒዩተር ላይ
tasks-my-tasks = የእኔ ተግባራት
# The line under an account in the side list whose task lists could not
# come: why, and the one click that fixes it.
tasks-account-sign-in = ተግባራትን ለማሳየት እንደገና ይግቡ
tasks-account-signed-in = እንደገና ወደ { $address } ገብተዋል። ተግባራትዎን በማምጣት ላይ…
tasks-account-sign-in-refused = { $provider } Katnaን አላስገባም። እንደገና ይሞክሩ፣ እና ተግባራትዎን እንዲደርስባቸው ይፍቀዱ።
tasks-account-refused = አገልጋዩ የይለፍ ቃሉን አልተቀበለም። Yahoo፣ iCloud፣ Zoho እና ሌሎችም የመተግበሪያ የይለፍ ቃል ያስፈልጋቸዋል።
tasks-account-change-password = የይለፍ ቃል ቀይር
tasks-account-change-password-tooltip = ቅንብሮች > መለያዎች ክፈት
tasks-account-not-enabled = ለKatna የተግባራት መዳረሻ ገና አልበራም።
tasks-account-failed = የተግባር ዝርዝሮቹን ማንበብ አልተቻለም።
# $reason is the server's own words, in English.
tasks-account-error = የተግባር ዝርዝሮቹን ማንበብ አልተቻለም፦ { $reason }
tasks-account-none = ምንም የተግባር ዝርዝር አልተገኘም
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
tasks-account-none-why = ምንም የተግባር ዝርዝር አልተገኘም: { $reason }
# A Gmail or Outlook account added with a password: its tasks need the
# provider's sign-in.
tasks-account-use-sign-in = { $provider } ተግባራትን የሚያሳየው በ{ $provider } ለገባ Katna ብቻ ነው።
tasks-account-sign-in-with = በ{ $provider } ይግቡ
tasks-account-looking = የተግባር ዝርዝሮችን በመፈለግ ላይ…
tasks-account-try-again = እንደገና ሞክር
tasks-account-try-again-tooltip = የዚህን መለያ ተግባራት አሁን እንደገና ፈትሽ
tasks-account-fixing = በሂደት ላይ…
tasks-list-name-placeholder = የዝርዝር ስም

## Lists and tasks

tasks-loading = ተግባሮችዎ በማንበብ ላይ…
tasks-no-lists = የተግባር ዝርዝሮችዎ እዚህ ይታያሉ።
tasks-search = ተግባራትን ፈልግ
tasks-search-none = ከፍለጋዎ ጋር የሚዛመድ ተግባር የለም።
tasks-add = ተግባር ያክሉ
tasks-title-placeholder = ርዕስ
tasks-add-step = ንዑስ ተግባር ያክሉ
tasks-empty = ገና ምንም ተግባር የለም። ከላይ አንድ ያክሉ።
tasks-starred-empty = እዚህ ለማየት በተግባር ላይ ኮከብ ያድርጉ።
tasks-today-empty = ዛሬ የሚጠናቀቅ ምንም የለም።
tasks-today-date = { $weekday }፣ { $day }
tasks-overdue = ያለፈባቸው
tasks-completed = { $count ->
    [one] የተጠናቀቁ ({ $count })
   *[other] የተጠናቀቁ ({ $count })
}
tasks-list-options = የዝርዝር አማራጮች
tasks-rename-list = ዝርዝር ዳግም ሰይም
tasks-delete-list = ዝርዝር ሰርዝ
tasks-mark-done = እንደተጠናቀቀ ምልክት አድርግ
tasks-mark-open = እንዳልተጠናቀቀ ምልክት አድርግ
tasks-star = ኮከብ አድርግ
tasks-unstar = ኮከብ አስወግድ
tasks-edit-title = ርዕስ አርትዕ
tasks-details = ዝርዝሮች
tasks-delete = ሰርዝ
tasks-move-to = ወደ { $list } ውሰድ
tasks-from-mail = ደብዳቤ
tasks-open-mail = ደብዳቤውን ክፈት
tasks-from-note = ማስታወሻ
tasks-open-note = ማስታወሻውን ክፈት
tasks-note-gone = ያ ማስታወሻ ከእንግዲህ እዚህ የለም።
tasks-no-subject = (ርዕሰ ጉዳይ የለም)

## The details dialog

tasks-notes-placeholder = ዝርዝሮችን ያክሉ
tasks-date = ቀን
tasks-no-date = ቀን የለም
tasks-time-placeholder = ሰዓት ያክሉ
tasks-repeat = ድገም
tasks-repeat-never = አይደገምም
tasks-repeat-daily = በየቀኑ
tasks-repeat-weekly = በየሳምንቱ
tasks-repeat-monthly = በየወሩ
tasks-repeat-yearly = በየዓመቱ
tasks-repeat-other = ብጁ
tasks-remind = አስታውሰኝ
tasks-remind-off = አታስታውሰኝ
tasks-remind-on-time = በሰዓቱ
tasks-remind-morning = በዕለቱ፣ { $time }
tasks-remind-hour-before = ከአንድ ሰዓት በፊት
tasks-remind-day-before = ከአንድ ቀን በፊት
tasks-cancel = ይቅር
tasks-save = አስቀምጥ
tasks-not-a-time = “{ $text }” ሰዓት አይደለም፣ ለምሳሌ { $example }።

## Due days

tasks-due-today = ዛሬ
tasks-due-tomorrow = ነገ
tasks-due-yesterday = ትናንት
tasks-due-at = { $day }፣ { $time }

## Notes at the bottom

tasks-toast-done = ተግባሩ ተጠናቅቋል
tasks-toast-next = ተጠናቋል። ቀጣዩ በ{ $date }
tasks-toast-deleted = ተግባሩ ተሰርዟል
tasks-toast-added = { $count ->
    [one] ወደ ተግባራት ታክሏል
   *[other] { $count } ተግባራት ታክለዋል
}
tasks-mail-gone = ያ ደብዳቤ ከእንግዲህ እዚህ የለም።
tasks-toast-list-deleted = ዝርዝሩ ተሰርዟል
tasks-toast-moved = ወደ { $list } ተወስዷል
tasks-toast-rescheduled = ተግባሩ በአዲስ መርሐግብር ተይዟል
