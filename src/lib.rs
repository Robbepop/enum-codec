#![no_std]

use enum_ref::{EnumMut, EnumRef};
use core::marker::PhantomData;

/// Implemented by Rust `enum` types that allow for space-efficient encoding.
pub trait EnumCodec: EnumRef + EnumMut {
    /// Generated type used to encode instances of `enum` type `Self`.
    type Encoder: Encode<Item = Self> + Decode + DecodeMut;
}

/// Trait implemented by generated `enum` encoders to encode items.
pub trait Encode {
    /// The key type used to uniquely identify encoded items.
    type Key;

    /// The type of the `enum` that is encoded.
    type Item;

    /// Encodes the `enum` value and returns a reference to it.
    fn encode(&mut self, value: &Self::Item) -> Key<Self, Self::Item>;
}

/// Trait implemented by generated `enum` encoders to return shared references to encoded items.
pub trait Decode: Encode
where
    <Self as Encode>::Item: EnumRef,
{
    /// Decode the encoded `enum` value at `key` and returns a shared reference to it.
    fn decode(&self, eref: Self::Key) -> Option<<Self::Item as EnumRef>::Ref<'_>>;
}

/// Trait implemented by generated `enum` encoders to return exclusive references to encoded items.
pub trait DecodeMut: Encode
where
    <Self as Encode>::Item: EnumMut,
{
    /// Decode the encoded `enum` value at `key` and returns an exclusive reference to it.
    fn decode_mut(&mut self, eref: Self::Key) -> Option<<Self::Item as EnumMut>::Mut<'_>>;
}

/// Trait implemented by generated `enum` encoders to efficiently decode and visit items.
pub trait DecodeVisit<V: Visitor>: Encode {
    /// Decode the encoded `enum` value and calls the respective visit method.
    fn decode_visit(&self, key: Self::Key, visitor: &mut V) -> <V as Visitor>::Output;
}

/// Trait implemented by decode visitors.
pub trait Visitor {
    /// The common output type of all visit methods of the visitor.
    type Output;
}

/// A pointer to an encoded `Item` in an `Encoder`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Key<Encoder: ?Sized, Item: ?Sized> {
    /// The index at which the encoded `Item` resides in the `Encoder`.
    index: RawKey,
    /// Marker to trick the Rust compiler that `Encoder` is in use.
    marker_encoder: PhantomData<fn() -> Encoder>,
    /// Marker to trick the Rust compiler that `Item` is in use.
    marker_item: PhantomData<fn() -> Item>,
}

/// A raw pointer to an encoded `Item` in an `Encoder`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RawKey {
    /// The relative index into the encoded buffer where the encoded item resides.
    index: usize,
}
