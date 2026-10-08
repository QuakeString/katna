# Katna Mail, Tamil (தமிழ்).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = அஞ்சல் சர்வர்
problems-signed-out = { $provider } { $address } இலிருந்து Katna-வை வெளியேற்றியது. அஞ்சல் ஒத்திசைவு நின்றுவிட்டது.
problems-password-refused = { $provider } { $address } க்கான கடவுச்சொல்லை ஏற்கவில்லை. அது மாறியிருக்கலாம்.
problems-no-answer = { $provider } { $address } க்குப் பதிலளிக்கவில்லை. Katna தொடர்ந்து முயல்கிறது.
problems-offline = நீங்கள் ஆஃப்லைனில் உள்ளீர்கள். உங்கள் அஞ்சல் இங்கேயே உள்ளது; நீங்கள் அனுப்பும் அஞ்சல் நீங்கள் திரும்பும் வரை காத்திருக்கும்.
problems-accounts-need-you = { $count ->
    [one] { $count } கணக்குக்கு உங்கள் கவனம் தேவை
   *[other] { $count } கணக்குகளுக்கு உங்கள் கவனம் தேவை
}
problems-show = காட்டு
problems-later = பின்னர்
problems-new-password = புதிய கடவுச்சொல்
problems-try-again = மீண்டும் முயல்க

## The New password card

problems-password-title = புதிய கடவுச்சொல்
problems-password-detail = { $provider } { $address } க்காகச் சேமித்த கடவுச்சொல்லை ஏற்கவில்லை. புதியதை உள்ளிடுங்கள்; வைத்துக்கொள்ளும் முன் Katna அதைச் சரிபார்க்கும்.
problems-password-placeholder = கடவுச்சொல்
problems-password-show = கடவுச்சொல்லைக் காட்டு
problems-password-hide = கடவுச்சொல்லை மறை
problems-password-cancel = ரத்துசெய்
problems-password-save = சேமி
problems-password-checking = சரிபார்க்கிறது…
problems-password-refused-again = { $provider } இந்தக் கடவுச்சொல்லையும் ஏற்கவில்லை. சரிபார்த்து மீண்டும் முயலுங்கள்.
problems-password-saved = { $address } க்கான கடவுச்சொல் சேமிக்கப்பட்டது. உங்கள் அஞ்சலைப் பெறுகிறது…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = { $address } இன் அஞ்சல் சர்வர் { $count ->
    [one] ஒரு மெசேஜை நகர்த்துவதை ஏற்கவில்லை, எனவே அது முன்பிருந்த இடத்துக்கே திரும்பியது.
   *[other] { $count } மெசேஜ்களை நகர்த்துவதை ஏற்கவில்லை, எனவே அவை முன்பிருந்த இடத்துக்கே திரும்பின.
}
problems-refused-flags = { $address } இன் அஞ்சல் சர்வர் { $count ->
    [one] ஒரு மெசேஜைக் குறிப்பதை (படித்தது, நட்சத்திரமிட்டது…) ஏற்கவில்லை, எனவே அது முன்பிருந்தபடியே உள்ளது.
   *[other] { $count } மெசேஜ்களைக் குறிப்பதை (படித்தது, நட்சத்திரமிட்டது…) ஏற்கவில்லை, எனவே அவை முன்பிருந்தபடியே உள்ளன.
}
problems-refused-label = { $address } இன் அஞ்சல் சர்வர் { $count ->
    [one] ஒரு மெசேஜின் லேபிள்களை மாற்றுவதை ஏற்கவில்லை, எனவே அது முன்பிருந்தபடியே உள்ளது.
   *[other] { $count } மெசேஜ்களின் லேபிள்களை மாற்றுவதை ஏற்கவில்லை, எனவே அவை முன்பிருந்தபடியே உள்ளன.
}
problems-refused-delete = { $address } இன் அஞ்சல் சர்வர் { $count ->
    [one] ஒரு மெசேஜை நீக்குவதை ஏற்கவில்லை, எனவே அது திரும்ப வந்துள்ளது.
   *[other] { $count } மெசேஜ்களை நீக்குவதை ஏற்கவில்லை, எனவே அவை திரும்ப வந்துள்ளன.
}
problems-refused-other = { $address } இன் அஞ்சல் சர்வர் { $count ->
    [one] ஒரு மாற்றத்தை ஏற்கவில்லை, எனவே Katna அதை முன்பிருந்தபடியே மாற்றியது.
   *[other] { $count } மாற்றங்களை ஏற்கவில்லை, எனவே Katna அவற்றை முன்பிருந்தபடியே மாற்றியது.
}
problems-details = விவரங்கள்

## Katna's background service (katna-daemon) isn't running

service-starting = Katna-வின் பின்னணிச் சேவையைத் தொடங்குகிறது…
service-failed = Katna-வின் பின்னணிச் சேவை தொடங்கவில்லை, எனவே அஞ்சல் ஒத்திசைக்கப்படவில்லை.
service-start-again = மீண்டும் தொடங்கு
service-started-again = Katna-வின் பின்னணிச் சேவை நின்றுவிட்டது, மீண்டும் தொடங்கப்பட்டது.
service-details-title = சேவை ஏன் தொடங்கவில்லை
service-details-body = இதை நகலெடுத்து உங்கள் புகாருடன் அனுப்புங்கள். இதில் அஞ்சலோ கடவுச்சொற்களோ இல்லை.
service-details-copy = நகலெடு
service-details-close = மூடு
service-not-running = Katna பின்னணிச் சேவை இயங்கவில்லை.
service-no-answer = Katna பின்னணிச் சேவை பதிலளிக்கவில்லை: { $error }
service-no-session = D-Bus அமர்வு இல்லை: { $error }
