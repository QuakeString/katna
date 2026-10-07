# Katna Mail, Swahili (Kiswahili).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = Seva ya barua

problems-signed-out = { $provider } imeitoa Katna kwenye { $address }. Barua zimeacha kusawazishwa.
problems-password-refused = { $provider } imekataa nenosiri la { $address }. Huenda limebadilika.
problems-no-answer = { $provider } haijibu kwa { $address }. Katna inaendelea kujaribu.
problems-offline = Uko nje ya mtandao. Barua zako bado ziko hapa, na barua unazotuma zinasubiri hadi utakaporudi.
problems-accounts-need-you = { $count ->
    [one] Akaunti 1 inakuhitaji
   *[other] Akaunti { $count } zinakuhitaji
}
problems-show = Onyesha
problems-later = Baadaye
problems-new-password = Nenosiri jipya
problems-try-again = Jaribu tena

## The New password card

problems-password-title = Nenosiri jipya
problems-password-detail = { $provider } imekataa nenosiri lililohifadhiwa la { $address }. Andika jipya; Katna hulikagua kabla ya kulihifadhi.
problems-password-placeholder = Nenosiri
problems-password-show = Onyesha nenosiri
problems-password-hide = Ficha nenosiri
problems-password-cancel = Ghairi
problems-password-save = Hifadhi
problems-password-checking = Inakagua…
problems-password-refused-again = { $provider } imekataa nenosiri hili pia. Likague na ujaribu tena.
problems-password-saved = Nenosiri limehifadhiwa kwa { $address }. Inapata barua zako…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = Seva ya barua ya { $address } haikukubali kuhamisha { $count ->
    [one] ujumbe, hivyo umerudi ulipokuwa.
   *[other] jumbe { $count }, hivyo zimerudi zilipokuwa.
}
problems-refused-flags = Seva ya barua ya { $address } haikukubali kutia alama { $count ->
    [one] kwenye ujumbe (imesomwa, ina nyota…), hivyo umerudi kama ulivyokuwa.
   *[other] kwenye jumbe { $count } (imesomwa, ina nyota…), hivyo zimerudi kama zilivyokuwa.
}
problems-refused-label = Seva ya barua ya { $address } haikukubali kubadilisha lebo za { $count ->
    [one] ujumbe, hivyo umerudi kama ulivyokuwa.
   *[other] jumbe { $count }, hivyo zimerudi kama zilivyokuwa.
}
problems-refused-delete = Seva ya barua ya { $address } haikukubali kufuta { $count ->
    [one] ujumbe, hivyo umerudi.
   *[other] jumbe { $count }, hivyo zimerudi.
}
problems-refused-other = Seva ya barua ya { $address } haikukubali { $count ->
    [one] badiliko, hivyo Katna imelirudisha kama lilivyokuwa.
   *[other] mabadiliko { $count }, hivyo Katna imeyarudisha kama yalivyokuwa.
}
problems-details = Maelezo

## Katna's background service (katna-daemon) isn't running

service-starting = Inaanzisha huduma ya chinichini ya Katna…
service-failed = Huduma ya chinichini ya Katna haianzi, hivyo barua hazisawazishwi.
service-start-again = Anzisha tena
service-started-again = Huduma ya chinichini ya Katna ilisimama na imeanzishwa tena.
service-details-title = Kwa nini huduma haianzi
service-details-body = Nakili hiki na ukitume pamoja na ripoti yako. Hakina barua wala manenosiri.
service-details-copy = Nakili
service-details-close = Funga
