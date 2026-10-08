# Katna Mail, Thai (ไทย).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = ปิด
reader-back = กลับ
reader-mark-unread = ทำเครื่องหมายว่ายังไม่อ่าน
reader-move-to = ย้ายไปที่
reader-snooze = เลื่อนเวลา
reader-remind = เตือนฉัน
reader-more = เพิ่มเติม
reader-original-colors = แสดงสีดั้งเดิม
reader-dark-colors = แสดงด้วยสีเข้ม
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
reader-sending = กำลังส่ง…
reader-me = ฉัน
reader-to = ถึง { $names }
reader-to-label = ถึง
reader-tick-delivered = ส่งถึงแล้ว { $when }
reader-tick-no-bounce = ส่งแล้ว { $when } ไม่มีอีเมลตีกลับ จึงน่าจะส่งถึงแล้ว
reader-tick-bounced = ส่งไม่ถึง: ตีกลับเมื่อ { $when }
reader-tick-read = อ่านแล้ว { $when } (ใบตอบรับการอ่าน)
reader-tick-opened = เปิดแล้ว ล่าสุด { $when } (การติดตามการเปิด)
reader-starred = ติดดาวแล้ว
reader-chip-remove = นำ { $label } ออก
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
reader-download-failed-reason = ดาวน์โหลดข้อความนี้ไม่ได้ { $reason }
reader-download-offline = บัญชีนี้ออฟไลน์อยู่ ออนไลน์เพื่อดาวน์โหลดข้อความนี้
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
security-look-up-key = ค้นหาคีย์

## a key to import (looked up, or attached to the message)

key-card-verified = ลายเซ็นที่ยืนยันแล้ว
key-card-verified-detail = ลายเซ็นถูกต้อง และคุณเชื่อถือคีย์นี้
key-card-unverified = ลายเซ็นยังไม่ได้รับการยืนยัน
key-card-unverified-detail = ลายเซ็นถูกต้อง แต่ไม่มีอะไรยืนยันว่าคีย์เป็นของเขา เทียบลายนิ้วมือกับเขา แล้วตั้งให้เชื่อถือคีย์ใน GnuPG (Kleopatra หรือ gpg --edit-key)
key-card-not-sender = ลงนามโดยบุคคลอื่น
key-card-not-sender-detail = ลายเซ็นถูกต้อง แต่คีย์ไม่ใช่ของผู้ส่ง
key-card-untrusted = คีย์ไม่น่าเชื่อถือ
key-card-untrusted-detail = คุณทำเครื่องหมายคีย์นี้ว่าไม่น่าเชื่อถือใน GnuPG
key-card-signature-expired = ลายเซ็นหมดอายุแล้ว
key-card-signature-expired-detail = ลายเซ็นเคยถูกต้อง แต่หมดอายุแล้ว
key-card-key-expired = คีย์หมดอายุแล้ว
key-card-key-expired-detail = ลายเซ็นถูกต้อง แต่คีย์หมดอายุไปแล้วหลังจากนั้น
key-card-key-revoked = คีย์ถูกเพิกถอนแล้ว
key-card-key-revoked-detail = เจ้าของเพิกถอนคีย์นี้แล้ว จึงเชื่อถือลายเซ็นไม่ได้
key-card-bad = ลายเซ็นไม่ถูกต้อง
key-card-bad-detail = ข้อความนี้ถูกเปลี่ยนแปลงหลังจากลงนาม หรือลายเซ็นถูกปลอมแปลง
key-card-signed-by = ลงนามโดย
key-card-belongs-to = เป็นของ
key-card-fingerprint = ลายนิ้วมือ
key-card-signed = ลงนามเมื่อ
key-card-key = คีย์
key-card-kind = { $standard }, { $algorithm }
key-card-created = สร้างเมื่อ
key-card-expires = หมดอายุ
key-card-never = ไม่มีกำหนด
key-card-issued-by = ออกโดย
key-card-found-in = พบใน
key-card-keyring = พวงกุญแจ GnuPG ของคุณ
key-card-copy = คัดลอกลายนิ้วมือ
key-card-import-title = นำเข้าคีย์นี้ไหม
key-card-from-directory = พบในไดเรกทอรีคีย์ของ { $domain }
key-card-from-attachment = จากไฟล์แนบ { $name }
key-card-import-note = จากนั้น Katna จะตรวจสอบลายเซ็นของบุคคลนี้และเข้ารหัสอีเมลถึงเขาได้ หากต้องการเชื่อถือคีย์อย่างเต็มที่ ให้เทียบลายนิ้วมือกับเขา
key-card-cancel = ยกเลิก
key-card-import = นำเข้าคีย์
key-card-looking-up = กำลังค้นหาคีย์…
key-card-looking-up-detail = กำลังสอบถามไดเรกทอรีคีย์ของ { $domain }
key-card-not-found = ไม่พบคีย์
key-card-not-found-detail = { $domain } ไม่ได้เผยแพร่คีย์สำหรับที่อยู่นี้ ขอให้ผู้ส่งส่งคีย์ของเขามาให้คุณ
key-card-not-kept = ใช้คีย์ที่พบไม่ได้
key-card-failed = รับคีย์ไม่ได้

## of a sender nothing confirmed

sender-failed-title = ข้อความนี้อาจไม่ได้มาจาก { $domain }
sender-failed-body = ข้อความนี้ไม่ผ่านการตรวจสอบผู้ส่งของ { $provider } ระวังลิงก์ ไฟล์แนบ และการตอบกลับ
sender-provider-unknown = ผู้ให้บริการอีเมลของคุณ
sender-details = รายละเอียด
sender-details-hide = ซ่อนรายละเอียด
sender-looks-safe = ดูปลอดภัย
sender-move-to-spam = ย้ายไปที่สแปม
sender-checked-by = ตรวจสอบโดย { $provider }
sender-checked-by-server = ตรวจสอบโดย { $provider } ({ $server })
sender-dmarc = โดเมนผู้ส่ง (DMARC)
sender-dkim = ลายเซ็น (DKIM)
sender-spf = เซิร์ฟเวอร์ที่ส่ง (SPF)
sender-result-pass = ผ่าน
sender-result-fail = ไม่ผ่าน
sender-result-unsure = ไม่แน่ใจ
sender-result-none = ไม่มี
sender-result-missing = ไม่ได้ตรวจสอบ
sender-dmarc-pass = { $domain } ยืนยันผู้ส่งรายนี้
sender-dmarc-fail = อีเมลนี้ไม่ตรงกับวิธีที่ { $domain } ระบุว่าใช้ส่งอีเมล
sender-dmarc-none = { $domain } ไม่ได้เผยแพร่กฎใด ๆ สำหรับอีเมลของตน
sender-dkim-pass = ลงนามโดย { $domain }
sender-dkim-fail = ลายเซ็นจาก { $domain } ไม่ตรงกับอีเมล
sender-dkim-none = ข้อความนี้ไม่ได้ลงนาม
sender-spf-pass = ส่งจากเซิร์ฟเวอร์ที่ { $domain } ระบุไว้
sender-spf-fail = ส่งจากเซิร์ฟเวอร์ที่ { $domain } ไม่ได้ระบุไว้
sender-spf-none = { $domain } ไม่ได้ระบุเซิร์ฟเวอร์ของตน
sender-check-unsure = การตรวจสอบให้คำตอบที่ชัดเจนไม่ได้
sender-unconfirmed = { $provider } ยืนยันไม่ได้ว่าข้อความนี้มาจาก { $domain } ใครก็ตั้งชื่อผู้ส่งเป็นอะไรก็ได้
sender-link-title = เปิดลิงก์นี้ไหม
sender-link-body = อีเมลนี้ไม่ผ่านการตรวจสอบผู้ส่ง ลิงก์นี้ไปที่ { $host }:
sender-link-cancel = ยกเลิก
sender-link-open = เปิด

## sent message's star, and the line above a read receipt)

tracking-opened = { $who } เปิดแล้ว { $count } ครั้ง ล่าสุด { $when }
tracking-opens-clicks = { $who } เปิดแล้ว { $opens } ครั้ง และคลิกลิงก์ { $clicks } ครั้ง ล่าสุด { $when }
tracking-clicked = { $who } คลิกลิงก์ { $clicks } ครั้ง ล่าสุด { $when }
tracking-maybe-opened = { $who } อาจเปิดแล้ว (Apple Mail โหลดรูปภาพเพื่อความเป็นส่วนตัว)
tracking-seen-none = ยังไม่มีใครเปิดหรือคลิกลิงก์
tracking-receipt = { $who } ส่งใบตอบรับการอ่านแล้ว
tracking-receipt-read = { $who } อ่านแล้ว (ใบตอบรับการอ่าน) { $when }
tracking-receipt-displayed = ใบตอบรับการอ่าน: { $who } เปิดข้อความของคุณแล้ว
tracking-receipt-other = ใบตอบรับการอ่าน: { $who } ลบหรือจัดการข้อความของคุณโดยไม่ได้เปิด

## Remote images and pictures

remote-hidden = รูปภาพในข้อความนี้ถูกซ่อนไว้
remote-hidden-unconfirmed = ซ่อนรูปภาพไว้: ไม่สามารถยืนยันผู้ส่งได้
remote-hidden-failed = ซ่อนรูปภาพไว้: อีเมลนี้ไม่ผ่านการตรวจสอบผู้ส่ง
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
attachment-forward = ส่งต่อ
attachment-save-all = บันทึกทั้งหมด
attachment-save-all-tooltip = บันทึกไฟล์แนบทั้งหมดลงในโฟลเดอร์
attachment-save-here = บันทึกที่นี่
attachment-not-downloaded = ข้อความนี้ไม่ได้ดาวน์โหลดไว้
attachment-open-message = เปิดข้อความนี้เพื่อดูไฟล์แนบ
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
print-preview-layout = เลย์เอาต์
print-preview-as-shown = ตามที่แสดง
print-preview-simple = ข้อความเท่านั้น
print-preview-backgrounds = พื้นหลัง
print-preview-cancel = ยกเลิก
print-preview-print = พิมพ์
print-not-downloaded = (ยังไม่ได้ดาวน์โหลด)
print-encrypted = (เข้ารหัสอยู่ เปิดใน Katna Mail เพื่อพิมพ์เนื้อหา)
print-to = ถึง: { $addresses }
print-cc = สำเนา: { $addresses }

## Message text (right-click menu in the reading pane)

text-pin = ปักหมุดไว้ด้านบน
text-copy-address = คัดลอกที่อยู่
text-copy = คัดลอก
text-select-all = เลือกทั้งหมด
