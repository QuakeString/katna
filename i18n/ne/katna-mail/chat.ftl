# Katna Mail, Nepali (नेपाली).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

chat-heading = पढाइ
chat-view = वार्तालापहरू च्याटका रूपमा
chat-view-detail = मानिसहरूबीचको मेल समूह च्याटजस्तै पढिन्छ: प्रत्येक मेलका लागि लेखिएको कुरा मात्र भएको एउटा बबल, तपाईंका आफ्नै दायाँतिर। न्यूजलेटरहरू सामान्य दृश्यमै रहन्छन्।
chat-view-switch = वार्तालापहरू च्याटका रूपमा देखाउनुहोस्
chat-view-switch-detail = उद्धृत मेल र हस्ताक्षरहरू प्रत्येक बबलमा ··· पछाडि रहन्छन्

chat-switch-chat = च्याट
chat-switch-mail = मेल
chat-people = { $names } र तपाईं · { $count ->
    [one] { $count } मेल
   *[other] { $count } मेल
}
chat-people-heading = { $count ->
    [one] यस च्याटमा · { $count } व्यक्ति
   *[other] यस च्याटमा · { $count } व्यक्ति
}
chat-member-mails = { $count ->
    [0] कुनै मेल छैन
    [one] { $count } मेल
   *[other] { $count } मेल
}
chat-today = आज
chat-yesterday = हिजो
chat-added = { $who } ले { $names } लाई थप्नुभयो
chat-renamed = { $who } ले विषय बदलेर “{ $subject }” बनाउनुभयो
chat-you = तपाईं
chat-not-downloaded = अझै डाउनलोड भएको छैन
chat-forwarded = फर्वार्ड गरिएको
chat-show-quoted = उद्धृत मेल र हस्ताक्षर देखाउनुहोस्
chat-hide-quoted = उद्धृत मेल र हस्ताक्षर लुकाउनुहोस्
chat-hide-dots = ··· लुकाउनुहोस्
chat-show-card = उहाँको कार्ड देखाउनुहोस्
chat-reply-all = सबैलाई जवाफ दिनुहोस्
chat-more = थप
chat-reply-only = { $name } लाई मात्र जवाफ दिनुहोस्
chat-forward = फर्वार्ड गर्नुहोस्
chat-copy-text = पाठ कपी गर्नुहोस्
chat-show-as-mail = मेलका रूपमा देखाउनुहोस्
chat-pin = माथि पिन गर्नुहोस्
chat-pin-file = फाइल माथि पिन गर्नुहोस्
chat-unpin = पिन हटाउनुहोस्
chat-unpin-file = फाइलको पिन हटाउनुहोस्
chat-pinned-of = { $count } मध्ये { $at } पिन गरिएको
chat-pins-all = सबै पिनहरू
chat-pins-heading = पिन गरिएका · { $most } मध्ये { $count }
chat-pins-drag = क्रम बदल्न तान्नुहोस्
chat-pin-from-mail = { $name } को मेल · { $when }
chat-pin-from-file = { $name } को फाइल · { $when }
chat-pin-from-text = { $name } को पाठ · { $when }
chat-pins-full = यस च्याटमा पहिले नै 5 पिन छन्
chat-pins-replace-title = पिन बदल्नुहोस्
chat-pins-replace-hint = एउटा च्याटमा 5 वटासम्म पिन हुन्छन्। हटाउने एउटा छान्नुहोस्।
chat-pins-replace = बदल्नुहोस्
chat-pins-cancel = रद्द गर्नुहोस्
chat-undo = पूर्ववत गर्नुहोस्

chat-reply-to = { $names } लाई जवाफ दिनुहोस्
chat-send = पठाउनुहोस् (Ctrl+Enter)
chat-attach = संलग्न गर्नुहोस्
chat-attach-photo = फोटो
chat-attach-file = फाइल
chat-attach-library = फाइलहरूबाट
chat-attach-template = टेम्प्लेट
chat-attach-signature = हस्ताक्षर
chat-replying-to = { $name } लाई जवाफ दिँदै
chat-reply-newest = सबैभन्दा नयाँ मेललाई जवाफ दिनुहोस्

## The attach picker (paperclip > From Files)

picker-title = फाइलहरूबाट संलग्न गर्नुहोस्
picker-search = नाम, व्यक्ति, विषय खोज्नुहोस्
picker-search-drive = यो ड्राइभ खोज्नुहोस्
picker-mail-files = मेलका फाइलहरू
picker-this-chat = यो वार्तालाप
picker-this-computer = यो कम्प्युटर…
picker-in-chat = यस वार्तालापमा
picker-recent = हालैका
picker-preview = पूर्वावलोकन
picker-cancel = रद्द गर्नुहोस्
picker-attach = संलग्न गर्नुहोस्
picker-attach-count = { $count } संलग्न गर्नुहोस्
picker-selected = { $count } चयन गरिएको
picker-of-limit = { $limit } मध्ये
picker-in-mail = मेलमै { $size }
picker-drive-links = { $count ->
    [one] 1 Google Drive लिङ्कका रूपमा
   *[other] { $count } Google Drive लिङ्कका रूपमा
}
picker-onedrive-links = { $count ->
    [one] 1 OneDrive लिङ्कका रूपमा
   *[other] { $count } OneDrive लिङ्कका रूपमा
}
picker-over = { $size }, एउटा मेलले लैजान सक्ने { $limit } भन्दा बढी
picker-getting = { $count ->
    [one] ड्राइभबाट फाइल ल्याउँदै…
   *[other] ड्राइभबाट { $count } फाइलहरू ल्याउँदै…
}
picker-some-failed = { $count ->
    [one] एउटा फाइल पढ्न सकिएन
   *[other] { $count } फाइलहरू पढ्न सकिएनन्
}
