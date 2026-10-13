# Katna Mail, Thai (ไทย).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The snooze menu (the clock on a line, or Snooze in the right-click menu)

snooze-until = เลื่อนเวลาไว้จนถึง…
snooze-later-today = ภายหลังวันนี้
snooze-tomorrow = พรุ่งนี้
snooze-this-weekend = สุดสัปดาห์นี้
snooze-next-week = สัปดาห์หน้า
snooze-pick = เลือกวันที่และเวลา
snooze-back = กลับไปที่เวลา
snooze-type-placeholder = พิมพ์เวลา
snooze-type-hint = เช่น “อังคาร บ่าย 3 โมง” “พรุ่งนี้” หรือ “อีก 2 ชั่วโมง”
snooze-type-hint-unclear = Katna อ่านข้อความนั้นเป็นเวลาไม่ได้
snooze-type-unclear = “{ $text }” ไม่ใช่เวลาที่ Katna รู้จัก

## Snooze and Remind me on mail: one menu, switched at its top (B and H)

snooze-tab = เลื่อนเวลา
remind-tab = เตือนฉัน
snooze-says = ซ่อนไว้จนถึงเวลานั้น
remind-says = เก็บไว้ที่เดิมและแจ้งเตือนคุณ
remind-before-due = ก่อนถึงกำหนด
remind-note = โน้ต (ไม่บังคับ)
remind-note-placeholder = หัวเรื่อง หากเว้นว่างไว้
toast-remind-set = ตั้งการช่วยเตือนไว้ที่ { $date }
remind-chat-line = เตือน { $date } · { $title }
remind-done = เสร็จแล้ว
toast-remind-done = การช่วยเตือนเสร็จแล้ว
snooze-chat-line = เลื่อนเวลาไว้จนถึง { $date }
snooze-chat-change = เปลี่ยน

## The date and time picker

snooze-cancel = ยกเลิก
snooze-save = บันทึก
snooze-in-the-past = เลือกเวลาที่ช้ากว่าตอนนี้

## beside Send

follow-up-menu = ติดตามหากไม่มีการตอบกลับ…
follow-up-title = ติดตามหากไม่มีการตอบกลับ
follow-up-off = ปิด
follow-up-days = { $days ->
   *[other] { $days } วัน
}
follow-up-weeks = { $weeks ->
   *[other] { $weeks } สัปดาห์
}
follow-up-pick = เลือก…
follow-up-pick-title = ติดตามหากไม่มีการตอบกลับภายใน
follow-up-remind = เตือนฉัน
follow-up-remind-note = การสนทนาจะกลับไปอยู่บนสุดของกล่องจดหมาย
follow-up-send = ส่งข้อความติดตามแทนฉัน
follow-up-send-note = ถึงคนกลุ่มเดิม ในการสนทนาเดิม
follow-up-send-encrypted = ใช้กับอีเมลที่เข้ารหัสไม่ได้
follow-up-text-placeholder = สิ่งที่จะเขียน
follow-up-text-named = สวัสดี { $name } ขอสอบถามว่าได้เห็นข้อความของฉันด้านล่างแล้วหรือยัง
follow-up-text = สวัสดี ขอสอบถามว่าได้เห็นข้อความของฉันด้านล่างแล้วหรือยัง
follow-up-template = ใช้เทมเพลต
follow-up-signature = เพิ่มลายเซ็นของคุณแล้ว
follow-up-again = หากยังไม่มีการตอบกลับ ให้ติดตามอีกครั้งหลังจาก
follow-up-note = จะหยุดทันทีที่มีคนในการสนทนาตอบกลับ การตอบกลับอัตโนมัติไม่นับ
follow-up-note-send = จะหยุดทันทีที่มีคนในการสนทนาตอบกลับ ส่งในวันธรรมดาตั้งแต่ { $start } ถึง { $end } และไม่ช้ากว่ากำหนดเกินหนึ่งวัน
follow-up-cancel = ยกเลิก
follow-up-done = เสร็จสิ้น
follow-up-chip-send = ติดตามในอีก { $time }
follow-up-chip-remind = เตือนในอีก { $time }
follow-up-chip-send-on = ติดตาม { $date }
follow-up-chip-remind-on = เตือน { $date }

## The card over an open conversation a follow-up waits on.

follow-up-card-title = ยังไม่มีการตอบกลับ
follow-up-card-title-waiting = ข้อความติดตามของคุณรออยู่
follow-up-card-send = Katna จะส่งข้อความติดตามของคุณใน { $date } และจะหยุดเมื่อมีคนตอบกลับ
follow-up-card-send-twice = Katna จะส่งข้อความติดตามของคุณใน { $date } แล้วส่งอีกครั้งภายหลัง และจะหยุดเมื่อมีคนตอบกลับ
follow-up-card-remind = หากไม่มีใครตอบกลับ การสนทนานี้จะกลับมาที่กล่องจดหมายของคุณใน { $date }
follow-up-card-waiting = ถึงกำหนดขณะที่คอมพิวเตอร์ของคุณปิดอยู่ จึงไม่ได้ส่งช้ากว่ากำหนด ส่งตอนนี้ เลือกเวลาใหม่ หรือหยุดไว้
follow-up-card-edit = แก้ไข
follow-up-card-edit-title = ติดตามเมื่อ
follow-up-card-send-now = ส่งเลย
follow-up-card-stop = หยุด
follow-up-chat-send = ติดตาม · { $date } หากไม่มีใครตอบกลับ
follow-up-chat-step = ติดตามครั้งที่ { $step } จาก { $steps } · { $date } หากไม่มีใครตอบกลับ
follow-up-chat-waiting = การติดตามรออยู่ · ถึงกำหนดขณะที่คอมพิวเตอร์ของคุณปิดอยู่
follow-up-chat-remind = กลับมาที่กล่องจดหมาย { $date } หากไม่มีการตอบกลับ
toast-follow-up-sent = ส่งข้อความติดตามแล้ว
toast-follow-up-stopped = หยุดการติดตามแล้ว
toast-follow-up-moved = ย้ายการติดตามไปที่ { $date } แล้ว
nudge-row = ส่งเมื่อ { $days ->
   *[other] { $days } วันที่แล้ว
} ติดตามไหม
nudge-row-tip = เขียนข้อความติดตามถึงทุกคนในการสนทนา
nudge-follow-up = ติดตาม
nudge-dismiss = ปิด
nudge-card-title = ยังไม่มีการตอบกลับ
nudge-card-text = คุณถามบางอย่างไว้เมื่อ { $days ->
   *[other] { $days } วันที่แล้ว
} และยังไม่มีใครตอบ
nudge-chat-line = ส่งเมื่อ { $days ->
   *[other] { $days } วันที่แล้ว
} ยังไม่มีการตอบกลับ
toast-nudge-dismissed = ปิดการกระตุ้นเตือนแล้ว
