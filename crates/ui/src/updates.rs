//! The new-version notice. A build given a feed address (`MINIKEYBOARD_UPDATE_FEED`) reads it at launch and once a
//! day while it runs — unless that is switched off in Preferences — and when asked from the menu. A newer version
//! shows a banner with a link to where it is downloaded; nothing is installed, as an app in a Flatpak sandbox can't
//! replace itself. A build without a feed has none of this.
//!
//! The feed is an RSS file of the macOS app's kind: an `<item>` for each version, with its `<sparkle:version>` and a
//! `<link>` to the version's download page.

use std::cmp::Ordering;

use gtk::{gio, glib};
use soup::prelude::*;

/// A version the feed offers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Release {
    pub version: String,
    pub link: String,
}

/// The newest version in a feed, or `None` if it offers none that can be read.
pub fn newest(feed: &str) -> Option<Release> {
    elements(feed, "item")
        .filter_map(|item| {
            let version = elements(item, "sparkle:version").next()?.trim();
            let link = elements(item, "link").next()?.trim();
            numbers(version)?;
            Some(Release { version: unescape(version), link: unescape(link) })
        })
        .max_by(|a, b| compare(&a.version, &b.version))
}

/// Whether `candidate` comes after `current` ("1.0.0" after "0.9.9"; "1.0" and "1.0.0" are the same).
pub fn is_newer(candidate: &str, current: &str) -> bool {
    numbers(candidate).is_some() && numbers(current).is_some() && compare(candidate, current) == Ordering::Greater
}

fn numbers(version: &str) -> Option<Vec<u64>> {
    version.split('.').map(|part| part.parse().ok()).collect()
}

fn compare(a: &str, b: &str) -> Ordering {
    let (a, b) = (numbers(a).unwrap_or_default(), numbers(b).unwrap_or_default());
    (0..a.len().max(b.len()))
        .map(|index| a.get(index).unwrap_or(&0).cmp(b.get(index).unwrap_or(&0)))
        .find(|ordering| ordering.is_ne())
        .unwrap_or(Ordering::Equal)
}

/// The contents of each `<name …>…</name>`, in order. Enough for the feed, which has no nesting of the same name.
fn elements<'a>(xml: &'a str, name: &'a str) -> impl Iterator<Item = &'a str> + 'a {
    let (open, close) = (format!("<{name}"), format!("</{name}>"));
    let mut rest = xml;
    std::iter::from_fn(move || {
        loop {
            let start = rest.find(&open)?;
            let after = &rest[start + open.len()..];
            // `<link>` mustn't match `<linker>`.
            if !after.starts_with(['>', ' ', '\t', '\n', '\r']) {
                rest = after;
                continue;
            }
            let body = &after[after.find('>')? + 1..];
            let end = body.find(&close)?;
            rest = &body[end + close.len()..];
            return Some(&body[..end]);
        }
    })
}

fn unescape(text: &str) -> String {
    text.replace("&lt;", "<").replace("&gt;", ">").replace("&quot;", "\"").replace("&apos;", "'").replace("&amp;", "&")
}

/// Reads the feed over HTTPS (libsoup, from the GNOME runtime).
pub async fn fetch(url: &str, user_agent: &str) -> Result<String, glib::Error> {
    let session = soup::Session::builder().user_agent(user_agent).timeout(30).build();
    let message = soup::Message::new("GET", url)
        .map_err(|error| glib::Error::new(gio::IOErrorEnum::InvalidArgument, &error.to_string()))?;
    let body = session.send_and_read_future(&message, glib::Priority::DEFAULT).await?;
    if message.status() != soup::Status::Ok {
        return Err(glib::Error::new(gio::IOErrorEnum::Failed, &format!("HTTP {:?}", message.status())));
    }
    Ok(String::from_utf8_lossy(&body).into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    const FEED: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<rss xmlns:sparkle="http://www.andymatuschak.org/xml-namespaces/sparkle" version="2.0">
  <channel>
    <title>MINIKeyboard for Linux</title>
    <link>https://example.com/</link>
    <item>
      <title>0.9.9</title>
      <sparkle:version>0.9.9</sparkle:version>
      <link>https://example.com/releases/v0.9.9</link>
    </item>
    <item>
      <title>1.0.0</title>
      <sparkle:version>1.0.0</sparkle:version>
      <link>https://example.com/releases/v1.0.0?a=1&amp;b=2</link>
    </item>
  </channel>
</rss>"#;

    #[test]
    fn the_newest_item_wins_wherever_it_is() {
        let release = newest(FEED).unwrap();
        assert_eq!(release.version, "1.0.0");
        assert_eq!(release.link, "https://example.com/releases/v1.0.0?a=1&b=2");
    }

    #[test]
    fn items_without_a_version_or_a_link_are_skipped() {
        let feed = "<rss><item><link>https://a</link></item><item><sparkle:version>x</sparkle:version>\
                    <link>https://b</link></item><item><sparkle:version>0.1</sparkle:version></item></rss>";
        assert_eq!(newest(feed), None);
        assert_eq!(newest(""), None);
    }

    #[test]
    fn versions_compare_by_number() {
        assert!(is_newer("1.0.0", "0.9.9"));
        assert!(is_newer("0.10.0", "0.9.9"));
        assert!(!is_newer("1.0", "1.0.0"));
        assert!(!is_newer("0.9.9", "1.0.0"));
        assert!(!is_newer("1.0.0-beta", "0.9.9"));
    }
}
