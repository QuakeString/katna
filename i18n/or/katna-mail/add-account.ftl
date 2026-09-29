# Katna Mail, Odia (ଓଡ଼ିଆ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = ଏକ ମେଲ ଆକାଉଣ୍ଟ ଯୋଗ କରନ୍ତୁ
add-account-looking = { $address }ର ମେଲ ସର୍ଭର ଖୋଜାଯାଉଛି…
add-account-address-intro = ଆପଣଙ୍କ ଇମେଲ ଠିକଣା ଦିଅନ୍ତୁ। Katna ଆପଣଙ୍କ ପାଇଁ ସର୍ଭର ଖୋଜିଦିଏ।
add-account-servers-title = ସର୍ଭର ସେଟିଂସ
add-account-servers-intro = { $address } ପାଇଁ Katna କେଉଁଠାରୁ ମେଲ ପଢ଼େ ଓ ପଠାଏ।
add-account-password-title = ଆପଣଙ୍କ ପାସୱାର୍ଡ ଦିଅନ୍ତୁ
add-account-signing-in = ସାଇନ ଇନ କରାଯାଉଛି…
add-account-browser-title = ଆପଣଙ୍କ ବ୍ରାଉଜରରେ ଜାରି ରଖନ୍ତୁ
add-account-browser-intro = Katna ଆପଣଙ୍କ ବ୍ରାଉଜରରେ { $provider } ସାଇନ-ଇନ ପୃଷ୍ଠା ଖୋଲିଛି। ସେଠାରେ ସାଇନ ଇନ କରନ୍ତୁ ଏବଂ Katnaକୁ ଆପଣଙ୍କ ମେଲ ପଢ଼ିବା ଓ ପଠାଇବାକୁ ଅନୁମତି ଦିଅନ୍ତୁ, ତାପରେ ଏଠାକୁ ଫେରି ଆସନ୍ତୁ।
add-account-browser-hint = କୌଣସି ପୃଷ୍ଠା ଖୋଲିଲା ନାହିଁ? ଆପଣଙ୍କ ବ୍ରାଉଜରର ୱିଣ୍ଡୋଗୁଡ଼ିକ ଯାଞ୍ଚ କରନ୍ତୁ, କିମ୍ବା ପଛକୁ ଯାଇ ପୁଣି ଚେଷ୍ଟା କରନ୍ତୁ।

## Add a mail account: fields

add-account-field-address = ଇମେଲ ଠିକଣା
add-account-incoming = ଆସୁଥିବା ମେଲ ({ $protocol })
add-account-outgoing = ଯାଉଥିବା ମେଲ ({ $protocol })
add-account-field-server = ସର୍ଭର
add-account-field-port = ପୋର୍ଟ
add-account-security-none = କିଛି ନାହିଁ
add-account-security-none-warning = ଏନକ୍ରିପ୍ଟ ହୋଇନାହିଁ: ଆପଣଙ୍କ ପାସୱାର୍ଡ ଓ ମେଲ ବାଟରେ ପଢ଼ାଯାଇପାରେ।
add-account-field-username = ଉପଯୋଗକର୍ତ୍ତା ନାମ
add-account-field-password = ପାସୱାର୍ଡ
add-account-show-password = ପାସୱାର୍ଡ ଦେଖାନ୍ତୁ
add-account-app-password-hint = { $provider } ପାଇଁ ଏଠାରେ ଏକ ଆପ ପାସୱାର୍ଡ ଦରକାର, ଆପଣ ୱେବରେ ବ୍ୟବହାର କରୁଥିବା ପାସୱାର୍ଡ ନୁହେଁ। ଆପଣଙ୍କ { $provider } ଆକାଉଣ୍ଟର ସୁରକ୍ଷା ସେଟିଂସରେ ଗୋଟିଏ ତିଆରି କରନ୍ତୁ।
add-account-field-name = ଆପଣଙ୍କ ନାମ (ଇଚ୍ଛାଧୀନ)
add-account-name-hint = ଆପଣ ଯେଉଁମାନଙ୍କୁ ଲେଖନ୍ତି ସେମାନଙ୍କୁ ଦେଖାଯାଏ।
add-account-servers-pair = { $imap } ଓ { $smtp }
add-account-servers-found = { $source ->
    [built-in] ସର୍ଭର: { $servers }, Katnaର ପ୍ରଦାତା ତାଲିକାରେ ମିଳିଲା।
    [provider] ସର୍ଭର: { $servers }, ଆପଣଙ୍କ ପ୍ରଦାତାଙ୍କ ସେଟିଂସରେ ମିଳିଲା।
    [ispdb] ସର୍ଭର: { $servers }, Thunderbirdର ପ୍ରଦାତା ତାଲିକାରେ ମିଳିଲା।
    [dns] ସର୍ଭର: { $servers }, ଆପଣଙ୍କ ଡୋମେନର DNS ରେକର୍ଡରେ ମିଳିଲା।
   *[other] ସର୍ଭର: { $servers }, ଅନୁମାନରୁ; ସାଇନ ଇନ ବିଫଳ ହେଲେ ଯାଞ୍ଚ କରନ୍ତୁ।
}
add-account-servers-entered = ସର୍ଭର: { $servers }, ଯେପରି ଦିଆଯାଇଛି।
add-account-or = କିମ୍ବା
add-account-sign-in-with = { $provider } ସହିତ ସାଇନ ଇନ କରନ୍ତୁ
add-account-sign-in-instead = ଏହା ବଦଳରେ { $provider } ସହିତ ସାଇନ ଇନ କରନ୍ତୁ

## Add a mail account: buttons

add-account-servers-button = ସର୍ଭର ସେଟିଂସ
add-account-back = ପଛକୁ
add-account-add = ଆକାଉଣ୍ଟ ଯୋଗ କରନ୍ତୁ
add-account-next = ପରବର୍ତ୍ତୀ
add-account-cancel = ବାତିଲ କରନ୍ତୁ

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] ଆସୁଥିବା ମେଲର ସର୍ଭର ଦିଅନ୍ତୁ।
   *[outgoing] ଯାଉଥିବା ମେଲର ସର୍ଭର ଦିଅନ୍ତୁ।
}
add-account-server-space = { $kind ->
    [incoming] ଆସୁଥିବା ମେଲର ସର୍ଭର ନାମରେ ଏକ ସ୍ପେସ ଅଛି।
   *[outgoing] ଯାଉଥିବା ମେଲର ସର୍ଭର ନାମରେ ଏକ ସ୍ପେସ ଅଛି।
}
add-account-port-invalid = { $kind ->
    [incoming] ଆସୁଥିବା ମେଲର ପୋର୍ଟ { $min }ରୁ { $max } ମଧ୍ୟରେ ଏକ ସଂଖ୍ୟା ହେବା ଉଚିତ।
   *[outgoing] ଯାଉଥିବା ମେଲର ପୋର୍ଟ { $min }ରୁ { $max } ମଧ୍ୟରେ ଏକ ସଂଖ୍ୟା ହେବା ଉଚିତ।
}
add-account-address-empty = ଏକ ଇମେଲ ଠିକଣା ଦିଅନ୍ତୁ।
add-account-address-invalid = { $example } ପରି ଏକ ଇମେଲ ଠିକଣା ଦିଅନ୍ତୁ।
add-account-not-found = Katna { $address } ପାଇଁ ସର୍ଭର ଖୋଜିପାଇଲା ନାହିଁ, ତେଣୁ ସାଧାରଣ ନାମଗୁଡ଼ିକ ଭରିଦେଲା। ଆପଣଙ୍କ ପ୍ରଦାତାଙ୍କ ସହ ସେଗୁଡ଼ିକ ଯାଞ୍ଚ କରନ୍ତୁ।
add-account-password-empty = ପାସୱାର୍ଡ ଦିଅନ୍ତୁ।
add-account-name-is-password = ନାମ ଓ ପାସୱାର୍ଡ ସମାନ। ସେଠାରେ ବରଂ ଲୋକମାନେ ଯେପରି ଦେଖିବା ଉଚିତ, ସେପରି ଆପଣଙ୍କ ନାମ ଟାଇପ କରନ୍ତୁ।
add-account-added = { $address } ଯୋଗ ହେଲା। ଆପଣଙ୍କ ମେଲ ଅଣାଯାଉଛି…
add-account-app-password-refused = { $provider } ପାସୱାର୍ଡ ପ୍ରତ୍ୟାଖ୍ୟାନ କଲା। ଏକ ଆପ ପାସୱାର୍ଡ ଦରକାର, ଆପଣ ୱେବରେ ବ୍ୟବହାର କରୁଥିବା ପାସୱାର୍ଡ ନୁହେଁ।
add-account-password-refused = ସର୍ଭର ପାସୱାର୍ଡ ପ୍ରତ୍ୟାଖ୍ୟାନ କଲା। ଏହାକୁ ଯାଞ୍ଚ କରି ପୁଣି ଚେଷ୍ଟା କରନ୍ତୁ।
add-account-sign-in-refused = { $provider } Katnaକୁ ଭିତରକୁ ଆସିବାକୁ ଦେଲା ନାହିଁ। ପୁଣି ଚେଷ୍ଟା କରନ୍ତୁ, ଏବଂ ଆପଣଙ୍କ ମେଲକୁ ଆକ୍ସେସ ଦିଅନ୍ତୁ।
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] Katnaର ଏହି କପି ଏପର୍ଯ୍ୟନ୍ତ Microsoft ଆକାଉଣ୍ଟରେ ସାଇନ ଇନ କରିପାରିବ ନାହିଁ।
    [Google] Katnaର ଏହି କପି ଏପର୍ଯ୍ୟନ୍ତ Google ଆକାଉଣ୍ଟରେ ସାଇନ ଇନ କରିପାରିବ ନାହିଁ।
   *[other] ଏହି ପ୍ରଦାତା କେବଳ ନିଜ ପୃଷ୍ଠାରେ ସାଇନ ଇନ କରିବାକୁ ଦିଏ, ଯାହା Katna ଏହା ପାଇଁ ଏପର୍ଯ୍ୟନ୍ତ କରିପାରିବ ନାହିଁ।
}
add-account-signed-in = { $provider } ସହିତ ସାଇନ ଇନ ହୋଇଛି। ଆପଣଙ୍କ ମେଲ ଅଣାଯାଉଛି…

## The account menu (from the account button on the top bar)

add-account-menu-another = ଆଉ ଏକ ଆକାଉଣ୍ଟ ଯୋଗ କରନ୍ତୁ
add-account-menu-manage = ଆକାଉଣ୍ଟ ପରିଚାଳନା କରନ୍ତୁ
app-menu = ମୁଖ୍ୟ ମେନୁ
app-menu-back = ପଛକୁ
