use core::{
    future::{IntoFuture, Ready},
    pin::Pin,
};

pub trait Lender {
    type Lend<'a>: Lend
    where
        Self: 'a;

    fn lend_next(self: Pin<&mut Self>) -> Self::Lend<'_>;
}

pub trait Lend: IntoFinite {
    type Item;
}

impl<F> Lend for F
where
    F: IntoFinite,
{
    type Item = F::Item;
}

// pub trait ReleaseLendBeforeAwait: Lender {
//     type ReleasedBeforeAwait: IntoFuture;
//     fn release_lend_before_await(x: Self::Next<'_>) -> Self::ReleasedBeforeAwait;
// }

pub trait ReleaseLend: Lender {
    type Released;
    fn release_lend(x: <Self::Lend<'_> as IntoFuture>::Output) -> Self::Released;
}

// impl<I> ReleaseLendAfterAwait for I
// where
//     I: ReleaseLend,
// {
//     type ReleasedAfterAwait = <I::Released as IntoFuture>::Output;

//     fn release_lend_after_await(
//         x: <Self::Next<'_> as IntoFuture>::Output,
//     ) -> Self::ReleasedAfterAwait {
//         Self::release_lend(x)
//     }
// }

// impl<I, T> ReleaseLend<Finite<T>> for I
// where
//     I: ReleaseLend<T>,
// {
//     type Released = Finite<I::Released>;

//     fn release(x: Finite<T>) -> Self::Released {
//         match x {
//             Finite::Next(x) => Finite::Next(Self::release(x)),
//             Finite::End => Finite::End,
//         }
//     }
// }

// TODO: Summary type?
pub enum Finite<T> {
    Next(T),
    End,
}

pub struct Infinite<T> {
    next: T,
}

pub trait IntoFinite {
    type Item;
    fn into_finite(self) -> Finite<Self::Item>;
}

impl<T> IntoFinite for Finite<T> {
    type Item = T;

    fn into_finite(self) -> Finite<Self::Item> {
        self
    }
}

impl<T> IntoFinite for Infinite<T> {
    type Item = T;

    fn into_finite(self) -> Finite<Self::Item> {
        Finite::Next(self.next)
    }
}

pub trait SkipAwait: IntoFuture {
    fn skip_await(self) -> Self::Output;
}

impl<T> SkipAwait for Ready<T> {
    fn skip_await(self) -> Self::Output {
        self.into_inner()
    }
}

pub struct Regularized<I>(I);

// impl<I> Iterator for Regularized<I>
// where
//     I: ReleaseLendBeforeAwait + Unpin,
//     I::ReleasedBeforeAwait: IntoFinite,
// {
//     type Item = <<I as ReleaseLendBeforeAwait>::ReleasedBeforeAwait as IntoFinite>::Item;

//     fn next(&mut self) -> Option<Self::Item> {
//         match I::release_lend_before_await(Pin::new(&mut self.0).lend_next()).into_finite() {
//             Finite::Next(item) => Some(item),
//             Finite::End => None,
//         }
//     }
// }
