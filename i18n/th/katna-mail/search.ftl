# Katna Mail, Thai (ไทย).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Search options (the panel from the button at the right of the search box)

search-options = ตัวเลือกการค้นหา
search-options-close = ปิด
search-from = จาก
search-to = ถึง
search-subject = หัวเรื่อง
search-has-words = มีคำว่า
search-without = ไม่มีคำว่า
search-date-within = วันที่ภายใน
search-has-attachment = มีไฟล์แนบ
search-attachment-custom = กำหนดเอง
search-attachment-custom-hint = พิมพ์นามสกุลไฟล์ เช่น png แล้วกดเว้นวรรค
search-attachment-remove = นำออก
search-clear-filter = ล้างตัวกรอง

## Search options: "Date within" choices

search-within-any = เวลาใดก็ได้
search-within-days = { $count ->
   *[other] { $count } วัน
}
search-within-weeks = { $count ->
   *[other] { $count } สัปดาห์
}
search-within-months = { $count ->
   *[other] { $count } เดือน
}
search-within-years = { $count ->
   *[other] { $count } ปี
}
search-within-custom = กำหนดเอง

## Search options: custom dates (the calendar popover)

search-dates-on = ในวันที่
search-dates-before = ก่อน
search-dates-since = ตั้งแต่
search-dates-between = ระหว่าง
search-dates-from = จาก
search-dates-to = ถึง
search-dates-placeholder = ปปปป-ดด-วว
search-dates-missing = เลือกวันที่
search-dates-unreadable = ใช้วันที่ เช่น 2026-09-01
search-dates-out-of-range = วันที่นั้นอยู่นอกช่วง
search-dates-chip-before = ก่อน { $date }
search-dates-chip-since = ตั้งแต่ { $date }
search-dates-chip-between = { $first } – { $last }
search-dates-cancel = ยกเลิก
search-dates-done = เสร็จสิ้น
search-dates-month-back = เดือนก่อนหน้า
search-dates-month-on = เดือนถัดไป
search-dates-year-back = ปีก่อนหน้า
search-dates-year-on = ปีถัดไป
