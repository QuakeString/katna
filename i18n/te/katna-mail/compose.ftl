# Katna Mail, Telugu (తెలుగు).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Compose window: title bar

compose-new-message = కొత్త మెసేజ్
compose-restore = పునరుద్ధరించండి
compose-minimize = కనిష్టీకరించండి
compose-exit-full-screen = ఫుల్ స్క్రీన్ నుండి నిష్క్రమించండి
compose-open-window = కొత్త విండోలో తెరవండి
compose-save-close = సేవ్ చేసి మూసివేయండి
compose-back-to-mail = మెయిల్ విండోకు తిరిగి వెళ్లండి
compose-pop-out-reply = రిప్లయిని విడిగా తెరవండి
compose-edit-recipients = స్వీకర్తలను ఎడిట్ చేయండి
compose-summary-cc = Cc: { $names }
compose-summary-bcc = Bcc: { $names }
compose-more-recipients = మరో { $count }
compose-show-trimmed = కత్తిరించిన కంటెంట్‌ను చూపండి
compose-hide-trimmed = కత్తిరించిన కంటెంట్‌ను దాచండి
compose-remove-trimmed = కోట్ చేసిన టెక్స్ట్‌ను తీసివేయండి
compose-trimmed-removed = కోట్ చేసిన టెక్స్ట్ తీసివేయబడింది

## Recipients and subject

compose-to = స్వీకర్త
compose-cc = Cc
compose-bcc = Bcc
compose-from = పంపినవారు
compose-from-choose = మరో ఖాతా నుండి పంపండి
compose-recipients = స్వీకర్తలు
compose-subject = సబ్జెక్ట్

## Sending (the notes at the bottom of the window)

compose-open-elsewhere = ముందుగా తెరిచి ఉన్న మెసేజ్‌ను పంపండి లేదా విస్మరించండి.
compose-bad-address = “{ $address }” ఈమెయిల్ అడ్రస్ కాదు.
compose-no-recipients = కనీసం ఒక స్వీకర్తను జోడించండి.
compose-attachments-too-large = అటాచ్‌మెంట్‌లు { $size } ఉన్నాయి; మెయిల్ సర్వర్‌లు { $limit } వరకు మాత్రమే తీసుకుంటాయి.
compose-no-account = మెయిల్ పంపడానికి ఒక ఖాతాను జోడించండి.
compose-past-time = భవిష్యత్తులోని సమయాన్ని ఎంచుకోండి.
compose-scheduling = షెడ్యూల్ చేస్తోంది…
compose-sending = పంపుతోంది…
compose-scheduled = { $when }కి పంపడానికి షెడ్యూల్ చేయబడింది
compose-sent-archived = పంపబడింది, ఆర్కైవ్ చేయబడింది
compose-sent = మెసేజ్ పంపబడింది
compose-discarded = డ్రాఫ్ట్ విస్మరించబడింది
compose-draft-saved = డ్రాఫ్ట్ సేవ్ చేయబడింది
compose-draft-failed = డ్రాఫ్ట్‌ను సేవ్ చేయడం సాధ్యం కాలేదు: { $error }
compose-draft-not-opened = డ్రాఫ్ట్‌ను తెరవడం సాధ్యం కాలేదు.

## Attachments

compose-picker-insert = చొప్పించండి
compose-picker-attach = అటాచ్ చేయండి
compose-file-too-large = { $name } చాలా పెద్దది: ఒక మెసేజ్ { $limit } వరకు మాత్రమే తీసుకెళ్లగలదు.
compose-attachment-size = ({ $size })
compose-remove-attachment = అటాచ్‌మెంట్‌ను తీసివేయండి
compose-attachments-total = { $count ->
    [one] { $count } ఫైల్, { $size }
   *[other] { $count } ఫైల్‌లు, { $size }
}
compose-drive-note = { $name } { $limit } కంటే ఎక్కువ ఉంది, కాబట్టి అది మీ Google Driveకు వెళుతుంది, మెసేజ్‌లో లింక్ ఉంటుంది.
compose-drive-tip = మీ Google Driveలో; మెసేజ్‌లో లింక్ ఉంటుంది
compose-drive-uploading = అప్‌లోడ్ అవుతోంది { $percent }%
compose-drive-allow = Driveను అనుమతించండి
compose-drive-allow-tip = పెద్ద ఫైల్‌లను మీ Driveలో ఉంచడానికి Katnaకు అనుమతించేలా Googleతో మళ్లీ సైన్ ఇన్ చేయండి
compose-drive-retry = మళ్లీ ట్రై చేయండి
compose-drive-sends-when-uploaded = { $name } అప్‌లోడ్ అయిన తర్వాత పంపబడుతుంది
compose-drive-not-uploaded = { $name } ఇంకా Google Driveలో లేదు
compose-drive-share-failed = Google Driveలో ఫైల్‌లను షేర్ చేయడం సాధ్యం కాలేదు: { $error }
compose-drive-share-title = ఫైల్‌లను అందరితో షేర్ చేయాలా?
compose-drive-share-text = { $count ->
    [one] Google ఖాతా లేని { $addresses }తో Google Drive ఫైల్‌లను షేర్ చేయలేకపోతోంది. బదులుగా, లింక్ ఉన్న ఎవరైనా వాటిని తెరవవచ్చు.
   *[other] Google ఖాతా లేని { $addresses }తో Google Drive ఫైల్‌లను షేర్ చేయలేకపోతోంది. బదులుగా, లింక్ ఉన్న ఎవరైనా వాటిని తెరవవచ్చు.
}
compose-drive-share-link = లింక్‌తో షేర్ చేయండి
compose-drive-send-without = షేర్ చేయకుండా పంపండి
compose-drive-share-cancel = రద్దు చేయండి
compose-drive-card-detail = { $size } · Google Drive
compose-drop-files = ఫైల్‌లను ఇక్కడ వదలండి
compose-drop-here = ఇక్కడ వదలండి
compose-paste-keep-formatting = ఫార్మాటింగ్‌ను ఉంచండి
compose-paste-table = టేబుల్
compose-paste-picture = చిత్రం
compose-paste-plain-text = సాధారణ టెక్స్ట్
compose-paste-inline = టెక్స్ట్‌లో
compose-paste-attachment = అటాచ్‌మెంట్

## Encryption and signing (the toggles by the recipients)

compose-encrypt = ఎన్‌క్రిప్ట్ చేయండి
compose-encrypted = ఎన్‌క్రిప్ట్ చేయబడింది: స్వీకర్తలు మాత్రమే చదవగలరు
compose-sign = సంతకం చేయండి
compose-signed = సంతకం చేయబడింది: ఇది మీ నుండే వచ్చిందని స్వీకర్తలు తనిఖీ చేయగలరు
compose-track = ఓపెన్‌లు, క్లిక్‌లను ట్రాక్ చేయండి
compose-tracked = ట్రాక్ చేయబడుతోంది: ప్రతి స్వీకర్త దీన్ని తెరిచినప్పుడు లేదా లింక్‌ను తెరిచినప్పుడు మీకు కనిపిస్తుంది
compose-track-clicks = లింక్ క్లిక్‌లను ట్రాక్ చేయండి (సాధారణ టెక్స్ట్‌లో తెరవడం చూపించలేము)
compose-tracked-clicks = ట్రాక్ చేయబడుతోంది: ప్రతి స్వీకర్త లింక్‌ను తెరిచినప్పుడు మీకు కనిపిస్తుంది
compose-track-sign-in = ఓపెన్‌లు, క్లిక్‌లను ట్రాక్ చేయడానికి Katna ఖాతాకు సైన్ ఇన్ చేయండి
compose-receipt = రీడ్ రసీదును అభ్యర్థించండి
compose-receipt-on = రీడ్ రసీదు అభ్యర్థించబడింది: స్వీకర్త యాప్ దాన్ని పంపమని వారిని అడగవచ్చు
compose-delivery = డెలివరీ రసీదును అభ్యర్థించండి
compose-delivery-on = డెలివరీ రసీదు అభ్యర్థించబడింది: ప్రతి స్వీకర్త సర్వర్ మెసేజ్‌ను అంగీకరించినప్పుడు మీ మెయిల్ సర్వర్ మీకు ఈమెయిల్ పంపుతుంది
compose-delivery-unavailable = మీ మెయిల్ సర్వర్ డెలివరీ రసీదులను పంపదు

## Spelling

spell-no-dictionary = { $language } కోసం స్పెల్లింగ్ డిక్షనరీ ఇన్‌స్టాల్ కాలేదు (ఉదాహరణకు hunspell-en_us).
spell-dictionary-error = స్పెల్లింగ్ డిక్షనరీ: { $error }

## Grammar checking (the right-click menu on a grammar mistake)

grammar-replace = “{ $words }”
grammar-add = “{ $words }”ను జోడించండి
grammar-remove = “{ $words }”ను తీసివేయండి
grammar-ignore = విస్మరించండి

## Send checks (asked before a message goes out)

send-check-attachment-title = మీరు ఫైల్‌లను అటాచ్ చేయాలనుకున్నారా?
send-check-attachment-text = మీరు అటాచ్‌మెంట్ గురించి రాశారు, కానీ ఏదీ అటాచ్ చేయలేదు.
send-check-attach = ఫైల్‌ను అటాచ్ చేయండి
send-check-subject-title = సబ్జెక్ట్ లేకుండా పంపాలా?
send-check-subject-text = ఈ మెసేజ్‌కు సబ్జెక్ట్ లేదు.
send-check-add-subject = సబ్జెక్ట్ జోడించండి
send-check-send-anyway = అయినా పంపండి
recipient-not-valid = చెల్లుబాటు అయ్యే ఈమెయిల్ అడ్రస్ కాదు
recipient-show-address = అడ్రస్ చూపండి
recipient-remove = తీసివేయండి
recipient-bad-title = అడ్రస్‌ను తనిఖీ చేయండి
recipient-bad-text = “{ $address }” చెల్లుబాటు అయ్యే ఈమెయిల్ అడ్రస్ కాదు. పంపే ముందు దాన్ని సరిచేయండి లేదా తీసివేయండి.
recipient-bad-fix = సరిచేయండి
