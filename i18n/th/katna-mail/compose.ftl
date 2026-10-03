# Katna Mail, Thai (ไทย).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Compose window: title bar

compose-new-message = ข้อความใหม่
compose-restore = คืนขนาด
compose-minimize = ย่อ
compose-exit-full-screen = ออกจากโหมดเต็มหน้าจอ
compose-open-window = เปิดในหน้าต่างใหม่
compose-save-close = บันทึกและปิด
compose-back-to-mail = กลับไปที่หน้าต่างอีเมล
compose-pop-out-reply = แยกการตอบกลับออกมา
compose-edit-recipients = แก้ไขผู้รับ
compose-summary-cc = สำเนา: { $names }
compose-summary-bcc = สำเนาลับ: { $names }
compose-more-recipients = อีก { $count } คน
compose-show-trimmed = แสดงเนื้อหาที่ตัดออก
compose-hide-trimmed = ซ่อนเนื้อหาที่ตัดออก
compose-remove-trimmed = นำข้อความที่อ้างอิงออก
compose-trimmed-removed = นำข้อความที่อ้างอิงออกแล้ว

## Recipients and subject

compose-to = ถึง
compose-cc = สำเนา
compose-bcc = สำเนาลับ
compose-from = จาก
compose-from-choose = ส่งจากบัญชีอื่น
compose-recipients = ผู้รับ
compose-subject = หัวเรื่อง

## Sending (the notes at the bottom of the window)

compose-open-elsewhere = ส่งหรือทิ้งข้อความที่เปิดอยู่ก่อน
compose-bad-address = “{ $address }” ไม่ใช่ที่อยู่อีเมล
compose-no-recipients = เพิ่มผู้รับอย่างน้อยหนึ่งคน
compose-attachments-too-large = ไฟล์แนบมีขนาด { $size } แต่เซิร์ฟเวอร์อีเมลรับได้สูงสุด { $limit }
compose-no-account = เพิ่มบัญชีเพื่อใช้ส่งอีเมล
compose-past-time = เลือกเวลาในอนาคต
compose-scheduling = กำลังกำหนดเวลา…
compose-sending = กำลังส่ง…
compose-scheduled = กำหนดเวลาส่งไว้ที่ { $when }
compose-sent-archived = ส่งและเก็บถาวรแล้ว
compose-sent = ส่งข้อความแล้ว
compose-discarded = ทิ้งฉบับร่างแล้ว
compose-draft-saved = บันทึกฉบับร่างแล้ว
compose-draft-saving = กำลังบันทึก…
compose-draft-failed = บันทึกฉบับร่างไม่ได้: { $error }
compose-draft-not-opened = เปิดฉบับร่างไม่ได้

## Attachments

compose-picker-insert = แทรก
compose-picker-attach = แนบ
compose-file-too-large = { $name } ใหญ่เกินไป: ข้อความหนึ่งรับได้สูงสุด { $limit }
compose-forward-files-missing = ไฟล์ของข้อความที่ส่งต่อยังไม่ได้ดาวน์โหลด จึงไม่ได้แนบไป
compose-attachment-size = ({ $size })
compose-remove-attachment = นำไฟล์แนบออก
compose-attachment-open-tip = เปิดเพื่อตรวจดู
compose-attachments-total = { $count } ไฟล์ รวม { $size }
compose-drive-note = { $name } มีขนาดเกิน { $limit } จึงจะถูกอัปโหลดไปที่ Google Drive ของคุณ และข้อความจะมีลิงก์ของไฟล์
compose-drive-tip = อยู่ใน Google Drive ของคุณ ข้อความจะมีลิงก์
compose-drive-uploading = กำลังอัปโหลด { $percent }%
compose-drive-allow = อนุญาต Drive
compose-drive-allow-tip = ลงชื่อเข้าใช้ด้วย Google อีกครั้งเพื่อให้ Katna วางไฟล์ขนาดใหญ่ใน Drive ของคุณได้
compose-drive-retry = ลองอีกครั้ง
compose-drive-sends-when-uploaded = จะส่งเมื่ออัปโหลด { $name } เสร็จ
compose-drive-not-uploaded = { $name } ยังไม่อยู่ใน Google Drive
compose-drive-share-failed = ไม่สามารถแชร์ไฟล์ใน Google Drive ได้: { $error }
compose-drive-share-title = แชร์ไฟล์กับทุกคนหรือไม่
compose-drive-share-text = { $count ->
   *[other] Google Drive แชร์ไฟล์กับ { $addresses } ซึ่งไม่มีบัญชี Google ไม่ได้ แต่ทุกคนที่มีลิงก์จะเปิดไฟล์ได้แทน
}
compose-drive-share-link = แชร์ด้วยลิงก์
compose-drive-send-without = ส่งโดยไม่แชร์
compose-drive-share-cancel = ยกเลิก
compose-drive-card-detail = { $size } · Google Drive
compose-drive-card-name = Google Drive
compose-onedrive-note = { $name } มีขนาดเกิน { $limit } จึงจะถูกอัปโหลดไปที่ OneDrive ของคุณ และข้อความจะมีลิงก์ของไฟล์
compose-onedrive-tip = อยู่ใน OneDrive ของคุณ ข้อความจะมีลิงก์
compose-onedrive-allow = อนุญาต OneDrive
compose-onedrive-allow-tip = ลงชื่อเข้าใช้ด้วย Microsoft อีกครั้งเพื่อให้ Katna วางไฟล์ขนาดใหญ่ใน OneDrive ของคุณได้
compose-onedrive-not-uploaded = { $name } ยังไม่อยู่ใน OneDrive
compose-onedrive-share-failed = ไม่สามารถแชร์ไฟล์ใน OneDrive ได้: { $error }
compose-onedrive-share-text = { $count ->
   *[other] OneDrive แชร์ไฟล์กับ { $addresses } ไม่ได้ แต่ทุกคนที่มีลิงก์จะเปิดไฟล์ได้แทน
}
compose-onedrive-card-detail = { $size } · OneDrive
compose-onedrive-card-name = OneDrive
compose-drop-files = วางไฟล์ที่นี่
compose-drop-here = วางที่นี่

## Paste options (a small bar under what was just pasted or dropped)

compose-paste-keep-formatting = คงการจัดรูปแบบไว้
compose-paste-table = ตาราง
compose-paste-picture = รูปภาพ
compose-paste-plain-text = ข้อความธรรมดา
compose-paste-inline = ในเนื้อความ
compose-paste-attachment = ไฟล์แนบ

## Encryption and signing (the toggles by the recipients)

compose-encrypt = เข้ารหัส
compose-encrypted = เข้ารหัสแล้ว: มีเพียงผู้รับที่อ่านได้
compose-sign = ลงลายเซ็น
compose-signed = ลงลายเซ็นแล้ว: ผู้รับตรวจสอบได้ว่ามาจากคุณ

## Open and click tracking and read receipts (toggles after Sign)

compose-track = ติดตามการเปิดและการคลิก
compose-tracked = ติดตามอยู่: คุณจะเห็นเมื่อผู้รับแต่ละคนเปิดอีเมลหรือคลิกลิงก์
compose-track-clicks = ติดตามการคลิกลิงก์ (ข้อความธรรมดาไม่สามารถแสดงการเปิดได้)
compose-tracked-clicks = ติดตามอยู่: คุณจะเห็นเมื่อผู้รับแต่ละคนคลิกลิงก์
compose-track-sign-in = ลงชื่อเข้าใช้บัญชี Katna เพื่อติดตามการเปิดและการคลิก
compose-receipt = ขอใบตอบรับการอ่าน
compose-receipt-on = ขอใบตอบรับการอ่านแล้ว: แอปของผู้รับอาจถามให้เขาส่งกลับมา
compose-delivery = ขอใบตอบรับการส่งถึง
compose-delivery-on = ขอใบตอบรับการส่งถึงแล้ว: เซิร์ฟเวอร์อีเมลของคุณจะส่งอีเมลแจ้งคุณเมื่อเซิร์ฟเวอร์ของผู้รับแต่ละคนรับข้อความไว้
compose-delivery-unavailable = เซิร์ฟเวอร์อีเมลของคุณไม่ส่งใบตอบรับการส่งถึง

## Spelling

spell-no-dictionary = ไม่ได้ติดตั้งพจนานุกรมการสะกดคำสำหรับ { $language } (เช่น hunspell-en_us)
spell-dictionary-error = พจนานุกรมการสะกดคำ: { $error }

## Grammar checking (the right-click menu on a grammar mistake)

grammar-replace = “{ $words }”
grammar-add = เพิ่ม “{ $words }”
grammar-remove = ลบ “{ $words }”
grammar-ignore = ละเว้น

## Send checks (asked before a message goes out)

send-check-attachment-title = คุณตั้งใจจะแนบไฟล์หรือไม่
send-check-attachment-text = คุณเขียนถึงไฟล์แนบ แต่ยังไม่ได้แนบอะไรเลย
send-check-attach = แนบไฟล์
send-check-subject-title = ส่งโดยไม่มีหัวเรื่องหรือไม่
send-check-subject-text = ข้อความนี้ไม่มีหัวเรื่อง
send-check-add-subject = เพิ่มหัวเรื่อง
send-check-send-anyway = ส่งเลย

## Recipients (To, Cc and Bcc)

recipient-not-valid = ไม่ใช่ที่อยู่อีเมลที่ถูกต้อง
recipient-show-address = แสดงที่อยู่
recipient-remove = นำออก
recipient-bad-title = ตรวจสอบที่อยู่
recipient-bad-text = “{ $address }” ไม่ใช่ที่อยู่อีเมลที่ถูกต้อง โปรดแก้ไขหรือนำออกก่อนส่ง
recipient-bad-fix = แก้ไข
