# Katna Mail, Afrikaans (Afrikaans).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Accounts

accounts-folder-pane = Vouerpaneel
accounts-folder-pane-detail = Watter rekeninge se vouers die paneel aan die linkerkant wys.
accounts-shown-one = Een rekening op 'n slag; wissel in die rekeningkaart
accounts-shown-all = Alle rekeninge, een ná die ander
accounts-row = Rekeninge
accounts-row-detail = Die vouerpaneel en die rekeningkieslys wys rekeninge in hierdie volgorde; die eerste is die verstek. As jy 'n rekening verwyder, word Katna se kopie van sy e-pos op hierdie rekenaar uitgevee. Die e-pos bly op die bediener.
accounts-none = Nog geen rekeninge nie.
accounts-kind-imported = Ingevoer
accounts-picture-reset = Gebruik werkskermprent
accounts-picture-change = Verander prent
accounts-picture-remove = Verwyder prent
accounts-rename = Hernoem
accounts-name-save = Stoor
accounts-name-cancel = Kanselleer
accounts-name-placeholder = Jou naam
accounts-rename-failed = Kon nie die rekening hernoem nie: { $error }
accounts-move-up = Skuif op
accounts-move-down = Skuif af
accounts-drag = Sleep om die volgorde te verander
accounts-remove = Verwyder
accounts-delete-all-row = Vee alle data uit
accounts-delete-all-row-detail = Begin oor, soos met 'n nuwe installasie.
accounts-delete-all-about = Vee elke rekening, alle gestoorde e-pos, kontakte en kalenders, die soekindeks, jou instellings en gestoorde wagwoorde van hierdie rekenaar uit. Niks verander op jou e-posbedieners nie.
accounts-delete-all-open = Vee alle Katna-data uit

## Settings > Accounts: snackbars after deleting

accounts-removed-local = { $address } is uit Katna verwyder.
accounts-removed = { $address } is uit Katna verwyder. Sy e-pos is steeds op die bediener.
accounts-all-deleted = Alle Katna-data is van hierdie rekenaar uitgevee.

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = Verwyder { $address }?
accounts-remove-confirm = Verwyder rekening
accounts-removing = Verwyder tans…
accounts-remove-local-mail = { $folders ->
    [0] Alle e-pos wat in hierdie rekening ingevoer is
    [one] Alle e-pos wat in hierdie rekening ingevoer is, in sy vouer
   *[other] Alle e-pos wat in hierdie rekening ingevoer is, in sy { $folders } vouers
}
accounts-remove-local-settings = Sy Katna-instellings
accounts-remove-mail = { $folders ->
    [0] Al hierdie rekening se e-pos wat Katna gestoor het
    [one] Al hierdie rekening se e-pos wat Katna in sy vouer gestoor het
   *[other] Al hierdie rekening se e-pos wat Katna in sy { $folders } vouers gestoor het
}
accounts-remove-outbox = Sy boodskappe wat in die uitkassie wag
accounts-remove-settings = Sy gestoorde wagwoord en sy Katna-instellings
accounts-delete-all-title = Vee alle Katna-data uit?
accounts-delete-all-confirm = Vee alles uit
accounts-deleting = Vee tans uit…
accounts-delete-all-accounts = Elke rekening, en alle e-pos en aanhegsels wat Katna gestoor het
accounts-delete-all-contacts = Kontakte, kalenders en die soekindeks
accounts-delete-all-settings = Alle instellings, handtekeninge en kortpadsleutels
accounts-delete-all-passwords = Elke gestoorde wagwoord
accounts-deleted-heading = Van hierdie rekenaar uitgevee:
accounts-cannot-undo = Dit kan nie ontdoen word nie.
accounts-server-delete-all = Niks verander op jou e-posbedieners nie: jou e-pos bly daar, en as jy 'n rekening weer byvoeg, word dit weer afgelaai. E-pos wat uit lêers ingevoer is, is net in Katna; die lêers word nie aangeraak nie.
accounts-server-local = Hierdie e-pos is uit lêers ingevoer, dus het Katna die enigste kopie. Die lêers waaruit dit kom, word nie aangeraak nie; voer hulle weer in om dit terug te kry.
accounts-server-remove = Niks verander op die e-posbediener nie: jou e-pos bly daar, en as jy die rekening weer byvoeg, word dit weer afgelaai.
accounts-confirm-word = skrap
accounts-confirm-placeholder = Tik “{ accounts-confirm-word }”
accounts-confirm-prompt = Tik “{ accounts-confirm-word }” om te bevestig:
accounts-cancel = Kanselleer
reset-cache-about = Vee die e-pos en aanhegsels wat Katna afgelaai het, senderprente en die soekindeks uit, en laai dan onlangse e-pos weer af. Rekeninge, instellings en e-pos wat net op hierdie rekenaar is, bly.
reset-cache-button = Herstel kas
reset-cache-title = Herstel die kas?
reset-cache-deleted = Uitgevee, dan weer afgelaai:
reset-cache-mail = E-pos en aanhegsels wat van jou IMAP-bedieners afgelaai is: onlangse e-pos word nou weer afgelaai, ouer e-pos wanneer jy dit oopmaak
reset-cache-index = Die soekindeks, wat dadelik herbou word
reset-cache-pictures = Senderprente
reset-cache-kept = Bly behoue: jou rekeninge, wagwoorde en instellings; sterre, etikette, leesmerke en vasspelde; konsepte, die uitkassie en veranderinge wat nog nie op die bediener is nie; en e-pos van POP3-rekeninge of ingevoerde lêers, wat dalk geen ander kopie het nie. Niks verander op jou e-posbedieners nie.
reset-cache-confirm = Herstel kas
reset-cache-busy = Herstel tans…
reset-cache-done = Die kas is herstel. Onlangse e-pos word weer afgelaai.
reset-cache-done-freed = Die kas is herstel en { $size } is vrygemaak. Onlangse e-pos word weer afgelaai.
