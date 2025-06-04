use super::{
    Axis, Battery, Blade, Blank, Blunt, Canon, ConstOne, Honed, Shape, ShapeFamily, ShapeMap,
    Sorted, SquareMap, Squared,
};
use crate::err;
use itertools::{Either, Itertools, chain};
use quote::format_ident;
use std::{
    fmt::{Display, Write},
    str::FromStr,
};
use syn::{parse::Parse, parse_quote, punctuated::Punctuated, visit::Visit};

impl Parse for Shape {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: syn::Ident = input.parse()?;
        ident.to_string().parse()
    }
}

impl FromStr for Shape {
    type Err = syn::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut segments = s.split('_');
        let mut inner = vec![];
        if segments.next() != Some("Mv") {
            return Err(err!("unrecognized shape named"));
        }
        for seg in segments {
            inner.push(seg.parse()?)
        }
        Ok(Shape(inner))
    }
}

struct ShapeMacroBody {
    family_id: syn::Ident,
    #[allow(unused)]
    comma: syn::Token![,],
    families: Punctuated<ShapeFamily, syn::Token![,]>,
}
impl Parse for ShapeMacroBody {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            family_id: input.parse()?,
            comma: input.parse()?,
            families: Punctuated::<ShapeFamily, syn::Token![,]>::parse_separated_nonempty(input)?,
        })
    }
}
impl Parse for ShapeFamily {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let path: syn::Path = input.parse()?;
        let mut visitor = ShapeGatherer::default();
        visitor.visit_path(&path);
        match visitor {
            ShapeGatherer { err: Some(e), .. } => Err(e),
            ShapeGatherer { fam: Some(fam), .. } => Ok(fam),
            _ => Err(err!(path, "not a shape family pattern")),
        }
    }
}

#[derive(Debug, Default)]
struct ShapeGatherer {
    err: Option<syn::Error>,
    fam: Option<ShapeFamily>,
}
impl Visit<'_> for ShapeGatherer {
    fn visit_path(&mut self, path: &syn::Path) {
        let segment = match path.segments.iter().exactly_one() {
            Ok(segment) => segment,
            Err(e) => return self.err = Some(err!(path, e.to_string())),
        };
        match segment.ident.to_string().as_str() {
            "Mv" => {
                self.fam = Some(ShapeFamily::Shape(Shape(vec![])));
                self.visit_path_segment(&segment);
            }
            "Powerset" => {
                self.visit_path_segment(&segment);
                match &self.fam {
                    Some(ShapeFamily::Shape(inner)) => {
                        self.fam = Some(ShapeFamily::Powerset(inner.clone()))
                    }
                    Some(_) => self.err = Some(err!("not supported")),
                    None => (),
                }
            }
            basis => match self.fam.as_mut() {
                Some(ShapeFamily::Shape(Shape(inner))) => match basis.parse() {
                    Ok(blunt) => inner.push(blunt),
                    Err(e) => self.err = Some(e),
                },
                _ => (),
            },
        }
    }
}

impl ShapeMap {
    pub fn expand_item_macro(
        &mut self,
        item: syn::ItemMacro,
    ) -> syn::Result<impl Iterator<Item = syn::Item>> {
        if item.mac.path.get_ident() != Some(&format_ident!("shape")) {
            return Ok(Either::Left(std::iter::empty()));
        }
        let ShapeMacroBody {
            family_id: fam_id,
            families,
            ..
        } = item.mac.parse_body()?;
        let extended_family = self.0.entry(fam_id.clone()).or_default();
        let not_mv = fam_id != format_ident!("Mv");
        let mv_trait = not_mv.then_some(syn::Item::Trait(parse_quote!(trait #fam_id {})));
        let items = families
            .into_iter()
            .flat_map(|family| {
                extended_family.push(family.clone());
                family
            })
            .flat_map(move |shape| {
                let attrs = item.attrs.clone();
                let shape_id = format_ident!("{shape}");
                let blades = shape.0.iter().map(|b| format_ident!("{b}"));
                chain!(
                    not_mv.then_some(syn::Item::Impl(parse_quote!(impl #fam_id for #shape_id {}),)),
                    Some(syn::Item::Impl(parse_quote!(impl Mv for #shape_id {}))),
                    Some(syn::Item::Struct(
                        parse_quote!(#(#attrs)* pub struct #shape_id { #(pub #blades: Field,)* })
                    ))
                )
            });
        Ok(Either::Right(chain!(mv_trait, items)))
    }
}

struct SquareMacroBody {
    axis: Axis,
    #[allow(unused)]
    comma: syn::Token![,],
    square: Squared,
}
impl Parse for SquareMacroBody {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: syn::Ident = input.parse()?;
        Ok(Self {
            axis: ident.to_string().parse()?,
            comma: input.parse()?,
            square: match (
                input.parse::<Option<syn::Token![-]>>()?.is_some(),
                input.parse::<syn::LitInt>()?.base10_parse()?,
            ) {
                (true, 1) => -Squared::One,
                (false, 0) => Squared::Zero,
                (false, 1) => Squared::One,
                (true, n) => Err(err!(input, format!("-{n} is not a valid basis square")))?,
                (false, n) => Err(err!(input, format!("{n} is not a valid basis square")))?,
            },
        })
    }
}
impl SquareMap {
    pub fn expand_item_macro(
        &mut self,
        item: syn::ItemMacro,
    ) -> syn::Result<impl Iterator<Item = syn::Item>> {
        if item.mac.path.get_ident() == Some(&format_ident!("square")) {
            let SquareMacroBody { axis, square, .. } = item.mac.parse_body()?;
            if self.0.insert(axis, square).is_some() {
                return Err(err!(item, "duplicate squares defined"));
            }
            if let Blade::Pos(Honed(frame)) = &mut self.1 {
                frame.push(axis) // note: potentially violates sorted invariant, invariant is restored before use
            } else {
                unreachable!()
            }
        }
        Ok(std::iter::empty()) // currently no code is generated here, maybe a const in the future?
    }
}

fn frame_fmt(frame: &[Axis], f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    if frame.is_empty() {
        return f.write_str("scalar");
    }
    let mut prefix = '~'; // any non-alphanumeric char will do here
    for axis in frame {
        if prefix != axis.0 {
            prefix = axis.0;
            f.write_char(axis.0)?;
        }
        f.write_char(axis.1)?;
    }
    Ok(())
}
impl Display for Canon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        frame_fmt(&self.0, f)
    }
}
impl Display for Blank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Zero => return f.write_str("Zero"),
            Self::Pos(frame) => frame_fmt(frame, f),
            Self::Neg(frame) => {
                f.write_char('N')?;
                frame_fmt(frame, f)
            }
        }
    }
}
impl Display for Blunt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Zero => return f.write_str("Zero"),
            Self::Pos(Sorted(frame)) => frame_fmt(frame, f),
            Self::Neg(Sorted(frame)) => {
                f.write_char('N')?;
                frame_fmt(frame, f)
            }
        }
    }
}
impl Display for Blade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Zero => return f.write_str("Zero"),
            Self::Pos(Honed(frame)) => frame_fmt(frame, f),
            Self::Neg(Honed(frame)) => {
                f.write_char('N')?;
                frame_fmt(frame, f)
            }
        }
    }
}

impl Display for Battery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Mv")?;
        for bl in self.0.iter() {
            f.write_char('_')?;
            bl.fmt(f)?;
        }
        Ok(())
    }
}
impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Mv")?;
        for bl in self.0.iter() {
            f.write_char('_')?;
            bl.fmt(f)?;
        }
        Ok(())
    }
}

impl From<Shape> for syn::Type {
    fn from(value: Shape) -> Self {
        Self::Path(syn::TypePath {
            qself: None,
            path: format_ident!("{value}").into(),
        })
    }
}

impl Display for Axis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.0)?;
        f.write_char(self.1)
    }
}

impl FromStr for Axis {
    type Err = syn::Error;
    fn from_str(s: &str) -> syn::Result<Self> {
        let blank: Blank = s.parse()?;
        match blank {
            Blank::Pos(frame) if frame.len() == 1 => Ok(frame[0]),
            _ => Err(err!("not an axis")),
        }
    }
}

impl FromStr for Blank {
    type Err = syn::Error;
    fn from_str(s: &str) -> syn::Result<Self> {
        let mut chars = s.chars().peekable();
        let mut parity = false;
        match chars.peek() {
            None => return Err(err!("cannot create blade from empty string")),
            Some('Z') | Some('0') => return Ok(Blank::Zero),
            Some('1') => return Ok(Blank::One),
            Some('N') | Some('-') => {
                parity ^= true;
                chars.next();
                if matches!(chars.peek(), Some('1')) {
                    return Ok(-Blank::One);
                }
            }
            Some(_) => (),
        }
        let mut frame = vec![];
        while let Some(prefix) = chars.next() {
            if !prefix.is_ascii_lowercase() {
                return Err(err!(format!("unrecognized blade prefix '{prefix}'")));
            }
            while chars
                .peek()
                .is_some_and(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
            {
                let index = chars.next().unwrap();
                frame.push(Axis(prefix, index));
            }
        }
        match parity {
            false => Ok(Blank::Pos(frame)),
            true => Ok(Blank::Neg(frame)),
        }
    }
}

impl TryFrom<syn::Ident> for Blank {
    type Error = syn::Error;
    fn try_from(value: syn::Ident) -> syn::Result<Self> {
        value.to_string().parse()
    }
}
