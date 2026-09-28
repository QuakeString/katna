# Katna Mail, Persian (فارسی).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = بستن
reader-back = برگشت
reader-mark-unread = علامت‌گذاری به‌عنوان خوانده‌نشده
reader-move-to = انتقال به
reader-more = بیشتر
reader-original-colors = نمایش رنگ‌های اصلی
reader-dark-colors = نمایش با رنگ‌های تیره
reader-print-all = چاپ همه
reader-new-window = در پنجرهٔ جدید
reader-position = { $position } از { $total }
reader-newer = جدیدتر
reader-older = قدیمی‌تر

## Reading pane: the conversation

reader-removed = این مکالمه حذف شد.
reader-no-subject = (بدون موضوع)
reader-collapse-all = جمع کردن همه
reader-expand-all = باز کردن همه
reader-unknown-sender = (فرستندهٔ ناشناس)
reader-date-ago = { $date } ({ $ago })
reader-me = من
reader-to = به { $names }
reader-to-label = به
reader-tick-delivered = تحویل شد { $when }
reader-tick-no-bounce = فرستاده شد { $when }؛ هیچ پیام ناکامی برنگشت، پس به احتمال زیاد رسیده است
reader-tick-bounced = تحویل نشد: { $when } برگشت خورد
reader-tick-read = خوانده شد { $when } (رسید خواندن)
reader-tick-opened = باز شد، آخرین بار { $when } (ردیابی باز شدن)
reader-starred = ستاره‌دار
reader-not-starred = بدون ستاره
reader-too-long = این پیام برای نمایش کامل بیش از حد طولانی است.
reader-encrypted-images = تصاویر وب هرگز در ایمیل رمزگذاری‌شده بار نمی‌شوند.
reader-window-failed = باز کردن پنجرهٔ جدید ممکن نشد.

## Reading pane: message details (opened from "to me")

reader-details-from = از:
reader-details-to = به:
reader-details-cc = رونوشت:
reader-details-date = تاریخ:
reader-details-subject = موضوع:

## Reading pane: downloading a message

reader-downloading = در حال بارگیری این پیام از سرور…
reader-download-failed = بارگیری این پیام ممکن نشد.
reader-try-again = امتحان مجدد

## Reply row

reply-reply = پاسخ
reply-reply-all = پاسخ به همه
reply-forward = بازارسال

## Encrypted and signed mail

security-decrypting = در حال رمزگشایی…
security-checking = در حال بررسی امضا…
security-partly-encrypted = فقط بخشی از این پیام رمزگذاری شده است. بقیه بیرون از محافظت اضافه شده و ممکن است از هر کسی باشد.
security-partly-signed = فقط بخشی از این پیام امضا شده است. بقیه بیرون از محافظت اضافه شده و ممکن است از هر کسی باشد.
security-encrypted = پیام رمزگذاری‌شده
security-encrypted-smime = پیام رمزگذاری‌شده (S/MIME)
security-no-key = رمزگشایی این پیام ممکن نیست: برای کلیدی رمزگذاری شده که آن را ندارید.
security-cancelled = رمزگشایی لغو شد.
security-damaged = رمزگشایی این پیام ممکن نیست: داده‌های رمزگذاری‌شده آسیب دیده یا تغییر کرده است.
security-decrypt-unavailable = رمزگشایی این پیام ممکن نیست: برای خواندن ایمیل رمزگذاری‌شده { $tool } را نصب کنید.
security-decrypt-failed = رمزگشایی این پیام ممکن نیست: { $reason }
security-unknown-signer = امضاکننده‌ای ناشناس
security-signed-verified = امضاشده توسط { $signer } · تأییدشده
security-signed-not-sender = امضاشده توسط { $signer } که فرستنده نیست
security-signed-untrusted = امضاشده توسط { $signer } با کلیدی که آن را غیرقابل‌اعتماد علامت زده‌اید
security-signed-unverified = امضاشده توسط { $signer } · کلید تأیید نشده است
security-bad-signature = امضای نامعتبر: این پیام پس از امضا تغییر کرده یا امضا جعلی است.
security-signature-expired = امضاشده توسط { $signer } · امضا منقضی شده است
security-key-expired = امضاشده توسط { $signer } · کلید از آن زمان منقضی شده است
security-key-revoked = امضاشده توسط { $signer } با کلیدی که باطل شده است
security-missing-key = با کلیدی امضا شده که آن را ندارید، بنابراین بررسی آن ممکن نیست
security-missing-key-id = با کلیدی امضا شده که آن را ندارید ({ $key })، بنابراین بررسی آن ممکن نیست
security-signature-unavailable = امضاشده؛ برای بررسی امضا { $tool } را نصب کنید
security-signature-error = بررسی امضا ممکن نشد.
tracking-opened = { $who } آن را { $count ->
    [one] یک بار
   *[other] { $count } بار
} باز کرد، آخرین بار { $when }
tracking-opens-clicks = { $who } آن را { $opens ->
    [one] یک بار
   *[other] { $opens } بار
} باز کرد و { $clicks ->
    [one] یک بار
   *[other] { $clicks } بار
} پیوندی را دنبال کرد، آخرین بار { $when }
tracking-clicked = { $who } { $clicks ->
    [one] یک بار
   *[other] { $clicks } بار
} پیوندی را دنبال کرد، آخرین بار { $when }
tracking-maybe-opened = شاید { $who } آن را باز کرده باشد (Apple Mail برای حفظ حریم خصوصی تصاویر را بارگیری می‌کند)
tracking-seen-none = هنوز کسی آن را باز نکرده یا پیوندی را دنبال نکرده است
tracking-receipt = { $who } رسید خواندن فرستاد
tracking-receipt-displayed = رسید خواندن: { $who } پیام شما را باز کرد
tracking-receipt-other = رسید خواندن: { $who } پیام شما را بدون باز کردن حذف کرد یا به آن رسیدگی کرد

## Remote images and pictures

remote-hidden = تصاویر این پیام پنهان شده‌اند.
remote-show = نمایش تصاویر
remote-always-show = همیشه از این فرستنده نمایش داده شود
remote-picture-use = استفاده
remote-picture-too-big = تصویری با حجم 8 مگابایت یا کمتر انتخاب کنید.
remote-picture-type = تصویری با قالب PNG، JPEG، GIF، WebP یا SVG انتخاب کنید.
remote-picture-read-failed = خواندن تصویر ممکن نیست: { $error }
remote-picture-keep-failed = نگه‌داشتن تصویر ممکن نیست: { $error }
remote-picture-remove-failed = حذف تصویر ممکن نیست: { $error }

## Attachments

attachment-count = { $count ->
    [one] { $count } پیوست
   *[other] { $count } پیوست
}
attachment-save = ذخیره
attachment-save-all = ذخیرهٔ همه
attachment-save-all-tooltip = ذخیرهٔ همهٔ پیوست‌ها در یک پوشه
attachment-save-here = ذخیره در اینجا
attachment-not-downloaded = این پیام بارگیری نشده است.
attachment-not-found = این پیوست در پیام پیدا نشد.
attachment-read-failed = خواندن { $name } ممکن نشد
attachment-numbered = پیوست { $number }
attachment-saved-all = { $count ->
    [one] { $count } فایل در { $place } ذخیره شد
   *[other] { $count } فایل در { $place } ذخیره شد
}
attachment-saved-some = { $total ->
    [one] { $saved } از { $total } فایل در { $place } ذخیره شد. ذخیرهٔ { $failed } ممکن نشد
   *[other] { $saved } از { $total } فایل در { $place } ذخیره شد. ذخیرهٔ { $failed } ممکن نشد
}
attachment-saved-to = در { $path } ذخیره شد
attachment-save-failed = ذخیرهٔ { $name } ممکن نشد: { $error }
attachment-open-failed = باز کردن { $name } ممکن نشد: { $error }
attachment-risky = این فایل ممکن است برنامه‌ای را اجرا کند، بنابراین Katna آن را باز نمی‌کند. به‌جای آن ذخیره‌اش کنید.
attachment-encrypted-open = این فایل رمزگذاری‌شده رسیده است. برای باز کردن آن در جای دیگر، ذخیره‌اش کنید.

## Printing

print-failed = چاپ ممکن نشد: { $error }
print-no-font = هیچ قلمی پیدا نشد
print-opened-as-pdf = به‌صورت PDF باز شد تا از آنجا چاپ شود.
print-preview-title = پیش‌نمایش چاپ
print-preview-laying-out = در حال صفحه‌آرایی…
print-preview-pages = { $count ->
    [one] { $count } صفحه
   *[other] { $count } صفحه
}
print-preview-more = { $count ->
    [one] و { $count } صفحهٔ دیگر
   *[other] و { $count } صفحهٔ دیگر
}
print-preview-failed = صفحه‌ها نمایش داده نشدند
print-preview-paper = کاغذ
print-preview-a4 = A4
print-preview-letter = Letter
print-preview-layout = چیدمان
print-preview-as-shown = همان‌گونه که نمایش داده شده
print-preview-simple = متن ساده
print-preview-backgrounds = پس‌زمینه‌ها
print-preview-cancel = لغو
print-preview-print = چاپ
print-not-downloaded = (هنوز بارگیری نشده است.)
print-encrypted = (رمزگذاری‌شده. برای چاپ متن آن، آن را در Katna Mail باز کنید.)
print-to = به: { $addresses }
print-cc = رونوشت: { $addresses }

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = برای خواندن پیوست‌های این پیام، آن را باز کنید.
text-copy = کپی
text-select-all = انتخاب همه
