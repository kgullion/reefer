#[reefer::algebraic]
/// Create a timespace Algebra with 1,3 metric.
mod sta {
    use std::ops::{Add, BitAnd, BitOr, BitXor, Rem};

    type Field = f32;

    trait Pow {
        fn pow(self, n: Self) -> Self;
    }
    impl Pow for Field {
        fn pow(self, n: Self) -> Self {
            self.powf(n)
        }
    }

    square!(e1, 1);
    square!(e2, -1);
    square!(e3, -1);
    square!(e4, -1);

    shape!(Time, Mv<e1>);
    shape!(Position, Mv<e2, e3, e4>);
    #[derive(Debug)]
    shape!(Event, Mv<e1, e2, e3, e4>);
    #[derive(Clone)]
    shape!(Frame, Mv<e12, e13, e14>);
    #[derive(Clone)]
    shape!(Motor, Mv<scalar, e12, e13, e14>);

    #[reify(Position as P)]
    #[reify(Time as T)]
    impl Add<P> for T {
        type Output = impl Event;
        fn add(self, rhs: P) -> Self::Output {
            self + rhs
        }
    }

    #[reify(Event as E)]
    #[reify(Motor as F)]
    impl Rem<F> for E {
        type Output = impl Event;
        fn rem(self, rhs: F) -> Self::Output {
            rhs.sandwich(self).simplify()
        }
    }

    pub trait Exp {
        type Output;
        fn exp(self) -> Self::Output;
    }
    #[reify(Frame as F)]
    impl Exp for F {
        type Output = impl Motor;
        /// todo: this is incorrect and only here as a placeholder example.
        /// can use `exp` once it is reifiable, but will need to sort out
        /// runtime range reduction w/ comptime taylor series expansion.
        fn exp(self) -> Self::Output {
            mv![scalar: 1.0] + self
        }
    }
}

use libm::{atanhf, fabsf};
use reefer::mv;

#[test]
/// from https://enkimute.github.io/ganja.js/examples/coffeeshop.html#timespace_lorentz
fn test_lorentz() {
    use sta::*;
    println!(
        "We implement the example from https://enkimute.github.io/ganja.js/examples/coffeeshop.html#timespace_lorentz in a coordinate free way.

In the earth's reference frame, a tree is at the origin and a pole is at x=20km
Lightning strikes both the tree and the pole at t=10microseconds. The lightning
strikes are observed by a rocket traveling in the positive x direction at 0.5c

1. what time does the lightning strikes take place for the rocket.
2. are they still simultanious ?"
    );
    // The spacetime unit we use is lightseconds (for both time and space)
    let micros = |t: f32| mv![e1: t * 0.000001];
    let km =
        |x: f32, y: f32, z: f32| mv![e2: x / 299792.458, e3: y / 299792.458, e4: z / 299792.458];
    // Define two events in our own reference frame. (the earth's)
    let strike_tree = micros(10.0) + km(0.0, 0.0, 0.0);
    let strike_pole = micros(10.0) + km(20.0, 0.0, 0.0);

    // Calculate the lorentz transform to go to the rockets frame.
    let rocket_speed = 0.5; // Half the speed of light.
    let rocket_frame = mv![e12: 0.5 * atanhf(rocket_speed), e13: 0.0, e14: 0.0].exp();

    // Transform the events to the rockets frame.
    let strike_tree_r = strike_tree % rocket_frame.clone();
    let strike_pole_r = strike_pole % rocket_frame;

    // Output our results.
    println!("----");
    println!(
        "rocket sees lightning strike the tree at time = {} ms",
        strike_tree_r.e1 * 1000000.0
    );
    println!(
        "rocket sees lightning strike the pole at time = {} ms",
        strike_pole_r.e1 * 1000000.0
    );
    if fabsf(strike_tree_r.e1 - strike_pole_r.e1) < 0.0000000001 {
        println!("Hey that's at the same time !")
    } else {
        println!("Wow! That's not at the same time!")
    }
    println!("----");
    println!("Lets also do https://www.khanacademy.org/science/physics/special-relativity/lorentz-transformation/v/evaluating-a-lorentz-transformation
what does the timespace event at (1,1) to a static observer look like for an observer
going at 0.5c ?
");
    let event = mv![e1: 1.0, e2: 1.0, e3: 0.0, e4: 0.0];
    let frame = mv![e12: 0.5*atanhf(0.5), e13: 0.0, e14: 0.0].exp();
    let event_in_frame = event % frame;

    println!("event time as seen at 0.5c = {}", event_in_frame.e1);
    println!("event position as seen at 0.5c = {}", event_in_frame.e2);
    println!(
        "\nnote: our answers are currently incorrect because of a naive and incorrect implementation of exp (todo: fix)"
    );
}
