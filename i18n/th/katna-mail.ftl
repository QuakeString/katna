# Katna Mail, Thai (ไทย).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Language picker (top bar and Settings > General)

language-tooltip = ภาษา: { $language }
language-tooltip-system = ภาษา: { $language } (ตามระบบ)
language-search = ค้นหาภาษา
language-system-default = ค่าเริ่มต้นของระบบ
language-system-now = ขณะนี้ใช้{ $language }
language-no-match = ไม่มีภาษาที่ตรงกับ “{ $query }”
language-machine = แปลโดยเครื่อง ช่วยกันปรับปรุงคำแปลได้
language-setting = ภาษา
language-setting-detail = ภาษาของเมนู ปุ่ม และข้อความ รวมถึงรูปแบบวันที่และตัวเลข ค่าเริ่มต้นของระบบจะใช้ตามการตั้งค่าของเดสก์ท็อป

## Dates and sizes

ago-just-now = เมื่อสักครู่
ago-minutes = { $count } นาทีที่แล้ว
ago-hours = { $count } ชั่วโมงที่แล้ว
ago-days = { $count } วันที่แล้ว
size-bytes = { $count } ไบต์
size-kb = { $size } KB
size-mb = { $size } MB
size-gb = { $size } GB
size-tb = { $size } TB

## Top bar

folders-hide = ซ่อนโฟลเดอร์
folders-show = แสดงโฟลเดอร์
compose = เขียน
search = ค้นหา
search-mail = ค้นหาอีเมล
search-settings = ค้นหาการตั้งค่า
search-clear = ล้างการค้นหา
search-options-show = แสดงตัวเลือกการค้นหา
settings = การตั้งค่า
account-add = เพิ่มบัญชี

## App rail (and the bottom bar on a phone)

rail-mail = อีเมล
rail-calendar = ปฏิทิน
rail-contacts = รายชื่อติดต่อ
rail-tasks = งาน
rail-notes = โน้ต
rail-feeds = ฟีด

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = เร็วๆ นี้
app-calendar-promise = ปฏิทิน CalDAV ของคุณ คำเชิญประชุมจากอีเมล และการช่วยเตือน อยู่ข้างกล่องจดหมายของคุณ
app-tasks-promise = รายการสิ่งที่ต้องทำที่ซิงค์กับ CalDAV และงานที่สร้างจากอีเมล
app-notes-promise = โน้ตสั้นๆ และโน้ตในอีเมลหรือการสนทนาไว้ดูภายหลัง
app-feeds-promise = อ่านฟีด RSS และ Atom ข้างอีเมลของคุณ

## Contacts page

app-contacts-loading = กำลังรวบรวมรายชื่อจากอีเมลของคุณ…
app-contacts-empty = คนที่คุณติดต่อทางอีเมลจะแสดงที่นี่
app-contacts-count = { $count } คนจากอีเมลของคุณ เรียงจากคนที่ติดต่อบ่อยที่สุด
app-contacts-top = { $count } คนแรกจากอีเมลของคุณ เรียงจากคนที่ติดต่อบ่อยที่สุด
app-contacts-messages = { $count } ข้อความ
app-contacts-last = ล่าสุด { $date }

## Navigation (the folders pane)

nav-labels = ป้ายกำกับ
nav-folders = โฟลเดอร์
nav-label-new = สร้างป้ายกำกับใหม่
nav-folder-new = สร้างโฟลเดอร์ใหม่
nav-account-unnamed = บัญชี { $number }
nav-tab-new = ใหม่ { $count } รายการ

## Special folders (the user's own folders keep their names)

folder-inbox = กล่องจดหมาย
folder-starred = ที่ติดดาว
folder-drafts = ฉบับร่าง
folder-sent = ส่งแล้ว
folder-archive = เก็บถาวร
folder-spam = สแปม
folder-trash = ถังขยะ
folder-all-mail = จดหมายทั้งหมด
folder-scheduled = กำหนดเวลาไว้

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = ป้ายกำกับใหม่
label-folder-new-title = โฟลเดอร์ใหม่
label-prompt = โปรดป้อนชื่อป้ายกำกับใหม่:
label-folder-prompt = โปรดป้อนชื่อโฟลเดอร์ใหม่:
label-name-hint = ชื่อป้ายกำกับ
label-folder-name-hint = ชื่อโฟลเดอร์
label-nest = ซ้อนป้ายกำกับไว้ใต้:
label-folder-nest = ซ้อนโฟลเดอร์ไว้ใต้:
label-cancel = ยกเลิก
label-create = สร้าง
label-creating = กำลังสร้าง…
label-created = สร้างป้ายกำกับ “{ $name }” แล้ว
label-folder-created = สร้างโฟลเดอร์ “{ $name }” แล้ว

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = หลัก
tab-promotions = โปรโมชัน
tab-social = โซเชียล
tab-updates = อัปเดต
tab-forums = ฟอรัม
tab-focused = โฟกัส
tab-other = อื่นๆ
tab-inbox = กล่องจดหมาย
tab-newsletters = จดหมายข่าว
tab-notifications = การแจ้งเตือน
tab-new = ใหม่ { $count } รายการ
tab-provider-other = จัดเรียงโดย Katna

## Mail list: toolbar

list-select = เลือก
list-refresh = รีเฟรช
list-more = เพิ่มเติม
list-mark-read = ทำเครื่องหมายว่าอ่านแล้ว
list-mark-unread = ทำเครื่องหมายว่ายังไม่อ่าน
list-move-to = ย้ายไปที่
list-archive = เก็บถาวร
list-spam = รายงานสแปม
list-delete = ลบ
list-newer = ใหม่กว่า
list-older = เก่ากว่า
list-range = { $first }–{ $last } จาก { $total }
list-range-about = { $first }–{ $last } จากประมาณ { $total }
list-results = ผลการค้นหาสำหรับ “{ $query }”
list-results-corrected = กำลังแสดงผลการค้นหาสำหรับ “{ $query }”
list-search-instead = ค้นหา “{ $query }” แทน
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = ทั้งหมด
list-pick-none = ไม่เลือกเลย
list-pick-read = อ่านแล้ว
list-pick-unread = ยังไม่อ่าน
list-pick-starred = ติดดาว
list-pick-unstarred = ไม่ติดดาว

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] เลือกการสนทนาทั้ง { $count } รายการแล้ว
   *[message] เลือกข้อความทั้ง { $count } รายการแล้ว
}
list-selected-all-in = { $kind ->
    [conversation] เลือกการสนทนาทั้ง { $count } รายการใน { $folder } แล้ว
   *[message] เลือกข้อความทั้ง { $count } รายการใน { $folder } แล้ว
}
list-selected-screen = { $kind ->
    [conversation] เลือกการสนทนาทั้ง { $count } รายการบนหน้าจอแล้ว
   *[message] เลือกข้อความทั้ง { $count } รายการบนหน้าจอแล้ว
}
list-select-all = { $kind ->
    [conversation] เลือกการสนทนาทั้ง { $count } รายการ
   *[message] เลือกข้อความทั้ง { $count } รายการ
}
list-select-all-in = { $kind ->
    [conversation] เลือกการสนทนาทั้ง { $count } รายการใน { $folder }
   *[message] เลือกข้อความทั้ง { $count } รายการใน { $folder }
}
list-clear-selection = ล้างการเลือก

## Mail list: empty states

list-empty-search = ไม่มีข้อความที่ตรงกับการค้นหาของคุณ
list-empty-tab = ไม่มีอีเมลใน { $tab }
list-empty-tab-unknown = ไม่มีอีเมลในแท็บนี้
list-empty-folder = ไม่มีข้อความใน { $folder }
list-empty-folder-unknown = ไม่มีข้อความในโฟลเดอร์นี้
list-first-sync = กำลังรับอีเมลของคุณ…
list-first-sync-detail = อีเมลจะแสดงที่นี่เมื่อมาถึง

## Mail list: lines

row-removed = ข้อความนี้ถูกนำออกแล้ว
row-starred = ติดดาวแล้ว
row-not-starred = ไม่ได้ติดดาว
row-important = สำคัญ คลิกเพื่อทำเครื่องหมายว่าไม่สำคัญ
row-mark-important = ทำเครื่องหมายว่าสำคัญ
row-pinned = ปักหมุดไว้ด้านบน
row-pin = ปักหมุดไว้ด้านบน
row-unpin = เลิกปักหมุด

## Mail list: More menu and right-click menu

menu-reply = ตอบกลับ
menu-reply-all = ตอบกลับทั้งหมด
menu-forward = ส่งต่อ
menu-archive = เก็บถาวร
menu-delete = ลบ
menu-spam = รายงานสแปม
menu-mark-read = ทำเครื่องหมายว่าอ่านแล้ว
menu-mark-unread = ทำเครื่องหมายว่ายังไม่อ่าน
menu-mark-all-read = ทำเครื่องหมายทั้งหมดว่าอ่านแล้ว
menu-star = ติดดาว
menu-unstar = นำดาวออก
menu-important = ทำเครื่องหมายว่าสำคัญ
menu-not-important = ทำเครื่องหมายว่าไม่สำคัญ
menu-pin = ปักหมุดไว้ด้านบน
menu-unpin = เลิกปักหมุด
menu-print-all = พิมพ์ทั้งหมด
menu-new-window = เปิดในหน้าต่างใหม่
menu-move-to = ย้ายไปที่
menu-move-to-heading = ย้ายไปที่:
menu-find-from = ค้นหาอีเมลจาก { $name }

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] เก็บถาวรการสนทนา { $count } รายการแล้ว
   *[message] เก็บถาวรข้อความ { $count } รายการแล้ว
}
toast-trashed = { $kind ->
    [conversation] ย้ายการสนทนา { $count } รายการไปที่ถังขยะแล้ว
   *[message] ย้ายข้อความ { $count } รายการไปที่ถังขยะแล้ว
}
toast-moved = { $kind ->
    [conversation] ย้ายการสนทนา { $count } รายการแล้ว
   *[message] ย้ายข้อความ { $count } รายการแล้ว
}
toast-starred = { $kind ->
    [conversation] ติดดาวการสนทนา { $count } รายการแล้ว
   *[message] ติดดาวข้อความ { $count } รายการแล้ว
}
toast-unstarred = { $kind ->
    [conversation] นำดาวออกจากการสนทนา { $count } รายการแล้ว
   *[message] นำดาวออกจากข้อความ { $count } รายการแล้ว
}
toast-important = { $kind ->
    [conversation] ทำเครื่องหมายการสนทนา { $count } รายการว่าสำคัญแล้ว
   *[message] ทำเครื่องหมายข้อความ { $count } รายการว่าสำคัญแล้ว
}
toast-not-important = { $kind ->
    [conversation] ทำเครื่องหมายการสนทนา { $count } รายการว่าไม่สำคัญแล้ว
   *[message] ทำเครื่องหมายข้อความ { $count } รายการว่าไม่สำคัญแล้ว
}
toast-pinned = { $kind ->
    [conversation] ปักหมุดการสนทนา { $count } รายการไว้ด้านบนแล้ว
   *[message] ปักหมุดข้อความ { $count } รายการไว้ด้านบนแล้ว
}
toast-unpinned = { $kind ->
    [conversation] เลิกปักหมุดการสนทนา { $count } รายการแล้ว
   *[message] เลิกปักหมุดข้อความ { $count } รายการแล้ว
}
toast-spam = { $kind ->
    [conversation] รายงานการสนทนา { $count } รายการว่าเป็นสแปมแล้ว
   *[message] รายงานข้อความ { $count } รายการว่าเป็นสแปมแล้ว
}
toast-deleted-forever = { $kind ->
    [conversation] ลบการสนทนา { $count } รายการอย่างถาวรแล้ว
   *[message] ลบข้อความ { $count } รายการอย่างถาวรแล้ว
}
toast-undone = เลิกทำการดำเนินการแล้ว
toast-undo = เลิกทำ
toast-no-spam-folder = บัญชีนี้ไม่มีโฟลเดอร์สแปม

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
print-not-downloaded = (ยังไม่ได้ดาวน์โหลด)
print-encrypted = (เข้ารหัสอยู่ เปิดใน Katna Mail เพื่อพิมพ์เนื้อหา)
print-to = ถึง: { $addresses }
print-cc = สำเนา: { $addresses }
