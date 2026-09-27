# Katna Mail, Zulu (isiZulu).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = Vala
reader-back = Emuva
reader-mark-unread = Maka njengokungafundiwe
reader-move-to = Hambisa ku-
reader-more = Okuningi
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
reader-me = mina
reader-to = ku-{ $names }
reader-starred = Kunenkanyezi
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
tracking-opened = U-{ $who } uwuvule { $count ->
    [one] kanye
   *[other] izikhathi ezingu-{ $count }
}, okokugcina { $when }
tracking-opened-clicked = U-{ $who } uwuvule walandela isixhumanisi { $count ->
    [one] kanye
   *[other] izikhathi ezingu-{ $count }
}, okokugcina { $when }
tracking-maybe-opened = Kungenzeka ukuthi u-{ $who } uwuvulile (i-Apple Mail ilayisha izithombe ngenxa yobumfihlo)
tracking-not-opened = U-{ $who } akakawuvuli
tracking-receipt = U-{ $who } uthumele isaziso sokufunda
tracking-receipt-displayed = Isaziso sokufunda: u-{ $who } uvule umlayezo wakho
tracking-receipt-other = Isaziso sokufunda: u-{ $who } ususile noma uphathe umlayezo wakho engawuvulanga

## Remote images and pictures

remote-hidden = Izithombe kulo mlayezo zifihliwe.
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
print-preview-cancel = Khansela
print-preview-print = Phrinta
print-not-downloaded = (Akukalandwa.)
print-encrypted = (Kubethelwe. Kuvule ku-Katna Mail ukuze uphrinte umbhalo wakho.)
print-to = Ku: { $addresses }
print-cc = Cc: { $addresses }

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = Vula lo mlayezo ukuze ufunde okunamathiselwe kuwo.
text-copy = Kopisha
text-select-all = Khetha konke
