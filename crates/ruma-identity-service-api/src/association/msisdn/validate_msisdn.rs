//! `POST /_matrix/identity/*/validate/msisdn/submitToken`
//!
//! Validate the ownership of a phone number.

pub mod v2 {
    //! `/v2/` ([spec])
    //!
    //! [spec]: https://spec.matrix.org/latest/identity-service-api/#post_matrixidentityv2validatemsisdnsubmittoken

    use ruma_common::{
        api::{request, response, Metadata},
        metadata, OwnedClientSecret, OwnedSessionId,
    };

    const METADATA: Metadata = metadata! {
        method: POST,
        rate_limited: false,
        authentication: AccessToken,
        history: {
            1.0 => "/_matrix/identity/v2/validate/msisdn/submitToken",
        }
    };

    /// Request type for the `validate_msisdn` endpoint.
    #[request]
    pub struct Request {
        /// The session ID, generated by the `requestToken` call.
        pub sid: OwnedSessionId,

        /// The client secret that was supplied to the `requestToken` call.
        pub client_secret: OwnedClientSecret,

        /// The token generated by the `requestToken` call and sent to the user.
        pub token: String,
    }

    /// Response type for the `validate_msisdn` endpoint.
    #[response]
    pub struct Response {
        /// Whether the validation was successful or not.
        pub success: bool,
    }

    impl Request {
        /// Create a new `Request` with the given session ID, client secret and token.
        pub fn new(sid: OwnedSessionId, client_secret: OwnedClientSecret, token: String) -> Self {
            Self { sid, client_secret, token }
        }
    }

    impl Response {
        /// Create a new `Response` with the success status.
        pub fn new(success: bool) -> Self {
            Self { success }
        }
    }
}
