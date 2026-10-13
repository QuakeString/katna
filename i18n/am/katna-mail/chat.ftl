# Katna Mail, Amharic (አማርኛ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

chat-heading = ንባብ
chat-view = ውይይቶች እንደ ቻት
chat-view-detail = በሰዎች መካከል ያለ ደብዳቤ እንደ የቡድን ቻት ይነበባል፦ ለእያንዳንዱ ደብዳቤ የተጻፈውን ብቻ የያዘ አረፋ፣ የእርስዎ በቀኝ በኩል። የዜና መጽሔቶች የተለመደውን እይታ ይይዛሉ።
chat-view-switch = ውይይቶችን እንደ ቻት አሳይ
chat-view-switch-detail = የተጠቀሰው ደብዳቤ እና ፊርማዎች በእያንዳንዱ አረፋ ውስጥ ከ··· ጀርባ ይጠብቃሉ

chat-switch-chat = ቻት
chat-switch-mail = ደብዳቤ
chat-people = { $names } እና እርስዎ · { $count ->
    [one] { $count } ደብዳቤ
   *[other] { $count } ደብዳቤዎች
}
chat-people-heading = { $count ->
    [one] በዚህ ቻት ውስጥ · { $count } ሰው
   *[other] በዚህ ቻት ውስጥ · { $count } ሰዎች
}
chat-member-mails = { $count ->
    [0] ምንም ደብዳቤ የለም
    [one] { $count } ደብዳቤ
   *[other] { $count } ደብዳቤዎች
}
chat-today = ዛሬ
chat-yesterday = ትናንት
chat-added = { $who } { $names }ን አከለ
chat-renamed = { $who } ርዕሰ ጉዳዩን ወደ «{ $subject }» ቀየረ
chat-you = እርስዎ
chat-not-downloaded = ገና አልወረደም
chat-forwarded = የተላለፈ
chat-show-quoted = የተጠቀሰውን ደብዳቤ እና ፊርማ አሳይ
chat-hide-quoted = የተጠቀሰውን ደብዳቤ እና ፊርማ ደብቅ
chat-hide-dots = ···ን ደብቅ
chat-show-card = ካርዳቸውን አሳይ
chat-reply-all = ለሁሉም መልስ
chat-more = ተጨማሪ
chat-reply-only = ለ{ $name } ብቻ መልስ
chat-forward = አስተላልፍ
chat-copy-text = ጽሑፍ ቅዳ
chat-show-as-mail = እንደ ደብዳቤ አሳይ
chat-go-down = ወደ አዲሱ ደብዳቤ ሂድ
chat-pin = ከላይ ሰካ
chat-pin-file = ፋይሉን ከላይ ሰካ
chat-unpin = ንቀል
chat-unpin-file = ፋይሉን ንቀል
chat-pinned-of = የተሰካ { $at } ከ{ $count }
chat-pins-all = ሁሉም የተሰኩ
chat-pins-heading = የተሰኩ · { $count } ከ{ $most }
chat-pins-drag = ቅደም ተከተሉን ለመቀየር ይጎትቱ
chat-pin-from-mail = ከ{ $name } የመጣ ደብዳቤ · { $when }
chat-pin-from-file = ከ{ $name } የመጣ ፋይል · { $when }
chat-pin-from-text = ከ{ $name } የመጣ ጽሑፍ · { $when }
chat-pins-full = ይህ ቻት አስቀድሞ 5 የተሰኩ ይዟል
chat-pins-replace-title = የተሰካን ተካ
chat-pins-replace-hint = አንድ ቻት እስከ 5 የተሰኩ ይይዛል። የሚነሳውን ይምረጡ።
chat-pins-replace = ተካ
chat-pins-cancel = ይቅር
chat-undo = ቀልብስ

chat-reply-to = ለ{ $names } መልስ
chat-send = ላክ (Ctrl+Enter)። ለተጨማሪ በቀኝ ጠቅ ያድርጉ ወይም ተጭነው ይያዙ
chat-send-now = አሁን ላክ
chat-attach = አያይዝ
chat-attach-photo = ፎቶ
chat-attach-file = ፋይል
chat-attach-library = ከፋይሎች
chat-attach-template = አብነት
chat-attach-signature = ፊርማ
chat-replying-to = ለ{ $name } በመመለስ ላይ
chat-reply-newest = ለአዲሱ ደብዳቤ መልስ

## The attach picker (paperclip > From Files)

picker-title = ከፋይሎች አያይዝ
picker-search = ስሞችን፣ ሰዎችን፣ ርዕሰ ጉዳዮችን ፈልግ
picker-search-drive = በዚህ ድራይቭ ውስጥ ፈልግ
picker-mail-files = የደብዳቤ ፋይሎች
picker-this-chat = ይህ ውይይት
picker-this-computer = ይህ ኮምፒውተር…
picker-in-chat = በዚህ ውይይት ውስጥ
picker-recent = የቅርብ ጊዜ
picker-preview = ቅድመ እይታ
picker-cancel = ይቅር
picker-attach = አያይዝ
picker-attach-count = { $count } አያይዝ
picker-selected = { $count } ተመርጠዋል
picker-of-limit = ከ{ $limit }
picker-in-mail = { $size } በደብዳቤው ውስጥ
picker-drive-links = { $count ->
    [one] { $count } እንደ Google Drive አገናኝ
   *[other] { $count } እንደ Google Drive አገናኞች
}
picker-onedrive-links = { $count ->
    [one] { $count } እንደ OneDrive አገናኝ
   *[other] { $count } እንደ OneDrive አገናኞች
}
picker-over = { $size }፣ አንድ ደብዳቤ ሊይዘው ከሚችለው { $limit } በላይ
picker-getting = { $count ->
    [one] ፋይሉን ከድራይቩ በማምጣት ላይ…
   *[other] { $count } ፋይሎችን ከድራይቩ በማምጣት ላይ…
}
picker-some-failed = { $count ->
    [one] { $count } ፋይል ማንበብ አልተቻለም
   *[other] { $count } ፋይሎችን ማንበብ አልተቻለም
}
