// SPDX-License-Identifier: GPL-3.0-or-later

//! A Google API switched off in the Google Cloud project Katna signs in
//! with, as the daemon passes it on: the detail of a `not-enabled` sync
//! state, or a failed Drive upload's error. It holds the API's name and
//! Google's page that turns it on, so the app can say which API it is and
//! open that page.

/// Between the name and the page. Neither holds a line break.
const SEPARATOR: char = '\n';

/// Where Google's pages that turn an API on live. A page anywhere else is
/// never opened.
const CONSOLES: [&str; 2] = [
    "https://console.developers.google.com/",
    "https://console.cloud.google.com/",
];

/// The detail for API `api` (`People API`), turned on at `enable_url`.
pub fn detail(api: &str, enable_url: &str) -> String {
    let api = api.replace(['\r', '\n'], " ");
    format!("{}{SEPARATOR}{enable_url}", api.trim())
}

/// The API's name and the page that turns it on, when `detail` is one
/// made by [`detail`] with a page at Google Cloud.
pub fn parse(detail: &str) -> Option<(&str, &str)> {
    let (api, url) = detail.split_once(SEPARATOR)?;
    (!api.is_empty() && is_console_page(url)).then_some((api, url))
}

/// Whether `url` is a page at Google Cloud that [`parse`] would open.
pub fn is_console_page(url: &str) -> bool {
    CONSOLES.iter().any(|console| url.starts_with(console)) && !url.contains(char::is_whitespace)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_only_google_cloud_pages() {
        let url = "https://console.developers.google.com/apis/api/people.googleapis.com/overview?project=1";
        let made = detail("People API", url);
        assert_eq!(parse(&made), Some(("People API", url)));
        assert_eq!(parse(&detail("Evil", "https://example.com/x")), None);
        assert_eq!(parse("the Google Tasks API is not enabled"), None);
        assert_eq!(parse(&detail("", url)), None);
        assert!(is_console_page(url));
        assert!(!is_console_page(
            "https://console.developers.google.com.evil/x"
        ));
    }
}
