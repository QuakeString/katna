# Katna Mail, Thai (ไทย).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.
chat-heading = การอ่าน
chat-view = การสนทนาแบบแชท
chat-view-detail = อีเมลระหว่างผู้คนจะอ่านเหมือนแชทกลุ่ม: หนึ่งบับเบิลต่ออีเมลหนึ่งฉบับ แสดงเฉพาะสิ่งที่เขียน ของคุณเองอยู่ทางขวา จดหมายข่าวยังแสดงแบบปกติ
chat-view-switch = แสดงการสนทนาแบบแชท
chat-view-switch-detail = อีเมลที่อ้างอิงและลายเซ็นจะซ่อนอยู่หลัง ··· ในแต่ละบับเบิล
chat-switch-chat = แชท
chat-switch-mail = อีเมล
chat-people = { $names } และคุณ · { $count ->
   *[other] อีเมล { $count } ฉบับ
}
chat-people-heading = { $count ->
   *[other] ในแชทนี้ · { $count } คน
}
chat-member-mails = { $count ->
    [0] ไม่มีอีเมล
    [one] อีเมล { $count } ฉบับ
   *[other] อีเมล { $count } ฉบับ
}
chat-today = วันนี้
chat-yesterday = เมื่อวาน
chat-added = { $who } เพิ่ม { $names }
chat-renamed = { $who } เปลี่ยนหัวเรื่องเป็น “{ $subject }”
chat-you = คุณ
chat-not-downloaded = ยังไม่ได้ดาวน์โหลด
chat-forwarded = ส่งต่อ
chat-show-quoted = แสดงอีเมลที่อ้างอิงและลายเซ็น
chat-hide-quoted = ซ่อนอีเมลที่อ้างอิงและลายเซ็น
chat-hide-dots = ซ่อน ···
chat-show-card = แสดงการ์ดของผู้ติดต่อ
chat-reply-all = ตอบกลับทุกคน
chat-more = เพิ่มเติม
chat-reply-only = ตอบกลับเฉพาะ { $name }
chat-forward = ส่งต่อ
chat-copy-text = คัดลอกข้อความ
chat-show-as-mail = แสดงเป็นอีเมล
chat-pin = ปักหมุดไว้ด้านบน
chat-pin-file = ปักหมุดไฟล์ไว้ด้านบน
chat-unpin = เลิกปักหมุด
chat-unpin-file = เลิกปักหมุดไฟล์
chat-pinned-of = ปักหมุด { $at } จาก { $count }
chat-pins-all = หมุดทั้งหมด
chat-pins-heading = ปักหมุดแล้ว · { $count } จาก { $most }
chat-pins-drag = ลากเพื่อจัดลำดับใหม่
chat-pin-from-mail = อีเมลจาก { $name } · { $when }
chat-pin-from-file = ไฟล์จาก { $name } · { $when }
chat-pin-from-text = ข้อความจาก { $name } · { $when }
chat-pins-full = แชทนี้มีหมุดครบ 5 รายการแล้ว
chat-pins-replace-title = แทนที่หมุด
chat-pins-replace-hint = แชทหนึ่งมีหมุดได้สูงสุด 5 รายการ เลือกรายการที่จะนำออก
chat-pins-replace = แทนที่
chat-pins-cancel = ยกเลิก
chat-undo = เลิกทำ
chat-reply-to = ตอบกลับ { $names }
chat-send = ส่ง (Ctrl+Enter)
chat-attach = แนบ
chat-attach-photo = รูปภาพ
chat-attach-file = ไฟล์
chat-attach-library = จากไฟล์
chat-attach-template = เทมเพลต
chat-attach-signature = ลายเซ็น
chat-replying-to = กำลังตอบกลับ { $name }
chat-reply-newest = ตอบกลับอีเมลล่าสุด

## The attach picker (paperclip > From Files)

picker-title = แนบจากไฟล์
picker-search = ค้นหาชื่อ ผู้คน หัวเรื่อง
picker-search-drive = ค้นหาในไดรฟ์นี้
picker-mail-files = ไฟล์ในอีเมล
picker-this-chat = การสนทนานี้
picker-this-computer = คอมพิวเตอร์เครื่องนี้…
picker-in-chat = ในการสนทนานี้
picker-recent = ล่าสุด
picker-preview = ดูตัวอย่าง
picker-cancel = ยกเลิก
picker-attach = แนบ
picker-attach-count = แนบ { $count } รายการ
picker-selected = เลือกแล้ว { $count } รายการ
picker-of-limit = จาก { $limit }
picker-in-mail = { $size } ในอีเมล
picker-drive-links = { $count ->
   *[other] { $count } รายการเป็นลิงก์ Google Drive
}
picker-onedrive-links = { $count ->
   *[other] { $count } รายการเป็นลิงก์ OneDrive
}
picker-over = { $size } เกิน { $limit } ที่อีเมลหนึ่งฉบับรับได้
picker-getting = { $count ->
   *[other] กำลังดึงไฟล์ { $count } รายการจากไดรฟ์…
}
picker-some-failed = { $count ->
   *[other] อ่านไฟล์ { $count } รายการไม่ได้
}
