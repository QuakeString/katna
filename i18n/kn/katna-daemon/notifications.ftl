# Katna Mail, Kannada (ಕನ್ನಡ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification

notify-new-emails = { $count ->
    [one] { $count } ಹೊಸ ಇಮೇಲ್
   *[other] { $count } ಹೊಸ ಇಮೇಲ್‌ಗಳು
}
notify-and-more = ಮತ್ತು ಇನ್ನೂ { $count }
notify-no-subject = (ವಿಷಯವಿಲ್ಲ)
notify-unknown-sender = ಅಜ್ಞಾತ ಕಳುಹಿಸುವವರು
notify-snooze-back = ಸ್ನೂಜ್‌ನಿಂದ ಮರಳಿದೆ
notify-no-reply = ಇನ್ನೂ ಉತ್ತರವಿಲ್ಲ
notify-no-reply-to = “{ $subject }” ಗೆ ಯಾರೂ ಉತ್ತರಿಸಿಲ್ಲ.
notify-tracking-opened = { $who } ಅವರು { $subject } ತೆರೆದಿದ್ದಾರೆ
notify-tracking-clicked = { $who } ಅವರು { $subject } ನಲ್ಲಿನ ಲಿಂಕ್ ಕ್ಲಿಕ್ ಮಾಡಿದ್ದಾರೆ

## An update of Katna is downloaded and ready to install

notify-update-ready = Katna Mail ಅನ್ನು ಅಪ್‌ಡೇಟ್ ಮಾಡಬಹುದು
notify-update-ready-body = ಆವೃತ್ತಿ { $version } ಡೌನ್‌ಲೋಡ್ ಆಗಿದೆ. ಅಪ್‌ಡೇಟ್ ಅದನ್ನು ಇನ್‌ಸ್ಟಾಲ್ ಮಾಡಿ Katna Mail ಅನ್ನು ಮರುಪ್ರಾರಂಭಿಸುತ್ತದೆ.
notify-update = ಅಪ್‌ಡೇಟ್
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

## Its buttons

notify-open = ತೆರೆಯಿರಿ
notify-reply-all = ಎಲ್ಲರಿಗೂ ಪ್ರತ್ಯುತ್ತರಿಸಿ
notify-mark-read = ಓದಲಾಗಿದೆ ಎಂದು ಗುರುತಿಸಿ
notify-mark-all-read = ಎಲ್ಲವನ್ನೂ ಓದಲಾಗಿದೆ ಎಂದು ಗುರುತಿಸಿ
notify-archive = ಆರ್ಕೈವ್ ಮಾಡಿ
