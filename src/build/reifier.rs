use std::{collections::HashMap, iter::FusedIterator};

use crate::{
    cas::CasExpr,
    err,
    geometry::{Blade, Blank, Blunt, CanonMap, Honed, Mvect, Shape, ShapeMap, SquareMap},
};
use itertools::{Either, Itertools, MultiProduct};
use proc_macro2::Span;
use quote::format_ident;
use syn::{parse::Parse, punctuated::Punctuated, spanned::Spanned, visit_mut::VisitMut};

use super::BladeValue;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ShapeCast {
    pub attrs: Vec<syn::Attribute>,
    pub shape_id: syn::Ident,
    pub as_token: syn::Token![as],
    pub ty: syn::Type,
}
#[derive(Debug)]
pub struct BuildSpec {
    canons: CanonMap,
    shapes: ShapeMap,
    squares: SquareMap,
}

impl TryFrom<&mut syn::ItemMod> for BuildSpec {
    type Error = syn::Error;
    fn try_from(mod_: &mut syn::ItemMod) -> Result<Self, Self::Error> {
        let mut spec = BuildSpec {
            canons: Default::default(),
            shapes: Default::default(),
            squares: Default::default(),
        };
        let Some((brace, items)) = mod_.content.take() else {
            return Err(err!(mod_, "bare module not supported"));
        };
        // parse reefer related macro items
        let mut new_items = vec![];
        for item in items {
            match item {
                syn::Item::Macro(item) => match item.mac.path.get_ident() {
                    Some(ident) if format_ident!("shape").eq(ident) => {
                        new_items.extend(spec.shapes.expand_item_macro(item)?)
                    }
                    Some(ident) if format_ident!("square").eq(ident) => {
                        new_items.extend(spec.squares.expand_item_macro(item)?)
                    }
                    _ => new_items.push(syn::Item::Macro(item)),
                },
                _ => new_items.push(item),
            }
        }
        mod_.content = Some((brace, new_items));
        // restore the blade invariants for the psuedoscalar
        let Blade::Pos(Honed(frame)) = std::mem::take(&mut spec.squares.1) else {
            unreachable!()
        };
        let ps: Blunt = Blank::Pos(frame).into();
        let ps: Blade = ps.hone(&spec.squares);
        spec.squares.1 = ps;
        // build canonical form lookup
        spec.canons = spec.shapes.clone().into_canon_map(&spec.squares);
        Ok(spec)
    }
}
impl BuildSpec {
    pub fn reify_mod(self, mod_: &mut syn::ItemMod) -> syn::Result<()> {
        let Some((brace, items)) = mod_.content.take() else {
            return Err(err!(mod_, "bare module not supported"));
        };
        let mut new_items = vec![];
        new_items.push(syn::Item::Trait(syn::parse_quote!(
            trait Mv {}
        )));
        for item in items {
            match item {
                syn::Item::Impl(impl_) => {
                    let span = impl_.impl_token.span;
                    let mut is_empty = true;
                    for result_item in self.reify_impl(impl_) {
                        is_empty = false;
                        new_items.push(result_item?);
                    }
                    if is_empty {
                        return Err(err!(
                            span,
                            "no code was generated for this impl, check your shape bounds"
                        ));
                    }
                }
                _ => new_items.push(item),
            }
        }
        mod_.content = Some((brace, new_items));
        Ok(())
    }
    fn reify_impl(&self, mut impl_: syn::ItemImpl) -> impl Iterator<Item = syn::Result<syn::Item>> {
        // get the reify attrs
        let mut shape_binds = vec![];
        let mut other_attrs = vec![];
        let mut verbose = false;
        for attr in impl_.attrs {
            match attr.meta {
                syn::Meta::List(ml) if ml.path.get_ident() == Some(&format_ident!("reify")) => {
                    match ml.parse_args_with(ShapeCast::parse) {
                        Ok(bound) => shape_binds.push(bound),
                        Err(e) => return Either::Left(std::iter::once(Err(e))),
                    }
                }
                syn::Meta::Path(path) if path.get_ident() == Some(&format_ident!("verbose")) => {
                    verbose = true;
                }
                _ => other_attrs.push(attr),
            }
        }
        impl_.attrs = other_attrs;
        if shape_binds.is_empty() {
            // return original impl if no reify attrs found
            Either::Left(std::iter::once(Ok(syn::Item::Impl(impl_))))
        } else {
            // otherwise reify the cartesian product of the type shapes
            let items = Reifier {
                err: None,
                reifiable: true,
                template: impl_,
                reified_types_iter: shape_binds
                    .into_iter()
                    .map(|cast| self.reify_shape_binds(cast))
                    .multi_cartesian_product(),
                reified_types: vec![],
                reified_args: vec![],
                assoc_types: Default::default(),
                rec_shape: None,
                ret_shape: None,
                mv_cas: None,
                squares: &self.squares,
                shapes: &self.shapes,
                canons: &self.canons,
                verbose,
            };
            Either::Right(items.into_iter())
        }
    }
    fn reify_shape_binds(&self, cast: ShapeCast) -> Vec<(syn::Type, Shape)> {
        self.shapes
            .shapes(&cast.shape_id)
            .map(move |shape| (cast.ty.clone(), shape))
            .collect()
    }
}

#[derive(Debug)]
struct Reifier<'a> {
    err: Option<syn::Error>,
    reifiable: bool,
    template: syn::ItemImpl,
    reified_types_iter: MultiProduct<std::vec::IntoIter<(syn::Type, Shape)>>,
    reified_types: Vec<(syn::Type, Shape)>,
    reified_args: Vec<(syn::Ident, Shape)>,
    rec_shape: Option<Shape>,
    ret_shape: Option<Shape>,
    assoc_types: HashMap<syn::Ident, syn::Type>,
    squares: &'a SquareMap,
    shapes: &'a ShapeMap,
    canons: &'a CanonMap,
    mv_cas: Option<Mvect<'a, CasExpr>>,
    verbose: bool,
}
impl Iterator for Reifier<'_> {
    type Item = syn::Result<syn::Item>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut impl_ = self.template.clone();
            self.reified_types = self.reified_types_iter.next()?;
            self.visit_item_impl_mut(&mut impl_);
            if self.reifiable {
                return Some(self.err.take().map_or(Ok(syn::Item::Impl(impl_)), Err));
            }
            self.reifiable = true;
        }
    }
}
impl FusedIterator for Reifier<'_> {}

macro_rules! unwrap_or_err {
    ($elf:expr, $expr:expr) => {
        match $expr {
            Ok(result) => result,
            Err(e) => return $elf.err = Some(e),
        }
    };
}
macro_rules! unwrap_or_ret {
    ($expr:expr) => {
        match $expr {
            Some(option) => option,
            None => return,
        }
    };
}
// todo should also pick up literals?
macro_rules! visit_mvect {
    ($elf:expr, $expr:expr) => {{
        $elf.visit_expr_mut($expr);
        unwrap_or_ret!($elf.mv_cas.take())
    }};
}
macro_rules! visit_bin {
    ($elf:expr, $op:ident, $lhs:expr, $rhs:expr) => {{
        let lhs = visit_mvect!($elf, $lhs);
        let rhs = visit_mvect!($elf, $rhs);
        $elf.mv_cas = Some(lhs.$op(rhs));
    }};
}
macro_rules! visit_un {
    ($elf:expr, $op:ident, $rec:expr) => {{
        let lhs = visit_mvect!($elf, $rec);
        $elf.mv_cas = Some(lhs.$op());
    }};
}
macro_rules! visit_lit_op {
    ($elf:expr, $op:ident, $lhs:expr, $rhs:expr) => {{
        let lhs = visit_mvect!($elf, $lhs);
        let n: usize = match $rhs {
            syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Int(n),
                ..
            }) => match n.base10_parse() {
                Ok(n) => n,
                Err(e) => return $elf.err = Some(e),
            },
            expr => return $elf.err = Some(err!(expr, "expected usize")),
        };
        $elf.mv_cas = Some(lhs.$op(n));
    }};
}
macro_rules! visit_duality {
    ($elf:expr, $meth:expr, $fun:ident) => {{
        match $meth.args.first() {
            None => {
                let mv = visit_mvect!($elf, $meth.receiver.as_mut());
                $elf.mv_cas = Some(mv.$fun($elf.squares.1.clone()))
            }
            Some(syn::Expr::Path(syn::ExprPath { path, .. })) => {
                let ps: Blank = match path.get_ident().unwrap().to_string().as_str().parse() {
                    Ok(ps) => ps,
                    Err(e) => return $elf.err = Some(e),
                };
                let ps: Blunt = ps.into();
                let ps = ps.hone($elf.squares);
                let mv = visit_mvect!($elf, $meth.receiver.as_mut());
                $elf.mv_cas = Some(mv.$fun(ps))
            }
            _ => return $elf.err = Some(err!($meth.args, "unrecognized psuedoscalar")),
        }
    }};
}
impl VisitMut for Reifier<'_> {
    fn visit_fn_arg_mut(&mut self, arg: &mut syn::FnArg) {
        match arg {
            syn::FnArg::Receiver(_) => self.save_receiver_arg(),
            syn::FnArg::Typed(pat_ty) => self.save_pat_ty_arg(pat_ty),
        }
        syn::visit_mut::visit_fn_arg_mut(self, arg);
    }
    fn visit_type_mut(&mut self, ty: &mut syn::Type) {
        if let Some(shape) = self.get_reified_shape(ty) {
            *ty = shape.clone().into()
        }
    }
    fn visit_item_impl_mut(&mut self, impl_: &mut syn::ItemImpl) {
        self.save_receiver_shape(&impl_.self_ty);
        self.save_assoc_types(&impl_.items);
        syn::visit_mut::visit_item_impl_mut(self, impl_);
        self.update_assoc_types(&mut impl_.items);
    }
    fn visit_impl_item_fn_mut(&mut self, impl_fn: &mut syn::ImplItemFn) {
        syn::visit_mut::visit_impl_item_fn_mut(self, impl_fn);
        self.save_return_type(&impl_fn.sig.output);
        self.reified_args.clear(); // arg cache only valid per function
    }
    fn visit_stmt_mut(&mut self, stmt: &mut syn::Stmt) {
        self.ret_shape = None;
        let span = stmt.span();
        match stmt {
            syn::Stmt::Local(_local) => todo!(),
            syn::Stmt::Expr(expr, semi_token) => {
                self.visit_expr_mut(expr);
                if semi_token.is_none() {
                    self.save_return_shape(span);
                    if let Some(mv_cas) = self.mv_cas.take() {
                        *expr = unwrap_or_err!(self, mv_cas.try_into_expr(self.canons, span))
                    }
                }
            }
            syn::Stmt::Macro(stmt_mac) => {
                syn::visit_mut::visit_stmt_macro_mut(self, stmt_mac);
                if stmt_mac.semi_token.is_none() {
                    self.save_return_shape(span);
                    if let Some(mv_cas) = self.mv_cas.take() {
                        let expr = unwrap_or_err!(self, mv_cas.try_into_expr(self.canons, span));
                        *stmt = syn::Stmt::Expr(expr, stmt_mac.semi_token);
                    }
                }
            }
            _ => syn::visit_mut::visit_stmt_mut(self, stmt),
        }
    }
    fn visit_expr_mut(&mut self, expr: &mut syn::Expr) {
        let span = expr.span();
        match expr {
            syn::Expr::Array(expr) => expr.elems.iter_mut().for_each(|elem| {
                *elem = unwrap_or_err!(
                    self,
                    visit_mvect!(self, elem).try_into_expr(self.canons, span)
                )
            }),
            syn::Expr::Assign(_) => todo!(),
            syn::Expr::If(_) => todo!(),
            syn::Expr::Return(syn::ExprReturn {
                expr: Some(expr), ..
            }) => {
                let expr = expr.as_mut();
                *expr = unwrap_or_err!(
                    self,
                    visit_mvect!(self, expr).try_into_expr(self.canons, span)
                )
            }
            syn::Expr::Field(expr_field) => {
                let mut mv = visit_mvect!(self, expr_field.base.as_mut());
                let syn::Member::Named(ident) = &expr_field.member else {
                    unimplemented!()
                };
                let blank: Blank = unwrap_or_err!(self, ident.clone().try_into());
                let blade: Blade = Blunt::from(blank).hone(self.squares);
                let value = unwrap_or_ret!(mv.take(&blade));
                *expr = unwrap_or_err!(self, value.try_into())
            }
            syn::Expr::MethodCall(expr) => {
                let rec = expr.receiver.as_mut();
                match expr.method.to_string().as_str() {
                    "commutate" => visit_bin!(self, commutate, rec, &mut expr.args[0]),
                    "anticomm" => visit_bin!(self, anticomm, rec, &mut expr.args[0]),
                    "sandwich" => visit_bin!(self, sandwich, rec, &mut expr.args[0]),
                    "inverse" => visit_un!(self, inv, rec),
                    "grade" => visit_lit_op!(self, grade, rec, &mut expr.args[0]),
                    "pow" => visit_lit_op!(self, pow, rec, &mut expr.args[0]),
                    "norm" => visit_un!(self, norm, rec),
                    "normed" => visit_un!(self, normed, rec),
                    "exp" => visit_un!(self, exp, rec),
                    "log" => visit_un!(self, log, rec),
                    "sqrt" => visit_un!(self, sqrt, rec),
                    "add" => visit_bin!(self, add, rec, &mut expr.args[0]),
                    "sub" => visit_bin!(self, sub, rec, &mut expr.args[0]),
                    "mul" => visit_bin!(self, mul, rec, &mut expr.args[0]),
                    "wedge" => visit_bin!(self, wedge, rec, &mut expr.args[0]),
                    "regressive" => visit_bin!(self, regressive, rec, &mut expr.args[0]),
                    "dot" => visit_bin!(self, dot, rec, &mut expr.args[0]),
                    "fat_dot" => visit_bin!(self, fat_dot, rec, &mut expr.args[0]),
                    "div" => visit_bin!(self, div, rec, &mut expr.args[0]),
                    "ldiv" => visit_bin!(self, ldiv, rec, &mut expr.args[0]),
                    "lcontract" => visit_bin!(self, lcontract, rec, &mut expr.args[0]),
                    "rcontract" => visit_bin!(self, rcontract, rec, &mut expr.args[0]),
                    "dual" => visit_duality!(self, expr, dual),
                    "undual" => visit_duality!(self, expr, undual),
                    "ldual" => visit_duality!(self, expr, ldual),
                    "lundual" => visit_duality!(self, expr, lundual),
                    "neg" => visit_un!(self, neg, rec),
                    "aut" => visit_un!(self, aut, rec),
                    "rev" => visit_un!(self, rev, rec),
                    "conj" => visit_un!(self, conj, rec),
                    "simplify" => visit_un!(self, simplify, rec),
                    _ => self.err = Some(err!(expr.method, "Unrecognized method")),
                }
            }
            syn::Expr::Index(_expr) => todo!(),
            syn::Expr::Unary(expr) => match expr.op {
                syn::UnOp::Neg(_) => visit_un!(self, neg, expr.expr.as_mut()),
                syn::UnOp::Not(_) => {
                    let mv = visit_mvect!(self, expr.expr.as_mut());
                    self.mv_cas = Some(mv.dual(self.squares.1.clone()))
                }
                _ => unimplemented!(),
            },
            syn::Expr::Binary(expr) => {
                let lhs = expr.left.as_mut();
                let rhs = expr.right.as_mut();
                match expr.op {
                    syn::BinOp::Add(_) => visit_bin!(self, add, lhs, rhs),
                    syn::BinOp::AddAssign(_) => unimplemented!(),
                    syn::BinOp::And(_) => unimplemented!(),
                    syn::BinOp::BitAnd(_) => visit_bin!(self, regressive, lhs, rhs),
                    syn::BinOp::BitAndAssign(_) => unimplemented!(),
                    syn::BinOp::BitOr(_) => visit_bin!(self, fat_dot, lhs, rhs),
                    syn::BinOp::BitOrAssign(_) => unimplemented!(),
                    syn::BinOp::BitXor(_) => visit_bin!(self, wedge, lhs, rhs),
                    syn::BinOp::BitXorAssign(_) => unimplemented!(),
                    syn::BinOp::Div(_) => visit_bin!(self, div, lhs, rhs),
                    syn::BinOp::DivAssign(_) => unimplemented!(),
                    syn::BinOp::Eq(_) => todo!(),
                    syn::BinOp::Ge(_) => unimplemented!(),
                    syn::BinOp::Gt(_) => unimplemented!(),
                    syn::BinOp::Le(_) => unimplemented!(),
                    syn::BinOp::Lt(_) => unimplemented!(),
                    syn::BinOp::Mul(_) => visit_bin!(self, mul, lhs, rhs),
                    syn::BinOp::MulAssign(_) => unimplemented!(),
                    syn::BinOp::Ne(_) => todo!(),
                    syn::BinOp::Rem(_) => visit_bin!(self, sandwich, lhs, rhs),
                    syn::BinOp::RemAssign(_) => unimplemented!(),
                    syn::BinOp::Shl(_) => visit_bin!(self, lcontract, lhs, rhs),
                    syn::BinOp::ShlAssign(_) => unimplemented!(),
                    syn::BinOp::Shr(_) => visit_bin!(self, rcontract, lhs, rhs),
                    syn::BinOp::ShrAssign(_) => unimplemented!(),
                    syn::BinOp::Sub(_) => visit_bin!(self, sub, lhs, rhs),
                    syn::BinOp::SubAssign(_) => unimplemented!(),
                    _ => unimplemented!(),
                }
            }
            syn::Expr::Path(expr) => {
                let ident = unwrap_or_ret!(expr.path.get_ident()).clone();
                let shape = unwrap_or_ret!(self.get_reified_arg(&ident)).clone();
                self.mv_cas = Some(shape.into_mv_cas(ident, self.squares))
            }
            expr => syn::visit_mut::visit_expr_mut(self, expr),
        }
    }
    fn visit_macro_mut(&mut self, mac: &mut syn::Macro) {
        match mac.path.get_ident() {
            Some(id) if format_ident!("mv").eq(id) => {
                let parsed = mac.parse_body_with(
                    Punctuated::<BladeValue, syn::Token![,]>::parse_separated_nonempty,
                );
                let body = unwrap_or_err!(self, parsed);
                let mv = unwrap_or_err!(self, Mvect::from_iter(body.into_iter(), self.squares));
                self.mv_cas = Some(mv)
            }
            Some(id) if format_ident!("Mv").eq(id) => todo!(),
            _ => (),
        }
    }
}

impl Reifier<'_> {
    fn save_return_shape(&mut self, span: Span) {
        if let Some(mv_cas) = self.mv_cas.clone() {
            let shape = unwrap_or_err!(self, mv_cas.try_into_shape(self.canons, span));
            self.ret_shape = Some(shape) // update the return type
        }
    }
    fn save_receiver_shape(&mut self, ty: &syn::Type) {
        self.rec_shape = self.get_reified_shape(ty).cloned()
    }
    fn save_receiver_arg(&mut self) {
        if let Some(shape) = self.rec_shape.clone() {
            self.reified_args.push((format_ident!("self"), shape))
        }
    }
    fn save_pat_ty_arg(&mut self, pat_ty: &mut syn::PatType) {
        if let Some(shape) = self.get_reified_shape(&pat_ty.ty) {
            if let syn::Pat::Ident(pat) = pat_ty.pat.as_ref() {
                self.reified_args.push((pat.ident.clone(), shape.clone()))
            }
        }
    }
    fn save_return_type(&mut self, output: &syn::ReturnType) {
        let Some(shape) = self.ret_shape.take() else {
            return; // no return shape found
        };
        let reified_ty = shape.clone().into();
        let syn::ReturnType::Type(_, box_ty) = output else {
            return self.err = Some(err!(output, "missing return type"));
        };
        match box_ty.as_ref() {
            syn::Type::Path(ty) => {
                if ty.path.segments.len() == 2 && ty.path.segments[0].ident == format_ident!("Self")
                {
                    let ty_id = ty.path.segments[1].ident.clone();
                    match self.assoc_types.insert(ty_id, reified_ty) {
                        None => (),
                        Some(syn::Type::ImplTrait(ty)) => {
                            let is_reifiable = ty
                                .bounds
                                .into_iter()
                                .flat_map(|bound| match bound {
                                    syn::TypeParamBound::Trait(bound) => {
                                        bound.path.get_ident().map(|id| {
                                            self.shapes.family(id).any(|fam| fam.contains(&shape))
                                        })
                                    }
                                    _ => unimplemented!(),
                                })
                                .any(|x| x);
                            if !is_reifiable {
                                self.reifiable = false;
                                if self.verbose {
                                    println!("shape not found: {}", shape)
                                }
                            }
                        }
                        Some(ty) => return self.err = Some(err!(ty, "")),
                    };
                }
            }
            _ => (),
        }
    }

    fn save_assoc_types(&mut self, items: &[syn::ImplItem]) {
        for item in items {
            match item {
                syn::ImplItem::Type(syn::ImplItemType { ident, ty, .. }) => {
                    self.assoc_types.insert(ident.clone(), ty.clone());
                }
                _ => (),
            }
        }
    }
    fn update_assoc_types(&mut self, it: &mut [syn::ImplItem]) {
        for item in it {
            match item {
                syn::ImplItem::Type(syn::ImplItemType { ident, ty, .. }) => {
                    if let Some(assoc_ty) = self.assoc_types.remove(&ident) {
                        *ty = assoc_ty
                    }
                }
                _ => (),
            }
        }
    }

    fn get_reified_shape(&self, ty: &syn::Type) -> Option<&Shape> {
        self.reified_types
            .iter()
            .find_map(|(t, s)| ty.eq(t).then_some(s))
    }
    fn get_reified_arg(&self, ident: &syn::Ident) -> Option<&Shape> {
        self.reified_args
            .iter()
            .find_map(|(id, ty)| (id == ident).then_some(ty))
    }
}
