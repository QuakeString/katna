# Katna Mail, Amharic (አማርኛ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = የደብዳቤ መለያ አክል
add-account-providers-intro = የደብዳቤ አቅራቢዎን ይምረጡ። የቀረውን Katna ያገኘዋል።
add-account-provider-other = ሌላ ደብዳቤ
add-account-provider-other-detail = ማንኛውም የIMAP ወይም POP3 መለያ
add-account-provider-google-detail = Gmail እና Google Workspace
add-account-provider-microsoft-detail = Outlook እና Microsoft 365
add-account-provider-mail = { $provider } Mail
add-account-form-title = ወደ { $provider } ይግቡ
add-account-form-title-other = የደብዳቤ መለያዎ
add-account-form-intro = Katna የይለፍ ቃልዎን በሥርዓትዎ የቁልፍ ቀለበት ውስጥ ያስቀምጣል።
add-account-looking = የ{ $address } የደብዳቤ አገልጋዮችን በመፈለግ ላይ…
add-account-address-intro = የኢሜይል አድራሻዎን ያስገቡ። Katna አገልጋዮቹን ያገኝልዎታል።
add-account-servers-title = የአገልጋይ ቅንብሮች
add-account-servers-intro = Katna የ{ $address } ደብዳቤን የሚያነብበት እና የሚልክበት።
add-account-signing-in = በመግባት ላይ…
add-account-browser-title = በአሳሽዎ ውስጥ ይቀጥሉ
add-account-browser-intro = Katna የ{ $provider } መግቢያ ገጽን በአሳሽዎ ውስጥ ከፍቷል። እዚያ ይግቡና Katna ደብዳቤዎን እንዲያነብና እንዲልክ ይፍቀዱ፣ ከዚያ ወደዚህ ይመለሱ።
add-account-browser-hint = ምንም ገጽ አልተከፈተም? የአሳሽዎን መስኮቶች ይፈትሹ፣ ወይም ተመልሰው እንደገና ይሞክሩ።
add-account-stage-browser = በአሳሽዎ ውስጥ እስኪገቡ በመጠበቅ ላይ…
add-account-stage-signing-in-at = በ{ $server } በመግባት ላይ…
add-account-help-app-password-link = የመተግበሪያ ይለፍ ቃል እንዴት እንደሚሠራ
add-account-help-turn-on-imap = { $provider } የደብዳቤ መተግበሪያዎችን የሚያስገባው በድር ደብዳቤው ቅንብሮች ውስጥ የIMAP እና POP3 መዳረሻ ሲበራ ብቻ ነው።
add-account-help-turn-on-imap-link = እንዴት እንደሚበራ

## Add a mail account: fields

add-account-field-address = የኢሜይል አድራሻ
add-account-receive-with = ደብዳቤ በዚህ ተቀበል
add-account-imap-about = IMAP ደብዳቤዎን እና አቃፊዎችዎን በአገልጋዩ ላይ ያቆያል፤ በሁሉም መሣሪያ ላይ አንድ ዓይነት ነው። ሲቻል ይህን ይምረጡ።
add-account-pop3-about = POP3 ደብዳቤዎን ወደዚህ ኮምፒውተር ያወርዳል። እዚህ የሚያነቡት ወይም የሚያንቀሳቅሱት ደብዳቤ በአገልጋዩ እና በሌሎች መሣሪያዎችዎ ላይ እንዳለ ይቆያል።
add-account-incoming = ገቢ ደብዳቤ ({ $protocol })
add-account-outgoing = ወጪ ደብዳቤ ({ $protocol })
add-account-field-server = አገልጋይ
add-account-field-port = ወደብ
add-account-security-none = የለም
add-account-security-none-warning = አልተመሰጠረም፦ የይለፍ ቃልዎ እና ደብዳቤዎ በመንገድ ላይ ሊነበቡ ይችላሉ።
add-account-field-username = የተጠቃሚ ስም
add-account-field-password = የይለፍ ቃል
add-account-show-password = የይለፍ ቃል አሳይ
add-account-app-password-hint = { $provider } እዚህ የመተግበሪያ የይለፍ ቃል ያስፈልገዋል፣ በድር ላይ የሚጠቀሙበትን አይደለም። በ{ $provider } መለያዎ የደህንነት ቅንብሮች ውስጥ አንድ ይሥሩ።
add-account-field-name = ስምዎ (አማራጭ)
add-account-name-hint = ለሚጽፉላቸው ሰዎች ይታያል።
add-account-servers-pair = { $imap } እና { $smtp }
add-account-servers-found = { $source ->
    [built-in] አገልጋዮች፦ { $servers }፣ በKatna የአቅራቢዎች ዝርዝር ውስጥ የተገኙ።
    [provider] አገልጋዮች፦ { $servers }፣ በአቅራቢዎ ቅንብሮች ውስጥ የተገኙ።
    [ispdb] አገልጋዮች፦ { $servers }፣ በThunderbird የአቅራቢዎች ዝርዝር ውስጥ የተገኙ።
    [dns] አገልጋዮች፦ { $servers }፣ በጎራዎ የDNS መዝገቦች ውስጥ የተገኙ።
   *[other] አገልጋዮች፦ { $servers }፣ በግምት የተገኙ፤ መግባት ካልተሳካ ይፈትሿቸው።
}
add-account-servers-entered = አገልጋዮች፦ { $servers }፣ እንደገቡት።
add-account-sign-in-with = በ{ $provider } ይግቡ
add-account-sign-in-instead = በምትኩ በ{ $provider } ይግቡ

## Add a mail account: buttons

add-account-servers-button = የአገልጋይ ቅንብሮች
add-account-back = ተመለስ
add-account-add = መለያ አክል
add-account-done = ተጠናቀቀ
add-account-another = ሌላ መለያ አክል
add-account-cancel = ይቅር

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] የገቢ አገልጋዩን ያስገቡ።
   *[outgoing] የወጪ አገልጋዩን ያስገቡ።
}
add-account-server-space = { $kind ->
    [incoming] የገቢ አገልጋዩ ስም ክፍተት አለው።
   *[outgoing] የወጪ አገልጋዩ ስም ክፍተት አለው።
}
add-account-port-invalid = { $kind ->
    [incoming] የገቢ ወደቡ ከ{ $min } እስከ { $max } ያለ ቁጥር መሆን አለበት።
   *[outgoing] የወጪ ወደቡ ከ{ $min } እስከ { $max } ያለ ቁጥር መሆን አለበት።
}
add-account-address-empty = የኢሜይል አድራሻ ያስገቡ።
add-account-address-invalid = እንደ { $example } ያለ የኢሜይል አድራሻ ያስገቡ።
add-account-not-found = Katna የ{ $address } አገልጋዮችን ማግኘት አልቻለም፣ ስለዚህ የተለመዱትን ስሞች ሞልቷል። ከአቅራቢዎ ጋር ያረጋግጧቸው።
add-account-password-empty = የይለፍ ቃሉን ያስገቡ።
add-account-name-is-password = ስሙ ከይለፍ ቃሉ ጋር አንድ ነው። በምትኩ ሰዎች እንዲያዩት በሚፈልጉት መንገድ ስምዎን እዚያ ይተይቡ።
add-account-app-password-refused = { $provider } የይለፍ ቃሉን አልተቀበለም። በድር ላይ የሚጠቀሙበትን ሳይሆን የመተግበሪያ የይለፍ ቃል ያስፈልገዋል።
add-account-password-refused = አገልጋዩ የይለፍ ቃሉን አልተቀበለም። ይፈትሹትና እንደገና ይሞክሩ።
add-account-sign-in-refused = { $provider } Katnaን አላስገባም። እንደገና ይሞክሩ፣ እና ደብዳቤዎን እንዲደርስበት ይፍቀዱ።
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] ይህ የKatna ቅጂ ገና ወደ Microsoft መለያዎች መግባት አይችልም።
    [Google] ይህ የKatna ቅጂ ገና ወደ Google መለያዎች መግባት አይችልም።
   *[other] ይህ አቅራቢ መግባትን የሚፈቅደው በራሱ ገጽ ላይ ብቻ ነው፤ Katna ደግሞ ለእሱ ይህን ገና ማድረግ አይችልም።
}
add-account-smtp-not-found = Katna ደብዳቤዎን የሚያነብበትን ቦታ አገኘ፣ የሚልክበትን ግን አላገኘም። የወጪ አገልጋዩን ያስገቡ።

## Add a mail account: the last step

add-account-done-title = መለያዎ ዝግጁ ነው
add-account-done-intro = Katna አሁን ደብዳቤዎን እያመጣ ነው። አዲስ ደብዳቤ ሲደርስ ይታያል።
add-account-done-sign-in = መግቢያ
add-account-done-signed-in-with = በ{ $provider }፣ በአሳሽዎ ውስጥ
add-account-done-receiving = ደብዳቤ መቀበያ
add-account-done-sending = ደብዳቤ መላኪያ
add-account-done-on-server = በአገልጋዩ ላይ ያለ ደብዳቤ
add-account-done-kept = በKatna ውስጥ እስኪሰርዙት ድረስ ይቆያል
add-account-done-pop3-hint = በአገልጋዩ ላይ ባለ ደብዳቤ ላይ የሚሆነውን በቅንብሮች > መለያዎች ውስጥ ይቀይሩ።
add-account-done-zoho-title = ተግባራት እና ቀን መቁጠሪያዎች
add-account-done-zoho-about = Zoho እነዚህን ከደብዳቤ ለይቶ ያስቀምጣል። ወደ Katna ለማምጣት አንድ ጊዜ በZoho ይግቡ።
add-account-done-linked = ተግባራት እና ቀን መቁጠሪያዎች ተገናኝተዋል

## The account menu (from the account button on the top bar)

add-account-menu-another = ሌላ መለያ አክል
app-menu = ዋና ምናሌ
app-menu-back = ተመለስ
