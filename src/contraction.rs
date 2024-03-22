use core::{
    future::{Future as CoreFuture, IntoFuture as CoreIntoFuture},
    ops::ControlFlow,
    pin::Pin,
    task::{ready, Context},
};

use crate::wrap;

pub trait Contraction {
    type Contracted;

    fn contract(self) -> Self::Contracted;
}

impl<T> Contraction for Option<Option<T>> {
    type Contracted = Option<T>;

    fn contract(self) -> Self::Contracted {
        self.flatten()
    }
}

impl<T, E> Contraction for Result<Result<T, E>, E> {
    type Contracted = Result<T, E>;

    fn contract(self) -> Self::Contracted {
        self.and_then(core::convert::identity)
    }
}

impl<B, C> Contraction for ControlFlow<B, ControlFlow<B, C>> {
    type Contracted = ControlFlow<B, C>;

    fn contract(self) -> Self::Contracted {
        match self {
            ControlFlow::Continue(ControlFlow::Continue(c)) => ControlFlow::Continue(c),
            ControlFlow::Break(b) | ControlFlow::Continue(ControlFlow::Break(b)) => {
                ControlFlow::Break(b)
            }
        }
    }
}

impl<F> Contraction for wrap::Awaitable<F>
where
    F: CoreIntoFuture,
    F::Output: CoreIntoFuture,
{
    type Contracted = wrap::Awaitable<IntoFuture<F>>;

    fn contract(self) -> Self::Contracted {
        wrap::Awaitable(IntoFuture(self.0))
    }
}

pub struct IntoFuture<F>(pub F);

impl<F> CoreIntoFuture for IntoFuture<F>
where
    F: CoreIntoFuture,
    F::Output: CoreIntoFuture,
{
    type Output = <F::Output as CoreIntoFuture>::Output;

    type IntoFuture = Future<F::IntoFuture>;

    fn into_future(self) -> Self::IntoFuture {
        Future(FutureInner::First(self.0.into_future()))
    }
}

pub struct Future<F>(FutureInner<F>)
where
    F: CoreFuture,
    F::Output: CoreIntoFuture;

enum FutureInner<F>
where
    F: CoreFuture,
    F::Output: CoreIntoFuture,
{
    First(F),
    Second(<F::Output as CoreIntoFuture>::IntoFuture),
}

impl<F> CoreFuture for Future<F>
where
    F: CoreFuture,
    F::Output: CoreIntoFuture,
{
    type Output = <F::Output as CoreIntoFuture>::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> core::task::Poll<Self::Output> {
        unsafe {
            let this = self.get_unchecked_mut();
            loop {
                match &mut this.0 {
                    FutureInner::First(f) => {
                        this.0 = FutureInner::Second(
                            ready!(Pin::new_unchecked(f).poll(cx)).into_future(),
                        );
                    }
                    FutureInner::Second(f) => return Pin::new_unchecked(f).poll(cx),
                }
            }
        }
    }
}
