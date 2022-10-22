use std::{fmt::Debug, hash::Hash, sync::Arc};

use crate::{
    cast::{DowncastFrom, UpcastFrom},
    fold::Fold,
    grammar::{Lt, Ty},
    parse::Parse,
};

pub trait Term:
    Clone
    + Fold
    + Parse
    + Ord
    + Eq
    + Hash
    + Debug
    + UpcastFrom<Self>
    + DowncastFrom<Self>
    + 'static
    + Sized
{
}

impl<T: Term> Term for Vec<T> {}

impl<T: Term> Term for Option<T> {}

impl<T: Term> Term for Arc<T> {}

impl Term for Ty {}
crate::self_from_term_impl!(Ty);

impl Term for Lt {}
crate::self_from_term_impl!(Lt);

impl Term for usize {}
crate::self_from_term_impl!(usize);

impl Term for u32 {}
crate::self_from_term_impl!(u32);

impl<A: Term, B: Term> Term for (A, B) {}

impl<A: Term, B: Term, C: Term> Term for (A, B, C) {}
