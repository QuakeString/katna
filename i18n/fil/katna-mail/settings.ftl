# Katna Mail, Filipino (Filipino).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings page: its tabs

settings-tab-general = Pangkalahatan
settings-tab-inbox = Inbox
settings-tab-accounts = Mga Account
settings-tab-subscriptions = Mga Subscription
settings-tab-appearance = Hitsura
settings-tab-shortcuts = Mga Shortcut
settings-tab-default-apps = Mga default na app
settings-tab-folders-rules = Mga folder at panuntunan
settings-tab-compose = Mag-compose
settings-tab-mcp-server = MCP server
settings-tab-feedback = Feedback ng user
settings-tab-experimental = Pang-eksperimento

## Settings page: tabs still to come

settings-tab-subscriptions-coming = Tingnan ang mga newsletter at mailing list na natatanggap mo, at mag-unsubscribe sa isang click.
settings-tab-folders-rules-coming = Gumawa, mag-rename, maglipat at magtago ng mga folder at label, at piliin kung alin ang magsi-sync. Kusang inaayos, nilalagyan ng label, ipinapasa o dine-delete ng mga panuntunan ang bagong mail, ayon sa nagpadala, subject o mga salita.
settings-tab-mcp-server-coming = Payagan ang mga AI assistant sa computer na ito na maghanap, magbasa at mag-draft ng iyong mail, nang may pahintulot mo.

## Settings > General

settings-general-conversations = View ng pag-uusap
settings-general-conversations-group = Pagsamahin ang mga sagot sa iisang mail
settings-general-conversations-group-detail = Isang linya bawat pag-uusap sa listahan
settings-general-reading = Pagbabasa
settings-general-newest-first = Pinakabagong mensahe muna
settings-general-newest-first-detail = Nagsisimula ang pag-uusap sa pinakahuling sagot nito
settings-general-full-headers = Ipakita ang buong header
settings-general-full-headers-detail = Nakabukas ang mula kay, para kay, cc, petsa at subject sa bawat mensahe
settings-general-full-names = Buong pangalan ng mga tatanggap
settings-general-full-names-detail = “para sa akin, Ada Lovelace” sa halip na “para sa akin, Ada”
settings-general-mark-read = Markahan bilang nabasa na
settings-general-mark-read-now = Sa sandaling mabuksan ito
settings-general-mark-read-1s = Pagkatapos itong mabuksan nang 1 segundo
settings-general-mark-read-3s = Pagkatapos itong mabuksan nang 3 segundo
settings-general-mark-read-never = Kapag minarkahan ko lang itong nabasa na
settings-general-auto-advance = Auto-advance
settings-general-auto-advance-detail = Pagkatapos mong i-delete, i-archive o ilipat ang nakabukas na pag-uusap
settings-general-auto-advance-next = Buksan ang susunod na pag-uusap
settings-general-auto-advance-previous = Buksan ang nakaraang pag-uusap
settings-general-auto-advance-list = Bumalik sa listahan
settings-general-reply-button = Button na Sumagot
settings-general-reply-all = Sumagot sa lahat
settings-general-reply-all-detail = Sumasagot sa lahat ang button na sumagot sa tabi ng bawat mensahe, hindi lang sa nagpadala
settings-general-remote-images = Mga larawan mula sa web
settings-general-remote-images-detail = Kapag nilo-load ang mga larawan ng isang mensahe, nalalaman ng nagpadala na binuksan mo ito, kailan, at halos kung saan. Kapag naka-off, nagtatanong muna ang bawat mensahe, at palagi mong maipapakita ang mga larawan ng isang nagpadala.
settings-general-remote-images-always = Palaging ipakita ang mga larawan
settings-general-remote-images-always-detail = Sa bawat mensahe, hindi lang mula sa mga nagpadalang pinagkakatiwalaan mo
settings-general-sending = Pagpapadala
settings-general-sending-detail = Gaano katagal naghihintay ang naipadalang mensahe, para mabawi pa ito.
settings-general-offline = Offline na mail
settings-general-offline-detail = Buong dina-download ang kamakailang mail, para mabasa nang walang koneksyon. Dina-download ang mas lumang mail kapag binuksan mo ito.
settings-general-offline-days = { $count ->
    [one] { $count } araw
   *[other] { $count } araw
}
settings-general-offline-years = { $count ->
    [one] { $count } taon
   *[other] { $count } taon
}
settings-general-offline-all = Lahat ng mail
settings-general-offline-note = Kapag pumili ng mas kaunting araw, mananatili ang mail na na-download na. Walang nagbabago sa server.
settings-general-notifications = Mga Notification
settings-general-notifications-detail = Para sa bagong mail sa Inbox, kahit nakasara ang Katna Mail.
settings-general-new-mail = Abisuhan ako tungkol sa bagong mail
settings-general-new-mail-detail = May Sumagot sa lahat, Markahan bilang nabasa na at I-archive
settings-general-new-mail-sound = Magpatugtog ng tunog
settings-general-new-mail-sound-detail = Ang tunog ng bagong mail ng desktop
settings-general-reset-cache = I-reset ang cache
settings-general-reset-cache-detail = Kapag mukhang mali o luma ang mail, o para magbakante ng espasyo sa disk. Walang nagbabago sa iyong mga mail server.
settings-general-desktop = Desktop
settings-general-start-at-login = Simulan ang Katna sa pag-log in
settings-general-start-at-login-detail = Nagsi-sync ng mail at nagpapakita ng mga notification ng bagong mail at ng icon sa system tray, nang hindi binubuksan ang window
settings-general-login-window = Buksan din ang window ng Katna Mail
settings-general-login-window-detail = Bubukas din ang window sa pag-log in
settings-general-tray = Ipakita ang Katna sa system tray
settings-general-tray-detail = May bilang ng hindi pa nabasa at isang menu
settings-general-unread-badge = Bilang ng hindi pa nabasa sa icon sa taskbar
settings-general-unread-badge-detail = Ilang mensahe sa Inbox ang hindi pa nabasa

## Settings > Inbox

settings-inbox-tabs = Mga tab ng inbox
settings-inbox-tabs-detail = Ayusin ang inbox sa mga tab, gaya ng ginagawa ng website ng iyong mail provider.
settings-inbox-tabs-show = Ipakita ang mga tab ng inbox
settings-inbox-tabs-show-detail = Kapag naka-off, iisang listahan ang ipinapakita para sa bawat account
settings-inbox-no-accounts = Magdagdag ng account para mapili ang mga tab nito.
settings-inbox-tabs-automatic = Awtomatiko: { $tabs } ({ $provider })
settings-inbox-tabs-off = Walang tab
settings-inbox-tabs-gmail = Pangunahin, Mga Promosyon, Social, Mga Update, Mga Forum
settings-inbox-tabs-focused = Naka-focus at Iba pa
settings-inbox-tabs-zoho = Inbox, Mga Newsletter at Mga Notification
settings-inbox-tabs-shown = Mga tab na ipinapakita. Mananatili sa { $tab } ang mail ng tab na io-off mo.

## Settings > Appearance

settings-appearance-reading-pane = Pane ng pagbabasa
settings-appearance-reading-pane-detail = Kung saan ipinapakita ang nakabukas na pag-uusap.
settings-appearance-pane-right = Sa kanan ng listahan
settings-appearance-pane-none = Walang hati
settings-appearance-density = Density
settings-appearance-density-default = Default
settings-appearance-density-compact = Compact
settings-appearance-scaling = Scaling
settings-appearance-scaling-detail = Pinalalaki o pinaliliit ang lahat ng nasa Katna Mail, dagdag sa sariling scale ng desktop: text, mga icon, espasyo at mga divider. Pinapanatili ng mail na ipinapadala mo ang sarili nitong laki ng font. Maaaring mahirap i-click ang mga icon sa napakaliliit na laki.
settings-appearance-theme = Tema
settings-appearance-theme-system = System
settings-appearance-theme-light = Maliwanag
settings-appearance-theme-dark = Madilim
settings-appearance-desktop-colors = Mga kulay ng desktop
settings-appearance-desktop-colors-use = Gamitin ang mga kulay ng desktop
settings-appearance-desktop-colors-use-detail = Ang color scheme at accent color ng desktop
settings-appearance-app-names = Mga pangalan ng app
settings-appearance-app-names-show = Ipakita ang mga pangalan ng app
settings-appearance-app-names-show-detail = Mga pangalan sa ilalim ng mga icon ng app sa dulong kaliwa
settings-appearance-sender-pictures = Mga larawan ng nagpadala
settings-appearance-sender-pictures-show = Ipakita ang mga logo ng kumpanya
settings-appearance-sender-pictures-show-detail = Hinahanap ayon sa domain ng nagpadala, hindi kailanman ayon sa mensahe, at itinatabi nang isang linggo
settings-appearance-important = Mga marker ng Mahalaga
settings-appearance-important-show = Ipakita ang mga marker ng Mahalaga
settings-appearance-important-show-detail = Sa tabi ng bawat mensahe sa listahan
settings-appearance-message-width = Lapad ng mensahe
settings-appearance-message-width-limit = Limitahan ang lapad ng mga mensahe
settings-appearance-message-width-limit-detail = Mas madaling basahin ang mahahabang linya sa malapad na window
settings-appearance-mail-colors = Mga kulay ng mail
settings-appearance-mail-colors-detail = Idinisenyo ang karamihan ng mail para sa puting pahina. Sa madilim na tema, pinapalitan ang mga kulay nito ng madidilim na kulay na madaling basahin; kapag naka-off, pinapanatili nito ang mga kulay ng nagpadala sa maliwanag na pahina.
settings-appearance-dark-mail = Madidilim na kulay rin para sa mail
settings-appearance-dark-mail-detail = Habang madilim lang ang tema
settings-appearance-attachment-previews = Mga preview ng attachment
settings-appearance-attachment-previews-show = Ipakita ang mga preview ng mga attachment
settings-appearance-attachment-previews-show-detail = Maliit na larawan ng nilalaman ng bawat file sa card nito

## Settings > Default apps

settings-default-apps-intro = Kung saan bumubukas ang mga attachment kapag kini-click mo ang mga ito. Palaging makakapagbukas din ang viewer ng file sa ibang app. Itinatakda ang mga default na app ng desktop sa sarili nitong mga setting.
settings-default-apps-pdf = Mga PDF file
settings-default-apps-pdf-detail = Mga pahina, may zoom.
settings-default-apps-pictures = Mga larawan
settings-default-apps-pictures-detail = Mga litrato (itinuwid), PNG, GIF, WebP, BMP, TIFF at SVG.
settings-default-apps-text = Mga text file
settings-default-apps-text-detail = Plain text, mga log, code at iba pang text.
settings-default-apps-sheets = Mga spreadsheet
settings-default-apps-sheets-detail = Excel (xlsx, xls), OpenDocument (ods) at CSV.
settings-default-apps-documents = Mga dokumento
settings-default-apps-documents-detail = Word (docx, doc), OpenDocument text (odt) at mga slide (pptx, ppt, odp).
settings-default-apps-katna = Viewer ng Katna Mail
settings-default-apps-system = Default na app ng desktop
settings-default-apps-ask = Itanong kung aling app sa bawat pagkakataon
settings-default-apps-after-saving = Pagkatapos mag-save
settings-default-apps-show-folder = Ipakita ang mga na-save na file sa folder nila
settings-default-apps-show-folder-detail = Binubuksan ang file manager na nakapili ang mga na-save na attachment

## Settings > Compose

settings-compose-send-from = Ipadala ang mga bagong mensahe mula sa
settings-compose-send-from-detail = Nagsisimula ang mga bagong mensahe sa account na ito; pumipili ng iba ang row na Mula kay. Palaging ipinapadala ang mga sagot at pagpapasa mula sa account na pinadalhan ng orihinal na mensahe.
settings-compose-send-from-current = Ang account na kinaroroonan mo
settings-compose-send-on-replies = Ipadala sa mga sagot
settings-compose-send-on-replies-detail = Ang ginagawa ng Ipadala sa isang sagot o pagpapasa. Iniaalok ng menu sa tabi ng Ipadala ang isa pa.
settings-compose-send-plain = Ipadala
settings-compose-send-archive = Ipadala at i-archive
settings-compose-signatures = Mga lagda
settings-compose-signatures-detail = Idinaragdag sa ibaba ng iyong mensahe, pagkatapos ng linyang “--”. Pumili ng iba sa window ng pag-compose.
settings-compose-untitled = Walang pamagat
settings-compose-signature-name = Pangalan, gaya ng Trabaho
settings-compose-signature-first = Aking lagda
settings-compose-signature-numbered = Lagda { $number }
settings-compose-signature-delete = I-delete
settings-compose-signature-deleted = Na-delete ang lagda
settings-compose-signature-new = Gumawa ng bago
settings-compose-no-signatures = Wala pang lagda.
settings-compose-no-signature = Walang lagda
settings-compose-for-new-mail = Para sa bagong mail
settings-compose-for-replies = Para sa mga sagot at pagpapasa
settings-compose-for-replies-detail = Sa pag-uusap kung saan nilagdaan mo ang isang mensahe, sa lagdang iyon nagsisimula ang sagot.
settings-compose-format = Format
settings-compose-plain-text = Sumulat sa plain text
settings-compose-plain-text-detail = Nagsisimula ang bagong mail nang walang formatting; maaari itong palitan sa window ng pag-compose
settings-compose-spelling = Pagbaybay
settings-compose-spell-check = Suriin ang pagbaybay habang sumusulat ako
settings-compose-spell-check-detail = Sinasalungguhitan ang mga maling baybay na salita, may mga mungkahi sa right-click
settings-compose-spell-desktop = Wika ng desktop ({ $language })
settings-compose-templates = Mga template
settings-compose-templates-detail = I-save ang mail na madalas mong isinusulat, at magsimula ng bagong mail o sagot mula rito.

## Settings > Shortcuts

settings-shortcuts-set = Set ng shortcut
settings-shortcuts-set-detail = Magsimula sa mga key ng mail app na kilala mo. Ctrl ang Cmd dito. Nananatili ang sarili mong mga pagbabago sa ibabaw ng set, at ibinabalik ng Ibalik ang mga default ang mga key ng set.
settings-shortcuts-single = Mga shortcut na iisang key
settings-shortcuts-single-detail = Mga key na walang Ctrl o Alt, gaya sa webmail: nag-a-archive ang e, gumagalaw ang j at k, naghahanap ang /. Gumagana ang mga ito sa listahan at sa nakabukas na pag-uusap, hindi kailanman habang nagta-type.
settings-shortcuts-single-use = Gamitin ang mga shortcut na iisang key
settings-shortcuts-single-use-detail = Palaging gumagana ang mga Ctrl shortcut
settings-shortcuts-how = I-click ang isang key para palitan ito, o ang + para magdagdag, pagkatapos ay pindutin ang mga bagong key. Kinakansela ng Esc.
settings-shortcuts-restore = Ibalik ang mga default
settings-shortcuts-no-key = Walang key
settings-shortcuts-press = Pumindot ng mga key…
settings-shortcuts-then = { $keys } at pagkatapos…
settings-shortcuts-moved = Ginagawa na ngayon ng { $keys } ang “{ $action }” sa halip na “{ $previous }”.
settings-shortcuts-single-off = Naka-off ang mga shortcut na iisang key, kaya gagana ang key na ito kapag na-on na ang mga ito.
settings-shortcuts-restored = Nasa mga key na ulit ng set nito ang bawat shortcut.

## Settings search: the line under a result

settings-general-language-summary = Wika ng app, mga petsa at numero
settings-general-reading-summary = Pinakabagong mensahe muna, buong header, buong pangalan ng mga tatanggap
settings-general-mark-read-summary = Kailan minamarkahang nabasa na ang nakabukas na pag-uusap: kaagad, pagkatapos ng 1 o 3 segundo, o mano-mano
settings-general-auto-advance-summary = Ano ang bubukas pagkatapos mong i-delete, i-archive o ilipat ang nakabukas na pag-uusap: ang susunod, ang nakaraan, o ang listahan
settings-general-reply-button-summary = Sumasagot sa lahat ang button na sumagot sa tabi ng bawat mensahe
settings-general-remote-images-summary = Palaging ipakita ang mga larawan ng bawat mensahe
settings-general-sending-summary = I-undo ang pagpapadala: gaano katagal naghihintay ang naipadalang mensahe, para mabawi pa ito
settings-general-offline-summary = Ilang araw ng kamakailang mail ang buong dina-download, para mabasa nang walang koneksyon
settings-general-notifications-summary = Mga notification ng bagong mail at ang tunog nito
settings-general-reset-cache-summary = I-delete ang na-download na mail, mga larawan ng nagpadala at ang search index, at i-download muli ang mga ito
settings-general-desktop-summary = Simulan ang Katna sa pag-log in, ang icon sa system tray at ang bilang ng hindi pa nabasa sa icon sa taskbar
settings-accounts-accounts-summary = Magdagdag o mag-alis ng account, o palitan ang larawan nito
settings-appearance-density-summary = Default o compact na mga linya sa listahan
settings-appearance-scaling-summary = Palakihin o paliitin ang lahat: text, mga icon, espasyo at mga divider
settings-appearance-theme-summary = System, maliwanag o madilim
settings-appearance-sender-pictures-summary = Mga logo ng kumpanya, hinahanap ayon sa domain ng nagpadala
settings-appearance-important-summary = Ang marker ng Mahalaga sa tabi ng bawat mensahe sa listahan
settings-appearance-mail-colors-summary = Madidilim na kulay para sa HTML mail sa madilim na tema, o ang mga kulay ng nagpadala nito
settings-appearance-attachment-previews-summary = Maliit na larawan ng nilalaman ng bawat attachment
settings-shortcuts-set-summary = Magsimula sa mga key ng Gmail, Inbox by Gmail, Apple Mail, Outlook o Thunderbird
settings-shortcuts-single-summary = Mga key na walang Ctrl o Alt, gaya sa webmail
settings-default-apps-pdf-summary = Kung saan bumubukas ang mga PDF attachment
settings-default-apps-pictures-summary = Kung saan bumubukas ang mga litrato at larawan
settings-default-apps-text-summary = Kung saan bumubukas ang plain text, mga log at code
settings-default-apps-sheets-summary = Kung saan bumubukas ang mga Excel, OpenDocument at CSV file
settings-default-apps-documents-summary = Kung saan bumubukas ang Word, OpenDocument text at mga slide
settings-default-apps-after-saving-summary = Ipakita ang mga na-save na attachment sa folder nila
settings-compose-send-from-summary = Ang account na pinagpapadalhan ng bagong mail: ang una, iba pa, o ang kinaroroonan mo
settings-compose-send-on-replies-summary = Ipadala, o Ipadala at i-archive ang pag-uusap, sa mga sagot at pagpapasa
settings-compose-signatures-summary = Idinaragdag sa ibaba ng iyong mensahe, pagkatapos ng linyang “--”
settings-compose-for-new-mail-summary = Ang lagdang pinagsisimulan ng bagong mail
settings-compose-for-replies-summary = Ang lagdang pinagsisimulan ng mga sagot at pagpapasa
settings-compose-format-summary = Sumulat ng bagong mail sa plain text
settings-compose-spelling-summary = Suriin ang pagbaybay habang sumusulat, at ang wika ng diksyunaryo
settings-compose-templates-summary = Malapit na: i-save ang mail na madalas mong isinusulat, at magsimula ng bagong mail o sagot mula rito
settings-feedback-crash-reports-summary = Mag-save ng mga ulat ng pag-crash sa computer na ito kapag nag-crash ang Katna Mail o ang serbisyo nito sa background
settings-feedback-saved-summary = Tingnan, kopyahin o i-delete ang mga ulat ng pag-crash na naka-save sa computer na ito
settings-feedback-help-improve-summary = Magpadala ng mga ulat ng pag-crash para makatulong ayusin ang nagkaproblema; naka-off maliban kung i-on mo
settings-experimental-blur-summary = Tumatagos ang desktop sa itaas na bar, malabo, at parang frosted glass ang mga menu
settings-search-shortcut = Keyboard shortcut
settings-search-tab = Tab ng mga setting
settings-search-none = Walang setting na tumutugma sa “{ $query }”.
settings-search-results = Mga setting na tumutugma sa “{ $query }”

## Settings: opening at login

settings-open-at-login-failed = Hindi mabago ang pagsisimula sa pag-log in: { $error }

## Settings > General > Time

settings-time = Oras
settings-clock-language = Kung paano ito isinusulat ng wika
settings-clock-12 = 12-oras, gaya ng 2:05 PM
settings-clock-24 = 24-oras, gaya ng 14:05
settings-time-summary = 12-oras o 24-oras na orasan, o kung paano ito isinusulat ng wika

## Settings > General > Default mail app, Settings > Compose > Grammar

settings-general-mail-app = Default na mail app
settings-general-mail-app-detail = Nagbubukas dito ng bagong mensahe ang mga email link sa ibang app at sa mga website.
mail-app-is-default = Ang Katna Mail ang iyong default na mail app.
mail-app-is-other = Bumubukas sa ibang app ang mga email link.
mail-app-make-default = Gawing default
mail-app-make-default-failed = Hindi mabago ang default na mail app.
settings-general-mail-app-summary = Buksan sa Katna Mail ang mga email link mula sa ibang app at website
settings-compose-grammar = Grammar
settings-compose-grammar-detail = Sinusuri sa computer na ito gamit ang Harper. English lang sa ngayon: hindi ginagalaw ang text sa ibang wika.
settings-compose-grammar-check = Suriin ang grammar
settings-compose-grammar-check-detail = Salungguhitan ang mga mali sa grammar habang sumusulat, sa English
settings-compose-suggestions = Mga mungkahi sa pagsulat
settings-compose-suggestions-detail = Natutunan sa computer na ito mula sa mail na ipinadala mo at sa mail na sinasagot mo; walang lumalabas dito. Pindutin ang Tab para tanggapin ang mungkahi, o magpatuloy lang sa pag-type.
settings-compose-suggestions-on = Magmungkahi habang nagsusulat
settings-compose-suggestions-on-detail = Ipakita nang kulay abo ang malamang na karugtong ng parirala habang nagta-type
settings-compose-grammar-summary = Salungguhitan ang mga mali sa grammar habang sumusulat, sa English
settings-compose-suggestions-summary = Ipakita nang kulay abo ang malamang na karugtong ng parirala habang nagta-type
