// SPDX-License-Identifier: GPL-3.0-or-later

//! A small arena DOM that `html5ever` builds, so the tree builder's error
//! recovery (unclosed `<p>`, stray `<td>`, …) matches a browser's. Only
//! elements, their attributes and text are kept; comments, doctypes and
//! processing instructions are dropped.

use std::borrow::Cow;
use std::cell::{Ref, RefCell};

use html5ever::interface::{ElementFlags, NodeOrText, QuirksMode, TreeSink};
use html5ever::tendril::{StrTendril, TendrilSink};
use html5ever::{Attribute, QualName, parse_document};

/// A node's index in [`Dom::nodes`].
pub type NodeId = usize;

/// The document node.
pub const DOCUMENT: NodeId = 0;

pub enum Data {
    Document,
    Element {
        attrs: Vec<(String, String)>,
    },
    Text(String),
    /// A comment, doctype or processing instruction: kept only so handles
    /// stay valid, never shown.
    Other,
}

pub struct Node {
    pub data: Data,
    /// The element's name; empty for other nodes.
    pub name: QualName,
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
}

impl Node {
    /// The lower-case tag name of an HTML element, or `""`.
    pub fn tag(&self) -> &str {
        match &self.data {
            Data::Element { .. } if self.name.ns == html5ever::ns!(html) => &self.name.local,
            _ => "",
        }
    }

    pub fn attr(&self, key: &str) -> Option<&str> {
        match &self.data {
            Data::Element { attrs, .. } => attrs
                .iter()
                .find(|(k, _)| k == key)
                .map(|(_, v)| v.as_str()),
            _ => None,
        }
    }
}

pub struct Dom {
    pub nodes: Vec<Node>,
}

/// Parses `html` the way a browser would.
pub fn parse(html: &str) -> Dom {
    let sink = Sink {
        nodes: RefCell::new(vec![Node {
            data: Data::Document,
            name: no_name(),
            parent: None,
            children: Vec::new(),
        }]),
    };
    parse_document(sink, Default::default()).one(StrTendril::from(html))
}

fn no_name() -> QualName {
    QualName::new(None, html5ever::ns!(), html5ever::local_name!(""))
}

struct Sink {
    nodes: RefCell<Vec<Node>>,
}

impl Sink {
    fn push(&self, data: Data, name: QualName) -> NodeId {
        let mut nodes = self.nodes.borrow_mut();
        nodes.push(Node {
            data,
            name,
            parent: None,
            children: Vec::new(),
        });
        nodes.len() - 1
    }

    fn detach(nodes: &mut [Node], id: NodeId) {
        if let Some(parent) = nodes[id].parent.take() {
            nodes[parent].children.retain(|&c| c != id);
        }
    }

    /// Inserts `child` into `parent` at `at` (the end when `None`), merging
    /// text into a neighbouring text node like a browser does.
    fn insert(&self, parent: NodeId, at: Option<usize>, child: NodeOrText<NodeId>) {
        let mut nodes = self.nodes.borrow_mut();
        let at = at.unwrap_or(nodes[parent].children.len());
        match child {
            NodeOrText::AppendText(text) => {
                if at > 0 {
                    let before = nodes[parent].children[at - 1];
                    if let Data::Text(existing) = &mut nodes[before].data {
                        existing.push_str(&text);
                        return;
                    }
                }
                nodes.push(Node {
                    data: Data::Text(text.to_string()),
                    name: no_name(),
                    parent: Some(parent),
                    children: Vec::new(),
                });
                let id = nodes.len() - 1;
                nodes[parent].children.insert(at, id);
            }
            NodeOrText::AppendNode(id) => {
                Self::detach(&mut nodes, id);
                let at = at.min(nodes[parent].children.len());
                nodes[id].parent = Some(parent);
                nodes[parent].children.insert(at, id);
            }
        }
    }
}

impl TreeSink for Sink {
    type Handle = NodeId;
    type Output = Dom;
    type ElemName<'a> = Ref<'a, QualName>;

    fn finish(self) -> Dom {
        Dom {
            nodes: self.nodes.into_inner(),
        }
    }

    fn parse_error(&self, _msg: Cow<'static, str>) {}

    fn get_document(&self) -> NodeId {
        DOCUMENT
    }

    fn elem_name<'a>(&'a self, target: &'a NodeId) -> Ref<'a, QualName> {
        Ref::map(self.nodes.borrow(), |nodes| &nodes[*target].name)
    }

    fn create_element(&self, name: QualName, attrs: Vec<Attribute>, _: ElementFlags) -> NodeId {
        let attrs = attrs
            .into_iter()
            .map(|a| (a.name.local.to_string(), a.value.to_string()))
            .collect();
        self.push(Data::Element { attrs }, name)
    }

    fn create_comment(&self, _text: StrTendril) -> NodeId {
        self.push(Data::Other, no_name())
    }

    fn create_pi(&self, _target: StrTendril, _data: StrTendril) -> NodeId {
        self.push(Data::Other, no_name())
    }

    fn append(&self, parent: &NodeId, child: NodeOrText<NodeId>) {
        self.insert(*parent, None, child);
    }

    fn append_based_on_parent_node(
        &self,
        element: &NodeId,
        prev_element: &NodeId,
        child: NodeOrText<NodeId>,
    ) {
        let has_parent = self.nodes.borrow()[*element].parent.is_some();
        if has_parent {
            self.append_before_sibling(element, child);
        } else {
            self.append(prev_element, child);
        }
    }

    fn append_doctype_to_document(&self, _: StrTendril, _: StrTendril, _: StrTendril) {}

    fn get_template_contents(&self, target: &NodeId) -> NodeId {
        // Template contents are never shown; the element itself will do.
        *target
    }

    fn same_node(&self, x: &NodeId, y: &NodeId) -> bool {
        x == y
    }

    fn set_quirks_mode(&self, _mode: QuirksMode) {}

    fn append_before_sibling(&self, sibling: &NodeId, new_node: NodeOrText<NodeId>) {
        let (parent, at) = {
            let nodes = self.nodes.borrow();
            let Some(parent) = nodes[*sibling].parent else {
                return;
            };
            let at = nodes[parent].children.iter().position(|c| c == sibling);
            (parent, at)
        };
        self.insert(parent, at, new_node);
    }

    fn add_attrs_if_missing(&self, target: &NodeId, attrs: Vec<Attribute>) {
        let mut nodes = self.nodes.borrow_mut();
        if let Data::Element {
            attrs: existing, ..
        } = &mut nodes[*target].data
        {
            for attr in attrs {
                let key = attr.name.local.to_string();
                if !existing.iter().any(|(k, _)| *k == key) {
                    existing.push((key, attr.value.to_string()));
                }
            }
        }
    }

    fn remove_from_parent(&self, target: &NodeId) {
        Self::detach(&mut self.nodes.borrow_mut(), *target);
    }

    fn reparent_children(&self, node: &NodeId, new_parent: &NodeId) {
        let mut nodes = self.nodes.borrow_mut();
        let children = std::mem::take(&mut nodes[*node].children);
        for &child in &children {
            nodes[child].parent = Some(*new_parent);
        }
        nodes[*new_parent].children.extend(children);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn outline(dom: &Dom, id: NodeId, out: &mut String) {
        let node = &dom.nodes[id];
        match &node.data {
            Data::Text(text) => out.push_str(&format!("'{text}'")),
            Data::Element { .. } => {
                out.push_str(&format!("<{}>", node.tag()));
                for &c in &node.children {
                    outline(dom, c, out);
                }
                out.push_str(&format!("</{}>", node.tag()));
            }
            _ => {
                for &c in &node.children {
                    outline(dom, c, out);
                }
            }
        }
    }

    #[test]
    fn recovers_like_a_browser() {
        let dom = parse("<p>one<p>two <b>bold</p><table><td>cell</table><!-- x -->");
        let mut out = String::new();
        outline(&dom, DOCUMENT, &mut out);
        assert_eq!(
            out,
            "<html><head></head><body><p>'one'</p><p>'two '<b>'bold'</b></p>\
<table><tbody><tr><td>'cell'</td></tr></tbody></table></body></html>"
        );
    }

    #[test]
    fn attributes() {
        let dom = parse(r#"<a href="https://example.org/" title=x>link</a>"#);
        let a = dom.nodes.iter().find(|n| n.tag() == "a").unwrap();
        assert_eq!(a.attr("href"), Some("https://example.org/"));
        assert_eq!(a.attr("title"), Some("x"));
        assert_eq!(a.attr("rel"), None);
    }
}
