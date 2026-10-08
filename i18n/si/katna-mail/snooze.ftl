# Katna Mail, Sinhala (සිංහල).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The snooze menu (the clock on a line, or Snooze in the right-click menu)

snooze-until = කල් දමන වේලාව…
snooze-later-today = අද පසුව
snooze-tomorrow = හෙට
snooze-this-weekend = මෙම සති අන්තය
snooze-next-week = ලබන සතිය
snooze-pick = දිනය සහ වේලාව තෝරන්න
snooze-back = වේලාවන් වෙත ආපසු
snooze-type-placeholder = වේලාවක් ටයිප් කරන්න
snooze-type-hint = උදා: “tue 3pm”, “tomorrow” හෝ “in 2 hours”
snooze-type-hint-unclear = Katna ට එය වේලාවක් ලෙස කියවිය නොහැක
snooze-type-unclear = “{ $text }” Katna දන්නා වේලාවක් නොවේ

## Snooze and Remind me on mail: one menu, switched at its top (B and H)

snooze-tab = කල් දමන්න
remind-tab = මට මතක් කරන්න
snooze-says = එතෙක් එය සඟවයි
remind-says = එය තිබෙන තැනම තබා ඔබට දැනුම් දෙයි
remind-before-due = නියමිත දිනට පෙර
remind-note = සටහන (විකල්ප)
remind-note-placeholder = හිස්ව තැබුවොත්, විෂය
toast-remind-set = { $date } සඳහා සිහිකැඳවීම සකසා ඇත
remind-chat-line = සිහිකැඳවීම { $date } · { $title }
remind-done = නිම කළා
toast-remind-done = සිහිකැඳවීම නිම කළා
snooze-chat-line = { $date } දක්වා කල් දමා ඇත
snooze-chat-change = වෙනස් කරන්න

## The date and time picker

snooze-cancel = අවලංගු කරන්න
snooze-save = සුරකින්න
snooze-in-the-past = දැනට වඩා පසු වේලාවක් තෝරන්න.

## beside Send

follow-up-menu = පිළිතුරක් නොලැබුණොත් පසු විපරම් කරන්න…
follow-up-title = පිළිතුරක් නොලැබුණොත් පසු විපරම් කරන්න
follow-up-off = අක්‍රියයි
follow-up-days = { $days ->
    [one] දින { $days }
   *[other] දින { $days }
}
follow-up-weeks = { $weeks ->
    [one] සති { $weeks }
   *[other] සති { $weeks }
}
follow-up-pick = තෝරන්න…
follow-up-pick-title = මේ වන විට පිළිතුරක් නොලැබුණොත් පසු විපරම් කරන්න
follow-up-remind = මට මතක් කරන්න
follow-up-remind-note = සංවාදය ඔබේ එන ලිපිවල ඉහළට නැවත පැමිණේ
follow-up-send = මා වෙනුවෙන් පසු විපරම් ලිපියක් යවන්න
follow-up-send-note = එම පුද්ගලයන්ටම, එම සංවාදයේම
follow-up-send-encrypted = සංකේතනය කළ තැපැල් සඳහා නොවේ
follow-up-text-placeholder = ලිවිය යුතු දේ
follow-up-text-named = ආයුබෝවන් { $name }, මගේ පහත පණිවිඩය ඔබ දුටුවාදැයි විමසීමට පමණි.
follow-up-text = ආයුබෝවන්, මගේ පහත පණිවිඩය ඔබ දුටුවාදැයි විමසීමට පමණි.
follow-up-template = අච්චුවක් භාවිත කරන්න
follow-up-signature = ඔබේ අත්සන එක් කෙරේ
follow-up-again = තවමත් පිළිතුරක් නැත්නම්, මෙයින් පසු නැවත පසු විපරම් කරන්න
follow-up-note = සංවාදයේ ඕනෑම කෙනෙකු පිළිතුරු දුන් වහාම නවතී. ස්වයංක්‍රීය පිළිතුරු ගණන් නොගැනේ.
follow-up-note-send = සංවාදයේ ඕනෑම කෙනෙකු පිළිතුරු දුන් වහාම නවතී. සතියේ දිනවල { $start } සිට { $end } දක්වා යවනු ලබන අතර, කිසිදා දිනකට වඩා ප්‍රමාද නොවේ.
follow-up-cancel = අවලංගු කරන්න
follow-up-done = නිමයි
follow-up-chip-send = { $time } කින් පසු විපරම
follow-up-chip-remind = { $time } කින් සිහිකැඳවීම
follow-up-chip-send-on = පසු විපරම { $date }
follow-up-chip-remind-on = සිහිකැඳවීම { $date }

## The card over an open conversation a follow-up waits on.

follow-up-card-title = තවම පිළිතුරක් නැත
follow-up-card-title-waiting = ඔබේ පසු විපරම බලාපොරොත්තුවෙන්
follow-up-card-send = Katna ඔබේ පසු විපරම { $date } දින යවයි. කිසිවෙකු පිළිතුරු දුන් විට එය නවතී.
follow-up-card-send-twice = Katna ඔබේ පසු විපරම { $date } දින යවා, පසුව තවත් වරක් යවයි. කිසිවෙකු පිළිතුරු දුන් විට එය නවතී.
follow-up-card-remind = කිසිවෙකු පිළිතුරු නොදුන්නොත්, මෙම සංවාදය { $date } දින ඔබේ එන ලිපිවලට නැවත පැමිණේ.
follow-up-card-waiting = ඔබේ පරිගණකය අක්‍රියව තිබියදී එහි වේලාව පැමිණි නිසා, එය ප්‍රමාද වී යැවුණේ නැත. එය දැන් යවන්න, නව වේලාවක් තෝරන්න, නැතහොත් නවත්වන්න.
follow-up-card-edit = සංස්කරණය කරන්න
follow-up-card-edit-title = පසු විපරම් කරන්න
follow-up-card-send-now = දැන් යවන්න
follow-up-card-stop = නවත්වන්න
follow-up-chat-send = පසු විපරම · කිසිවෙකු පිළිතුරු නොදුන්නොත් { $date }
follow-up-chat-step = පසු විපරම { $steps } න් { $step } · කිසිවෙකු පිළිතුරු නොදුන්නොත් { $date }
follow-up-chat-waiting = පසු විපරම බලාපොරොත්තුවෙන් · ඔබේ පරිගණකය අක්‍රියව තිබියදී එහි වේලාව පැමිණියා
follow-up-chat-remind = පිළිතුරක් නොලැබුණොත් { $date } එන ලිපිවලට නැවත
toast-follow-up-sent = පසු විපරම යැව්වා
toast-follow-up-stopped = පසු විපරම නැවැත්තුවා
toast-follow-up-moved = පසු විපරම { $date } වෙත ගෙන ගියා
nudge-row = { $days ->
    [one] දින { $days }කට පෙර
   *[other] දින { $days }කට පෙර
} යැව්වා. පසු විපරම් කරන්නද?
nudge-row-tip = එහි සිටින සියල්ලන්ටම පසු විපරම් ලිපියක් ලියන්න
nudge-follow-up = පසු විපරම් කරන්න
nudge-dismiss = ඉවත ලන්න
nudge-card-title = තවම පිළිතුරක් නැත
nudge-card-text = ඔබ { $days ->
    [one] දින { $days }කට පෙර
   *[other] දින { $days }කට පෙර
} යමක් විමසුවා, කිසිවෙකු පිළිතුරු දුන්නේ නැත.
nudge-chat-line = { $days ->
    [one] දින { $days }කට පෙර
   *[other] දින { $days }කට පෙර
} යැව්වා, තවම පිළිතුරක් නැත
toast-nudge-dismissed = මතක් කිරීම ඉවත ලෑවා
