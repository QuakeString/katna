# Katna Mail, Thai (ไทย).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = เซิร์ฟเวอร์อีเมล
problems-signed-out = { $provider } ให้ Katna ออกจากระบบ { $address } อีเมลหยุดซิงค์แล้ว
problems-password-refused = { $provider } ปฏิเสธรหัสผ่านของ { $address } รหัสผ่านอาจเปลี่ยนไปแล้ว
problems-no-answer = { $provider } ไม่ตอบสนองสำหรับ { $address } Katna จะลองต่อไป
problems-offline = คุณออฟไลน์อยู่ อีเมลของคุณยังอยู่ที่นี่ และอีเมลที่คุณส่งจะรอจนกว่าคุณจะกลับมาออนไลน์
problems-accounts-need-you = { $count ->
   *[other] { $count } บัญชีต้องให้คุณดำเนินการ
}
problems-show = แสดง
problems-later = ภายหลัง
problems-new-password = รหัสผ่านใหม่
problems-try-again = ลองอีกครั้ง

## The New password card

problems-password-title = รหัสผ่านใหม่
problems-password-detail = { $provider } ปฏิเสธรหัสผ่านที่บันทึกไว้ของ { $address } พิมพ์รหัสผ่านใหม่ แล้ว Katna จะตรวจสอบก่อนบันทึก
problems-password-placeholder = รหัสผ่าน
problems-password-show = แสดงรหัสผ่าน
problems-password-hide = ซ่อนรหัสผ่าน
problems-password-cancel = ยกเลิก
problems-password-save = บันทึก
problems-password-checking = กำลังตรวจสอบ…
problems-password-refused-again = { $provider } ปฏิเสธรหัสผ่านนี้เช่นกัน โปรดตรวจสอบแล้วลองอีกครั้ง
problems-password-saved = บันทึกรหัสผ่านของ { $address } แล้ว กำลังรับอีเมลของคุณ…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = เซิร์ฟเวอร์อีเมลของ { $address } ไม่ยอมรับการย้าย{ $count ->
   *[other] ข้อความ { $count } รายการ จึงกลับไปอยู่ที่เดิม
}
problems-refused-flags = เซิร์ฟเวอร์อีเมลของ { $address } ไม่ยอมรับการทำเครื่องหมาย{ $count ->
   *[other] ข้อความ { $count } รายการ (อ่านแล้ว ติดดาว…) จึงกลับเป็นเหมือนเดิม
}
problems-refused-label = เซิร์ฟเวอร์อีเมลของ { $address } ไม่ยอมรับการเปลี่ยนป้ายกำกับของ{ $count ->
   *[other] ข้อความ { $count } รายการ จึงกลับเป็นเหมือนเดิม
}
problems-refused-delete = เซิร์ฟเวอร์อีเมลของ { $address } ไม่ยอมรับการลบ{ $count ->
   *[other] ข้อความ { $count } รายการ จึงกลับมาเหมือนเดิม
}
problems-refused-other = เซิร์ฟเวอร์อีเมลของ { $address } ไม่ยอมรับ{ $count ->
   *[other] การเปลี่ยนแปลง { $count } รายการ Katna จึงเปลี่ยนกลับเป็นเหมือนเดิม
}
problems-details = รายละเอียด

## Katna's background service (katna-daemon) isn't running

service-starting = กำลังเริ่มบริการเบื้องหลังของ Katna…
service-failed = บริการเบื้องหลังของ Katna เริ่มทำงานไม่ได้ อีเมลจึงไม่ซิงค์
service-start-again = เริ่มอีกครั้ง
service-started-again = บริการเบื้องหลังของ Katna หยุดทำงานและได้เริ่มใหม่อีกครั้งแล้ว
service-details-title = สาเหตุที่บริการเริ่มทำงานไม่ได้
service-details-body = คัดลอกข้อความนี้แล้วส่งไปพร้อมรายงานของคุณ ไม่มีอีเมลหรือรหัสผ่านอยู่ในนี้
service-details-copy = คัดลอก
service-details-close = ปิด
service-not-running = บริการเบื้องหลังของ Katna ไม่ได้ทำงานอยู่
service-no-answer = บริการเบื้องหลังของ Katna ไม่ตอบสนอง: { $error }
service-no-session = ไม่มีเซสชัน D-Bus: { $error }

## Safe mode: an update left Katna's background service unable to start

safe-line = Katna อยู่ในโหมดปลอดภัยหลังเกิดปัญหากับการอัปเดต อีเมลจึงไม่ซิงค์
safe-try-again = ลองอีกครั้ง
safe-restore = กู้คืน
safe-restoring = กำลังกู้คืนข้อมูลของคุณจาก { $when }…
safe-restored = กู้คืนข้อมูลของคุณจาก { $when } แล้ว ข้อมูลที่มีอยู่ก่อนหน้าเก็บไว้ในโฟลเดอร์
safe-show-folder = แสดงโฟลเดอร์
safe-restore-failed = กู้คืนข้อมูลของคุณไม่ได้: { $error }
safe-restore-title = กู้คืนข้อมูลของคุณจากก่อนการอัปเดตหรือไม่?
safe-restore-body = Katna จะกลับไปใช้สำเนาที่คุณเลือก อีเมลที่เข้ามาหลังจากนั้นจะดาวน์โหลดใหม่จากบัญชีของคุณ
safe-restore-none = ยังไม่มีสำเนา Katna จะทำสำเนาไว้ก่อนการอัปเดตแต่ละครั้งเปลี่ยนแปลงข้อมูลของคุณ
safe-restore-keep = ข้อมูลที่มีอยู่ตอนนี้ รวมถึงอีเมลที่ยังไม่ได้ส่ง ฉบับร่าง และการเปลี่ยนแปลงที่ยังไม่ซิงค์ จะเก็บไว้ในโฟลเดอร์ก่อน จึงไม่มีอะไรหายไป
safe-restore-cancel = ยกเลิก
safe-restore-mail = อีเมล
safe-restore-pim = บัญชีและรายชื่อติดต่อ
safe-restore-blobs = ไฟล์แนบ
safe-report-title = รายงานดีบัก
safe-report-body = คัดลอกข้อความนี้แล้วแนบไปกับรายงานบั๊กของคุณ ไม่มีอีเมล ที่อยู่ หรือรหัสผ่านอยู่ในนี้
safe-report-restore = กู้คืน…
safe-report-copied = คัดลอกรายงานดีบักแล้ว
