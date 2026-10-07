# Katna Mail, Odia (ଓଡ଼ିଆ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification

notify-new-emails = { $count }ଟି ନୂଆ ଇମେଲ
notify-and-more = ଆଉ { $count }ଟି
notify-no-subject = (କୌଣସି ବିଷୟ ନାହିଁ)
notify-unknown-sender = ଅଜଣା ପ୍ରେରକ
notify-snooze-back = ସ୍ନୁଜରୁ ଫେରିଆସିଛି
notify-no-reply = ଏପର୍ଯ୍ୟନ୍ତ କୌଣସି ଉତ୍ତର ନାହିଁ
notify-no-reply-to = “{ $subject }”ର ଉତ୍ତର କେହି ଦେଇନାହାନ୍ତି।
notify-follow-up-sent = ଫଲୋ-ଅପ ପଠାଗଲା
notify-follow-up-sent-to = “{ $subject }”ର ଉତ୍ତର କେହି ଦେଇନଥିଲେ, ତେଣୁ Katna ଫଲୋ-ଅପ ପଠାଇଲା।
notify-follow-up-waiting = ଫଲୋ-ଅପ ପଠାଗଲା ନାହିଁ
notify-follow-up-waiting-to = ଏହି କମ୍ପ୍ୟୁଟର ବନ୍ଦ ଥିବା ସମୟରେ ଏହାର ସମୟ ହୋଇଥିଲା। “{ $subject }” ଆପଣଙ୍କ ଇନବକ୍ସକୁ ଫେରିଆସିଛି।
notify-tracking-opened = { $who } { $subject } ଖୋଲିଲେ
notify-tracking-clicked = { $who } { $subject }ରେ ଥିବା ଏକ ଲିଙ୍କରେ କ୍ଲିକ କଲେ
notify-update-ready = Katna Mail ଅପଡେଟ କରାଯାଇପାରିବ
notify-update-ready-body = ସଂସ୍କରଣ { $version } ଡାଉନଲୋଡ ହୋଇଗଲା। ଅପଡେଟ ଏହାକୁ ଇନଷ୍ଟଲ କରେ ଏବଂ Katna Mail କୁ ରିଷ୍ଟାର୍ଟ କରେ।
notify-update = ଅପଡେଟ

## Something needs the user, shown once per problem

notify-signed-out = ପୁଣି ସାଇନ ଇନ କରନ୍ତୁ
notify-signed-out-body = { $provider } Katnaକୁ { $address }ରୁ ସାଇନ ଆଉଟ କରିଦେଲା। ମେଲ ସିଙ୍କ ହେବା ବନ୍ଦ ହୋଇଛି।
notify-sign-in = ସାଇନ ଇନ କରନ୍ତୁ
notify-password-refused = ପାସୱାର୍ଡ ଗ୍ରହଣ ହେଲା ନାହିଁ
notify-password-refused-body = ମେଲ ସର୍ଭର { $address } ପାଇଁ ପାସୱାର୍ଡ ଗ୍ରହଣ କଲା ନାହିଁ। ଏହା ବଦଳିଥାଇପାରେ।
notify-new-password = ନୂଆ ପାସୱାର୍ଡ
notify-not-sent = “{ $subject }” ପଠାଗଲା ନାହିଁ
notify-not-sent-no-subject = ଗୋଟିଏ ମେସେଜ ପଠାଗଲା ନାହିଁ
notify-not-sent-body = ଏହା ଆଉଟବକ୍ସରେ ଅଛି, ସେଠାରେ କାରଣ ଲେଖାଅଛି।
notify-open-outbox = ଆଉଟବକ୍ସ ଖୋଲନ୍ତୁ
notify-event-now = ଏବେ
notify-event-in-minutes = { $count ->
    [one] { $count } ମିନିଟରେ
   *[other] { $count } ମିନିଟରେ
}
notify-event-in-hours = { $count ->
    [one] { $count } ଘଣ୍ଟାରେ
   *[other] { $count } ଘଣ୍ଟାରେ
}
notify-event-in-days = { $count ->
    [1] କାଲି
    [one] { $count } ଦିନରେ
   *[other] { $count } ଦିନରେ
}
notify-event-all-day = ସାରା ଦିନ
notify-event-join = ଯୋଗ ଦିଅନ୍ତୁ
notify-event-snooze = 5 ମିନିଟ୍ ସ୍ନୁଜ କରନ୍ତୁ
notify-task-done = ସମ୍ପୂର୍ଣ୍ଣ ଭାବେ ଚିହ୍ନିତ କରନ୍ତୁ

## Its buttons

notify-open = ଖୋଲନ୍ତୁ
notify-peek = ଝଲକ
notify-reply = ଉତ୍ତର ଦିଅନ୍ତୁ
notify-reply-placeholder = { $name }ଙ୍କୁ ଉତ୍ତର ଦିଅନ୍ତୁ…
notify-send = ପଠାନ୍ତୁ
notify-reply-all = ସମସ୍ତଙ୍କୁ ଉତ୍ତର ଦିଅନ୍ତୁ
notify-mark-read = ପଢ଼ାଯାଇଛି ଭାବେ ଚିହ୍ନିତ କରନ୍ତୁ
notify-mark-all-read = ସବୁକୁ ପଢ଼ାଯାଇଛି ଭାବେ ଚିହ୍ନିତ କରନ୍ତୁ
notify-archive = ଆର୍କାଇଭ କରନ୍ତୁ
notify-snooze-hour = 1 ଘଣ୍ଟା ସ୍ନୁଜ କରନ୍ତୁ
notify-snooze-tomorrow = କାଲି
notify-copy-code = { $code } କପି କରନ୍ତୁ
notify-link-verify = { $domain }ରେ ଯାଞ୍ଚ କରନ୍ତୁ
notify-link-confirm = { $domain }ରେ ନିଶ୍ଚିତ କରନ୍ତୁ
notify-link-activate = { $domain }ରେ ସକ୍ରିୟ କରନ୍ତୁ

## After Archive on a notification: a short note in the same place

notify-archived = ଆର୍କାଇଭ ହେଲା
notify-archived-count = { $count ->
    [one] { $count }ଟି ମେସେଜ ଇନବକ୍ସରୁ ଘୁଞ୍ଚାଗଲା
   *[other] { $count }ଟି ମେସେଜ ଇନବକ୍ସରୁ ଘୁଞ୍ଚାଗଲା
}
notify-undo = ପୂର୍ବବତ୍ କରନ୍ତୁ

## After Copy on a notification: a short note, with the code under the title

notify-code-copied = କୋଡ କପି ହେଲା
notify-code-not-copied = କୋଡ କପି କରିହେଲା ନାହିଁ

## After a reply typed into a notification: a note in the same place while
## it waits for the undo time

notify-reply-sent = { $name }ଙ୍କୁ ଉତ୍ତର ପଠାଗଲା
notify-open-in-katna = Katnaରେ ଖୋଲନ୍ତୁ
