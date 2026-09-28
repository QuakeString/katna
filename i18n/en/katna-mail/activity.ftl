# Katna Mail, English: Activity, who opened mail sent with open and click
# tracking and who followed its links (the button beside the search box),
# and the Details report over a period.
# Guide: i18n/README.md. Keep ids stable; change the text freely.
# In plural forms, write { $count } rather than the digit, so languages
# with their own digits show them.

## The list under the Activity button

# Opens the report.
activity-details = Details
# Takes every open and click so far off the list (not out of the report).
activity-clear-all = Clear all
# Tooltip of the × on a line: takes that open or click off the list.
activity-remove = Remove from the list
# $who: a recipient's name or address. $subject: the message's subject.
activity-feed-opened = { $who } opened “{ $subject }”
activity-feed-clicked = { $who } clicked a link in “{ $subject }”
# Apple Mail fetches pictures for privacy whether or not the mail is read.
activity-feed-maybe = { $who } may have opened “{ $subject }”
activity-feed-empty = No opens or clicks yet. Turn on the eye when you write a message to see when it's read.
# The sent message is no longer on this computer.
activity-message-gone = That message is no longer in Sent.

## The Details report

activity-report = Activity report
# The period the report covers.
activity-range-week = Last 7 days
activity-range-month = Last 30 days
activity-range-all = All time
activity-range-custom = Custom
activity-range-from = From
activity-range-to = To
activity-range-apply = Apply

## Totals at the top

# How many tracked messages are counted.
activity-messages = Tracked messages
# Of every recipient of every tracked message, the share who opened it or
# followed a link.
activity-open-rate = Open rate
activity-click-rate = Click rate
# $percent: a whole number, already in the language's digits.
activity-percent = { $percent }%

## Opens and clicks over time

activity-by-day = Opens and clicks
# The key of the chart. $count: the total, already in the language's digits.
activity-opens = Opens: { $count }
activity-clicks = Clicks: { $count }
# Long periods have one bar a week.
activity-by-week = One bar per week

## The messages

# Heading of the list, best first.
activity-by-open-rate = Subject lines by open rate
# $opened and $clicked of $recipients recipients.
activity-opened = Opened by { $opened } of { $recipients }
activity-clicked = Link followed by { $clicked } of { $recipients }
activity-no-subject = (no subject)
activity-nothing-period = No tracked mail was sent in this period.
activity-close = Close

## Your mailbox: counted on this computer from all mail in the period

insights-heading = Your mailbox
insights-counting = Counting your mail…
insights-failed = Your mail could not be counted.
insights-sent = Sent
insights-received = Received
insights-replies = Replies
# $percent: a whole number, already in the language's digits. $replied of
# $messages messages were answered within two weeks.
insights-you-replied = You answered { $percent }% of mail from others ({ $replied } of { $messages })
insights-they-replied = Others answered { $percent }% of your mail ({ $replied } of { $messages })
# $time: how long an answer usually takes, one of the three below.
insights-median = Usually within { $time }
insights-minutes = { $count ->
    [one] a minute
   *[other] { $count } minutes
}
insights-hours = { $count ->
    [one] an hour
   *[other] { $count } hours
}
insights-days = { $count ->
    [one] a day
   *[other] { $count } days
}
insights-people = People you write with most
# How many messages went to and came from one person.
insights-person-counts = { $sent } sent · { $received } received
# A grid of weekdays and hours.
insights-hours-heading = When mail arrives
