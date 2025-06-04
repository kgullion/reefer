use super::{
    Battery, Blade, Blank, Blunt, Canon, CanonMap, Honed, Mvect, Shape, ShapeFamily, ShapeMap,
    SquareMap,
};
use crate::cas::CasExpr;
use itertools::{Either, Itertools, Powerset};
use std::{collections::BTreeMap, iter::FilterMap, ops::Not};

impl ShapeMap {
    pub fn shapes(&self, id: &syn::Ident) -> impl Iterator<Item = Shape> {
        self.family(id).flatten()
    }
    pub fn family(&self, id: &syn::Ident) -> impl Iterator<Item = ShapeFamily> {
        match self.0.get(id) {
            Some(family) => Either::Left(family.clone().into_iter()),
            None => Either::Right(std::iter::empty()),
        }
    }
    pub fn into_canon_map(self, squares: &SquareMap) -> CanonMap {
        CanonMap(
            self.0
                .into_values()
                .flatten()
                .flatten()
                .map(|shape| (shape.clone().into_battery(squares), shape))
                .collect(),
        )
    }
}
impl Shape {
    pub fn into_mv_cas(self, ident: syn::Ident, squares: &SquareMap) -> Mvect<CasExpr> {
        self.0
            .into_iter()
            .fold(Mvect(BTreeMap::new(), squares), |mv, blank| {
                let value = CasExpr::var(format!("{ident}__{blank}"));
                let blade = Blunt::hone(blank.into(), squares);
                mv.add_blade_value(blade, value)
            })
    }
    pub fn into_battery(self, squares: &SquareMap) -> Battery {
        self.into_iter()
            .map(|bl| match Blunt::from(bl).hone(squares) {
                Blade::Zero => unreachable!(),
                Blade::Pos(canon) | Blade::Neg(canon) => canon,
            })
            .collect()
    }
}
impl ShapeFamily {
    pub fn contains(&self, shape: &Shape) -> bool {
        if shape.0.is_empty() {
            return false;
        }
        match self {
            Self::Shape(s) => s == shape,
            Self::Powerset(ps) => {
                let mut ps_iter = ps.0.iter();
                for bl in &shape.0 {
                    loop {
                        match ps_iter.next() {
                            Some(ps_bl) if ps_bl == bl => break,
                            None => return false,
                            _ => (),
                        }
                    }
                }
                true
            }
        }
    }
}
// impl IntoIterator for ShapeMap {
//     type IntoIter = Flatten<Flatten<hash_map::IntoValues<syn::Ident, Vec<ShapeFamily>>>>;
//     type Item = Shape;
//     fn into_iter(self) -> Self::IntoIter {
//         self.0.into_values().flatten().flatten()
//     }
// }
impl IntoIterator for ShapeFamily {
    type IntoIter = Either<
        std::iter::Once<Self::Item>,
        FilterMap<Powerset<std::vec::IntoIter<Blank>>, fn(Vec<Blank>) -> Option<Shape>>,
    >;
    type Item = Shape;
    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::Shape(shape) => Either::Left(std::iter::once(shape)),
            Self::Powerset(shape) => Either::Right(
                shape
                    .into_iter()
                    .powerset()
                    .filter_map(|v| v.is_empty().not().then_some(Shape(v))),
            ),
        }
    }
}
impl IntoIterator for Shape {
    type IntoIter = std::vec::IntoIter<Self::Item>;
    type Item = Blank;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl FromIterator<Blank> for Shape {
    fn from_iter<T: IntoIterator<Item = Blank>>(iter: T) -> Self {
        Shape(iter.into_iter().collect())
    }
}
impl FromIterator<Canon> for Battery {
    fn from_iter<T: IntoIterator<Item = Canon>>(iter: T) -> Self {
        Honed(iter.into_iter().sorted().dedup().collect())
    }
}
