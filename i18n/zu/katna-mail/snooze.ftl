# Katna Mail, Zulu (isiZulu).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.
snooze-until = Libazisa kuze kube…
snooze-later-today = Kamuva namuhla
snooze-tomorrow = Kusasa
snooze-this-weekend = Ngale mpelasonto
snooze-next-week = Ngesonto elizayo
snooze-pick = Khetha usuku nesikhathi
snooze-back = Buyela ezikhathini
snooze-type-placeholder = Thayipha isikhathi
snooze-type-hint = Njengokuthi “tue 3pm”, “tomorrow” noma “in 2 hours”
snooze-type-hint-unclear = I-Katna ayikwazi ukufunda lokho njengesikhathi
snooze-type-unclear = “{ $text }” akusona isikhathi i-Katna esaziyo

## Snooze and Remind me on mail: one menu, switched at its top (B and H)

snooze-tab = Libazisa
remind-tab = Ngikhumbuze
snooze-says = Iyayifihla kuze kube yileso sikhathi
remind-says = Iyigcina lapho ikhona futhi ikwazise
remind-before-due = Ngaphambi kokuba kufike isikhathi sayo
remind-note = Inothi (ongakukhetha)
remind-note-placeholder = Isihloko, uma kushiywe kungenalutho
toast-remind-set = Isikhumbuzo sisethelwe u-{ $date }
remind-chat-line = Isikhumbuzo { $date } · { $title }
remind-done = Kwenziwe
toast-remind-done = Isikhumbuzo senziwe
snooze-chat-line = Kulibazisiwe kuze kube ngu-{ $date }
snooze-chat-change = Shintsha
snooze-cancel = Khansela
snooze-save = Londoloza
snooze-in-the-past = Khetha isikhathi esingemva kwamanje.

## Follow up if no reply: compose's send menu, its popover and the chip
## beside Send

follow-up-menu = Landelela uma kungekho mpendulo…
follow-up-title = Landelela uma kungekho mpendulo
follow-up-off = Kuvaliwe
follow-up-days = { $days ->
    [one] Usuku olu-{ $days }
   *[other] Izinsuku ezingu-{ $days }
}
follow-up-weeks = { $weeks ->
    [one] Iviki eli-{ $weeks }
   *[other] Amaviki angu-{ $weeks }
}
follow-up-pick = Khetha…
follow-up-pick-title = Landelela uma kungekho mpendulo ngo-
follow-up-remind = Ngikhumbuze
follow-up-remind-note = Ingxoxo ibuyela phezulu ebhokisini lakho lokungenayo
follow-up-send = Ngithumelele umlayezo wokulandelela
follow-up-send-note = Kubantu abafanayo, engxoxweni efanayo
follow-up-send-encrypted = Akwenzelwe imeyili ebethelwe
follow-up-text-placeholder = Okuzobhalwa
follow-up-text-named = Sawubona { $name }, ngihlola nje ukuthi ubone umlayezo wami ongezansi.
follow-up-text = Sawubona, ngihlola nje ukuthi ubone umlayezo wami ongezansi.
follow-up-template = Sebenzisa isifanekiso
follow-up-signature = Isiginesha yakho iyengezwa
follow-up-again = Uma kusekho mpendulo, landelela futhi emva kwe-
follow-up-note = Kuyama uma nje noma ubani osengxoxweni ephendula. Izimpendulo ezizenzakalelayo azibalwa.
follow-up-note-send = Kuyama uma nje noma ubani osengxoxweni ephendula. Kuphuma phakathi nesonto kusuka ku-{ $start } kuya ku-{ $end }, futhi akulokothi kubambezeleke ngaphezu kosuku.
follow-up-cancel = Khansela
follow-up-done = Kwenziwe
follow-up-chip-send = Ukulandelela ngemva kuka-{ $time }
follow-up-chip-remind = Isikhumbuzo ngemva kuka-{ $time }
follow-up-chip-send-on = Ukulandelela { $date }
follow-up-chip-remind-on = Isikhumbuzo { $date }

## The card over an open conversation a follow-up waits on.

follow-up-card-title = Ayikabi bikho impendulo
follow-up-card-title-waiting = Ukulandelela kwakho kulindile
follow-up-card-send = I-Katna ithumela ukulandelela kwakho ngo-{ $date }. Kuyama uma noma ubani ephendula.
follow-up-card-send-twice = I-Katna ithumela ukulandelela kwakho ngo-{ $date }, bese iphinda kanye futhi kamuva. Kuyama uma noma ubani ephendula.
follow-up-card-remind = Uma kungekho ophendulayo, le ngxoxo ibuyela ebhokisini lakho lokungenayo ngo-{ $date }.
follow-up-card-waiting = Isikhathi sakho sifike ngesikhathi ikhompyutha yakho ivaliwe, ngakho akuthunyelwanga sekwephuzile. Kuthumele manje, khetha isikhathi esisha, noma ukumise.
follow-up-card-edit = Hlela
follow-up-card-edit-title = Landelela ngo-
follow-up-card-send-now = Thumela manje
follow-up-card-stop = Misa
follow-up-chat-send = Ukulandelela · { $date } uma kungekho ophendulayo
follow-up-chat-step = Ukulandelela { $step } kokungu-{ $steps } · { $date } uma kungekho ophendulayo
follow-up-chat-waiting = Ukulandelela kulindile · isikhathi sakho sifike ngesikhathi ikhompyutha yakho ivaliwe
follow-up-chat-remind = Kubuyela ebhokisini lokungenayo { $date } uma kungekho mpendulo
toast-follow-up-sent = Ukulandelela kuthunyelwe
toast-follow-up-stopped = Ukulandelela kumisiwe
toast-follow-up-moved = Ukulandelela kuhanjiswe ku-{ $date }
nudge-row = Kuthunyelwe { $days ->
    [one] osukwini olu-{ $days } olwedlule
   *[other] ezinsukwini ezingu-{ $days } ezedlule
}. Landelela?
nudge-row-tip = Bhala ukulandelela kubo bonke abakuyo
nudge-follow-up = Landelela
nudge-dismiss = Cashisa
nudge-card-title = Ayikabi bikho impendulo
nudge-card-text = Ubuze okuthile { $days ->
    [one] osukwini olu-{ $days } olwedlule
   *[other] ezinsukwini ezingu-{ $days } ezedlule
} futhi akekho ophendulile.
nudge-chat-line = Kuthunyelwe { $days ->
    [one] osukwini olu-{ $days } olwedlule
   *[other] ezinsukwini ezingu-{ $days } ezedlule
}, ayikabi bikho impendulo
toast-nudge-dismissed = Isikhumbuzo esincane sicashisiwe
