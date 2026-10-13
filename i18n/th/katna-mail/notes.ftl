# Katna Mail, Thai (ไทย): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = โน้ต
notes-view-reminders = การช่วยเตือน
notes-view-archive = เก็บถาวร
notes-view-trash = ถังขยะ
notes-edit-labels = แก้ไขป้ายกำกับ
notes-search = ค้นหาโน้ต
notes-loading = กำลังเปิดโน้ตของคุณ…

## Board

notes-take-a-note = จดโน้ต…
notes-new-list = รายการใหม่
notes-new-note = โน้ตใหม่
notes-pinned = ปักหมุดแล้ว
notes-others = อื่นๆ
notes-empty = โน้ตที่คุณเพิ่มจะปรากฏที่นี่
notes-archive-empty = โน้ตที่เก็บถาวรจะปรากฏที่นี่
notes-trash-empty = ไม่มีโน้ตในถังขยะ
notes-none-found = ไม่พบโน้ตที่ตรงกัน
notes-label-empty = ยังไม่มีโน้ตที่มีป้ายกำกับนี้
notes-reminders-empty = โน้ตที่มีการช่วยเตือนที่กำลังจะถึงจะปรากฏที่นี่
notes-trash-note = โน้ตในถังขยะจะถูกลบหลังจาก 7 วัน
notes-empty-trash = ล้างถังขยะ
notes-ticked = { $count ->
   *[other] + รายการที่ทำเครื่องหมายแล้ว { $count } รายการ
}
notes-select = เลือกโน้ต
notes-selected = { $count ->
   *[other] เลือก { $count } รายการ
}
notes-select-clear = ล้างการเลือก

## A note's buttons

notes-pin = ปักหมุดโน้ต
notes-unpin = เลิกปักหมุดโน้ต
notes-archive = เก็บถาวร
notes-unarchive = เลิกเก็บถาวร
notes-delete = ลบโน้ต
notes-restore = กู้คืน
notes-delete-forever = ลบถาวร
notes-color = ตัวเลือกพื้นหลัง
notes-checkboxes = แสดง/ซ่อนช่องทำเครื่องหมาย
notes-labels = ป้ายกำกับ
notes-close = ปิด
notes-more = เพิ่มเติม
notes-make-copy = ทำสำเนา
notes-remind = เตือนฉัน
notes-add-picture = เพิ่มรูปภาพ
notes-history = ประวัติเวอร์ชัน
notes-ai = ช่วยฉันเขียน
notes-send-as-mail = ส่งเป็นอีเมล
notes-save-markdown = บันทึกเป็น Markdown
notes-save-pdf = บันทึกเป็น PDF

## The open note

notes-title = ชื่อ
notes-edited = แก้ไขเมื่อ { $date }
notes-on-this-computer = ในคอมพิวเตอร์เครื่องนี้
notes-where = ที่เก็บโน้ตนี้
notes-untitled = โน้ตไม่มีชื่อ

## Pictures

notes-picture-choose = เพิ่มรูปภาพ
notes-picture-remove = นำรูปภาพออก
notes-picture-too-big = ใส่รูปภาพในโน้ตได้ไม่เกิน { $size }
notes-picture-kind = ไฟล์นั้นไม่ใช่รูปภาพที่ Katna แสดงได้
notes-picture-unreadable = อ่าน { $name } ไม่ได้: { $error }

## Reminders

notes-remind-me = เตือนฉัน
notes-remind-off = นำการช่วยเตือนออก
notes-remind-in-the-past = โปรดเลือกเวลาที่ยังมาไม่ถึง
notes-remind-today = วันนี้, { $time }
notes-remind-tomorrow = พรุ่งนี้, { $time }
notes-remind-weekday = { $day }, { $time }
notes-reminder-set = ตั้งการช่วยเตือนไว้ที่ { $when }
notes-reminder-off = นำการช่วยเตือนออกแล้ว

## Links between notes

notes-link-note = ลิงก์โน้ต
notes-link-new = โน้ตใหม่ “{ $title }”
notes-linked-from = ลิงก์มาจาก
notes-link-gone = โน้ตนั้นไม่อยู่แล้ว
notes-new-note-gone = โน้ตใหม่หายไปแล้ว

## Version history

notes-versions = เวอร์ชัน
notes-version-now = ตอนนี้
notes-version-here = คุณ ในคอมพิวเตอร์เครื่องนี้
notes-version-yesterday = เมื่อวาน, { $time }
notes-version-changes = { $count ->
   *[other] เปลี่ยน { $count } จุด
}
notes-version-from = จาก { $device }
notes-version-elsewhere = จากอุปกรณ์อื่น
notes-version-created = สร้างแล้ว
notes-version-restore = กู้คืนเวอร์ชันนี้
notes-version-restored = กู้คืนเวอร์ชันแล้ว
notes-history-none = ยังไม่มีเวอร์ชันก่อนหน้า

## AI help

notes-ai-tidy = เรียบเรียงข้อความ
notes-ai-checklist = เปลี่ยนเป็นรายการตรวจสอบ
notes-ai-summarise = สรุป
notes-ai-empty = เขียนอะไรสักอย่างก่อน
notes-ai-tidied = เรียบเรียงข้อความแล้ว กด Ctrl+Z เพื่อย้อนกลับ
notes-ai-listed = เปลี่ยนเป็นรายการตรวจสอบแล้ว กด Ctrl+Z เพื่อย้อนกลับ
notes-ai-summarised = เพิ่มสรุปไว้ด้านบนแล้ว

## Labels

notes-label-note = ติดป้ายกำกับโน้ต
notes-label-name = ป้อนชื่อป้ายกำกับ
notes-label-create = สร้าง “{ $name }”
notes-label-remove = นำป้ายกำกับออก
notes-label-delete = ลบป้ายกำกับ
notes-labels-none = ยังไม่มีป้ายกำกับ เพิ่มได้จากปุ่มป้ายกำกับของโน้ต
notes-labels-done = เสร็จสิ้น
notes-label-renamed = เปลี่ยนชื่อป้ายกำกับเป็น “{ $name }” แล้ว
notes-label-deleted = ลบป้ายกำกับ “{ $name }” แล้ว

## A note about a mail

notes-mail = อีเมล
notes-open-mail = เปิดอีเมล
notes-open-note = เปิดโน้ต

## Meeting notes

notes-meeting-take = จดโน้ตการประชุม
notes-meeting-title = { $title } · { $date }
notes-meeting-attendees = ผู้เข้าร่วม: { $names }
notes-meeting-notes = โน้ต
notes-meeting-actions = รายการที่ต้องดำเนินการ
notes-event = กิจกรรม
notes-open-event = เปิดกิจกรรม

## Formatting

notes-format = การจัดรูปแบบ
notes-format-heading-1 = หัวเรื่อง 1
notes-format-heading-2 = หัวเรื่อง 2
notes-format-normal = ข้อความปกติ
notes-format-bold = ตัวหนา
notes-format-italic = ตัวเอียง
notes-format-underline = ขีดเส้นใต้
notes-format-quote = คำพูด
notes-format-code = โค้ด
notes-format-divider = เส้นคั่น
notes-format-clear = ล้างการจัดรูปแบบ

## Tasks

notes-make-task = ทำเป็นงาน

## Colors (tooltips)

notes-color-none = ไม่มีสี
notes-color-coral = ปะการัง
notes-color-peach = พีช
notes-color-sand = ทราย
notes-color-mint = มินต์
notes-color-sage = เซจ
notes-color-fog = หมอก
notes-color-storm = พายุ
notes-color-dusk = สนธยา
notes-color-blossom = ดอกไม้บาน
notes-color-clay = ดินเหนียว
notes-color-chalk = ชอล์ก

## Messages at the foot of the window

notes-archived = เก็บโน้ตถาวรแล้ว
notes-unarchived = เลิกเก็บโน้ตถาวรแล้ว
notes-trashed = ย้ายโน้ตไปที่ถังขยะแล้ว
notes-restored = กู้คืนโน้ตแล้ว
notes-saved = บันทึกโน้ตแล้ว
notes-pinned-count = { $count ->
   *[other] ปักหมุดโน้ต { $count } รายการแล้ว
}
notes-unpinned-count = { $count ->
   *[other] เลิกปักหมุดโน้ต { $count } รายการแล้ว
}
notes-colored-count = { $count ->
   *[other] เปลี่ยนสีโน้ต { $count } รายการแล้ว
}
notes-archived-count = { $count ->
   *[other] เก็บโน้ต { $count } รายการถาวรแล้ว
}
notes-unarchived-count = { $count ->
   *[other] เลิกเก็บถาวรโน้ต { $count } รายการแล้ว
}
notes-trashed-count = { $count ->
   *[other] ย้ายโน้ต { $count } รายการไปที่ถังขยะแล้ว
}
notes-restored-count = { $count ->
   *[other] กู้คืนโน้ต { $count } รายการแล้ว
}
notes-copied-count = { $count ->
   *[other] ทำสำเนา { $count } รายการแล้ว
}
notes-empty-discarded = ทิ้งโน้ตเปล่าแล้ว
notes-mail-gone = ไม่มีอีเมลนั้นแล้ว
notes-deleted-forever = { $count ->
   *[other] ลบโน้ต { $count } รายการถาวรแล้ว
}
