# Katna, English: the names in the desktop's app menu, launcher and file
# manager, from the .desktop files in packaging/ (packaging/linux/
# localize-desktop.sh writes each language's Name[…]= lines into them when
# a package is built). Guide: i18n/README.md. Keep ids stable; change the
# text freely. One line each: no { $variables } and no line breaks.

## Katna Mail in the app menu (packaging/desktop/<mail app ID>.desktop)

# The app's name; a brand, so it normally stays as it is.
desktop-mail-name = Katna Mail
# Shown under or beside the name by some menus.
desktop-mail-generic-name = Email for all your accounts
# The tooltip in the app menu.
desktop-mail-comment = Read, search and write email from all your accounts in one place
# Words a menu search also finds Katna Mail by: a list, each word ending
# with ";". Keep the English words if people search with them too.
desktop-mail-keywords = email;mail;inbox;imap;
# Right-click on Katna Mail in the app menu, launcher or task bar.
desktop-mail-action-new-message = New Message
desktop-mail-action-inbox = Open Inbox
desktop-mail-action-calendar = Calendar
desktop-mail-action-contacts = Contacts
desktop-mail-action-preferences = Preferences

## The app notifications name (hidden from menus; some desktops show it in
## their notification settings)

desktop-notifications-name = Katna Mail
desktop-notifications-comment = Katna Mail's notifications

## KRunner (Plasma's search), in its list of search plugins

desktop-krunner-name = Katna Mail
desktop-krunner-comment = Mail and contacts from Katna Mail

## Dolphin's right-click menu on files and folders

desktop-send-files-action-send = Send with Katna Mail
