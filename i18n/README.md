# Translating Katna

Katna speaks 51 languages (49 translations; English (India), English (UK)
and English (US) share one text and differ only in date and number
formats). The design is in `docs/ARCHITECTURE.md` §13.10.

**Every translation except English was first drafted by AI** and has not
been checked by a native speaker yet. If you speak one of these languages,
your corrections are very welcome, even a single word.

## Where the text lives

```
i18n/
  languages.toml          the picker's list: names, flags, status
  en/katna-mail/          English, the source, one file per area:
    common.ftl              top bar, language picker, dates and sizes
    list.ftl                the mail list
    reader.ftl              the reading pane
    settings.ftl            the Settings page
    …
  en/katna-ui.ftl         shared widgets
  en/katna-daemon/        the background service: notifications, the
                          tray icon
  bn/katna-mail/          Bengali, the same files
  bn/katna-mail/whats-new.toml
                          Katna Mail's What's new highlights in Bengali
  ar/katna-mail/          Arabic
  …
```

Each language has the same files as English, holding the same messages.

The files use [Fluent](https://projectfluent.org/). A message is an id,
`=`, and the text:

```ftl
compose = লিখুন
language-tooltip = ভাষা: { $language }
ago-hours = { $count ->
    [one] { $count } ঘণ্টা আগে
   *[other] { $count } ঘণ্টা আগে
}
```

- Change only the text after `=`. Keep the id on the left as it is.
- Keep every `{ $variable }`; you may move it anywhere in the sentence.
- Plural forms use your language's
  [CLDR plural categories](https://www.unicode.org/cldr/charts/latest/supplemental/language_plural_rules.html)
  (`zero`, `one`, `two`, `few`, `many`, `other`); `*` marks the default.
  Languages without plurals (Chinese, Japanese, Korean, Thai, …) need only
  `*[other]`. Write `{ $count }` rather than a digit, so numbers show in
  your language's digits.
- A message missing from your file shows in English, so a partial file is
  fine.

What's new, shown after an update, is written in English in
`apps/katna-mail/whats-new/highlights/`, one file per highlight. Each
language translates them in `<folder>/katna-mail/whats-new.toml`, a table
per highlight named by its file (`["2026-09-27-0444-about-katna"]`) with
a `title` and a `text`; one without a table shows in English. See
`apps/katna-mail/whats-new/README.md`.

## Correcting a translation

1. Open your language's file on GitHub, for example
   [`i18n/bn/katna-mail/list.ftl`](bn/katna-mail/list.ftl), and press the pencil
   (Edit). GitHub makes a copy for you.
2. Change the text, then press **Propose changes** and open a pull
   request. Say which language you speak natively.
3. Or open an issue titled "Translation: <language>" saying where the text
   is, what it says now and what it should say.

To see your change in Katna before sending it, put the file in
`~/.local/share/katna/i18n/<folder>/` (for example
`~/.local/share/katna/i18n/bn/katna-mail/list.ftl`) and restart Katna
Mail (for `katna-daemon/` files, the service too:
`systemctl --user restart katna-daemon`). It is loaded over the built-in
text, message by message.

Once a native speaker has reviewed a whole language, its entry in
`languages.toml` changes from `status = "machine"` to `"reviewed"` with
their name in `reviewers`, and the picker stops calling it machine
translated.

## Testing

`KATNA_LANGUAGE=bn katna-mail` starts in Bengali whatever the desktop
says. Two pseudo-languages help find problems without reading the
language: `qps-ploc` shows every text accented and longer (plain English
left over was never made translatable; a cut-off label has no room for a
longer language), and `qps-plocm` also mirrors the layout, as Arabic,
Persian, Hebrew and Urdu do.

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
changes made side by side rarely touch the same lines. A new area gets a
new file (`compose.ftl`); the build picks up every `.ftl` file in the
folder. A message's id must be unique across the binary's files. The other languages are drafted in a follow-up; until then they
show the English text. `cargo test -p katna-i18n` checks that every id in
the code has an English message and that each translation's variables
match English.
