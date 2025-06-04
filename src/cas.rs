use crate::{
    err,
    geometry::{One, Zero},
    traits::Squareroot,
};
pub use cas_compute::symbolic::expr::{Primary, SymExpr};
use itertools::Itertools;
use quote::{ToTokens, format_ident};
use std::{
    fmt::{Display, Write},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CasExpr(SymExpr);

fn cas_into_syn(cas: SymExpr) -> syn::Result<syn::Expr> {
    match cas {
        SymExpr::Primary(Primary::Integer(_) | Primary::Float(_)) => {
            Ok(syn::Expr::Cast(syn::ExprCast {
                attrs: Default::default(),
                expr: Box::new(syn::parse_str(cas.to_string().as_str())?),
                as_token: Default::default(),
                ty: syn::parse_quote!(Field),
            }))
        }
        SymExpr::Primary(_) => Ok(syn::parse_str(cas.to_string().replace("__", ".").as_str())?),
        SymExpr::Add(exprs) => match exprs.into_iter().map(cas_into_syn).process_results(|it| {
            it.reduce(|a, b| {
                syn::Expr::Binary(syn::ExprBinary {
                    attrs: Default::default(),
                    left: Box::new(a),
                    op: syn::BinOp::Add(Default::default()),
                    right: Box::new(b),
                })
            })
        }) {
            Ok(None) => todo!(), // need a "zero" value for syn::Expr
            Ok(Some(expr)) => Ok(expr),
            Err(e) => Err(e),
        },
        SymExpr::Mul(exprs) => match exprs.into_iter().map(cas_into_syn).process_results(|it| {
            it.reduce(|a, b| {
                syn::Expr::Binary(syn::ExprBinary {
                    attrs: Default::default(),
                    left: Box::new(a),
                    op: syn::BinOp::Mul(Default::default()),
                    right: Box::new(b),
                })
            })
        }) {
            Ok(None) => todo!(), // need a "zero" value for syn::Expr
            Ok(Some(expr)) => Ok(expr),
            Err(e) => Err(e),
        },
        SymExpr::Exp(base, power) => cas_exp_into_syn(*base, *power),
    }
}
fn cas_exp_into_syn(base: SymExpr, power: SymExpr) -> syn::Result<syn::Expr> {
    match power {
        // _ => {
        //     let ex: syn::Expr = syn::parse_str(power.to_string().as_str())?;
        //     Ok(syn::Expr::MethodCall(syn::ExprMethodCall {
        //         attrs: Default::default(),
        //         receiver: Box::new(cas2syn(base)?),
        //         dot_token: Default::default(),
        //         turbofish: None,
        //         method: format_ident!("pow"),
        //         paren_token: Default::default(),
        //         args: std::iter::once(ex).collect(),
        //     }))
        // }
        _ => Ok(syn::Expr::MethodCall(syn::ExprMethodCall {
            attrs: Default::default(),
            receiver: Box::new(cas_into_syn(base)?),
            dot_token: Default::default(),
            turbofish: None,
            method: format_ident!("pow"),
            paren_token: Default::default(),
            args: std::iter::once(cas_into_syn(power)?).collect(),
        })),
    }
}

impl TryFrom<CasExpr> for syn::Expr {
    type Error = syn::Error;
    fn try_from(value: CasExpr) -> syn::Result<Self> {
        cas_into_syn(value.0)
    }
}
impl TryFrom<syn::Expr> for CasExpr {
    type Error = syn::Error;
    fn try_from(value: syn::Expr) -> Result<Self, Self::Error> {
        use cas_parser::parser::{Parser, ast::Expr};

        match Parser::new(value.to_token_stream().to_string().as_str()).try_parse_full::<Expr>() {
            Ok(expr) => Ok(CasExpr(expr.into())),
            Err(_) => Err(err!("unrecognized cas expression")),
        }
    }
}

impl CasExpr {
    pub fn simplify(&self) -> Self {
        Self(cas_compute::symbolic::simplify(&self.0))
    }
}

impl CasExpr {
    pub fn int<T: Into<rug::Integer>>(n: T) -> CasExpr {
        CasExpr(cas_compute::symbolic::expr::SymExpr::Primary(
            cas_compute::symbolic::expr::Primary::Integer(n.into()),
        ))
    }
    pub fn var<T: ToString>(name: T) -> CasExpr {
        CasExpr(SymExpr::Primary(Primary::Symbol(name.to_string())))
    }
}

impl Display for CasExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('(')?;
        self.0.fmt(f)?;
        f.write_char(')')
    }
}

impl Zero for CasExpr {
    fn zero() -> Self {
        Self::int(0)
    }
}
impl One for CasExpr {
    fn one() -> Self {
        Self::int(1)
    }
}

impl Add for CasExpr {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        CasExpr(self.0 + rhs.0)
    }
}
impl AddAssign for CasExpr {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}
impl Div for CasExpr {
    type Output = CasExpr;
    fn div(self, rhs: Self) -> Self::Output {
        let recip = SymExpr::Exp(Box::new(rhs.0), Box::new(-Self::int(1).0));
        CasExpr(self.0 * recip)
    }
}
impl DivAssign for CasExpr {
    fn div_assign(&mut self, rhs: Self) {
        let recip = SymExpr::Exp(Box::new(rhs.0), Box::new(-Self::int(1).0));
        self.0 *= recip;
    }
}
impl Mul for CasExpr {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        CasExpr(self.0 * rhs.0)
    }
}
impl MulAssign for CasExpr {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}
impl Squareroot for CasExpr {
    type Output = Self;
    fn sqrt(self) -> Self::Output {
        Self(self.0.sqrt())
    }
}
impl Neg for CasExpr {
    type Output = Self;
    fn neg(self) -> Self::Output {
        CasExpr(-self.0)
    }
}
impl Sub for CasExpr {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        CasExpr(self.0 + -rhs.0)
    }
}
impl SubAssign for CasExpr {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 += -rhs.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let expr = CasExpr::int(42);
        assert_eq!(format!("{}", expr), "(42)");

        let x = CasExpr::var("x");
        assert_eq!(format!("{}", x), "(x)");

        let sum = x.clone() + CasExpr::int(5);
        assert_eq!(format!("{}", sum), "(x + 5)");
    }

    #[test]
    fn test_zero_one() {
        let zero = CasExpr::zero();
        assert_eq!(zero, CasExpr::int(0));

        let one = CasExpr::one();
        assert_eq!(one, CasExpr::int(1));
    }

    #[test]
    fn test_equality() {
        let a = CasExpr::int(5);
        let b = CasExpr::int(5);
        let c = CasExpr::int(6);

        assert_eq!(a, b);
        assert_ne!(a, c);

        let x = CasExpr::var("x");
        let y = CasExpr::var("x");
        let z = CasExpr::var("y");

        assert_eq!(x, y);
        assert_ne!(x, z);
    }

    #[test]
    fn test_add() {
        let a = CasExpr::int(5);
        let b = CasExpr::int(7);
        assert_eq!(a + b, CasExpr::int(12));
    }

    #[test]
    fn test_add_assign() {
        let mut a = CasExpr::int(5);
        a += CasExpr::int(7);
        assert_eq!(a, CasExpr::int(12));
    }

    #[test]
    fn test_sub() {
        let a = CasExpr::int(10);
        let b = CasExpr::int(3);
        assert_eq!(a.clone() - b.clone(), CasExpr::int(7));
        assert_eq!(b.clone() - a.clone(), CasExpr::int(-7));
        assert_eq!(b.clone() - a.clone(), -CasExpr::int(7));
    }

    #[test]
    fn test_sub_assign() {
        let mut a = CasExpr::int(10);
        a -= CasExpr::int(3);
        assert_eq!(a, CasExpr::int(7));
    }

    #[test]
    fn test_neg() {
        assert_ne!(CasExpr::int(5), CasExpr::int(-5));
        assert_eq!(-CasExpr::int(5), CasExpr::int(-5));
        assert_eq!(CasExpr::int(5), -CasExpr::int(-5));
        assert_eq!(CasExpr::int(0), -CasExpr::int(0));
    }

    #[test]
    fn test_mul() {
        let lhs = CasExpr::var("x") + CasExpr::var("x");
        let rhs = CasExpr::int(2) * CasExpr::var("x");
        assert_eq!(lhs.simplify(), rhs.simplify())
    }

    #[test]
    fn test_mul_assign() {
        let lhs = CasExpr::var("x") + CasExpr::var("x");
        let mut x = CasExpr::var("x");
        x *= CasExpr::int(2);
        assert_eq!(lhs.simplify(), x.simplify())
    }

    #[test]
    fn test_div() {
        let x = CasExpr::var("x");
        let lhs = x.clone();
        let rhs = (x.clone() + x.clone()) / CasExpr::int(2);
        assert_eq!(lhs.simplify(), rhs.simplify())
    }

    #[test]
    fn test_div_assign() {
        let x = CasExpr::var("x");
        let lhs = x.clone();
        let mut rhs = x.clone() + x.clone();
        rhs /= CasExpr::int(2);
        assert_eq!(lhs.simplify(), rhs.simplify())
    }

    #[test]
    fn test_asym() {
        let a_e0 = CasExpr::var("a__e0");
        let a_e1 = CasExpr::var("a__e1");
        let b_e0 = CasExpr::var("b__e0");
        let b_e1 = CasExpr::var("b__e1");
        let x = a_e0 * b_e1 - a_e1 * b_e0;
        println!("{}", x.simplify())
    }
}
