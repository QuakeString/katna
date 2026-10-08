# Katna Mail, Odia (ଓଡ଼ିଆ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = ମେଲ ସର୍ଭର

problems-signed-out = { $provider } Katnaକୁ { $address }ରୁ ସାଇନ ଆଉଟ କରିଦେଲା। ମେଲ ସିଙ୍କ ହେବା ବନ୍ଦ ହୋଇଗଲା।
problems-password-refused = { $provider } { $address } ପାଇଁ ପାସୱାର୍ଡ ପ୍ରତ୍ୟାଖ୍ୟାନ କଲା। ଏହା ବଦଳିଥାଇପାରେ।
problems-no-answer = { $provider } { $address } ପାଇଁ ଉତ୍ତର ଦେଉନାହିଁ। Katna ଚେଷ୍ଟା ଜାରି ରଖିଛି।
problems-offline = ଆପଣ ଅଫଲାଇନ ଅଛନ୍ତି। ଆପଣଙ୍କ ମେଲ ଏଠାରେ ଅଛି, ଏବଂ ଆପଣ ପଠାଉଥିବା ମେଲ ଆପଣ ଫେରିବା ପର୍ଯ୍ୟନ୍ତ ଅପେକ୍ଷା କରେ।
problems-accounts-need-you = { $count ->
    [one] 1ଟି ଆକାଉଣ୍ଟକୁ ଆପଣଙ୍କ ଦରକାର
   *[other] { $count }ଟି ଆକାଉଣ୍ଟକୁ ଆପଣଙ୍କ ଦରକାର
}
problems-show = ଦେଖାନ୍ତୁ
problems-later = ପରେ
problems-new-password = ନୂଆ ପାସୱାର୍ଡ
problems-try-again = ପୁଣି ଚେଷ୍ଟା କରନ୍ତୁ

## The New password card

problems-password-title = ନୂଆ ପାସୱାର୍ଡ
problems-password-detail = { $provider } { $address } ପାଇଁ ସେଭ ହୋଇଥିବା ପାସୱାର୍ଡ ପ୍ରତ୍ୟାଖ୍ୟାନ କଲା। ନୂଆଟି ଟାଇପ କରନ୍ତୁ; Katna ରଖିବା ପୂର୍ବରୁ ଏହାକୁ ଯାଞ୍ଚ କରେ।
problems-password-placeholder = ପାସୱାର୍ଡ
problems-password-show = ପାସୱାର୍ଡ ଦେଖାନ୍ତୁ
problems-password-hide = ପାସୱାର୍ଡ ଲୁଚାନ୍ତୁ
problems-password-cancel = ବାତିଲ କରନ୍ତୁ
problems-password-save = ସେଭ କରନ୍ତୁ
problems-password-checking = ଯାଞ୍ଚ କରାଯାଉଛି…
problems-password-refused-again = { $provider } ଏହି ପାସୱାର୍ଡକୁ ମଧ୍ୟ ପ୍ରତ୍ୟାଖ୍ୟାନ କଲା। ଯାଞ୍ଚ କରି ପୁଣି ଚେଷ୍ଟା କରନ୍ତୁ।
problems-password-saved = { $address } ପାଇଁ ପାସୱାର୍ଡ ସେଭ ହେଲା। ଆପଣଙ୍କ ମେଲ ଅଣାଯାଉଛି…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = { $address }ର ମେଲ ସର୍ଭର { $count ->
    [one] ଏକ ମେସେଜ ଘୁଞ୍ଚାଇବା ଗ୍ରହଣ କଲା ନାହିଁ, ତେଣୁ ଏହା ଯେଉଁଠି ଥିଲା ସେଠାକୁ ଫେରିଛି।
   *[other] { $count }ଟି ମେସେଜ ଘୁଞ୍ଚାଇବା ଗ୍ରହଣ କଲା ନାହିଁ, ତେଣୁ ସେଗୁଡ଼ିକ ଯେଉଁଠି ଥିଲା ସେଠାକୁ ଫେରିଛି।
}
problems-refused-flags = { $address }ର ମେଲ ସର୍ଭର { $count ->
    [one] ଏକ ମେସେଜ ଚିହ୍ନିତ କରିବା (ପଢ଼ାଯାଇଛି, ତାରାଙ୍କିତ…) ଗ୍ରହଣ କଲା ନାହିଁ, ତେଣୁ ଏହା ପୂର୍ବ ପରି ହୋଇଯାଇଛି।
   *[other] { $count }ଟି ମେସେଜ ଚିହ୍ନିତ କରିବା (ପଢ଼ାଯାଇଛି, ତାରାଙ୍କିତ…) ଗ୍ରହଣ କଲା ନାହିଁ, ତେଣୁ ସେଗୁଡ଼ିକ ପୂର୍ବ ପରି ହୋଇଯାଇଛି।
}
problems-refused-label = { $address }ର ମେଲ ସର୍ଭର { $count ->
    [one] ଏକ ମେସେଜର ଲେବଲ ବଦଳାଇବା ଗ୍ରହଣ କଲା ନାହିଁ, ତେଣୁ ଏହା ପୂର୍ବ ପରି ହୋଇଯାଇଛି।
   *[other] { $count }ଟି ମେସେଜର ଲେବଲ ବଦଳାଇବା ଗ୍ରହଣ କଲା ନାହିଁ, ତେଣୁ ସେଗୁଡ଼ିକ ପୂର୍ବ ପରି ହୋଇଯାଇଛି।
}
problems-refused-delete = { $address }ର ମେଲ ସର୍ଭର { $count ->
    [one] ଏକ ମେସେଜ ଡିଲିଟ କରିବା ଗ୍ରହଣ କଲା ନାହିଁ, ତେଣୁ ଏହା ଫେରିଆସିଛି।
   *[other] { $count }ଟି ମେସେଜ ଡିଲିଟ କରିବା ଗ୍ରହଣ କଲା ନାହିଁ, ତେଣୁ ସେଗୁଡ଼ିକ ଫେରିଆସିଛି।
}
problems-refused-other = { $address }ର ମେଲ ସର୍ଭର { $count ->
    [one] ଏକ ପରିବର୍ତ୍ତନ ଗ୍ରହଣ କଲା ନାହିଁ, ତେଣୁ Katna ଏହାକୁ ପୂର୍ବ ପରି କରିଦେଲା।
   *[other] { $count }ଟି ପରିବର୍ତ୍ତନ ଗ୍ରହଣ କଲା ନାହିଁ, ତେଣୁ Katna ସେଗୁଡ଼ିକୁ ପୂର୍ବ ପରି କରିଦେଲା।
}
problems-details = ବିବରଣୀ

## Katna's background service (katna-daemon) isn't running

service-starting = Katnaର ପୃଷ୍ଠଭୂମି ସେବା ଆରମ୍ଭ ହେଉଛି…
service-failed = Katnaର ପୃଷ୍ଠଭୂମି ସେବା ଆରମ୍ଭ ହେଉନାହିଁ, ତେଣୁ ମେଲ ସିଙ୍କ ହେଉନାହିଁ।
service-start-again = ପୁଣି ଆରମ୍ଭ କରନ୍ତୁ
service-started-again = Katnaର ପୃଷ୍ଠଭୂମି ସେବା ବନ୍ଦ ହୋଇଯାଇଥିଲା ଏବଂ ପୁଣି ଆରମ୍ଭ କରାଗଲା।
service-details-title = ସେବା କାହିଁକି ଆରମ୍ଭ ହେଉନାହିଁ
service-details-body = ଏହାକୁ କପି କରି ଆପଣଙ୍କ ରିପୋର୍ଟ ସହ ପଠାନ୍ତୁ। ଏଥିରେ କୌଣସି ମେଲ କିମ୍ବା ପାସୱାର୍ଡ ନାହିଁ।
service-details-copy = କପି କରନ୍ତୁ
service-details-close = ବନ୍ଦ କରନ୍ତୁ
service-not-running = Katnaର ବ୍ୟାକଗ୍ରାଉଣ୍ଡ ସେବା ଚାଲୁନାହିଁ।
service-no-answer = Katnaର ବ୍ୟାକଗ୍ରାଉଣ୍ଡ ସେବା ଉତ୍ତର ଦେଲା ନାହିଁ: { $error }
service-no-session = କୌଣସି D-Bus ସେସନ ନାହିଁ: { $error }

## Safe mode: an update left Katna's background service unable to start

safe-line = ଅପଡେଟରେ ଏକ ସମସ୍ୟା ପରେ Katna ସୁରକ୍ଷିତ ମୋଡରେ ଅଛି, ତେଣୁ ମେଲ ସିଙ୍କ ହେଉନାହିଁ।
safe-try-again = ପୁଣି ଚେଷ୍ଟା କରନ୍ତୁ
safe-restore = ପୁନଃସ୍ଥାପନ କରନ୍ତୁ
safe-restoring = { $when }ରୁ ଆପଣଙ୍କ ଡାଟା ପୁନଃସ୍ଥାପନ କରାଯାଉଛି…
safe-restored = { $when }ରୁ ଆପଣଙ୍କ ଡାଟା ପୁନଃସ୍ଥାପନ ହେଲା। ପୂର୍ବରୁ ଯାହା ଥିଲା ତାହା ଏକ ଫୋଲ୍ଡରରେ ରଖାଯାଇଛି।
safe-show-folder = ଫୋଲ୍ଡର ଦେଖାନ୍ତୁ
safe-restore-failed = ଆପଣଙ୍କ ଡାଟା ପୁନଃସ୍ଥାପନ କରିହେଲା ନାହିଁ: { $error }
safe-restore-title = ଅପଡେଟ ପୂର୍ବରୁ ଥିବା ଆପଣଙ୍କ ଡାଟା ପୁନଃସ୍ଥାପନ କରିବେ?
safe-restore-body = ଆପଣ ବାଛିଥିବା କପିକୁ Katna ଫେରିଯାଏ। ତା' ପରେ ଆସିଥିବା ମେଲ ଆପଣଙ୍କ ଆକାଉଣ୍ଟରୁ ପୁଣି ଡାଉନଲୋଡ ହୁଏ।
safe-restore-none = ଏପର୍ଯ୍ୟନ୍ତ କୌଣସି କପି ନାହିଁ। ପ୍ରତ୍ୟେକ ଅପଡେଟ ଆପଣଙ୍କ ଡାଟା ବଦଳାଇବା ପୂର୍ବରୁ Katna ଗୋଟିଏ କପି ତିଆରି କରେ।
safe-restore-keep = ବର୍ତ୍ତମାନ ଯାହା ଅଛି, ନପଠାଯାଇଥିବା ମେଲ, ଡ୍ରାଫ୍ଟ ଓ ସିଙ୍କ ହୋଇନଥିବା ପରିବର୍ତ୍ତନ ସହିତ, ପ୍ରଥମେ ଏକ ଫୋଲ୍ଡରରେ ରଖାଯାଏ, ତେଣୁ କିଛି ହଜେ ନାହିଁ।
safe-restore-cancel = ବାତିଲ କରନ୍ତୁ
safe-restore-mail = ମେଲ
safe-restore-pim = ଆକାଉଣ୍ଟ ଓ ଯୋଗାଯୋଗ
safe-restore-blobs = ସଂଲଗ୍ନକ
safe-report-title = ଡିବଗ ରିପୋର୍ଟ
safe-report-body = ଏହାକୁ କପି କରି ଆପଣଙ୍କ ବଗ ରିପୋର୍ଟରେ ସଂଲଗ୍ନ କରନ୍ତୁ। ଏଥିରେ କୌଣସି ମେଲ, ଠିକଣା କିମ୍ବା ପାସୱାର୍ଡ ନାହିଁ।
safe-report-restore = ପୁନଃସ୍ଥାପନ କରନ୍ତୁ…
safe-report-copied = ଡିବଗ ରିପୋର୍ଟ କପି ହେଲା
