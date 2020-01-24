use crate::bytelevel::{PCons, PNil, ReferenceBytes, slot::*};

/// A marker trait implemented if the layout `T` is compatible with the layout
/// `Self`.
pub unsafe trait FromLayout<T> {}

/// ANYTHING -> []
unsafe impl<T> FromLayout<T> for PNil {}

/// [Bytes|_] -> [Bytes|_]
#[rustfmt::skip] unsafe impl<TKind, TSize, TRest, UKind, USize, URest>
FromLayout<PCons<Bytes<TKind, TSize>, TRest>>
       for PCons<Bytes<UKind, USize>, URest>
where
    crate::TODO:
{}

/// [SharedRef|_] -> [SharedRef|_]
#[rustfmt::skip] unsafe impl<'t, 'u, T, TRest, U, URest>
FromLayout<PCons<SharedRef<'t, T>, TRest>>
       for PCons<SharedRef<'u, U>, URest>
where
    crate::TODO:
{}

/// [UniqueRef|_] -> [SharedRef|_]
#[rustfmt::skip] unsafe impl<'t, 'u, T, TRest, U, URest>
FromLayout<PCons<UniqueRef<'t, T>, TRest>>
       for PCons<SharedRef<'u, U>, URest>
where
    crate::TODO:
{}

/// [UniqueRef|_] -> [UniqueRef|_]
#[rustfmt::skip] unsafe impl<'t, 'u, T, TRest, U, URest>
FromLayout<PCons<UniqueRef<'t, T>, TRest>>
       for PCons<UniqueRef<'u, U>, URest>
where
    crate::TODO:
{}

/// [UniqueRef|_] -> [Bytes|_]
#[rustfmt::skip] unsafe impl<'t, T, TRest, UKind, USize, URest>
FromLayout<PCons<UniqueRef<'t, T>, TRest>>
       for PCons<Bytes<UKind, USize>, URest>
where
    Self: FromLayout<ReferenceBytes<TRest>>
{}

/// [SharedRef|_] -> [Bytes|_]
#[rustfmt::skip] unsafe impl<'t, T, TRest, UKind, USize, URest>
FromLayout<PCons<SharedRef<'t, T>, TRest>>
       for PCons<Bytes<UKind, USize>, URest>
where
    Self: FromLayout<ReferenceBytes<TRest>>
{}
