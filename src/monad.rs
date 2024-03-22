// WIP

pub trait Monad {
    type Map<T>;
    type Holds;
}

impl<T, E> Monad for Result<T, E> {
    type Map<U> = Result<U, E>;
    type Holds = T;
}
