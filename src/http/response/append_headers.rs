use super::{IntoResponse, IntoResponseParts, Response, ResponseParts, TryIntoHeaderError};
use crate::utils::macros::impl_deref;
use http::header::{HeaderName, HeaderValue};
use std::fmt;

/// Append headers to a response.
///
/// Returning something like `[("content-type", "foo=bar")]` from a handler will override any
/// existing `content-type` headers. If instead you want to append headers, use `AppendHeaders`:
///
/// ```rust
/// use rama::{
///     http::response::{AppendHeaders, IntoResponse},
///     http::header::SET_COOKIE,
/// };
///
/// async fn handler() -> impl IntoResponse {
///     // something that sets the `set-cookie` header
///     let set_some_cookies = /* ... */
///     # rama::http::HeaderMap::new();
///
///     (
///         set_some_cookies,
///         // append two `set-cookie` headers to the response
///         // without overriding the ones added by `set_some_cookies`
///         AppendHeaders([
///             (SET_COOKIE, "foo=bar"),
///             (SET_COOKIE, "baz=qux"),
///         ])
///     )
/// }
/// ```
pub struct AppendHeaders<I>(pub I);

impl<I: fmt::Debug> fmt::Debug for AppendHeaders<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("AppendHeaders").field(&self.0).finish()
    }
}

impl<I: Clone> Clone for AppendHeaders<I> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl_deref!(AppendHeaders);

impl<I, K, V> IntoResponse for AppendHeaders<I>
where
    I: IntoIterator<Item = (K, V)>,
    K: TryInto<HeaderName>,
    K::Error: fmt::Display,
    V: TryInto<HeaderValue>,
    V::Error: fmt::Display,
{
    fn into_response(self) -> Response {
        (self, ()).into_response()
    }
}

impl<I, K, V> IntoResponseParts for AppendHeaders<I>
where
    I: IntoIterator<Item = (K, V)>,
    K: TryInto<HeaderName>,
    K::Error: fmt::Display,
    V: TryInto<HeaderValue>,
    V::Error: fmt::Display,
{
    type Error = TryIntoHeaderError<K::Error, V::Error>;

    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        for (key, value) in self.0 {
            let key = key.try_into().map_err(TryIntoHeaderError::key)?;
            let value = value.try_into().map_err(TryIntoHeaderError::value)?;
            res.headers_mut().append(key, value);
        }

        Ok(res)
    }
}
