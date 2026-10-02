# Katna Mail, Hindi (हिन्दी).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

chat-heading = पढ़ना
chat-view = बातचीत चैट की तरह
chat-view-detail = लोगों के बीच का मेल ग्रुप चैट की तरह दिखता है: हर मेल के लिए एक बबल, जिसमें सिर्फ़ लिखा गया टेक्स्ट होता है, और आपके मेल दाईं ओर। न्यूज़लेटर सामान्य व्यू में ही दिखते हैं।
chat-view-switch = बातचीत को चैट की तरह दिखाएं
chat-view-switch-detail = कोट किया गया मेल और हस्ताक्षर हर बबल में ··· के पीछे रहते हैं

chat-switch-chat = चैट
chat-switch-mail = मेल
chat-people = { $names } और आप · { $count ->
    [one] { $count } मेल
   *[other] { $count } मेल
}
chat-people-heading = { $count ->
    [one] इस चैट में · { $count } व्यक्ति
   *[other] इस चैट में · { $count } लोग
}
chat-member-mails = { $count ->
    [0] कोई मेल नहीं
    [one] { $count } मेल
   *[other] { $count } मेल
}
chat-today = आज
chat-yesterday = कल
chat-added = { $who } ने { $names } को जोड़ा
chat-renamed = { $who } ने विषय बदलकर “{ $subject }” किया
chat-you = आप
chat-not-downloaded = अभी डाउनलोड नहीं हुआ
chat-forwarded = फ़ॉरवर्ड किया गया
chat-show-quoted = कोट किया गया मेल और हस्ताक्षर दिखाएं
chat-hide-quoted = कोट किया गया मेल और हस्ताक्षर छिपाएं
chat-hide-dots = ··· छिपाएं
chat-show-card = उनका कार्ड दिखाएं
chat-reply-all = सभी को जवाब दें
chat-more = ज़्यादा
chat-reply-only = सिर्फ़ { $name } को जवाब दें
chat-forward = फ़ॉरवर्ड करें
chat-copy-text = टेक्स्ट कॉपी करें
chat-show-as-mail = मेल की तरह दिखाएं
chat-pin = सबसे ऊपर पिन करें
chat-pin-file = फ़ाइल सबसे ऊपर पिन करें
chat-unpin = अनपिन करें
chat-unpin-file = फ़ाइल अनपिन करें
chat-pinned-of = { $count } में से { $at } पिन
chat-pins-all = सभी पिन
chat-pins-heading = पिन किए गए · { $most } में से { $count }
chat-pins-drag = क्रम बदलने के लिए खींचें
chat-pin-from-mail = { $name } का मेल · { $when }
chat-pin-from-file = { $name } की फ़ाइल · { $when }
chat-pin-from-text = { $name } का टेक्स्ट · { $when }
chat-pins-full = इस चैट में पहले से 5 पिन हैं
chat-pins-replace-title = कोई पिन बदलें
chat-pins-replace-hint = एक चैट में ज़्यादा से ज़्यादा 5 पिन हो सकते हैं। हटाने के लिए एक चुनें।
chat-pins-replace = बदलें
chat-pins-cancel = रद्द करें
chat-undo = पहले जैसा करें

chat-reply-to = { $names } को जवाब दें
chat-send = भेजें (Ctrl+Enter)
chat-attach = अटैच करें
chat-attach-photo = फ़ोटो
chat-attach-file = फ़ाइल
chat-attach-library = फ़ाइलें पेज से
chat-attach-template = टेम्प्लेट
chat-attach-signature = हस्ताक्षर
chat-replying-to = { $name } को जवाब दिया जा रहा है
chat-reply-newest = सबसे नए मेल का जवाब दें

## The attach picker (paperclip > From Files)

picker-title = फ़ाइलें पेज से अटैच करें
picker-search = नाम, लोग, विषय खोजें
picker-search-drive = यह ड्राइव खोजें
picker-mail-files = मेल की फ़ाइलें
picker-this-chat = यह बातचीत
picker-this-computer = यह कंप्यूटर…
picker-in-chat = इस बातचीत में
picker-recent = हाल की
picker-preview = प्रीव्यू
picker-cancel = रद्द करें
picker-attach = अटैच करें
picker-attach-count = { $count } अटैच करें
picker-selected = { $count } चुनी गईं
picker-of-limit = { $limit } में से
picker-in-mail = मेल में { $size }
picker-drive-links = { $count ->
    [one] 1 Google Drive लिंक के रूप में
   *[other] { $count } Google Drive लिंक के रूप में
}
picker-onedrive-links = { $count ->
    [one] 1 OneDrive लिंक के रूप में
   *[other] { $count } OneDrive लिंक के रूप में
}
picker-over = { $size }, एक मेल की सीमा { $limit } से ज़्यादा
picker-getting = { $count ->
    [one] ड्राइव से फ़ाइल लाई जा रही है…
   *[other] ड्राइव से { $count } फ़ाइलें लाई जा रही हैं…
}
picker-some-failed = { $count ->
    [one] एक फ़ाइल पढ़ी नहीं जा सकी
   *[other] { $count } फ़ाइलें पढ़ी नहीं जा सकीं
}
