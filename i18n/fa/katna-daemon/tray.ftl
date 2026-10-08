# Katna Mail, Persian (فارسی).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The tray icon's menu

tray-open-inbox = باز کردن _صندوق ورودی
tray-new-message = _پیام جدید
tray-new-task = _کار جدید
tray-new-note = _یادداشت جدید
tray-preferences = _تنظیمات
tray-quit = _خروج

## The tray icon's tooltip

tray-unread = { $count ->
    [0] پیام خوانده‌نشده‌ای نیست
   *[other] { $count } پیام خوانده‌نشده
}
tray-password-refused = برای { $address } گذرواژهٔ جدید لازم است
tray-signed-out = دوباره به { $address } وارد شوید
tray-accounts-need-you = { $count } حساب به شما نیاز دارند
tray-not-sent = { $count ->
    [one] { $count } پیام ارسال نشد
   *[other] { $count } پیام ارسال نشد
}
