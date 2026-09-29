# Katna Mail, Thai (ไทย): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = สร้าง
tasks-all = งานทั้งหมด
tasks-today = วันนี้
tasks-starred = ติดดาว
tasks-new-list = สร้างรายการใหม่
tasks-on-this-computer = ในคอมพิวเตอร์เครื่องนี้
tasks-my-tasks = งานของฉัน
# The line under an account in the side list whose task lists could not
# come: why, and the one click that fixes it.
tasks-account-sign-in = ลงชื่อเข้าใช้อีกครั้งเพื่อแสดงงาน
tasks-account-signed-in = ลงชื่อเข้าใช้ { $address } อีกครั้งแล้ว กำลังดึงงานของคุณ…
tasks-account-sign-in-refused = { $provider } ไม่อนุญาตให้ Katna เข้าใช้ ลองอีกครั้ง และอนุญาตให้เข้าถึงงานของคุณ
tasks-account-refused = เซิร์ฟเวอร์ไม่ยอมรับรหัสผ่าน Yahoo, iCloud, Zoho และอื่นๆ ต้องใช้รหัสผ่านสำหรับแอป
tasks-account-change-password = เปลี่ยนรหัสผ่าน
tasks-account-change-password-tooltip = เปิด การตั้งค่า > บัญชี
tasks-account-not-enabled = ยังไม่ได้เปิดการเข้าถึงงานสำหรับ Katna
tasks-account-failed = อ่านรายการงานไม่ได้
# $reason is the server's own words, in English.
tasks-account-error = อ่านรายการงานไม่ได้: { $reason }
tasks-account-none = ไม่พบรายการงาน
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
tasks-today-empty = ไม่มีงานที่ครบกำหนดวันนี้
tasks-today-date = { $weekday } { $day }
tasks-overdue = เลยกำหนด
tasks-completed = { $count ->
   *[other] เสร็จแล้ว ({ $count })
}
tasks-list-options = ตัวเลือกรายการ
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
tasks-toast-added = { $count ->
   *[other] เพิ่ม { $count } งานแล้ว
}
tasks-mail-gone = ไม่มีอีเมลนั้นแล้ว
tasks-toast-list-deleted = ลบรายการแล้ว
tasks-toast-moved = ย้ายไปที่ { $list } แล้ว
tasks-toast-rescheduled = เลื่อนเวลางานแล้ว
