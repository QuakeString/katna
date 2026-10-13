# Katna Mail, Filipino (Filipino).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The snooze menu (the clock on a line, or Snooze in the right-click menu)

snooze-until = I-snooze hanggang…
snooze-later-today = Mamaya
snooze-tomorrow = Bukas
snooze-this-weekend = Ngayong weekend
snooze-next-week = Sa susunod na linggo
snooze-pick = Pumili ng petsa at oras
snooze-back = Bumalik sa mga oras
snooze-type-placeholder = Mag-type ng oras
snooze-type-hint = Gaya ng “tue 3pm”, “tomorrow” o “in 2 hours”
snooze-type-hint-unclear = Hindi iyan mabasa ng Katna bilang oras
snooze-type-unclear = Ang “{ $text }” ay hindi oras na alam ng Katna

## Snooze and Remind me on mail: one menu, switched at its top (B and H)

snooze-tab = I-snooze
remind-tab = Paalalahanan ako
snooze-says = Itinatago ito hanggang sa oras na iyon
remind-says = Iniiwan ito sa kinalalagyan at inaabisuhan ka
remind-before-due = Bago ang takdang araw
remind-note = Tala (opsyonal)
remind-note-placeholder = Ang subject, kung iiwang blangko
toast-remind-set = Nakatakda ang paalala sa { $date }
remind-chat-line = Paalala { $date } · { $title }
remind-done = Tapos na
toast-remind-done = Tapos na ang paalala
snooze-chat-line = Naka-snooze hanggang { $date }
snooze-chat-change = Baguhin

## The date and time picker

snooze-cancel = Kanselahin
snooze-save = I-save
snooze-in-the-past = Pumili ng oras na mas huli kaysa ngayon.

## Follow up if no reply: compose's send menu, its popover and the chip
## beside Send

follow-up-menu = Mag-follow up kung walang sagot…
follow-up-title = Mag-follow up kung walang sagot
follow-up-off = Naka-off
follow-up-days = { $days ->
    [one] { $days } araw
   *[other] { $days } araw
}
follow-up-weeks = { $weeks ->
    [one] { $weeks } linggo
   *[other] { $weeks } linggo
}
follow-up-pick = Pumili…
follow-up-pick-title = Mag-follow up kung walang sagot pagsapit ng
follow-up-remind = Paalalahanan ako
follow-up-remind-note = Babalik ang pag-uusap sa itaas ng iyong Inbox
follow-up-send = Magpadala ng follow-up para sa akin
follow-up-send-note = Sa parehong mga tao, sa parehong pag-uusap
follow-up-send-encrypted = Hindi para sa naka-encrypt na mail
follow-up-text-placeholder = Ano ang isusulat
follow-up-text-named = Hi { $name }, gusto ko lang tiyakin kung nakita mo ang mensahe ko sa ibaba.
follow-up-text = Hi, gusto ko lang tiyakin kung nakita mo ang mensahe ko sa ibaba.
follow-up-template = Gumamit ng template
follow-up-signature = Idinaragdag ang iyong lagda
follow-up-again = Kung wala pa ring sagot, mag-follow up ulit pagkalipas ng
follow-up-note = Hihinto agad kapag may sumagot sa pag-uusap. Hindi kasama ang mga auto-reply.
follow-up-note-send = Hihinto agad kapag may sumagot sa pag-uusap. Ipinapadala tuwing weekday mula { $start } hanggang { $end }, at hindi kailanman lalampas nang higit sa isang araw.
follow-up-cancel = Kanselahin
follow-up-done = Tapos na
follow-up-chip-send = Follow-up sa loob ng { $time }
follow-up-chip-remind = Paalala sa loob ng { $time }
follow-up-chip-send-on = Follow-up { $date }
follow-up-chip-remind-on = Paalala { $date }

## The card over an open conversation a follow-up waits on.

follow-up-card-title = Wala pang sagot
follow-up-card-title-waiting = Naghihintay ang iyong follow-up
follow-up-card-send = Ipapadala ng Katna ang iyong follow-up sa { $date }. Hihinto ito kapag may sumagot.
follow-up-card-send-twice = Ipapadala ng Katna ang iyong follow-up sa { $date }, at isa pa mamaya. Hihinto ito kapag may sumagot.
follow-up-card-remind = Kung walang sumagot, babalik ang pag-uusap na ito sa iyong Inbox sa { $date }.
follow-up-card-waiting = Dapat itong ipadala habang nakapatay ang iyong computer, kaya hindi ito ipinadala nang huli. Ipadala ito ngayon, pumili ng bagong oras, o ihinto ito.
follow-up-card-edit = I-edit
follow-up-card-edit-title = Mag-follow up sa
follow-up-card-send-now = Ipadala ngayon
follow-up-card-stop = Ihinto
follow-up-chat-send = Follow-up · { $date } kung walang sumagot
follow-up-chat-step = Follow-up { $step } sa { $steps } · { $date } kung walang sumagot
follow-up-chat-waiting = Naghihintay ang follow-up · dapat itong ipadala habang nakapatay ang iyong computer
follow-up-chat-remind = Babalik sa Inbox { $date } kung walang sagot
toast-follow-up-sent = Naipadala ang follow-up
toast-follow-up-stopped = Inihinto ang follow-up
toast-follow-up-moved = Inilipat ang follow-up sa { $date }
nudge-row = Ipinadala { $days ->
    [one] { $days } araw ang nakalipas
   *[other] { $days } araw ang nakalipas
}. Mag-follow up?
nudge-row-tip = Sumulat ng follow-up sa lahat ng nasa usapan
nudge-follow-up = Mag-follow up
nudge-dismiss = Isara
nudge-card-title = Wala pang sagot
nudge-card-text = May itinanong ka { $days ->
    [one] { $days } araw ang nakalipas
   *[other] { $days } araw ang nakalipas
} at walang sumagot.
nudge-chat-line = Ipinadala { $days ->
    [one] { $days } araw ang nakalipas
   *[other] { $days } araw ang nakalipas
}, wala pang sagot
toast-nudge-dismissed = Isinara ang paalala
