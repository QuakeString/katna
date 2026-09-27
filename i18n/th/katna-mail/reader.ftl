# Katna Mail, Thai (ไทย).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = ปิด
reader-back = กลับ
reader-mark-unread = ทำเครื่องหมายว่ายังไม่อ่าน
reader-move-to = ย้ายไปที่
reader-more = เพิ่มเติม
reader-print-all = พิมพ์ทั้งหมด
reader-new-window = ในหน้าต่างใหม่
reader-position = { $position } จาก { $total }
reader-newer = ใหม่กว่า
reader-older = เก่ากว่า

## Reading pane: the conversation

reader-removed = การสนทนานี้ถูกนำออกแล้ว
reader-no-subject = (ไม่มีหัวเรื่อง)
reader-collapse-all = ยุบทั้งหมด
reader-expand-all = ขยายทั้งหมด
reader-unknown-sender = (ไม่ทราบผู้ส่ง)
reader-date-ago = { $date } ({ $ago })
reader-me = ฉัน
reader-to = ถึง { $names }
reader-starred = ติดดาวแล้ว
reader-not-starred = ไม่ได้ติดดาว
reader-too-long = ข้อความยาวเกินกว่าจะแสดงได้ทั้งหมด
reader-encrypted-images = จะไม่โหลดรูปภาพจากเว็บในอีเมลที่เข้ารหัสเลย
reader-window-failed = เปิดหน้าต่างใหม่ไม่ได้

## Reading pane: message details (opened from "to me")

reader-details-from = จาก:
reader-details-to = ถึง:
reader-details-cc = สำเนา:
reader-details-date = วันที่:
reader-details-subject = หัวเรื่อง:

## Reading pane: downloading a message

reader-downloading = กำลังดาวน์โหลดข้อความนี้จากเซิร์ฟเวอร์…
reader-download-failed = ดาวน์โหลดข้อความนี้ไม่ได้
reader-try-again = ลองอีกครั้ง

## Reply row

reply-reply = ตอบกลับ
reply-reply-all = ตอบกลับทั้งหมด
reply-forward = ส่งต่อ

## Encrypted and signed mail

security-decrypting = กำลังถอดรหัส…
security-checking = กำลังตรวจสอบลายเซ็น…
security-partly-encrypted = ข้อความนี้เข้ารหัสไว้เพียงบางส่วน ส่วนที่เหลือถูกเพิ่มนอกการป้องกันและอาจมาจากใครก็ได้
security-partly-signed = ข้อความนี้มีลายเซ็นเพียงบางส่วน ส่วนที่เหลือถูกเพิ่มนอกการป้องกันและอาจมาจากใครก็ได้
security-encrypted = ข้อความที่เข้ารหัส
security-encrypted-smime = ข้อความที่เข้ารหัส (S/MIME)
security-no-key = ถอดรหัสข้อความนี้ไม่ได้: ข้อความถูกเข้ารหัสสำหรับคีย์ที่คุณไม่มี
security-cancelled = ยกเลิกการถอดรหัสแล้ว
security-damaged = ถอดรหัสข้อความนี้ไม่ได้: ข้อมูลที่เข้ารหัสเสียหายหรือถูกเปลี่ยนแปลง
security-decrypt-unavailable = ถอดรหัสข้อความนี้ไม่ได้: ติดตั้ง { $tool } เพื่ออ่านอีเมลที่เข้ารหัส
security-decrypt-failed = ถอดรหัสข้อความนี้ไม่ได้: { $reason }
security-unknown-signer = ผู้ลงนามที่ไม่รู้จัก
security-signed-verified = ลงนามโดย { $signer } · ยืนยันแล้ว
security-signed-not-sender = ลงนามโดย { $signer } ซึ่งไม่ใช่ผู้ส่ง
security-signed-untrusted = ลงนามโดย { $signer } ด้วยคีย์ที่คุณทำเครื่องหมายว่าไม่น่าเชื่อถือ
security-signed-unverified = ลงนามโดย { $signer } · คีย์ยังไม่ได้รับการยืนยัน
security-bad-signature = ลายเซ็นไม่ถูกต้อง: ข้อความนี้ถูกเปลี่ยนแปลงหลังจากลงนาม หรือลายเซ็นถูกปลอมแปลง
security-signature-expired = ลงนามโดย { $signer } · ลายเซ็นหมดอายุแล้ว
security-key-expired = ลงนามโดย { $signer } · คีย์หมดอายุไปแล้ว
security-key-revoked = ลงนามโดย { $signer } ด้วยคีย์ที่ถูกเพิกถอนแล้ว
security-missing-key = ลงนามด้วยคีย์ที่คุณไม่มี จึงตรวจสอบไม่ได้
security-missing-key-id = ลงนามด้วยคีย์ที่คุณไม่มี ({ $key }) จึงตรวจสอบไม่ได้
security-signature-unavailable = มีลายเซ็น ติดตั้ง { $tool } เพื่อตรวจสอบลายเซ็น
security-signature-error = ตรวจสอบลายเซ็นไม่ได้
tracking-opened = { $who } เปิดแล้ว { $count } ครั้ง ล่าสุด { $when }
tracking-opens-clicks = { $who } เปิดแล้ว { $opens } ครั้ง และคลิกลิงก์ { $clicks } ครั้ง ล่าสุด { $when }
tracking-clicked = { $who } คลิกลิงก์ { $clicks } ครั้ง ล่าสุด { $when }
tracking-maybe-opened = { $who } อาจเปิดแล้ว (Apple Mail โหลดรูปภาพเพื่อความเป็นส่วนตัว)
tracking-not-opened = { $who } ยังไม่ได้เปิด
tracking-receipt = { $who } ส่งใบตอบรับการอ่านแล้ว
tracking-receipt-displayed = ใบตอบรับการอ่าน: { $who } เปิดข้อความของคุณแล้ว
tracking-receipt-other = ใบตอบรับการอ่าน: { $who } ลบหรือจัดการข้อความของคุณโดยไม่ได้เปิด

## Remote images and pictures

remote-hidden = รูปภาพในข้อความนี้ถูกซ่อนไว้
remote-show = แสดงรูปภาพ
remote-always-show = แสดงรูปภาพจากผู้ส่งนี้เสมอ
remote-picture-use = ใช้
remote-picture-too-big = เลือกรูปภาพขนาดไม่เกิน 8 MB
remote-picture-type = เลือกรูปภาพ PNG, JPEG, GIF, WebP หรือ SVG
remote-picture-read-failed = อ่านรูปภาพไม่ได้: { $error }
remote-picture-keep-failed = เก็บรูปภาพไม่ได้: { $error }
remote-picture-remove-failed = นำรูปภาพออกไม่ได้: { $error }

## Attachments

attachment-count = ไฟล์แนบ { $count } รายการ
attachment-save = บันทึก
attachment-save-all = บันทึกทั้งหมด
attachment-save-all-tooltip = บันทึกไฟล์แนบทั้งหมดลงในโฟลเดอร์
attachment-save-here = บันทึกที่นี่
attachment-not-downloaded = ข้อความนี้ไม่ได้ดาวน์โหลดไว้
attachment-not-found = ไม่พบไฟล์แนบนี้ในข้อความ
attachment-read-failed = อ่าน { $name } ไม่ได้
attachment-numbered = ไฟล์แนบ { $number }
attachment-saved-all = บันทึก { $count } ไฟล์ไปที่ { $place } แล้ว
attachment-saved-some = บันทึก { $saved } จาก { $total } ไฟล์ไปที่ { $place } แล้ว บันทึก { $failed } ไม่ได้
attachment-saved-to = บันทึกไปที่ { $path } แล้ว
attachment-save-failed = บันทึก { $name } ไม่ได้: { $error }
attachment-open-failed = เปิด { $name } ไม่ได้: { $error }
attachment-risky = ไฟล์นี้อาจเรียกใช้โปรแกรมได้ Katna จึงไม่เปิดไฟล์นี้ โปรดบันทึกไฟล์แทน
attachment-encrypted-open = ไฟล์นี้ถูกเข้ารหัสมา บันทึกไฟล์ไว้เพื่อเปิดด้วยโปรแกรมอื่น

## Printing

print-failed = พิมพ์ไม่ได้: { $error }
print-no-font = ไม่พบแบบอักษร
print-opened-as-pdf = เปิดเป็น PDF แล้ว เพื่อสั่งพิมพ์จากที่นั่น
print-preview-title = ตัวอย่างก่อนพิมพ์
print-preview-laying-out = กำลังจัดหน้า…
print-preview-pages = { $count } หน้า
print-preview-more = และอีก { $count } หน้า
print-preview-failed = แสดงหน้าไม่ได้
print-preview-paper = กระดาษ
print-preview-a4 = A4
print-preview-letter = Letter
print-preview-cancel = ยกเลิก
print-preview-print = พิมพ์
print-not-downloaded = (ยังไม่ได้ดาวน์โหลด)
print-encrypted = (เข้ารหัสอยู่ เปิดใน Katna Mail เพื่อพิมพ์เนื้อหา)
print-to = ถึง: { $addresses }
print-cc = สำเนา: { $addresses }

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = เปิดข้อความนี้เพื่อดูไฟล์แนบ
text-copy = คัดลอก
text-select-all = เลือกทั้งหมด
