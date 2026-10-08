# Katna Mail, Kannada (ಕನ್ನಡ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification (the desktop shows it, even with Katna Mail closed)

notify-new-emails = { $count ->
    [one] { $count } ಹೊಸ ಇಮೇಲ್
   *[other] { $count } ಹೊಸ ಇಮೇಲ್‌ಗಳು
}
notify-and-more = ಮತ್ತು ಇನ್ನೂ { $count }
notify-no-subject = (ವಿಷಯವಿಲ್ಲ)
notify-unknown-sender = ಅಜ್ಞಾತ ಕಳುಹಿಸುವವರು

## Reminders the user asked for (same buttons)

notify-snooze-back = ಸ್ನೂಜ್‌ನಿಂದ ಮರಳಿದೆ
notify-no-reply = ಇನ್ನೂ ಉತ್ತರವಿಲ್ಲ
notify-no-reply-to = “{ $subject }” ಗೆ ಯಾರೂ ಉತ್ತರಿಸಿಲ್ಲ.
notify-follow-up-sent = ಫಾಲೋ-ಅಪ್ ಕಳುಹಿಸಲಾಗಿದೆ
notify-follow-up-sent-to = “{ $subject }” ಗೆ ಯಾರೂ ಉತ್ತರಿಸಿರಲಿಲ್ಲ, ಹಾಗಾಗಿ Katna ಫಾಲೋ-ಅಪ್ ಮಾಡಿದೆ.
notify-follow-up-waiting = ಫಾಲೋ-ಅಪ್ ಕಳುಹಿಸಲಾಗಿಲ್ಲ
notify-follow-up-waiting-to = ಈ ಕಂಪ್ಯೂಟರ್ ಆಫ್ ಆಗಿದ್ದಾಗ ಇದರ ಸಮಯ ಬಂದಿತ್ತು. “{ $subject }” ನಿಮ್ಮ ಇನ್‌ಬಾಕ್ಸ್‌ಗೆ ಮರಳಿದೆ.

## Open and click tracking (only for mail sent with "Track opens and clicks")

notify-tracking-opened = { $who } ಅವರು { $subject } ತೆರೆದಿದ್ದಾರೆ
notify-tracking-clicked = { $who } ಅವರು { $subject } ನಲ್ಲಿನ ಲಿಂಕ್ ಕ್ಲಿಕ್ ಮಾಡಿದ್ದಾರೆ

## An update of Katna is downloaded and ready to install

notify-update-ready = Katna Mail ಅನ್ನು ಅಪ್‌ಡೇಟ್ ಮಾಡಬಹುದು
notify-update-ready-body = ಆವೃತ್ತಿ { $version } ಡೌನ್‌ಲೋಡ್ ಆಗಿದೆ. ಅಪ್‌ಡೇಟ್ ಅದನ್ನು ಇನ್‌ಸ್ಟಾಲ್ ಮಾಡಿ Katna Mail ಅನ್ನು ಮರುಪ್ರಾರಂಭಿಸುತ್ತದೆ.
notify-update = ಅಪ್‌ಡೇಟ್

## Something needs the user, shown once per problem

notify-signed-out = ಮತ್ತೆ ಸೈನ್ ಇನ್ ಮಾಡಿ
notify-signed-out-body = { $provider } Katna ಅನ್ನು { $address } ನಿಂದ ಸೈನ್ ಔಟ್ ಮಾಡಿದೆ. ಮೇಲ್ ಸಿಂಕ್ ಆಗುವುದು ನಿಂತಿದೆ.
notify-sign-in = ಸೈನ್ ಇನ್ ಮಾಡಿ
notify-password-refused = ಪಾಸ್‌ವರ್ಡ್ ನಿರಾಕರಿಸಲಾಗಿದೆ
notify-password-refused-body = ಮೇಲ್ ಸರ್ವರ್ { $address } ನ ಪಾಸ್‌ವರ್ಡ್ ಅನ್ನು ನಿರಾಕರಿಸಿದೆ. ಅದು ಬದಲಾಗಿರಬಹುದು.
notify-new-password = ಹೊಸ ಪಾಸ್‌ವರ್ಡ್
notify-not-sent = “{ $subject }” ಕಳುಹಿಸಲಾಗಿಲ್ಲ
notify-not-sent-no-subject = ಒಂದು ಸಂದೇಶ ಕಳುಹಿಸಲಾಗಿಲ್ಲ
notify-not-sent-body = ಇದು ಔಟ್‌ಬಾಕ್ಸ್‌ನಲ್ಲಿದೆ, ಅಲ್ಲಿ ಕಾರಣ ತಿಳಿಸಲಾಗಿದೆ.
notify-open-outbox = ಔಟ್‌ಬಾಕ್ಸ್ ತೆರೆಯಿರಿ

## Reminders of calendar events

notify-event-now = ಈಗ
notify-event-in-minutes = { $count ->
    [one] { $count } ನಿಮಿಷಗಳಲ್ಲಿ
   *[other] { $count } ನಿಮಿಷಗಳಲ್ಲಿ
}
notify-event-in-hours = { $count ->
    [one] { $count } ಗಂಟೆಗಳಲ್ಲಿ
   *[other] { $count } ಗಂಟೆಗಳಲ್ಲಿ
}
notify-event-in-days = { $count ->
    [1] ನಾಳೆ
    [one] { $count } ದಿನಗಳಲ್ಲಿ
   *[other] { $count } ದಿನಗಳಲ್ಲಿ
}
notify-event-all-day = ಇಡೀ ದಿನ
notify-event-join = ಸೇರಿ
notify-event-snooze = 5 ನಿಮಿಷ ಸ್ನೂಜ್ ಮಾಡಿ
notify-task-done = ಪೂರ್ಣಗೊಂಡಿದೆ ಎಂದು ಗುರುತಿಸಿ

## The buttons of new-mail notifications and reminders

notify-open = ತೆರೆಯಿರಿ
notify-peek = ಇಣುಕಿ ನೋಡಿ
notify-reply = ಪ್ರತ್ಯುತ್ತರಿಸಿ
notify-reply-placeholder = { $name } ಗೆ ಪ್ರತ್ಯುತ್ತರಿಸಿ…
notify-send = ಕಳುಹಿಸಿ
notify-reply-quote-header = { $date } ರಂದು, { $from } ಅವರು ಬರೆದಿದ್ದಾರೆ:
notify-reply-quote-header-no-date = { $from } ಅವರು ಬರೆದಿದ್ದಾರೆ:
notify-reply-all = ಎಲ್ಲರಿಗೂ ಪ್ರತ್ಯುತ್ತರಿಸಿ
notify-mark-read = ಓದಲಾಗಿದೆ ಎಂದು ಗುರುತಿಸಿ
notify-mark-all-read = ಎಲ್ಲವನ್ನೂ ಓದಲಾಗಿದೆ ಎಂದು ಗುರುತಿಸಿ
notify-archive = ಆರ್ಕೈವ್ ಮಾಡಿ
notify-snooze-hour = 1 ಗಂಟೆ ಸ್ನೂಜ್ ಮಾಡಿ
notify-snooze-tomorrow = ನಾಳೆ
notify-copy-code = { $code } ನಕಲಿಸಿ
notify-link-verify = { $domain } ನಲ್ಲಿ ಪರಿಶೀಲಿಸಿ
notify-link-confirm = { $domain } ನಲ್ಲಿ ದೃಢೀಕರಿಸಿ
notify-link-activate = { $domain } ನಲ್ಲಿ ಸಕ್ರಿಯಗೊಳಿಸಿ

## After Archive on a notification: a short note in the same place

notify-archived = ಆರ್ಕೈವ್ ಮಾಡಲಾಗಿದೆ
notify-archived-count = { $count ->
    [one] { $count } ಸಂದೇಶವನ್ನು ಇನ್‌ಬಾಕ್ಸ್‌ನಿಂದ ಹೊರಗೆ ಸರಿಸಲಾಗಿದೆ
   *[other] { $count } ಸಂದೇಶಗಳನ್ನು ಇನ್‌ಬಾಕ್ಸ್‌ನಿಂದ ಹೊರಗೆ ಸರಿಸಲಾಗಿದೆ
}
notify-undo = ರದ್ದುಗೊಳಿಸಿ

## After Copy on a notification: a short note, with the code under the title

notify-code-copied = ಕೋಡ್ ನಕಲಿಸಲಾಗಿದೆ
notify-code-not-copied = ಕೋಡ್ ನಕಲಿಸಲು ಸಾಧ್ಯವಾಗಲಿಲ್ಲ

## it waits for the undo time

notify-reply-sent = { $name } ಗೆ ಪ್ರತ್ಯುತ್ತರ ಕಳುಹಿಸಲಾಗಿದೆ
notify-open-in-katna = Katna ದಲ್ಲಿ ತೆರೆಯಿರಿ
