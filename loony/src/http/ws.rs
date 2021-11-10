//! Websockets protocol helpers
use derive_more::Display;

use crate::http::error::ResponseError;
use crate::http::message::RequestHead;
use crate::http::response::{Response, ResponseBuilder};
use crate::http::{header, Method, StatusCode};

/// Websocket handshake errors
#[derive(PartialEq, Debug, Display)]
pub enum HandshakeError {
    /// Only get method is allowed
    #[display(fmt = "Method not allowed")]
    GetMethodRequired,
    /// Upgrade header if not set to websocket
    #[display(fmt = "Websocket upgrade is expected")]
    NoWebsocketUpgrade,
    /// Connection header is not set to upgrade
    #[display(fmt = "Connection upgrade is expected")]
    NoConnectionUpgrade,
    /// Websocket version header is not set
    #[display(fmt = "Websocket version header is required")]
    NoVersionHeader,
    /// Unsupported websocket version
    #[display(fmt = "Unsupported version")]
    UnsupportedVersion,
    /// Websocket key is not set or wrong
    #[display(fmt = "Unknown websocket key")]
    BadWebsocketKey,
}

impl ResponseError for HandshakeError {
    fn error_response(&self) -> Response {
        match *self {
            HandshakeError::GetMethodRequired => Response::MethodNotAllowed()
                .header(header::ALLOW, "GET")
                .finish(),
            HandshakeError::NoWebsocketUpgrade => Response::BadRequest()
                .reason("No WebSocket UPGRADE header found")
                .finish(),
            HandshakeError::NoConnectionUpgrade => Response::BadRequest()
                .reason("No CONNECTION upgrade")
                .finish(),
            HandshakeError::NoVersionHeader => Response::BadRequest()
                .reason("Websocket version header is required")
                .finish(),
            HandshakeError::UnsupportedVersion => Response::BadRequest()
                .reason("Unsupported version")
                .finish(),
            HandshakeError::BadWebsocketKey => {
                Response::BadRequest().reason("Handshake error").finish()
            }
        }
    }
}

impl ResponseError for crate::ws::ProtocolError {}

/// Verify `WebSocket` handshake request and create handshake reponse.
// /// `protocols` is a sequence of known protocols. On successful handshake,
// /// the returned response headers contain the first protocol in this list
// /// which the server also knows.
pub fn handshake(req: &RequestHead) -> Result<ResponseBuilder, HandshakeError> {
    verify_handshake(req)?;
    Ok(handshake_response(req))
}

/// Verify `WebSocket` handshake request.
// /// `protocols` is a sequence of known protocols. On successful handshake,
// /// the returned response headers contain the first protocol in this list
// /// which the server also knows.
pub fn verify_handshake(req: &RequestHead) -> Result<(), HandshakeError> {
    // WebSocket accepts only GET
    if req.method != Method::GET {
        return Err(HandshakeError::GetMethodRequired);
    }

    // Check for "UPGRADE" to websocket header
    let has_hdr = if let Some(hdr) = req.headers().get(header::UPGRADE) {
        if let Ok(s) = hdr.to_str() {
            s.to_ascii_lowercase().contains("websocket")
        } else {
            false
        }
    } else {
        false
    };
    if !has_hdr {
        return Err(HandshakeError::NoWebsocketUpgrade);
    }

    // Upgrade connection
    if !req.upgrade() {
        return Err(HandshakeError::NoConnectionUpgrade);
    }

    // check supported version
    if !req.headers().contains_key(header::SEC_WEBSOCKET_VERSION) {
        return Err(HandshakeError::NoVersionHeader);
    }
    let supported_ver = {
        if let Some(hdr) = req.headers().get(header::SEC_WEBSOCKET_VERSION) {
            hdr == "13" || hdr == "8" || hdr == "7"
        } else {
            false
        }
    };
    if !supported_ver {
        return Err(HandshakeError::UnsupportedVersion);
    }

    // check client handshake for validity
    if !req.headers().contains_key(header::SEC_WEBSOCKET_KEY) {
        return Err(HandshakeError::BadWebsocketKey);
    }
    Ok(())
}

/// Create websocket's handshake response
///
/// This function returns handshake `Response`, ready to send to peer.
pub fn handshake_response(req: &RequestHead) -> ResponseBuilder {
    let key = {
        let key = req.headers().get(header::SEC_WEBSOCKET_KEY).unwrap();
        crate::ws::hash_key(key.as_ref())
    };

    Response::build(StatusCode::SWITCHING_PROTOCOLS)
        .upgrade("websocket")
        .header(header::TRANSFER_ENCODING, "chunked")
        .header(header::SEC_WEBSOCKET_ACCEPT, key.as_str())
        .take()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::test::TestRequest;
    use crate::http::{header, Method};

    #[test]
    fn test_handshake() {
        let req = TestRequest::default().method(Method::POST).finish();
        assert_eq!(
            HandshakeError::GetMethodRequired,
            verify_handshake(req.head()).err().unwrap()
        );

        let req = TestRequest::default().finish();
        assert_eq!(
            HandshakeError::NoWebsocketUpgrade,
            verify_handshake(req.head()).err().unwrap()
        );

        let req = TestRequest::default()
            .header(header::UPGRADE, header::HeaderValue::from_static("test"))
            .finish();
        assert_eq!(
            HandshakeError::NoWebsocketUpgrade,
            verify_handshake(req.head()).err().unwrap()
        );

        let req = TestRequest::default()
            .header(
                header::UPGRADE,
                header::HeaderValue::from_static("websocket"),
            )
            .finish();
        assert_eq!(
            HandshakeError::NoConnectionUpgrade,
            verify_handshake(req.head()).err().unwrap()
        );

        let req = TestRequest::default()
            .header(
                header::UPGRADE,
                header::HeaderValue::from_static("websocket"),
            )
            .header(
                header::CONNECTION,
                header::HeaderValue::from_static("upgrade"),
            )
            .finish();
        assert_eq!(
            HandshakeError::NoVersionHeader,
            verify_handshake(req.head()).err().unwrap()
        );

        let req = TestRequest::default()
            .header(
                header::UPGRADE,
                header::HeaderValue::from_static("websocket"),
            )
            .header(
                header::CONNECTION,
                header::HeaderValue::from_static("upgrade"),
            )
            .header(
                header::SEC_WEBSOCKET_VERSION,
                header::HeaderValue::from_static("5"),
            )
            .finish();
        assert_eq!(
            HandshakeError::UnsupportedVersion,
            verify_handshake(req.head()).err().unwrap()
        );

        let req = TestRequest::default()
            .header(
                header::UPGRADE,
                header::HeaderValue::from_static("websocket"),
            )
            .header(
                header::CONNECTION,
                header::HeaderValue::from_static("upgrade"),
            )
            .header(
                header::SEC_WEBSOCKET_VERSION,
                header::HeaderValue::from_static("13"),
            )
            .finish();
        assert_eq!(
            HandshakeError::BadWebsocketKey,
            verify_handshake(req.head()).err().unwrap()
        );

        let req = TestRequest::default()
            .header(
                header::UPGRADE,
                header::HeaderValue::from_static("websocket"),
            )
            .header(
                header::CONNECTION,
                header::HeaderValue::from_static("upgrade"),
            )
            .header(
                header::SEC_WEBSOCKET_VERSION,
                header::HeaderValue::from_static("13"),
            )
            .header(
                header::SEC_WEBSOCKET_KEY,
                header::HeaderValue::from_static("13"),
            )
            .finish();
        assert_eq!(
            StatusCode::SWITCHING_PROTOCOLS,
            handshake_response(req.head()).finish().status()
        );
    }

    #[test]
    fn test_wserror_http_response() {
        let resp: Response = HandshakeError::GetMethodRequired.error_response();
        assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED);
        let resp: Response = HandshakeError::NoWebsocketUpgrade.error_response();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let resp: Response = HandshakeError::NoConnectionUpgrade.error_response();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let resp: Response = HandshakeError::NoVersionHeader.error_response();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let resp: Response = HandshakeError::UnsupportedVersion.error_response();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let resp: Response = HandshakeError::BadWebsocketKey.error_response();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
}
