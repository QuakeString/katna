# Katna Mail, Thai (ไทย).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Accounts

accounts-folder-pane = บานหน้าต่างโฟลเดอร์
accounts-folder-pane-detail = บัญชีที่จะแสดงโฟลเดอร์ในบานหน้าต่างด้านซ้าย
accounts-shown-one = ทีละบัญชี สลับได้ที่การ์ดบัญชี
accounts-shown-all = ทุกบัญชี เรียงต่อกัน
accounts-row = บัญชี
accounts-row-detail = บานหน้าต่างโฟลเดอร์และเมนูบัญชีจะแสดงบัญชีตามลำดับนี้ บัญชีแรกคือบัญชีเริ่มต้น การนำบัญชีออกจะลบสำเนาอีเมลของบัญชีนั้นที่ Katna เก็บไว้ในคอมพิวเตอร์เครื่องนี้ อีเมลยังคงอยู่บนเซิร์ฟเวอร์
accounts-none = ยังไม่มีบัญชี
accounts-kind-imported = นำเข้า
accounts-picture-reset = ใช้รูปภาพของเดสก์ท็อป
accounts-picture-change = เปลี่ยนรูปภาพ
accounts-picture-remove = นำรูปภาพออก
accounts-rename = เปลี่ยนชื่อ
accounts-name-save = บันทึก
accounts-name-cancel = ยกเลิก
accounts-name-placeholder = ชื่อของคุณ
accounts-rename-failed = เปลี่ยนชื่อบัญชีไม่ได้: { $error }
accounts-move-up = เลื่อนขึ้น
accounts-move-down = เลื่อนลง
accounts-drag = ลากเพื่อเปลี่ยนลำดับ
accounts-remove = นำออก
accounts-delete-all-row = ลบข้อมูลทั้งหมด
accounts-delete-all-row-detail = เริ่มต้นใหม่ เหมือนติดตั้งใหม่
accounts-delete-all-about = ลบทุกบัญชี อีเมลที่เก็บไว้ทั้งหมด รายชื่อติดต่อและปฏิทิน ดัชนีการค้นหา การตั้งค่า และรหัสผ่านที่บันทึกไว้ออกจากคอมพิวเตอร์เครื่องนี้ ไม่มีการเปลี่ยนแปลงใดๆ บนเซิร์ฟเวอร์อีเมลของคุณ
accounts-delete-all-open = ลบข้อมูล Katna ทั้งหมด

## Settings > Accounts: snackbars after deleting

accounts-removed-local = นำ { $address } ออกจาก Katna แล้ว
accounts-removed = นำ { $address } ออกจาก Katna แล้ว อีเมลของบัญชีนี้ยังอยู่บนเซิร์ฟเวอร์
accounts-all-deleted = ลบข้อมูล Katna ทั้งหมดออกจากคอมพิวเตอร์เครื่องนี้แล้ว

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = นำ { $address } ออกไหม
accounts-remove-confirm = นำบัญชีออก
accounts-removing = กำลังนำออก…
accounts-remove-local-mail = { $folders ->
    [0] อีเมลทั้งหมดที่นำเข้ามาในบัญชีนี้
   *[other] อีเมลทั้งหมดที่นำเข้ามาในบัญชีนี้ ใน { $folders } โฟลเดอร์ของบัญชี
}
accounts-remove-local-settings = การตั้งค่า Katna ของบัญชีนี้
accounts-remove-mail = { $folders ->
    [0] อีเมลทั้งหมดของบัญชีนี้ที่ Katna เก็บไว้
   *[other] อีเมลทั้งหมดของบัญชีนี้ที่ Katna เก็บไว้ ใน { $folders } โฟลเดอร์ของบัญชี
}
accounts-remove-outbox = ข้อความของบัญชีนี้ที่รออยู่ในกล่องขาออก
accounts-remove-settings = รหัสผ่านที่บันทึกไว้และการตั้งค่า Katna ของบัญชีนี้
accounts-delete-all-title = ลบข้อมูล Katna ทั้งหมดไหม
accounts-delete-all-confirm = ลบทุกอย่าง
accounts-deleting = กำลังลบ…
accounts-delete-all-accounts = ทุกบัญชี รวมถึงอีเมลและไฟล์แนบทั้งหมดที่ Katna เก็บไว้
accounts-delete-all-contacts = รายชื่อติดต่อ ปฏิทิน และดัชนีการค้นหา
accounts-delete-all-settings = การตั้งค่า ลายเซ็น และแป้นพิมพ์ลัดทั้งหมด
accounts-delete-all-passwords = รหัสผ่านที่บันทึกไว้ทั้งหมด
accounts-deleted-heading = จะถูกลบออกจากคอมพิวเตอร์เครื่องนี้:
accounts-cannot-undo = การดำเนินการนี้เลิกทำไม่ได้
accounts-server-delete-all = ไม่มีการเปลี่ยนแปลงใดๆ บนเซิร์ฟเวอร์อีเมลของคุณ อีเมลของคุณยังอยู่ที่นั่น และการเพิ่มบัญชีอีกครั้งจะดาวน์โหลดอีเมลใหม่ อีเมลที่นำเข้าจากไฟล์มีอยู่ใน Katna เท่านั้น ไฟล์ต้นฉบับจะไม่ถูกแตะต้อง
accounts-server-local = อีเมลนี้นำเข้าจากไฟล์ Katna จึงมีสำเนาเพียงชุดเดียว ไฟล์ต้นฉบับจะไม่ถูกแตะต้อง นำเข้าไฟล์อีกครั้งเพื่อกู้คืนอีเมล
accounts-server-remove = ไม่มีการเปลี่ยนแปลงใดๆ บนเซิร์ฟเวอร์อีเมล อีเมลของคุณยังอยู่ที่นั่น และการเพิ่มบัญชีอีกครั้งจะดาวน์โหลดอีเมลใหม่
accounts-confirm-word = ลบ
accounts-confirm-placeholder = พิมพ์ “{ accounts-confirm-word }”
accounts-confirm-prompt = หากต้องการยืนยัน ให้พิมพ์ “{ accounts-confirm-word }”:
accounts-cancel = ยกเลิก
reset-cache-about = ลบอีเมลและไฟล์แนบที่ Katna ดาวน์โหลดไว้ รูปภาพผู้ส่ง และดัชนีการค้นหา แล้วดาวน์โหลดอีเมลล่าสุดใหม่ บัญชี การตั้งค่า และอีเมลที่มีอยู่ในคอมพิวเตอร์เครื่องนี้เท่านั้นจะยังอยู่
reset-cache-button = รีเซ็ตแคช
reset-cache-title = รีเซ็ตแคชไหม
reset-cache-deleted = ลบแล้วดาวน์โหลดใหม่:
reset-cache-mail = อีเมลและไฟล์แนบที่ดาวน์โหลดจากเซิร์ฟเวอร์ IMAP ของคุณ: อีเมลล่าสุดจะดาวน์โหลดใหม่ทันที ส่วนอีเมลที่เก่ากว่าจะดาวน์โหลดเมื่อคุณเปิด
reset-cache-index = ดัชนีการค้นหา ซึ่งจะสร้างใหม่ทันที
reset-cache-pictures = รูปภาพผู้ส่ง
reset-cache-kept = เก็บไว้: บัญชี รหัสผ่าน และการตั้งค่าของคุณ ดาว ป้ายกำกับ เครื่องหมายอ่านแล้ว และการปักหมุด ฉบับร่าง กล่องขาออก และการเปลี่ยนแปลงที่ยังไม่ถึงเซิร์ฟเวอร์ รวมถึงอีเมลจากบัญชี POP3 หรือไฟล์ที่นำเข้า ซึ่งอาจไม่มีสำเนาอื่น ไม่มีการเปลี่ยนแปลงใดๆ บนเซิร์ฟเวอร์อีเมลของคุณ
reset-cache-confirm = รีเซ็ตแคช
reset-cache-busy = กำลังรีเซ็ต…
reset-cache-done = รีเซ็ตแคชแล้ว กำลังดาวน์โหลดอีเมลล่าสุดใหม่
reset-cache-done-freed = รีเซ็ตแคชแล้วและได้พื้นที่ว่างคืนมา { $size } กำลังดาวน์โหลดอีเมลล่าสุดใหม่
