# Katna Mail, Tamil (தமிழ்).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = அஞ்சல் கணக்கைச் சேர்
add-account-providers-intro = உங்கள் அஞ்சல் வழங்குநரைத் தேர்வுசெய்யுங்கள். மற்றதை Katna கண்டறியும்.
add-account-provider-other = பிற அஞ்சல்
add-account-provider-other-detail = எந்த IMAP அல்லது POP3 கணக்கும்
add-account-provider-google-detail = Gmail மற்றும் Google Workspace
add-account-provider-microsoft-detail = Outlook மற்றும் Microsoft 365
add-account-provider-mail = { $provider } Mail
add-account-form-title = { $provider } இல் உள்நுழையுங்கள்
add-account-form-title-other = உங்கள் அஞ்சல் கணக்கு
add-account-form-intro = Katna உங்கள் கடவுச்சொல்லை உங்கள் சிஸ்டத்தின் கீரிங்கில் வைக்கும்.
add-account-looking = { $address } க்கான அஞ்சல் சர்வர்களைத் தேடுகிறது…
add-account-address-intro = உங்கள் மின்னஞ்சல் முகவரியை உள்ளிடுங்கள். Katna உங்களுக்காகச் சர்வர்களைக் கண்டறியும்.
add-account-servers-title = சர்வர் அமைப்புகள்
add-account-servers-intro = { $address } க்கான அஞ்சலை Katna எங்கே படிக்கிறது, எங்கிருந்து அனுப்புகிறது.
add-account-signing-in = உள்நுழைகிறது…
add-account-browser-title = உங்கள் உலாவியில் தொடருங்கள்
add-account-browser-intro = Katna உங்கள் உலாவியில் { $provider } உள்நுழைவுப் பக்கத்தைத் திறந்துள்ளது. அங்கே உள்நுழைந்து, உங்கள் அஞ்சலைப் படிக்கவும் அனுப்பவும் Katna-வை அனுமதியுங்கள், பிறகு இங்கே திரும்புங்கள்.
add-account-browser-hint = பக்கம் எதுவும் திறக்கவில்லையா? உங்கள் உலாவிச் சாளரங்களைப் பாருங்கள், அல்லது பின்சென்று மீண்டும் முயலுங்கள்.
add-account-stage-browser = உங்கள் பிரவுசரில் நீங்கள் உள்நுழையக் காத்திருக்கிறது…
add-account-stage-signing-in-at = { $server } இல் உள்நுழைகிறது…
add-account-help-app-password-link = ஆப்ஸ் கடவுச்சொல்லை உருவாக்குவது எப்படி
add-account-help-turn-on-imap = { $provider } இன் வெப் மெயில் அமைப்புகளில் IMAP, POP3 அணுகல் இயக்கப்பட்ட பிறகே அது அஞ்சல் ஆப்ஸை அனுமதிக்கும்.
add-account-help-turn-on-imap-link = அதை இயக்குவது எப்படி

## Add a mail account: fields

add-account-field-address = மின்னஞ்சல் முகவரி
add-account-receive-with = அஞ்சலைப் பெறும் முறை
add-account-imap-about = IMAP உங்கள் அஞ்சலையும் ஃபோல்டர்களையும் சர்வரில் வைத்திருக்கும், எல்லாச் சாதனங்களிலும் ஒரே மாதிரி. முடிந்தால் இதையே தேர்வுசெய்யுங்கள்.
add-account-pop3-about = POP3 உங்கள் அஞ்சலை இந்தக் கணினிக்குப் பதிவிறக்கும். இங்கே நீங்கள் படிக்கும் அல்லது நகர்த்தும் அஞ்சல், சர்வரிலும் உங்கள் பிற சாதனங்களிலும் அப்படியே இருக்கும்.
add-account-incoming = உள்வரும் அஞ்சல் ({ $protocol })
add-account-outgoing = வெளிச்செல்லும் அஞ்சல் ({ $protocol })
add-account-field-server = சர்வர்
add-account-field-port = போர்ட்
add-account-security-none = எதுவுமில்லை
add-account-security-none-warning = மறையாக்கம் இல்லை: உங்கள் கடவுச்சொல்லையும் அஞ்சலையும் வழியிலேயே படிக்க முடியும்.
add-account-field-username = பயனர் பெயர்
add-account-field-password = கடவுச்சொல்
add-account-show-password = கடவுச்சொல்லைக் காட்டு
add-account-app-password-hint = இங்கே { $provider } க்கு ஆப் கடவுச்சொல் தேவை, இணையத்தில் நீங்கள் பயன்படுத்துவது அல்ல. உங்கள் { $provider } கணக்கின் பாதுகாப்பு அமைப்புகளில் ஒன்றை உருவாக்குங்கள்.
add-account-field-name = உங்கள் பெயர் (விருப்பத்தேர்வு)
add-account-name-hint = நீங்கள் எழுதுபவர்களுக்குக் காட்டப்படும்.
add-account-servers-pair = { $imap }, { $smtp }
add-account-servers-found = { $source ->
    [built-in] சர்வர்கள்: { $servers }, Katna இன் வழங்குநர் பட்டியலில் கண்டறியப்பட்டன.
    [provider] சர்வர்கள்: { $servers }, உங்கள் வழங்குநரின் அமைப்புகளில் கண்டறியப்பட்டன.
    [ispdb] சர்வர்கள்: { $servers }, Thunderbird இன் வழங்குநர் பட்டியலில் கண்டறியப்பட்டன.
    [dns] சர்வர்கள்: { $servers }, உங்கள் டொமைனின் DNS பதிவுகளில் கண்டறியப்பட்டன.
   *[other] சர்வர்கள்: { $servers }, ஊகிக்கப்பட்டவை; உள்நுழைவு தோல்வியடைந்தால் அவற்றைச் சரிபாருங்கள்.
}
add-account-servers-entered = சர்வர்கள்: { $servers }, உள்ளிட்டபடி.

## Add a mail account: buttons

add-account-sign-in-with = { $provider } மூலம் உள்நுழை
add-account-sign-in-instead = பதிலாக { $provider } மூலம் உள்நுழை
add-account-servers-button = சர்வர் அமைப்புகள்
add-account-back = பின்செல்
add-account-add = கணக்கைச் சேர்
add-account-done = முடிந்தது
add-account-another = இன்னொரு கணக்கைச் சேர்
add-account-cancel = ரத்துசெய்

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] உள்வரும் சர்வரை உள்ளிடுங்கள்.
   *[outgoing] வெளிச்செல்லும் சர்வரை உள்ளிடுங்கள்.
}
add-account-server-space = { $kind ->
    [incoming] உள்வரும் சர்வரின் பெயரில் இடைவெளி உள்ளது.
   *[outgoing] வெளிச்செல்லும் சர்வரின் பெயரில் இடைவெளி உள்ளது.
}
add-account-port-invalid = { $kind ->
    [incoming] உள்வரும் போர்ட் { $min } முதல் { $max } வரையிலான எண்ணாக இருக்க வேண்டும்.
   *[outgoing] வெளிச்செல்லும் போர்ட் { $min } முதல் { $max } வரையிலான எண்ணாக இருக்க வேண்டும்.
}
add-account-address-empty = மின்னஞ்சல் முகவரியை உள்ளிடுங்கள்.
add-account-address-invalid = { $example } போன்ற மின்னஞ்சல் முகவரியை உள்ளிடுங்கள்.
add-account-not-found = { $address } க்கான சர்வர்களை Katna ஆல் கண்டறிய முடியவில்லை, எனவே வழக்கமான பெயர்களை நிரப்பியுள்ளது. உங்கள் வழங்குநரிடம் அவற்றைச் சரிபாருங்கள்.
add-account-password-empty = கடவுச்சொல்லை உள்ளிடுங்கள்.
add-account-name-is-password = பெயரும் கடவுச்சொல்லும் ஒன்றாக உள்ளன. அதற்குப் பதிலாக, மற்றவர்கள் பார்க்க வேண்டியபடி உங்கள் பெயரை அங்கே உள்ளிடுங்கள்.
add-account-app-password-refused = { $provider } கடவுச்சொல்லை ஏற்கவில்லை. அதற்கு ஆப் கடவுச்சொல் தேவை, இணையத்தில் நீங்கள் பயன்படுத்துவது அல்ல.
add-account-password-refused = சர்வர் கடவுச்சொல்லை ஏற்கவில்லை. அதைச் சரிபார்த்து மீண்டும் முயலுங்கள்.
add-account-sign-in-refused = { $provider } Katna-வை உள்ளே அனுமதிக்கவில்லை. மீண்டும் முயன்று, உங்கள் அஞ்சலை அணுக அனுமதியுங்கள்.
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] Katna-வின் இந்த நகலால் இன்னும் Microsoft கணக்குகளில் உள்நுழைய முடியாது.
    [Google] Katna-வின் இந்த நகலால் இன்னும் Google கணக்குகளில் உள்நுழைய முடியாது.
   *[other] இந்த வழங்குநர் அதன் சொந்தப் பக்கத்தில் மட்டுமே உள்நுழைய அனுமதிக்கிறது; அதற்காக Katna-வால் இன்னும் அதைச் செய்ய முடியாது.
}
add-account-smtp-not-found = உங்கள் அஞ்சலைப் படிக்கும் இடத்தை Katna கண்டறிந்தது, ஆனால் அனுப்பும் இடத்தைக் கண்டறியவில்லை. வெளிச்செல்லும் சர்வரை உள்ளிடுங்கள்.

## Add a mail account: the last step

add-account-done-title = உங்கள் கணக்கு தயார்
add-account-done-intro = Katna இப்போது உங்கள் அஞ்சலைப் பெறுகிறது. புதிய அஞ்சல் வந்தவுடன் காட்டப்படும்.
add-account-done-sign-in = உள்நுழைவு
add-account-done-signed-in-with = உங்கள் பிரவுசரில், { $provider } மூலம்
add-account-done-receiving = அஞ்சலைப் பெறுதல்
add-account-done-sending = அஞ்சலை அனுப்புதல்
add-account-done-on-server = சர்வரில் உள்ள அஞ்சல்
add-account-done-kept = Katna-வில் நீங்கள் நீக்கும் வரை வைக்கப்படும்
add-account-done-pop3-hint = சர்வரில் உள்ள அஞ்சலுக்கு என்ன ஆகும் என்பதை அமைப்புகள் > கணக்குகள் என்பதில் மாற்றலாம்.
add-account-done-zoho-title = பணிகளும் கேலெண்டர்களும்
add-account-done-zoho-about = Zoho இவற்றை அஞ்சலிலிருந்து தனியாக வைக்கிறது. இவற்றை Katna-வுக்குக் கொண்டுவர Zoho மூலம் ஒருமுறை உள்நுழையுங்கள்.
add-account-done-linked = பணிகளும் கேலெண்டர்களும் இணைக்கப்பட்டன

## The account menu (from the account button on the top bar)

add-account-menu-another = இன்னொரு கணக்கைச் சேர்
app-menu = முதன்மை மெனு
app-menu-back = பின்செல்
