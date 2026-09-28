# Katna Mail, English: the contact panel beside an open conversation.
# Guide: i18n/README.md. Keep ids stable; change the text freely.
# In plural forms, write { $count } rather than the digit, so languages
# with their own digits show them.

## Reading pane toolbar

# Shows the panel about the sender beside the conversation.
contact-panel-show = Show contact details
contact-panel-hide = Hide contact details

## The panel

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
contact-conversations = Recent conversations
contact-files = Files
# The other people of the open conversation; a click shows one of them.
contact-people = In this conversation
# How the panel's details were found, at its foot.
contact-local-only = From your mail on this computer only
