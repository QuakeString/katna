# Katna Mail, Amharic (አማርኛ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The tray icon's menu

tray-open-inbox = _ገቢ መልዕክት ሳጥን ክፈት
tray-new-message = _አዲስ መልዕክት
tray-new-task = አዲስ _ተግባር
tray-new-note = አዲስ _ማስታወሻ
tray-preferences = _ቅንብሮች
tray-quit = _ውጣ

## The tray icon's tooltip

tray-unread = { $count ->
    [0] ያልተነበበ መልዕክት የለም
    [one] { $count } ያልተነበበ መልዕክት
   *[other] { $count } ያልተነበቡ መልዕክቶች
}
tray-password-refused = ለ{ $address } አዲስ የይለፍ ቃል ያስፈልጋል
tray-signed-out = ወደ { $address } እንደገና ይግቡ
tray-accounts-need-you = { $count } መለያዎች እርስዎን ይፈልጋሉ
tray-not-sent = { $count ->
    [one] { $count } መልዕክት አልተላከም
   *[other] { $count } መልዕክቶች አልተላኩም
}
