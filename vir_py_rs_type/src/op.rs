use crate::base::{Value, ValueContainer, ValueKind, Downcast};
use crate::builtin::{VirPyFloat, VirPyInt};
use bumpalo::Bump;
use std::ops::Add;


type OpFn = for<'ctx> fn(
    lhs: Value<'ctx>,
    rhs: Value<'ctx>,
    arena: &'ctx Bump,
) -> Option<Value<'ctx>>;

#[macro_export]
macro_rules! __register_op {
    (
        $lhs_type:ty,
        $rhs_type:ty,
        $func:expr,
        $output_wrapper:path,
        $impl_path:path
    ) => {
        const _: () = {
            fn _op_impl<'ctx>(
                lhs: $crate::base::Value<'ctx>,
                rhs: $crate::base::Value<'ctx>,
                arena: &'ctx ::bumpalo::Bump,
            ) -> Option<$crate::base::Value<'ctx>> {
                let lhs_val = <$lhs_type as $crate::base::Downcast>::from_value(lhs)?;
                let rhs_val = <$rhs_type as $crate::base::Downcast>::from_value(rhs)?;
                let result = $func(lhs_val.clone(), rhs_val.clone());
                Some($crate::base::ValueContainer::new($output_wrapper(result), arena))
            }

            ::inventory::submit! {
                $impl_path { function: _op_impl }
            }
        };
    };
}

pub struct OpAddImpl { pub function: OpFn }
inventory::collect!(OpAddImpl);

pub fn op_add<'ctx>(lhs: Value<'ctx>, rhs: Value<'ctx>, arena: &'ctx Bump) -> Option<Value<'ctx>> {
    for implementation in inventory::iter::<OpAddImpl> {
        if let Some(result) = (implementation.function)(lhs, rhs, arena) {
            return Some(result);
        }
    }
    None
}

#[macro_export]
macro_rules! register_op_add {
    ($lhs_type:ty, $rhs_type:ty, $output_wrapper:path) => {
        $crate::__register_op!(
            $lhs_type,
            $rhs_type,
            |a, b| a + b,
            $output_wrapper,
            $crate::op::OpAddImpl
        );
    };
}

register_op_add!(VirPyInt,   VirPyInt,   ValueKind::Int);
register_op_add!(VirPyFloat, VirPyFloat, ValueKind::Float);
