# Katna Mail, English: the compose window.
# Guide: i18n/README.md. Keep ids stable; change the text freely.
# In plural forms, write { $count } rather than the digit, so languages
# with their own digits show them.

## Grammar checking (the right-click menu on a grammar mistake)

# A fix that puts other words in place of the marked ones.
grammar-replace = “{ $words }”
# A fix that adds words after the marked ones.
grammar-add = Add “{ $words }”
# A fix that removes the marked words.
grammar-remove = Remove “{ $words }”
grammar-ignore = Ignore

## Send checks (asked before a message goes out)

send-check-attachment-title = Did you mean to attach files?
send-check-attachment-text = You wrote about an attachment, but nothing is attached.
send-check-attach = Attach a file
send-check-subject-title = Send without a subject?
send-check-subject-text = This message has no subject.
send-check-add-subject = Add subject
send-check-send-anyway = Send anyway

## Recipients (To, Cc and Bcc)

# The hint in an empty To field.
recipients-placeholder = Recipients
# The tooltip of a recipient that is not an email address.
recipient-not-valid = Not a valid email address
# The tooltip of the arrow on a recipient that shows its address.
recipient-show-address = Show address
# Asked when Send finds a recipient that is not an email address.
recipient-bad-title = Check the address
recipient-bad-text = “{ $address }” is not a valid email address. Fix it or remove it before sending.
recipient-bad-fix = Fix it
