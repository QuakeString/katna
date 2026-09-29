# Katna Mail, Amharic (አማርኛ): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = እውቂያዎች
contacts-frequent = ተደጋጋሚ
contacts-labels = መሰየሚያዎች
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
