use std::{
    cmp,
    collections::{BTreeMap, HashMap},
};

mod blade;
mod mvect;
mod parse;
mod shape;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Oriented<T> {
    Pos(T),
    Zero,
    Neg(T),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// marker trait for "is sorted, unique, and elements are in canonical form"
pub struct Honed<T: ?Sized + Clone>(pub(super) T);
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// marker trait for "is sorted"
pub struct Sorted<T: ?Sized>(pub(super) T);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Axis(char, char);

// pub type Vector = Oriented<Axis>;
pub type Frame = Vec<Axis>;
pub type Blank = Oriented<Frame>;
pub type Blunt = Oriented<Sorted<Frame>>;
pub type Blade = Oriented<Honed<Frame>>;
pub type Squared = Oriented<()>;
pub type Canon = Honed<Frame>;

impl Ord for Canon {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.0.len().cmp(&other.0.len()).then(self.0.cmp(&other.0))
    }
}
impl PartialOrd for Canon {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub type Battery = Honed<Vec<Canon>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shape(Vec<Blank>);

#[derive(Debug, Clone)]
pub enum ShapeFamily {
    Shape(Shape),
    Powerset(Shape),
}

#[derive(Debug, Clone, Default)]
pub struct CanonMap(HashMap<Battery, Shape>);

#[derive(Debug, Clone, Default)]
pub struct ShapeMap(HashMap<syn::Ident, Vec<ShapeFamily>>);

#[derive(Debug, Clone)]
pub struct SquareMap(HashMap<Axis, Squared>, pub Blade);
impl Default for SquareMap {
    fn default() -> Self {
        Self(Default::default(), Blade::One)
    }
}

#[derive(Debug, Clone)]
pub struct Mvect<'a, T>(BTreeMap<Canon, T>, &'a SquareMap);

pub trait ConstOne {
    #[allow(non_upper_case_globals)]
    const One: Self;
}
pub trait Zero {
    fn zero() -> Self;
}
pub trait One {
    fn one() -> Self;
}
