# Katna Mail, Gujarati (ગુજરાતી).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification

notify-new-emails = { $count ->
    [one] { $count } નવો ઇમેઇલ
   *[other] { $count } નવા ઇમેઇલ
}
notify-and-more = અને વધુ { $count }
notify-no-subject = (કોઈ વિષય નથી)
notify-unknown-sender = અજાણ્યા મોકલનાર
notify-snooze-back = સ્નૂઝમાંથી પાછું આવ્યું
notify-no-reply = હજી કોઈ જવાબ નથી
notify-no-reply-to = “{ $subject }”નો કોઈએ જવાબ આપ્યો નથી.
notify-tracking-opened = { $who }એ { $subject } ખોલ્યો
notify-tracking-clicked = { $who }એ { $subject }માંની એક લિંક પર ક્લિક કર્યું
notify-update-ready = Katna Mail અપડેટ કરી શકાય છે
notify-update-ready-body = આવૃત્તિ { $version } ડાઉનલોડ થઈ ગઈ છે. અપડેટ તેને ઇન્સ્ટૉલ કરે છે અને Katna Mail રીસ્ટાર્ટ કરે છે.
notify-update = અપડેટ
notify-event-now = હમણાં
notify-event-in-minutes = { $count ->
    [one] { $count } મિનિટમાં
   *[other] { $count } મિનિટમાં
}
notify-event-in-hours = { $count ->
    [one] { $count } કલાકમાં
   *[other] { $count } કલાકમાં
}
notify-event-in-days = { $count ->
    [1] આવતીકાલે
    [one] { $count } દિવસમાં
   *[other] { $count } દિવસમાં
}
notify-event-all-day = આખો દિવસ
notify-event-join = જોડાઓ
notify-event-snooze = 5 મિનિટ સ્નૂઝ કરો
notify-task-done = પૂર્ણ તરીકે ચિહ્નિત કરો

## Its buttons

notify-open = ખોલો
notify-peek = ઝલક જુઓ
notify-reply = જવાબ આપો
notify-reply-placeholder = { $name } ને જવાબ આપો…
notify-send = મોકલો
notify-reply-all = બધાને જવાબ આપો
notify-mark-read = વાંચેલા તરીકે ચિહ્નિત કરો
notify-mark-all-read = બધાને વાંચેલા તરીકે ચિહ્નિત કરો
notify-archive = આર્કાઇવ કરો

## After Archive on a notification: a short note in the same place

notify-archived = આર્કાઇવ કર્યો
notify-archived-count = { $count ->
    [one] { $count } મેસેજ ઇનબૉક્સમાંથી ખસેડ્યો
   *[other] { $count } મેસેજ ઇનબૉક્સમાંથી ખસેડ્યા
}
notify-undo = પૂર્વવત્ કરો

## After a reply typed into a notification: a note in the same place while
## it waits for the undo time

notify-reply-sent = { $name } ને જવાબ મોકલ્યો
notify-open-in-katna = Katna માં ખોલો
