# Katna Mail, Thai (ไทย).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Folders & rules

settings-rules = กฎ
settings-rules-summary = จัดเรียง ติดป้ายกำกับ ส่งต่อ หรือปิดเสียงอีเมลใหม่โดยอัตโนมัติ
settings-rules-intro = กฎจะจัดเรียงอีเมลใหม่โดยอัตโนมัติตามลำดับนี้ ลากเพื่อจัดลำดับใหม่
settings-rules-all-accounts = ทุกบัญชี
settings-rules-new = กฎใหม่
settings-rules-none = ยังไม่มีกฎ กฎจะจัดเรียงอีเมลใหม่โดยอัตโนมัติตามผู้ส่ง หัวเรื่อง หรือคำ
settings-rules-none-account = ยังไม่มีกฎสำหรับบัญชีนี้
settings-rules-drag = ลากเพื่อจัดลำดับใหม่
settings-rules-edit = แก้ไขกฎ
settings-rules-turn-off = ปิดกฎนี้
settings-rules-turn-on = เปิดกฎนี้

## Turning one on makes it one of the user's rules.

settings-rules-starters = กฎสำเร็จรูป
settings-rules-starters-intro = ปิดอยู่จนกว่าคุณจะเปิด ใช้ได้กับทุกบัญชีของคุณ แก้ไขกฎเพื่อเปลี่ยนการทำงาน
settings-rules-starter-turning-on = กำลังเปิด “{ $name }”…
settings-rules-starter-failed = เปิด “{ $name }” ไม่ได้: { $error }
rules-starter-promotions = ปิดเสียงโปรโมชัน
rules-starter-newsletters = จดหมายข่าวไปที่อ่านภายหลัง
rules-starter-receipts = ใบเสร็จและใบแจ้งหนี้
rules-starter-deliveries = การจัดส่ง
rules-starter-train = ตั๋วรถไฟ
rules-starter-flight = ตั๋วเครื่องบิน
rules-starter-codes = รหัสแบบใช้ครั้งเดียว
rules-starter-security = การแจ้งเตือนด้านความปลอดภัย
rules-starter-social = อีเมลโซเชียล
rules-starter-invites = คำเชิญในปฏิทิน
rules-starter-folder-reading = อ่านภายหลัง
rules-starter-folder-receipts = ใบเสร็จ
rules-starter-folder-deliveries = การจัดส่ง
rules-starter-folder-travel = การเดินทาง
rules-starter-folder-social = โซเชียล
rules-runs-katna = ทำงานใน Katna
rules-runs-gmail = ทำงานบน Gmail
rules-runs-sieve = ทำงานบนเซิร์ฟเวอร์
rules-stopped = หยุดทำงาน
rules-error-folder-gone = โฟลเดอร์ที่กฎนี้ใช้ไม่มีอยู่แล้ว แก้ไขกฎเพื่อเลือกโฟลเดอร์อื่น
rules-error-no-archive = บัญชีนี้ไม่มีโฟลเดอร์เก็บถาวร แก้ไขกฎเพื่อให้ทำอย่างอื่น
rules-error-no-trash = บัญชีนี้ไม่มีโฟลเดอร์ถังขยะ แก้ไขกฎเพื่อให้ทำอย่างอื่น
rules-error-cannot-send = บัญชีนี้ส่งอีเมลไม่ได้ กฎจึงส่งต่ออีเมลไม่ได้
rules-error-other = { $error } แก้ไขกฎแล้วเปิดใช้อีกครั้ง
settings-folders = โฟลเดอร์
settings-folders-summary = จำนวนที่ยังไม่อ่านในบานหน้าต่างโฟลเดอร์
settings-folders-unread-counts = จำนวนที่ยังไม่อ่านในทุกโฟลเดอร์
settings-folders-unread-counts-detail = ปิด: เฉพาะกล่องจดหมายที่แสดงจำนวนที่ยังไม่อ่าน

## inbox, label Reading".

rules-summary = { $when } → { $then }
rules-summary-and = { $first } และ { $next }
rules-summary-or = { $first } หรือ { $next }
rules-summary-more = อีก { $count } คำ
rules-summary-list = { $first }, { $next }
rules-summary-condition = { $field } { $comparator } { $value }
rules-summary-has-attachment = มีไฟล์แนบ
rules-summary-no-attachment = ไม่มีไฟล์แนบ
rules-summary-mailing-list = มาจากรายชื่ออีเมล
rules-summary-not-mailing-list = ไม่ได้มาจากรายชื่ออีเมล
rules-summary-tab = อยู่ในแท็บ{ $tab }
rules-summary-not-tab = ไม่ได้อยู่ในแท็บ{ $tab }
rules-summary-move = ย้ายไปที่ { $folder }
rules-summary-archive = ข้ามกล่องจดหมาย
rules-summary-trash = ย้ายไปที่ถังขยะ
rules-summary-mark-read = ทำเครื่องหมายว่าอ่านแล้ว
rules-summary-star = ติดดาว
rules-summary-important = ทำเครื่องหมายว่าสำคัญ
rules-summary-label = ติดป้ายกำกับ { $label }
rules-summary-forward = ส่งต่อไปที่ { $address }
rules-summary-dont-notify = ไม่ต้องแจ้งเตือน
rules-summary-read-after = { $count ->
   *[other] ทำเครื่องหมายว่าอ่านแล้วหลังจาก { $count } วัน
}
rules-summary-folder-gone = โฟลเดอร์ที่ไม่มีอยู่แล้ว

## The rule editor

rules-editor-new-title = กฎใหม่
rules-editor-edit-title = แก้ไขกฎ
rules-editor-name-hint = ชื่อกฎ
rules-editor-when = เมื่ออีเมลใหม่ตรงกับ
rules-editor-of-these = ของเงื่อนไขเหล่านี้:
rules-mode-all = ทั้งหมด
rules-mode-any = ข้อใดข้อหนึ่ง
rules-field-from = จาก
rules-field-to = ถึง
rules-field-cc = สำเนา
rules-field-any-recipient = ถึงหรือสำเนา
rules-field-reply-to = ตอบกลับไปที่
rules-field-subject = หัวเรื่อง
rules-field-body = ข้อความ
rules-field-attachment-name = ชื่อไฟล์แนบ
rules-field-has-attachment = มีไฟล์แนบ
rules-field-mailing-list = มาจากรายชื่ออีเมล
rules-field-tab = แท็บกล่องจดหมาย
rules-comparator-contains = มีคำว่า
rules-comparator-not-contains = ไม่มีคำว่า
rules-comparator-begins-with = ขึ้นต้นด้วย
rules-comparator-ends-with = ลงท้ายด้วย
rules-comparator-equals = ตรงกับ
rules-comparator-matches = ตรงกับรูปแบบ
rules-has-yes = ใช่
rules-has-no = ไม่
rules-editor-value-hint = คำหรือที่อยู่อีเมล
rules-editor-add-condition = เพิ่มเงื่อนไข
rules-editor-remove = นำออก
rules-editor-then = จากนั้น:
rules-action-move = ย้ายไปที่
rules-action-archive = ข้ามกล่องจดหมาย (เก็บถาวร)
rules-action-trash = ย้ายไปที่ถังขยะ
rules-action-mark-read = ทำเครื่องหมายว่าอ่านแล้ว
rules-action-star = ติดดาว
rules-action-important = ทำเครื่องหมายว่าสำคัญ
rules-action-label = เพิ่มป้ายกำกับ
rules-action-forward = ส่งต่อไปที่
rules-action-dont-notify = ไม่ต้องแจ้งเตือน
rules-action-read-after = ทำเครื่องหมายว่าอ่านแล้วหลังจาก
rules-editor-choose-folder = เลือกโฟลเดอร์
rules-editor-choose-label = เลือกป้ายกำกับ
rules-editor-new-folder = ใหม่: { $name }
rules-editor-folder-of = { $folder } ({ $account })
rules-editor-forward-hint = ที่อยู่อีเมล
rules-editor-days = วัน
rules-editor-add-action = เพิ่มการดำเนินการ
rules-editor-stop = หยุดที่นี่: กฎถัดไปจะไม่ทำงานกับอีเมลนี้
rules-editor-accounts = บัญชี:
rules-editor-accounts-none = เลือกบัญชี
rules-editor-accounts-many = { $count ->
   *[other] { $count } บัญชี
}
rules-editor-matches = ตรงกับ { $mails } จาก { $days } วันที่ผ่านมา
rules-editor-mails = { $count ->
   *[other] อีเมล { $count } ฉบับ
}
rules-editor-counting = กำลังนับอีเมลที่ตรงกัน…
rules-editor-show = แสดงอีเมลเหล่านี้
rules-editor-also-apply = ใช้กับ { $count } ฉบับนี้ด้วย
rules-editor-runs-katna = ทำงานใน Katna ขณะที่คอมพิวเตอร์เครื่องนี้เปิดอยู่
rules-editor-runs-gmail = ทำงานบน Gmail จึงใช้ได้บนโทรศัพท์ของคุณและเมื่อปิดคอมพิวเตอร์เครื่องนี้ด้วย
rules-editor-runs-sieve = ทำงานบนเซิร์ฟเวอร์อีเมลของคุณ จึงใช้ได้บนโทรศัพท์ของคุณและเมื่อปิดคอมพิวเตอร์เครื่องนี้ด้วย
rules-note-gmail-action = ทำงานใน Katna: ตัวกรองของ Gmail ทำ “{ $action }” ไม่ได้
rules-note-sieve-action = ทำงานใน Katna: กฎของเซิร์ฟเวอร์อีเมลของคุณทำ “{ $action }” ไม่ได้
rules-note-test = { $field } { $comparator }
rules-note-gmail-condition = ทำงานใน Katna: ตัวกรองของ Gmail ตรวจสอบ “{ $test }” แบบที่ Katna ทำไม่ได้
rules-note-sieve-condition = ทำงานใน Katna: กฎของเซิร์ฟเวอร์อีเมลของคุณตรวจสอบ “{ $test }” แบบที่ Katna ทำไม่ได้
rules-note-order = ทำงานใน Katna เหมือนกฎก่อนหน้าของบัญชีนี้ เพราะกฎจะทำงานตามลำดับในรายการ
rules-note-gmail-stop = ทำงานใน Katna: ตัวกรองของ Gmail หยุดกฎถัดไปไม่ให้ทำงานไม่ได้
rules-note-gmail-forward = ทำงานใน Katna: Gmail ส่งต่อได้เฉพาะที่อยู่ที่ยืนยันแล้วในการตั้งค่า และ { $address } ไม่ใช่หนึ่งในนั้น
rules-note-gmail-folder = ทำงานใน Katna: Gmail ไม่มีป้ายกำกับสำหรับโฟลเดอร์ที่กฎนี้ใช้
rules-note-sieve-folder = ทำงานใน Katna: เซิร์ฟเวอร์อีเมลของคุณไม่มีโฟลเดอร์ที่กฎนี้ใช้
rules-note-gmail-sign-in = ทำงานใน Katna จนกว่าคุณจะลงชื่อเข้าใช้ Google อีกครั้งและอนุญาตให้ Katna สร้างตัวกรอง Gmail
rules-note-sieve-other-script = ทำงานใน Katna: มีสคริปต์กฎอื่น (“{ $name }”) ทำงานอยู่บนเซิร์ฟเวอร์อีเมลของคุณ
rules-note-gmail-failed = ทำงานใน Katna: Gmail ไม่รับกฎนี้ ({ $error })
rules-note-sieve-failed = ทำงานใน Katna: เซิร์ฟเวอร์อีเมลของคุณไม่รับกฎนี้ ({ $error })
rules-editor-cancel = ยกเลิก
rules-editor-save = บันทึก
rules-editor-saving = กำลังบันทึก…
rules-editor-delete = ลบกฎ
rules-editor-delete-ask = ลบกฎนี้ไหม
rules-editor-delete-keep = เก็บไว้
rules-editor-delete-confirm = ลบ
rules-editor-needs-folder = เลือกโฟลเดอร์สำหรับ “ย้ายไปที่” แต่ละรายการ และป้ายกำกับสำหรับ “เพิ่มป้ายกำกับ” แต่ละรายการ
rules-editor-needs-days = “ทำเครื่องหมายว่าอ่านแล้วหลังจาก” ต้องระบุจำนวนวันตั้งแต่ 1 ถึง 3650
rules-saved = บันทึกกฎแล้ว
rules-saved-applied = { $count ->
   *[other] บันทึกกฎและใช้กับอีเมล { $count } ฉบับแล้ว
}
rules-apply-failed = บันทึกกฎแล้ว แต่ใช้กฎไม่สำเร็จ: { $error }
rules-deleted = ลบกฎแล้ว
rules-delete-failed = ลบกฎไม่ได้: { $error }
rules-change-failed = เปลี่ยนกฎไม่ได้: { $error }
