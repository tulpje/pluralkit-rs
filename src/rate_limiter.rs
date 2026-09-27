use std::time::Duration;

#[cfg(test)]
use mock_instant::thread_local::{SystemTime, UNIX_EPOCH};
#[cfg(not(test))]
use std::time::{SystemTime, UNIX_EPOCH};

use reqwest::header::HeaderMap;

use crate::Error;

#[derive(Debug)]
pub(crate) struct RateLimitData {
    pub(crate) limit: u16,
    pub(crate) remaining: u16,
    pub(crate) reset: u64,
    pub(crate) scope: String,
}

// see https://pluralkit.me/api/#rate-limiting
pub(crate) fn parse_ratelimit_headers(headers: &HeaderMap) -> Result<RateLimitData, Error> {
    let Some(limit_value) = headers.get("X-RateLimit-Limit") else {
        return Err("couldn't parse rate limit headers, missing X-RateLimit-Limit".into());
    };
    let Some(remaining_value) = headers.get("X-RateLimit-Remaining") else {
        return Err("couldn't parse rate limit headers, missing X-RateLimit-Remaining".into());
    };
    let Some(reset_value) = headers.get("X-RateLimit-Reset") else {
        return Err("couldn't parse rate limit headers, missing X-RateLimit-Reset".into());
    };
    let Some(scope_value) = headers.get("X-RateLimit-Scope") else {
        return Err("couldn't parse rate limit headers, missing X-RateLimit-Scope".into());
    };

    let limit: u16 = limit_value.to_str()?.parse()?;
    let remaining: u16 = remaining_value.to_str()?.parse()?;
    let reset: u64 = reset_value.to_str()?.parse()?;
    let scope: String = scope_value.to_str()?.to_string();

    Ok(RateLimitData {
        limit,
        remaining,
        reset,
        scope,
    })
}

// TODO: Somehow handle each bucket separately?
//       need to know buckets beforehand, or track them as they come in
//       allowing parallel requests but only per bucket
pub(crate) fn handle_ratelimit_headers(headers: &HeaderMap) -> Option<Duration> {
    let rate_limit_data = parse_ratelimit_headers(headers);
    match rate_limit_data {
        Ok(rate_limit_data) => {
            // TODO: debug log ratelimit data
            if rate_limit_data.remaining == 0 {
                let current_timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("SystemTime::now is before unix epoch, exiting as that indicates a bigger issue")
                    .as_millis() as u64;

                return Some(Duration::from_millis(
                    rate_limit_data.reset.saturating_sub(current_timestamp),
                ));
            }
        }
        Err(err) => {
            tracing::info!("couldn't parse rate limit data, ignoring: {err}");
        }
    };

    None
}

#[cfg(test)]
mod test {
    use http::{HeaderName, HeaderValue};
    use mock_instant::thread_local::MockClock;

    use super::*;

    const LIMIT_HEADER: (HeaderName, HeaderValue) = (
        HeaderName::from_static("x-ratelimit-limit"),
        HeaderValue::from_static("5"),
    );
    const REMAINING_HEADER: (HeaderName, HeaderValue) = (
        HeaderName::from_static("x-ratelimit-remaining"),
        HeaderValue::from_static("0"),
    );
    const RESET_HEADER: (HeaderName, HeaderValue) = (
        HeaderName::from_static("x-ratelimit-reset"),
        HeaderValue::from_static("1234567"),
    );
    const SCOPE_HEADER: (HeaderName, HeaderValue) = (
        HeaderName::from_static("x-ratelimit-scope"),
        HeaderValue::from_static("generic_get"),
    );

    // parse_ratelimit_headers
    #[test]
    fn test_parse_ratelimit_headers_parses_correctly() {
        let headers =
            HeaderMap::from_iter([LIMIT_HEADER, REMAINING_HEADER, RESET_HEADER, SCOPE_HEADER]);

        let parsed_headers = parse_ratelimit_headers(&headers).expect("should parse correctly");

        assert_eq!(parsed_headers.limit, 5);
        assert_eq!(parsed_headers.remaining, 0);
        assert_eq!(parsed_headers.reset, 1234567);
        assert_eq!(parsed_headers.scope, "generic_get");
    }

    #[test]
    fn test_parse_ratelimit_headers_errors_when_missing() {
        assert!(
            parse_ratelimit_headers(&HeaderMap::from_iter([
                REMAINING_HEADER,
                RESET_HEADER,
                SCOPE_HEADER
            ]))
            .is_err()
        );
        assert!(
            parse_ratelimit_headers(&HeaderMap::from_iter([
                LIMIT_HEADER,
                RESET_HEADER,
                SCOPE_HEADER
            ]))
            .is_err()
        );
        assert!(
            parse_ratelimit_headers(&HeaderMap::from_iter([
                LIMIT_HEADER,
                REMAINING_HEADER,
                SCOPE_HEADER
            ]))
            .is_err()
        );
        assert!(
            parse_ratelimit_headers(&HeaderMap::from_iter([
                LIMIT_HEADER,
                REMAINING_HEADER,
                RESET_HEADER,
            ]))
            .is_err()
        );
    }

    // handle_ratelimiting
    #[test]
    fn handle_ratelimiting_none_below_limit() {
        let headers = HeaderMap::from_iter([
            (
                HeaderName::from_static("x-ratelimit-limit"),
                HeaderValue::from_static("5"),
            ),
            (
                HeaderName::from_static("x-ratelimit-remaining"),
                HeaderValue::from_static("1"),
            ),
            (
                HeaderName::from_static("x-ratelimit-reset"),
                HeaderValue::from_static("1000"),
            ),
            (
                HeaderName::from_static("x-ratelimit-scope"),
                HeaderValue::from_static("generic_get"),
            ),
        ]);

        assert!(handle_ratelimit_headers(&headers).is_none())
    }
    #[test]
    fn handle_ratelimiting_returns_correct_duration_when_remaining_zero() {
        MockClock::set_time(Duration::ZERO);

        let headers = HeaderMap::from_iter([
            (
                HeaderName::from_static("x-ratelimit-limit"),
                HeaderValue::from_static("5"),
            ),
            (
                HeaderName::from_static("x-ratelimit-remaining"),
                HeaderValue::from_static("0"),
            ),
            (
                HeaderName::from_static("x-ratelimit-reset"),
                HeaderValue::from_static("1000"),
            ),
            (
                HeaderName::from_static("x-ratelimit-scope"),
                HeaderValue::from_static("generic_get"),
            ),
        ]);

        let Some(duration) = handle_ratelimit_headers(&headers) else {
            panic!("handle_ratelimiting should return Duration");
        };

        assert_eq!(duration.as_millis(), 1_000);
    }
}
