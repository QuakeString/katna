# Katna Mail, Marathi (मराठी).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

chat-heading = वाचन
chat-view = संभाषणे चॅटसारखी
chat-view-detail = लोकांमधील मेल ग्रुप चॅटसारखा वाचता येतो: प्रत्येक मेलसाठी फक्त लिहिलेल्या मजकुराचा एक बबल, तुमचे स्वतःचे उजवीकडे. वृत्तपत्रे नेहमीच्या दृश्यात राहतात.
chat-view-switch = संभाषणे चॅटसारखी दाखवा
chat-view-switch-detail = उद्धृत मेल आणि स्वाक्षऱ्या प्रत्येक बबलमध्ये ··· मागे राहतात

chat-switch-chat = चॅट
chat-switch-mail = मेल
chat-people = { $names } आणि तुम्ही · { $count ->
    [one] { $count } मेल
   *[other] { $count } मेल
}
chat-people-heading = { $count ->
    [one] या चॅटमध्ये · { $count } व्यक्ती
   *[other] या चॅटमध्ये · { $count } व्यक्ती
}
chat-member-mails = { $count ->
    [0] एकही मेल नाही
    [one] { $count } मेल
   *[other] { $count } मेल
}
chat-today = आज
chat-yesterday = काल
chat-added = { $who } यांनी { $names } यांना जोडले
chat-renamed = { $who } यांनी विषय बदलून “{ $subject }” केला
chat-you = तुम्ही
chat-not-downloaded = अजून डाउनलोड केलेले नाही
chat-forwarded = फॉरवर्ड केलेले
chat-show-quoted = उद्धृत मेल आणि स्वाक्षरी दाखवा
chat-hide-quoted = उद्धृत मेल आणि स्वाक्षरी लपवा
chat-hide-dots = ··· लपवा
chat-show-card = त्यांचे कार्ड दाखवा
chat-reply-all = सर्वांना उत्तर द्या
chat-more = अधिक
chat-reply-only = फक्त { $name } यांना उत्तर द्या
chat-forward = फॉरवर्ड करा
chat-copy-text = मजकूर कॉपी करा
chat-show-as-mail = मेल म्हणून दाखवा
chat-go-down = सर्वात नवीन मेलवर जा
chat-pin = सर्वात वर पिन करा
chat-pin-file = फाइल सर्वात वर पिन करा
chat-unpin = अनपिन करा
chat-unpin-file = फाइल अनपिन करा
chat-pinned-of = { $count } पैकी { $at } पिन केलेले
chat-pins-all = सर्व पिन
chat-pins-heading = पिन केलेले · { $most } पैकी { $count }
chat-pins-drag = क्रम बदलण्यासाठी ड्रॅग करा
chat-pin-from-mail = { $name } यांचा मेल · { $when }
chat-pin-from-file = { $name } यांची फाइल · { $when }
chat-pin-from-text = { $name } यांचा मजकूर · { $when }
chat-pins-full = या चॅटमध्ये आधीच 5 पिन आहेत
chat-pins-replace-title = पिन बदला
chat-pins-replace-hint = एका चॅटमध्ये जास्तीत जास्त 5 पिन असतात. काढायचा असलेला पिन निवडा.
chat-pins-replace = बदला
chat-pins-cancel = रद्द करा
chat-undo = पूर्ववत करा

chat-reply-to = { $names } यांना उत्तर द्या
chat-send = पाठवा (Ctrl+Enter). अधिकसाठी राइट-क्लिक करा किंवा दाबून ठेवा
chat-send-now = आता पाठवा
chat-attach = अटॅच करा
chat-attach-photo = फोटो
chat-attach-file = फाइल
chat-attach-library = फाइल्समधून
chat-attach-template = टेम्पलेट
chat-attach-signature = स्वाक्षरी
chat-replying-to = { $name } यांना उत्तर देत आहे
chat-reply-newest = सर्वात नवीन मेलला उत्तर द्या

## The attach picker (paperclip > From Files)

picker-title = फाइल्समधून अटॅच करा
picker-search = नावे, लोक, विषय शोधा
picker-search-drive = ही ड्राइव्ह शोधा
picker-mail-files = मेलमधील फाइल्स
picker-this-chat = हे संभाषण
picker-this-computer = हा कॉम्प्युटर…
picker-in-chat = या संभाषणात
picker-recent = अलीकडील
picker-preview = पूर्वावलोकन
picker-cancel = रद्द करा
picker-attach = अटॅच करा
picker-attach-count = { $count } अटॅच करा
picker-selected = { $count } निवडले
picker-of-limit = / { $limit }
picker-in-mail = मेलमध्ये { $size }
picker-drive-links = { $count ->
    [one] 1 Google Drive लिंक म्हणून
   *[other] { $count } Google Drive लिंक म्हणून
}
picker-onedrive-links = { $count ->
    [one] 1 OneDrive लिंक म्हणून
   *[other] { $count } OneDrive लिंक म्हणून
}
picker-over = { $size }, एका मेलमध्ये जाऊ शकणाऱ्या { $limit } पेक्षा जास्त
picker-getting = { $count ->
    [one] ड्राइव्हवरून फाइल मिळवत आहे…
   *[other] ड्राइव्हवरून { $count } फाइल्स मिळवत आहे…
}
picker-some-failed = { $count ->
    [one] एक फाइल वाचता आली नाही
   *[other] { $count } फाइल्स वाचता आल्या नाहीत
}
