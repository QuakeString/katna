# Katna Mail, Amharic (አማርኛ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification

notify-new-emails = { $count ->
    [one] { $count } አዲስ ኢሜይል
   *[other] { $count } አዲስ ኢሜይሎች
}
notify-and-more = እና { $count } ተጨማሪ
notify-no-subject = (ርዕሰ ጉዳይ የለም)
notify-unknown-sender = ያልታወቀ ላኪ
notify-snooze-back = ከማሸለብ ተመልሷል
notify-no-reply = እስካሁን ምላሽ የለም
notify-no-reply-to = ለ«{ $subject }» ማንም አልመለሰም።
notify-follow-up-sent = የክትትል መልዕክቱ ተልኳል
notify-follow-up-sent-to = ለ«{ $subject }» ማንም ስላልመለሰ Katna ክትትል አድርጓል።
notify-follow-up-waiting = የክትትል መልዕክቱ አልተላከም
notify-follow-up-waiting-to = ጊዜው የደረሰው ይህ ኮምፒውተር ጠፍቶ እያለ ነበር። «{ $subject }» ወደ ገቢ መልዕክት ሳጥንዎ ተመልሷል።
notify-tracking-opened = { $who } { $subject }ን ከፈተ
notify-tracking-clicked = { $who } በ{ $subject } ውስጥ ያለ አገናኝ ጠቅ አደረገ

notify-update-ready = Katna Mail ሊዘምን ይችላል
notify-update-ready-body = ስሪት { $version } ወርዷል። ዘምን የሚለው ይጭነዋል እና Katna Mailን እንደገና ያስጀምረዋል።
notify-update = አዘምን

## Something needs the user, shown once per problem

notify-signed-out = እንደገና ይግቡ
notify-signed-out-body = { $provider } Katnaን ከ{ $address } አስወጥቷል። ደብዳቤ መመሳሰሉን አቁሟል።
notify-sign-in = ግባ
notify-password-refused = የይለፍ ቃሉ ተቀባይነት አላገኘም
notify-password-refused-body = የደብዳቤ አገልጋዩ የ{ $address }ን የይለፍ ቃል አልተቀበለም። ተቀይሮ ሊሆን ይችላል።
notify-new-password = አዲስ የይለፍ ቃል
notify-not-sent = «{ $subject }» አልተላከም
notify-not-sent-no-subject = አንድ መልዕክት አልተላከም
notify-not-sent-body = በወጪ መልዕክት ሳጥን ውስጥ ነው፣ ምክንያቱንም ይናገራል።
notify-open-outbox = የወጪ መልዕክት ሳጥን ክፈት
notify-event-now = አሁን
notify-event-in-minutes = { $count ->
    [one] በ{ $count } ደቂቃ ውስጥ
   *[other] በ{ $count } ደቂቃ ውስጥ
}
notify-event-in-hours = { $count ->
    [one] በ{ $count } ሰዓት ውስጥ
   *[other] በ{ $count } ሰዓት ውስጥ
}
notify-event-in-days = { $count ->
    [1] ነገ
    [one] በ{ $count } ቀን ውስጥ
   *[other] በ{ $count } ቀን ውስጥ
}
notify-event-all-day = ቀኑን ሙሉ
notify-event-join = ተቀላቀል
notify-event-snooze = ለ5 ደቂቃ አሸልብ
notify-task-done = እንደተጠናቀቀ ምልክት አድርግ

## Its buttons

notify-open = ክፈት
notify-peek = ቅኝት
notify-reply = መልስ
notify-reply-placeholder = ለ{ $name } መልስ…
notify-send = ላክ
notify-reply-quote-header = በ{ $date } ላይ { $from } እንዲህ ሲል ጻፈ፦
notify-reply-quote-header-no-date = { $from } እንዲህ ሲል ጻፈ፦
notify-reply-all = ለሁሉም መልስ
notify-mark-read = እንደተነበበ ምልክት አድርግ
notify-mark-all-read = ሁሉንም እንደተነበቡ ምልክት አድርግ
notify-archive = ወደ ማህደር አስቀምጥ
notify-snooze-hour = ለ1 ሰዓት አሸልብ
notify-snooze-tomorrow = ነገ
notify-copy-code = { $code }ን ቅዳ
notify-link-verify = በ{ $domain } ላይ አረጋግጥ
notify-link-confirm = በ{ $domain } ላይ አጽድቅ
notify-link-activate = በ{ $domain } ላይ አግብር

## After Archive on a notification: a short note in the same place

notify-archived = ወደ ማህደር ተቀምጧል
notify-archived-count = { $count ->
    [one] { $count } መልዕክት ከገቢ መልዕክት ሳጥን ወጥቷል
   *[other] { $count } መልዕክቶች ከገቢ መልዕክት ሳጥን ወጥተዋል
}
notify-undo = ቀልብስ

## After Copy on a notification: a short note, with the code under the title

notify-code-copied = ኮዱ ተቀድቷል
notify-code-not-copied = ኮዱን መቅዳት አልተቻለም

## After a reply typed into a notification: a note in the same place while
## it waits for the undo time

notify-reply-sent = መልስ ለ{ $name } ተልኳል
notify-open-in-katna = በKatna ውስጥ ክፈት
