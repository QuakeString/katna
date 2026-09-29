# Katna Mail, Thai (ไทย): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = วันนี้
calendar-today-tip = ไปที่วันนี้
calendar-view-day = วัน
calendar-view-week = สัปดาห์
calendar-view-month = เดือน
calendar-view-year = ปี
calendar-view-schedule = กำหนดการ
calendar-view-days =
    { $count ->
       *[other] { $count } วัน
    }
calendar-options = ตัวเลือก
calendar-density = ความหนาแน่น
calendar-density-responsive = ปรับตามหน้าจอของคุณ
calendar-density-comfortable = สบาย
calendar-density-compact = กะทัดรัด
calendar-custom-days = มุมมองกำหนดเอง
calendar-second-zone = เขตเวลาที่สอง
calendar-zone-none = ไม่มี
calendar-zone = { $zone } ({ $offset })
calendar-share-free = แชร์เวลาว่าง
calendar-free-subject = เวลาที่ฉันว่าง
calendar-free-intro = นี่คือเวลาที่ฉันว่าง ({ $zone }):
calendar-free-day = { $weekday } { $date }: { $times }
calendar-free-range = { $start } – { $end }
calendar-free-none = ฉันไม่มีเวลาว่างในอีกสองสามวันทำการข้างหน้า
calendar-previous-day = วันก่อนหน้า
calendar-next-day = วันถัดไป
calendar-previous-week = สัปดาห์ก่อนหน้า
calendar-next-week = สัปดาห์ถัดไป
calendar-previous-month = เดือนก่อนหน้า
calendar-next-month = เดือนถัดไป
calendar-previous-year = ปีก่อนหน้า
calendar-next-year = ปีถัดไป
calendar-previous-period = ก่อนหน้า
calendar-next-period = ถัดไป
calendar-title-months = { $first } – { $last }
calendar-loading = กำลังโหลด…
calendar-read-failed = อ่านปฏิทินไม่ได้: { $error }
calendar-sets = ชุดปฏิทิน
calendar-set-add = บันทึกปฏิทินที่แสดงอยู่เป็นชุด
calendar-set-name = ชื่อชุด
calendar-set-remove = นำชุดออก
calendar-local = คอมพิวเตอร์เครื่องนี้
calendar-account-gone = บัญชีที่ถูกนำออกแล้ว
calendar-account-sign-in = ลงชื่อเข้าใช้อีกครั้งเพื่อแสดงปฏิทิน
calendar-account-signed-in = ลงชื่อเข้าใช้ { $address } อีกครั้งแล้ว กำลังดึงปฏิทินของคุณ…
calendar-account-sign-in-refused = { $provider } ไม่อนุญาตให้ Katna เข้าใช้ ลองอีกครั้ง และอนุญาตให้เข้าถึงปฏิทินของคุณ
calendar-account-refused = เซิร์ฟเวอร์ไม่ยอมรับรหัสผ่าน Yahoo, iCloud, Zoho และอื่นๆ ต้องใช้รหัสผ่านสำหรับแอป
calendar-account-change-password = เปลี่ยนรหัสผ่าน
calendar-account-change-password-tooltip = เปิด การตั้งค่า > บัญชี
calendar-account-not-enabled = ยังไม่ได้เปิดการเข้าถึงปฏิทินสำหรับ Katna
calendar-account-failed = อ่านปฏิทินไม่ได้
calendar-account-error = อ่านปฏิทินไม่ได้: { $reason }
calendar-account-none = ไม่พบปฏิทิน
calendar-account-looking = กำลังค้นหาปฏิทิน…
calendar-account-try-again = ลองอีกครั้ง
calendar-account-try-again-tooltip = ตรวจสอบปฏิทินของบัญชีนี้อีกครั้งตอนนี้
calendar-account-fixing = กำลังดำเนินการ…
calendar-birthdays = วันเกิด
calendar-birthday-of = วันเกิดของ { $name }
calendar-empty-title = ยังไม่มีปฏิทิน
calendar-empty-text = ปฏิทินของบัญชี Google และ Microsoft ของคุณจะแสดงที่นี่เมื่อซิงค์แล้ว รวมถึงปฏิทินจากเซิร์ฟเวอร์อื่นที่รองรับ CalDAV
calendar-schedule-empty = ไม่มีกิจกรรมที่วางแผนไว้ใน 2 เดือนข้างหน้า
calendar-search = ค้นหากิจกรรม
calendar-search-past = กิจกรรมที่ผ่านมาแล้ว
calendar-search-none = ไม่มีกิจกรรมที่ตรงกับการค้นหา
calendar-no-title = (ไม่มีชื่อ)
calendar-all-day = ทั้งวัน
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }, { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = อีก { $count } รายการ
calendar-repeats = เกิดซ้ำ
calendar-join = เข้าร่วม
calendar-email-guests = ส่งอีเมลถึงแขก
calendar-running-late = จะไปสาย
calendar-late-subject = จะไปสาย: { $title }
calendar-late-body = ขออภัยที่ต้องไปถึง { $title } ช้าไปสองสามนาที เดี๋ยวจะไปถึงเร็ว ๆ นี้
calendar-guests =
    { $count ->
       *[other] แขก { $count } คน
    }
calendar-guest-answers = ตอบรับ { $yes }, อาจจะ { $maybe }, ปฏิเสธ { $no }, รอการตอบกลับ { $waiting }
calendar-organizer = ผู้จัด
calendar-optional = ไม่บังคับ
calendar-open-web = เปิดในเบราว์เซอร์
calendar-open-contact = เปิดรายชื่อติดต่อ
calendar-close = ปิด

## Adding, changing and deleting events.

calendar-add-title = เพิ่มชื่อ
calendar-add-location = เพิ่มสถานที่
calendar-add-notes = เพิ่มคำอธิบาย
calendar-add-guests = เพิ่มแขก
calendar-remove-guest = นำออก
calendar-add-meet = เพิ่มการประชุมทางวิดีโอของ Google Meet
calendar-add-teams = เพิ่มการประชุม Teams
calendar-has-call = เพิ่มการโทรวิดีโอแล้ว
calendar-weekday-day = { $weekday } { $day }
calendar-all-day-box = ตลอดวัน
calendar-more-options = ตัวเลือกเพิ่มเติม
calendar-save = บันทึก
calendar-saved = บันทึกกิจกรรมแล้ว
calendar-deleted = ลบกิจกรรมแล้ว
calendar-discard = ทิ้งการเปลี่ยนแปลง
calendar-edit = แก้ไขกิจกรรม
calendar-delete = ลบกิจกรรม
calendar-event-details = รายละเอียดกิจกรรม
calendar-kind-event = กิจกรรม
calendar-kind-focus = เวลาโฟกัส
calendar-kind-out-of-office = ไม่อยู่ที่ทำงาน
calendar-kind-working-location = สถานที่ทำงาน
calendar-working-home = บ้าน
calendar-busy = ไม่ว่าง
calendar-free = ว่าง
calendar-cancel = ยกเลิก
calendar-ok = ตกลง
calendar-read-only = คุณเปลี่ยนแปลงกิจกรรมในปฏิทินนี้ไม่ได้
calendar-none-editable = ยังไม่มีปฏิทินที่คุณเพิ่มกิจกรรมได้
calendar-no-such-time = ไม่มีเวลานั้นในเขตเวลาของคุณ
calendar-end-before-start = กิจกรรมสิ้นสุดก่อนเริ่ม
calendar-repeat-never = ไม่เกิดซ้ำ
calendar-repeat-daily = ทุกวัน
calendar-repeat-weekly = ทุกสัปดาห์ใน { $weekday }
calendar-repeat-monthly =
    { $nth ->
        [1] ทุกเดือนใน { $weekday } แรก
        [2] ทุกเดือนใน { $weekday } ที่สอง
        [3] ทุกเดือนใน { $weekday } ที่สาม
        [4] ทุกเดือนใน { $weekday } ที่สี่
       *[other] ทุกเดือนใน { $weekday } สุดท้าย
    }
calendar-repeat-yearly = ทุกปีใน { $day }
calendar-repeat-weekdays = ทุกวันทำการ (วันจันทร์ถึงวันศุกร์)
calendar-repeat-custom = กำหนดเอง
calendar-reminder-none = ไม่มีการแจ้งเตือน
calendar-reminder-at-start = ตอนเริ่ม
calendar-reminder-minutes =
    { $count ->
       *[other] { $count } นาทีก่อน
    }
calendar-reminder-hours =
    { $count ->
       *[other] { $count } ชั่วโมงก่อน
    }
calendar-reminder-days =
    { $count ->
       *[other] { $count } วันก่อน
    }
calendar-scope-edit-title = แก้ไขกิจกรรมที่เกิดซ้ำ
calendar-scope-delete-title = ลบกิจกรรมที่เกิดซ้ำ
calendar-scope-this = กิจกรรมนี้
calendar-scope-following = กิจกรรมนี้และกิจกรรมต่อๆ ไป
calendar-scope-all = กิจกรรมทั้งหมด
calendar-scope-respond-title = ตอบกิจกรรมที่เกิดซ้ำ
calendar-going = จะไปไหม
calendar-answer-yes = ใช่
calendar-answer-no = ไม่ใช่
calendar-answer-maybe = อาจจะ
calendar-answered-yes = คุณจะไป
calendar-answered-no = คุณจะไม่ไป
calendar-answered-maybe = คุณอาจจะไป

## The card at the top of a mail with an invitation.

calendar-invite = คำเชิญ
calendar-invite-cancelled = ยกเลิกกิจกรรมแล้ว
calendar-invite-reply = { $name } ตอบกลับแล้ว
calendar-invite-reply-yes = { $name } ตอบรับแล้ว
calendar-invite-reply-no = { $name } ปฏิเสธแล้ว
calendar-invite-reply-maybe = { $name } อาจจะไป
calendar-invite-organizer = จัดโดย { $name }
calendar-invite-open = เปิดในปฏิทิน
calendar-invite-not-yet = ยังไม่อยู่ในปฏิทินของคุณ ตอบได้เมื่อซิงค์แล้ว
calendar-invite-by-mail = ไม่อยู่ในปฏิทินของคุณ: คำตอบของคุณจะส่งถึงผู้จัดทางอีเมล
calendar-mail-yes = ตอบรับแล้ว: { $title }
calendar-mail-yes-body = { $name } ตอบรับคำเชิญนี้แล้ว
calendar-mail-no = ปฏิเสธแล้ว: { $title }
calendar-mail-no-body = { $name } ปฏิเสธคำเชิญนี้แล้ว
calendar-mail-maybe = อาจจะไป: { $title }
calendar-mail-maybe-body = { $name } ตอบรับคำเชิญนี้แบบยังไม่แน่ใจ
calendar-invite-your-day = วันของคุณ
calendar-invite-clashes =
    { $count ->
       *[other] ทับซ้อนกับ { $count } กิจกรรม
    }

## The day's agenda beside the mail.

agenda-show = แสดงกำหนดการของวัน
agenda-hide = ซ่อนกำหนดการ
agenda-today = วันนี้, { $date }
agenda-day = { $weekday }, { $date }
agenda-empty = ไม่มีกิจกรรมที่วางแผนไว้ในวันนี้
