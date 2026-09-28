# Katna Mail, Khmer (ខ្មែរ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = បិទ
reader-back = ថយក្រោយ
reader-mark-unread = សម្គាល់ថាមិនទាន់អាន
reader-move-to = ផ្លាស់ទីទៅ
reader-more = ច្រើនទៀត
reader-original-colors = បង្ហាញពណ៌ដើម
reader-dark-colors = បង្ហាញជាពណ៌ងងឹត
reader-print-all = បោះពុម្ពទាំងអស់
reader-new-window = ក្នុងបង្អួចថ្មី
reader-position = { $position } នៃ { $total }
reader-newer = ថ្មីជាង
reader-older = ចាស់ជាង

## Reading pane: the conversation

reader-removed = ការសន្ទនានេះត្រូវបានដកចេញ។
reader-no-subject = (គ្មានប្រធានបទ)
reader-collapse-all = បង្រួមទាំងអស់
reader-expand-all = ពង្រីកទាំងអស់
reader-unknown-sender = (មិនស្គាល់អ្នកផ្ញើ)
reader-date-ago = { $date } ({ $ago })
reader-me = ខ្ញុំ
reader-to = ទៅ { $names }
reader-to-label = ទៅ
reader-tick-delivered = បានបញ្ជូន { $when }
reader-tick-no-bounce = បានផ្ញើ { $when } ហើយគ្មានការជូនដំណឹងបរាជ័យត្រឡប់មកវិញ ដូច្នេះវាប្រហែលជាបានទៅដល់
reader-tick-bounced = មិនបានបញ្ជូន៖ ត្រឡប់មកវិញ { $when }
reader-tick-read = បានអាន { $when } (បង្កាន់ដៃអាន)
reader-tick-opened = បានបើក លើកចុងក្រោយ { $when } (ការតាមដានការបើក)
reader-starred = មានផ្កាយ
reader-not-starred = គ្មានផ្កាយ
reader-too-long = សារនេះវែងពេក មិនអាចបង្ហាញទាំងស្រុងបានទេ។
reader-encrypted-images = រូបភាពពីបណ្ដាញមិនដែលត្រូវបានផ្ទុកក្នុងសំបុត្រដែលបានអ៊ិនគ្រីបទេ។
reader-window-failed = មិនអាចបើកបង្អួចថ្មីបានទេ។

## Reading pane: message details (opened from "to me")

reader-details-from = ពី៖
reader-details-to = ទៅ៖
reader-details-cc = ចម្លងជូន៖
reader-details-date = កាលបរិច្ឆេទ៖
reader-details-subject = ប្រធានបទ៖

## Reading pane: downloading a message

reader-downloading = កំពុងទាញយកសារនេះពីម៉ាស៊ីនមេ…
reader-download-failed = មិនអាចទាញយកសារនេះបានទេ។
reader-try-again = ព្យាយាមម្ដងទៀត

## Reply row

reply-reply = ឆ្លើយតប
reply-reply-all = ឆ្លើយតបទាំងអស់
reply-forward = បញ្ជូនបន្ត

## Encrypted and signed mail

security-decrypting = កំពុងឌិគ្រីប…
security-checking = កំពុងពិនិត្យហត្ថលេខា…
security-partly-encrypted = មានតែផ្នែកខ្លះនៃសារនេះប៉ុណ្ណោះដែលត្រូវបានអ៊ិនគ្រីប។ ផ្នែកដែលនៅសល់ត្រូវបានបន្ថែមនៅក្រៅការការពារ ហើយអាចមកពីនរណាក៏បាន។
security-partly-signed = មានតែផ្នែកខ្លះនៃសារនេះប៉ុណ្ណោះដែលមានហត្ថលេខា។ ផ្នែកដែលនៅសល់ត្រូវបានបន្ថែមនៅក្រៅការការពារ ហើយអាចមកពីនរណាក៏បាន។
security-encrypted = សារដែលបានអ៊ិនគ្រីប
security-encrypted-smime = សារដែលបានអ៊ិនគ្រីប (S/MIME)
security-no-key = មិនអាចឌិគ្រីបសារនេះបានទេ៖ វាត្រូវបានអ៊ិនគ្រីបសម្រាប់សោដែលអ្នកមិនមាន។
security-cancelled = ការឌិគ្រីបត្រូវបានបោះបង់។
security-damaged = មិនអាចឌិគ្រីបសារនេះបានទេ៖ ទិន្នន័យដែលបានអ៊ិនគ្រីបខូច ឬត្រូវបានកែប្រែ។
security-decrypt-unavailable = មិនអាចឌិគ្រីបសារនេះបានទេ៖ ដំឡើង { $tool } ដើម្បីអានសំបុត្រដែលបានអ៊ិនគ្រីប។
security-decrypt-failed = មិនអាចឌិគ្រីបសារនេះបានទេ៖ { $reason }
security-unknown-signer = អ្នកចុះហត្ថលេខាដែលមិនស្គាល់
security-signed-verified = ចុះហត្ថលេខាដោយ { $signer } · បានផ្ទៀងផ្ទាត់
security-signed-not-sender = ចុះហត្ថលេខាដោយ { $signer } ដែលមិនមែនជាអ្នកផ្ញើ
security-signed-untrusted = ចុះហត្ថលេខាដោយ { $signer } ដោយប្រើសោដែលអ្នកបានសម្គាល់ថាមិនគួរទុកចិត្ត
security-signed-unverified = ចុះហត្ថលេខាដោយ { $signer } · សោមិនទាន់បានផ្ទៀងផ្ទាត់
security-bad-signature = ហត្ថលេខាមិនត្រឹមត្រូវ៖ សារនេះត្រូវបានកែប្រែបន្ទាប់ពីចុះហត្ថលេខា ឬហត្ថលេខាត្រូវបានក្លែងបន្លំ។
security-signature-expired = ចុះហត្ថលេខាដោយ { $signer } · ហត្ថលេខាបានផុតកំណត់
security-key-expired = ចុះហត្ថលេខាដោយ { $signer } · សោបានផុតកំណត់តាំងពីពេលនោះមក
security-key-revoked = ចុះហត្ថលេខាដោយ { $signer } ដោយប្រើសោដែលត្រូវបានដកហូត
security-missing-key = ចុះហត្ថលេខាដោយសោដែលអ្នកមិនមាន ដូច្នេះមិនអាចពិនិត្យបានទេ
security-missing-key-id = ចុះហត្ថលេខាដោយសោដែលអ្នកមិនមាន ({ $key }) ដូច្នេះមិនអាចពិនិត្យបានទេ
security-signature-unavailable = មានហត្ថលេខា។ ដំឡើង { $tool } ដើម្បីពិនិត្យហត្ថលេខា
security-signature-error = មិនអាចពិនិត្យហត្ថលេខាបានទេ។
tracking-opened = { $who } បានបើកវា { $count } ដង លើកចុងក្រោយ { $when }
tracking-opens-clicks = { $who } បានបើកវា { $opens } ដង ហើយចុចតំណ { $clicks } ដង លើកចុងក្រោយ { $when }
tracking-clicked = { $who } បានចុចតំណ { $clicks } ដង លើកចុងក្រោយ { $when }
tracking-maybe-opened = { $who } ប្រហែលជាបានបើកវា (Apple Mail ផ្ទុករូបភាពដើម្បីឯកជនភាព)
tracking-not-opened = { $who } មិនទាន់បានបើកវានៅឡើយទេ
tracking-receipt = { $who } បានផ្ញើបង្កាន់ដៃអាន
tracking-receipt-displayed = បង្កាន់ដៃអាន៖ { $who } បានបើកសាររបស់អ្នក
tracking-receipt-other = បង្កាន់ដៃអាន៖ { $who } បានលុប ឬដោះស្រាយសាររបស់អ្នក ដោយមិនបានបើកវា

## Remote images and pictures

remote-hidden = រូបភាពក្នុងសារនេះត្រូវបានលាក់។
remote-show = បង្ហាញរូបភាព
remote-always-show = បង្ហាញជានិច្ចពីអ្នកផ្ញើនេះ
remote-picture-use = ប្រើ
remote-picture-too-big = ជ្រើសរើសរូបភាពដែលមានទំហំ 8 MB ឬតិចជាងនេះ។
remote-picture-type = ជ្រើសរើសរូបភាព PNG, JPEG, GIF, WebP ឬ SVG។
remote-picture-read-failed = មិនអាចអានរូបភាពបានទេ៖ { $error }
remote-picture-keep-failed = មិនអាចរក្សាទុករូបភាពបានទេ៖ { $error }
remote-picture-remove-failed = មិនអាចដករូបភាពចេញបានទេ៖ { $error }

## Attachments

attachment-count = ឯកសារភ្ជាប់ { $count }
attachment-save = រក្សាទុក
attachment-save-all = រក្សាទុកទាំងអស់
attachment-save-all-tooltip = រក្សាទុកឯកសារភ្ជាប់ទាំងអស់ទៅក្នុងថតមួយ
attachment-save-here = រក្សាទុកនៅទីនេះ
attachment-not-downloaded = សារនេះមិនត្រូវបានទាញយកទេ។
attachment-not-found = រកមិនឃើញឯកសារភ្ជាប់នេះក្នុងសារទេ។
attachment-read-failed = មិនអាចអាន { $name } បានទេ
attachment-numbered = ឯកសារភ្ជាប់ { $number }
attachment-saved-all = បានរក្សាទុកឯកសារ { $count } ទៅ { $place }
attachment-saved-some = បានរក្សាទុកឯកសារ { $saved } ក្នុងចំណោម { $total } ទៅ { $place }។ មិនអាចរក្សាទុក { $failed } បានទេ
attachment-saved-to = បានរក្សាទុកទៅ { $path }
attachment-save-failed = មិនអាចរក្សាទុក { $name } បានទេ៖ { $error }
attachment-open-failed = មិនអាចបើក { $name } បានទេ៖ { $error }
attachment-risky = ឯកសារនេះអាចដំណើរការកម្មវិធីមួយ ដូច្នេះ Katna មិនបើកវាទេ។ សូមរក្សាទុកវាជំនួសវិញ។
attachment-encrypted-open = ឯកសារនេះត្រូវបានអ៊ិនគ្រីបមក។ រក្សាទុកវា ដើម្បីបើកនៅកន្លែងផ្សេង។

## Printing

print-failed = មិនអាចបោះពុម្ពបានទេ៖ { $error }
print-no-font = រកមិនឃើញពុម្ពអក្សរ
print-opened-as-pdf = បានបើកជា PDF ដើម្បីបោះពុម្ពពីទីនោះ។
print-preview-title = មើលការបោះពុម្ពជាមុន
print-preview-laying-out = កំពុងរៀបចំទំព័រ…
print-preview-pages = { $count } ទំព័រ
print-preview-more = និង { $count } ទំព័រទៀត
print-preview-failed = មិនអាចបង្ហាញទំព័របានទេ
print-preview-paper = ក្រដាស
print-preview-a4 = A4
print-preview-letter = Letter
print-preview-layout = ប្លង់
print-preview-as-shown = ដូចដែលបង្ហាញ
print-preview-simple = អត្ថបទសាមញ្ញ
print-preview-backgrounds = ផ្ទៃខាងក្រោយ
print-preview-cancel = បោះបង់
print-preview-print = បោះពុម្ព
print-not-downloaded = (មិនទាន់បានទាញយក។)
print-encrypted = (បានអ៊ិនគ្រីប។ បើកវាក្នុង Katna Mail ដើម្បីបោះពុម្ពអត្ថបទរបស់វា។)
print-to = ទៅ៖ { $addresses }
print-cc = ចម្លងជូន៖ { $addresses }

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = បើកសារនេះ ដើម្បីមើលឯកសារភ្ជាប់របស់វា។
text-copy = ចម្លង
text-select-all = ជ្រើសរើសទាំងអស់
