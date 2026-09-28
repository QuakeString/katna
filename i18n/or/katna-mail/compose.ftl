# Katna Mail, Odia (ଓଡ଼ିଆ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Compose window: title bar

compose-new-message = ନୂଆ ମେସେଜ
compose-restore = ପୁନଃସ୍ଥାପନ କରନ୍ତୁ
compose-minimize = ଛୋଟ କରନ୍ତୁ
compose-exit-full-screen = ପୂର୍ଣ୍ଣ ସ୍କ୍ରିନରୁ ବାହାରନ୍ତୁ
compose-open-window = ନୂଆ ୱିଣ୍ଡୋରେ ଖୋଲନ୍ତୁ
compose-save-close = ସେଭ କରି ବନ୍ଦ କରନ୍ତୁ
compose-back-to-mail = ମେଲ ୱିଣ୍ଡୋକୁ ଫେରନ୍ତୁ
compose-pop-out-reply = ଉତ୍ତରକୁ ଅଲଗା ୱିଣ୍ଡୋରେ ଖୋଲନ୍ତୁ
compose-edit-recipients = ପ୍ରାପକମାନଙ୍କୁ ସମ୍ପାଦନ କରନ୍ତୁ
compose-summary-cc = Cc: { $names }
compose-summary-bcc = Bcc: { $names }
compose-show-trimmed = କଟାଯାଇଥିବା ବିଷୟବସ୍ତୁ ଦେଖାନ୍ତୁ
compose-hide-trimmed = କଟାଯାଇଥିବା ବିଷୟବସ୍ତୁ ଲୁଚାନ୍ତୁ
compose-remove-trimmed = ଉଦ୍ଧୃତ ଟେକ୍ସଟ କାଢ଼ନ୍ତୁ
compose-trimmed-removed = ଉଦ୍ଧୃତ ଟେକ୍ସଟ କଢ଼ାଗଲା

## Recipients and subject

compose-to = ପ୍ରାପକ
compose-cc = Cc
compose-bcc = Bcc
compose-from = ପ୍ରେରକ
compose-from-choose = ଅନ୍ୟ ଆକାଉଣ୍ଟରୁ ପଠାନ୍ତୁ
compose-recipients = ପ୍ରାପକମାନେ
compose-subject = ବିଷୟ

## Sending (the notes at the bottom of the window)

compose-open-elsewhere = ପ୍ରଥମେ ଖୋଲା ଥିବା ମେସେଜଟି ପଠାନ୍ତୁ କିମ୍ବା ବାତିଲ କରନ୍ତୁ।
compose-bad-address = “{ $address }” ଏକ ଇମେଲ ଠିକଣା ନୁହେଁ।
compose-no-recipients = ଅତି କମରେ ଜଣେ ପ୍ରାପକ ଯୋଗ କରନ୍ତୁ।
compose-attachments-too-large = ଆଟାଚମେଣ୍ଟଗୁଡ଼ିକ { $size }; ମେଲ ସର୍ଭରମାନେ { $limit } ପର୍ଯ୍ୟନ୍ତ ଗ୍ରହଣ କରନ୍ତି।
compose-no-account = ମେଲ ପଠାଇବା ପାଇଁ ଏକ ଆକାଉଣ୍ଟ ଯୋଗ କରନ୍ତୁ।
compose-past-time = ଭବିଷ୍ୟତର ଏକ ସମୟ ବାଛନ୍ତୁ।
compose-scheduling = ସମୟ ସ୍ଥିର କରାଯାଉଛି…
compose-sending = ପଠାଯାଉଛି…
compose-scheduled = { $when } ରେ ପଠାଇବା ପାଇଁ ସମୟ ସ୍ଥିର ହେଲା
compose-sent-archived = ପଠାଗଲା ଓ ଆର୍କାଇଭ ହେଲା
compose-sent = ମେସେଜ ପଠାଗଲା
compose-discarded = ଡ୍ରାଫ୍ଟ ବାତିଲ ହେଲା
compose-draft-saved = ଡ୍ରାଫ୍ଟ ସେଭ ହେଲା
compose-draft-failed = ଡ୍ରାଫ୍ଟ ସେଭ କରାଯାଇପାରିଲା ନାହିଁ: { $error }
compose-draft-not-opened = ଡ୍ରାଫ୍ଟ ଖୋଲାଯାଇପାରିଲା ନାହିଁ।

## Attachments

compose-picker-insert = ଭର୍ତ୍ତି କରନ୍ତୁ
compose-picker-attach = ଆଟାଚ କରନ୍ତୁ
compose-file-too-large = { $name } ବହୁତ ବଡ଼: ଏକ ମେସେଜ { $limit } ପର୍ଯ୍ୟନ୍ତ ନେଇପାରେ।
compose-attachment-size = ({ $size })
compose-remove-attachment = ଆଟାଚମେଣ୍ଟ ହଟାନ୍ତୁ
compose-attachments-total = { $count ->
    [one] { $count }ଟି ଫାଇଲ, { $size }
   *[other] { $count }ଟି ଫାଇଲ, { $size }
}
compose-drop-files = ଫାଇଲଗୁଡ଼ିକ ଏଠାରେ ଛାଡ଼ନ୍ତୁ
compose-drop-here = ଏଠାରେ ଛାଡ଼ନ୍ତୁ
compose-paste-keep-formatting = ଫର୍ମାଟିଂ ରଖନ୍ତୁ
compose-paste-table = ଟେବୁଲ
compose-paste-picture = ଛବି
compose-paste-plain-text = ସାଧା ଟେକ୍ସଟ
compose-paste-inline = ଟେକ୍ସଟ ଭିତରେ
compose-paste-attachment = ଆଟାଚମେଣ୍ଟ

## Encryption and signing (the toggles by the recipients)

compose-encrypt = ଏନକ୍ରିପ୍ଟ କରନ୍ତୁ
compose-encrypted = ଏନକ୍ରିପ୍ଟ ହୋଇଛି: କେବଳ ପ୍ରାପକମାନେ ଏହାକୁ ପଢ଼ିପାରିବେ
compose-sign = ଦସ୍ତଖତ କରନ୍ତୁ
compose-signed = ଦସ୍ତଖତ ହୋଇଛି: ଏହା ଆପଣଙ୍କଠାରୁ ଆସିଛି ବୋଲି ପ୍ରାପକମାନେ ଯାଞ୍ଚ କରିପାରିବେ
compose-track = ଖୋଲିବା ଓ କ୍ଲିକ ଟ୍ରାକ କରନ୍ତୁ
compose-tracked = ଟ୍ରାକ ହେଉଛି: ପ୍ରତ୍ୟେକ ପ୍ରାପକ ଏହାକୁ କେବେ ଖୋଲନ୍ତି କିମ୍ବା ଲିଙ୍କ ଖୋଲନ୍ତି ଆପଣ ଦେଖିପାରିବେ
compose-track-clicks = ଲିଙ୍କ କ୍ଲିକ ଟ୍ରାକ କରନ୍ତୁ (ସାଧା ଟେକ୍ସଟରେ ଖୋଲିବା ଦେଖାଯାଇପାରିବ ନାହିଁ)
compose-tracked-clicks = ଟ୍ରାକ ହେଉଛି: ପ୍ରତ୍ୟେକ ପ୍ରାପକ କେବେ ଲିଙ୍କ ଖୋଲନ୍ତି ଆପଣ ଦେଖିପାରିବେ
compose-track-sign-in = ଖୋଲିବା ଓ କ୍ଲିକ ଟ୍ରାକ କରିବାକୁ ଏକ Katna ଆକାଉଣ୍ଟରେ ସାଇନ ଇନ କରନ୍ତୁ
compose-receipt = ପଢ଼ିବା ରସିଦ ମାଗନ୍ତୁ
compose-receipt-on = ପଢ଼ିବା ରସିଦ ମଗାଯାଇଛି: ପ୍ରାପକଙ୍କ ଆପ ତାଙ୍କୁ ଗୋଟିଏ ପଠାଇବାକୁ କହିପାରେ
compose-delivery = ଡେଲିଭରି ରସିଦ ମାଗନ୍ତୁ
compose-delivery-on = ଡେଲିଭରି ରସିଦ ମଗାଯାଇଛି: ପ୍ରତ୍ୟେକ ପ୍ରାପକଙ୍କ ସର୍ଭର ମେସେଜଟି ଗ୍ରହଣ କଲେ ଆପଣଙ୍କ ମେଲ ସର୍ଭର ଆପଣଙ୍କୁ ଇମେଲ ପଠାଇବ
compose-delivery-unavailable = ଆପଣଙ୍କ ମେଲ ସର୍ଭର ଡେଲିଭରି ରସିଦ ପଠାଏ ନାହିଁ

## Spelling

spell-no-dictionary = { $language } ପାଇଁ କୌଣସି ବନାନ ଅଭିଧାନ ଇନଷ୍ଟଲ ହୋଇନାହିଁ (ଯେପରି hunspell-en_us)।
spell-dictionary-error = ବନାନ ଅଭିଧାନ: { $error }

## Grammar checking (the right-click menu on a grammar mistake)

grammar-replace = “{ $words }”
grammar-add = “{ $words }” ଯୋଗ କରନ୍ତୁ
grammar-remove = “{ $words }” ହଟାନ୍ତୁ
grammar-ignore = ଅଣଦେଖା କରନ୍ତୁ

## Send checks (asked before a message goes out)

send-check-attachment-title = ଆପଣ ଫାଇଲ ଆଟାଚ କରିବାକୁ ଚାହୁଁଥିଲେ କି?
send-check-attachment-text = ଆପଣ ଏକ ଆଟାଚମେଣ୍ଟ ବିଷୟରେ ଲେଖିଛନ୍ତି, କିନ୍ତୁ କିଛି ଆଟାଚ ହୋଇନାହିଁ।
send-check-attach = ଏକ ଫାଇଲ ଆଟାଚ କରନ୍ତୁ
send-check-subject-title = ବିଷୟ ବିନା ପଠାଇବେ କି?
send-check-subject-text = ଏହି ମେସେଜର କୌଣସି ବିଷୟ ନାହିଁ।
send-check-add-subject = ବିଷୟ ଯୋଗ କରନ୍ତୁ
send-check-send-anyway = ତଥାପି ପଠାନ୍ତୁ
recipient-not-valid = ବୈଧ ଇମେଲ ଠିକଣା ନୁହେଁ
recipient-show-address = ଠିକଣା ଦେଖାନ୍ତୁ
recipient-remove = ହଟାନ୍ତୁ
recipient-bad-title = ଠିକଣା ଯାଞ୍ଚ କରନ୍ତୁ
recipient-bad-text = “{ $address }” ଏକ ବୈଧ ଇମେଲ ଠିକଣା ନୁହେଁ। ପଠାଇବା ପୂର୍ବରୁ ଏହାକୁ ଠିକ କରନ୍ତୁ କିମ୍ବା କାଢ଼ି ଦିଅନ୍ତୁ।
recipient-bad-fix = ଠିକ କରନ୍ତୁ
