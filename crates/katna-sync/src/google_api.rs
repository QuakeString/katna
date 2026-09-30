// SPDX-License-Identifier: GPL-3.0-or-later

//! Google's answer when one of its APIs is switched off in the Google
//! Cloud project Katna signs in with (`SERVICE_DISABLED`, or the older
//! `accessNotConfigured`): which API, and the page that turns it on.

use katna_core::api_off;
use serde::Deserialize;

use crate::Error;

#[derive(Deserialize, Default)]
#[serde(default)]
struct Answer {
    error: Body,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct Body {
    message: String,
    errors: Vec<Item>,
    details: Vec<Item>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct Item {
    reason: String,
    #[serde(rename = "extendedHelp")]
    extended_help: String,
    metadata: Metadata,
    links: Vec<Link>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct Metadata {
    service: String,
    #[serde(rename = "serviceTitle")]
    service_title: String,
    #[serde(rename = "activationUrl")]
    activation_url: String,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct Link {
    url: String,
}

/// [`Error::NotEnabled`] when a `403` answer says the API is switched off,
/// its detail naming the API and, when Google gave it, the page that turns
/// it on ([`api_off::detail`]).
pub(crate) fn switched_off(status: u16, body: &[u8]) -> Option<Error> {
    if status != 403 {
        return None;
    }
    let answer: Answer = serde_json::from_slice(body).ok()?;
    let error = answer.error;
    let items = || error.errors.iter().chain(&error.details);
    if !items().any(|i| {
        matches!(
            i.reason.as_str(),
            "SERVICE_DISABLED" | "accessNotConfigured"
        )
    }) {
        return None;
    }
    let api = items()
        .map(|i| i.metadata.service_title.trim())
        .find(|title| !title.is_empty())
        .map(str::to_owned)
        .or_else(|| {
            // "People API has not been used in project 1 before or it is disabled."
            let (name, _) = error.message.split_once(" has not been used")?;
            let name = name.trim();
            (!name.is_empty() && name.len() <= 80).then(|| name.to_owned())
        })
        .or_else(|| {
            items()
                .map(|i| i.metadata.service.trim())
                .find(|service| !service.is_empty())
                .map(str::to_owned)
        })
        .unwrap_or_else(|| "A Google API".to_owned());
    let page = items()
        .flat_map(|i| {
            [i.metadata.activation_url.as_str(), i.extended_help.as_str()]
                .into_iter()
                .chain(i.links.iter().map(|l| l.url.as_str()))
        })
        .chain(error.message.split_whitespace())
        .map(|url| url.trim_end_matches(['.', ',', ';', ')']))
        .find(|url| api_off::is_console_page(url));
    Some(Error::NotEnabled(match page {
        Some(page) => api_off::detail(&api, page),
        None => format!("{api} is not enabled for Katna's Google Cloud project"),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// What Google answered when People API was off (his Contacts, r414).
    const PEOPLE_OFF: &str = r#"{
  "error": {
    "code": 403,
    "message": "People API has not been used in project 679905849385 before or it is disabled. Enable it by visiting https://console.developers.google.com/apis/api/people.googleapis.com/overview?project=679905849385 then retry. If you enabled this API recently, wait a few minutes for the action to propagate to our systems and retry.",
    "status": "PERMISSION_DENIED",
    "details": [
      {
        "@type": "type.googleapis.com/google.rpc.ErrorInfo",
        "reason": "SERVICE_DISABLED",
        "domain": "googleapis.com",
        "metadata": {
          "consumer": "projects/679905849385",
          "service": "people.googleapis.com",
          "serviceTitle": "People API",
          "activationUrl": "https://console.developers.google.com/apis/api/people.googleapis.com/overview?project=679905849385"
        }
      },
      {
        "@type": "type.googleapis.com/google.rpc.Help",
        "links": [
          {
            "description": "Google developers console API activation",
            "url": "https://console.developers.google.com/apis/api/people.googleapis.com/overview?project=679905849385"
          }
        ]
      }
    ]
  }
}"#;

    fn detail(status: u16, body: &str) -> Option<String> {
        match switched_off(status, body.as_bytes())? {
            Error::NotEnabled(detail) => Some(detail),
            other => panic!("{other}"),
        }
    }

    #[test]
    fn names_the_api_and_its_page() {
        let detail = detail(403, PEOPLE_OFF).unwrap();
        assert_eq!(
            api_off::parse(&detail),
            Some((
                "People API",
                "https://console.developers.google.com/apis/api/people.googleapis.com/overview?project=679905849385"
            ))
        );
    }

    #[test]
    fn older_answers_name_it_in_the_message() {
        let body = r#"{"error":{"code":403,"errors":[{"domain":"usageLimits","reason":"accessNotConfigured","message":"x","extendedHelp":"https://console.developers.google.com/apis/api/tasks.googleapis.com/overview?project=5"}],"message":"Google Tasks API has not been used in project 5 before or it is disabled. Enable it by visiting https://console.developers.google.com/apis/api/tasks.googleapis.com/overview?project=5 then retry."}}"#;
        let detail = detail(403, body).unwrap();
        assert_eq!(
            api_off::parse(&detail),
            Some((
                "Google Tasks API",
                "https://console.developers.google.com/apis/api/tasks.googleapis.com/overview?project=5"
            ))
        );
    }

    #[test]
    fn other_answers_are_not_it() {
        assert_eq!(detail(401, PEOPLE_OFF), None);
        assert_eq!(
            detail(
                403,
                r#"{"error":{"errors":[{"reason":"insufficientPermissions"}]}}"#
            ),
            None
        );
        assert_eq!(detail(403, "not json"), None);
        // A page elsewhere is never offered.
        let elsewhere = r#"{"error":{"message":"Drive API has not been used","details":[{"reason":"SERVICE_DISABLED","metadata":{"activationUrl":"https://evil.example/x"}}]}}"#;
        assert_eq!(
            detail(403, elsewhere).as_deref(),
            Some("Drive API is not enabled for Katna's Google Cloud project")
        );
    }
}
