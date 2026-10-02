# Katna Mail, Thai (ไทย).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = ค้นหาไฟล์

## Left side (and chips on a phone)

files-all = ไฟล์ทั้งหมด
files-pictures = รูปภาพ
files-pdfs = PDF
files-documents = เอกสาร
files-sheets = สเปรดชีต
files-slides = สไลด์
files-other = อื่นๆ
files-accounts = บัญชี
files-drives = ไดรฟ์
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = แชร์กับฉัน
files-shown = ที่แสดง
files-received = ที่ได้รับ
files-sent = ที่ฉันส่ง

## Over the files

files-count = { $count ->
   *[other] { $count } ไฟล์ · { $size }
}
files-anyone = ทุกคน
files-from-person = จาก { $name }
files-time-any = ทุกช่วงเวลา
files-time-today = วันนี้
files-time-yesterday = เมื่อวาน
files-time-this-week = สัปดาห์นี้
files-time-last-week = สัปดาห์ที่แล้ว
files-time-this-month = เดือนนี้
files-time-last-month = เดือนที่แล้ว
files-time-between = { $first } – { $last }
files-time-hint = คลิกวัน หรือลากผ่านหลายวัน
files-time-summary = { $count ->
   *[other] { $days } · { $count } ไฟล์
}
files-time-clear = ล้าง
files-time-month-back = เดือนก่อนหน้า
files-time-month-on = เดือนถัดไป
files-time-wheel = เลื่อนเพื่อขยับวันที่เหล่านี้ โดยคงช่วงเวลาไว้
files-sort-newest = ใหม่สุดก่อน
files-sort-oldest = เก่าสุดก่อน
files-sort-largest = ใหญ่สุดก่อน
files-sort-name = ตามชื่อ
files-grid = การ์ด
files-list = รายการ
files-this-week = สัปดาห์นี้
files-undated = ไม่มีวันที่
files-me = ฉัน
files-no-subject = (ไม่มีหัวเรื่อง)
files-loading = กำลังรวบรวมไฟล์จากอีเมลของคุณ…
files-empty = ไฟล์จากอีเมลของคุณจะแสดงที่นี่
files-none-match = ไม่มีไฟล์ที่ตรงกัน
files-load-failed = อ่านไฟล์ไม่ได้: { $error }

## A file's menu and buttons

files-open = เปิด
files-open-with = เปิดด้วย…
files-save = บันทึก…
files-show-mail = แสดงอีเมล
files-mail-window = เปิดอีเมลในหน้าต่างใหม่
files-forward = ส่งต่อไฟล์
files-from-them = ไฟล์จาก { $name }
files-copy-name = คัดลอกชื่อไฟล์
files-name-copied = คัดลอกชื่อไฟล์แล้ว
files-downloading = กำลังดาวน์โหลดอีเมล…
files-download-failed = ดาวน์โหลดอีเมลนี้ไม่ได้

## A cloud drive in place of the mail files

files-drive-mine = ไดรฟ์ของฉัน
files-drive-mine-onedrive = ไฟล์ของฉัน
files-drive-results = “{ $words }”
files-drive-count = { $folders ->
    [0] { $files ->
        [one] 1 ไฟล์
       *[other] { $files } ไฟล์
    }
    [one] 1 โฟลเดอร์ · { $files ->
        [one] 1 ไฟล์
       *[other] { $files } ไฟล์
    }
   *[other] { $folders } โฟลเดอร์ · { $files ->
        [one] 1 ไฟล์
       *[other] { $files } ไฟล์
    }
}
files-drive-folders = โฟลเดอร์
files-drive-files = ไฟล์
files-drive-folder = โฟลเดอร์
files-drive-meta = { $what } · แก้ไขเมื่อ { $date }
files-drive-as-link = { $what } · เป็นลิงก์
files-drive-google-doc = Google เอกสาร
files-drive-google-sheet = Google ชีต
files-drive-google-slides = Google สไลด์
files-drive-google-drawing = Google ภาพวาด
files-drive-fetching = กำลังดึงไฟล์…
files-drive-loading = กำลังเปิดไดรฟ์…
files-drive-empty = โฟลเดอร์นี้ว่างเปล่า
files-drive-unreachable = เชื่อมต่อ { $drive } ไม่ได้
files-drive-try-again = ลองอีกครั้ง
files-drive-needs-permission = Katna ต้องได้รับอนุญาตจากคุณหนึ่งครั้งเพื่อแสดงไดรฟ์นี้ ลงชื่อเข้าใช้อีกครั้งและอนุญาตให้ Katna ดูไฟล์ของคุณ
files-drive-allow = อนุญาต
files-drive-allow-failed = การลงชื่อเข้าใช้ไม่เสร็จสมบูรณ์ ไดรฟ์จึงยังปิดอยู่
files-drive-attach = แนบ
files-drive-more = เพิ่มเติม
files-drive-download = ดาวน์โหลด…
files-drive-open-web = เปิดใน { $drive }
files-drive-copy-link = คัดลอกลิงก์
files-drive-link-copied = คัดลอกลิงก์แล้ว
files-drive-share = แชร์…
files-drive-rename = เปลี่ยนชื่อ
files-drive-trash = ย้ายไปที่ถังขยะ
files-drive-trashed = “{ $name }” อยู่ในถังขยะของ { $drive } แล้ว
files-drive-renamed = เปลี่ยนชื่อเป็น “{ $name }” แล้ว
files-drive-getting = กำลังดึง { $name } จาก { $drive }…
files-drive-get-failed = ดึง { $name } ไม่ได้: { $error }
files-drive-upload = อัปโหลด
files-drive-upload-files = อัปโหลดไฟล์
files-drive-upload-folder = อัปโหลดโฟลเดอร์
files-drive-upload-failed = อัปโหลด { $name } ไม่ได้: { $error }
files-drive-upload-needs = หากต้องการอัปโหลด Katna ต้องได้รับอนุญาตจากคุณหนึ่งครั้ง: กด อนุญาต ใน การตั้งค่า › แอปเริ่มต้น › หน้าไฟล์

## The Share dialog of a drive file or folder

files-share-title = แชร์ “{ $name }”
files-share-add = เพิ่มผู้คนด้วยชื่อหรือที่อยู่อีเมล
files-share-not-address = “{ $text }” ไม่ใช่ที่อยู่อีเมล
files-share-notify = ให้ { $drive } ส่งอีเมลแจ้งพวกเขาด้วย
files-share-people = ผู้ที่มีสิทธิ์เข้าถึง
files-share-general = การเข้าถึงทั่วไป
files-share-loading = กำลังอ่านว่าใครมีสิทธิ์เข้าถึง…
files-share-restricted = จำกัด
files-share-restricted-about = เฉพาะผู้ที่มีสิทธิ์เข้าถึงเท่านั้นที่เปิดด้วยลิงก์ได้
files-share-anyone = ทุกคนที่มีลิงก์
files-share-anyone-can = { $role ->
    [editor] ทุกคนที่มีลิงก์แก้ไขได้
    [commenter] ทุกคนที่มีลิงก์แสดงความคิดเห็นได้
   *[viewer] ทุกคนที่มีลิงก์ดูได้
}
files-share-anyone-about = { $role ->
    [editor] ทุกคนบนอินเทอร์เน็ตที่มีลิงก์แก้ไขได้
    [commenter] ทุกคนบนอินเทอร์เน็ตที่มีลิงก์แสดงความคิดเห็นได้
   *[viewer] ทุกคนบนอินเทอร์เน็ตที่มีลิงก์ดูได้
}
files-share-role-owner = เจ้าของ
files-share-role-editor = ผู้แก้ไข
files-share-role-commenter = ผู้แสดงความคิดเห็น
files-share-role-viewer = ผู้ดู
files-share-you = { $name } (คุณ)
files-share-domain = ทุกคนที่ { $domain }
files-share-inherited = สิทธิ์เข้าถึงจากโฟลเดอร์ที่ไฟล์นี้อยู่
files-share-remove = นำสิทธิ์เข้าถึงออก
files-share-copy-link = คัดลอกลิงก์
files-share-share = แชร์
files-share-done = เสร็จสิ้น
files-share-close = ปิด
files-share-sharing = กำลังแชร์…
files-share-shared = { $count ->
   *[other] แชร์กับ { $count } คนแล้ว
}
files-share-refused = { $drive } แชร์กับ { $addresses } ไม่ได้
files-share-failed = เปลี่ยนการแชร์ไม่ได้: { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
   *[other] กำลังอัปโหลด { $count } รายการ
}
files-tray-done = { $count ->
   *[other] อัปโหลดเสร็จ { $count } รายการ
}
files-tray-some-failed = อัปโหลดแล้ว { $done } รายการ ไม่สำเร็จ { $failed } รายการ
files-tray-minutes-left = { $minutes ->
   *[other] เหลืออีกประมาณ { $minutes } นาที
}
files-tray-seconds-left = เหลืออีกไม่ถึงหนึ่งนาที
files-tray-starting = กำลังเริ่ม…
files-tray-cancel-all = ยกเลิกทั้งหมด
files-tray-cancel = ยกเลิก
files-tray-fold = ซ่อนรายการ
files-tray-unfold = แสดงรายการ
files-tray-close = ปิด
files-tray-progress = { $place } · { $sent } จาก { $size }
files-tray-in = ใน { $place }
files-tray-cancelled = ยกเลิกแล้ว
