// SPDX-License-Identifier: GPL-3.0-or-later

//! Mail templates (`template`, `template_attachment` in `pim.db`): saved
//! messages the compose window starts new mail or a reply from.

use rusqlite::{OptionalExtension, TransactionBehavior, params};

use crate::Store;
use crate::db::unix_now;
use crate::error::Result;

/// A template as listed: without its body and attachments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateSummary {
    pub id: i64,
    pub name: String,
    pub subject: String,
}

/// A file that goes with a template.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateFile {
    pub name: String,
    pub mime: String,
    pub data: Vec<u8>,
}

/// A whole template.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Template {
    /// 0 for one not saved yet.
    pub id: i64,
    pub name: String,
    pub subject: String,
    /// The formatted body, pictures inside as `data:` URIs.
    pub html: String,
    /// The same body as plain text.
    pub text: String,
    pub attachments: Vec<TemplateFile>,
}

impl Store {
    /// Every template, by name.
    pub fn templates(&self) -> Result<Vec<TemplateSummary>> {
        let mut stmt = self.pim.prepare_cached(
            "SELECT id, name, subject FROM template ORDER BY name COLLATE NOCASE, id",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(TemplateSummary {
                id: row.get(0)?,
                name: row.get(1)?,
                subject: row.get(2)?,
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// Template `id` with its body and attachments, if it exists.
    pub fn template(&self, id: i64) -> Result<Option<Template>> {
        let found = self
            .pim
            .prepare_cached("SELECT name, subject, html, text FROM template WHERE id = ?1")?
            .query_row([id], |row| {
                Ok(Template {
                    id,
                    name: row.get(0)?,
                    subject: row.get(1)?,
                    html: row.get(2)?,
                    text: row.get(3)?,
                    attachments: Vec::new(),
                })
            })
            .optional()?;
        let Some(mut template) = found else {
            return Ok(None);
        };
        let mut stmt = self.pim.prepare_cached(
            "SELECT name, mime, data FROM template_attachment
             WHERE template_id = ?1 ORDER BY position",
        )?;
        let files = stmt.query_map([id], |row| {
            Ok(TemplateFile {
                name: row.get(0)?,
                mime: row.get(1)?,
                data: row.get(2)?,
            })
        })?;
        template.attachments = files.collect::<rusqlite::Result<_>>()?;
        Ok(Some(template))
    }

    /// Saves `template`: a new one when its ID is 0 (or no longer exists),
    /// else in place of the old one, attachments and all. Returns its ID.
    pub fn save_template(&mut self, template: &Template) -> Result<i64> {
        self.check_writable()?;
        let tx = self
            .pim
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let now = unix_now();
        let updated = template.id != 0
            && tx.execute(
                "UPDATE template SET name = ?2, subject = ?3, html = ?4, text = ?5,
                                     updated_at = ?6
                 WHERE id = ?1",
                params![
                    template.id,
                    template.name,
                    template.subject,
                    template.html,
                    template.text,
                    now
                ],
            )? > 0;
        let id = if updated {
            tx.execute(
                "DELETE FROM template_attachment WHERE template_id = ?1",
                [template.id],
            )?;
            template.id
        } else {
            tx.execute(
                "INSERT INTO template (name, subject, html, text, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    template.name,
                    template.subject,
                    template.html,
                    template.text,
                    now
                ],
            )?;
            tx.last_insert_rowid()
        };
        for (position, file) in template.attachments.iter().enumerate() {
            tx.execute(
                "INSERT INTO template_attachment (template_id, position, name, mime, data)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![id, position as i64, file.name, file.mime, file.data],
            )?;
        }
        tx.commit()?;
        Ok(id)
    }

    /// Renames template `id`. Returns whether it exists.
    pub fn rename_template(&mut self, id: i64, name: &str) -> Result<bool> {
        self.check_writable()?;
        Ok(self.pim.execute(
            "UPDATE template SET name = ?2, updated_at = ?3 WHERE id = ?1",
            params![id, name, unix_now()],
        )? > 0)
    }

    /// Deletes template `id` and its attachments. Returns whether it
    /// existed.
    pub fn delete_template(&mut self, id: i64) -> Result<bool> {
        self.check_writable()?;
        Ok(self
            .pim
            .execute("DELETE FROM template WHERE id = ?1", [id])?
            > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Mode;
    use katna_core::Paths;

    fn store() -> (tempfile::TempDir, Store) {
        let dir = tempfile::tempdir().unwrap();
        let paths = Paths::with_root(dir.path());
        let store = Store::open(&paths, Mode::ReadWrite).unwrap();
        (dir, store)
    }

    fn file(name: &str) -> TemplateFile {
        TemplateFile {
            name: name.to_owned(),
            mime: "text/plain".to_owned(),
            data: name.as_bytes().to_vec(),
        }
    }

    #[test]
    fn saves_lists_renames_and_deletes() {
        let (_dir, mut store) = store();
        let mut template = Template {
            name: "Welcome".to_owned(),
            subject: "Hello".to_owned(),
            html: "<p>Hi {first name}</p>".to_owned(),
            text: "Hi {first name}".to_owned(),
            attachments: vec![file("a.txt"), file("b.txt")],
            ..Template::default()
        };
        let id = store.save_template(&template).unwrap();
        template.id = id;
        assert_eq!(store.template(id).unwrap().as_ref(), Some(&template));
        let other = store
            .save_template(&Template {
                name: "after".to_owned(),
                ..Template::default()
            })
            .unwrap();
        let names: Vec<String> = store
            .templates()
            .unwrap()
            .into_iter()
            .map(|t| t.name)
            .collect();
        assert_eq!(names, ["after", "Welcome"]);

        template.attachments = vec![file("c.txt")];
        template.text = "Changed".to_owned();
        assert_eq!(store.save_template(&template).unwrap(), id);
        assert_eq!(store.template(id).unwrap().as_ref(), Some(&template));

        assert!(store.rename_template(other, "Zed").unwrap());
        assert_eq!(store.templates().unwrap()[1].name, "Zed");
        assert!(store.delete_template(id).unwrap());
        assert!(!store.delete_template(id).unwrap());
        assert_eq!(store.template(id).unwrap(), None);
        assert_eq!(store.templates().unwrap().len(), 1);
    }
}
