# Katna Mail, Thai (ไทย): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = โน้ต
notes-view-archive = เก็บถาวร
notes-view-trash = ถังขยะ
notes-edit-labels = แก้ไขป้ายกำกับ
notes-search = ค้นหาโน้ต
notes-loading = กำลังเปิดโน้ตของคุณ…

## Board

notes-take-a-note = จดโน้ต…
notes-new-list = รายการใหม่
notes-pinned = ปักหมุดแล้ว
notes-others = อื่นๆ
notes-empty = โน้ตที่คุณเพิ่มจะปรากฏที่นี่
notes-archive-empty = โน้ตที่เก็บถาวรจะปรากฏที่นี่
notes-trash-empty = ไม่มีโน้ตในถังขยะ
notes-none-found = ไม่พบโน้ตที่ตรงกัน
notes-label-empty = ยังไม่มีโน้ตที่มีป้ายกำกับนี้
notes-trash-note = โน้ตในถังขยะจะถูกลบหลังจาก 7 วัน
notes-empty-trash = ล้างถังขยะ
notes-ticked = { $count ->
   *[other] + รายการที่ทำเครื่องหมายแล้ว { $count } รายการ
}

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

## The open note

notes-title = ชื่อ
notes-edited = แก้ไขเมื่อ { $date }
notes-on-this-computer = ในคอมพิวเตอร์เครื่องนี้
notes-where = ที่เก็บโน้ตนี้

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
notes-empty-discarded = ทิ้งโน้ตเปล่าแล้ว
notes-mail-gone = ไม่มีอีเมลนั้นแล้ว
notes-deleted-forever = { $count ->
   *[other] ลบโน้ต { $count } รายการถาวรแล้ว
}
