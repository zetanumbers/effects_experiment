// WIP

use core::{
    future::{self, Future, IntoFuture},
    pin::Pin,
    task::{Context, Poll},
};

use crate::wrap;

pub trait Exchange {
    type Exchanged;

    fn exchange(self) -> Self::Exchanged;
}

impl<T, E1, E2> Exchange for Result<Result<T, E1>, E2> {
    type Exchanged = Result<Result<T, E2>, E1>;

    fn exchange(self) -> Self::Exchanged {
        match self {
            Ok(Ok(t)) => Ok(Ok(t)),
            Ok(Err(e)) => Err(e),
            Err(e) => Ok(Err(e)),
        }
    }
}

impl<F, E> Exchange for Result<wrap::Awaitable<F>, E> {
    type Exchanged = wrap::Awaitable<IntoResultFuture<F, E>>;

    fn exchange(self) -> Self::Exchanged {
        wrap::Awaitable(IntoResultFuture(self.map(|t| t.0)))
    }
}

pub struct IntoResultFuture<F, E>(Result<F, E>);

impl<F, E> IntoFuture for IntoResultFuture<F, E>
where
    F: IntoFuture,
{
    type Output = Result<F::Output, E>;

    type IntoFuture = ResultFuture<F::IntoFuture, E>;

    fn into_future(self) -> Self::IntoFuture {
        ResultFuture(match self.0 {
            Ok(f) => Ok(f.into_future()),
            Err(e) => Err(future::ready(e)),
        })
    }
}

pub struct ResultFuture<F, E>(Result<F, future::Ready<E>>);

impl<F, E> Future for ResultFuture<F, E>
where
    F: Future,
{
    type Output = Result<F::Output, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        unsafe {
            let this = self.get_unchecked_mut();
            match &mut this.0 {
                Ok(f) => Pin::new_unchecked(f).poll(cx).map(Ok),
                Err(e) => Pin::new_unchecked(e).poll(cx).map(Err),
            }
        }
    }
}
