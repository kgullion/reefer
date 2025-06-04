use super::{BladeValue, reifier::ShapeCast};
use crate::err;
use crate::geometry::Blank;
use quote::format_ident;
use syn::parse::Parse;

impl Parse for BladeValue {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let syn::FieldValue {
            attrs,
            member: syn::Member::Named(ident),
            colon_token: Some(colon),
            expr,
        } = input.parse()?
        else {
            return Err(err!(input, "unrecognized blade value"));
        };
        let blade: Blank = ident.to_string().parse()?;
        Ok(Self {
            attrs,
            blade,
            colon,
            expr,
        })
    }
}

impl From<BladeValue> for syn::FieldValue {
    fn from(value: BladeValue) -> Self {
        let bl = value.blade;
        Self {
            attrs: value.attrs,
            member: syn::Member::Named(format_ident!("{bl}")),
            colon_token: Some(value.colon),
            expr: value.expr,
        }
    }
}

impl Parse for ShapeCast {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let expr: syn::ExprCast = input.parse()?;
        let syn::Expr::Path(syn::ExprPath { path, .. }) = *expr.expr else {
            todo!()
        };
        let Some(id) = path.get_ident() else { todo!() };
        Ok(ShapeCast {
            attrs: expr.attrs,
            shape_id: id.clone(),
            as_token: expr.as_token,
            ty: *expr.ty,
        })
    }
}
