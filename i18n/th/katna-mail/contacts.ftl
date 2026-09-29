# Katna Mail, Thai (ไทย): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = รายชื่อติดต่อ
contacts-frequent = ที่ติดต่อบ่อย
contacts-other = รายชื่อติดต่ออื่นๆ
contacts-other-about = ผู้ที่คุณเคยส่งอีเมลหาจาก Gmail แต่ไม่ได้บันทึกไว้
contacts-other-email = ส่งอีเมล
contacts-other-empty = ไม่มีรายชื่อติดต่ออื่นๆ ผู้ที่คุณส่งอีเมลหาจาก Gmail แต่ไม่ได้บันทึกไว้จะแสดงที่นี่
contacts-other-allow = หากต้องการดูรายชื่อติดต่ออื่นๆ ให้ลงชื่อเข้าใช้บัญชี Gmail อีกครั้งและอนุญาตให้ Katna ดูรายชื่อเหล่านั้น
contacts-labels = ป้ายกำกับ
contacts-label-options = ตัวเลือกป้ายกำกับ
contacts-label-rename = เปลี่ยนชื่อป้ายกำกับ
contacts-label-email = ส่งอีเมลถึงทุกคน
contacts-label-delete = ลบป้ายกำกับ
contacts-label-new = ป้ายกำกับใหม่
contacts-label-name = ชื่อป้ายกำกับ
contacts-label-button = ป้ายกำกับ
contacts-label-menu = ติดป้ายกำกับเป็น:
contacts-label-added = เพิ่มใน { $name } แล้ว
contacts-label-removed = นำออกจาก { $name } แล้ว
contacts-label-renamed = เปลี่ยนชื่อป้ายกำกับเป็น { $name } แล้ว
contacts-label-deleted = ลบป้ายกำกับ { $name } แล้ว
contacts-label-no-email = ไม่มีใครในป้ายกำกับนี้ที่มีที่อยู่อีเมล
contacts-manage = แก้ไขและจัดการ
contacts-merge = รวมและแก้ไข
contacts-merge-about = { $count ->
   *[other] คำแนะนำ { $count } รายการ: รายชื่อติดต่อที่ดูเหมือนเป็นบุคคลเดียวกัน
}
contacts-merge-none = ไม่มีรายการซ้ำ รายชื่อติดต่อที่มีชื่อหรือหมายเลขโทรศัพท์เดียวกันจะแสดงที่นี่
contacts-merge-count = { $count ->
   *[other] รายชื่อติดต่อ { $count } รายการ
}
contacts-merge-all = รวมทั้งหมด
contacts-merge-button = รวม
contacts-merge-dismiss = ปิด
contacts-merged = { $count ->
    [1] รวมรายชื่อติดต่อแล้ว
   *[other] รวมแล้ว { $count } รายการ
}
contacts-import = นำเข้า
contacts-export = ส่งออก
contacts-import-title = นำเข้ารายชื่อติดต่อจากไฟล์ vCard
contacts-imported = { $count ->
   *[other] นำเข้ารายชื่อติดต่อ { $count } รายการไปยัง { $place } แล้ว
}
contacts-imported-some = { $count ->
   *[other] นำเข้ารายชื่อติดต่อ { $count } รายการไปยัง { $place } แล้ว ข้ามรายชื่อที่บันทึกไว้แล้ว { $skipped } รายการ
}
contacts-import-none = ไม่พบรายชื่อติดต่อใน { $name }
contacts-import-all-saved = ทุกคนใน { $name } ถูกบันทึกไว้แล้ว
contacts-import-failed = อ่าน { $name } ไม่ได้: { $error }
contacts-exported = { $count ->
   *[other] ส่งออกรายชื่อติดต่อ { $count } รายการไปยัง { $path } แล้ว
}
contacts-export-none = ไม่มีรายชื่อติดต่อให้ส่งออก
contacts-export-failed = ส่งออกรายชื่อติดต่อไม่ได้: { $error }
contacts-create = สร้างรายชื่อติดต่อ

## Search and the list

contacts-search = ค้นหารายชื่อติดต่อ
contacts-loading = กำลังโหลดรายชื่อติดต่อ…
contacts-empty = ยังไม่มีรายชื่อติดต่อที่บันทึกไว้ รายชื่อติดต่อที่คุณบันทึกใน Gmail, Outlook หรือบริการอีเมลของคุณจะแสดงที่นี่
contacts-empty-no-books = รายชื่อติดต่อจากบัญชีของคุณจะแสดงที่นี่เมื่อซิงก์แล้ว
contacts-none-found = ไม่มีรายชื่อติดต่อที่ตรงกับการค้นหา
contacts-starred = { $count ->
   *[other] รายชื่อติดต่อที่ติดดาว ({ $count })
}
contacts-count = รายชื่อติดต่อ ({ $count })
contacts-col-name = ชื่อ
contacts-col-email = อีเมล
contacts-col-phone = หมายเลขโทรศัพท์
contacts-col-job = ตำแหน่งงานและบริษัท
contacts-col-labels = ป้ายกำกับ

## Asking to allow contacts, for accounts signed in before Katna read them

contacts-allow = อนุญาตให้ Katna อ่านรายชื่อติดต่อของ { $address }
contacts-allow-many = { $more ->
   *[other] อนุญาตให้ Katna อ่านรายชื่อติดต่อของ { $address } และอีก { $more } บัญชี
}
contacts-allow-button = อนุญาต

## A contact's page

contacts-back = กลับไปที่รายชื่อติดต่อ
contacts-edit = แก้ไข
contacts-delete = ลบ
contacts-deleted = ลบ { $name } แล้ว
contacts-added = เพิ่ม { $name } ในรายชื่อติดต่อแล้ว
contacts-find-mail = อีเมล
contacts-details = รายละเอียดผู้ติดต่อ
contacts-saved-in = บันทึกไว้ใน
contacts-notes = โน้ต
contacts-birthday = วันเกิด
contacts-nickname = ชื่อเล่น
contacts-this-computer = คอมพิวเตอร์เครื่องนี้
contacts-kind-home = บ้าน
contacts-kind-work = ที่ทำงาน
contacts-kind-mobile = มือถือ
contacts-kind-other = อื่นๆ
contacts-source-google = Google รายชื่อติดต่อ
contacts-source-microsoft = รายชื่อติดต่อ Outlook
contacts-source-carddav = CardDAV

## Creating and changing a contact

contacts-edit-new-title = สร้างรายชื่อติดต่อ
contacts-edit-title = แก้ไขรายชื่อติดต่อ
contacts-edit-save = บันทึก
contacts-edit-saving = กำลังบันทึก…
contacts-edit-cancel = ยกเลิก
contacts-saved = บันทึกรายชื่อติดต่อแล้ว
contacts-edit-save-to = บันทึกไปยัง
contacts-edit-changes-go-to = ระบบจะบันทึกการเปลี่ยนแปลงไปยัง { $place }
contacts-edit-given = ชื่อ
contacts-edit-family = นามสกุล
contacts-edit-company = บริษัท
contacts-edit-job = ตำแหน่งงาน
contacts-edit-email = อีเมล
contacts-edit-phone = โทรศัพท์
contacts-edit-with-kind = { $field } ({ $kind })
contacts-edit-add-email = เพิ่มอีเมล
contacts-edit-add-phone = เพิ่มหมายเลขโทรศัพท์
contacts-edit-street = ที่อยู่ถนน
contacts-edit-city = เมือง
contacts-edit-postcode = รหัสไปรษณีย์
contacts-edit-country = ประเทศ
contacts-edit-birthday = วันเกิด (YYYY-MM-DD)
contacts-edit-empty = เพิ่มชื่อ อีเมล หรือหมายเลขโทรศัพท์ก่อน
