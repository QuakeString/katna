// SPDX-License-Identifier: GPL-3.0-or-later

//! Importing contacts from a file (`docs/ARCHITECTURE.md` §8.6): the app
//! reads the vCards, the daemon saves each in the chosen address book the
//! way Create contact does, labels included.

use katna_core::contact::Card;
use serde::Deserialize;

use super::{CommandError, Daemon};

/// One card to import, as the app sends it.
#[derive(Deserialize)]
struct Import {
    card: Card,
    #[serde(default)]
    labels: Vec<String>,
}

impl Daemon {
    /// Saves `cards` (a JSON list of `{card, labels}`) as new contacts in
    /// address book `book` (0: this computer). Returns the new cards' ids;
    /// fails only when none could be saved.
    pub async fn import_contacts(&self, book: i64, cards: &str) -> Result<Vec<i64>, CommandError> {
        let cards: Vec<Import> = serde_json::from_str(cards)
            .map_err(|e| CommandError::InvalidArgs(format!("contacts to import: {e}")))?;
        let mut ids = Vec::new();
        let mut first_error = None;
        for Import { card, labels } in cards {
            if self.closing() {
                break;
            }
            let json = serde_json::to_string(&card)
                .map_err(|e| CommandError::InvalidArgs(format!("contact card: {e}")))?;
            match self.save_contact(0, book, &json).await {
                Ok(id) => {
                    if !labels.is_empty()
                        && let Err(err) = self.set_contact_labels(id, labels).await
                    {
                        tracing::info!(id, %err, "imported contact kept without its labels");
                    }
                    ids.push(id);
                }
                Err(err) => {
                    tracing::info!(%err, "a contact was not imported");
                    first_error.get_or_insert(err);
                }
            }
        }
        match first_error {
            Some(err) if ids.is_empty() => Err(err),
            _ => {
                tracing::info!(count = ids.len(), book, "contacts imported");
                Ok(ids)
            }
        }
    }
}
