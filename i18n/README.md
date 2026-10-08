# Translating Katna

Katna speaks 51 languages (49 translations; English (India), English (UK)
and English (US) share one text and differ only in date and number
formats). The design is in `docs/ARCHITECTURE.md` §13.10.

**Every translation except English was first drafted by AI** and has not
been checked by a native speaker yet. If you speak one of these languages,
your corrections are very welcome, even a single word: open a
[Translation correction](#sending-a-correction) issue, or change the file
yourself.

## Where the text lives

```
i18n/
  languages.toml          the picker's list: names, flags, folder, status
  en/                     English, the source of every other language
    katna-mail/           Katna Mail, one file per area:
      common.ftl            top bar, language picker, dates and sizes
      list.ftl              the mail list
      reader.ftl            the reading pane
      settings.ftl          the Settings page
      …
    katna-ui.ftl          shared widgets (no messages yet)
    katna-daemon/         the background service: notifications, the
                          tray icon, the file manager's menus
    katna-setup/          Katna Setup, the Windows installer
  bn/                     Bengali: the same folders and files
    katna-mail/whats-new.toml
                          Katna Mail's What's new highlights in Bengali
  ar/                     Arabic
  …
```

A language's folder is its `translation` in `languages.toml`, usually its
tag (`bn`, `pt-BR`, `zh-Hant`). Each has the same files as English, holding
the same messages. An id is unique within its binary (`katna-mail`,
`katna-daemon`, …), so which file of the binary's folder holds a message
does not matter to Katna; keep them in the same files as English all the
same. Katna Mail builds in `katna-mail/` and `katna-ui.ftl`, the service
`katna-daemon/`, Katna Setup `katna-setup/`.

`languages.toml` gives each language its `tag` (the `general.language`
setting), its own `name` and its `english` name, the `flag`, the
`translation` folder, the `formats` locale for dates and numbers,
`rtl = true` when it reads right to left, and its `status` (see
[Review](#review)).

What's new, shown once after an update, is written in English in
`apps/katna-mail/whats-new/highlights/`, one file per highlight. Each
language translates them in `<folder>/katna-mail/whats-new.toml`, a table
per highlight named by its file (`["2026-09-27-0444-about-katna"]`) with
a `title` and a `text`; one without a table shows in English. See
`apps/katna-mail/whats-new/README.md`.

The clock on the desktop panel (Katna Digital Clock for Plasma and the
GNOME Shell extension, in `integrations/`) uses gettext instead:
`integrations/po/<lang>.po` (`pt_BR`, `zh_CN`, `zh_TW` in gettext's
spelling), made from `integrations/po/katna-clock.pot`. See
`integrations/README.md`.

## Fluent, as Katna uses it

The `.ftl` files use [Fluent](https://projectfluent.org/). A message is an
id, `=`, and the text; a line starting with `#` is a comment for
translators, and `##` starts a section:

```ftl
## Language picker (top bar and Settings > General)

compose = লিখুন
# $language: the language's own name.
language-tooltip = ভাষা: { $language }
ago-hours = { $count ->
    [one] { $count } ঘণ্টা আগে
   *[other] { $count } ঘণ্টা আগে
}
```

- **Change only the text after `=`.** The id on the left stays as it is,
  in every language.
- **Variables.** Keep every `{ $variable }` that English has, and add
  none; you may move it anywhere in the sentence. The English comment
  above a message says what each one holds. A message whose variables
  differ from English's is not used: Katna shows the English one instead.
- **Plurals.** Choose the forms with your language's
  [CLDR plural categories](https://www.unicode.org/cldr/charts/latest/supplemental/language_plural_rules.html)
  (`zero`, `one`, `two`, `few`, `many`, `other`), not English's: Arabic
  has six, Russian four, English two. `*` marks the default, which every
  choice needs. Languages without plurals (Chinese, Japanese, Korean,
  Thai, …) need only `*[other]`. Write `{ $count }` rather than a digit,
  so numbers show in your language's digits.
- **Terms** (Fluent's shared `-brand = Katna`) are not used: names are
  written out in each message.

A message missing from your file shows in English, so a partial file is
fine.

### What not to translate

- Ids, variable names (`$count`, not `$compte`), and the `[one]`,
  `*[other]` keys of plural forms.
- **Katna** and the names built on it (Katna Mail, Katna Calendar, Katna
  Setup); other brands and products (Gmail, Outlook, Zoho, Gemini,
  Mistral, Dolphin, …); protocols and formats (IMAP, SMTP, CalDAV,
  CardDAV, OAuth, PDF, CSV); file names, paths, addresses, URLs and the
  names of keys (Ctrl, Alt, Shift).
- Whatever the comment above a message asks you to keep, such as the one
  `_` before the letter pressed with Alt in the tray's menu (put it before
  a letter of your translation) or a date pattern like `YYYY-MM-DD`.

### Style

- Write as your desktop's and phone's own apps do: the words people
  already know from their system, the usual form of address (`du` or
  `Sie`, `tu` or `vous`: follow GNOME and KDE), and your language's
  punctuation and quotation marks.
- Keep it about as short as English. Buttons and menu items have little
  room; `qps-ploc` (below) shows where it is tight.
- Each translated file starts with three comment lines, which stay:

  ```ftl
  # Katna Mail, Bengali (বাংলা).
  # Machine-drafted by AI; not yet reviewed by a native speaker.
  # Corrections welcome: see i18n/README.md.
  ```

### Review

Once a native speaker has reviewed a whole language (plan L.8), its entry
in `languages.toml` changes from `status = "machine"` to `"reviewed"` with
their name in `reviewers`, and the picker stops calling it
machine translated. Until then a reviewed message is a correction like
any other.

## Trying a change

`KATNA_LANGUAGE=bn katna-mail` starts in Bengali whatever the setting and
the desktop say: any `tag` from `languages.toml`. The service and Katna
Setup read it too.

To see a corrected file in Katna before sending it, put it in the
override folder, under your language's folder with the same path as in
`i18n/`:

- Linux: `~/.local/share/katna/i18n/` (`$XDG_DATA_HOME/katna/i18n` when
  that is set), for example
  `~/.local/share/katna/i18n/bn/katna-mail/list.ftl`;
- Windows: `%LOCALAPPDATA%\Katna\Data\i18n\`.

Restart Katna Mail (for `katna-daemon/` files, the service too:
`systemctl --user restart katna-daemon`). The file is loaded over the
built-in text message by message, so it may hold only the messages you
changed; `en/` works the same way for English. Katna Setup and What's new
have no override.

Two pseudo-languages find problems without reading a language:

- `KATNA_LANGUAGE=qps-ploc` shows every text accented and about 40 %
  longer, in brackets. Plain English left over was never made
  translatable; a cut-off label has no room for a longer language.
- `KATNA_LANGUAGE=qps-plocm` does the same and mirrors the layout, as
  Arabic, Persian, Hebrew and Urdu do.

`cargo test -p katna-i18n` checks that every file parses and lists the
translated messages whose variables differ from English's.

## Sending a correction

Either way, say whether you speak the language natively.

- **An issue.** Open a
  [Translation correction](https://github.com/QuakeString/katna/issues/new?template=translation-correction.yml)
  issue: the language, where in Katna the text is, what it says now, what
  it should say, and a screenshot if you have one. No files to find.
- **A pull request.** Open your language's file on GitHub, for example
  [`i18n/bn/katna-mail/list.ftl`](bn/katna-mail/list.ftl), and press the
  pencil (Edit); GitHub makes a copy for you. Change the text, press
  **Propose changes** and open the pull request. To find a message,
  search `i18n/en/` for the English text, then look for its id in the
  file of the same name in your language's folder.

Don't add an id that English doesn't have: it is never shown.

## Coverage

CI writes a coverage report to each run's summary (Actions, the run,
**Summary**): for each language and binary, how many of English's
messages it has, the ids it is missing (shown in English), stale ids
English no longer has (never shown), messages with other variables than
English's (shown in English), and the What's new highlights not yet
translated. It only reports and never fails the build. To make it
yourself:

```sh
KATNA_I18N_COVERAGE=coverage.md cargo test -p katna-i18n --test translation_coverage
```

The [Translation correction](../.github/ISSUE_TEMPLATE/translation-correction.yml)
form lists every language of `languages.toml`. After adding or renaming
one, run `KATNA_I18N_WRITE_TEMPLATE=1 cargo test -p katna-i18n
issue_template`; a test fails until the list matches.

## For developers

Every text the user sees goes through `katna_i18n::tr!`:

```rust
use katna_i18n::tr;
let label = tr!("compose");
let unread = tr!("unread-count", count = n);
```

Add the English message to the area's file in `i18n/en/<binary>/` (for
example `i18n/en/katna-mail/settings.ftl`) in the same pull request, next
to the messages it belongs with rather than at the end of the file, so
changes made side by side rarely touch the same lines. A comment above it
tells translators what each variable holds. A new area gets a new file
(`compose.ftl`); the build picks up every `.ftl` file in the folder. A
message's id must be unique across the binary's files. The other
languages are drafted in a follow-up; until then they show the English
text. `cargo test -p katna-i18n` checks that every id in the code has an
English message and that English ids are unique. Dates and numbers go
through `katna_i18n::format`, never `strftime` or `{}`.
