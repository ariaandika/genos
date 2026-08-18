use crate::fd::{AsRawFd, BorrowedFd};

pub(crate) trait IntoArg {
    fn into_arg(self) -> usize;
}

macro_rules! impl_into_arg_cast {
    ($($me:ty),*) => {$(
        impl IntoArg for $me {
            fn into_arg(self) -> usize {
                self as _
            }
        }
    )*};
}
impl_into_arg_cast!(u32, i32);

impl IntoArg for usize {
    fn into_arg(self) -> usize {
        self
    }
}

impl IntoArg for BorrowedFd<'_> {
    fn into_arg(self) -> usize {
        self.as_raw_fd() as _
    }
}

impl<T> IntoArg for &T {
    fn into_arg(self) -> usize {
        self as *const T as _
    }
}

impl<T> IntoArg for &mut T {
    fn into_arg(self) -> usize {
        self as *mut T as _
    }
}

impl<T> IntoArg for &[T] {
    fn into_arg(self) -> usize {
        self.as_ptr() as _
    }
}

impl<T> IntoArg for &mut [T] {
    fn into_arg(self) -> usize {
        self.as_mut_ptr() as _
    }
}

impl<T> IntoArg for *const T {
    fn into_arg(self) -> usize {
        self as _
    }
}

impl<T> IntoArg for *mut T {
    fn into_arg(self) -> usize {
        self as _
    }
}
