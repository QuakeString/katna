# Katna Mail, Thai (ไทย).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## About Katna (the dialog from the "i" button on the top bar)

about-tooltip = เกี่ยวกับ Katna
about-tagline = อีเมลและปฏิทินสำหรับเดสก์ท็อป Linux
about-whats-new = มีอะไรใหม่

## Updates, in a box under the version in About (only in packages that
## update themselves). $version is a version such as 0.0.0.r236.g1a2b3c4.

about-update-not-checked = ยังไม่ได้ตรวจสอบการอัปเดต
about-update-checking = กำลังตรวจสอบการอัปเดต…
about-update-up-to-date = Katna Mail เป็นเวอร์ชันล่าสุดแล้ว
about-update-check-failed = ตรวจสอบการอัปเดตไม่ได้
about-update-available = มีเวอร์ชัน { $version } ให้อัปเดต
about-update-downloading = กำลังดาวน์โหลดเวอร์ชัน { $version }… { $percent }%
about-update-download-failed = การดาวน์โหลดเวอร์ชัน { $version } ไม่สำเร็จ
about-update-ready = เวอร์ชัน { $version } พร้อมติดตั้งแล้ว
about-update-ready-detail = Katna Mail จะรีสตาร์ทเพื่อติดตั้งการอัปเดตให้เสร็จสมบูรณ์
about-update-confirm = ติดตั้งเวอร์ชัน { $version } หรือไม่?
about-update-confirm-detail = Katna Mail จะปิด ติดตั้งการอัปเดต แล้วเปิดขึ้นใหม่ตรงจุดที่คุณค้างไว้ คอมพิวเตอร์ของคุณจะขอรหัสผ่าน
about-update-installing = กำลังติดตั้งเวอร์ชัน { $version }…
about-update-installing-detail = ป้อนรหัสผ่านของคุณในหน้าต่างที่เปิดขึ้น
about-update-cancelled = ไม่ได้ติดตั้งการอัปเดต เพราะไม่ได้ป้อนรหัสผ่าน
about-update-failed = ติดตั้งการอัปเดตไม่ได้: { $error }
about-update-unsupported = Katna Mail ชุดนี้อัปเดตโดยตัวจัดการแพ็กเกจของคุณ
about-update-restart-failed = ติดตั้งการอัปเดตแล้ว แต่ Katna Mail เปิดขึ้นใหม่ไม่ได้ ({ $error }) โปรดเปิดเองด้วยตนเอง
about-update-check = ตรวจสอบการอัปเดต
about-update-download = ดาวน์โหลด
about-update-retry = ลองอีกครั้ง
about-update-button = อัปเดต
about-update-restart = อัปเดตและรีสตาร์ท
about-update-cancel = ไว้ทีหลัง
about-changelog = บันทึกการเปลี่ยนแปลง
about-source = ซอร์สโค้ด
about-coffee = เลี้ยงกาแฟฉันสักแก้ว
about-coming-soon = เร็วๆ นี้
about-follow-me = ติดตามฉันบน
about-love-title = สร้างด้วยความรักต่อ Rust, KDE และ Linux
about-love-text = Rust ทำให้การเขียนแอปอีเมลที่เร็วและปลอดภัยเป็นเรื่องสนุก: Katna ไม่มีโค้ด unsafe เลย เดสก์ท็อป Plasma ของ KDE และชุด PIM ของ KDE เป็นแรงบันดาลใจให้ Katna ส่วน Linux และชุมชนซอฟต์แวร์เสรีสร้างรากฐานที่ Katna ยืนอยู่ ขอบคุณ และขอบคุณไลบรารีด้านล่างนี้ด้วย
about-kde-text = KDE สร้างเดสก์ท็อปที่ Katna รู้สึกเหมือนอยู่บ้านที่สุด ซึ่งสร้างโดยอาสาสมัครและได้รับทุนจากผู้คนอย่างคุณ หากคุณชอบ Plasma หรือแอปของ KDE โปรดพิจารณาบริจาคให้ KDE
about-donate-kde = บริจาคให้ KDE
about-gpui-title = สร้างบน GPUI จากโปรเจกต์ Zed
about-gpui-text = อินเทอร์เฟซทั้งหมดของ Katna Mail สร้างบน GPUI เฟรมเวิร์ก UI ที่รวดเร็วและเร่งความเร็วด้วย GPU ซึ่ง Zed Industries สร้างขึ้นสำหรับโปรแกรมแก้ไขโค้ด Zed ทุกพิกเซล แอนิเมชัน และหน้าต่างที่คุณเห็นถูกวาดด้วย GPUI ขอบคุณทีม Zed ที่สร้างมันอย่างเปิดเผย Apache-2.0
about-gpui-github = GPUI บน GitHub
about-personal-title = โปรเจกต์ส่วนตัว
about-personal-text = Katna Mail ไม่ได้พยายามเป็นสิ่งใหม่หรือปฏิวัติวงการ มันคือแอปอีเมลที่ผู้พัฒนาอยากได้ ฟีเจอร์และหน้าตายืมมาจาก Gmail, Mailspring และ Thunderbird และเป็นไปได้ก็เพราะ LLM ก้าวหน้ามาไกลมาก
about-built-on = สร้างบนซอฟต์แวร์เสรี
about-credit-pimalaya = IMAP, SMTP และการลงชื่อเข้าใช้ (io-imap, io-smtp, io-sasl)
about-credit-imap-codec = การอ่านและเขียน IMAP
about-credit-tantivy = การค้นหา
about-credit-sqlite = คลังอีเมล
about-credit-rustls = การเชื่อมต่อที่ปลอดภัย
about-credit-mail-parser = การอ่านอีเมล จาก Stalwart Labs
about-credit-html5ever = อีเมล HTML จากโปรเจกต์ Servo
about-credit-zbus = การสื่อสารกับเดสก์ท็อปผ่าน D-Bus และ portals
about-credit-oo7 = รหัสผ่านในพวงกุญแจของเดสก์ท็อป
about-credit-hayro = การดูและพิมพ์ PDF
about-credit-calamine = ตัวอย่างสเปรดชีต
about-credit-resvg = รูปภาพ SVG
about-credit-jiff = วันที่และเขตเวลา
about-credit-spellbook = การตรวจตัวสะกด จากโปรแกรมแก้ไขโค้ด Helix
about-credit-smol = การทำหลายอย่างพร้อมกัน
about-all-libraries = ไลบรารีทั้งหมดที่ Katna ใช้ ({ $count })
about-library-authors = โดย { $authors }
about-license = Katna เป็นซอฟต์แวร์เสรีภายใต้ GNU GPL เวอร์ชัน 3 หรือใหม่กว่า
about-close = ปิด

## What’s new (shown after an update)

whats-new-title = มีอะไรใหม่ใน Katna Mail
whats-new-updated = อัปเดตเป็นเวอร์ชัน { $version } แล้ว
whats-new-version = เวอร์ชัน { $version }
whats-new-more = { $count ->
   *[other] และอีก { $count } รายการในบันทึกการเปลี่ยนแปลงฉบับเต็ม
}
whats-new-changelog = บันทึกการเปลี่ยนแปลงฉบับเต็ม
whats-new-got-it = เข้าใจแล้ว

## First run: welcome page

onboarding-welcome-title = ยินดีต้อนรับสู่ Katna Mail
onboarding-welcome-lead = อีเมลของคุณอยู่บนคอมพิวเตอร์ของคุณเอง: ค้นหาได้เร็ว อ่านได้แม้ออฟไลน์ และเป็นส่วนตัว
onboarding-fast-title = เร็ว แม้ออฟไลน์
onboarding-fast-text = Katna เก็บสำเนาอีเมลของคุณไว้ที่นี่ การเปิดและค้นหาจึงทำได้ทันที ไม่ว่าจะมีการเชื่อมต่อหรือไม่
onboarding-providers-title = ใช้ได้กับอีเมลของคุณ
onboarding-providers-text = Gmail, Outlook, Yahoo, iCloud และบัญชี IMAP หรือ POP อื่นๆ
onboarding-private-title = เป็นส่วนตัว
onboarding-private-text = อีเมลของคุณส่งตรงจากผู้ให้บริการมายังคอมพิวเตอร์เครื่องนี้ ไม่มีเซิร์ฟเวอร์ของ Katna เห็นอีเมลเลย
onboarding-get-started = เริ่มต้นใช้งาน

## First run: adding an account

onboarding-service-checking = กำลังตรวจสอบบริการเบื้องหลังของ Katna…
onboarding-service-running = บริการเบื้องหลังของ Katna กำลังทำงาน
onboarding-service-missing = บริการเบื้องหลังของ Katna ไม่ได้ทำงาน
onboarding-service-start = บริการนี้รับและส่งอีเมลของคุณ เริ่มบริการจากเทอร์มินัล แล้วตรวจสอบอีกครั้ง:
onboarding-check-again = ตรวจสอบอีกครั้ง
onboarding-account-title = เพิ่มบัญชีอีเมลของคุณ
onboarding-account-lead = พิมพ์ที่อยู่อีเมลและรหัสผ่าน แล้ว Katna จะค้นหาการตั้งค่าเซิร์ฟเวอร์ให้ Gmail, Yahoo และ iCloud ต้องใช้รหัสผ่านสำหรับแอป ซึ่งสร้างได้ในการตั้งค่าความปลอดภัยของบัญชี
onboarding-add-account = เพิ่มบัญชี
onboarding-back = กลับ

## First run: choosing the look

onboarding-look-title = ปรับให้เป็นแบบของคุณ
onboarding-look-lead = เลือกวิธีเปิดอีเมลและหน้าตาของ Katna คุณเปลี่ยนได้ทุกเมื่อในการตั้งค่าด่วน
onboarding-reading-pane = บานหน้าต่างการอ่าน
onboarding-pane-right = ด้านขวาของรายการ
onboarding-pane-none = ไม่แยก
onboarding-theme = ธีม
onboarding-theme-system = ระบบ
onboarding-theme-light = สว่าง
onboarding-theme-dark = มืด
onboarding-density = ความหนาแน่น
onboarding-density-default = ค่าเริ่มต้น
onboarding-density-compact = กะทัดรัด
onboarding-continue = ดำเนินการต่อ

## First run: done

onboarding-ready-title = พร้อมแล้ว
onboarding-ready-lead = Katna กำลังรับอีเมลของคุณ อีเมลจะแสดงเมื่อมาถึง และอีเมลใหม่จะปรากฏขึ้นเอง
onboarding-ready-lead-address = Katna กำลังรับอีเมลของ { $address } อีเมลจะแสดงเมื่อมาถึง และอีเมลใหม่จะปรากฏขึ้นเอง
onboarding-ready-tour = ชมแนะนำการใช้งานหนึ่งนาทีเพื่อดูว่าทุกอย่างอยู่ที่ไหนไหม
onboarding-skip = ข้ามไปก่อน
onboarding-take-tour = ชมแนะนำการใช้งาน

## Asking to send crash reports (on its own and on the first-run pages)

share-title = ช่วยปรับปรุง Katna
share-lead = เมื่อ Katna ขัดข้อง จะบันทึกรายงานไว้ในคอมพิวเตอร์เครื่องนี้ การส่งรายงานเหล่านี้ช่วยแก้ไขสิ่งที่ผิดพลาด คุณเปลี่ยนได้ทุกเมื่อใน การตั้งค่า > ความคิดเห็นของผู้ใช้
share-sent = สิ่งที่ถูกส่ง
share-sent-detail = รายงานข้อขัดข้องตามที่คุณดูได้ในการตั้งค่า: อะไรขัดข้องและที่ส่วนไหนของ Katna เวอร์ชัน ระบบ Linux และเดสก์ท็อปของคุณ และบรรทัดบันทึกล่าสุดของ Katna ซึ่งอาจมีชื่อโฟลเดอร์อีเมล
share-never-sent = สิ่งที่ไม่ถูกส่งเลย
share-never-sent-detail = ข้อความ รายชื่อติดต่อ รหัสผ่าน ที่อยู่ IP ชื่อผู้ใช้ หรือชื่อคอมพิวเตอร์ของคุณ ที่อยู่อีเมลจะถูกลบออกจากรายงาน
share-where = ส่งไปที่ไหน
share-where-detail = ระบบติดตามข้อขัดข้องของ Katna ที่ Sentry จัดเก็บในสหภาพยุโรป ไม่มี ID ใดผูกรายงานกับตัวคุณ
share-dont-send = ไม่ส่ง
share-send = ส่งรายงานข้อขัดข้อง
share-sending = จะส่งรายงานข้อขัดข้อง ขอบคุณ
share-local = รายงานข้อขัดข้องจะอยู่ในคอมพิวเตอร์เครื่องนี้

## The tour (cards pointing at each part of the window)

tour-welcome-title = ยินดีต้อนรับสู่ Katna Mail
tour-welcome-text = แนะนำการใช้งานหนึ่งนาทีจะแสดงว่าทุกอย่างอยู่ที่ไหน
tour-not-now = ไม่ใช่ตอนนี้
tour-start = ชมแนะนำการใช้งาน
tour-close = ปิด
tour-skip = ข้ามการแนะนำ
tour-back = กลับ
tour-done = เสร็จสิ้น
tour-next = ถัดไป
tour-step = { $step } จาก { $total }
tour-compose-title = เขียนข้อความ
tour-compose-text = ปุ่มเขียนจะเปิดข้อความใหม่ที่มุมขวาล่าง คุณจึงอ่านต่อได้ขณะเขียน
tour-search-title = ค้นหาอีเมลทั้งหมดของคุณ
tour-search-text = การค้นหาใช้ได้แม้ออฟไลน์ ปุ่มที่ปลายด้านขวาใช้เพิ่มตัวกรอง: ผู้ส่ง ผู้รับ หัวเรื่อง วันที่ และไฟล์แนบ
tour-menu-title = แสดงหรือซ่อนโฟลเดอร์
tour-menu-text = ปุ่มนี้พับรายการโฟลเดอร์เก็บไว้ ขณะซ่อนอยู่ ให้วางตัวชี้บน อีเมล ที่ด้านซ้ายเพื่อดูโฟลเดอร์
tour-apps-title = แอปของคุณ
tour-apps-text = ตอนนี้อีเมลอยู่ที่นี่ ปฏิทิน รายชื่อติดต่อ งาน โน้ต และฟีด จะตามมาอยู่ในแถบนี้
tour-tabs-title = แท็บกล่องจดหมาย
tour-tabs-text = อีเมลใหม่จะถูกจัดเป็น หลัก โปรโมชัน โซเชียล อัปเดต และฟอรัม คุณปิดแท็บได้ในการตั้งค่าด่วน
tour-list-title = ข้อความของคุณ
tour-list-text = คลิกข้อความเพื่ออ่าน วางตัวชี้ไว้เพื่อดูการทำงานด่วน คลิกขวาเพื่อดูเพิ่มเติม หรือติ๊กหลายข้อความเพื่อจัดการพร้อมกัน
tour-settings-title = การตั้งค่าด่วน
tour-settings-text = เปลี่ยนบานหน้าต่างการอ่าน ความหนาแน่น และธีมได้ที่นี่ และเริ่มแนะนำการใช้งานอีกครั้งได้จากที่นี่เช่นกัน
tour-account-title = บัญชีของคุณ
tour-account-text = ดูว่าคุณอยู่ในบัญชีใด และเพิ่มบัญชีอื่น

## Crash notice (a bar at the bottom after a crash)

crash-daemon = { $more ->
    [0] บริการเบื้องหลังของ Katna หยุดทำงานโดยไม่คาดคิด
   *[other] บริการเบื้องหลังของ Katna หยุดทำงานโดยไม่คาดคิด มีรายงานข้อขัดข้องอื่นบันทึกไว้อีก { $more } รายการ
}
crash-mail = { $more ->
    [0] Katna Mail ปิดไปโดยไม่คาดคิดเมื่อครั้งที่แล้ว
   *[other] Katna Mail ปิดไปโดยไม่คาดคิดเมื่อครั้งที่แล้ว มีรายงานข้อขัดข้องอื่นบันทึกไว้อีก { $more } รายการ
}
crash-view = ดูรายงาน
crash-view-tooltip = เปิดรายงานที่บันทึกไว้ในคอมพิวเตอร์เครื่องนี้
crash-copy = คัดลอกรายงาน
crash-close = ปิด
sign-in-again-text = { $provider } ขอให้คุณลงชื่อเข้าใช้ { $address } อีกครั้ง
sign-in-again-button = ลงชื่อเข้าใช้
sign-in-again-tooltip = เปิดหน้าลงชื่อเข้าใช้ { $provider } ในเบราว์เซอร์ของคุณ
sign-in-again-waiting = กำลังรอเบราว์เซอร์ของคุณ…
sign-in-again-close = ปิด
sign-in-again-done = ลงชื่อเข้าใช้ { $address } อีกครั้งแล้ว กำลังรับอีเมลของคุณ…
delete-ask-title = { $kind ->
    [conversation] { $count ->
       *[other] ย้ายการสนทนา { $count } รายการไปที่ถังขยะไหม
    }
   *[message] { $count ->
       *[other] ย้ายข้อความ { $count } รายการไปที่ถังขยะไหม
    }
}
delete-ask-body = { $count ->
   *[other] คุณเลิกทำได้ทันทีหลังจากนั้น หรือนำกลับมาจากถังขยะได้ในภายหลัง
}
delete-ask-confirm = ย้ายไปที่ถังขยะ
delete-forever-title = { $kind ->
    [conversation] { $count ->
       *[other] ลบการสนทนา { $count } รายการอย่างถาวรไหม
    }
   *[message] { $count ->
       *[other] ลบข้อความ { $count } รายการอย่างถาวรไหม
    }
}
delete-forever-body = { $count ->
   *[other] จะถูกลบบนเซิร์ฟเวอร์ด้วย และไม่สามารถเลิกทำได้
}
delete-forever-confirm = ลบอย่างถาวร
delete-ask-dont-ask = ไม่ต้องถามอีก
delete-ask-cancel = ยกเลิก
