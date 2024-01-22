use clar2wasm::tools::crosscheck;
use proptest::proptest;
use proptest::strategy::Strategy;

use crate::{prop_signature, PropValue};

proptest! {
    #[test]
    fn if_true_returns_first_value(
        (v1, v2) in prop_signature()
            .prop_flat_map(|ty| {
                (PropValue::from_type(ty.clone()), PropValue::from_type(ty))
            })
        )
    {
        crosscheck(
            &format!(r#"(if true {v1} {v2})"#),
            Ok(Some(v1.into()))
        )
    }
}

proptest! {
    #[test]
    fn if_false_returns_second_value(
        (v1, v2) in prop_signature()
            .prop_flat_map(|ty| {
                (PropValue::from_type(ty.clone()), PropValue::from_type(ty))
            })
        )
    {
        crosscheck(
            &format!(r#"(if false {v1} {v2})"#),
            Ok(Some(v2.into()))
        )
    }
}
