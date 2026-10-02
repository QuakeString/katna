// SPDX-License-Identifier: GPL-3.0-or-later

//! Finds the quote, signature and forward in parsed HTML mail, for
//! [`super::trimmed`].
//!
//! Clients mark them: Gmail's `gmail_quote` and `gmail_signature`, Apple
//! Mail's and Thunderbird's `<blockquote type="cite">`, Outlook's
//! `divRplyFwdMsg` or a From / Sent / Subject block under a border, and so
//! on. Where there is no mark, the text gives it away: an "On … wrote:"
//! line, a `-- ` line, "Begin forwarded message:". The first quote or
//! forward decides: when anything the sender wrote follows it, the mail
//! is a reply written between quoted parts and nothing is cut.

use std::collections::HashSet;

use super::css;
use super::dom::{DOCUMENT, Data, Dom, Node, NodeId};
use crate::trim::{Field, Headers, Marker, attribution, header_line, marker, mobile_signature};

/// Elements whose text is never shown.
const INVISIBLE: &[&str] = &["head", "script", "style", "template", "title"];

/// Elements that put their text on lines of its own.
const LINE_TAGS: &[&str] = &[
    "address",
    "blockquote",
    "dd",
    "div",
    "dl",
    "dt",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "hr",
    "li",
    "ol",
    "p",
    "pre",
    "table",
    "tr",
    "ul",
];

/// Past this many elements deep, text is not read and nothing counts as
/// blank (see `build::MAX_DEPTH`).
const MAX_DEPTH: usize = 320;
/// A signature found by its `-- ` line is at most this many lines, with
/// what comes after it.
const MAX_SIGNATURE_LINES: usize = 15;

/// The parts of a message to cut, as the nodes that hold them.
#[derive(Default)]
pub(super) struct Cuts {
    pub quoted: Vec<NodeId>,
    pub signature: Vec<NodeId>,
    pub forward: Option<ForwardCut>,
}

pub(super) struct ForwardCut {
    /// Everything cut: the forwarded mail with its header block.
    pub roots: Vec<NodeId>,
    /// The forwarded mail's body.
    pub body: Vec<NodeId>,
    pub from: Option<String>,
    pub date: Option<String>,
    pub subject: Option<String>,
}

pub(super) fn find(dom: &Dom) -> Cuts {
    let mut cuts = Cuts::default();
    match quote_or_forward(dom) {
        Some(Found::Quote(roots)) => cuts.quoted = roots,
        Some(Found::Forward(f)) => cuts.forward = Some(f),
        None => {}
    }
    let mut cut: HashSet<NodeId> = cuts.quoted.iter().copied().collect();
    if let Some(f) = &cuts.forward {
        cut.extend(&f.roots);
    }
    cuts.signature = signature(dom, &cut).unwrap_or_default();
    cuts
}

enum Found {
    Quote(Vec<NodeId>),
    Forward(ForwardCut),
}

/// The first quote or forward in the message, if it ends the message.
fn quote_or_forward(dom: &Dom) -> Option<Found> {
    let found = preorder(dom, &HashSet::new(), |id| candidate(dom, id))?;
    let roots = match &found {
        Found::Quote(roots) => roots,
        Found::Forward(f) => &f.roots,
    };
    let more = rest_is_quote(dom, *roots.last()?)?;
    Some(match found {
        Found::Quote(mut roots) => {
            roots.extend(more);
            Found::Quote(roots)
        }
        Found::Forward(mut f) => {
            f.roots.extend(more);
            Found::Forward(f)
        }
    })
}

/// A quote or forward that starts at `id`.
fn candidate(dom: &Dom, id: NodeId) -> Option<Found> {
    let node = &dom.nodes[id];
    if has_class(node, "gmail_quote") {
        return Some(if first_line(dom, id).is_some_and(|l| is_forward(&l)) {
            Found::Forward(forward(dom, id, id))
        } else {
            Found::Quote(vec![id])
        });
    }
    if cite(node) {
        let start = lead_in(dom, id);
        let forwarded = start.is_some_and(|s| first_line(dom, s).is_some_and(|l| is_forward(&l)))
            || first_line(dom, id).is_some_and(|l| is_forward(&l));
        let start = start.unwrap_or(id);
        return Some(if forwarded {
            Found::Forward(forward(dom, start, id))
        } else {
            Found::Quote(siblings_between(dom, start, id))
        });
    }
    if has_class(node, "moz-forward-container") {
        return Some(Found::Forward(forward(dom, id, id)));
    }
    if id_is(node, "divRplyFwdMsg") {
        return Some(Found::Quote(to_end_of_parent(dom, id)));
    }
    if ["yahoo_quoted", "zmail_extra", "protonmail_quote"]
        .iter()
        .any(|c| has_class(node, c))
    {
        return Some(Found::Quote(vec![id]));
    }
    if matches!(node.tag(), "div" | "p")
        && (border_top(node) || node.parent.is_some_and(|p| border_top(&dom.nodes[p])))
        && outlook_header(dom, id)
    {
        return Some(Found::Quote(to_end_of_parent(dom, climb(dom, id))));
    }
    None
}

/// `<blockquote type="cite">`, as Apple Mail and Thunderbird quote.
fn cite(node: &Node) -> bool {
    node.tag() == "blockquote"
        && node
            .attr("type")
            .is_some_and(|t| t.trim().eq_ignore_ascii_case("cite"))
}

/// A mark that a quote goes on: more of the same quote when it follows one.
fn quote_mark(node: &Node) -> bool {
    cite(node)
        || ["gmail_quote", "yahoo_quoted", "protonmail_quote"]
            .iter()
            .any(|c| has_class(node, c))
}

fn signature_mark(node: &Node) -> bool {
    [
        "gmail_signature",
        "moz-signature",
        "protonmail_signature_block",
    ]
    .iter()
    .any(|c| has_class(node, c))
        || node
            .attr("data-smartmail")
            .is_some_and(|v| v.eq_ignore_ascii_case("gmail_signature"))
        || id_is(node, "signature")
        || id_is(node, "AppleMailSignature")
}

/// The "On … wrote:" line (or Thunderbird's `moz-cite-prefix`, or "Begin
/// forwarded message:") just before the quote `id`.
fn lead_in(dom: &Dom, id: NodeId) -> Option<NodeId> {
    let before = previous_siblings(dom, id).find(|&s| !blank(dom, s, 0))?;
    let node = &dom.nodes[before];
    if has_class(node, "moz-cite-prefix") {
        return Some(before);
    }
    let lines = lines(dom, before, &HashSet::new(), 400);
    let line = lines.join(" ");
    (lines.len() <= 2 && (attribution(&line) || is_forward(&line))).then_some(before)
}

/// Outlook's From / Sent / Subject block above the mail it quotes.
fn outlook_header(dom: &Dom, id: NodeId) -> bool {
    let lines = lines(dom, id, &HashSet::new(), 1000);
    let headers = Headers::read(&lines);
    lines
        .first()
        .and_then(|l| header_line(l))
        .is_some_and(|(f, _)| f == Field::From)
        && headers.date.is_some()
        && headers.subject.is_some()
}

fn border_top(node: &Node) -> bool {
    node.attr("style")
        .is_some_and(|s| css::declarations(s).iter().any(|(n, _)| n == "border-top"))
}

/// `id`, or the box around it when there is nothing else in that box.
fn climb(dom: &Dom, mut id: NodeId) -> NodeId {
    while let Some(parent) = dom.nodes[id].parent
        && !top(dom, parent)
        && dom.nodes[parent]
            .children
            .iter()
            .all(|&c| c == id || blank(dom, c, 0))
    {
        id = parent;
    }
    id
}

/// `id` and everything after it in its parent, with the rule (and
/// Outlook's empty `appendonsend` box) just above it.
fn to_end_of_parent(dom: &Dom, id: NodeId) -> Vec<NodeId> {
    let mut start = id;
    for s in previous_siblings(dom, id) {
        let node = &dom.nodes[s];
        if node.tag() == "hr" || id_is(node, "appendonsend") {
            start = s;
        } else if !blank(dom, s, 0) {
            break;
        }
    }
    let Some(parent) = dom.nodes[id].parent else {
        return vec![id];
    };
    let kids = &dom.nodes[parent].children;
    let from = kids.iter().position(|&c| c == start).unwrap_or(0);
    kids[from..].to_vec()
}

/// What follows the quote ending at `last`: `Some` with any further
/// quoted parts (Apple Mail on a phone splits its quote in two) when
/// there is nothing else but white space and a signature.
fn rest_is_quote(dom: &Dom, last: NodeId) -> Option<Vec<NodeId>> {
    let mut more = Vec::new();
    for id in following(dom, last) {
        let node = &dom.nodes[id];
        if blank(dom, id, 0) {
            continue;
        }
        if quote_mark(node) {
            more.push(id);
            continue;
        }
        if signature_mark(node) || first_line(dom, id).is_some_and(|l| starts_signature(&l)) {
            break;
        }
        return None;
    }
    Some(more)
}

/// A forwarded mail: `start` (the "Begin forwarded message:" line when
/// it sits outside) to `container`, which holds the header block and the
/// body.
fn forward(dom: &Dom, start: NodeId, container: NodeId) -> ForwardCut {
    let none = HashSet::new();
    let text = lines(dom, container, &none, 4000);
    let after = text
        .iter()
        .take(3)
        .position(|l| is_forward(l))
        .map_or(0, |m| m + 1);
    let headers = Headers::read(&text[after..]);

    // The body: what follows the header block, looking into a box that
    // holds both.
    let mut at = container;
    let body = loop {
        let kids = &dom.nodes[at].children;
        let content: Vec<usize> = (0..kids.len())
            .filter(|&ix| !blank(dom, kids[ix], 0))
            .collect();
        let header_kids = content
            .iter()
            .take_while(|&&ix| {
                lines(dom, kids[ix], &none, 2000)
                    .iter()
                    .all(|l| is_forward(l) || header_line(l).is_some())
            })
            .count();
        if header_kids == 0
            && let [only] = content.as_slice()
            && matches!(dom.nodes[kids[*only]].data, Data::Element { .. })
            && first_line(dom, kids[*only])
                .is_some_and(|l| is_forward(&l) || header_line(&l).is_some())
        {
            at = kids[*only];
            continue;
        }
        break match content.get(header_kids) {
            Some(&first) => kids[first..].to_vec(),
            None => Vec::new(),
        };
    };
    ForwardCut {
        roots: siblings_between(dom, start, container),
        body,
        from: headers.from,
        date: headers.date,
        subject: headers.subject,
    }
}

fn is_forward(line: &str) -> bool {
    matches!(marker(line), Some(Marker::Forward | Marker::Original))
}

/// A `-- ` line, or the one line phones sign with.
fn starts_signature(line: &str) -> bool {
    line == "--" || mobile_signature(line)
}

/// The signature, outside the nodes in `cut`: a marked one, or one that
/// starts with a `-- ` line, and everything after it up to the quote.
fn signature(dom: &Dom, cut: &HashSet<NodeId>) -> Option<Vec<NodeId>> {
    preorder(dom, cut, |id| {
        let node = &dom.nodes[id];
        let (start, mark) = if signature_mark(node) {
            (sig_prefix(dom, id), id)
        } else if dash_line(dom, id) {
            (id, id)
        } else {
            return None;
        };
        let rest: Vec<NodeId> = following(dom, mark)
            .into_iter()
            .filter(|n| !cut.contains(n))
            .collect();
        // A marked signature can be long; a `-- ` line only starts one
        // when what follows is short.
        let own = if signature_mark(node) {
            None
        } else {
            Some(mark)
        };
        let mut n = 0;
        for id in own.into_iter().chain(rest.iter().copied()) {
            n += lines(dom, id, cut, 4000).len();
            if n >= MAX_SIGNATURE_LINES {
                return None;
            }
        }
        let mut roots = siblings_between(dom, start, mark);
        roots.extend(rest);
        Some(roots)
    })
}

/// Gmail's `gmail_signature_prefix` (the `-- ` above the signature box),
/// or a `-- ` text just before `id`; `id` when there is none.
fn sig_prefix(dom: &Dom, id: NodeId) -> NodeId {
    for s in previous_siblings(dom, id) {
        let node = &dom.nodes[s];
        if has_class(node, "gmail_signature_prefix") || clean(&text_of(node)) == "--" {
            return s;
        }
        if !blank(dom, s, 0) {
            break;
        }
    }
    id
}

/// A `-- ` text at the start of a paragraph or after a line break.
fn dash_line(dom: &Dom, id: NodeId) -> bool {
    let node = &dom.nodes[id];
    if !matches!(&node.data, Data::Text(t) if t.len() < 32 && clean(t) == "--") {
        return false;
    }
    previous_siblings(dom, id)
        .find(|&s| !blank(dom, s, 0) || dom.nodes[s].tag() == "br")
        .is_none_or(|s| dom.nodes[s].tag() == "br")
}

fn text_of(node: &Node) -> String {
    match &node.data {
        Data::Text(t) => t.clone(),
        _ => String::new(),
    }
}

/// Walks the tree in document order, leaving out the nodes in `skip` and
/// what is in them, and gives the first thing `f` finds.
fn preorder<T>(
    dom: &Dom,
    skip: &HashSet<NodeId>,
    mut f: impl FnMut(NodeId) -> Option<T>,
) -> Option<T> {
    let mut stack = vec![DOCUMENT];
    while let Some(id) = stack.pop() {
        if skip.contains(&id) {
            continue;
        }
        let node = &dom.nodes[id];
        if INVISIBLE.contains(&node.tag()) {
            continue;
        }
        if id != DOCUMENT
            && let Some(found) = f(id)
        {
            return Some(found);
        }
        stack.extend(node.children.iter().rev());
    }
    None
}

/// The nodes after `id` to the end of the body: its later siblings, then
/// its parent's, and so on.
fn following(dom: &Dom, mut id: NodeId) -> Vec<NodeId> {
    let mut out = Vec::new();
    while let Some(parent) = dom.nodes[id].parent {
        let kids = &dom.nodes[parent].children;
        let at = kids
            .iter()
            .position(|&c| c == id)
            .map_or(kids.len(), |a| a + 1);
        out.extend(&kids[at..]);
        if top(dom, parent) {
            break;
        }
        id = parent;
    }
    out
}

/// The siblings before `id`, nearest first.
fn previous_siblings(dom: &Dom, id: NodeId) -> impl Iterator<Item = NodeId> + '_ {
    let kids: &[NodeId] = match dom.nodes[id].parent {
        Some(p) => &dom.nodes[p].children,
        None => &[],
    };
    let at = kids.iter().position(|&c| c == id).unwrap_or(0);
    kids[..at].iter().rev().copied()
}

/// `start` to `end`, siblings in that order; `[end]` when `start` is not
/// one of its earlier siblings.
fn siblings_between(dom: &Dom, start: NodeId, end: NodeId) -> Vec<NodeId> {
    if let Some(p) = dom.nodes[end].parent {
        let kids = &dom.nodes[p].children;
        if let (Some(a), Some(b)) = (
            kids.iter().position(|&c| c == start),
            kids.iter().position(|&c| c == end),
        ) && a <= b
        {
            return kids[a..=b].to_vec();
        }
    }
    vec![end]
}

/// The body, the root element or the document: as far as cuts reach.
fn top(dom: &Dom, id: NodeId) -> bool {
    id == DOCUMENT || matches!(dom.nodes[id].tag(), "body" | "html")
}

/// Whether `id` shows nothing but white space.
fn blank(dom: &Dom, id: NodeId, depth: usize) -> bool {
    let node = &dom.nodes[id];
    match &node.data {
        Data::Text(t) => clean(t).is_empty(),
        Data::Other => true,
        Data::Document | Data::Element { .. } => {
            let tag = node.tag();
            if INVISIBLE.contains(&tag) || tag == "hr" {
                return true;
            }
            depth < MAX_DEPTH
                && !matches!(tag, "img" | "svg" | "video")
                && node.children.iter().all(|&c| blank(dom, c, depth + 1))
        }
    }
}

/// The first line of text in `id`.
fn first_line(dom: &Dom, id: NodeId) -> Option<String> {
    lines(dom, id, &HashSet::new(), 400).into_iter().next()
}

/// The non-blank lines of text in `id` (about `limit` bytes of it),
/// white space collapsed, the nodes in `skip` left out.
fn lines(dom: &Dom, id: NodeId, skip: &HashSet<NodeId>, limit: usize) -> Vec<String> {
    let mut text = String::new();
    collect(dom, id, skip, limit, 0, &mut text);
    text.lines().map(clean).filter(|l| !l.is_empty()).collect()
}

fn collect(
    dom: &Dom,
    id: NodeId,
    skip: &HashSet<NodeId>,
    limit: usize,
    depth: usize,
    out: &mut String,
) {
    if out.len() >= limit || depth >= MAX_DEPTH || skip.contains(&id) {
        return;
    }
    let node = &dom.nodes[id];
    match &node.data {
        // Line breaks in the source are spaces; lines come from elements.
        Data::Text(t) => out.push_str(&t.replace(['\r', '\n'], " ")),
        Data::Other => {}
        Data::Document | Data::Element { .. } => {
            let tag = node.tag();
            if INVISIBLE.contains(&tag) {
                return;
            }
            let line = LINE_TAGS.contains(&tag);
            if line || tag == "br" {
                out.push('\n');
            }
            for &c in &node.children {
                collect(dom, c, skip, limit, depth + 1, out);
            }
            if line {
                out.push('\n');
            } else if matches!(tag, "td" | "th") {
                out.push(' ');
            }
        }
    }
}

/// `text` with white space collapsed and invisible characters dropped.
fn clean(text: &str) -> String {
    text.split(|c: char| c.is_whitespace())
        .map(|w| w.replace(['\u{200b}', '\u{feff}', '\u{ad}'], ""))
        .filter(|w| !w.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Whether `class` lists `name`, ignoring case.
fn has_class(node: &Node, name: &str) -> bool {
    node.attr("class").is_some_and(|c| {
        c.split_ascii_whitespace()
            .any(|t| t.eq_ignore_ascii_case(name))
    })
}

fn id_is(node: &Node, name: &str) -> bool {
    node.attr("id")
        .is_some_and(|v| v.trim().eq_ignore_ascii_case(name))
}

#[cfg(test)]
mod tests {
    use super::super::{Block, Document, trimmed};
    use crate::trim::Trimmed;

    fn trim(html: &str) -> Trimmed<Document> {
        trimmed(html, &|_| None)
    }

    /// The text of a document, a line per paragraph.
    fn text(doc: &Document) -> String {
        fn walk(blocks: &[Block], out: &mut Vec<String>) {
            for block in blocks {
                match block {
                    Block::Text(t) => out.push(t.text().trim().to_owned()),
                    Block::Box(b) => walk(&b.children, out),
                    Block::Rule => out.push("---".to_owned()),
                }
            }
        }
        let mut out = Vec::new();
        walk(&doc.blocks, &mut out);
        out.join("\n")
    }

    fn some_text(doc: &Option<Document>) -> String {
        doc.as_ref().map(text).unwrap_or_default()
    }

    #[test]
    fn gmail_reply() {
        let t = trim(
            r#"<div dir="ltr">Friday works for me.<br clear="all"><div><br></div>
<span class="gmail_signature_prefix">-- </span><br><div dir="ltr" class="gmail_signature"
 data-smartmail="gmail_signature"><div dir="ltr">Rajat Roy<br>Demo Labs</div></div></div>
<br><div class="gmail_quote gmail_quote_container"><div dir="ltr" class="gmail_attr">On Tue,
 30 Sep 2026 at 18:02, Priya Nair &lt;<a href="mailto:priya@demo.example">priya@demo.example</a>&gt;
 wrote:<br></div><blockquote class="gmail_quote" style="margin:0px 0px 0px 0.8ex;
border-left:1px solid rgb(204,204,204);padding-left:1ex"><div dir="ltr">Could we meet on
 Friday?</div></blockquote></div>"#,
        );
        assert_eq!(text(&t.said), "Friday works for me.");
        assert_eq!(some_text(&t.signature), "--\nRajat Roy\nDemo Labs");
        let quoted = some_text(&t.quoted);
        assert!(quoted.starts_with("On Tue, 30 Sep 2026"), "{quoted}");
        assert!(quoted.ends_with("Could we meet on Friday?"), "{quoted}");
        assert!(t.forwarded.is_none());
    }

    #[test]
    fn apple_mail_reply() {
        let t = trim(
            r#"<html><body dir="auto"><div dir="ltr">Sure, see you then.</div><div dir="ltr"><br>
<div id="AppleMailSignature">Sent from my iPhone</div><br><blockquote type="cite">On 30 Sep 2026,
 at 18:02, Priya Nair &lt;priya@demo.example&gt; wrote:<br><br></blockquote></div>
<blockquote type="cite"><div dir="ltr">Could we meet on Friday?</div></blockquote></body></html>"#,
        );
        assert_eq!(text(&t.said), "Sure, see you then.");
        assert_eq!(some_text(&t.signature), "Sent from my iPhone");
        assert_eq!(
            some_text(&t.quoted),
            "On 30 Sep 2026, at 18:02, Priya Nair <priya@demo.example> wrote:\n\
Could we meet on Friday?"
        );
    }

    #[test]
    fn outlook_web_reply() {
        let t = trim(
            r##"<div style="font-family:Aptos;color:rgb(0,0,0)">Approved.</div>
<div id="appendonsend"></div><hr style="display:inline-block;width:98%" tabindex="-1">
<div id="divRplyFwdMsg" dir="ltr"><font face="Calibri" style="font-size:11pt" color="#000000">
<b>From:</b> Priya Nair &lt;priya@demo.example&gt;<br><b>Sent:</b> Tuesday, September 30, 2026
 6:02 PM<br><b>To:</b> Omar Haddad &lt;omar@demo.example&gt;<br><b>Subject:</b> Budget</font>
<div>&nbsp;</div></div><div><p>Please approve the budget.</p></div>"##,
        );
        assert_eq!(text(&t.said), "Approved.");
        assert!(t.said.styled);
        let quoted = some_text(&t.quoted);
        assert!(quoted.starts_with("---\nFrom: Priya Nair"), "{quoted}");
        assert!(quoted.ends_with("Please approve the budget."), "{quoted}");
    }

    #[test]
    fn outlook_desktop_reply() {
        let t = trim(
            r#"<body lang="EN-GB"><div class="WordSection1"><p class="MsoNormal">Done, see the
 sheet.<o:p></o:p></p><p class="MsoNormal"><o:p>&nbsp;</o:p></p><div><div
 style="border:none;border-top:solid #E1E1E1 1.0pt;padding:3.0pt 0cm 0cm 0cm"><p
 class="MsoNormal"><b>From:</b> Priya Nair &lt;priya@demo.example&gt;<br><b>Sent:</b>
 30 September 2026 18:02<br><b>To:</b> Omar Haddad<br><b>Subject:</b> Budget<o:p></o:p></p>
</div></div><p class="MsoNormal"><o:p>&nbsp;</o:p></p><p class="MsoNormal">Can you update the
 sheet?<o:p></o:p></p></div></body>"#,
        );
        assert_eq!(text(&t.said), "Done, see the sheet.");
        let quoted = some_text(&t.quoted);
        assert!(quoted.starts_with("From: Priya Nair"), "{quoted}");
        assert!(quoted.ends_with("Can you update the sheet?"), "{quoted}");
    }

    #[test]
    fn thunderbird_reply() {
        let t = trim(
            r#"<html><head><meta http-equiv="content-type" content="text/html; charset=UTF-8">
</head><body><p>Friday works.</p><div class="moz-signature">-- <br>Rajat Roy</div>
<div class="moz-cite-prefix">On 30/09/2026 18:02, Priya Nair wrote:<br></div>
<blockquote type="cite" cite="mid:abc@demo.example"><p>Could we meet on Friday?</p>
</blockquote><br></body></html>"#,
        );
        assert_eq!(text(&t.said), "Friday works.");
        assert_eq!(some_text(&t.signature), "--\nRajat Roy");
        assert_eq!(
            some_text(&t.quoted),
            "On 30/09/2026 18:02, Priya Nair wrote:\nCould we meet on Friday?"
        );
    }

    #[test]
    fn gmail_forward() {
        let t = trim(
            r#"<div dir="ltr">FYI, our tickets.<br><br><div class="gmail_quote gmail_quote_container">
<div dir="ltr" class="gmail_attr">---------- Forwarded message ---------<br>From: <strong
 class="gmail_sendername" dir="auto">Demo Air</strong> <span dir="auto">&lt;<a
 href="mailto:fares@demo.example">fares@demo.example</a>&gt;</span><br>Date: Tue, 30 Sep 2026 at
 18:02<br>Subject: Your booking<br>To: &lt;<a
 href="mailto:priya@demo.example">priya@demo.example</a>&gt;<br></div>
<br><br><div dir="ltr"><p>Your flight is booked.</p><img src="https://demo.example/logo.png"
 width="120"></div></div></div>"#,
        );
        assert_eq!(text(&t.said), "FYI, our tickets.");
        assert!(t.quoted.is_none());
        let f = t.forwarded.unwrap();
        assert_eq!(f.from.as_deref(), Some("Demo Air <fares@demo.example>"));
        assert_eq!(f.date.as_deref(), Some("Tue, 30 Sep 2026 at 18:02"));
        assert_eq!(f.subject.as_deref(), Some("Your booking"));
        assert!(text(&f.body).starts_with("Your flight is booked."));
        assert_eq!(f.body.remote_images, 1);
        assert_eq!(t.said.remote_images, 0);
    }

    #[test]
    fn apple_and_thunderbird_forwards() {
        let t = trim(
            r#"<html><body dir="auto"><div dir="ltr"></div><div dir="ltr"><br><blockquote
 type="cite"><div dir="ltr">Begin forwarded message:</div><br
 class="Apple-interchange-newline"><div style="margin:0px"><span><b>From: </b></span><span>Demo
 Air &lt;fares@demo.example&gt;</span></div><div style="margin:0px"><span><b>Subject:
 </b></span><span><b>Your booking</b></span></div><div style="margin:0px"><span><b>Date:
 </b></span><span>30 September 2026 at 18:02:11 BST</span></div><div
 style="margin:0px"><span><b>To: </b></span><span>Priya Nair &lt;priya@demo.example&gt;</span></div>
<br><div dir="ltr">Your flight is booked.</div></blockquote></div></body></html>"#,
        );
        assert_eq!(text(&t.said), "");
        let f = t.forwarded.unwrap();
        assert_eq!(f.from.as_deref(), Some("Demo Air <fares@demo.example>"));
        assert_eq!(f.subject.as_deref(), Some("Your booking"));
        assert_eq!(f.date.as_deref(), Some("30 September 2026 at 18:02:11 BST"));
        assert_eq!(text(&f.body), "Your flight is booked.");

        let t = trim(
            r#"<body><p>See below.</p><div class="moz-forward-container"><br><br>
-------- Forwarded Message --------<table class="moz-email-headers-table" border="0"
 cellpadding="0" cellspacing="0"><tbody><tr><th valign="BASELINE" nowrap
 align="RIGHT">Subject: </th><td>Your booking</td></tr><tr><th valign="BASELINE" nowrap
 align="RIGHT">From: </th><td>Demo Air <a
 href="mailto:fares@demo.example">&lt;fares@demo.example&gt;</a></td></tr></tbody></table>
<br><p>Your flight is booked.</p></div></body>"#,
        );
        assert_eq!(text(&t.said), "See below.");
        let f = t.forwarded.unwrap();
        assert_eq!(f.from.as_deref(), Some("Demo Air <fares@demo.example>"));
        assert_eq!(f.subject.as_deref(), Some("Your booking"));
        assert_eq!(text(&f.body), "Your flight is booked.");
    }

    #[test]
    fn inline_replies_are_kept() {
        let t = trim(
            r#"<div class="moz-cite-prefix">On 30/09/2026 18:02, Priya Nair wrote:</div>
<blockquote type="cite">Can you do Friday?</blockquote><p>Yes, after lunch.</p>
<blockquote type="cite">And bring the slides?</blockquote><p>Will do.</p>"#,
        );
        assert_eq!(
            text(&t.said),
            "On 30/09/2026 18:02, Priya Nair wrote:\nCan you do Friday?\nYes, after lunch.\n\
And bring the slides?\nWill do."
        );
        assert!(t.quoted.is_none() && t.forwarded.is_none());
    }

    #[test]
    fn dash_signature_and_nothing_to_cut() {
        let t = trim("<p>See you at nine.</p><p>-- <br>Priya Nair<br>Demo Ltd</p>");
        assert_eq!(text(&t.said), "See you at nine.");
        assert_eq!(some_text(&t.signature), "--\nPriya Nair\nDemo Ltd");

        let html = r##"<body bgcolor="#fafafa"><p>Hi Priya,</p><p>the report -- with
 the numbers -- is attached.</p><img src="https://demo.example/chart.png"></body>"##;
        let t = trim(html);
        assert_eq!(
            t,
            Trimmed {
                said: super::super::document(html, &|_| None),
                ..Trimmed::default()
            }
        );
        assert_eq!(t.said.remote_images, 1);
    }
}
