# Katna Mail, English: writing help with AI in a message (the sparkle by
# selected text, the Rephrase card, and its problems) and conversation
# summaries.
# Guide: i18n/README.md. Keep ids stable; change the text freely.
# { $service } is who answers: "Katna AI", or a service such as Google
# Gemini or Mistral (a brand, not translated).

# The tooltip of the sparkle by selected text.
compose-ai-rephrase-tip = Rephrase (Ctrl+J)
compose-ai-tone-clearer = Clearer
compose-ai-tone-shorter = Shorter
compose-ai-tone-friendlier = Friendlier
compose-ai-tone-formal = Formal
compose-ai-tone-grammar = Fix grammar
compose-ai-tone-longer = Longer
# The field for the user's own instruction, such as "more apologetic".
compose-ai-custom = Tell it how…
compose-ai-more = More ways
compose-ai-replace = Replace
compose-ai-again = Try again
compose-ai-below = Add below
compose-ai-copy = Copy
compose-ai-cancel = Cancel
compose-ai-rephrase = Rephrase
compose-ai-replaced = Rephrased
compose-ai-added = Added below
compose-ai-copied = Copied
compose-ai-katna = Katna AI
# The user's own service on this computer or elsewhere, when it has no name.
compose-ai-own = Your AI service
compose-ai-trial-left = { $service } · { $days ->
    [one] 1 day free left
   *[other] { $days } days free left
}
compose-ai-encrypted = This message will be encrypted. Rephrasing sends the selected text to { $service } unencrypted. Rephrase anyway?
compose-ai-sign-in = Katna AI needs a Katna account. Sign in to use it, or use your own key.
compose-ai-pay = Your free month of Katna AI is over. It is $5 a month, or you can use your own key.
compose-ai-too-many = Too many requests for now. Try again in a little while.
compose-ai-no-key = Add your { $service } key in Settings to rephrase.
compose-ai-bad-key = { $service } did not accept your key. Check it in Settings.
compose-ai-off = Writing help with AI is off in Settings.
compose-ai-failed = { $service } could not be reached. Try again.
compose-ai-try-again = Try again
compose-ai-open-settings = Open Settings
# Writing a first draft of a reply or a forward's note, while it is empty.
compose-ai-write-reply-tip = Write reply (Ctrl+J)
compose-ai-write-note-tip = Write note (Ctrl+J)
compose-ai-rephrase-empty-tip = Type something to rephrase
compose-ai-write-reply = Write a reply
compose-ai-write-note = Write a note
compose-ai-write-from = { $count ->
    [one] from 1 mail
   *[other] from { $count } mails
}
compose-ai-write-ideas = Ideas from the conversation
compose-ai-write-own = Or say what it should say…
compose-ai-write-short = Short
compose-ai-write-longer = Longer
compose-ai-write-friendly = Friendly
compose-ai-write-formal = Formal
compose-ai-write-insert = Insert
compose-ai-write-back = Other ideas
compose-ai-written = Draft added
compose-ai-write-encrypted = This conversation is encrypted. Writing a reply sends its mails to { $service } unencrypted. Write anyway?
compose-ai-write-anyway = Write
compose-ai-write-encrypted-off = This conversation is encrypted, and Settings keeps writing help out of encrypted mail.

## Summing up a conversation: the list's right-click menu, the reading
## pane's sparkle, the chat's strip and the card each opens.

summary-summarize = Summarize
summary-hide = Hide summary
summary-close = Close
summary-fold = Fold
summary-title = Summary
summary-mails = { $count ->
    [one] 1 mail
   *[other] { $count } mails
}
# The summary covers the first { $count } of the conversation's { $total } mails.
summary-of-mails = { $count } of { $total } mails
# Beside a line of the list: the conversation's mails and people.
summary-peek-count = { $mails ->
    [one] 1 mail
   *[other] { $mails } mails
} · { $people ->
    [one] 1 person
   *[other] { $people } people
}
# A catch-up sums up only the mail that came since the user last read.
summary-catch-up = { $count ->
    [one] 1 new since you last read
   *[other] { $count } new since you last read
}
# The chat's strip when mail came after the summary.
summary-strip-newer = { $count ->
    [one] 1 new since · { $gist }
   *[other] { $count } new since · { $gist }
}
summary-add-new = { $count ->
    [one] Add 1 new
   *[other] Add { $count } new
}
summary-point-settled = Settled
summary-point-money = Money
summary-point-dates = Dates
summary-point-next = Next
summary-point-open = Open
summary-files = Files
summary-for-you = For you
# Who a point came from, in its tooltip.
summary-from-mail = { $name }, { $date }
# The user's own mail, beside a point.
summary-you = You
summary-made = { $service } · { $time }
summary-not-read = { $service } · not marked read
summary-copy = Copy
summary-copied = Summary copied
summary-again = Summarize again
summary-open = Open conversation
summary-asking = Asking { $service }…
summary-stop = Stop
summary-cancel = Cancel
summary-send = Send and summarize
summary-ask-short = Waiting for your OK
summary-encrypted = This conversation is encrypted. Summarizing sends its text to { $service } unencrypted.
summary-encrypted-off = This conversation is encrypted, and Settings keeps writing help out of encrypted mail.
summary-try-again = Try again
summary-open-settings = Open Settings
