# Katna Mail, Hindi (हिन्दी).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

compose-ai-rephrase-tip = दूसरे शब्दों में लिखें (Ctrl+J)
compose-ai-tone-clearer = ज़्यादा साफ़
compose-ai-tone-shorter = छोटा
compose-ai-tone-friendlier = ज़्यादा दोस्ताना
compose-ai-tone-formal = औपचारिक
compose-ai-tone-grammar = व्याकरण ठीक करें
compose-ai-tone-longer = लंबा
compose-ai-custom = बताएं कैसे…
compose-ai-more = और तरीके
compose-ai-replace = बदलें
compose-ai-again = फिर से कोशिश करें
compose-ai-below = नीचे जोड़ें
compose-ai-copy = कॉपी करें
compose-ai-cancel = रद्द करें
compose-ai-rephrase = दूसरे शब्दों में लिखें
compose-ai-replaced = दूसरे शब्दों में लिखा गया
compose-ai-added = नीचे जोड़ा गया
compose-ai-copied = कॉपी किया गया
compose-ai-katna = Katna AI
compose-ai-own = आपकी AI सेवा
compose-ai-trial-left = { $service } · { $days ->
    [one] 1 दिन मुफ़्त बचा है
   *[other] { $days } दिन मुफ़्त बचे हैं
}
compose-ai-encrypted = यह मैसेज एन्क्रिप्ट किया जाएगा। दूसरे शब्दों में लिखने पर चुना गया टेक्स्ट बिना एन्क्रिप्शन के { $service } को जाता है। फिर भी लिखें?
compose-ai-sign-in = Katna AI के लिए Katna खाता चाहिए। इसका उपयोग करने के लिए साइन इन करें, या अपनी खुद की कुंजी इस्तेमाल करें।
compose-ai-pay = Katna AI का आपका मुफ़्त महीना खत्म हो गया है। इसकी कीमत $5 प्रति महीना है, या आप अपनी खुद की कुंजी इस्तेमाल कर सकते हैं।
compose-ai-too-many = अभी बहुत ज़्यादा अनुरोध हैं। थोड़ी देर बाद फिर से कोशिश करें।
compose-ai-no-key = दूसरे शब्दों में लिखने के लिए सेटिंग में अपनी { $service } कुंजी जोड़ें।
compose-ai-bad-key = { $service } ने आपकी कुंजी स्वीकार नहीं की। इसे सेटिंग में जांचें।
compose-ai-off = AI से लिखने में मदद सेटिंग में बंद है।
compose-ai-failed = { $service } तक नहीं पहुंचा जा सका। फिर से कोशिश करें।
compose-ai-try-again = फिर से कोशिश करें
compose-ai-open-settings = सेटिंग खोलें
compose-ai-write-reply-tip = जवाब लिखें (Ctrl+J)
compose-ai-write-note-tip = नोट लिखें (Ctrl+J)
compose-ai-rephrase-empty-tip = दूसरे शब्दों में लिखने के लिए कुछ टाइप करें
compose-ai-write-reply = जवाब लिखें
compose-ai-write-note = नोट लिखें
compose-ai-write-from = { $count ->
    [one] 1 मेल से
   *[other] { $count } मेल से
}
compose-ai-write-ideas = बातचीत से सुझाव
compose-ai-write-own = या बताएं कि इसमें क्या लिखा जाए…
compose-ai-write-short = छोटा
compose-ai-write-longer = लंबा
compose-ai-write-friendly = दोस्ताना
compose-ai-write-formal = औपचारिक
compose-ai-write-insert = डालें
compose-ai-write-back = दूसरे सुझाव
compose-ai-written = ड्राफ़्ट जोड़ा गया
compose-ai-write-encrypted = यह बातचीत एन्क्रिप्टेड है। जवाब लिखने पर इसके मेल बिना एन्क्रिप्शन के { $service } को जाते हैं। फिर भी लिखें?
compose-ai-write-anyway = लिखें
compose-ai-write-encrypted-off = यह बातचीत एन्क्रिप्टेड है, और सेटिंग के अनुसार एन्क्रिप्टेड मेल में लिखने में मदद नहीं मिलती।
compose-ai-subject-tip = विषय दूसरे शब्दों में लिखें
compose-ai-subject-title = इसे कहने के दूसरे तरीके
compose-ai-subject-done = विषय बदला गया

## Summing up a conversation: the list's right-click menu, the reading
## pane's sparkle, the chat's strip and the card each opens.

summary-summarize = सारांश बनाएं
summary-hide = सारांश छिपाएं
summary-close = बंद करें
summary-fold = समेटें
summary-title = सारांश
summary-mails = { $count ->
    [one] 1 मेल
   *[other] { $count } मेल
}
summary-of-mails = { $total } में से { $count } मेल
summary-peek-count = { $mails ->
    [one] 1 मेल
   *[other] { $mails } मेल
} · { $people ->
    [one] 1 व्यक्ति
   *[other] { $people } लोग
}
summary-catch-up = { $count ->
    [one] पिछली बार पढ़ने के बाद से 1 नया
   *[other] पिछली बार पढ़ने के बाद से { $count } नए
}
summary-strip-newer = { $count ->
    [one] तब से 1 नया · { $gist }
   *[other] तब से { $count } नए · { $gist }
}
summary-add-new = { $count ->
    [one] 1 नया जोड़ें
   *[other] { $count } नए जोड़ें
}
summary-point-settled = तय हुआ
summary-point-money = पैसा
summary-point-dates = तारीखें
summary-point-next = आगे
summary-point-open = बाकी
summary-files = फ़ाइलें
summary-for-you = आपके लिए
summary-from-mail = { $name }, { $date }
summary-you = आप
summary-made = { $service } · { $time }
summary-not-read = { $service } · पढ़ा गया के रूप में मार्क नहीं किया
summary-copy = कॉपी करें
summary-copied = सारांश कॉपी किया गया
summary-again = फिर से सारांश बनाएं
summary-open = खोलें
summary-open-tip = बातचीत खोलें
summary-reply = जवाब दें
summary-reply-tip = AI से जवाब लिखें
summary-reply-to = { $name } को जवाब दें
summary-reply-summary = सारांश
summary-reply-send = भेजें
summary-reply-open = खोलें
summary-asking = { $service } से पूछा जा रहा है…
summary-stop = रोकें
summary-cancel = रद्द करें
summary-send = भेजें और सारांश बनाएं
summary-ask-short = आपकी मंज़ूरी का इंतज़ार
summary-encrypted = यह बातचीत एन्क्रिप्टेड है। सारांश बनाने पर इसका टेक्स्ट बिना एन्क्रिप्शन के { $service } को जाता है।
summary-encrypted-off = यह बातचीत एन्क्रिप्टेड है, और सेटिंग के अनुसार एन्क्रिप्टेड मेल में लिखने में मदद नहीं मिलती।
summary-try-again = फिर से कोशिश करें
summary-open-settings = सेटिंग खोलें
