# Katna Mail, Tamil (தமிழ்).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The snooze menu (the clock on a line, or Snooze in the right-click menu)

snooze-until = இது வரை உறக்கநிலையில் வை…
snooze-later-today = இன்று பின்னர்
snooze-tomorrow = நாளை
snooze-this-weekend = இந்த வார இறுதி
snooze-next-week = அடுத்த வாரம்
snooze-pick = தேதியையும் நேரத்தையும் தேர்ந்தெடு
snooze-back = நேரங்களுக்குத் திரும்பு
snooze-type-placeholder = நேரத்தை உள்ளிடுங்கள்
snooze-type-hint = எ.கா. “tue 3pm”, “tomorrow” அல்லது “in 2 hours”
snooze-type-hint-unclear = Katna-வால் அதை நேரமாகப் படிக்க முடியவில்லை
snooze-type-unclear = “{ $text }” என்பது Katna அறிந்த நேரம் அல்ல

## Snooze and Remind me on mail: one menu, switched at its top (B and H)

snooze-tab = உறக்கநிலை
remind-tab = எனக்கு நினைவூட்டு
snooze-says = அதுவரை மறைத்து வைக்கும்
remind-says = இருக்கும் இடத்திலேயே வைத்து உங்களுக்கு அறிவிக்கும்
remind-before-due = கெடுவுக்கு முன்
remind-note = குறிப்பு (விருப்பத்தேர்வு)
remind-note-placeholder = காலியாக விட்டால், பொருள்
toast-remind-set = { $date } க்கு நினைவூட்டல் அமைக்கப்பட்டது
remind-chat-line = நினைவூட்டல் { $date } · { $title }
remind-done = முடிந்தது
toast-remind-done = நினைவூட்டல் முடிந்தது
snooze-chat-line = { $date } வரை உறக்கநிலையில்
snooze-chat-change = மாற்று

## The date and time picker

snooze-cancel = ரத்துசெய்
snooze-save = சேமி
snooze-in-the-past = இப்போதைக்குப் பிறகான நேரத்தைத் தேர்ந்தெடு.

## beside Send

follow-up-menu = பதில் இல்லையெனில் ஃபாலோ-அப்…
follow-up-title = பதில் இல்லையெனில் ஃபாலோ-அப்
follow-up-off = முடக்கம்
follow-up-days = { $days ->
    [one] { $days } நாள்
   *[other] { $days } நாட்கள்
}
follow-up-weeks = { $weeks ->
    [one] { $weeks } வாரம்
   *[other] { $weeks } வாரங்கள்
}
follow-up-pick = தேர்ந்தெடு…
follow-up-pick-title = இதற்குள் பதில் இல்லையெனில் ஃபாலோ-அப்
follow-up-remind = எனக்கு நினைவூட்டு
follow-up-remind-note = உரையாடல் உங்கள் இன்பாக்ஸின் மேலே திரும்ப வரும்
follow-up-send = எனக்காக ஃபாலோ-அப் அனுப்பு
follow-up-send-note = அதே நபர்களுக்கு, அதே உரையாடலில்
follow-up-send-encrypted = மறையாக்கப்பட்ட அஞ்சலுக்கு அல்ல
follow-up-text-placeholder = என்ன எழுத வேண்டும்
follow-up-text-named = வணக்கம் { $name }, கீழே உள்ள என் மெசேஜைப் பார்த்தீர்களா என்று கேட்கிறேன்.
follow-up-text = வணக்கம், கீழே உள்ள என் மெசேஜைப் பார்த்தீர்களா என்று கேட்கிறேன்.
follow-up-template = டெம்ப்ளேட்டைப் பயன்படுத்து
follow-up-signature = உங்கள் கையொப்பம் சேர்க்கப்படும்
follow-up-again = இன்னும் பதில் இல்லையெனில், இதற்குப் பிறகு மீண்டும் ஃபாலோ-அப்
follow-up-note = உரையாடலில் யாராவது பதிலளித்தவுடன் நின்றுவிடும். தானியங்கு பதில்கள் கணக்கில் வராது.
follow-up-note-send = உரையாடலில் யாராவது பதிலளித்தவுடன் நின்றுவிடும். வார நாட்களில் { $start } முதல் { $end } வரை அனுப்பப்படும், ஒருபோதும் ஒரு நாளுக்கு மேல் தாமதமாகாது.
follow-up-cancel = ரத்துசெய்
follow-up-done = முடிந்தது
follow-up-chip-send = { $time } இல் ஃபாலோ-அப்
follow-up-chip-remind = { $time } இல் நினைவூட்டல்
follow-up-chip-send-on = ஃபாலோ-அப் { $date }
follow-up-chip-remind-on = நினைவூட்டல் { $date }

## The card over an open conversation a follow-up waits on.

follow-up-card-title = இன்னும் பதில் இல்லை
follow-up-card-title-waiting = உங்கள் ஃபாலோ-அப் காத்திருக்கிறது
follow-up-card-send = Katna உங்கள் ஃபாலோ-அப்பை { $date } அன்று அனுப்பும். யாராவது பதிலளித்தால் நின்றுவிடும்.
follow-up-card-send-twice = Katna உங்கள் ஃபாலோ-அப்பை { $date } அன்று அனுப்பும், பின்னர் இன்னொரு முறை அனுப்பும். யாராவது பதிலளித்தால் நின்றுவிடும்.
follow-up-card-remind = யாரும் பதிலளிக்கவில்லையெனில், இந்த உரையாடல் { $date } அன்று உங்கள் இன்பாக்ஸுக்குத் திரும்ப வரும்.
follow-up-card-waiting = உங்கள் கணினி அணைந்திருந்தபோது இதன் நேரம் வந்தது, எனவே தாமதமாக அனுப்பப்படவில்லை. இப்போதே அனுப்புங்கள், புதிய நேரத்தைத் தேர்ந்தெடுங்கள் அல்லது நிறுத்துங்கள்.
follow-up-card-edit = திருத்து
follow-up-card-edit-title = எப்போது ஃபாலோ-அப் செய்ய வேண்டும்
follow-up-card-send-now = இப்போதே அனுப்பு
follow-up-card-stop = நிறுத்து
follow-up-chat-send = ஃபாலோ-அப் · யாரும் பதிலளிக்கவில்லையெனில் { $date }
follow-up-chat-step = ஃபாலோ-அப் { $steps } இல் { $step } · யாரும் பதிலளிக்கவில்லையெனில் { $date }
follow-up-chat-waiting = ஃபாலோ-அப் காத்திருக்கிறது · உங்கள் கணினி அணைந்திருந்தபோது இதன் நேரம் வந்தது
follow-up-chat-remind = பதில் இல்லையெனில் { $date } அன்று இன்பாக்ஸுக்குத் திரும்பும்
toast-follow-up-sent = ஃபாலோ-அப் அனுப்பப்பட்டது
toast-follow-up-stopped = ஃபாலோ-அப் நிறுத்தப்பட்டது
toast-follow-up-moved = ஃபாலோ-அப் { $date } க்கு மாற்றப்பட்டது
nudge-row = { $days ->
    [one] { $days } நாளுக்கு முன்
   *[other] { $days } நாட்களுக்கு முன்
} அனுப்பப்பட்டது. ஃபாலோ-அப் செய்யவா?
nudge-row-tip = இதில் உள்ள அனைவருக்கும் ஃபாலோ-அப் எழுது
nudge-follow-up = ஃபாலோ-அப்
nudge-dismiss = நிராகரி
nudge-card-title = இன்னும் பதில் இல்லை
nudge-card-text = நீங்கள் { $days ->
    [one] { $days } நாளுக்கு முன்
   *[other] { $days } நாட்களுக்கு முன்
} ஏதோ கேட்டீர்கள், யாரும் பதிலளிக்கவில்லை.
nudge-chat-line = { $days ->
    [one] { $days } நாளுக்கு முன்
   *[other] { $days } நாட்களுக்கு முன்
} அனுப்பப்பட்டது, இன்னும் பதில் இல்லை
toast-nudge-dismissed = நினைவுத் தூண்டல் நிராகரிக்கப்பட்டது
