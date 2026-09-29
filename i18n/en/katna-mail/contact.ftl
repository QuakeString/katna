# Katna Mail, English: the contact panel beside an open conversation.
# Guide: i18n/README.md. Keep ids stable; change the text freely.
# In plural forms, write { $count } rather than the digit, so languages
# with their own digits show them.

## Reading pane toolbar

# Shows the panel about the sender beside the conversation.
contact-panel-show = Show contact details
contact-panel-hide = Hide contact details

## The panel

# Round buttons under the name: write to them, and find the mail with
# them (from them or to them). The call button uses contact-call.
contact-email = Send email
contact-search = Search mail with them
# Saves the person on the panel to the address book in one click.
contact-add-to-contacts = Add to contacts
# Opens the saved person on the Contacts page.
contact-open-contact = Open contact

# Mail exchanged with the person, over all accounts.
contact-messages = { $count ->
    [one] { $count } message
   *[other] { $count } messages
}
# Under the count: how many they sent, and how many the user sent them.
contact-from-to = { $from } from them, { $to } from you
# The date of the first and of the latest mail with them.
contact-first = First
contact-latest = Latest
# A click on their phone number (from their signature) calls it, through
# the phone app or KDE Connect; the icon beside it copies it.
contact-call = Call
contact-copy-number = Copy number
contact-number-copied = Number copied
# Their time now, from the time zone their mail is dated in: "9:41 PM
# their time (UTC+5:30)".
contact-local-time = { $time } their time ({ $offset })
# Shown for mail only between the user's own addresses, instead of the
# mail exchanged with them.
contact-own-account = This is one of your accounts.
# The panel when the open conversation names no one.
contact-nobody = No one to show for this conversation
contact-conversations = Recent conversations
# Under the first few recent conversations: show all of them, and fold
# them back.
contact-more = More
contact-less = Less
contact-files = Files
# Open tasks made from mail with them.
contact-tasks = Tasks
# Their next meetings (organizer or guest), soonest first.
contact-meetings = Upcoming meetings
# The other people of the open conversation; a click shows one of them.
contact-people = In this conversation
# How the panel's details were found, at its foot.
contact-local-only = From your mail on this computer only
