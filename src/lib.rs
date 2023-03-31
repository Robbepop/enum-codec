#![no_std]

pub use enum_ref::{EnumMut, EnumRef};
pub use enum_tag::EnumTag;
use core::marker::PhantomData;

/// Implemented by Rust `enum` types that allow for space-efficient encoding.
pub trait EnumCodec: EnumRef + EnumMut {
    /// Generated type used to encode instances of `enum` type `Self`.
    type Encoder: Encode<Item = Self> + Decode;
}

/// Trait implemented by generated `enum` encoders to encode items.
pub trait Encode {
    /// The key type used to uniquely identify encoded items.
    type Key;

    /// The type of the `enum` that the [`Encoder`] can encode.
    ///
    /// This way the [`Encoder`] can derive the `enum` tag type and others.
    type Item;

    /// Encodes the `enum` value for the [`Encoder`] and returns a reference to it.
    fn encode(&mut self, value: &Self::Item) -> Key<Self, Self::Item>;
}

/// Trait implemented by generated `enum` encoders to decode items.
pub trait Decode: Encode
where
    <Self as Encode>::Item: EnumRef + EnumMut,
{
    /// Decode the `enum` value encoded in the [`Encoder`] and returns a copy of it.
    fn decode(&self, eref: Self::Key) -> Option<<Self::Item as EnumRef>::Ref<'_>>;

    /// Decode the `enum` value encoded in the [`Encoder`] and returns a copy of it.
    fn decode_mut(&mut self, eref: Self::Key) -> Option<<Self::Item as EnumMut>::Mut<'_>>;
}

/// Trait implemented by generated `enum` encoders to efficiently decode and visit items.
pub trait DecodeVisit<V: Visitor>: Encode {
    /// Decode the `enum` value encoded in the [`Encoder`] and returns a copy of it.
    fn decode_visit(&self, key: Self::Key, visitor: &mut V) -> <V as Visitor>::Output;
}

/// Trait implemented by decode visitors.
pub trait Visitor {
    type Output;
}

/// A reference to an encoded `Item` in an `Encoder`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Key<Encoder: ?Sized, Item: ?Sized> {
    /// The index at which the encoded `Item` resides in the `Encoder`.
    index: usize,
    /// Marker to trick the Rust compiler that `Encoder` is in use.
    marker_encoder: PhantomData<fn() -> Encoder>,
    /// Marker to trick the Rust compiler that `Item` is in use.
    marker_item: PhantomData<fn() -> Item>,
}
