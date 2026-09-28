# Katna Mail, Thai (ไทย).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = เพิ่มบัญชีอีเมล
add-account-looking = กำลังค้นหาเซิร์ฟเวอร์อีเมลของ { $address }…
add-account-address-intro = ป้อนที่อยู่อีเมลของคุณ แล้ว Katna จะค้นหาเซิร์ฟเวอร์ให้
add-account-servers-title = การตั้งค่าเซิร์ฟเวอร์
add-account-servers-intro = ที่ที่ Katna อ่านและส่งอีเมลของ { $address }
add-account-password-title = ป้อนรหัสผ่านของคุณ
add-account-signing-in = กำลังลงชื่อเข้าใช้…
add-account-browser-title = ดำเนินการต่อในเบราว์เซอร์ของคุณ
add-account-browser-intro = Katna เปิดหน้าลงชื่อเข้าใช้ { $provider } ในเบราว์เซอร์ของคุณแล้ว ลงชื่อเข้าใช้ที่นั่นและอนุญาตให้ Katna อ่านและส่งอีเมลของคุณ แล้วกลับมาที่นี่
add-account-browser-hint = ไม่มีหน้าใดเปิดขึ้นใช่ไหม ตรวจดูหน้าต่างเบราว์เซอร์ของคุณ หรือย้อนกลับแล้วลองอีกครั้ง

## Add a mail account: fields

add-account-field-address = ที่อยู่อีเมล
add-account-incoming = อีเมลขาเข้า ({ $protocol })
add-account-outgoing = อีเมลขาออก ({ $protocol })
add-account-field-server = เซิร์ฟเวอร์
add-account-field-port = พอร์ต
add-account-security-none = ไม่มี
add-account-field-username = ชื่อผู้ใช้
add-account-field-password = รหัสผ่าน
add-account-show-password = แสดงรหัสผ่าน
add-account-app-password-hint = { $provider } ต้องใช้รหัสผ่านสำหรับแอปที่นี่ ไม่ใช่รหัสผ่านที่คุณใช้บนเว็บ สร้างได้ในการตั้งค่าความปลอดภัยของบัญชี { $provider } ของคุณ
add-account-field-name = ชื่อของคุณ (ไม่บังคับ)
add-account-name-hint = แสดงให้ผู้ที่คุณเขียนถึงเห็น
add-account-servers-pair = { $imap } และ { $smtp }
add-account-servers-found = { $source ->
    [built-in] เซิร์ฟเวอร์: { $servers } พบในรายชื่อผู้ให้บริการของ Katna
    [provider] เซิร์ฟเวอร์: { $servers } พบในการตั้งค่าของผู้ให้บริการของคุณ
    [ispdb] เซิร์ฟเวอร์: { $servers } พบในรายชื่อผู้ให้บริการของ Thunderbird
    [dns] เซิร์ฟเวอร์: { $servers } พบในระเบียน DNS ของโดเมนของคุณ
   *[other] เซิร์ฟเวอร์: { $servers } จากการคาดเดา โปรดตรวจสอบหากลงชื่อเข้าใช้ไม่สำเร็จ
}
add-account-servers-entered = เซิร์ฟเวอร์: { $servers } ตามที่ป้อน
add-account-or = หรือ
add-account-sign-in-with = ลงชื่อเข้าใช้ด้วย { $provider }
add-account-sign-in-instead = ลงชื่อเข้าใช้ด้วย { $provider } แทน

## Add a mail account: buttons

add-account-servers-button = การตั้งค่าเซิร์ฟเวอร์
add-account-back = กลับ
add-account-add = เพิ่มบัญชี
add-account-next = ถัดไป
add-account-cancel = ยกเลิก

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] ป้อนเซิร์ฟเวอร์ขาเข้า
   *[outgoing] ป้อนเซิร์ฟเวอร์ขาออก
}
add-account-server-space = { $kind ->
    [incoming] ชื่อเซิร์ฟเวอร์ขาเข้ามีช่องว่าง
   *[outgoing] ชื่อเซิร์ฟเวอร์ขาออกมีช่องว่าง
}
add-account-port-invalid = { $kind ->
    [incoming] พอร์ตขาเข้าต้องเป็นตัวเลขตั้งแต่ { $min } ถึง { $max }
   *[outgoing] พอร์ตขาออกต้องเป็นตัวเลขตั้งแต่ { $min } ถึง { $max }
}
add-account-address-empty = ป้อนที่อยู่อีเมล
add-account-address-invalid = ป้อนที่อยู่อีเมล เช่น { $example }
add-account-not-found = Katna ไม่พบเซิร์ฟเวอร์ของ { $address } จึงใส่ชื่อที่ใช้กันทั่วไปให้ โปรดตรวจสอบกับผู้ให้บริการของคุณ
add-account-password-empty = ป้อนรหัสผ่าน
add-account-name-is-password = ชื่อเหมือนกับรหัสผ่าน ให้พิมพ์ชื่อของคุณในช่องนั้นแทน ตามที่ต้องการให้ผู้อื่นเห็น
add-account-added = เพิ่ม { $address } แล้ว กำลังรับอีเมลของคุณ…
add-account-app-password-refused = { $provider } ปฏิเสธรหัสผ่าน ต้องใช้รหัสผ่านสำหรับแอป ไม่ใช่รหัสผ่านที่คุณใช้บนเว็บ
add-account-password-refused = เซิร์ฟเวอร์ปฏิเสธรหัสผ่าน โปรดตรวจสอบแล้วลองอีกครั้ง
add-account-sign-in-refused = { $provider } ไม่อนุญาตให้ Katna เข้าใช้ ลองอีกครั้ง และอนุญาตให้เข้าถึงอีเมลของคุณ
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] Katna ที่ติดตั้งอยู่นี้ยังลงชื่อเข้าใช้บัญชี Microsoft ไม่ได้
    [Google] Katna ที่ติดตั้งอยู่นี้ยังลงชื่อเข้าใช้บัญชี Google ไม่ได้
   *[other] ผู้ให้บริการนี้อนุญาตให้ลงชื่อเข้าใช้ได้เฉพาะในหน้าของตนเอง ซึ่ง Katna ยังทำให้ไม่ได้
}
add-account-signed-in = ลงชื่อเข้าใช้ด้วย { $provider } แล้ว กำลังรับอีเมลของคุณ…

## The account menu (from the account button on the top bar)

add-account-menu-another = เพิ่มบัญชีอื่น
add-account-menu-manage = จัดการบัญชี
