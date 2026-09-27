# Katna Mail, Amharic (አማርኛ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = የደብዳቤ መለያ አክል
add-account-looking = የ{ $address } የደብዳቤ አገልጋዮችን በመፈለግ ላይ…
add-account-address-intro = የኢሜይል አድራሻዎን ያስገቡ። Katna አገልጋዮቹን ያገኝልዎታል።
add-account-servers-title = የአገልጋይ ቅንብሮች
add-account-servers-intro = Katna የ{ $address } ደብዳቤን የሚያነብበት እና የሚልክበት።
add-account-password-title = የይለፍ ቃልዎን ያስገቡ
add-account-signing-in = በመግባት ላይ…

## Add a mail account: fields

add-account-field-address = የኢሜይል አድራሻ
add-account-incoming = ገቢ ደብዳቤ ({ $protocol })
add-account-outgoing = ወጪ ደብዳቤ ({ $protocol })
add-account-field-server = አገልጋይ
add-account-field-port = ወደብ
add-account-security-none = የለም
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

## Add a mail account: buttons

add-account-servers-button = የአገልጋይ ቅንብሮች
add-account-back = ተመለስ
add-account-add = መለያ አክል
add-account-next = ቀጣይ
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
add-account-added = { $address } ታክሏል። ደብዳቤዎን በማምጣት ላይ…
add-account-app-password-refused = { $provider } የይለፍ ቃሉን አልተቀበለም። በድር ላይ የሚጠቀሙበትን ሳይሆን የመተግበሪያ የይለፍ ቃል ያስፈልገዋል።
add-account-password-refused = አገልጋዩ የይለፍ ቃሉን አልተቀበለም። ይፈትሹትና እንደገና ይሞክሩ።

## The account menu (from the account button on the top bar)

add-account-menu-another = ሌላ መለያ አክል
add-account-menu-manage = መለያዎችን አስተዳድር
