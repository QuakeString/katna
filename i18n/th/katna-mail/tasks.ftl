# Katna Mail, Thai (ไทย): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = งานใหม่
tasks-all = งานทั้งหมด
tasks-today = วันนี้
tasks-upcoming = กำลังจะถึง
tasks-starred = ติดดาว
tasks-completed-view = เสร็จแล้ว
tasks-new-list = สร้างรายการใหม่
tasks-labels-heading = ป้ายกำกับ
tasks-on-this-computer = ในคอมพิวเตอร์เครื่องนี้
tasks-my-tasks = งานของฉัน
tasks-account-sign-in = ลงชื่อเข้าใช้อีกครั้งเพื่อแสดงงาน
tasks-account-signed-in = ลงชื่อเข้าใช้ { $address } อีกครั้งแล้ว กำลังดึงงานของคุณ…
tasks-account-sign-in-refused = { $provider } ไม่อนุญาตให้ Katna เข้าใช้ ลองอีกครั้ง และอนุญาตให้เข้าถึงงานของคุณ
tasks-account-refused = เซิร์ฟเวอร์ไม่ยอมรับรหัสผ่าน Yahoo, iCloud, Zoho และอื่นๆ ต้องใช้รหัสผ่านสำหรับแอป
tasks-account-change-password = เปลี่ยนรหัสผ่าน
tasks-account-change-password-tooltip = พิมพ์รหัสผ่านใหม่ แล้ว Katna จะตรวจสอบกับเซิร์ฟเวอร์
tasks-account-not-enabled = ยังไม่ได้เปิดการเข้าถึงงานสำหรับ Katna
tasks-account-failed = อ่านรายการงานไม่ได้
tasks-account-error = อ่านรายการงานไม่ได้: { $reason }
tasks-account-none = ไม่พบรายการงาน
tasks-account-none-why = ไม่พบรายการงาน: { $reason }
tasks-account-use-sign-in = { $provider } จะแสดงงานให้เฉพาะ Katna ที่ลงชื่อเข้าใช้ด้วย { $provider } เท่านั้น
tasks-account-sign-in-with = ลงชื่อเข้าใช้ด้วย { $provider }
tasks-account-looking = กำลังค้นหารายการงาน…
tasks-account-try-again = ลองอีกครั้ง
tasks-account-try-again-tooltip = ตรวจสอบงานของบัญชีนี้อีกครั้งตอนนี้
tasks-account-fixing = กำลังดำเนินการ…
tasks-list-name-placeholder = ชื่อรายการ

## Lists and tasks

tasks-loading = กำลังอ่านงานของคุณ…
tasks-no-lists = รายการงานของคุณจะแสดงที่นี่
tasks-search = ค้นหางาน
tasks-search-none = ไม่มีงานที่ตรงกับการค้นหา
tasks-add = เพิ่มงาน
tasks-title-placeholder = ชื่อ
tasks-add-step = เพิ่มงานย่อย
tasks-empty = ยังไม่มีงาน เพิ่มได้ด้านบน
tasks-starred-empty = ติดดาวงานเพื่อดูที่นี่
tasks-label-empty = ไม่มีงานที่ยังเปิดอยู่ซึ่งมีป้ายกำกับนี้
tasks-today-empty = ไม่มีงานที่ครบกำหนดวันนี้
tasks-completed-empty = งานที่คุณทำเสร็จจะแสดงที่นี่
tasks-upcoming-add = เพิ่มงานสำหรับ{ $day }
tasks-upcoming-overdue-day = { $weekday } { $day }
tasks-from-mail-quiet = จากอีเมล
tasks-from-note-quiet = จากโน้ต
tasks-steps-done = { $done }/{ $count }
tasks-today-date = { $weekday } { $day }
tasks-overdue = เลยกำหนด
tasks-completed = { $count ->
   *[other] เสร็จแล้ว ({ $count })
}
tasks-list-options = ตัวเลือกรายการ
tasks-sort-by = จัดเรียงตาม
tasks-sort-my-order = ลำดับของฉัน
tasks-sort-date = วันที่
tasks-sort-starred = ติดดาวล่าสุด
tasks-sort-title = ชื่อ
tasks-rename-list = เปลี่ยนชื่อรายการ
tasks-delete-list = ลบรายการ
tasks-mark-done = ทำเครื่องหมายว่าเสร็จแล้ว
tasks-mark-open = ทำเครื่องหมายว่ายังไม่เสร็จ
tasks-star = ติดดาว
tasks-unstar = เอาดาวออก
tasks-edit-title = แก้ไขชื่อ
tasks-details = รายละเอียด
tasks-delete = ลบ
tasks-move-to = ย้ายไปที่ { $list }
tasks-from-mail = อีเมล
tasks-open-mail = เปิดอีเมล
tasks-from-note = โน้ต
tasks-open-note = เปิดโน้ต
tasks-note-gone = ไม่มีโน้ตนั้นแล้ว
tasks-no-subject = (ไม่มีหัวเรื่อง)

## Several tasks selected (Ctrl+click, Shift+click)

tasks-selected = { $count ->
   *[other] เลือก { $count } รายการ
}
tasks-select-clear = ล้างการเลือก
tasks-select-move = ย้ายไปที่รายการ
tasks-select-date = ตั้งวันที่
tasks-next-week = สัปดาห์หน้า

## The details dialog

tasks-notes-placeholder = เพิ่มรายละเอียด
tasks-date = วันที่
tasks-no-date = ไม่มีวันที่
tasks-time-placeholder = เพิ่มเวลา
tasks-repeat = ทำซ้ำ
tasks-repeat-never = ไม่ทำซ้ำ
tasks-repeat-daily = ทุกวัน
tasks-repeat-weekly = ทุกสัปดาห์
tasks-repeat-monthly = ทุกเดือน
tasks-repeat-yearly = ทุกปี
tasks-repeat-other = กำหนดเอง
tasks-remind = เตือนฉัน
tasks-remind-off = ไม่ต้องเตือน
tasks-remind-on-time = ตามเวลา
tasks-remind-morning = ในวันนั้น { $time }
tasks-remind-hour-before = 1 ชั่วโมงก่อน
tasks-remind-day-before = 1 วันก่อน
tasks-label-add = เพิ่มป้ายกำกับ
tasks-label-task = ติดป้ายกำกับงาน
tasks-files-attach = แนบไฟล์
tasks-files-pick = แนบ
tasks-file-open = เปิด
tasks-file-remove = นำไฟล์ออก
tasks-file-here = เฉพาะในคอมพิวเตอร์เครื่องนี้
tasks-cancel = ยกเลิก
tasks-save = บันทึก
tasks-not-a-time = “{ $text }” ไม่ใช่เวลา ตัวอย่างเช่น { $example }

## Due days

tasks-due-today = วันนี้
tasks-due-tomorrow = พรุ่งนี้
tasks-due-yesterday = เมื่อวาน
tasks-due-at = { $day }, { $time }

## Notes at the bottom

tasks-toast-done = ทำงานเสร็จแล้ว
tasks-toast-next = เสร็จแล้ว งานถัดไปวันที่ { $date }
tasks-toast-deleted = ลบงานแล้ว
tasks-files-added = { $count ->
   *[other] แนบไฟล์ { $count } ไฟล์แล้ว
}
tasks-file-removed = นำ “{ $name }” ออกแล้ว
tasks-files-left-out = ไม่ได้แนบ: { $names } งานรับไฟล์ได้ไม่เกิน { $limit } และรับโฟลเดอร์ไม่ได้
tasks-file-missing = ไฟล์นั้นไม่อยู่ที่นี่แล้ว
tasks-toast-added = { $count ->
   *[other] เพิ่ม { $count } งานแล้ว
}
tasks-mail-gone = ไม่มีอีเมลนั้นแล้ว
tasks-toast-list-deleted = ลบรายการแล้ว
tasks-toast-moved = ย้ายไปที่ { $list } แล้ว
tasks-toast-placed = ย้ายงานแล้ว
tasks-toast-rescheduled = เลื่อนเวลางานแล้ว
tasks-toast-rescheduled-several = { $count ->
   *[other] เปลี่ยนกำหนดการงาน { $count } รายการแล้ว
}
tasks-toast-done-several = { $count ->
   *[other] ทำงาน { $count } รายการเสร็จแล้ว
}
tasks-toast-open-several = { $count ->
   *[other] ทำเครื่องหมายงาน { $count } รายการว่ายังไม่เสร็จแล้ว
}
tasks-toast-starred = { $count ->
   *[other] ติดดาวงาน { $count } รายการแล้ว
}
tasks-toast-unstarred = { $count ->
   *[other] นำดาวออกจากงาน { $count } รายการแล้ว
}
tasks-toast-deleted-several = { $count ->
   *[other] ลบงาน { $count } รายการแล้ว
}
