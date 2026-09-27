# Katna Mail, Filipino (Filipino).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Accounts

accounts-folder-pane = Pane ng folder
accounts-folder-pane-detail = Kung aling mga folder ng account ang ipinapakita ng pane sa kaliwa.
accounts-shown-one = Isang account sa bawat pagkakataon; magpalit sa account card
accounts-shown-all = Lahat ng account, sunud-sunod
accounts-row = Mga Account
accounts-row-detail = Sa ganitong ayos inililista ng pane ng folder at ng menu ng account ang mga account; ang una ang default. Kapag nag-alis ng account, mabubura ang kopya ng Katna ng mail nito sa computer na ito. Mananatili ang mail sa server.
accounts-none = Wala pang account.
accounts-kind-imported = Na-import
accounts-picture-reset = Gamitin ang larawan ng desktop
accounts-picture-change = Palitan ang larawan
accounts-picture-remove = Alisin ang larawan
accounts-rename = Palitan ang pangalan
accounts-name-save = I-save
accounts-name-cancel = Kanselahin
accounts-name-placeholder = Ang pangalan mo
accounts-rename-failed = Hindi mapalitan ang pangalan ng account: { $error }
accounts-move-up = Iakyat
accounts-move-down = Ibaba
accounts-drag = I-drag para baguhin ang ayos
accounts-remove = Alisin
accounts-delete-all-row = I-delete ang lahat ng data
accounts-delete-all-row-detail = Magsimulang muli, gaya ng sa bagong install.
accounts-delete-all-about = Dine-delete mula sa computer na ito ang bawat account, lahat ng naka-save na mail, mga contact at kalendaryo, ang search index, ang iyong mga setting at mga naka-save na password. Walang nagbabago sa iyong mga mail server.
accounts-delete-all-open = I-delete ang lahat ng data ng Katna

## Settings > Accounts: snackbars after deleting

accounts-removed-local = Inalis ang { $address } sa Katna.
accounts-removed = Inalis ang { $address } sa Katna. Nasa server pa rin ang mail nito.
accounts-all-deleted = Na-delete sa computer na ito ang lahat ng data ng Katna.

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = Alisin ang { $address }?
accounts-remove-confirm = Alisin ang account
accounts-removing = Inaalis…
accounts-remove-local-mail = { $folders ->
    [0] Lahat ng mail na na-import sa account na ito
    [1] Lahat ng mail na na-import sa account na ito, sa folder nito
   *[other] Lahat ng mail na na-import sa account na ito, sa { $folders } folder nito
}
accounts-remove-local-settings = Ang mga setting nito sa Katna
accounts-remove-mail = { $folders ->
    [0] Lahat ng mail ng account na ito na naka-save sa Katna
    [1] Lahat ng mail ng account na ito na naka-save sa Katna, sa folder nito
   *[other] Lahat ng mail ng account na ito na naka-save sa Katna, sa { $folders } folder nito
}
accounts-remove-outbox = Ang mga mensahe nitong naghihintay sa outbox
accounts-remove-settings = Ang naka-save nitong password at ang mga setting nito sa Katna
accounts-delete-all-title = I-delete ang lahat ng data ng Katna?
accounts-delete-all-confirm = I-delete lahat
accounts-deleting = Dine-delete…
accounts-delete-all-accounts = Bawat account, at lahat ng mail at attachment na naka-save sa Katna
accounts-delete-all-contacts = Mga contact, kalendaryo at ang search index
accounts-delete-all-settings = Lahat ng setting, lagda at keyboard shortcut
accounts-delete-all-passwords = Bawat naka-save na password
accounts-deleted-heading = Mabubura sa computer na ito:
accounts-cannot-undo = Hindi na ito maa-undo.
accounts-server-delete-all = Walang nagbabago sa iyong mga mail server: nananatili roon ang mail mo, at kapag idinagdag muli ang isang account, dina-download itong muli. Nasa Katna lang ang mail na na-import mula sa mga file; hindi ginagalaw ang mga file.
accounts-server-local = Na-import ang mail na ito mula sa mga file, kaya ang Katna lang ang may kopya. Hindi ginagalaw ang mga file na pinagmulan nito; i-import muli ang mga ito para maibalik ito.
accounts-server-remove = Walang nagbabago sa mail server: nananatili roon ang mail mo, at kapag idinagdag muli ang account, dina-download itong muli.
accounts-confirm-word = burahin
accounts-confirm-placeholder = I-type ang “{ accounts-confirm-word }”
accounts-confirm-prompt = Para kumpirmahin, i-type ang “{ accounts-confirm-word }”:
accounts-cancel = Kanselahin
