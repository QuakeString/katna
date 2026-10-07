# Katna Mail, Thai (ไทย).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The tray icon's menu

tray-open-inbox = _เปิดกล่องจดหมาย
tray-new-message = _ข้อความใหม่
tray-new-task = _งานใหม่
tray-new-note = โ_น้ตใหม่
tray-preferences = _การตั้งค่า
tray-quit = _ออก

## The tray icon's tooltip, under "Katna Mail"

tray-unread = { $count ->
    [0] ไม่มีอีเมลที่ยังไม่อ่าน
   *[other] ข้อความที่ยังไม่อ่าน { $count } ฉบับ
}
tray-password-refused = ต้องใช้รหัสผ่านใหม่สำหรับ { $address }
tray-signed-out = ลงชื่อเข้าใช้ { $address } อีกครั้ง
tray-accounts-need-you = { $count } บัญชีต้องให้คุณดำเนินการ
tray-not-sent = { $count ->
   *[other] ไม่ได้ส่งข้อความ { $count } ฉบับ
}
