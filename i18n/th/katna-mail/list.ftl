# Katna Mail, Thai (ไทย).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

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
tab-provider-other = จัดเรียงโดย Katna

## Mail list: toolbar

list-select = เลือก
list-refresh = รีเฟรช
list-back-to-top = กลับไปด้านบน
list-checking = กำลังตรวจสอบอีเมลใหม่…
list-more = เพิ่มเติม
list-mark-read = ทำเครื่องหมายว่าอ่านแล้ว
list-mark-unread = ทำเครื่องหมายว่ายังไม่อ่าน
list-move-to = ย้ายไปที่
list-archive = เก็บถาวร
list-spam = รายงานสแปม
list-delete = ลบ
list-snooze = เลื่อนเวลา
list-unsnooze = ยกเลิกการเลื่อนเวลา
list-newer = ใหม่กว่า
list-older = เก่ากว่า
list-range = { $first }–{ $last } จาก { $total }
list-range-about = { $first }–{ $last } จากประมาณ { $total }
list-results = ผลการค้นหาสำหรับ “{ $query }”
list-results-corrected = กำลังแสดงผลการค้นหาสำหรับ “{ $query }”
list-search-instead = ค้นหา “{ $query }” แทน
list-search-no-index = การค้นหายังไม่พร้อม: ยังไม่ได้สร้างดัชนี
list-search-not-ready = การค้นหายังไม่พร้อม: { $error }
list-files-more = +{ $count }
list-replied = คุณตอบกลับแล้ว

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
list-selected-picked-screen = { $pick ->
    [read] { $kind ->
        [conversation] เลือกการสนทนาที่อ่านแล้วทั้ง { $count } รายการบนหน้าจอแล้ว
       *[message] เลือกข้อความที่อ่านแล้วทั้ง { $count } รายการบนหน้าจอแล้ว
    }
   *[unread] { $kind ->
        [conversation] เลือกการสนทนาที่ยังไม่อ่านทั้ง { $count } รายการบนหน้าจอแล้ว
       *[message] เลือกข้อความที่ยังไม่อ่านทั้ง { $count } รายการบนหน้าจอแล้ว
    }
    [starred] { $kind ->
        [conversation] เลือกการสนทนาที่ติดดาวทั้ง { $count } รายการบนหน้าจอแล้ว
       *[message] เลือกข้อความที่ติดดาวทั้ง { $count } รายการบนหน้าจอแล้ว
    }
    [unstarred] { $kind ->
        [conversation] เลือกการสนทนาที่ไม่ติดดาวทั้ง { $count } รายการบนหน้าจอแล้ว
       *[message] เลือกข้อความที่ไม่ติดดาวทั้ง { $count } รายการบนหน้าจอแล้ว
    }
}
list-select-picked = { $pick ->
    [read] { $kind ->
        [conversation] เลือกการสนทนาที่อ่านแล้วทั้ง { $count } รายการ
       *[message] เลือกข้อความที่อ่านแล้วทั้ง { $count } รายการ
    }
   *[unread] { $kind ->
        [conversation] เลือกการสนทนาที่ยังไม่อ่านทั้ง { $count } รายการ
       *[message] เลือกข้อความที่ยังไม่อ่านทั้ง { $count } รายการ
    }
    [starred] { $kind ->
        [conversation] เลือกการสนทนาที่ติดดาวทั้ง { $count } รายการ
       *[message] เลือกข้อความที่ติดดาวทั้ง { $count } รายการ
    }
    [unstarred] { $kind ->
        [conversation] เลือกการสนทนาที่ไม่ติดดาวทั้ง { $count } รายการ
       *[message] เลือกข้อความที่ไม่ติดดาวทั้ง { $count } รายการ
    }
}
list-select-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] เลือกการสนทนาที่อ่านแล้วทั้ง { $count } รายการใน { $folder }
       *[message] เลือกข้อความที่อ่านแล้วทั้ง { $count } รายการใน { $folder }
    }
   *[unread] { $kind ->
        [conversation] เลือกการสนทนาที่ยังไม่อ่านทั้ง { $count } รายการใน { $folder }
       *[message] เลือกข้อความที่ยังไม่อ่านทั้ง { $count } รายการใน { $folder }
    }
    [starred] { $kind ->
        [conversation] เลือกการสนทนาที่ติดดาวทั้ง { $count } รายการใน { $folder }
       *[message] เลือกข้อความที่ติดดาวทั้ง { $count } รายการใน { $folder }
    }
    [unstarred] { $kind ->
        [conversation] เลือกการสนทนาที่ไม่ติดดาวทั้ง { $count } รายการใน { $folder }
       *[message] เลือกข้อความที่ไม่ติดดาวทั้ง { $count } รายการใน { $folder }
    }
}
list-selected-picked = { $pick ->
    [read] { $kind ->
        [conversation] เลือกการสนทนาที่อ่านแล้วทั้ง { $count } รายการแล้ว
       *[message] เลือกข้อความที่อ่านแล้วทั้ง { $count } รายการแล้ว
    }
   *[unread] { $kind ->
        [conversation] เลือกการสนทนาที่ยังไม่อ่านทั้ง { $count } รายการแล้ว
       *[message] เลือกข้อความที่ยังไม่อ่านทั้ง { $count } รายการแล้ว
    }
    [starred] { $kind ->
        [conversation] เลือกการสนทนาที่ติดดาวทั้ง { $count } รายการแล้ว
       *[message] เลือกข้อความที่ติดดาวทั้ง { $count } รายการแล้ว
    }
    [unstarred] { $kind ->
        [conversation] เลือกการสนทนาที่ไม่ติดดาวทั้ง { $count } รายการแล้ว
       *[message] เลือกข้อความที่ไม่ติดดาวทั้ง { $count } รายการแล้ว
    }
}
list-selected-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] เลือกการสนทนาที่อ่านแล้วทั้ง { $count } รายการใน { $folder } แล้ว
       *[message] เลือกข้อความที่อ่านแล้วทั้ง { $count } รายการใน { $folder } แล้ว
    }
   *[unread] { $kind ->
        [conversation] เลือกการสนทนาที่ยังไม่อ่านทั้ง { $count } รายการใน { $folder } แล้ว
       *[message] เลือกข้อความที่ยังไม่อ่านทั้ง { $count } รายการใน { $folder } แล้ว
    }
    [starred] { $kind ->
        [conversation] เลือกการสนทนาที่ติดดาวทั้ง { $count } รายการใน { $folder } แล้ว
       *[message] เลือกข้อความที่ติดดาวทั้ง { $count } รายการใน { $folder } แล้ว
    }
    [unstarred] { $kind ->
        [conversation] เลือกการสนทนาที่ไม่ติดดาวทั้ง { $count } รายการใน { $folder } แล้ว
       *[message] เลือกข้อความที่ไม่ติดดาวทั้ง { $count } รายการใน { $folder } แล้ว
    }
}
list-picked-none = { $pick ->
    [read] { $kind ->
        [conversation] ไม่มีการสนทนาที่อ่านแล้วที่นี่
       *[message] ไม่มีข้อความที่อ่านแล้วที่นี่
    }
   *[unread] { $kind ->
        [conversation] ไม่มีการสนทนาที่ยังไม่อ่านที่นี่
       *[message] ไม่มีข้อความที่ยังไม่อ่านที่นี่
    }
    [starred] { $kind ->
        [conversation] ไม่มีการสนทนาที่ติดดาวที่นี่
       *[message] ไม่มีข้อความที่ติดดาวที่นี่
    }
    [unstarred] { $kind ->
        [conversation] ไม่มีการสนทนาที่ไม่ติดดาวที่นี่
       *[message] ไม่มีข้อความที่ไม่ติดดาวที่นี่
    }
}
list-clear-selection = ล้างการเลือก

## Mail list: empty states

list-empty-search = ไม่มีข้อความที่ตรงกับการค้นหาของคุณ
list-empty-tab = ไม่มีอีเมลใน { $tab }
list-empty-tab-unknown = ไม่มีอีเมลในแท็บนี้
list-empty-folder = ไม่มีข้อความใน { $folder }
list-empty-folder-unknown = ไม่มีข้อความในโฟลเดอร์นี้
list-empty-waiting = ไม่มีอีเมลที่รอการตอบกลับ
list-empty-reminders = ไม่มีการช่วยเตือน กด H บนอีเมลเพื่อเพิ่ม
list-first-sync = กำลังรับอีเมลของคุณ…
list-first-sync-detail = อีเมลจะแสดงที่นี่เมื่อมาถึง
list-store-unreadable = เปิดที่เก็บอีเมลไม่ได้

## Mail list: lines

row-no-subject = (ไม่มีหัวเรื่อง)
row-unknown-sender = (ไม่ทราบผู้ส่ง)
row-to = ถึง:
row-no-recipients = (ไม่มีผู้รับ)
row-removed = ข้อความนี้ถูกนำออกแล้ว
row-starred = ติดดาวแล้ว
row-not-starred = ไม่ได้ติดดาว
row-important = สำคัญ คลิกเพื่อทำเครื่องหมายว่าไม่สำคัญ
row-mark-important = ทำเครื่องหมายว่าสำคัญ
row-pinned = ปักหมุดไว้ด้านบน
row-task = งาน
row-task-open = เปิดงาน: { $title }
row-tracking-none = ติดตามอยู่ ยังไม่มีใครเปิด
row-tracking-opened = เปิดแล้ว { $opened } จาก { $recipients } คน
row-tracking-clicked = เปิดแล้ว { $opened } จาก { $recipients } คน คลิกลิงก์ { $clicked } คน
row-pin = ปักหมุดไว้ด้านบน
row-unpin = เลิกปักหมุด
row-snoozed-until = เลื่อนเวลาไว้จนถึง { $when }
row-snoozed-day-time = { $day } { $time }
snoozed-group-today = วันนี้
snoozed-group-tomorrow = พรุ่งนี้
snoozed-group-this-week = สัปดาห์นี้
snoozed-group-later = ภายหลัง
row-follow-up-step = ติดตามครั้งที่ { $step } จาก { $steps } · { $date }
row-follow-up-waiting = การติดตามรออยู่
row-reminder = เตือน { $date }

## Mail list: More menu and right-click menu

menu-reply = ตอบกลับ
menu-reply-all = ตอบกลับทั้งหมด
menu-forward = ส่งต่อ
menu-archive = เก็บถาวร
menu-delete = ลบ
menu-delete-forever = ลบอย่างถาวร
menu-move-to-inbox = ย้ายไปที่กล่องจดหมาย
menu-spam = รายงานสแปม
menu-not-spam = ไม่ใช่สแปม
menu-mark-read = ทำเครื่องหมายว่าอ่านแล้ว
menu-mark-unread = ทำเครื่องหมายว่ายังไม่อ่าน
menu-mark-all-read = ทำเครื่องหมายทั้งหมดว่าอ่านแล้ว
menu-star = ติดดาว
menu-unstar = นำดาวออก
menu-important = ทำเครื่องหมายว่าสำคัญ
menu-not-important = ทำเครื่องหมายว่าไม่สำคัญ
menu-pin = ปักหมุดไว้ด้านบน
menu-unpin = เลิกปักหมุด
menu-snooze = เลื่อนเวลา
menu-remind = เตือนฉัน
menu-unsnooze = ยกเลิกการเลื่อนเวลา
menu-add-to-tasks = เพิ่มในงาน
menu-schedule-meeting = นัดประชุม
menu-start-call = เริ่มการประชุมทางวิดีโอ
menu-add-note = เพิ่มโน้ต
menu-print-all = พิมพ์ทั้งหมด
menu-new-window = เปิดในหน้าต่างใหม่
menu-move-to = ย้ายไปที่
menu-follow-up = ติดตามผล
menu-more = เพิ่มเติม
menu-move-to-heading = ย้ายไปที่:
menu-move-to-search = ย้ายไปที่…
menu-label-as = ติดป้ายกำกับ
menu-label-as-search = ติดป้ายกำกับ…
menu-no-folder = ไม่มีโฟลเดอร์ชื่อ “{ $name }”
menu-no-label = ไม่มีป้ายกำกับชื่อ “{ $name }”
menu-create-folder = สร้าง “{ $name }”
menu-always-move = ย้ายอีเมลจาก { $name } มาที่นี่เสมอ
toast-always-move-failed = ย้ายอีเมลแล้ว แต่สร้างกฎไม่สำเร็จ: { $error }
drag-mail = { $kind ->
    [conversation] { $count ->
        [one] การสนทนา { $count } รายการ
       *[other] การสนทนา { $count } รายการ
    }
   *[message] { $count ->
        [one] ข้อความ { $count } รายการ
       *[other] ข้อความ { $count } รายการ
    }
}
menu-find-from = ค้นหาอีเมลจาก { $name }
menu-make-rule = สร้างกฎ…

## Snackbar after an action on mail in the list

toast-key-imported = นำเข้าคีย์แล้ว
toast-key-updated = คุณมีคีย์นี้อยู่แล้ว ตอนนี้คีย์เป็นปัจจุบันแล้ว
toast-key-removed = นำคีย์ออกแล้ว
toast-key-not-removed = นำคีย์ออกไม่ได้
toast-fingerprint-copied = คัดลอกลายนิ้วมือแล้ว
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
toast-label-added = เพิ่มป้ายกำกับ “{ $label }” แล้ว
toast-label-removed = นำป้ายกำกับ “{ $label }” ออกแล้ว
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
toast-snoozed = { $kind ->
    [conversation] เลื่อนเวลาการสนทนา { $count } รายการไว้จนถึง { $when } แล้ว
   *[message] เลื่อนเวลาข้อความ { $count } รายการไว้จนถึง { $when } แล้ว
}
toast-unsnoozed = { $kind ->
    [conversation] การสนทนา { $count } รายการกลับมาที่กล่องจดหมายแล้ว
   *[message] ข้อความ { $count } รายการกลับมาที่กล่องจดหมายแล้ว
}
toast-spam = { $kind ->
    [conversation] รายงานการสนทนา { $count } รายการว่าเป็นสแปมแล้ว
   *[message] รายงานข้อความ { $count } รายการว่าเป็นสแปมแล้ว
}
toast-not-spam = { $kind ->
    [conversation] ทำเครื่องหมายการสนทนา { $count } รายการว่าไม่ใช่สแปมและย้ายไปที่กล่องจดหมายแล้ว
   *[message] ทำเครื่องหมายข้อความ { $count } รายการว่าไม่ใช่สแปมและย้ายไปที่กล่องจดหมายแล้ว
}
toast-deleted-forever = { $kind ->
    [conversation] ลบการสนทนา { $count } รายการอย่างถาวรแล้ว
   *[message] ลบข้อความ { $count } รายการอย่างถาวรแล้ว
}
toast-marked-read = { $kind ->
    [conversation] ทำเครื่องหมายการสนทนา { $count } รายการว่าอ่านแล้ว
   *[message] ทำเครื่องหมายข้อความ { $count } รายการว่าอ่านแล้ว
}
toast-marked-unread = { $kind ->
    [conversation] ทำเครื่องหมายการสนทนา { $count } รายการว่ายังไม่อ่านแล้ว
   *[message] ทำเครื่องหมายข้อความ { $count } รายการว่ายังไม่อ่านแล้ว
}
toast-undone = เลิกทำการดำเนินการแล้ว
toast-nothing-to-undo = ไม่มีอะไรให้เลิกทำ
toast-cannot-undo-delete-forever = อีเมลที่ลบอย่างถาวรแล้วจะนำกลับมาไม่ได้
toast-send-undone = เลิกทำการส่งแล้ว
toast-too-late-to-undo-send = สายเกินไปที่จะเลิกทำ: ข้อความถูกส่งไปแล้ว
toast-undo = เลิกทำ
toast-close = ปิด
toast-no-spam-folder = บัญชีนี้ไม่มีโฟลเดอร์สแปม
