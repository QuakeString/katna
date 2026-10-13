# Katna Mail, Thai (ไทย).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Why a message has not gone out (one line under its subject)

outbox-not-sent = ไม่ได้ส่ง เนื่องจาก{ $reason }
outbox-retrying = ยังไม่ได้ส่ง เนื่องจาก{ $reason } Katna จะลองอีกครั้งเอง
outbox-waiting-sign-in = กำลังรอให้คุณลงชื่อเข้าใช้ { $address } อีกครั้ง แล้วจะส่งออกไปทันที
outbox-waiting-password = กำลังรอรหัสผ่านใหม่ของ { $address } แล้วจะส่งออกไปทันที
outbox-waiting-connection = กำลังรอการเชื่อมต่อ จะส่งออกไปเมื่อคุณกลับมาออนไลน์

## Reasons, read from the mail server's answer (they follow "because")

outbox-reason-no-recipients = ไม่มีผู้รับ
outbox-reason-address = ที่อยู่ของผู้รับบางรายไม่มีอยู่จริง
outbox-reason-too-large = มีขนาดใหญ่เกินไปสำหรับเซิร์ฟเวอร์อีเมล
outbox-reason-blocked = เซิร์ฟเวอร์อีเมลบล็อกไว้
outbox-reason-gone = สำเนาในคอมพิวเตอร์เครื่องนี้หายไปแล้ว
outbox-reason-refused = เซิร์ฟเวอร์อีเมลปฏิเสธ

## Buttons and notes

outbox-try-again = ลองอีกครั้ง
outbox-edit = แก้ไข
outbox-delete = ลบ
outbox-deleted = ลบออกจากกล่องขาออกแล้ว
outbox-sending-again = กำลังส่งอีกครั้ง…
outbox-snackbar-not-sent = ไม่ได้ส่ง “{ $subject }” เนื่องจาก{ $reason }
outbox-open = กล่องขาออก
