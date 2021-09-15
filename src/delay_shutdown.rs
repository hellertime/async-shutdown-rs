use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::DelayShutdownToken;

/// Future wrapper that delays shutdown completion until the wrapper future completes.
pub struct DelayShutdown<F> {
	pub(crate) delay_token: Option<DelayShutdownToken>,
	pub(crate) future: F,
}

impl<F: Future> Future for DelayShutdown<F> {
	type Output = F::Output;

	#[inline]
	fn poll(self: Pin<&mut Self>, context: &mut Context) -> Poll<Self::Output> {
		// SAFETY: We never move `future`, so we can not violate the requirements of `F`.
		unsafe {
			let me = self.get_unchecked_mut();
			match Pin::new_unchecked(&mut me.future).poll(context) {
				Poll::Pending => Poll::Pending,
				Poll::Ready(value) => {
					me.delay_token = None;
					Poll::Ready(value)
				},
			}
		}
	}
}
