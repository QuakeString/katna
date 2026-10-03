// SPDX-License-Identifier: GPL-3.0-or-later

use super::*;

/// A 1×1 PNG.
const PNG: &[u8] = &[
    0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a, 0, 0, 0, 0x0d, b'I', b'H', b'D', b'R', 0, 0, 0,
    1, 0, 0, 0, 1, 8, 6, 0, 0, 0,
];

fn write(path: &Path, content: &[u8]) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, content).unwrap();
}

#[test]
fn thunderbird_identities_inline_and_from_a_file() {
    let home = tempfile::tempdir().unwrap();
    let home = home.path();
    let base = home.join(".thunderbird");
    write(
        &base.join("profiles.ini"),
        b"[General]\nStartWithLastProfile=1\n\n[Profile0]\nName=default\nIsRelative=1\nPath=abc.default\n",
    );
    let profile = base.join("abc.default");
    write(&profile.join("logo.png"), PNG);
    write(
        &profile.join("sig.html"),
        b"<p><img src=\"logo.png\"> <b>Ada</b> &middot; <img src=\"https://x.test/l.png\"></p>",
    );
    let prefs = format!(
        "// Mozilla User Preferences\n\
         user_pref(\"mail.identity.id1.fullName\", \"Ada Lovelace\");\n\
         user_pref(\"mail.identity.id1.useremail\", \"ada@x.test\");\n\
         user_pref(\"mail.identity.id1.htmlSigFormat\", true);\n\
         user_pref(\"mail.identity.id1.htmlSigText\", \"<b>Ada</b>\\n<i>\\\"Analyst\\\" \\u00e9</i>\");\n\
         user_pref(\"mail.identity.id2.useremail\", \"plain@x.test\");\n\
         user_pref(\"mail.identity.id2.htmlSigText\", \"Ada\\nAnalyst\");\n\
         user_pref(\"mail.identity.id3.useremail\", \"none@x.test\");\n\
         user_pref(\"mail.identity.id10.useremail\", \"file@x.test\");\n\
         user_pref(\"mail.identity.id10.attach_signature\", true);\n\
         user_pref(\"mail.identity.id10.sig_file\", \"{}\");\n",
        profile.join("sig.html").display()
    );
    write(&profile.join("prefs.js"), prefs.as_bytes());
    let found = find(home, &home.join(".config"), &home.join(".local/share"));
    assert_eq!(found.len(), 3, "{found:#?}");
    assert_eq!(found[0].source, Source::Thunderbird);
    assert_eq!(found[0].name, "ada@x.test");
    assert_eq!(found[0].html, "<b>Ada</b>\n<i>\"Analyst\" é</i>");
    assert_eq!(found[1].name, "plain@x.test");
    assert_eq!(
        (found[1].html.as_str(), found[1].text.as_str()),
        ("", "Ada\nAnalyst")
    );
    // The file's picture on this computer is inside; the web one waits
    // for the import.
    assert_eq!(found[2].name, "file@x.test");
    assert!(
        found[2]
            .html
            .contains("src=\"data:image/png;base64,iVBORw0KGgo"),
        "{}",
        found[2].html
    );
    assert!(found[2].html.contains("src=\"https://x.test/l.png\""));
}

#[test]
fn evolution_signatures_but_never_scripts() {
    let home = tempfile::tempdir().unwrap();
    let home = home.path();
    let (config, data) = (home.join(".config"), home.join(".local/share"));
    let sources = config.join("evolution/sources");
    write(
        &sources.join("1a.source"),
        b"\n[Data Source]\nDisplayName=Work\nEnabled=true\nParent=\n\n[Mail Signature]\nMimeType=text/html\n",
    );
    write(
        &data.join("evolution/signatures/1a"),
        b"<table><tr><td><b>Ada</b></td></tr></table>",
    );
    write(
        &sources.join("2b.source"),
        b"[Data Source]\nDisplayName=Home\n\n[Mail Signature]\nMimeType=text/plain\n",
    );
    write(&data.join("evolution/signatures/2b"), b"Ada\n");
    // A script: a link to it.
    write(
        &sources.join("3c.source"),
        b"[Data Source]\nDisplayName=Fortune\n\n[Mail Signature]\nMimeType=text/plain\n",
    );
    write(&home.join("fortune.sh"), b"#!/bin/sh\necho hi\n");
    std::os::unix::fs::symlink(
        home.join("fortune.sh"),
        data.join("evolution/signatures/3c"),
    )
    .unwrap();
    // An account, not a signature.
    write(
        &sources.join("4d.source"),
        b"[Data Source]\nDisplayName=ada@x.test\n\n[Mail Account]\nBackendName=imapx\n",
    );
    let found = find(home, &config, &data);
    let names: Vec<(&str, Source)> = found.iter().map(|f| (f.name.as_str(), f.source)).collect();
    assert_eq!(
        names,
        [("Work", Source::Evolution), ("Home", Source::Evolution)]
    );
    assert!(found[0].html.starts_with("<table>"));
    assert_eq!(found[1].text, "Ada");
}

#[test]
fn kmail_identities_but_never_commands() {
    let home = tempfile::tempdir().unwrap();
    let home = home.path();
    let config = home.join(".config");
    let images = home.join(".local/share/kmail2/sig-images");
    write(&images.join("logo.png"), PNG);
    let file = format!(
        "[Identity #0]\n\
         Email Address=ada@x.test\n\
         Identity=Work\n\
         Image Location={images}\n\
         Inline Signature=<p><img src=\\\"logo.png\\\">\\s<b>Ada</b></p>\\n<p>Analyst</p>\n\
         Inline Signature Is Html=true\n\
         Name=Ada Lovelace\n\
         Signature Type=inline\n\
         \n\
         [Identity #1]\n\
         Email Address=ada@home.test\n\
         Identity=\n\
         Name=Ada\n\
         Inline Signature=Ada\\nLondon\n\
         Signature Type=inline\n\
         \n\
         [Identity #2]\n\
         Identity=Fortune\n\
         Signature Command=fortune\n\
         Signature Type=command\n\
         \n\
         [Identity #3]\n\
         Identity=Off\n\
         Inline Signature=Hidden\n\
         Signature Type=disabled\n",
        images = images.display()
    );
    write(&config.join("emailidentities"), file.as_bytes());
    let found = find(home, &config, &home.join(".local/share"));
    assert_eq!(found.len(), 2, "{found:#?}");
    assert_eq!(found[0].name, "Work");
    assert!(
        found[0]
            .html
            .starts_with("<p><img src=\"data:image/png;base64,"),
        "{}",
        found[0].html
    );
    assert!(found[0].html.ends_with("<b>Ada</b></p>\n<p>Analyst</p>"));
    assert_eq!(found[1].name, "ada@home.test");
    assert_eq!(found[1].text, "Ada\nLondon");
}

#[test]
fn nothing_found_in_an_empty_home() {
    let home = tempfile::tempdir().unwrap();
    let home = home.path();
    assert!(find(home, &home.join(".config"), &home.join(".local/share")).is_empty());
}

#[test]
fn local_pictures_only() {
    let dir = tempfile::tempdir().unwrap();
    write(&dir.path().join("a b.png"), PNG);
    write(&dir.path().join("note.txt"), b"not a picture");
    let path = dir.path().join("a b.png");
    let html = format!(
        "<img src='file://{}'><img data-src=\"x\" src=\"a%20b.png\"><img src=\"note.txt\">\
         <img src=\"cid:x\"><a href=\"a b.png\">a</a>",
        path.display().to_string().replace(' ', "%20")
    );
    let out = inline_local_pictures(&html, Some(dir.path()));
    assert_eq!(out.matches("data:image/png;base64,").count(), 2, "{out}");
    assert!(out.contains("src=\"note.txt\"") && out.contains("src=\"cid:x\""));
    assert!(out.contains("data-src=\"x\"") && out.contains("href=\"a b.png\""));
}
