/// An implementation of [`poll_ready`]() that always signals readiness.
///
/// This should only be used for basic leaf services that have no concept of un-readiness.
/// For wrapper or other serivice types, use [`forward_ready!`] for simple cases or write a bespoke
/// `poll_ready` implementation.
///
/// [`poll_ready`]: crate::Service::poll_ready
///
/// # Examples
/// ```no_run
/// use actix_service::Service;
/// use futures_util::future::{ready, Ready};
///
/// struct IdentityService;
///
/// impl Service<u32> for IdentityService {
///     type Response = u32;
///     type Error = ();
///     type Future = Ready<Result<Self::Response, Self::Error>>;
///
///     actix_service::always_ready!();
///
///     fn call(&self, req: u32) -> Self::Future {
///         ready(Ok(req))
///     }
/// }
/// ```
#[macro_export]
macro_rules! always_ready {
    () => {
        #[inline]
        fn poll_ready(
            &self,
            _: &mut ::core::task::Context<'_>,
        ) -> ::core::task::Poll<Result<(), Self::Error>> {
            ::core::task::Poll::Ready(Ok(()))
        }
    };
}

/// An implementation of [`poll_ready`] that forwards readiness checks to a
/// named struct field.
///
/// Tuple structs are not supported.
///
/// [`poll_ready`]: crate::Service::poll_ready
///
/// # Examples
/// ```no_run
/// use actix_service::Service;
/// use futures_util::future::{ready, Ready};
///
/// struct WrapperService<S> {
///     inner: S,
/// }
///
/// impl<S> Service<()> for WrapperService<S>
/// where
///     S: Service<()>,
/// {
///     type Response = S::Response;
///     type Error = S::Error;
///     type Future = S::Future;
///
///     actix_service::forward_ready!(inner);
///
///     fn call(&self, req: ()) -> Self::Future {
///         self.inner.call(req)
///     }
/// }
/// ```
#[macro_export]
macro_rules! forward_ready {
    ($field:ident) => {
        #[inline]
        fn poll_ready(
            &self,
            cx: &mut ::core::task::Context<'_>,
        ) -> ::core::task::Poll<Result<(), Self::Error>> {
            self.$field
                .poll_ready(cx)
                .map_err(::core::convert::Into::into)
        }
    };
}
