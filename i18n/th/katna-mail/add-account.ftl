# Katna Mail, Thai (ไทย).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = เพิ่มบัญชีอีเมล
add-account-providers-intro = เลือกผู้ให้บริการอีเมลของคุณ แล้ว Katna จะหาส่วนที่เหลือให้
add-account-provider-other = อีเมลอื่นๆ
add-account-provider-other-detail = บัญชี IMAP หรือ POP3 ใดก็ได้
add-account-provider-google-detail = Gmail และ Google Workspace
add-account-provider-microsoft-detail = Outlook และ Microsoft 365
add-account-provider-mail = { $provider } Mail
add-account-form-title = ลงชื่อเข้าใช้ { $provider }
add-account-form-title-other = บัญชีอีเมลของคุณ
add-account-form-intro = Katna เก็บรหัสผ่านของคุณไว้ในพวงกุญแจของระบบ
add-account-looking = กำลังค้นหาเซิร์ฟเวอร์อีเมลของ { $address }…
add-account-address-intro = ป้อนที่อยู่อีเมลของคุณ แล้ว Katna จะค้นหาเซิร์ฟเวอร์ให้
add-account-servers-title = การตั้งค่าเซิร์ฟเวอร์
add-account-servers-intro = ที่ที่ Katna อ่านและส่งอีเมลของ { $address }
add-account-signing-in = กำลังลงชื่อเข้าใช้…
add-account-browser-title = ดำเนินการต่อในเบราว์เซอร์ของคุณ
add-account-browser-intro = Katna เปิดหน้าลงชื่อเข้าใช้ { $provider } ในเบราว์เซอร์ของคุณแล้ว ลงชื่อเข้าใช้ที่นั่นและอนุญาตให้ Katna อ่านและส่งอีเมลของคุณ แล้วกลับมาที่นี่
add-account-browser-hint = ไม่มีหน้าใดเปิดขึ้นใช่ไหม ตรวจดูหน้าต่างเบราว์เซอร์ของคุณ หรือย้อนกลับแล้วลองอีกครั้ง
add-account-stage-browser = กำลังรอให้คุณลงชื่อเข้าใช้ในเบราว์เซอร์…
add-account-stage-signing-in-at = กำลังลงชื่อเข้าใช้ที่ { $server }…
add-account-help-app-password-link = วิธีสร้างรหัสผ่านสำหรับแอป
add-account-help-turn-on-imap = { $provider } จะให้แอปอีเมลเข้าใช้ได้ก็ต่อเมื่อเปิดการเข้าถึง IMAP และ POP3 ในการตั้งค่าของเว็บเมลแล้ว
add-account-help-turn-on-imap-link = วิธีเปิดใช้

## Add a mail account: fields

add-account-field-address = ที่อยู่อีเมล
add-account-receive-with = รับอีเมลด้วย
add-account-imap-about = IMAP เก็บอีเมลและโฟลเดอร์ไว้บนเซิร์ฟเวอร์ เหมือนกันในทุกอุปกรณ์ เลือกแบบนี้หากทำได้
add-account-pop3-about = POP3 ดาวน์โหลดอีเมลมาไว้ในคอมพิวเตอร์เครื่องนี้ อีเมลที่คุณอ่านหรือย้ายที่นี่จะยังคงเหมือนเดิมบนเซิร์ฟเวอร์และอุปกรณ์อื่นๆ ของคุณ
add-account-incoming = อีเมลขาเข้า ({ $protocol })
add-account-outgoing = อีเมลขาออก ({ $protocol })
add-account-field-server = เซิร์ฟเวอร์
add-account-field-port = พอร์ต
add-account-security-none = ไม่มี
add-account-security-none-warning = ไม่ได้เข้ารหัส: รหัสผ่านและอีเมลของคุณอาจถูกอ่านได้ระหว่างทาง
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

## Add a mail account: buttons

add-account-sign-in-with = ลงชื่อเข้าใช้ด้วย { $provider }
add-account-sign-in-instead = ลงชื่อเข้าใช้ด้วย { $provider } แทน
add-account-servers-button = การตั้งค่าเซิร์ฟเวอร์
add-account-back = กลับ
add-account-add = เพิ่มบัญชี
add-account-done = เสร็จสิ้น
add-account-another = เพิ่มบัญชีอื่น
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
add-account-app-password-refused = { $provider } ปฏิเสธรหัสผ่าน ต้องใช้รหัสผ่านสำหรับแอป ไม่ใช่รหัสผ่านที่คุณใช้บนเว็บ
add-account-password-refused = เซิร์ฟเวอร์ปฏิเสธรหัสผ่าน โปรดตรวจสอบแล้วลองอีกครั้ง
add-account-sign-in-refused = { $provider } ไม่อนุญาตให้ Katna เข้าใช้ ลองอีกครั้ง และอนุญาตให้เข้าถึงอีเมลของคุณ
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] Katna ที่ติดตั้งอยู่นี้ยังลงชื่อเข้าใช้บัญชี Microsoft ไม่ได้
    [Google] Katna ที่ติดตั้งอยู่นี้ยังลงชื่อเข้าใช้บัญชี Google ไม่ได้
   *[other] ผู้ให้บริการนี้อนุญาตให้ลงชื่อเข้าใช้ได้เฉพาะในหน้าของตนเอง ซึ่ง Katna ยังทำให้ไม่ได้
}
add-account-smtp-not-found = Katna พบที่สำหรับอ่านอีเมลของคุณ แต่ไม่พบที่สำหรับส่ง โปรดป้อนเซิร์ฟเวอร์ขาออก

## Add a mail account: the last step

add-account-done-title = บัญชีของคุณพร้อมแล้ว
add-account-done-intro = Katna กำลังรับอีเมลของคุณ อีเมลใหม่จะแสดงเมื่อมาถึง
add-account-done-sign-in = การลงชื่อเข้าใช้
add-account-done-signed-in-with = ด้วย { $provider } ในเบราว์เซอร์ของคุณ
add-account-done-receiving = การรับอีเมล
add-account-done-sending = การส่งอีเมล
add-account-done-on-server = อีเมลบนเซิร์ฟเวอร์
add-account-done-kept = เก็บไว้จนกว่าคุณจะลบใน Katna
add-account-done-pop3-hint = เปลี่ยนสิ่งที่จะเกิดกับอีเมลบนเซิร์ฟเวอร์ได้ใน การตั้งค่า > บัญชี
add-account-done-zoho-title = งานและปฏิทิน
add-account-done-zoho-about = Zoho เก็บสิ่งเหล่านี้แยกจากอีเมล ลงชื่อเข้าใช้ด้วย Zoho หนึ่งครั้งเพื่อนำมาไว้ใน Katna
add-account-done-linked = เชื่อมต่องานและปฏิทินแล้ว

## The account menu (from the account button on the top bar)

add-account-menu-another = เพิ่มบัญชีอื่น
app-menu = เมนูหลัก
app-menu-back = กลับ
