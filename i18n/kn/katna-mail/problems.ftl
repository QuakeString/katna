# Katna Mail, Kannada (ಕನ್ನಡ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = ಮೇಲ್ ಸರ್ವರ್
problems-signed-out = { $provider } Katna ಅನ್ನು { $address } ನಿಂದ ಸೈನ್ ಔಟ್ ಮಾಡಿದೆ. ಮೇಲ್ ಸಿಂಕ್ ಆಗುವುದು ನಿಂತಿದೆ.
problems-password-refused = { $provider } { $address } ನ ಪಾಸ್‌ವರ್ಡ್ ಅನ್ನು ನಿರಾಕರಿಸಿದೆ. ಅದು ಬದಲಾಗಿರಬಹುದು.
problems-no-answer = { $provider } { $address } ಗೆ ಪ್ರತಿಕ್ರಿಯಿಸುತ್ತಿಲ್ಲ. Katna ಪ್ರಯತ್ನಿಸುತ್ತಲೇ ಇರುತ್ತದೆ.
problems-offline = ನೀವು ಆಫ್‌ಲೈನ್ ಆಗಿದ್ದೀರಿ. ನಿಮ್ಮ ಮೇಲ್ ಇಲ್ಲೇ ಇದೆ, ಮತ್ತು ನೀವು ಕಳುಹಿಸುವ ಮೇಲ್ ನೀವು ಮರಳಿ ಬರುವವರೆಗೆ ಕಾಯುತ್ತದೆ.
problems-accounts-need-you = { $count ->
    [one] { $count } ಖಾತೆಗೆ ನಿಮ್ಮ ಗಮನ ಬೇಕು
   *[other] { $count } ಖಾತೆಗಳಿಗೆ ನಿಮ್ಮ ಗಮನ ಬೇಕು
}
problems-show = ತೋರಿಸಿ
problems-later = ನಂತರ
problems-new-password = ಹೊಸ ಪಾಸ್‌ವರ್ಡ್
problems-try-again = ಮತ್ತೆ ಪ್ರಯತ್ನಿಸಿ

## The New password card

problems-password-title = ಹೊಸ ಪಾಸ್‌ವರ್ಡ್
problems-password-detail = { $provider } { $address } ಗಾಗಿ ಉಳಿಸಿದ ಪಾಸ್‌ವರ್ಡ್ ಅನ್ನು ನಿರಾಕರಿಸಿದೆ. ಹೊಸದನ್ನು ಟೈಪ್ ಮಾಡಿ; ಉಳಿಸುವ ಮೊದಲು Katna ಅದನ್ನು ಪರಿಶೀಲಿಸುತ್ತದೆ.
problems-password-placeholder = ಪಾಸ್‌ವರ್ಡ್
problems-password-show = ಪಾಸ್‌ವರ್ಡ್ ತೋರಿಸಿ
problems-password-hide = ಪಾಸ್‌ವರ್ಡ್ ಮರೆಮಾಡಿ
problems-password-cancel = ರದ್ದುಮಾಡಿ
problems-password-save = ಉಳಿಸಿ
problems-password-checking = ಪರಿಶೀಲಿಸಲಾಗುತ್ತಿದೆ…
problems-password-refused-again = { $provider } ಈ ಪಾಸ್‌ವರ್ಡ್ ಅನ್ನೂ ನಿರಾಕರಿಸಿದೆ. ಪರಿಶೀಲಿಸಿ ಮತ್ತೆ ಪ್ರಯತ್ನಿಸಿ.
problems-password-saved = { $address } ಗಾಗಿ ಪಾಸ್‌ವರ್ಡ್ ಉಳಿಸಲಾಗಿದೆ. ನಿಮ್ಮ ಮೇಲ್ ಪಡೆಯಲಾಗುತ್ತಿದೆ…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = { $address } ನ ಮೇಲ್ ಸರ್ವರ್ { $count ->
    [one] ಒಂದು ಸಂದೇಶವನ್ನು ಸರಿಸುವುದನ್ನು ಒಪ್ಪಲಿಲ್ಲ, ಹಾಗಾಗಿ ಅದು ಮೊದಲಿದ್ದಲ್ಲಿಗೆ ಮರಳಿದೆ.
   *[other] { $count } ಸಂದೇಶಗಳನ್ನು ಸರಿಸುವುದನ್ನು ಒಪ್ಪಲಿಲ್ಲ, ಹಾಗಾಗಿ ಅವು ಮೊದಲಿದ್ದಲ್ಲಿಗೆ ಮರಳಿವೆ.
}
problems-refused-flags = { $address } ನ ಮೇಲ್ ಸರ್ವರ್ { $count ->
    [one] ಒಂದು ಸಂದೇಶವನ್ನು ಗುರುತಿಸುವುದನ್ನು (ಓದಿದ್ದು, ನಕ್ಷತ್ರ…) ಒಪ್ಪಲಿಲ್ಲ, ಹಾಗಾಗಿ ಅದು ಮೊದಲಿನಂತೆಯೇ ಇದೆ.
   *[other] { $count } ಸಂದೇಶಗಳನ್ನು ಗುರುತಿಸುವುದನ್ನು (ಓದಿದ್ದು, ನಕ್ಷತ್ರ…) ಒಪ್ಪಲಿಲ್ಲ, ಹಾಗಾಗಿ ಅವು ಮೊದಲಿನಂತೆಯೇ ಇವೆ.
}
problems-refused-label = { $address } ನ ಮೇಲ್ ಸರ್ವರ್ { $count ->
    [one] ಒಂದು ಸಂದೇಶದ ಲೇಬಲ್‌ಗಳನ್ನು ಬದಲಾಯಿಸುವುದನ್ನು ಒಪ್ಪಲಿಲ್ಲ, ಹಾಗಾಗಿ ಅದು ಮೊದಲಿನಂತೆಯೇ ಇದೆ.
   *[other] { $count } ಸಂದೇಶಗಳ ಲೇಬಲ್‌ಗಳನ್ನು ಬದಲಾಯಿಸುವುದನ್ನು ಒಪ್ಪಲಿಲ್ಲ, ಹಾಗಾಗಿ ಅವು ಮೊದಲಿನಂತೆಯೇ ಇವೆ.
}
problems-refused-delete = { $address } ನ ಮೇಲ್ ಸರ್ವರ್ { $count ->
    [one] ಒಂದು ಸಂದೇಶವನ್ನು ಅಳಿಸುವುದನ್ನು ಒಪ್ಪಲಿಲ್ಲ, ಹಾಗಾಗಿ ಅದು ಮರಳಿ ಬಂದಿದೆ.
   *[other] { $count } ಸಂದೇಶಗಳನ್ನು ಅಳಿಸುವುದನ್ನು ಒಪ್ಪಲಿಲ್ಲ, ಹಾಗಾಗಿ ಅವು ಮರಳಿ ಬಂದಿವೆ.
}
problems-refused-other = { $address } ನ ಮೇಲ್ ಸರ್ವರ್ { $count ->
    [one] ಒಂದು ಬದಲಾವಣೆಯನ್ನು ಒಪ್ಪಲಿಲ್ಲ, ಹಾಗಾಗಿ Katna ಅದನ್ನು ಮೊದಲಿನಂತೆಯೇ ಮಾಡಿದೆ.
   *[other] { $count } ಬದಲಾವಣೆಗಳನ್ನು ಒಪ್ಪಲಿಲ್ಲ, ಹಾಗಾಗಿ Katna ಅವುಗಳನ್ನು ಮೊದಲಿನಂತೆಯೇ ಮಾಡಿದೆ.
}
problems-details = ವಿವರಗಳು

## Katna's background service (katna-daemon) isn't running

service-starting = Katna ದ ಹಿನ್ನೆಲೆ ಸೇವೆಯನ್ನು ಆರಂಭಿಸಲಾಗುತ್ತಿದೆ…
service-failed = Katna ದ ಹಿನ್ನೆಲೆ ಸೇವೆ ಆರಂಭವಾಗುತ್ತಿಲ್ಲ, ಹಾಗಾಗಿ ಮೇಲ್ ಸಿಂಕ್ ಆಗುತ್ತಿಲ್ಲ.
service-start-again = ಮತ್ತೆ ಆರಂಭಿಸಿ
service-started-again = Katna ದ ಹಿನ್ನೆಲೆ ಸೇವೆ ನಿಂತಿತ್ತು ಮತ್ತು ಮತ್ತೆ ಆರಂಭಿಸಲಾಗಿದೆ.
service-details-title = ಸೇವೆ ಏಕೆ ಆರಂಭವಾಗುತ್ತಿಲ್ಲ
service-details-body = ಇದನ್ನು ನಕಲಿಸಿ ನಿಮ್ಮ ವರದಿಯೊಂದಿಗೆ ಕಳುಹಿಸಿ. ಇದರಲ್ಲಿ ಯಾವುದೇ ಮೇಲ್ ಅಥವಾ ಪಾಸ್‌ವರ್ಡ್‌ಗಳಿಲ್ಲ.
service-details-copy = ನಕಲಿಸಿ
service-details-close = ಮುಚ್ಚಿ
service-not-running = Katna ಹಿನ್ನೆಲೆ ಸೇವೆ ಚಾಲನೆಯಲ್ಲಿಲ್ಲ.
service-no-answer = Katna ಹಿನ್ನೆಲೆ ಸೇವೆ ಉತ್ತರಿಸಲಿಲ್ಲ: { $error }
service-no-session = D-Bus ಸೆಷನ್ ಇಲ್ಲ: { $error }
