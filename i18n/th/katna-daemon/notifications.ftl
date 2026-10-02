# Katna Mail, Thai (ไทย).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification (the desktop shows it, even with Katna Mail closed)

notify-new-emails = อีเมลใหม่ { $count } ฉบับ
notify-and-more = และอีก { $count } ฉบับ
notify-no-subject = (ไม่มีหัวเรื่อง)
notify-unknown-sender = ไม่ทราบผู้ส่ง

## Reminders the user asked for (same buttons)

notify-snooze-back = กลับมาจากการเลื่อนเวลา
notify-no-reply = ยังไม่มีการตอบกลับ
notify-no-reply-to = ยังไม่มีใครตอบกลับ “{ $subject }”

## Open and click tracking (only for mail sent with "Track opens and clicks")

notify-tracking-opened = { $who } เปิด { $subject } แล้ว
notify-tracking-clicked = { $who } คลิกลิงก์ใน { $subject }

## An update of Katna is downloaded and ready to install

notify-update-ready = อัปเดต Katna Mail ได้แล้ว
notify-update-ready-body = ดาวน์โหลดเวอร์ชัน { $version } เสร็จแล้ว กดอัปเดตเพื่อติดตั้งและรีสตาร์ท Katna Mail
notify-update = อัปเดต

## Reminders of calendar events

notify-event-now = ตอนนี้
notify-event-in-minutes = { $count ->
   *[other] ในอีก { $count } นาที
}
notify-event-in-hours = { $count ->
   *[other] ในอีก { $count } ชั่วโมง
}
notify-event-in-days = { $count ->
    [1] พรุ่งนี้
   *[other] ในอีก { $count } วัน
}
notify-event-all-day = ทั้งวัน
notify-event-join = เข้าร่วม
notify-event-snooze = เลื่อนเวลา 5 นาที
notify-task-done = ทำเครื่องหมายว่าเสร็จแล้ว

## The buttons of new-mail notifications and reminders

notify-open = เปิด
notify-peek = ดูเพิ่ม
notify-reply = ตอบกลับ
notify-reply-placeholder = ตอบกลับ { $name }…
notify-send = ส่ง
notify-reply-all = ตอบกลับทั้งหมด
notify-mark-read = ทำเครื่องหมายว่าอ่านแล้ว
notify-mark-all-read = ทำเครื่องหมายทั้งหมดว่าอ่านแล้ว
notify-archive = เก็บถาวร

## After Archive on a notification: a short note in the same place

notify-archived = เก็บถาวรแล้ว
notify-archived-count = { $count ->
   *[other] ย้ายข้อความ { $count } รายการออกจากกล่องจดหมายแล้ว
}
notify-undo = เลิกทำ

## it waits for the undo time

notify-reply-sent = ส่งคำตอบถึง { $name } แล้ว
notify-open-in-katna = เปิดใน Katna
