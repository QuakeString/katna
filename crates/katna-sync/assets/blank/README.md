# Blank Office files

What New in Files makes on a OneDrive (`crates/katna-sync/src/onedrive/browse.rs`):
an empty Word document, Excel workbook and PowerPoint presentation, saved
by LibreOffice. Graph has no call that makes a blank Office file, and a file
of no bytes doesn't open in Office for the web, so Katna uploads these.
