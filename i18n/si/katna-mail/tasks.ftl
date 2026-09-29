# Katna Mail, Sinhala (සිංහල): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = සාදන්න
tasks-all = සියලු කාර්යයන්
tasks-today = අද
tasks-starred = තරු යෙදූ
tasks-new-list = නව ලැයිස්තුවක් සාදන්න
tasks-on-this-computer = මෙම පරිගණකයේ
tasks-my-tasks = මගේ කාර්යයන්
# The line under an account in the side list whose task lists could not
# come: why, and the one click that fixes it.
tasks-account-sign-in = කාර්යයන් පෙන්වීමට නැවත පුරනය වන්න
tasks-account-signed-in = { $address } වෙත නැවත පුරනය විය. ඔබේ කාර්යයන් ලබා ගනිමින්…
tasks-account-sign-in-refused = { $provider } Katna ට ඇතුළු වීමට ඉඩ දුන්නේ නැත. නැවත උත්සාහ කර, ඔබේ කාර්යයන්ට ප්‍රවේශය ඉඩ දෙන්න.
tasks-account-refused = සේවාදායකය මුරපදය පිළිගත්තේ නැත. Yahoo, iCloud, Zoho සහ වෙනත් ඒවාට යෙදුම් මුරපදයක් අවශ්‍යයි.
tasks-account-change-password = මුරපදය වෙනස් කරන්න
tasks-account-change-password-tooltip = සැකසීම් > ගිණුම් විවෘත කරන්න
tasks-account-not-enabled = Katna සඳහා කාර්ය ප්‍රවේශය තවම සක්‍රිය කර නැත.
tasks-account-failed = කාර්ය ලැයිස්තු කියවිය නොහැකි විය.
# $reason is the server's own words, in English.
tasks-account-error = කාර්ය ලැයිස්තු කියවිය නොහැකි විය: { $reason }
tasks-account-none = කාර්ය ලැයිස්තු හමු නොවීය
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
tasks-account-none-why = කාර්ය ලැයිස්තු හමු නොවීය: { $reason }
# A Gmail or Outlook account added with a password: its tasks need the
# provider's sign-in.
tasks-account-use-sign-in = { $provider } කාර්යයන් පෙන්වන්නේ { $provider } සමඟ පුරනය වූ Katna ට පමණි.
tasks-account-sign-in-with = { $provider } සමඟ පුරනය වන්න
tasks-account-looking = කාර්ය ලැයිස්තු සොයමින්…
tasks-account-try-again = නැවත උත්සාහ කරන්න
tasks-account-try-again-tooltip = මෙම ගිණුමේ කාර්යයන් දැන් නැවත පරීක්ෂා කරන්න
tasks-account-fixing = ඒ මත වැඩ කරමින්…
tasks-list-name-placeholder = ලැයිස්තුවේ නම

## Lists and tasks

tasks-loading = ඔබේ කාර්යයන් කියවමින්…
tasks-no-lists = ඔබේ කාර්ය ලැයිස්තු මෙහි පෙනෙනු ඇත.
tasks-search = කාර්යයන් සොයන්න
tasks-search-none = ඔබේ සෙවුමට ගැළපෙන කාර්යයන් නැත.
tasks-add = කාර්යයක් එක් කරන්න
tasks-title-placeholder = මාතෘකාව
tasks-add-step = උප කාර්යයක් එක් කරන්න
tasks-empty = තවම කාර්යයන් නැත. ඉහළින් එකක් එක් කරන්න.
tasks-starred-empty = මෙහි බැලීමට කාර්යයකට තරුවක් යොදන්න.
tasks-today-empty = අදට කිසිවක් නැත.
tasks-today-date = { $weekday }, { $day }
tasks-overdue = කල් ඉකුත් වූ
tasks-completed = { $count ->
    [one] සම්පූර්ණ කළ ({ $count })
   *[other] සම්පූර්ණ කළ ({ $count })
}
tasks-list-options = ලැයිස්තු විකල්ප
tasks-rename-list = ලැයිස්තුව නැවත නම් කරන්න
tasks-delete-list = ලැයිස්තුව මකන්න
tasks-mark-done = සම්පූර්ණ කළ ලෙස සලකුණු කරන්න
tasks-mark-open = සම්පූර්ණ නොකළ ලෙස සලකුණු කරන්න
tasks-star = තරුවක් එක් කරන්න
tasks-unstar = තරුව ඉවත් කරන්න
tasks-edit-title = මාතෘකාව සංස්කරණය කරන්න
tasks-details = විස්තර
tasks-delete = මකන්න
tasks-move-to = { $list } වෙත ගෙන යන්න
tasks-from-mail = තැපැල්
tasks-open-mail = තැපැල් විවෘත කරන්න
tasks-from-note = සටහන
tasks-open-note = සටහන විවෘත කරන්න
tasks-note-gone = එම සටහන තවදුරටත් මෙහි නැත.
tasks-no-subject = (විෂයක් නැත)

## The details dialog

tasks-notes-placeholder = විස්තර එක් කරන්න
tasks-date = දිනය
tasks-no-date = දිනයක් නැත
tasks-time-placeholder = වේලාව එක් කරන්න
tasks-repeat = පුනරාවර්තනය
tasks-repeat-never = පුනරාවර්තනය නොවේ
tasks-repeat-daily = දිනපතා
tasks-repeat-weekly = සතිපතා
tasks-repeat-monthly = මාසිකව
tasks-repeat-yearly = වාර්ෂිකව
tasks-repeat-other = අභිරුචි
tasks-remind = මට මතක් කරන්න
tasks-remind-off = මතක් කරන්න එපා
tasks-remind-on-time = එම වේලාවේදී
tasks-remind-morning = එදින, { $time }
tasks-remind-hour-before = පැයකට පෙර
tasks-remind-day-before = දවසකට පෙර
tasks-cancel = අවලංගු කරන්න
tasks-save = සුරකින්න
tasks-not-a-time = “{ $text }” වේලාවක් නොවේ, උදාහරණයක් ලෙස { $example }.

## Due days

tasks-due-today = අද
tasks-due-tomorrow = හෙට
tasks-due-yesterday = ඊයේ
tasks-due-at = { $day }, { $time }

## Notes at the bottom

tasks-toast-done = කාර්යය සම්පූර්ණ කළා
tasks-toast-next = අවසන්. ඊළඟ එක { $date } දින
tasks-toast-deleted = කාර්යය මකා දමන ලදී
tasks-toast-added = { $count ->
    [one] කාර්යයන් වෙත එක් කරන ලදී
   *[other] කාර්යයන් { $count }ක් එක් කරන ලදී
}
tasks-mail-gone = එම තැපැල් තවදුරටත් මෙහි නැත.
tasks-toast-list-deleted = ලැයිස්තුව මකා දමන ලදී
tasks-toast-moved = { $list } වෙත ගෙන යන ලදී
tasks-toast-rescheduled = කාර්යය නැවත සැලසුම් කරන ලදී
