# Katna Mail, Amharic (አማርኛ): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = እውቂያዎች
contacts-frequent = ተደጋጋሚ
contacts-other = ሌሎች እውቂያዎች
contacts-other-about = ከGmail ኢሜይል የላካችኋቸው ግን ያላስቀመጣችኋቸው ሰዎች
contacts-other-email = ኢሜይል ላክ
contacts-other-empty = ሌሎች እውቂያዎች የሉም። ከGmail ኢሜይል የሚልኩላቸው ግን የማያስቀምጧቸው ሰዎች እዚህ ይታያሉ።
contacts-other-allow = ሌሎች እውቂያዎችን ለማየት እንደገና ወደ የGmail መለያዎ ይግቡ እና Katna እንዲያያቸው ይፍቀዱ።
contacts-labels = መሰየሚያዎች
contacts-label-options = የመሰየሚያ አማራጮች
contacts-label-rename = መሰየሚያን ዳግም ሰይም
contacts-label-email = ለሁሉም ኢሜይል ላክ
contacts-label-delete = መሰየሚያን ሰርዝ
contacts-label-new = አዲስ መሰየሚያ
contacts-label-name = የመሰየሚያ ስም
contacts-label-button = መሰየሚያ
contacts-label-menu = በመሰየሚያ ሰይም:
contacts-label-added = ወደ { $name } ታክሏል
contacts-label-removed = ከ{ $name } ተወግዷል
contacts-label-renamed = መሰየሚያው ወደ { $name } ተቀይሯል
contacts-label-deleted = መሰየሚያ { $name } ተሰርዟል
contacts-label-no-email = በዚህ መሰየሚያ ውስጥ የኢሜይል አድራሻ ያለው ማንም የለም
contacts-manage = አስተካክል እና አስተዳድር
contacts-merge = አዋህድ እና አስተካክል
contacts-merge-about = { $count ->
   *[other] { $count } ጥቆማ፦ አንድ ሰው የሚመስሉ እውቂያዎች
}
contacts-merge-none = ምንም የተባዛ የለም። ተመሳሳይ ስም ወይም ስልክ ቁጥር ያላቸው እውቂያዎች እዚህ ይታያሉ።
contacts-merge-count = { $count ->
    [one] { $count } እውቂያ
   *[other] { $count } እውቂያዎች
}
contacts-merge-all = ሁሉንም አዋህድ
contacts-merge-button = አዋህድ
contacts-merge-dismiss = አሰናብት
contacts-merged = { $count ->
    [1] እውቂያዎች ተዋህደዋል
   *[other] { $count } ውህደቶች ተጠናቅቀዋል
}
contacts-import = አስመጣ
contacts-export = ወደ ውጭ ላክ
contacts-import-title = እውቂያዎችን ከ vCard ፋይል አስመጣ
contacts-imported = { $count ->
    [one] { $count } እውቂያ ወደ { $place } ተመጥቷል
   *[other] { $count } እውቂያዎች ወደ { $place } ተመጥተዋል
}
contacts-imported-some = { $count ->
    [one] { $count } እውቂያ ወደ { $place } ተመጥቷል፤ ቀድሞ የተቀመጡ { $skipped } ተዘልለዋል
   *[other] { $count } እውቂያዎች ወደ { $place } ተመጥተዋል፤ ቀድሞ የተቀመጡ { $skipped } ተዘልለዋል
}
contacts-import-none = በ{ $name } ውስጥ ምንም እውቂያ አልተገኘም
contacts-import-all-saved = በ{ $name } ውስጥ ያሉ ሁሉም ሰዎች አስቀድመው ተቀምጠዋል
contacts-import-failed = { $name }ን ማንበብ አልተቻለም፦ { $error }
contacts-exported = { $count ->
    [one] { $count } እውቂያ ወደ { $path } ተልኳል
   *[other] { $count } እውቂያዎች ወደ { $path } ተልከዋል
}
contacts-export-none = ወደ ውጭ የሚላክ እውቂያ የለም
contacts-export-failed = እውቂያዎችን ወደ ውጭ መላክ አልተቻለም፦ { $error }
contacts-create = እውቂያ ፍጠር

## Search and the list

contacts-search = እውቂያዎችን ፈልግ
contacts-loading = እውቂያዎችን በመጫን ላይ…
contacts-empty = እስካሁን የተቀመጡ እውቂያዎች የሉም። በGmail፣ በOutlook ወይም በደብዳቤ አገልግሎትዎ ያስቀመጧቸው እውቂያዎች እዚህ ይታያሉ።
contacts-empty-no-books = ከመለያዎችዎ የመጡ እውቂያዎች ከተመሳሰሉ በኋላ እዚህ ይታያሉ።
contacts-none-found = ከፍለጋዎ ጋር የሚዛመድ እውቂያ የለም።
contacts-starred = { $count ->
    [one] ኮከብ የተደረገበት እውቂያ ({ $count })
   *[other] ኮከብ የተደረገባቸው እውቂያዎች ({ $count })
}
contacts-count = እውቂያዎች ({ $count })
contacts-col-name = ስም
contacts-col-email = ኢሜይል
contacts-col-phone = ስልክ ቁጥር
contacts-col-job = የሥራ ማዕረግ እና ኩባንያ
contacts-col-labels = መሰየሚያዎች

## Asking to allow contacts, for accounts signed in before Katna read them

contacts-allow = Katna የ{ $address } እውቂያዎችን እንዲያነብ ፍቀድ።
contacts-allow-many = { $more ->
    [one] Katna የ{ $address } እና የሌላ { $more } መለያ እውቂያዎችን እንዲያነብ ፍቀድ።
   *[other] Katna የ{ $address } እና የሌሎች { $more } መለያዎች እውቂያዎችን እንዲያነብ ፍቀድ።
}
contacts-allow-button = ፍቀድ

## A contact's page

contacts-back = ወደ እውቂያዎች ተመለስ
contacts-edit = አርትዕ
contacts-delete = ሰርዝ
contacts-deleted = { $name } ተሰርዟል
contacts-added = { $name } ወደ እውቂያዎች ታክሏል
contacts-find-mail = ደብዳቤ
contacts-details = የእውቂያ ዝርዝሮች
contacts-saved-in = የተቀመጠው በ
contacts-notes = ማስታወሻዎች
contacts-birthday = የልደት ቀን
contacts-nickname = ቅጽል ስም
contacts-this-computer = ይህ ኮምፒውተር
contacts-kind-home = ቤት
contacts-kind-work = ሥራ
contacts-kind-mobile = ሞባይል
contacts-kind-other = ሌላ
contacts-source-google = የGoogle እውቂያዎች
contacts-source-microsoft = የOutlook እውቂያዎች
contacts-source-carddav = CardDAV

## Creating and changing a contact

contacts-edit-new-title = እውቂያ ፍጠር
contacts-edit-title = እውቂያ አርትዕ
contacts-edit-save = አስቀምጥ
contacts-edit-saving = በማስቀመጥ ላይ…
contacts-edit-cancel = ይቅር
contacts-saved = እውቂያ ተቀምጧል
contacts-edit-save-to = አስቀምጥ በ
contacts-edit-changes-go-to = ለውጦች በ{ $place } ውስጥ ይቀመጣሉ።
contacts-edit-given = የመጀመሪያ ስም
contacts-edit-family = የአባት ስም
contacts-edit-company = ኩባንያ
contacts-edit-job = የሥራ ማዕረግ
contacts-edit-email = ኢሜይል
contacts-edit-phone = ስልክ
contacts-edit-with-kind = { $field } ({ $kind })
contacts-edit-add-email = ኢሜይል ጨምር
contacts-edit-add-phone = ስልክ ጨምር
contacts-edit-street = የመንገድ አድራሻ
contacts-edit-city = ከተማ
contacts-edit-postcode = የፖስታ ኮድ
contacts-edit-country = ሀገር
contacts-edit-birthday = የልደት ቀን (YYYY-MM-DD)
contacts-edit-empty = መጀመሪያ ስም፣ ኢሜይል ወይም ስልክ ቁጥር ያክሉ።
