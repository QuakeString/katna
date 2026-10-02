# Katna Mail, English: the attachment viewer.
# Guide: i18n/README.md. Keep ids stable; change the text freely.
# In plural forms, write { $count } rather than the digit, so languages
# with their own digits show them.

## Attachment viewer

# Shown, under a turning arc, while a file is on its way or opening.
viewer-opening = Opening…
# Shown in place of a file the viewer cannot show.
viewer-unreadable = This attachment could not be read.
viewer-pdf-locked = This PDF is protected with a password.
viewer-pdf-unreadable = This PDF could not be read.
viewer-picture-unreadable = This picture could not be read.
viewer-sheet-unreadable = This spreadsheet could not be read.
viewer-document-unreadable = This document could not be read.
viewer-slides-unreadable = These slides could not be read.
viewer-no-preview = No preview available
# Above each slide of a presentation. $number: the slide's number.
viewer-slide = Slide { $number }
# In the pill under a PDF, before the box with the page number on show,
# which can be changed to go to another page.
viewer-page = Page
# The same box for slides, before the slide number on show.
viewer-slide-box = Slide
# After that box. $count: the PDF's number of pages.
viewer-page-count = of { $count }
viewer-go-to-page-tip = Type a page number and press Enter (Ctrl+G)
viewer-rotate-clockwise-tip = Rotate clockwise (Ctrl+R)
viewer-rotate-anticlockwise-tip = Rotate anticlockwise (Ctrl+Shift+R)
viewer-dark-pages-tip = Dark pages
viewer-light-pages-tip = Show pages as they are
viewer-fit-page-tip = Fit page
viewer-fit-picture-tip = Fit to window
viewer-fit-width-tip = Fit width
viewer-real-size-tip = Real size (1:1)
viewer-page-back-tip = Previous page
viewer-page-on-tip = Next page

## Marking up a PDF

viewer-markup-tip = Mark up
viewer-tool-select = Select text
viewer-tool-highlight = Highlight
viewer-tool-underline = Underline
viewer-tool-squiggly = Squiggle
viewer-tool-strike = Strike through
viewer-tool-pen = Pen
viewer-tool-note = Sticky note
viewer-tool-text = Text box
viewer-tool-eraser = Eraser
viewer-color-yellow = Yellow
viewer-color-green = Green
viewer-color-blue = Blue
viewer-color-pink = Pink
viewer-color-orange = Orange
viewer-color-red = Red
viewer-color-black = Black
viewer-color-purple = Purple
viewer-marks-undo-tip = Undo (Ctrl+Z)
viewer-marks-redo-tip = Redo (Ctrl+Shift+Z)
viewer-save-marked-tip = Save a copy with your marks (Ctrl+S)
viewer-reply-marked-tip = Reply with the marked copy
# Starts a new mail with only this file attached (the marked copy when
# the PDF has marks).
viewer-forward-tip = Forward the file
viewer-forward = Forward
viewer-open-with = Open with…
viewer-save = Save
# Typing a sticky note or a text box on the page.
viewer-note-placeholder = Write a note
viewer-text-placeholder = Type here
viewer-note-done = Done
viewer-note-delete = Delete
viewer-markup-protected = This PDF is protected against changes, so it can't be marked up.
viewer-marks-save-failed = The marked copy could not be saved.
# Asked when closing a PDF, or moving to another attachment, with marks
# that are not saved yet.
viewer-marks-unsaved-title = Save your marks?
viewer-marks-unsaved-text = Your marks on this PDF are not saved yet. They go into a copy; the attachment itself stays as it was.
viewer-marks-discard = Discard
viewer-marks-keep = Keep marking
viewer-marks-save = Save a copy
# The name of the copy of a PDF with marks, before ".pdf". $name: the
# attachment's name without ".pdf".
viewer-marked-name = { $name } (marked)
# The viewer's bar, opened from the attach picker: ticks the file shown.
viewer-pick = Select
viewer-picked = Selected
