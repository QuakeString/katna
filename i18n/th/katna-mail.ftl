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

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = เปิดข้อความนี้เพื่อดูไฟล์แนบ
text-copy = คัดลอก
text-select-all = เลือกทั้งหมด

## Settings page: its tabs

settings-tab-general = ทั่วไป
settings-tab-inbox = กล่องจดหมาย
settings-tab-accounts = บัญชี
settings-tab-subscriptions = การสมัครรับข้อมูล
settings-tab-appearance = ลักษณะที่ปรากฏ
settings-tab-shortcuts = แป้นพิมพ์ลัด
settings-tab-default-apps = แอปเริ่มต้น
settings-tab-folders-rules = โฟลเดอร์และกฎ
settings-tab-compose = การเขียน
settings-tab-mcp-server = เซิร์ฟเวอร์ MCP
settings-tab-feedback = ความคิดเห็นของผู้ใช้
settings-tab-experimental = ทดลอง

## Settings page: tabs still to come

settings-tab-subscriptions-coming = ดูจดหมายข่าวและรายชื่ออีเมลที่คุณได้รับ และยกเลิกการสมัครได้ในคลิกเดียว
settings-tab-folders-rules-coming = สร้าง เปลี่ยนชื่อ ย้าย และซ่อนโฟลเดอร์และป้ายกำกับ และเลือกว่าจะซิงค์รายการใด กฎจะจัดเรียง ติดป้ายกำกับ ส่งต่อ หรือลบอีเมลใหม่โดยอัตโนมัติ ตามผู้ส่ง หัวเรื่อง หรือคำ
settings-tab-mcp-server-coming = ให้ผู้ช่วย AI ในคอมพิวเตอร์เครื่องนี้ค้นหา อ่าน และร่างอีเมลของคุณได้ โดยคุณเป็นผู้อนุญาต

## Settings > General

settings-general-conversations = มุมมองการสนทนา
settings-general-conversations-group = จัดกลุ่มการตอบกลับอีเมลฉบับเดียวกัน
settings-general-conversations-group-detail = หนึ่งบรรทัดต่อหนึ่งการสนทนาในรายการ
settings-general-reading = การอ่าน
settings-general-newest-first = ข้อความใหม่สุดก่อน
settings-general-newest-first-detail = การสนทนาจะเริ่มจากการตอบกลับล่าสุด
settings-general-full-headers = แสดงส่วนหัวแบบเต็ม
settings-general-full-headers-detail = แสดงจาก ถึง สำเนา วันที่ และหัวเรื่องในทุกข้อความ
settings-general-full-names = ชื่อเต็มของผู้รับ
settings-general-full-names-detail = “ถึง ฉัน, Ada Lovelace” แทน “ถึง ฉัน, Ada”
settings-general-mark-read = ทำเครื่องหมายว่าอ่านแล้ว
settings-general-mark-read-now = ทันทีที่เปิด
settings-general-mark-read-1s = หลังจากเปิดไว้ 1 วินาที
settings-general-mark-read-3s = หลังจากเปิดไว้ 3 วินาที
settings-general-mark-read-never = เฉพาะเมื่อฉันทำเครื่องหมายเอง
settings-general-reply-button = ปุ่มตอบกลับ
settings-general-reply-all = ตอบกลับทุกคน
settings-general-reply-all-detail = ปุ่มตอบกลับข้างแต่ละข้อความจะตอบกลับทุกคน ไม่ใช่แค่ผู้ส่ง
settings-general-remote-images = รูปภาพจากเว็บ
settings-general-remote-images-detail = การโหลดรูปภาพในข้อความจะแจ้งผู้ส่งว่าคุณเปิดข้อความแล้ว เปิดเมื่อใด และอยู่ที่ใดโดยประมาณ หากปิดไว้ แต่ละข้อความจะถามก่อน และคุณแสดงรูปภาพของผู้ส่งได้เสมอ
settings-general-remote-images-always = แสดงรูปภาพเสมอ
settings-general-remote-images-always-detail = ในทุกข้อความ ไม่ใช่เฉพาะจากผู้ส่งที่คุณเชื่อถือ
settings-general-sending = การส่ง
settings-general-sending-detail = ระยะเวลาที่ข้อความที่ส่งจะรอ เพื่อให้ยกเลิกการส่งได้
settings-general-offline = อีเมลแบบออฟไลน์
settings-general-offline-detail = อีเมลล่าสุดจะดาวน์โหลดไว้ทั้งหมด เพื่อให้อ่านได้โดยไม่ต้องเชื่อมต่อ อีเมลที่เก่ากว่าจะดาวน์โหลดเมื่อคุณเปิด
settings-general-offline-days = { $count } วัน
settings-general-offline-years = { $count } ปี
settings-general-offline-all = อีเมลทั้งหมด
settings-general-offline-note = การเลือกจำนวนวันน้อยลงจะเก็บอีเมลที่ดาวน์โหลดไว้แล้ว ไม่มีการเปลี่ยนแปลงใดๆ บนเซิร์ฟเวอร์
settings-general-notifications = การแจ้งเตือน
settings-general-notifications-detail = สำหรับอีเมลใหม่ในกล่องจดหมาย แม้ขณะที่ปิด Katna Mail อยู่
settings-general-new-mail = แจ้งเตือนฉันเมื่อมีอีเมลใหม่
settings-general-new-mail-detail = พร้อมปุ่มตอบกลับทั้งหมด ทำเครื่องหมายว่าอ่านแล้ว และเก็บถาวร
settings-general-new-mail-sound = เล่นเสียง
settings-general-new-mail-sound-detail = เสียงอีเมลใหม่ของเดสก์ท็อป
settings-general-desktop = เดสก์ท็อป
settings-general-open-at-login = เปิด Katna Mail เมื่อเข้าสู่ระบบ
settings-general-open-at-login-detail = อีเมลจะซิงค์เมื่อเข้าสู่ระบบอยู่แล้ว ขณะที่บริการทำงาน
settings-general-tray = แสดง Katna ในถาดระบบ
settings-general-tray-detail = พร้อมจำนวนที่ยังไม่อ่านและเมนู
settings-general-unread-badge = จำนวนที่ยังไม่อ่านบนไอคอนในแถบงาน
settings-general-unread-badge-detail = จำนวนข้อความในกล่องจดหมายที่ยังไม่อ่าน

## Settings > Inbox

settings-inbox-tabs = แท็บกล่องจดหมาย
settings-inbox-tabs-detail = จัดกล่องจดหมายเป็นแท็บ เหมือนที่เว็บไซต์ผู้ให้บริการอีเมลของคุณทำ
settings-inbox-tabs-show = แสดงแท็บกล่องจดหมาย
settings-inbox-tabs-show-detail = หากปิดไว้ จะแสดงรายการเดียวสำหรับทุกบัญชี
settings-inbox-no-accounts = เพิ่มบัญชีเพื่อเลือกแท็บของบัญชีนั้น
settings-inbox-tabs-automatic = อัตโนมัติ: { $tabs } ({ $provider })
settings-inbox-tabs-off = ไม่มีแท็บ
settings-inbox-tabs-gmail = หลัก, โปรโมชัน, โซเชียล, อัปเดต, ฟอรัม
settings-inbox-tabs-focused = โฟกัสและอื่นๆ
settings-inbox-tabs-zoho = กล่องจดหมาย จดหมายข่าว และการแจ้งเตือน
settings-inbox-tabs-shown = แท็บที่แสดง อีเมลของแท็บที่คุณปิดจะอยู่ใน { $tab }

## Settings > Appearance

settings-appearance-reading-pane = บานหน้าต่างการอ่าน
settings-appearance-reading-pane-detail = ตำแหน่งที่แสดงการสนทนาที่เปิด
settings-appearance-pane-right = ด้านขวาของรายการ
settings-appearance-pane-none = ไม่แยก
settings-appearance-density = ความหนาแน่น
settings-appearance-density-default = ค่าเริ่มต้น
settings-appearance-density-compact = กะทัดรัด
settings-appearance-scaling = การปรับขนาด
settings-appearance-scaling-detail = ทำให้ทุกอย่างใน Katna Mail ใหญ่ขึ้นหรือเล็กลง เพิ่มเติมจากการปรับขนาดของเดสก์ท็อปเอง ได้แก่ ข้อความ ไอคอน ระยะห่าง และเส้นแบ่ง อีเมลที่คุณส่งจะใช้ขนาดแบบอักษรของตัวเอง ขนาดที่เล็กมากอาจทำให้คลิกไอคอนได้ยาก
settings-appearance-theme = ธีม
settings-appearance-theme-system = เหมือนเดสก์ท็อป
settings-appearance-theme-light = สว่าง
settings-appearance-theme-dark = มืด
settings-appearance-desktop-colors = สีของเดสก์ท็อป
settings-appearance-desktop-colors-use = ใช้สีของเดสก์ท็อป
settings-appearance-desktop-colors-use-detail = ชุดสีและสีเน้นของเดสก์ท็อป
settings-appearance-app-names = ชื่อแอป
settings-appearance-app-names-show = แสดงชื่อแอป
settings-appearance-app-names-show-detail = ชื่อใต้ไอคอนแอปที่ด้านซ้ายสุด
settings-appearance-sender-pictures = รูปภาพผู้ส่ง
settings-appearance-sender-pictures-show = แสดงโลโก้บริษัท
settings-appearance-sender-pictures-show-detail = ค้นหาจากโดเมนของผู้ส่ง ไม่ใช่จากข้อความ และเก็บไว้หนึ่งสัปดาห์
settings-appearance-important = เครื่องหมายสำคัญ
settings-appearance-important-show = แสดงเครื่องหมายสำคัญ
settings-appearance-important-show-detail = ข้างแต่ละข้อความในรายการ
settings-appearance-message-width = ความกว้างของข้อความ
settings-appearance-message-width-limit = จำกัดความกว้างของข้อความ
settings-appearance-message-width-limit-detail = บรรทัดยาวอ่านง่ายขึ้นในหน้าต่างกว้าง
settings-appearance-mail-colors = สีของอีเมล
settings-appearance-mail-colors-detail = อีเมลส่วนใหญ่ออกแบบมาสำหรับหน้ากระดาษสีขาว เมื่อใช้ธีมมืด สีของอีเมลจะเปลี่ยนเป็นสีเข้มที่อ่านง่าย หากปิดไว้ อีเมลจะใช้สีของผู้ส่งบนหน้าสีสว่าง
settings-appearance-dark-mail = ใช้สีเข้มกับอีเมลด้วย
settings-appearance-dark-mail-detail = เฉพาะขณะที่ใช้ธีมมืด
settings-appearance-attachment-previews = ตัวอย่างไฟล์แนบ
settings-appearance-attachment-previews-show = แสดงตัวอย่างไฟล์แนบ
settings-appearance-attachment-previews-show-detail = ภาพเล็กๆ ของเนื้อหาแต่ละไฟล์บนการ์ดของไฟล์นั้น

## Settings > Default apps

settings-default-apps-intro = แอปที่ใช้เปิดไฟล์แนบเมื่อคุณคลิก ตัวแสดงไฟล์เปิดไฟล์ในแอปอื่นได้เสมอ แอปเริ่มต้นของเดสก์ท็อปตั้งค่าได้ในการตั้งค่าของเดสก์ท็อปเอง
settings-default-apps-pdf = ไฟล์ PDF
settings-default-apps-pdf-detail = แสดงเป็นหน้า พร้อมการซูม
settings-default-apps-pictures = รูปภาพ
settings-default-apps-pictures-detail = รูปถ่าย (หมุนให้ตั้งตรง), PNG, GIF, WebP, BMP, TIFF และ SVG
settings-default-apps-text = ไฟล์ข้อความ
settings-default-apps-text-detail = ข้อความธรรมดา บันทึก โค้ด และข้อความอื่นๆ
settings-default-apps-sheets = สเปรดชีต
settings-default-apps-sheets-detail = Excel (xlsx, xls), OpenDocument (ods) และ CSV
settings-default-apps-documents = เอกสาร
settings-default-apps-documents-detail = Word (docx) และข้อความ OpenDocument (odt)
settings-default-apps-katna = ตัวแสดงไฟล์ของ Katna Mail
settings-default-apps-system = แอปเริ่มต้นของเดสก์ท็อป
settings-default-apps-ask = ถามทุกครั้งว่าจะใช้แอปใด
settings-default-apps-after-saving = หลังจากบันทึก
settings-default-apps-show-folder = แสดงไฟล์ที่บันทึกในโฟลเดอร์
settings-default-apps-show-folder-detail = เปิดตัวจัดการไฟล์โดยเลือกไฟล์แนบที่บันทึกไว้

## Settings > Compose

settings-compose-send-from = ส่งข้อความใหม่จาก
settings-compose-send-from-detail = การตอบกลับและการส่งต่อจะส่งจากบัญชีที่คุณใช้อยู่เสมอ
settings-compose-send-from-current = บัญชีที่คุณใช้อยู่
settings-compose-send-on-replies = การส่งเมื่อตอบกลับ
settings-compose-send-on-replies-detail = สิ่งที่ปุ่มส่งทำเมื่อตอบกลับหรือส่งต่อ เมนูข้างปุ่มส่งมีอีกตัวเลือกหนึ่ง
settings-compose-send-plain = ส่ง
settings-compose-send-archive = ส่งและเก็บถาวร
settings-compose-signatures = ลายเซ็น
settings-compose-signatures-detail = เพิ่มไว้ใต้ข้อความของคุณ หลังบรรทัด “--” เลือกลายเซ็นอื่นได้ในหน้าต่างเขียน
settings-compose-untitled = ไม่มีชื่อ
settings-compose-signature-name = ชื่อ เช่น งาน
settings-compose-signature-first = ลายเซ็นของฉัน
settings-compose-signature-numbered = ลายเซ็น { $number }
settings-compose-signature-delete = ลบ
settings-compose-signature-deleted = ลบลายเซ็นแล้ว
settings-compose-signature-new = สร้างใหม่
settings-compose-no-signatures = ยังไม่มีลายเซ็น
settings-compose-no-signature = ไม่มีลายเซ็น
settings-compose-for-new-mail = สำหรับอีเมลใหม่
settings-compose-for-replies = สำหรับการตอบกลับและการส่งต่อ
settings-compose-for-replies-detail = ในการสนทนาที่คุณเคยลงลายเซ็นในข้อความ การตอบกลับจะเริ่มด้วยลายเซ็นนั้นแทน
settings-compose-format = รูปแบบ
settings-compose-plain-text = เขียนเป็นข้อความธรรมดา
settings-compose-plain-text-detail = อีเมลใหม่จะเริ่มโดยไม่มีการจัดรูปแบบ สลับได้ในหน้าต่างเขียน
settings-compose-spelling = การสะกดคำ
settings-compose-spell-check = ตรวจการสะกดขณะที่ฉันเขียน
settings-compose-spell-check-detail = คำที่สะกดผิดจะมีขีดเส้นใต้ และคลิกขวาเพื่อดูคำแนะนำ
settings-compose-spell-desktop = ภาษาของเดสก์ท็อป ({ $language })
settings-compose-templates = เทมเพลต
settings-compose-templates-detail = บันทึกอีเมลที่คุณเขียนบ่อย และใช้เริ่มอีเมลใหม่หรือการตอบกลับ

## Settings > Shortcuts

settings-shortcuts-set = ชุดแป้นพิมพ์ลัด
settings-shortcuts-set-detail = เริ่มจากแป้นของแอปอีเมลที่คุณคุ้นเคย Cmd ในที่นี้คือ Ctrl การเปลี่ยนแปลงของคุณจะอยู่เหนือชุดแป้น และคืนค่าเริ่มต้นจะกลับไปใช้แป้นของชุดนั้น
settings-shortcuts-single = แป้นพิมพ์ลัดแบบแป้นเดียว
settings-shortcuts-single-detail = แป้นที่ไม่ต้องกด Ctrl หรือ Alt เหมือนในเว็บเมล: e เก็บถาวร, j และ k เลื่อน, / ค้นหา ใช้ได้ในรายการและการสนทนาที่เปิดอยู่ แต่ไม่ทำงานขณะพิมพ์
settings-shortcuts-single-use = ใช้แป้นพิมพ์ลัดแบบแป้นเดียว
settings-shortcuts-single-use-detail = แป้นพิมพ์ลัดที่ใช้ Ctrl ใช้ได้เสมอ
settings-shortcuts-how = คลิกแป้นเพื่อเปลี่ยน หรือคลิก + เพื่อเพิ่ม แล้วกดแป้นใหม่ กด Esc เพื่อยกเลิก
settings-shortcuts-restore = คืนค่าเริ่มต้น
settings-shortcuts-no-key = ไม่มีแป้น
settings-shortcuts-press = กดแป้น…
settings-shortcuts-then = { $keys } แล้ว…
settings-shortcuts-moved = ตอนนี้ { $keys } ทำ “{ $action }” แทน “{ $previous }”
settings-shortcuts-single-off = แป้นพิมพ์ลัดแบบแป้นเดียวปิดอยู่ แป้นนี้จะทำงานเมื่อเปิดใช้
settings-shortcuts-restored = แป้นพิมพ์ลัดทั้งหมดกลับไปใช้แป้นของชุดแล้ว

## Settings search: the line under a result

settings-general-language-summary = ภาษาของแอป วันที่ และตัวเลข
settings-general-reading-summary = ข้อความใหม่สุดก่อน ส่วนหัวแบบเต็ม ชื่อเต็มของผู้รับ
settings-general-mark-read-summary = เวลาที่การสนทนาที่เปิดถูกทำเครื่องหมายว่าอ่านแล้ว: ทันที หลัง 1 หรือ 3 วินาที หรือทำเอง
settings-general-reply-button-summary = ปุ่มตอบกลับข้างแต่ละข้อความจะตอบกลับทุกคน
settings-general-remote-images-summary = แสดงรูปภาพในทุกข้อความเสมอ
settings-general-sending-summary = ยกเลิกการส่ง: ระยะเวลาที่ข้อความที่ส่งจะรอ เพื่อให้ยกเลิกได้
settings-general-offline-summary = จำนวนวันของอีเมลล่าสุดที่ดาวน์โหลดไว้ทั้งหมด เพื่ออ่านได้โดยไม่ต้องเชื่อมต่อ
settings-general-notifications-summary = การแจ้งเตือนอีเมลใหม่และเสียง
settings-general-desktop-summary = เปิด Katna Mail เมื่อเข้าสู่ระบบ ไอคอนในถาดระบบ และจำนวนที่ยังไม่อ่านบนไอคอนในแถบงาน
settings-accounts-accounts-summary = เพิ่มหรือนำบัญชีออก หรือเปลี่ยนรูปภาพของบัญชี
settings-appearance-density-summary = บรรทัดในรายการแบบค่าเริ่มต้นหรือกะทัดรัด
settings-appearance-scaling-summary = ทำให้ทุกอย่างใหญ่ขึ้นหรือเล็กลง: ข้อความ ไอคอน ระยะห่าง และเส้นแบ่ง
settings-appearance-theme-summary = เหมือนเดสก์ท็อป สว่าง หรือมืด
settings-appearance-sender-pictures-summary = โลโก้บริษัท ค้นหาจากโดเมนของผู้ส่ง
settings-appearance-important-summary = เครื่องหมายสำคัญข้างแต่ละข้อความในรายการ
settings-appearance-mail-colors-summary = สีเข้มสำหรับอีเมล HTML ในธีมมืด หรือสีของผู้ส่ง
settings-appearance-attachment-previews-summary = ภาพเล็กๆ ของเนื้อหาไฟล์แนบแต่ละไฟล์
settings-shortcuts-set-summary = เริ่มจากแป้นของ Gmail, Inbox by Gmail, Apple Mail, Outlook หรือ Thunderbird
settings-shortcuts-single-summary = แป้นที่ไม่ต้องกด Ctrl หรือ Alt เหมือนในเว็บเมล
settings-default-apps-pdf-summary = แอปที่ใช้เปิดไฟล์แนบ PDF
settings-default-apps-pictures-summary = แอปที่ใช้เปิดรูปถ่ายและรูปภาพ
settings-default-apps-text-summary = แอปที่ใช้เปิดข้อความธรรมดา บันทึก และโค้ด
settings-default-apps-sheets-summary = แอปที่ใช้เปิดไฟล์ Excel, OpenDocument และ CSV
settings-default-apps-documents-summary = แอปที่ใช้เปิด Word และข้อความ OpenDocument
settings-default-apps-after-saving-summary = แสดงไฟล์แนบที่บันทึกในโฟลเดอร์
settings-compose-send-from-summary = บัญชีที่ใช้ส่งอีเมลใหม่: บัญชีที่คุณใช้อยู่ หรือบัญชีเดิมเสมอ
settings-compose-send-on-replies-summary = ส่ง หรือส่งและเก็บถาวรการสนทนา เมื่อตอบกลับและส่งต่อ
settings-compose-signatures-summary = เพิ่มไว้ใต้ข้อความของคุณ หลังบรรทัด “--”
settings-compose-for-new-mail-summary = ลายเซ็นที่ใช้เริ่มอีเมลใหม่
settings-compose-for-replies-summary = ลายเซ็นที่ใช้เริ่มการตอบกลับและการส่งต่อ
settings-compose-format-summary = เขียนอีเมลใหม่เป็นข้อความธรรมดา
settings-compose-spelling-summary = ตรวจการสะกดขณะเขียน และภาษาของพจนานุกรม
settings-compose-templates-summary = เร็วๆ นี้: บันทึกอีเมลที่คุณเขียนบ่อย และใช้เริ่มอีเมลใหม่หรือการตอบกลับ
settings-feedback-crash-reports-summary = บันทึกรายงานข้อขัดข้องไว้ในคอมพิวเตอร์เครื่องนี้เมื่อ Katna Mail หรือบริการเบื้องหลังขัดข้อง
settings-feedback-saved-summary = ดู คัดลอก หรือลบรายงานข้อขัดข้องที่บันทึกไว้ในคอมพิวเตอร์เครื่องนี้
settings-feedback-help-improve-summary = ส่งรายงานข้อขัดข้องเพื่อช่วยแก้ไขปัญหา ปิดอยู่จนกว่าคุณจะเปิด
settings-experimental-blur-summary = มองเห็นเดสก์ท็อปผ่านแถบด้านบนแบบเบลอ และเมนูเป็นแบบกระจกฝ้า
settings-search-shortcut = แป้นพิมพ์ลัด
settings-search-tab = แท็บการตั้งค่า
settings-search-none = ไม่มีการตั้งค่าที่ตรงกับ “{ $query }”
settings-search-results = การตั้งค่าที่ตรงกับ “{ $query }”

## Quick settings (the panel that slides in from the right)

quick-title = การตั้งค่าด่วน
quick-see-all = ดูการตั้งค่าทั้งหมด
quick-reading-pane = บานหน้าต่างการอ่าน
quick-pane-right = ด้านขวาของรายการ
quick-pane-none = ไม่แยก
quick-density = ความหนาแน่น
quick-density-default = ค่าเริ่มต้น
quick-density-compact = กะทัดรัด
quick-theme = ธีม
quick-theme-system = เหมือนเดสก์ท็อป
quick-theme-light = สว่าง
quick-theme-dark = มืด
quick-desktop-colors = สีของเดสก์ท็อป
quick-desktop-colors-detail = ชุดสีและสีเน้นของเดสก์ท็อป
quick-app-names = ชื่อแอป
quick-app-names-detail = ชื่อใต้ไอคอนแอปที่ด้านซ้ายสุด
quick-inbox-tabs = แท็บกล่องจดหมาย
quick-inbox-tabs-detail = แท็บของผู้ให้บริการอีเมลของแต่ละบัญชี
quick-choose-tabs = เลือกแท็บ
quick-choose-tabs-detail = แยกตามบัญชี ในการตั้งค่า
quick-sending = การส่ง
quick-undo-send = ยกเลิกการส่ง
quick-undo-send-off = ปิด
quick-undo-send-seconds = { $seconds } วิ
quick-signatures = ลายเซ็น
quick-signatures-none = ยังไม่มี
quick-signatures-one = { $name } ใช้เป็นค่าเริ่มต้น
quick-signatures-many = { $count } ลายเซ็น ค่าเริ่มต้นคือ { $name }
quick-signatures-no-default = { $count } ลายเซ็น ไม่มีค่าเริ่มต้น
quick-signature-untitled = ไม่มีชื่อ
quick-threading = การจัดชุดข้อความอีเมล
quick-conversation-view = มุมมองการสนทนา
quick-conversation-view-detail = จัดกลุ่มการตอบกลับอีเมลฉบับเดียวกัน
quick-help = ความช่วยเหลือ
quick-tour = ชมแนะนำการใช้งาน
quick-whats-new = มีอะไรใหม่
quick-about = เกี่ยวกับ Katna

## Settings: opening at login

settings-open-at-login-failed = เปลี่ยนการเปิดเมื่อเข้าสู่ระบบไม่ได้: { $error }

## Settings > Appearance > Scaling

scale-letter = ก
scale-percent = { $percent }%
scale-reset = กลับไปที่ { $percent }%

## Settings > Experimental > Look & Feel

look-intro = ฟีเจอร์ที่ยังอยู่ระหว่างทดลอง อาจเปลี่ยนแปลงหรือถูกนำออก
look-heading = รูปลักษณ์และความรู้สึก
look-window-frame = กรอบหน้าต่าง
look-window-frame-detail = ผู้วาดแถบชื่อเรื่อง ปุ่มหน้าต่าง มุม และเงา
look-frame-native-kde = ดั้งเดิม: กรอบของ KDE ตามธีม Plasma ของคุณ
look-frame-native = ดั้งเดิม: กรอบของเดสก์ท็อป
look-frame-katna = Katna: แถบด้านบนกลายเป็นแถบชื่อเรื่อง
look-frame-katna-note-named = Katna จะวาดมุมโค้งมนและเงาของตัวเอง กรอบจะไม่ใช้ธีมของ { $desktop } อีกต่อไป แต่กฎหน้าต่างยังคงมีผล
look-frame-katna-note = Katna จะวาดมุมโค้งมนและเงาของตัวเอง กรอบจะไม่ใช้ธีมของเดสก์ท็อปอีกต่อไป แต่กฎหน้าต่างยังคงมีผล
look-frame-client-side = เดสก์ท็อปของคุณให้แต่ละแอปวาดกรอบเอง Katna จึงวาดกรอบของตัวเองอยู่แล้ว
look-blurred-background = พื้นหลังแบบเบลอ
look-blurred-background-detail = มองเห็นเดสก์ท็อปผ่านแถบด้านบนและโฟลเดอร์แบบเบลอ และเมนูกับป๊อปโอเวอร์เป็นแบบกระจกฝ้า
look-blur = เบลอสิ่งที่อยู่ด้านหลังหน้าต่าง
look-blur-detail = อีเมลยังคงอยู่บนการ์ดทึบ ข้อความจึงยังคงมีความคมชัด
look-blur-off-kde = เอฟเฟกต์เบลอของ KDE ปิดอยู่ เปิด “เบลอ” ใน “การตั้งค่าระบบ” > “การจัดการหน้าต่าง” > “ลูกเล่นพื้นโต๊ะ” แล้วเปิด Katna Mail อีกครั้ง
look-blur-none-gnome = GNOME ไม่เบลอสิ่งที่อยู่ด้านหลังหน้าต่าง
look-blur-none-x11 = ตัวจัดการหน้าต่างของคุณไม่เบลอสิ่งที่อยู่ด้านหลังหน้าต่าง
look-blur-none-wayland = คอมโพสิเตอร์ของคุณไม่เบลอสิ่งที่อยู่ด้านหลังหน้าต่าง

## Settings > User feedback (crash reports)

feedback-intro-sending = รายงานข้อขัดข้องใหม่จะถูกส่งเพื่อช่วยแก้ไขปัญหา ไม่มีข้อมูลอื่นออกจากคอมพิวเตอร์เครื่องนี้
feedback-intro-local = Katna ไม่ส่งข้อมูลใดๆ ไปที่ไหน รายงานข้อขัดข้องจะอยู่ในคอมพิวเตอร์เครื่องนี้ ให้คุณดูหรือแนบไปกับรายงานข้อบกพร่อง
feedback-crash-reports = รายงานข้อขัดข้อง
feedback-crash-reports-detail = สร้างขึ้นเมื่อ Katna Mail หรือบริการเบื้องหลังขัดข้อง
feedback-save = บันทึกรายงานข้อขัดข้องไว้ในคอมพิวเตอร์เครื่องนี้
feedback-save-detail = ไม่รวมโฟลเดอร์หลัก ชื่อผู้ใช้ ชื่อคอมพิวเตอร์ และที่อยู่อีเมลของคุณ
feedback-saved = รายงานข้อขัดข้องที่บันทึกไว้
feedback-saved-detail = เก็บรายงานล่าสุดไว้ { $count } ฉบับ
feedback-help-improve = ช่วยปรับปรุง Katna
feedback-help-improve-detail = ปิดอยู่จนกว่าคุณจะเปิด และคุณปิดได้ที่นี่ทุกเมื่อ
feedback-send = ส่งรายงานข้อขัดข้อง
feedback-send-detail = รายงานที่บันทึกไว้จะถูกส่งไปยังระบบติดตามข้อขัดข้องของ Katna (Sentry ในสหภาพยุโรป) ตรงตามที่คุณดูได้ที่นี่ ไม่มีที่อยู่ IP ข้อความ หรือที่อยู่อีเมล
feedback-none-saved = ไม่มีรายงานข้อขัดข้องที่บันทึกไว้
feedback-delete-all = ลบทั้งหมด
feedback-app-daemon = บริการเบื้องหลัง
feedback-report-sent = { $date } · ส่งแล้ว
feedback-view = ดู
feedback-view-tooltip = เปิดรายงาน
feedback-copy-tooltip = คัดลอกเพื่อวางในรายงานข้อบกพร่อง
feedback-copied = คัดลอกรายงานข้อขัดข้องแล้ว
feedback-deleted-all = ลบรายงานข้อขัดข้องแล้ว
feedback-read-failed = อ่านรายงานข้อขัดข้องไม่ได้: { $error }
feedback-delete-failed = ลบรายงานข้อขัดข้องไม่ได้: { $error }
feedback-delete-all-failed = ลบรายงานข้อขัดข้องไม่ได้: { $error }

## Menu bar (the KDE global menu)

desktop-menu-file = _ไฟล์
desktop-menu-new-message = _ข้อความใหม่
desktop-menu-quit = _ออก
desktop-menu-edit = _แก้ไข
desktop-menu-undo = _เลิกทำ
desktop-menu-select-all = _เลือกทั้งหมด
desktop-menu-select-none = _ไม่เลือกเลย
desktop-menu-find = _ค้นหา…
desktop-menu-view = _มุมมอง
desktop-menu-folder-list = _แสดงรายการโฟลเดอร์
desktop-menu-refresh = _รีเฟรช
desktop-menu-go = _ไปที่
desktop-menu-inbox = _กล่องจดหมาย
desktop-menu-starred = _ที่ติดดาว
desktop-menu-sent = _ส่งแล้ว
desktop-menu-drafts = _ฉบับร่าง
desktop-menu-all-mail = _จดหมายทั้งหมด
desktop-menu-next = _การสนทนาถัดไป
desktop-menu-previous = _การสนทนาก่อนหน้า
desktop-menu-message = _ข้อความ
desktop-menu-open = _เปิด
desktop-menu-reply = _ตอบกลับ
desktop-menu-reply-all = _ตอบกลับทั้งหมด
desktop-menu-forward = _ส่งต่อ
desktop-menu-archive = _เก็บถาวร
desktop-menu-delete = _ลบ
desktop-menu-spam = _รายงานสแปม
desktop-menu-move-to = _ย้ายไปที่…
desktop-menu-mark-read = _ทำเครื่องหมายว่าอ่านแล้ว
desktop-menu-mark-unread = _ทำเครื่องหมายว่ายังไม่อ่าน
desktop-menu-star = _ติดดาว
desktop-menu-important = _ทำเครื่องหมายว่าสำคัญ
desktop-menu-not-important = _ทำเครื่องหมายว่าไม่สำคัญ
desktop-menu-settings = _การตั้งค่า
desktop-menu-quick-settings = _การตั้งค่าด่วน
desktop-menu-configure = _กำหนดค่า Katna Mail…
desktop-menu-help = _ความช่วยเหลือ
desktop-menu-shortcuts = _แป้นพิมพ์ลัด
desktop-menu-whats-new = _มีอะไรใหม่
desktop-menu-about = _เกี่ยวกับ Katna

## Settings > Keyboard shortcuts: the groups of the list

shortcut-group-moving = การเลื่อนไปมา
shortcut-group-actions = การดำเนินการ
shortcut-group-go-to = ไปที่
shortcut-group-app = แอปพลิเคชัน

## Settings > Keyboard shortcuts: what each shortcut does

shortcut-next = การสนทนาถัดไป
shortcut-previous = การสนทนาก่อนหน้า
shortcut-down = เลื่อนลงในรายการ
shortcut-up = เลื่อนขึ้นในรายการ
shortcut-first = รายการแรกในรายการ
shortcut-last = รายการสุดท้ายในรายการ
shortcut-page-down = เลื่อนรายการลงหนึ่งหน้า
shortcut-page-up = เลื่อนรายการขึ้นหนึ่งหน้า
shortcut-open = เปิดการสนทนา
shortcut-back = กลับไปที่รายการ
shortcut-scroll-down = เลื่อนลง
shortcut-scroll-up = เลื่อนขึ้น
shortcut-scroll-page-down = เลื่อนลงหนึ่งหน้า
shortcut-scroll-page-up = เลื่อนขึ้นหนึ่งหน้า
shortcut-compose = เขียน
shortcut-reply = ตอบกลับ
shortcut-reply-all = ตอบกลับทั้งหมด
shortcut-forward = ส่งต่อ
shortcut-archive = เก็บถาวร
shortcut-delete = ลบ
shortcut-spam = รายงานสแปม
shortcut-move-to = ย้ายไปที่
shortcut-mark-read = ทำเครื่องหมายว่าอ่านแล้ว
shortcut-mark-unread = ทำเครื่องหมายว่ายังไม่อ่าน
shortcut-star = ติดดาวหรือนำดาวออก
shortcut-important = ทำเครื่องหมายว่าสำคัญ
shortcut-not-important = ทำเครื่องหมายว่าไม่สำคัญ
shortcut-check = เลือกการสนทนา
shortcut-select-all = เลือกการสนทนาทั้งหมด
shortcut-select-none = ยกเลิกการเลือกการสนทนาทั้งหมด
shortcut-undo = เลิกทำการดำเนินการล่าสุด
shortcut-go-inbox = กล่องจดหมาย
shortcut-go-starred = ที่ติดดาว
shortcut-go-sent = ส่งแล้ว
shortcut-go-drafts = ฉบับร่าง
shortcut-go-all = จดหมายทั้งหมด
shortcut-search = ค้นหาอีเมล
shortcut-navigation = แสดงหรือพับเมนู
shortcut-quick-settings = การตั้งค่าด่วน
shortcut-settings = การตั้งค่าทั้งหมด
shortcut-shortcuts = แป้นพิมพ์ลัด
shortcut-reload = ตรวจหาอีเมลใหม่
shortcut-quit = ออก

## Keys pressed one after another, as a shortcut shows them ("G then I")

shortcut-sequence = { $first } แล้ว { $second }

## Settings > Accounts

accounts-folder-pane = บานหน้าต่างโฟลเดอร์
accounts-folder-pane-detail = บัญชีที่จะแสดงโฟลเดอร์ในบานหน้าต่างด้านซ้าย
accounts-shown-one = ทีละบัญชี สลับได้ที่การ์ดบัญชี
accounts-shown-all = ทุกบัญชี เรียงต่อกัน
accounts-row = บัญชี
accounts-row-detail = การนำบัญชีออกจะลบสำเนาอีเมลของบัญชีนั้นที่ Katna เก็บไว้ในคอมพิวเตอร์เครื่องนี้ อีเมลยังคงอยู่บนเซิร์ฟเวอร์
accounts-none = ยังไม่มีบัญชี
accounts-kind-imported = นำเข้า
accounts-picture-reset = ใช้รูปภาพของเดสก์ท็อป
accounts-picture-change = เปลี่ยนรูปภาพ
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
