// SPDX-License-Identifier: GPL-3.0-or-later

//! What the daemon and Katna Server say to each other for Katna AI
//! (`POST /api/v1/ai/rephrase` and `/api/v1/ai/complete`), and the
//! problems the daemon reports to Katna Mail.

use serde::{Deserialize, Serialize};

/// A rephrase request: the selected text, a [`crate::Tone`] id and, for a
/// custom tone, the user's instruction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RephraseRequest {
    pub text: String,
    pub tone: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub instruction: String,
}

/// A request to finish the sentence at the end of `before`; `answered` is
/// the mail being answered, when the user allows sending it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompleteRequest {
    pub before: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub answered: String,
}

/// Katna Server's answer: the text, and where the account stands.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiAnswer {
    pub text: String,
    #[serde(default)]
    pub plan: Plan,
}

/// Where a Katna account stands with Katna AI.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Plan {
    /// [`plan::TRIAL`] or [`plan::PAID`].
    #[serde(default)]
    pub kind: String,
    /// Whole days of the free month left, while it runs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days_left: Option<u32>,
}

pub mod plan {
    /// The free month, started by the first use.
    pub const TRIAL: &str = "trial";
    /// Paid for this month.
    pub const PAID: &str = "paid";
}

/// Why the daemon gave no text: the second field of its `AiRephrase` and
/// `AiComplete` answers, empty when it did.
pub mod problem {
    /// Writing help is off in the settings, or this build has no Katna
    /// Server for Katna AI.
    pub const OFF: &str = "off";
    /// Katna AI needs this computer signed in to a Katna account.
    pub const SIGN_IN: &str = "sign-in";
    /// The free month is over and this month is not paid for.
    pub const PAY: &str = "pay";
    /// Over a limit for now (this month's cap, or the service's own).
    pub const TOO_MANY: &str = "too-many";
    /// No key saved for the user's own service.
    pub const NO_KEY: &str = "no-key";
    /// The user's own service refused the key.
    pub const BAD_KEY: &str = "bad-key";
    /// The service could not be reached, or failed.
    pub const FAILED: &str = "failed";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requests_leave_out_empty_fields() {
        let request = RephraseRequest {
            text: "hi".into(),
            tone: "clearer".into(),
            instruction: String::new(),
        };
        assert_eq!(
            serde_json::to_string(&request).unwrap(),
            r#"{"text":"hi","tone":"clearer"}"#
        );
        let answer: AiAnswer = serde_json::from_str(r#"{"text":"Hi"}"#).unwrap();
        assert_eq!(answer.plan, Plan::default());
    }
}
