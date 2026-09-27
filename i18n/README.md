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
  en/katna-mail.ftl       English, the source
  en/katna-ui.ftl
  bn/katna-mail.ftl       Bengali
  ar/katna-mail.ftl       Arabic
  …
```

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

## Correcting a translation

1. Open your language's file on GitHub, for example
   [`i18n/bn/katna-mail.ftl`](bn/katna-mail.ftl), and press the pencil
   (Edit). GitHub makes a copy for you.
2. Change the text, then press **Propose changes** and open a pull
   request. Say which language you speak natively.
3. Or open an issue titled "Translation: <language>" saying where the text
   is, what it says now and what it should say.

To see your change in Katna before sending it, put the file in
`~/.local/share/katna/i18n/<folder>/` (for example
`~/.local/share/katna/i18n/bn/katna-mail.ftl`) and restart Katna Mail. It
is loaded over the built-in text, message by message.

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

Add the English message to `i18n/en/<binary>.ftl` in the same pull
request. The other languages are drafted in a follow-up; until then they
show the English text. `cargo test -p katna-i18n` checks that every id in
the code has an English message and that each translation's variables
match English.
