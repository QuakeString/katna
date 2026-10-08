# Katna Mail, Zulu (isiZulu).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = Vala
reader-back = Emuva
reader-mark-unread = Maka njengokungafundiwe
reader-move-to = Hambisa ku-
reader-snooze = Libazisa
reader-remind = Ngikhumbuze
reader-more = Okuningi
reader-original-colors = Bonisa imibala yangempela
reader-dark-colors = Bonisa ngemibala emnyama
reader-print-all = Phrinta konke
reader-new-window = Ewindini elisha
reader-position = { $position } kokungu-{ $total }
reader-newer = Okusha
reader-older = Okudala

## Reading pane: the conversation

reader-removed = Le ngxoxo isusiwe.
reader-no-subject = (asikho isihloko)
reader-collapse-all = Goqa konke
reader-expand-all = Nweba konke
reader-unknown-sender = (umthumeli ongaziwa)
reader-date-ago = { $date } ({ $ago })
reader-sending = Iyathumela…
reader-me = mina
reader-to = ku-{ $names }
reader-to-label = abamukeli:
reader-tick-delivered = Kulethiwe { $when }
reader-tick-no-bounce = Kuthunyelwe { $when }; ayikho imeyili ebuyile, ngakho cishe ifikile
reader-tick-bounced = Akulethwanga: kubuyile { $when }
reader-tick-read = Kufundiwe { $when } (isaziso sokufunda)
reader-tick-opened = Kuvuliwe, okokugcina { $when } (ukulandelela ukuvulwa)
reader-starred = Kunenkanyezi
reader-chip-remove = Susa { $label }
reader-not-starred = Akunankanyezi
reader-too-long = Umlayezo mude kakhulu ukuthi uboniswe wonke.
reader-encrypted-images = Izithombe ezivela kuwebhu azilayishwa neze kumeyili ebethelwe.
reader-window-failed = Ayikwazanga ukuvula iwindi elisha.

## Reading pane: message details (opened from "to me")

reader-details-from = kusuka ku:
reader-details-to = ku:
reader-details-cc = cc:
reader-details-date = usuku:
reader-details-subject = isihloko:

## Reading pane: downloading a message

reader-downloading = Ilanda lo mlayezo kuseva…
reader-download-failed = Ayikwazanga ukulanda lo mlayezo.
reader-download-failed-reason = Ayikwazanga ukulanda lo mlayezo. { $reason }
reader-download-offline = Le akhawunti ayixhunyiwe. Xhuma futhi ukuze ulande lo mlayezo.
reader-try-again = Zama futhi

## Reply row

reply-reply = Phendula
reply-reply-all = Phendula bonke
reply-forward = Dlulisela

## Encrypted and signed mail

security-decrypting = Iyasusa ukubethela…
security-checking = Ihlola isiginesha…
security-partly-encrypted = Yingxenye kuphela yalo mlayezo ebethelwe. Okunye kwengezwe ngaphandle kwesivikelo futhi kungavela kunoma ubani.
security-partly-signed = Yingxenye kuphela yalo mlayezo esayiniwe. Okunye kwengezwe ngaphandle kwesivikelo futhi kungavela kunoma ubani.
security-encrypted = Umlayezo obethelwe
security-encrypted-smime = Umlayezo obethelwe (S/MIME)
security-no-key = Ayikwazi ukususa ukubethela kulo mlayezo: ubethelwe ngokhiye ongenawo.
security-cancelled = Ukususa ukubethela kukhanseliwe.
security-damaged = Ayikwazi ukususa ukubethela kulo mlayezo: idatha ebethelwe yonakele noma ishintshiwe.
security-decrypt-unavailable = Ayikwazi ukususa ukubethela kulo mlayezo: faka i-{ $tool } ukuze ufunde imeyili ebethelwe.
security-decrypt-failed = Ayikwazi ukususa ukubethela kulo mlayezo: { $reason }
security-unknown-signer = umsayini ongaziwa
security-signed-verified = Kusayinwe ngu-{ $signer } · kuqinisekisiwe
security-signed-not-sender = Kusayinwe ngu-{ $signer }, ongesiye umthumeli
security-signed-untrusted = Kusayinwe ngu-{ $signer }, ngokhiye owumake njengongathembekile
security-signed-unverified = Kusayinwe ngu-{ $signer } · ukhiye awuqinisekisiwe
security-bad-signature = Isiginesha embi: lo mlayezo ushintshwe ngemva kokusayinwa, noma isiginesha ingumgunyathi.
security-signature-expired = Kusayinwe ngu-{ $signer } · isiginesha iphelelwe yisikhathi
security-key-expired = Kusayinwe ngu-{ $signer } · ukhiye usuphelelwe yisikhathi kusukela lapho
security-key-revoked = Kusayinwe ngu-{ $signer } ngokhiye ohoxisiwe
security-missing-key = Kusayinwe ngokhiye ongenawo, ngakho akukwazi ukuhlolwa
security-missing-key-id = Kusayinwe ngokhiye ongenawo ({ $key }), ngakho akukwazi ukuhlolwa
security-signature-unavailable = Kusayiniwe; faka i-{ $tool } ukuze uhlole isiginesha
security-signature-error = Isiginesha ayikwazanga ukuhlolwa.
security-look-up-key = Bheka ukhiye

## The key popover: the details of the key a message was signed with, and
## a key to import (looked up, or attached to the message)

key-card-verified = Isiginesha eqinisekisiwe
key-card-verified-detail = Isiginesha ilungile futhi uyamethemba lo khiye.
key-card-unverified = Isiginesha ayiqinisekisiwe
key-card-unverified-detail = Isiginesha ilungile, kodwa akukho okuqinisekisa ukuthi ukhiye ngowabo. Qhathanisa i-fingerprint nabo, bese uthemba ukhiye ku-GnuPG (Kleopatra noma gpg --edit-key).
key-card-not-sender = Kusayinwe ngomunye umuntu
key-card-not-sender-detail = Isiginesha ilungile, kodwa ukhiye akusiwo owomthumeli.
key-card-untrusted = Ukhiye awuthenjwa
key-card-untrusted-detail = Ulimake lo khiye njengongathembekile ku-GnuPG.
key-card-signature-expired = Isiginesha iphelelwe yisikhathi
key-card-signature-expired-detail = Isiginesha ibilungile, kodwa isiphelelwe yisikhathi.
key-card-key-expired = Ukhiye uphelelwe yisikhathi
key-card-key-expired-detail = Isiginesha ilungile, kodwa ukhiye usuphelelwe yisikhathi kusukela lapho.
key-card-key-revoked = Ukhiye uhoxisiwe
key-card-key-revoked-detail = Umnikazi wawo uhoxise lo khiye, ngakho isiginesha ayikwazi ukuthenjwa.
key-card-bad = Isiginesha embi
key-card-bad-detail = Lo mlayezo ushintshwe ngemva kokusayinwa, noma isiginesha ingumgunyathi.
key-card-signed-by = Kusayinwe ngu
key-card-belongs-to = Ungowaka
key-card-fingerprint = I-fingerprint
key-card-signed = Kusayinwe
key-card-key = Ukhiye
key-card-kind = { $standard }, { $algorithm }
key-card-created = Wenziwe
key-card-expires = Uphelelwa yisikhathi
key-card-never = Neze
key-card-issued-by = Ikhishwe ngu
key-card-found-in = Utholakale ku
key-card-keyring = I-keyring yakho ye-GnuPG
key-card-copy = Kopisha i-fingerprint
key-card-import-title = Ngenisa lo khiye?
key-card-from-directory = Utholakale kuhla lukakhiye lwe-{ $domain }.
key-card-from-attachment = Kusuka kokunamathiselwe { $name }.
key-card-import-note = I-Katna ingabe isihlola amasiginesha alo muntu futhi ibethele imeyili eya kuye. Ukuze uthembe ukhiye ngokugcwele, qhathanisa i-fingerprint naye.
key-card-cancel = Khansela
key-card-import = Ngenisa ukhiye
key-card-looking-up = Kubhekwa ukhiye…
key-card-looking-up-detail = Kubuzwa uhla lukakhiye lwe-{ $domain }.
key-card-not-found = Akukho khiye otholakele
key-card-not-found-detail = I-{ $domain } ayishicileli ukhiye waleli kheli. Cela umthumeli akuthumelele owakhe.
key-card-not-kept = Ukhiye otholakele awukwazi ukusetshenziswa.
key-card-failed = Ayikwazanga ukuthola ukhiye

## Sender checks: the banner on mail that failed the checks the user's mail
## provider ran on its sender (DMARC, DKIM, SPF), and the "?" on the picture
## of a sender nothing confirmed

sender-failed-title = Lokhu kungase kungaveli ku-{ $domain }
sender-failed-body = Kuhlulekile ekuhloleni umthumeli kwe-{ $provider }. Qaphela ngezixhumanisi, okunamathiselwe nezimpendulo.
sender-provider-unknown = umhlinzeki wakho we-imeyili
sender-details = Imininingwane
sender-details-hide = Fihla imininingwane
sender-looks-safe = Kubukeka kuphephile
sender-move-to-spam = Hambisa ku-Ugaxekile
sender-checked-by = Kuhlolwe yi-{ $provider }
sender-checked-by-server = Kuhlolwe yi-{ $provider } ({ $server })
sender-dmarc = Isizinda somthumeli (DMARC)
sender-dkim = Isiginesha (DKIM)
sender-spf = Iseva ethumelayo (SPF)
sender-result-pass = Kuphumelele
sender-result-fail = Kuhlulekile
sender-result-unsure = Akuqinisekile
sender-result-none = Lutho
sender-result-missing = Akuhlolwanga
sender-dmarc-pass = I-{ $domain } iyaqinisekisa lo mthumeli.
sender-dmarc-fail = Imeyili ayihambisani nendlela i-{ $domain } ethi imeyili yayo ithunyelwa ngayo.
sender-dmarc-none = I-{ $domain } ayishicileli mithetho yemeyili yayo.
sender-dkim-pass = Isayinwe yi-{ $domain }.
sender-dkim-fail = Isiginesha evela ku-{ $domain } ayihambisani nemeyili.
sender-dkim-none = Umlayezo awuzange usayinwe.
sender-spf-pass = Ithunyelwe kusuka kuseva i-{ $domain } eyibalayo.
sender-spf-fail = Ithunyelwe kusuka kuseva i-{ $domain } engayibali.
sender-spf-none = I-{ $domain } ayiwabali amaseva ayo.
sender-check-unsure = Ukuhlola akukwazanga ukunikeza impendulo ecacile.
sender-unconfirmed = I-{ $provider } ayikwazanga ukuqinisekisa ukuthi lokhu kuvela ku-{ $domain }. Noma ubani angabhala noma yimuphi umthumeli.
sender-link-title = Vula lesi sixhumanisi?
sender-link-body = Le meyili ihlulekile ekuhloleni umthumeli. Isixhumanisi siya ku-{ $host }:
sender-link-cancel = Khansela
sender-link-open = Vula
tracking-opened = U-{ $who } uwuvule { $count ->
    [one] kanye
   *[other] izikhathi ezingu-{ $count }
}, okokugcina { $when }
tracking-opens-clicks = U-{ $who } uwuvule { $opens ->
    [one] kanye
   *[other] izikhathi ezingu-{ $opens }
} futhi walandela isixhumanisi { $clicks ->
    [one] kanye
   *[other] izikhathi ezingu-{ $clicks }
}, okokugcina { $when }
tracking-clicked = U-{ $who } ulandele isixhumanisi { $clicks ->
    [one] kanye
   *[other] izikhathi ezingu-{ $clicks }
}, okokugcina { $when }
tracking-maybe-opened = Kungenzeka ukuthi u-{ $who } uwuvulile (i-Apple Mail ilayisha izithombe ngenxa yobumfihlo)
tracking-seen-none = Akekho osewuvulile noma olandele isixhumanisi okwamanje
tracking-receipt = U-{ $who } uthumele isaziso sokufunda
tracking-receipt-read = { $who } ukufundile (isaziso sokufundwa), { $when }
tracking-receipt-displayed = Isaziso sokufunda: u-{ $who } uvule umlayezo wakho
tracking-receipt-other = Isaziso sokufunda: u-{ $who } ususile noma uphathe umlayezo wakho engawuvulanga

## Remote images and pictures

remote-hidden = Izithombe kulo mlayezo zifihliwe.
remote-hidden-unconfirmed = Izithombe zifihliwe: umthumeli akakwazanga ukuqinisekiswa.
remote-hidden-failed = Izithombe zifihliwe: le meyili ihlulekile ekuhloleni umthumeli.
remote-show = Bonisa izithombe
remote-always-show = Bonisa njalo kusuka kulo mthumeli
remote-picture-use = Sebenzisa
remote-picture-too-big = Khetha isithombe esingu-8 MB noma ngaphansi.
remote-picture-type = Khetha isithombe se-PNG, JPEG, GIF, WebP noma SVG.
remote-picture-read-failed = Ayikwazi ukufunda isithombe: { $error }
remote-picture-keep-failed = Ayikwazi ukugcina isithombe: { $error }
remote-picture-remove-failed = Ayikwazi ukususa isithombe: { $error }

## Attachments

attachment-count = { $count ->
    [one] Okunamathiselwe okukodwa
   *[other] Okunamathiselwe okungu-{ $count }
}
attachment-save = Londoloza
attachment-forward = Dlulisela
attachment-save-all = Londoloza konke
attachment-save-all-tooltip = Londoloza konke okunamathiselwe kufolda
attachment-save-here = Londoloza lapha
attachment-not-downloaded = Lo mlayezo awulandiwe.
attachment-not-found = Lokhu okunamathiselwe akutholakalanga emlayezweni.
attachment-read-failed = Ayikwazanga ukufunda i-{ $name }
attachment-numbered = okunamathiselwe { $number }
attachment-saved-all = { $count ->
    [one] Kulondolozwe ifayela elingu-{ $count } ku-{ $place }
   *[other] Kulondolozwe amafayela angu-{ $count } ku-{ $place }
}
attachment-saved-some = { $total ->
    [one] Kulondolozwe angu-{ $saved } kwifayela elingu-{ $total } ku-{ $place }. Ayikwazanga ukulondoloza i-{ $failed }
   *[other] Kulondolozwe angu-{ $saved } kwamafayela angu-{ $total } ku-{ $place }. Ayikwazanga ukulondoloza i-{ $failed }
}
attachment-saved-to = Kulondolozwe ku-{ $path }
attachment-save-failed = Ayikwazanga ukulondoloza i-{ $name }: { $error }
attachment-open-failed = Ayikwazanga ukuvula i-{ $name }: { $error }
attachment-risky = Leli fayela lingaqalisa uhlelo, ngakho i-Katna ayilivuli. Kunalokho lilondoloze.
attachment-encrypted-open = Leli fayela lifike libethelwe. Lilondoloze ukuze ulivule kwenye indawo.

## Printing

print-failed = Ayikwazanga ukuphrinta: { $error }
print-no-font = alikho ifonti elitholakele
print-opened-as-pdf = Kuvulwe njenge-PDF ukuze uphrinte ukusuka lapho.
print-preview-title = Ukubuka kuqala kokuphrinta
print-preview-laying-out = Kuhlelwa amakhasi…
print-preview-pages = { $count ->
    [one] Ikhasi elingu-{ $count }
   *[other] Amakhasi angu-{ $count }
}
print-preview-more = { $count ->
    [one] nekhasi elingu-{ $count } elengeziwe
   *[other] namakhasi angu-{ $count } engeziwe
}
print-preview-failed = amakhasi awakwazanga ukuboniswa
print-preview-paper = Iphepha
print-preview-a4 = A4
print-preview-letter = Letter
print-preview-layout = Isakhiwo
print-preview-as-shown = Njengokuboniswa
print-preview-simple = Umbhalo kuphela
print-preview-backgrounds = Izingemuva
print-preview-cancel = Khansela
print-preview-print = Phrinta
print-not-downloaded = (Akukalandwa.)
print-encrypted = (Kubethelwe. Kuvule ku-Katna Mail ukuze uphrinte umbhalo wakho.)
print-to = Ku: { $addresses }
print-cc = Cc: { $addresses }

## Message text (right-click menu in the reading pane)

text-pin = Phina phezulu
text-copy-address = Kopisha ikheli

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = Vula lo mlayezo ukuze ufunde okunamathiselwe kuwo.
text-copy = Kopisha
text-select-all = Khetha konke
