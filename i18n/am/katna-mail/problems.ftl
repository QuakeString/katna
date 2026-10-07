# Katna Mail, Amharic (አማርኛ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = የደብዳቤ አገልጋዩ

problems-signed-out = { $provider } Katnaን ከ{ $address } አስወጥቷል። ደብዳቤ መመሳሰሉን አቁሟል።
problems-password-refused = { $provider } የ{ $address }ን የይለፍ ቃል አልተቀበለም። ተቀይሮ ሊሆን ይችላል።
problems-no-answer = { $provider } ለ{ $address } ምላሽ እየሰጠ አይደለም። Katna መሞከሩን ይቀጥላል።
problems-offline = ከመስመር ውጭ ነዎት። ደብዳቤዎ አሁንም እዚህ አለ፣ የሚልኩት ደብዳቤም እስኪመለሱ ይጠብቃል።
problems-accounts-need-you = { $count ->
    [one] { $count } መለያ እርስዎን ይፈልጋል
   *[other] { $count } መለያዎች እርስዎን ይፈልጋሉ
}
problems-show = አሳይ
problems-later = በኋላ
problems-new-password = አዲስ የይለፍ ቃል
problems-try-again = እንደገና ሞክር

## The New password card

problems-password-title = አዲስ የይለፍ ቃል
problems-password-detail = { $provider } ለ{ $address } የተቀመጠውን የይለፍ ቃል አልተቀበለም። አዲሱን ይተይቡ፤ Katna ከማስቀመጡ በፊት ያረጋግጠዋል።
problems-password-placeholder = የይለፍ ቃል
problems-password-show = የይለፍ ቃሉን አሳይ
problems-password-hide = የይለፍ ቃሉን ደብቅ
problems-password-cancel = ይቅር
problems-password-save = አስቀምጥ
problems-password-checking = በማረጋገጥ ላይ…
problems-password-refused-again = { $provider } ይህንንም የይለፍ ቃል አልተቀበለም። ያረጋግጡትና እንደገና ይሞክሩ።
problems-password-saved = ለ{ $address } የይለፍ ቃል ተቀምጧል። ደብዳቤዎን በማምጣት ላይ…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = የ{ $address } የደብዳቤ አገልጋይ { $count ->
    [one] አንድ መልዕክት ማንቀሳቀስን አልተቀበለም፣ ስለዚህ ወደነበረበት ተመልሷል።
   *[other] { $count } መልዕክቶችን ማንቀሳቀስን አልተቀበለም፣ ስለዚህ ወደነበሩበት ተመልሰዋል።
}
problems-refused-flags = የ{ $address } የደብዳቤ አገልጋይ { $count ->
    [one] አንድ መልዕክት ምልክት ማድረግን (የተነበበ፣ ኮከብ የተደረገበት…) አልተቀበለም፣ ስለዚህ እንደነበረ ተመልሷል።
   *[other] { $count } መልዕክቶችን ምልክት ማድረግን (የተነበበ፣ ኮከብ የተደረገበት…) አልተቀበለም፣ ስለዚህ እንደነበሩ ተመልሰዋል።
}
problems-refused-label = የ{ $address } የደብዳቤ አገልጋይ { $count ->
    [one] የአንድ መልዕክት መሰየሚያዎችን መቀየርን አልተቀበለም፣ ስለዚህ እንደነበረ ተመልሷል።
   *[other] የ{ $count } መልዕክቶች መሰየሚያዎችን መቀየርን አልተቀበለም፣ ስለዚህ እንደነበሩ ተመልሰዋል።
}
problems-refused-delete = የ{ $address } የደብዳቤ አገልጋይ { $count ->
    [one] አንድ መልዕክት መሰረዝን አልተቀበለም፣ ስለዚህ ተመልሷል።
   *[other] { $count } መልዕክቶችን መሰረዝን አልተቀበለም፣ ስለዚህ ተመልሰዋል።
}
problems-refused-other = የ{ $address } የደብዳቤ አገልጋይ { $count ->
    [one] አንድ ለውጥን አልተቀበለም፣ ስለዚህ Katna እንደነበረ መልሶታል።
   *[other] { $count } ለውጦችን አልተቀበለም፣ ስለዚህ Katna እንደነበሩ መልሷቸዋል።
}
problems-details = ዝርዝሮች

## Katna's background service (katna-daemon) isn't running

service-starting = የKatna የጀርባ አገልግሎት በመጀመር ላይ…
service-failed = የKatna የጀርባ አገልግሎት አይጀምርም፣ ስለዚህ ደብዳቤ እየተመሳሰለ አይደለም።
service-start-again = እንደገና ጀምር
service-started-again = የKatna የጀርባ አገልግሎት ቆሞ ነበር፣ እንደገና ተጀምሯል።
service-details-title = አገልግሎቱ ለምን እንደማይጀምር
service-details-body = ይህን ይቅዱ እና ከሪፖርትዎ ጋር ይላኩት። ምንም ደብዳቤ ወይም የይለፍ ቃል አልያዘም።
service-details-copy = ቅዳ
service-details-close = ዝጋ
