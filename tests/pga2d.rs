#[reefer::algebraic]
mod pga2d {
    use std::ops::{BitAnd, BitOr, BitXor};

    type Field = f32;

    trait Pow {
        fn pow(self, n: Self) -> Self;
    }
    impl Pow for Field {
        fn pow(self, n: Self) -> Self {
            self.powf(n)
        }
    }

    square!(e0, 0);
    square!(e1, 1);
    square!(e2, 1);

    shape!(Scalar, Mv<scalar>);
    // shape!(Line, Mv<e1, e2, e0>);
    shape!(Ideal, Mv<Powerset<e01, e20>>);
    shape!(Point, Mv<e12>, Mv<e01, e12>, Mv<e02, e12>, Mv<e01, e20, e12>);
    shape!(Line, Mv<Powerset<e1, e2, e0>>);
    // shape!(Point, Mv<Powerset<e20, e01, e12>>);

    #[reify(Line as A)]
    #[reify(Line as B)]
    impl BitXor<B> for A {
        type Output = impl Point;
        /// meet two lines into a point
        fn bitxor(self, line: B) -> Self::Output {
            self ^ line
        }
    }

    #[reify(Point as A)]
    #[reify(Point as B)]
    impl BitAnd<B> for A {
        type Output = impl Line;
        /// join two points into a line
        fn bitand(self, point: B) -> Self::Output {
            self & point
        }
    }

    #[reify(Line as L)]
    #[reify(Point as P)]
    impl BitOr<P> for L {
        type Output = impl Line;
        /// line orthogonal to self, through the point other
        fn bitor(self, point: P) -> Self::Output {
            self | point
        }
    }

    pub trait Projection<Rhs> {
        type Output;
        fn project(self, other: Rhs) -> Self::Output;
    }
    #[reify(Point as P)]
    #[reify(Line as L)]
    impl Projection<L> for P {
        type Output = impl Point;
        /// project point onto line
        fn project(self, line: L) -> Self::Output {
            ((line | self) * line).simplify()
        }
    }

    #[reify(Line as L)]
    #[reify(Point as P)]
    impl Projection<P> for L {
        type Output = impl Line;
        /// project line onto point
        fn project(self, point: P) -> Self::Output {
            ((self | point) * point).simplify()
        }
    }

    pub trait Orthogonal {
        type Output;
        fn orthogonal(self) -> Self::Output;
    }
    #[reify(Line as L)]
    impl Orthogonal for L {
        type Output = impl Ideal;
        fn orthogonal(self) -> Self::Output {
            self * mv!(e012: 1.0)
        }
    }

    pub trait Normalize {
        type Output;
        fn normalized(self) -> Self::Output;
        fn norm(self) -> Field;
        fn ideal_norm(self) -> Field;
    }
    #[reify(Line as L)]
    impl Normalize for L {
        type Output = impl Line;
        fn normalized(self) -> Self::Output {
            self.normed()
        }
        fn norm(self) -> Field {
            self.norm().scalar
        }
        fn ideal_norm(self) -> Field {
            self.dual().norm().scalar
        }
    }

    // pub trait Distance<Rhs> {
    //     fn dist(self, rhs: Rhs) -> Field;
    // }
    // #[reify(Point as A)]
    // #[reify(Point as B)]
    // impl Distance<B> for A {
    //     fn dist(self, rhs: B) -> Field {
    //         self.dual()
    //             .normed()
    //             .undual()
    //             .regressive(rhs.dual().normed().undual())
    //             .norm()
    //             .simplify()
    //             .scalar
    //     }
    // }
}
