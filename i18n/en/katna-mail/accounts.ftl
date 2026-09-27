# Katna Mail, English: Settings > Accounts.
# Guide: i18n/README.md. Keep ids stable; change the text freely.
# In plural forms, write { $count } rather than the digit, so languages
# with their own digits show them.

## Settings > Accounts

# Settings row: which accounts the folder pane lists.
accounts-folder-pane = Folder pane
accounts-folder-pane-detail = Which accounts' folders the pane on the left shows.
# Radio choice under "Folder pane"; the account card is the account's picture and name at the top of the pane.
accounts-shown-one = One account at a time; switch in the account card
accounts-shown-all = All accounts, one after another
# Settings row: the unified inbox, over the accounts in the folder pane.
accounts-unified = Unified inbox
# Switch under "Unified inbox".
accounts-unified-switch = Show the mail of all accounts together
accounts-unified-switch-detail = "All Accounts" heads the folder pane, with the inbox, sent mail and more of every account in one list. The accounts below it start folded.
# Settings row: the list of accounts.
accounts-row = Accounts
accounts-row-detail = The folder pane and the account menu list accounts in this order; the first is the default. Removing an account deletes Katna's copy of its mail on this computer. The mail stays on the server.
accounts-none = No accounts yet.
# Account type shown after the address, for mail imported from files (mbox, Maildir…).
accounts-kind-imported = Imported
# Button: the account uses the picture of the desktop's user account.
accounts-picture-reset = Use desktop picture
accounts-picture-change = Change picture
# Button: the account shows its coloured letter again instead of a picture.
accounts-picture-remove = Remove picture
# Button: changes the name the account is shown with.
accounts-rename = Rename
# Buttons beside the field for the account's name.
accounts-name-save = Save
accounts-name-cancel = Cancel
# Placeholder in the empty name field; left empty, the account takes the name its sent mail uses.
accounts-name-placeholder = Your name
# $error: the system's error, in English.
accounts-rename-failed = Could not rename the account: { $error }
# Tooltips on the arrows and the handle that change the order of accounts.
accounts-move-up = Move up
accounts-move-down = Move down
accounts-drag = Drag to change the order
# Button: removes the account from Katna.
accounts-remove = Remove
# Settings row: deletes everything Katna stores.
accounts-delete-all-row = Delete all data
accounts-delete-all-row-detail = Start over, as on a new install.
accounts-delete-all-about = Deletes every account, all stored mail, contacts and calendars, the search index, your settings and saved passwords from this computer. Nothing changes on your mail servers.
accounts-delete-all-open = Delete all Katna data

## Settings > Accounts: snackbars after deleting

# $address: the account's email address.
accounts-removed-local = { $address } was removed from Katna.
# $address: the account's email address.
accounts-removed = { $address } was removed from Katna. Its mail is still on the server.
accounts-all-deleted = All Katna data was deleted from this computer.

## Settings > Accounts: the dialog that asks before deleting

# Title. $address: the account's email address.
accounts-remove-title = Remove { $address }?
# Red button of the dialog.
accounts-remove-confirm = Remove account
accounts-removing = Removing…
# Listed under "Deleted from this computer:", for an account of mail imported from files. $folders: how many folders the account has.
accounts-remove-local-mail = { $folders ->
    [0] All mail imported into this account
    [one] All mail imported into this account in its folder
   *[other] All mail imported into this account in its { $folders } folders
}
accounts-remove-local-settings = Its Katna settings
# Listed under "Deleted from this computer:". $folders: how many folders the account has.
accounts-remove-mail = { $folders ->
    [0] All of this account's mail stored by Katna
    [one] All of this account's mail stored by Katna in its folder
   *[other] All of this account's mail stored by Katna in its { $folders } folders
}
accounts-remove-outbox = Its messages waiting in the outbox
accounts-remove-settings = Its saved password and its Katna settings
accounts-delete-all-title = Delete all Katna data?
# Red button of the dialog.
accounts-delete-all-confirm = Delete everything
accounts-deleting = Deleting…
# Listed under "Deleted from this computer:".
accounts-delete-all-accounts = Every account, and all mail and attachments stored by Katna
accounts-delete-all-contacts = Contacts, calendars and the search index
accounts-delete-all-settings = All settings, signatures and keyboard shortcuts
accounts-delete-all-passwords = Every saved password
# Heading of the red list of what goes.
accounts-deleted-heading = Deleted from this computer:
accounts-cannot-undo = This cannot be undone.
accounts-server-delete-all = Nothing changes on your mail servers: your mail stays there, and adding an account again downloads it again. Mail imported from files is only in Katna; the files are not touched.
accounts-server-local = This mail was imported from files, so Katna has the only copy. The files it came from are not touched; import them again to get it back.
accounts-server-remove = Nothing changes on the mail server: your mail stays there, and adding the account again downloads it again.
# The word to type before everything is deleted; one lowercase word that is easy to type.
accounts-confirm-word = delete
# Placeholder of the field where the word is typed.
accounts-confirm-placeholder = Type “{ accounts-confirm-word }”
accounts-confirm-prompt = To confirm, type “{ accounts-confirm-word }”:
accounts-cancel = Cancel

## Reset cache (Settings > General), in the same dialog

reset-cache-about = Deletes the mail and attachments Katna downloaded, sender pictures and the search index, then downloads recent mail again. Accounts, settings and mail that is only on this computer stay.
reset-cache-button = Reset cache
reset-cache-title = Reset the cache?
# Heading of the list of what goes; it all downloads again.
reset-cache-deleted = Deleted, then downloaded again:
reset-cache-mail = Mail and attachments downloaded from your IMAP servers: recent mail downloads again now, older mail when you open it
reset-cache-index = The search index, which is rebuilt right away
reset-cache-pictures = Sender pictures
reset-cache-kept = Kept: your accounts, passwords and settings; stars, labels, read marks and pins; drafts, the outbox and changes not yet on the server; and mail from POP3 accounts or imported files, which may have no other copy. Nothing changes on your mail servers.
reset-cache-confirm = Reset cache
reset-cache-busy = Resetting…
reset-cache-done = The cache was reset. Recent mail is downloading again.
# $size: the disk space freed, such as “12 MB”.
reset-cache-done-freed = The cache was reset and { $size } freed. Recent mail is downloading again.
