use crate::ta;
use typenum::TypeArray;

pub trait Reverse<Accumalator: TypeArray = ta![]>: TypeArray {
    type Output: TypeArray;
}
impl<Acc: TypeArray> Reverse<Acc> for ta![] {
    type Output = Acc;
}
impl<Acc: TypeArray, H, T: Reverse<ta![H | Acc]>> Reverse<Acc> for ta![H | T] {
    type Output = <T as Reverse<ta![H | Acc]>>::Output;
}

#[cfg(test)]
mod tests {
    use crate::ta;
    use crate::utils::reverse::Reverse;
    use typenum::assert_type_eq;
    use typenum::{U0, U1, U2, U5};

    #[test]
    fn reverse_type_array() {
        assert_type_eq!(<ta![] as Reverse>::Output, ta![]);
        assert_type_eq!(<ta![U0] as Reverse>::Output, ta![U0]);
        assert_type_eq!(<ta![U0, U1] as Reverse>::Output, ta![U1, U0]);
        assert_type_eq!(<ta![U1, U0] as Reverse>::Output, ta![U0, U1]);
        assert_type_eq!(
            <ta![U0, U2, U1, U5, U1, U1] as Reverse>::Output,
            ta![U1, U1, U5, U1, U2, U0]
        );
    }
}
