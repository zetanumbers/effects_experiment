use core::future::IntoFuture as CoreIntoFuture;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Awaitable<F>(pub F);

impl<F> CoreIntoFuture for Awaitable<F>
where
    F: CoreIntoFuture,
{
    type Output = F::Output;

    type IntoFuture = F::IntoFuture;

    fn into_future(self) -> Self::IntoFuture {
        self.0.into_future()
    }
}

pub trait IntoFutureExt: Sized {
    fn wrap_awaitable(self) -> Awaitable<Self>;
}

impl<T> IntoFutureExt for T
where
    T: CoreIntoFuture,
{
    fn wrap_awaitable(self) -> Awaitable<Self> {
        Awaitable(self)
    }
}
