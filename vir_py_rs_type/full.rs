#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
pub mod op {
    use crate::base::Value;
    use bumpalo::Bump;
    type BinaryOpFn = for<'ctx> fn(
        lhs: Value<'ctx>,
        rhs: Value<'ctx>,
        arena: &'ctx Bump,
    ) -> Option<Value<'ctx>>;
    type UnaryOpFn = for<'ctx> fn(
        rhs: Value<'ctx>,
        arena: &'ctx Bump,
    ) -> Option<Value<'ctx>>;
    pub struct OpAddImpl {
        pub function: crate::op::BinaryOpFn,
    }
    impl ::inventory::Collect for OpAddImpl {
        #[inline]
        fn registry() -> &'static ::inventory::Registry {
            static REGISTRY: ::inventory::Registry = ::inventory::Registry::new();
            &REGISTRY
        }
    }
    pub fn op_add<'ctx>(
        lhs: crate::base::Value<'ctx>,
        rhs: crate::base::Value<'ctx>,
        arena: &'ctx ::bumpalo::Bump,
    ) -> ::core::option::Option<crate::base::Value<'ctx>> {
        for implementation in ::inventory::iter::<OpAddImpl> {
            if let ::core::option::Option::Some(result) = (implementation
                .function)(lhs, rhs, arena)
            {
                return Some(result);
            }
        }
        None
    }
    pub struct OpSubImpl {
        pub function: crate::op::BinaryOpFn,
    }
    impl ::inventory::Collect for OpSubImpl {
        #[inline]
        fn registry() -> &'static ::inventory::Registry {
            static REGISTRY: ::inventory::Registry = ::inventory::Registry::new();
            &REGISTRY
        }
    }
    pub fn op_sub<'ctx>(
        lhs: crate::base::Value<'ctx>,
        rhs: crate::base::Value<'ctx>,
        arena: &'ctx ::bumpalo::Bump,
    ) -> ::core::option::Option<crate::base::Value<'ctx>> {
        for implementation in ::inventory::iter::<OpSubImpl> {
            if let ::core::option::Option::Some(result) = (implementation
                .function)(lhs, rhs, arena)
            {
                return Some(result);
            }
        }
        None
    }
    pub struct OpMulImpl {
        pub function: crate::op::BinaryOpFn,
    }
    impl ::inventory::Collect for OpMulImpl {
        #[inline]
        fn registry() -> &'static ::inventory::Registry {
            static REGISTRY: ::inventory::Registry = ::inventory::Registry::new();
            &REGISTRY
        }
    }
    pub fn op_mul<'ctx>(
        lhs: crate::base::Value<'ctx>,
        rhs: crate::base::Value<'ctx>,
        arena: &'ctx ::bumpalo::Bump,
    ) -> ::core::option::Option<crate::base::Value<'ctx>> {
        for implementation in ::inventory::iter::<OpMulImpl> {
            if let ::core::option::Option::Some(result) = (implementation
                .function)(lhs, rhs, arena)
            {
                return Some(result);
            }
        }
        None
    }
    pub struct OpDivImpl {
        pub function: crate::op::BinaryOpFn,
    }
    impl ::inventory::Collect for OpDivImpl {
        #[inline]
        fn registry() -> &'static ::inventory::Registry {
            static REGISTRY: ::inventory::Registry = ::inventory::Registry::new();
            &REGISTRY
        }
    }
    pub fn op_div<'ctx>(
        lhs: crate::base::Value<'ctx>,
        rhs: crate::base::Value<'ctx>,
        arena: &'ctx ::bumpalo::Bump,
    ) -> ::core::option::Option<crate::base::Value<'ctx>> {
        for implementation in ::inventory::iter::<OpDivImpl> {
            if let ::core::option::Option::Some(result) = (implementation
                .function)(lhs, rhs, arena)
            {
                return Some(result);
            }
        }
        None
    }
    pub struct OpEqImpl {
        pub function: crate::op::BinaryOpFn,
    }
    impl ::inventory::Collect for OpEqImpl {
        #[inline]
        fn registry() -> &'static ::inventory::Registry {
            static REGISTRY: ::inventory::Registry = ::inventory::Registry::new();
            &REGISTRY
        }
    }
    pub fn op_eq<'ctx>(
        lhs: crate::base::Value<'ctx>,
        rhs: crate::base::Value<'ctx>,
        arena: &'ctx ::bumpalo::Bump,
    ) -> ::core::option::Option<crate::base::Value<'ctx>> {
        for implementation in ::inventory::iter::<OpEqImpl> {
            if let ::core::option::Option::Some(result) = (implementation
                .function)(lhs, rhs, arena)
            {
                return Some(result);
            }
        }
        None
    }
    pub struct OpGeImpl {
        pub function: crate::op::BinaryOpFn,
    }
    impl ::inventory::Collect for OpGeImpl {
        #[inline]
        fn registry() -> &'static ::inventory::Registry {
            static REGISTRY: ::inventory::Registry = ::inventory::Registry::new();
            &REGISTRY
        }
    }
    pub fn op_ge<'ctx>(
        lhs: crate::base::Value<'ctx>,
        rhs: crate::base::Value<'ctx>,
        arena: &'ctx ::bumpalo::Bump,
    ) -> ::core::option::Option<crate::base::Value<'ctx>> {
        for implementation in ::inventory::iter::<OpGeImpl> {
            if let ::core::option::Option::Some(result) = (implementation
                .function)(lhs, rhs, arena)
            {
                return Some(result);
            }
        }
        None
    }
    pub struct OpGtImpl {
        pub function: crate::op::BinaryOpFn,
    }
    impl ::inventory::Collect for OpGtImpl {
        #[inline]
        fn registry() -> &'static ::inventory::Registry {
            static REGISTRY: ::inventory::Registry = ::inventory::Registry::new();
            &REGISTRY
        }
    }
    pub fn op_gt<'ctx>(
        lhs: crate::base::Value<'ctx>,
        rhs: crate::base::Value<'ctx>,
        arena: &'ctx ::bumpalo::Bump,
    ) -> ::core::option::Option<crate::base::Value<'ctx>> {
        for implementation in ::inventory::iter::<OpGtImpl> {
            if let ::core::option::Option::Some(result) = (implementation
                .function)(lhs, rhs, arena)
            {
                return Some(result);
            }
        }
        None
    }
    pub struct OpLeImpl {
        pub function: crate::op::BinaryOpFn,
    }
    impl ::inventory::Collect for OpLeImpl {
        #[inline]
        fn registry() -> &'static ::inventory::Registry {
            static REGISTRY: ::inventory::Registry = ::inventory::Registry::new();
            &REGISTRY
        }
    }
    pub fn op_le<'ctx>(
        lhs: crate::base::Value<'ctx>,
        rhs: crate::base::Value<'ctx>,
        arena: &'ctx ::bumpalo::Bump,
    ) -> ::core::option::Option<crate::base::Value<'ctx>> {
        for implementation in ::inventory::iter::<OpLeImpl> {
            if let ::core::option::Option::Some(result) = (implementation
                .function)(lhs, rhs, arena)
            {
                return Some(result);
            }
        }
        None
    }
    pub struct OpLtImpl {
        pub function: crate::op::BinaryOpFn,
    }
    impl ::inventory::Collect for OpLtImpl {
        #[inline]
        fn registry() -> &'static ::inventory::Registry {
            static REGISTRY: ::inventory::Registry = ::inventory::Registry::new();
            &REGISTRY
        }
    }
    pub fn op_lt<'ctx>(
        lhs: crate::base::Value<'ctx>,
        rhs: crate::base::Value<'ctx>,
        arena: &'ctx ::bumpalo::Bump,
    ) -> ::core::option::Option<crate::base::Value<'ctx>> {
        for implementation in ::inventory::iter::<OpLtImpl> {
            if let ::core::option::Option::Some(result) = (implementation
                .function)(lhs, rhs, arena)
            {
                return Some(result);
            }
        }
        None
    }
    pub struct OpNeImpl {
        pub function: crate::op::BinaryOpFn,
    }
    impl ::inventory::Collect for OpNeImpl {
        #[inline]
        fn registry() -> &'static ::inventory::Registry {
            static REGISTRY: ::inventory::Registry = ::inventory::Registry::new();
            &REGISTRY
        }
    }
    pub fn op_ne<'ctx>(
        lhs: crate::base::Value<'ctx>,
        rhs: crate::base::Value<'ctx>,
        arena: &'ctx ::bumpalo::Bump,
    ) -> ::core::option::Option<crate::base::Value<'ctx>> {
        for implementation in ::inventory::iter::<OpNeImpl> {
            if let ::core::option::Option::Some(result) = (implementation
                .function)(lhs, rhs, arena)
            {
                return Some(result);
            }
        }
        None
    }
    pub struct OpModImpl {
        pub function: crate::op::BinaryOpFn,
    }
    impl ::inventory::Collect for OpModImpl {
        #[inline]
        fn registry() -> &'static ::inventory::Registry {
            static REGISTRY: ::inventory::Registry = ::inventory::Registry::new();
            &REGISTRY
        }
    }
    pub fn op_moduls<'ctx>(
        lhs: crate::base::Value<'ctx>,
        rhs: crate::base::Value<'ctx>,
        arena: &'ctx ::bumpalo::Bump,
    ) -> ::core::option::Option<crate::base::Value<'ctx>> {
        for implementation in ::inventory::iter::<OpModImpl> {
            if let ::core::option::Option::Some(result) = (implementation
                .function)(lhs, rhs, arena)
            {
                return Some(result);
            }
        }
        None
    }
    pub struct OpBslImpl {
        pub function: crate::op::BinaryOpFn,
    }
    impl ::inventory::Collect for OpBslImpl {
        #[inline]
        fn registry() -> &'static ::inventory::Registry {
            static REGISTRY: ::inventory::Registry = ::inventory::Registry::new();
            &REGISTRY
        }
    }
    pub fn op_bsl<'ctx>(
        lhs: crate::base::Value<'ctx>,
        rhs: crate::base::Value<'ctx>,
        arena: &'ctx ::bumpalo::Bump,
    ) -> ::core::option::Option<crate::base::Value<'ctx>> {
        for implementation in ::inventory::iter::<OpBslImpl> {
            if let ::core::option::Option::Some(result) = (implementation
                .function)(lhs, rhs, arena)
            {
                return Some(result);
            }
        }
        None
    }
    pub struct OpBsrImpl {
        pub function: crate::op::BinaryOpFn,
    }
    impl ::inventory::Collect for OpBsrImpl {
        #[inline]
        fn registry() -> &'static ::inventory::Registry {
            static REGISTRY: ::inventory::Registry = ::inventory::Registry::new();
            &REGISTRY
        }
    }
    pub fn op_bsr<'ctx>(
        lhs: crate::base::Value<'ctx>,
        rhs: crate::base::Value<'ctx>,
        arena: &'ctx ::bumpalo::Bump,
    ) -> ::core::option::Option<crate::base::Value<'ctx>> {
        for implementation in ::inventory::iter::<OpBsrImpl> {
            if let ::core::option::Option::Some(result) = (implementation
                .function)(lhs, rhs, arena)
            {
                return Some(result);
            }
        }
        None
    }
    pub struct OpBitwiseAndImpl {
        pub function: crate::op::BinaryOpFn,
    }
    impl ::inventory::Collect for OpBitwiseAndImpl {
        #[inline]
        fn registry() -> &'static ::inventory::Registry {
            static REGISTRY: ::inventory::Registry = ::inventory::Registry::new();
            &REGISTRY
        }
    }
    pub fn op_band<'ctx>(
        lhs: crate::base::Value<'ctx>,
        rhs: crate::base::Value<'ctx>,
        arena: &'ctx ::bumpalo::Bump,
    ) -> ::core::option::Option<crate::base::Value<'ctx>> {
        for implementation in ::inventory::iter::<OpBitwiseAndImpl> {
            if let ::core::option::Option::Some(result) = (implementation
                .function)(lhs, rhs, arena)
            {
                return Some(result);
            }
        }
        None
    }
    pub struct OpBitwiseOrImpl {
        pub function: crate::op::BinaryOpFn,
    }
    impl ::inventory::Collect for OpBitwiseOrImpl {
        #[inline]
        fn registry() -> &'static ::inventory::Registry {
            static REGISTRY: ::inventory::Registry = ::inventory::Registry::new();
            &REGISTRY
        }
    }
    pub fn op_bor<'ctx>(
        lhs: crate::base::Value<'ctx>,
        rhs: crate::base::Value<'ctx>,
        arena: &'ctx ::bumpalo::Bump,
    ) -> ::core::option::Option<crate::base::Value<'ctx>> {
        for implementation in ::inventory::iter::<OpBitwiseOrImpl> {
            if let ::core::option::Option::Some(result) = (implementation
                .function)(lhs, rhs, arena)
            {
                return Some(result);
            }
        }
        None
    }
    pub struct OpBitwiseXorImpl {
        pub function: crate::op::BinaryOpFn,
    }
    impl ::inventory::Collect for OpBitwiseXorImpl {
        #[inline]
        fn registry() -> &'static ::inventory::Registry {
            static REGISTRY: ::inventory::Registry = ::inventory::Registry::new();
            &REGISTRY
        }
    }
    pub fn op_bxor<'ctx>(
        lhs: crate::base::Value<'ctx>,
        rhs: crate::base::Value<'ctx>,
        arena: &'ctx ::bumpalo::Bump,
    ) -> ::core::option::Option<crate::base::Value<'ctx>> {
        for implementation in ::inventory::iter::<OpBitwiseXorImpl> {
            if let ::core::option::Option::Some(result) = (implementation
                .function)(lhs, rhs, arena)
            {
                return Some(result);
            }
        }
        None
    }
    pub struct OpNotImpl {
        pub function: crate::op::UnaryOpFn,
    }
    impl ::inventory::Collect for OpNotImpl {
        #[inline]
        fn registry() -> &'static ::inventory::Registry {
            static REGISTRY: ::inventory::Registry = ::inventory::Registry::new();
            &REGISTRY
        }
    }
    pub fn op_not<'ctx>(
        rhs: crate::base::Value<'ctx>,
        arena: &'ctx ::bumpalo::Bump,
    ) -> ::core::option::Option<crate::base::Value<'ctx>> {
        for implementation in ::inventory::iter::<OpNotImpl> {
            if let ::core::option::Option::Some(result) = (implementation
                .function)(rhs, arena)
            {
                return Some(result);
            }
        }
        None
    }
    pub struct OpPosImpl {
        pub function: crate::op::UnaryOpFn,
    }
    impl ::inventory::Collect for OpPosImpl {
        #[inline]
        fn registry() -> &'static ::inventory::Registry {
            static REGISTRY: ::inventory::Registry = ::inventory::Registry::new();
            &REGISTRY
        }
    }
    pub fn op_pos<'ctx>(
        rhs: crate::base::Value<'ctx>,
        arena: &'ctx ::bumpalo::Bump,
    ) -> ::core::option::Option<crate::base::Value<'ctx>> {
        for implementation in ::inventory::iter::<OpPosImpl> {
            if let ::core::option::Option::Some(result) = (implementation
                .function)(rhs, arena)
            {
                return Some(result);
            }
        }
        None
    }
    pub struct OpNegImpl {
        pub function: crate::op::UnaryOpFn,
    }
    impl ::inventory::Collect for OpNegImpl {
        #[inline]
        fn registry() -> &'static ::inventory::Registry {
            static REGISTRY: ::inventory::Registry = ::inventory::Registry::new();
            &REGISTRY
        }
    }
    pub fn op_neg<'ctx>(
        rhs: crate::base::Value<'ctx>,
        arena: &'ctx ::bumpalo::Bump,
    ) -> ::core::option::Option<crate::base::Value<'ctx>> {
        for implementation in ::inventory::iter::<OpNegImpl> {
            if let ::core::option::Option::Some(result) = (implementation
                .function)(rhs, arena)
            {
                return Some(result);
            }
        }
        None
    }
    pub struct OpBitwiseNotImpl {
        pub function: crate::op::UnaryOpFn,
    }
    impl ::inventory::Collect for OpBitwiseNotImpl {
        #[inline]
        fn registry() -> &'static ::inventory::Registry {
            static REGISTRY: ::inventory::Registry = ::inventory::Registry::new();
            &REGISTRY
        }
    }
    pub fn op_bnot<'ctx>(
        rhs: crate::base::Value<'ctx>,
        arena: &'ctx ::bumpalo::Bump,
    ) -> ::core::option::Option<crate::base::Value<'ctx>> {
        for implementation in ::inventory::iter::<OpBitwiseNotImpl> {
            if let ::core::option::Option::Some(result) = (implementation
                .function)(rhs, arena)
            {
                return Some(result);
            }
        }
        None
    }
}
mod ast {
    pub mod core {
        use crate::base::ValueContainer;
        use crate::exec_ctx::{ExecutionContext, Result};
        use proc_macro2::Span;
        use std::cell::RefCell;
        use std::rc::Rc;
        pub trait ASTNode {
            type Output;
            fn eval(&self, ctx: Rc<RefCell<ExecutionContext>>) -> Result<Self::Output>;
            fn get_callsite(&self) -> Option<Span>;
        }
        pub struct Node<T>
        where
            T: ASTNode,
        {
            pub kind: T,
            pub span: Option<Span>,
        }
        #[automatically_derived]
        impl<T: ::core::fmt::Debug> ::core::fmt::Debug for Node<T>
        where
            T: ASTNode,
        {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Node",
                    "kind",
                    &self.kind,
                    "span",
                    &&self.span,
                )
            }
        }
        #[automatically_derived]
        impl<T: ::core::clone::Clone> ::core::clone::Clone for Node<T>
        where
            T: ASTNode,
        {
            #[inline]
            fn clone(&self) -> Node<T> {
                Node {
                    kind: ::core::clone::Clone::clone(&self.kind),
                    span: ::core::clone::Clone::clone(&self.span),
                }
            }
        }
        pub struct Module {
            pub body: Vec<Node<Stmt>>,
        }
        impl ASTNode for Module {
            type Output = ();
            fn eval(&self, ctx: Rc<RefCell<ExecutionContext>>) -> Result<()> {
                for stmt in self.body.clone() {
                    stmt.kind.eval(ctx.clone())?;
                    ctx.borrow_mut().consume_one()?;
                }
                Ok(())
            }
            fn get_callsite(&self) -> Option<Span> {
                ::core::panicking::panic("not yet implemented")
            }
        }
        pub enum Expr {
            Literal(Literal),
            Variable(String),
            BinaryOp {
                left: Box<Node<Expr>>,
                op: BinaryOperator,
                right: Box<Node<Expr>>,
            },
            UnaryOp { op: UaryOperator, operand: Box<Node<Expr>> },
            Call { function: Box<Node<Expr>>, args: Vec<Node<Expr>> },
            Attribute { value: Box<Node<Expr>>, attr: String },
            Subscript { value: Box<Node<Expr>>, slice: Box<Node<Expr>> },
            Range { lower: Option<i64>, upper: Option<i64>, step: Option<i64> },
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Expr {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Expr::Literal(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Literal",
                            &__self_0,
                        )
                    }
                    Expr::Variable(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Variable",
                            &__self_0,
                        )
                    }
                    Expr::BinaryOp { left: __self_0, op: __self_1, right: __self_2 } => {
                        ::core::fmt::Formatter::debug_struct_field3_finish(
                            f,
                            "BinaryOp",
                            "left",
                            __self_0,
                            "op",
                            __self_1,
                            "right",
                            &__self_2,
                        )
                    }
                    Expr::UnaryOp { op: __self_0, operand: __self_1 } => {
                        ::core::fmt::Formatter::debug_struct_field2_finish(
                            f,
                            "UnaryOp",
                            "op",
                            __self_0,
                            "operand",
                            &__self_1,
                        )
                    }
                    Expr::Call { function: __self_0, args: __self_1 } => {
                        ::core::fmt::Formatter::debug_struct_field2_finish(
                            f,
                            "Call",
                            "function",
                            __self_0,
                            "args",
                            &__self_1,
                        )
                    }
                    Expr::Attribute { value: __self_0, attr: __self_1 } => {
                        ::core::fmt::Formatter::debug_struct_field2_finish(
                            f,
                            "Attribute",
                            "value",
                            __self_0,
                            "attr",
                            &__self_1,
                        )
                    }
                    Expr::Subscript { value: __self_0, slice: __self_1 } => {
                        ::core::fmt::Formatter::debug_struct_field2_finish(
                            f,
                            "Subscript",
                            "value",
                            __self_0,
                            "slice",
                            &__self_1,
                        )
                    }
                    Expr::Range { lower: __self_0, upper: __self_1, step: __self_2 } => {
                        ::core::fmt::Formatter::debug_struct_field3_finish(
                            f,
                            "Range",
                            "lower",
                            __self_0,
                            "upper",
                            __self_1,
                            "step",
                            &__self_2,
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Expr {
            #[inline]
            fn clone(&self) -> Expr {
                match self {
                    Expr::Literal(__self_0) => {
                        Expr::Literal(::core::clone::Clone::clone(__self_0))
                    }
                    Expr::Variable(__self_0) => {
                        Expr::Variable(::core::clone::Clone::clone(__self_0))
                    }
                    Expr::BinaryOp { left: __self_0, op: __self_1, right: __self_2 } => {
                        Expr::BinaryOp {
                            left: ::core::clone::Clone::clone(__self_0),
                            op: ::core::clone::Clone::clone(__self_1),
                            right: ::core::clone::Clone::clone(__self_2),
                        }
                    }
                    Expr::UnaryOp { op: __self_0, operand: __self_1 } => {
                        Expr::UnaryOp {
                            op: ::core::clone::Clone::clone(__self_0),
                            operand: ::core::clone::Clone::clone(__self_1),
                        }
                    }
                    Expr::Call { function: __self_0, args: __self_1 } => {
                        Expr::Call {
                            function: ::core::clone::Clone::clone(__self_0),
                            args: ::core::clone::Clone::clone(__self_1),
                        }
                    }
                    Expr::Attribute { value: __self_0, attr: __self_1 } => {
                        Expr::Attribute {
                            value: ::core::clone::Clone::clone(__self_0),
                            attr: ::core::clone::Clone::clone(__self_1),
                        }
                    }
                    Expr::Subscript { value: __self_0, slice: __self_1 } => {
                        Expr::Subscript {
                            value: ::core::clone::Clone::clone(__self_0),
                            slice: ::core::clone::Clone::clone(__self_1),
                        }
                    }
                    Expr::Range { lower: __self_0, upper: __self_1, step: __self_2 } => {
                        Expr::Range {
                            lower: ::core::clone::Clone::clone(__self_0),
                            upper: ::core::clone::Clone::clone(__self_1),
                            step: ::core::clone::Clone::clone(__self_2),
                        }
                    }
                }
            }
        }
        impl ASTNode for Expr {
            type Output = ();
            fn eval(&self, ctx: Rc<RefCell<ExecutionContext>>) -> Result<Self::Output> {
                ::core::panicking::panic("not yet implemented")
            }
            fn get_callsite(&self) -> Option<Span> {
                ::core::panicking::panic("not yet implemented")
            }
        }
        pub enum Literal {
            Int(i64),
            Float(f64),
            String(String),
            Bool(bool),
            None,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Literal {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Literal::Int(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Int",
                            &__self_0,
                        )
                    }
                    Literal::Float(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Float",
                            &__self_0,
                        )
                    }
                    Literal::String(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "String",
                            &__self_0,
                        )
                    }
                    Literal::Bool(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Bool",
                            &__self_0,
                        )
                    }
                    Literal::None => ::core::fmt::Formatter::write_str(f, "None"),
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Literal {
            #[inline]
            fn clone(&self) -> Literal {
                match self {
                    Literal::Int(__self_0) => {
                        Literal::Int(::core::clone::Clone::clone(__self_0))
                    }
                    Literal::Float(__self_0) => {
                        Literal::Float(::core::clone::Clone::clone(__self_0))
                    }
                    Literal::String(__self_0) => {
                        Literal::String(::core::clone::Clone::clone(__self_0))
                    }
                    Literal::Bool(__self_0) => {
                        Literal::Bool(::core::clone::Clone::clone(__self_0))
                    }
                    Literal::None => Literal::None,
                }
            }
        }
        impl ASTNode for Literal {
            type Output = ();
            fn eval(&self, ctx: Rc<RefCell<ExecutionContext>>) -> Result<Self::Output> {
                ::core::panicking::panic("not yet implemented")
            }
            fn get_callsite(&self) -> Option<Span> {
                ::core::panicking::panic("not yet implemented")
            }
        }
        pub enum BinaryOperator {
            Add,
            Subtract,
            Multiply,
            Divide,
            Pow,
            And,
            Or,
            Xor,
            Modulo,
            BitwiseAnd,
            BitwiseOr,
            BitwiseXor,
            Eq,
            NotEq,
            Lt,
            Lte,
            Gt,
            Gte,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for BinaryOperator {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        BinaryOperator::Add => "Add",
                        BinaryOperator::Subtract => "Subtract",
                        BinaryOperator::Multiply => "Multiply",
                        BinaryOperator::Divide => "Divide",
                        BinaryOperator::Pow => "Pow",
                        BinaryOperator::And => "And",
                        BinaryOperator::Or => "Or",
                        BinaryOperator::Xor => "Xor",
                        BinaryOperator::Modulo => "Modulo",
                        BinaryOperator::BitwiseAnd => "BitwiseAnd",
                        BinaryOperator::BitwiseOr => "BitwiseOr",
                        BinaryOperator::BitwiseXor => "BitwiseXor",
                        BinaryOperator::Eq => "Eq",
                        BinaryOperator::NotEq => "NotEq",
                        BinaryOperator::Lt => "Lt",
                        BinaryOperator::Lte => "Lte",
                        BinaryOperator::Gt => "Gt",
                        BinaryOperator::Gte => "Gte",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for BinaryOperator {
            #[inline]
            fn clone(&self) -> BinaryOperator {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for BinaryOperator {}
        pub enum UaryOperator {
            Positive,
            Negative,
            Not,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for UaryOperator {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        UaryOperator::Positive => "Positive",
                        UaryOperator::Negative => "Negative",
                        UaryOperator::Not => "Not",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for UaryOperator {
            #[inline]
            fn clone(&self) -> UaryOperator {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for UaryOperator {}
        pub enum Stmt {
            Expression(Node<Expr>),
            Assign { target: Node<Expr>, value: Node<Expr> },
            If {
                test: Node<Expr>,
                body: Vec<Node<Stmt>>,
                otherwise: Option<Vec<Node<Stmt>>>,
            },
            FunctionDef { name: String, args: Vec<String>, body: Vec<Node<Stmt>> },
            ClassDef { name: String, bases: Vec<Node<Expr>>, body: Vec<Node<Stmt>> },
            ForLoop {
                target: Node<Expr>,
                iter_expr: Node<Expr>,
                body: Vec<Node<Stmt>>,
                not_break: Vec<Node<Stmt>>,
            },
            WhileLoop {
                test: Node<Expr>,
                body: Vec<Node<Stmt>>,
                otherwise: Option<Vec<Node<Stmt>>>,
            },
            Return(Option<Node<Expr>>),
            Break,
            Continue,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Stmt {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Stmt::Expression(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Expression",
                            &__self_0,
                        )
                    }
                    Stmt::Assign { target: __self_0, value: __self_1 } => {
                        ::core::fmt::Formatter::debug_struct_field2_finish(
                            f,
                            "Assign",
                            "target",
                            __self_0,
                            "value",
                            &__self_1,
                        )
                    }
                    Stmt::If { test: __self_0, body: __self_1, otherwise: __self_2 } => {
                        ::core::fmt::Formatter::debug_struct_field3_finish(
                            f,
                            "If",
                            "test",
                            __self_0,
                            "body",
                            __self_1,
                            "otherwise",
                            &__self_2,
                        )
                    }
                    Stmt::FunctionDef {
                        name: __self_0,
                        args: __self_1,
                        body: __self_2,
                    } => {
                        ::core::fmt::Formatter::debug_struct_field3_finish(
                            f,
                            "FunctionDef",
                            "name",
                            __self_0,
                            "args",
                            __self_1,
                            "body",
                            &__self_2,
                        )
                    }
                    Stmt::ClassDef {
                        name: __self_0,
                        bases: __self_1,
                        body: __self_2,
                    } => {
                        ::core::fmt::Formatter::debug_struct_field3_finish(
                            f,
                            "ClassDef",
                            "name",
                            __self_0,
                            "bases",
                            __self_1,
                            "body",
                            &__self_2,
                        )
                    }
                    Stmt::ForLoop {
                        target: __self_0,
                        iter_expr: __self_1,
                        body: __self_2,
                        not_break: __self_3,
                    } => {
                        ::core::fmt::Formatter::debug_struct_field4_finish(
                            f,
                            "ForLoop",
                            "target",
                            __self_0,
                            "iter_expr",
                            __self_1,
                            "body",
                            __self_2,
                            "not_break",
                            &__self_3,
                        )
                    }
                    Stmt::WhileLoop {
                        test: __self_0,
                        body: __self_1,
                        otherwise: __self_2,
                    } => {
                        ::core::fmt::Formatter::debug_struct_field3_finish(
                            f,
                            "WhileLoop",
                            "test",
                            __self_0,
                            "body",
                            __self_1,
                            "otherwise",
                            &__self_2,
                        )
                    }
                    Stmt::Return(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Return",
                            &__self_0,
                        )
                    }
                    Stmt::Break => ::core::fmt::Formatter::write_str(f, "Break"),
                    Stmt::Continue => ::core::fmt::Formatter::write_str(f, "Continue"),
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Stmt {
            #[inline]
            fn clone(&self) -> Stmt {
                match self {
                    Stmt::Expression(__self_0) => {
                        Stmt::Expression(::core::clone::Clone::clone(__self_0))
                    }
                    Stmt::Assign { target: __self_0, value: __self_1 } => {
                        Stmt::Assign {
                            target: ::core::clone::Clone::clone(__self_0),
                            value: ::core::clone::Clone::clone(__self_1),
                        }
                    }
                    Stmt::If { test: __self_0, body: __self_1, otherwise: __self_2 } => {
                        Stmt::If {
                            test: ::core::clone::Clone::clone(__self_0),
                            body: ::core::clone::Clone::clone(__self_1),
                            otherwise: ::core::clone::Clone::clone(__self_2),
                        }
                    }
                    Stmt::FunctionDef {
                        name: __self_0,
                        args: __self_1,
                        body: __self_2,
                    } => {
                        Stmt::FunctionDef {
                            name: ::core::clone::Clone::clone(__self_0),
                            args: ::core::clone::Clone::clone(__self_1),
                            body: ::core::clone::Clone::clone(__self_2),
                        }
                    }
                    Stmt::ClassDef {
                        name: __self_0,
                        bases: __self_1,
                        body: __self_2,
                    } => {
                        Stmt::ClassDef {
                            name: ::core::clone::Clone::clone(__self_0),
                            bases: ::core::clone::Clone::clone(__self_1),
                            body: ::core::clone::Clone::clone(__self_2),
                        }
                    }
                    Stmt::ForLoop {
                        target: __self_0,
                        iter_expr: __self_1,
                        body: __self_2,
                        not_break: __self_3,
                    } => {
                        Stmt::ForLoop {
                            target: ::core::clone::Clone::clone(__self_0),
                            iter_expr: ::core::clone::Clone::clone(__self_1),
                            body: ::core::clone::Clone::clone(__self_2),
                            not_break: ::core::clone::Clone::clone(__self_3),
                        }
                    }
                    Stmt::WhileLoop {
                        test: __self_0,
                        body: __self_1,
                        otherwise: __self_2,
                    } => {
                        Stmt::WhileLoop {
                            test: ::core::clone::Clone::clone(__self_0),
                            body: ::core::clone::Clone::clone(__self_1),
                            otherwise: ::core::clone::Clone::clone(__self_2),
                        }
                    }
                    Stmt::Return(__self_0) => {
                        Stmt::Return(::core::clone::Clone::clone(__self_0))
                    }
                    Stmt::Break => Stmt::Break,
                    Stmt::Continue => Stmt::Continue,
                }
            }
        }
        impl ASTNode for Stmt {
            type Output = ();
            fn eval(&self, ctx: Rc<RefCell<ExecutionContext>>) -> Result<Self::Output> {
                ::core::panicking::panic("not yet implemented")
            }
            fn get_callsite(&self) -> Option<Span> {
                ::core::panicking::panic("not yet implemented")
            }
        }
    }
}
pub mod base {
    use crate::builtin::{VirPyFloat, VirPyInt, VirPyObject};
    use bumpalo::Bump;
    use std::fmt::Debug;
    use crate::error::SandboxExecutionError;
    pub type Value<'ctx> = &'ctx ValueContainer<'ctx>;
    pub enum ValueKind<'ctx> {
        Int(VirPyInt),
        Float(VirPyFloat),
        Object(VirPyObject<'ctx>),
        ErrorWrapped(SandboxExecutionError),
        Bool(bool),
    }
    #[automatically_derived]
    impl<'ctx> ::core::fmt::Debug for ValueKind<'ctx> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                ValueKind::Int(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Int",
                        &__self_0,
                    )
                }
                ValueKind::Float(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Float",
                        &__self_0,
                    )
                }
                ValueKind::Object(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Object",
                        &__self_0,
                    )
                }
                ValueKind::ErrorWrapped(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ErrorWrapped",
                        &__self_0,
                    )
                }
                ValueKind::Bool(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Bool",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl<'ctx> ::core::clone::Clone for ValueKind<'ctx> {
        #[inline]
        fn clone(&self) -> ValueKind<'ctx> {
            match self {
                ValueKind::Int(__self_0) => {
                    ValueKind::Int(::core::clone::Clone::clone(__self_0))
                }
                ValueKind::Float(__self_0) => {
                    ValueKind::Float(::core::clone::Clone::clone(__self_0))
                }
                ValueKind::Object(__self_0) => {
                    ValueKind::Object(::core::clone::Clone::clone(__self_0))
                }
                ValueKind::ErrorWrapped(__self_0) => {
                    ValueKind::ErrorWrapped(::core::clone::Clone::clone(__self_0))
                }
                ValueKind::Bool(__self_0) => {
                    ValueKind::Bool(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    pub trait Downcast<'ctx>: Sized {
        fn from_value(value: Value<'ctx>) -> Option<&'ctx Self>;
    }
    pub trait Upcast<'ctx>: Sized {
        fn from_value(&'ctx self) -> ValueKind<'ctx>;
    }
    impl<'ctx> Downcast<'ctx> for bool {
        fn from_value(value: Value<'ctx>) -> Option<&'ctx Self> {
            value.as_bool()
        }
    }
    impl<'ctx> Upcast<'ctx> for bool {
        fn from_value(&'ctx self) -> ValueKind<'ctx> {
            ValueKind::Bool((*self).clone())
        }
    }
    pub struct ValueContainer<'ctx> {
        pub kind: ValueKind<'ctx>,
    }
    #[automatically_derived]
    impl<'ctx> ::core::fmt::Debug for ValueContainer<'ctx> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "ValueContainer",
                "kind",
                &&self.kind,
            )
        }
    }
    impl<'ctx> ValueContainer<'ctx> {
        pub fn new(kind: ValueKind<'ctx>, arena: &'ctx Bump) -> Value<'ctx> {
            arena.alloc(ValueContainer { kind })
        }
        pub fn clone_in_arena(&self, arena: &'ctx Bump) -> Value<'ctx> {
            let new_kind = match &self.kind {
                ValueKind::Int(i) => ValueKind::Int(i.clone()),
                ValueKind::Float(f) => ValueKind::Float(f.clone()),
                ValueKind::Object(o) => ValueKind::Object(o.clone()),
                ValueKind::ErrorWrapped(e) => ValueKind::ErrorWrapped(e.clone()),
                ValueKind::Bool(b) => ValueKind::Bool(b.clone()),
            };
            ValueContainer::new(new_kind, arena)
        }
        pub fn as_int(&self) -> Option<&VirPyInt> {
            match &self.kind {
                ValueKind::Int(i) => Some(i),
                _ => None,
            }
        }
        pub fn as_float(&self) -> Option<&VirPyFloat> {
            match &self.kind {
                ValueKind::Float(f) => Some(f),
                _ => None,
            }
        }
        pub fn as_object(&self) -> Option<&VirPyObject<'ctx>> {
            match &self.kind {
                ValueKind::Object(o) => Some(o),
                _ => None,
            }
        }
        pub fn as_error(&self) -> Option<&SandboxExecutionError> {
            match &self.kind {
                ValueKind::ErrorWrapped(e) => Some(e),
                _ => None,
            }
        }
        pub fn as_bool(&self) -> Option<&bool> {
            match &self.kind {
                ValueKind::Bool(e) => Some(e),
                _ => None,
            }
        }
    }
}
pub mod builtin {
    use crate::base::{Downcast, Upcast, Value, ValueKind};
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::ops::{Add, Div, Mul, Rem, Sub};
    use std::rc::Rc;
    use crate::error::Result;
    pub struct VirPyInt {
        pub value: i64,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for VirPyInt {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "VirPyInt",
                "value",
                &&self.value,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for VirPyInt {
        #[inline]
        fn clone(&self) -> VirPyInt {
            let _: ::core::clone::AssertParamIsClone<i64>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for VirPyInt {}
    impl VirPyInt {
        pub fn new(value: i64) -> Self {
            Self { value }
        }
    }
    pub struct VirPyFloat {
        pub value: f64,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for VirPyFloat {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "VirPyFloat",
                "value",
                &&self.value,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for VirPyFloat {
        #[inline]
        fn clone(&self) -> VirPyFloat {
            let _: ::core::clone::AssertParamIsClone<f64>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for VirPyFloat {}
    impl VirPyFloat {
        pub fn new(value: f64) -> Self {
            Self { value }
        }
    }
    pub struct Mapping<'ctx> {
        pub mapping: HashMap<String, Rc<RefCell<Value<'ctx>>>>,
    }
    #[automatically_derived]
    impl<'ctx> ::core::fmt::Debug for Mapping<'ctx> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "Mapping",
                "mapping",
                &&self.mapping,
            )
        }
    }
    #[automatically_derived]
    impl<'ctx> ::core::clone::Clone for Mapping<'ctx> {
        #[inline]
        fn clone(&self) -> Mapping<'ctx> {
            Mapping {
                mapping: ::core::clone::Clone::clone(&self.mapping),
            }
        }
    }
    pub struct VirPyObject<'ctx> {
        pub mapping: Rc<RefCell<Mapping<'ctx>>>,
    }
    #[automatically_derived]
    impl<'ctx> ::core::fmt::Debug for VirPyObject<'ctx> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "VirPyObject",
                "mapping",
                &&self.mapping,
            )
        }
    }
    #[automatically_derived]
    impl<'ctx> ::core::clone::Clone for VirPyObject<'ctx> {
        #[inline]
        fn clone(&self) -> VirPyObject<'ctx> {
            VirPyObject {
                mapping: ::core::clone::Clone::clone(&self.mapping),
            }
        }
    }
    impl<'ctx> VirPyObject<'ctx> {
        pub fn new() -> Self {
            Self {
                mapping: Rc::new(RefCell::new(Mapping { mapping: HashMap::new() })),
            }
        }
        pub fn get(&self, key: &str) -> Option<Rc<RefCell<Value<'ctx>>>> {
            self.mapping.borrow().mapping.get(key).cloned()
        }
        pub fn set(&self, key: String, value: Value<'ctx>) {
            let value_cell = Rc::new(RefCell::new(value));
            self.mapping.borrow_mut().mapping.insert(key, value_cell);
        }
        pub fn clone(&self) -> Self {
            Self {
                mapping: Rc::clone(&self.mapping),
            }
        }
    }
    impl<'ctx> Downcast<'ctx> for VirPyInt {
        fn from_value(value: Value<'ctx>) -> Option<&'ctx Self> {
            value.as_int()
        }
    }
    impl<'ctx> Downcast<'ctx> for VirPyFloat {
        fn from_value(value: Value<'ctx>) -> Option<&'ctx Self> {
            value.as_float()
        }
    }
    impl<'ctx> Upcast<'ctx> for VirPyInt {
        fn from_value(&'ctx self) -> ValueKind<'ctx> {
            ValueKind::Int((*self).clone())
        }
    }
    impl<'ctx> Upcast<'ctx> for VirPyFloat {
        fn from_value(&'ctx self) -> ValueKind<'ctx> {
            ValueKind::Float((*self).clone())
        }
    }
    impl Add for VirPyInt {
        type Output = Result<Self>;
        fn add(self, rhs: Self) -> Self::Output {
            Ok(Self::new(self.value + rhs.value))
        }
    }
    impl Add for VirPyFloat {
        type Output = Result<Self>;
        fn add(self, rhs: Self) -> Self::Output {
            Ok(Self::new(self.value + rhs.value))
        }
    }
    impl Add<VirPyInt> for VirPyFloat {
        type Output = Result<Self>;
        fn add(self, rhs: VirPyInt) -> Self::Output {
            Ok(Self::new(self.value + (rhs.value as f64)))
        }
    }
    impl Add<VirPyFloat> for VirPyInt {
        type Output = Result<VirPyFloat>;
        fn add(self, rhs: VirPyFloat) -> Self::Output {
            Ok(VirPyFloat::new(self.value as f64 + rhs.value))
        }
    }
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyInt as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyInt as crate::base::Downcast>::from_value(rhs)?;
            match (|a, b| a + b)(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(crate::base::ValueContainer::new(ValueKind::Int(result), arena))
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpAddImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyFloat as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyFloat as crate::base::Downcast>::from_value(rhs)?;
            match (|a, b| a + b)(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Float(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpAddImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyInt as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyFloat as crate::base::Downcast>::from_value(rhs)?;
            match (|a, b| a + b)(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Float(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpAddImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyFloat as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyInt as crate::base::Downcast>::from_value(rhs)?;
            match (|a, b| a + b)(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Float(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpAddImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    impl Sub for VirPyInt {
        type Output = Result<Self>;
        fn sub(self, rhs: Self) -> Self::Output {
            Ok(Self::new(self.value - rhs.value))
        }
    }
    impl Sub for VirPyFloat {
        type Output = Result<Self>;
        fn sub(self, rhs: Self) -> Self::Output {
            Ok(Self::new(self.value - rhs.value))
        }
    }
    impl Sub<VirPyInt> for VirPyFloat {
        type Output = Result<Self>;
        fn sub(self, rhs: VirPyInt) -> Self::Output {
            Ok(Self::new(self.value - (rhs.value as f64)))
        }
    }
    impl Sub<VirPyFloat> for VirPyInt {
        type Output = Result<VirPyFloat>;
        fn sub(self, rhs: VirPyFloat) -> Self::Output {
            Ok(VirPyFloat::new(self.value as f64 - rhs.value))
        }
    }
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyInt as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyInt as crate::base::Downcast>::from_value(rhs)?;
            match (|a, b| a - b)(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(crate::base::ValueContainer::new(ValueKind::Int(result), arena))
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpSubImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyFloat as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyFloat as crate::base::Downcast>::from_value(rhs)?;
            match (|a, b| a - b)(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Float(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpSubImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyInt as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyFloat as crate::base::Downcast>::from_value(rhs)?;
            match (|a, b| a - b)(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Float(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpSubImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyFloat as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyInt as crate::base::Downcast>::from_value(rhs)?;
            match (|a, b| a - b)(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Float(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpSubImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    impl Mul for VirPyInt {
        type Output = Result<Self>;
        fn mul(self, rhs: Self) -> Self::Output {
            Ok(Self::new(self.value * rhs.value))
        }
    }
    impl Mul for VirPyFloat {
        type Output = Result<Self>;
        fn mul(self, rhs: Self) -> Self::Output {
            Ok(Self::new(self.value * rhs.value))
        }
    }
    impl Mul<VirPyInt> for VirPyFloat {
        type Output = Result<Self>;
        fn mul(self, rhs: VirPyInt) -> Self::Output {
            Ok(Self::new(self.value * (rhs.value as f64)))
        }
    }
    impl Mul<VirPyFloat> for VirPyInt {
        type Output = Result<VirPyFloat>;
        fn mul(self, rhs: VirPyFloat) -> Self::Output {
            Ok(VirPyFloat::new(self.value as f64 * rhs.value))
        }
    }
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyInt as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyInt as crate::base::Downcast>::from_value(rhs)?;
            match (|a, b| a * b)(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(crate::base::ValueContainer::new(ValueKind::Int(result), arena))
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpMulImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyFloat as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyFloat as crate::base::Downcast>::from_value(rhs)?;
            match (|a, b| a * b)(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Float(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpMulImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyInt as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyFloat as crate::base::Downcast>::from_value(rhs)?;
            match (|a, b| a * b)(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Float(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpMulImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyFloat as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyInt as crate::base::Downcast>::from_value(rhs)?;
            match (|a, b| a * b)(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Float(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpMulImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    impl Div for VirPyInt {
        type Output = Result<VirPyFloat>;
        fn div(self, rhs: Self) -> Self::Output {
            if (rhs.value == 0) {
                return Err(crate::error::SandboxExecutionError::DivideByZeroError);
            }
            Ok(VirPyFloat::new((self.value as f64) / (rhs.value as f64)))
        }
    }
    impl Div for VirPyFloat {
        type Output = Result<Self>;
        fn div(self, rhs: Self) -> Self::Output {
            if (rhs.value == 0f64) {
                return Err(crate::error::SandboxExecutionError::DivideByZeroError);
            }
            Ok(Self::new(self.value / rhs.value))
        }
    }
    impl Div<VirPyInt> for VirPyFloat {
        type Output = Result<Self>;
        fn div(self, rhs: VirPyInt) -> Self::Output {
            if (rhs.value == 0) {
                return Err(crate::error::SandboxExecutionError::DivideByZeroError);
            }
            Ok(Self::new(self.value / (rhs.value as f64)))
        }
    }
    impl Div<VirPyFloat> for VirPyInt {
        type Output = Result<VirPyFloat>;
        fn div(self, rhs: VirPyFloat) -> Self::Output {
            if (rhs.value == 0f64) {
                return Err(crate::error::SandboxExecutionError::DivideByZeroError);
            }
            Ok(VirPyFloat::new(self.value as f64 / rhs.value))
        }
    }
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyInt as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyInt as crate::base::Downcast>::from_value(rhs)?;
            match (|a, b| a / b)(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Float(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpDivImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyFloat as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyFloat as crate::base::Downcast>::from_value(rhs)?;
            match (|a, b| a / b)(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Float(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpDivImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyInt as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyFloat as crate::base::Downcast>::from_value(rhs)?;
            match (|a, b| a / b)(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Float(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpDivImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyFloat as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyInt as crate::base::Downcast>::from_value(rhs)?;
            match (|a, b| a / b)(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Float(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpDivImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    impl Rem for VirPyInt {
        type Output = Result<VirPyInt>;
        fn rem(self, rhs: Self) -> Self::Output {
            if (rhs.value == 0) {
                return Err(crate::error::SandboxExecutionError::DivideByZeroError);
            }
            let mut v = (self.value) % (rhs.value);
            if (v < 0) {
                v += rhs.value;
            }
            Ok(VirPyInt::new(v))
        }
    }
    impl Rem for VirPyFloat {
        type Output = Result<VirPyFloat>;
        fn rem(self, rhs: Self) -> Self::Output {
            if (rhs.value == 0f64) {
                return Err(crate::error::SandboxExecutionError::DivideByZeroError);
            }
            let mut v = (self.value) % (rhs.value);
            if (v < 0f64) {
                v += rhs.value;
            }
            Ok(VirPyFloat::new(v))
        }
    }
    impl Rem<VirPyInt> for VirPyFloat {
        type Output = Result<VirPyFloat>;
        fn rem(self, rhs: VirPyInt) -> Self::Output {
            if (rhs.value == 0) {
                return Err(crate::error::SandboxExecutionError::DivideByZeroError);
            }
            let mut v = (self.value) % (rhs.value as f64);
            if (v < 0f64) {
                v += rhs.value as f64;
            }
            Ok(VirPyFloat::new(v))
        }
    }
    impl Rem<VirPyFloat> for VirPyInt {
        type Output = Result<VirPyFloat>;
        fn rem(self, rhs: VirPyFloat) -> Self::Output {
            if (rhs.value == 0f64) {
                return Err(crate::error::SandboxExecutionError::DivideByZeroError);
            }
            let mut v = (self.value as f64) % (rhs.value);
            if (v < 0f64) {
                v += rhs.value;
            }
            Ok(VirPyFloat::new(v))
        }
    }
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyInt as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyInt as crate::base::Downcast>::from_value(rhs)?;
            match (|a, b| a % b)(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(crate::base::ValueContainer::new(ValueKind::Int(result), arena))
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpModImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyFloat as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyFloat as crate::base::Downcast>::from_value(rhs)?;
            match (|a, b| a % b)(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Float(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpModImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyInt as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyFloat as crate::base::Downcast>::from_value(rhs)?;
            match (|a, b| a % b)(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Float(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpModImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyFloat as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyInt as crate::base::Downcast>::from_value(rhs)?;
            match (|a, b| a % b)(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Float(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpModImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <bool as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <bool as crate::base::Downcast>::from_value(rhs)?;
            match (|a, b| Ok(a == b))(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Bool(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpEqImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <bool as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <bool as crate::base::Downcast>::from_value(rhs)?;
            match (|a, b| Ok(a <= b))(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Bool(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpLeImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <bool as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <bool as crate::base::Downcast>::from_value(rhs)?;
            match (|a, b| Ok(a < b))(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Bool(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpLtImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <bool as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <bool as crate::base::Downcast>::from_value(rhs)?;
            match (|a, b| Ok(a >= b))(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Bool(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpGeImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <bool as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <bool as crate::base::Downcast>::from_value(rhs)?;
            match (|a, b| Ok(a > b))(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Bool(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpGtImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <bool as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <bool as crate::base::Downcast>::from_value(rhs)?;
            match (|a, b| Ok(a != b))(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Bool(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpNeImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyInt as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyInt as crate::base::Downcast>::from_value(rhs)?;
            match (|a: VirPyInt, b: VirPyInt| Ok(
                a.value == b.value,
            ))(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Bool(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpEqImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyInt as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyFloat as crate::base::Downcast>::from_value(rhs)?;
            match (|a: VirPyInt, b: VirPyFloat| Ok(
                a.value as f64 == b.value,
            ))(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Bool(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpEqImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyFloat as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyInt as crate::base::Downcast>::from_value(rhs)?;
            match (|a: VirPyFloat, b: VirPyInt| Ok(
                a.value == b.value as f64,
            ))(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Bool(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpEqImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyFloat as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyFloat as crate::base::Downcast>::from_value(rhs)?;
            match (|a: VirPyFloat, b: VirPyFloat| Ok(
                a.value == b.value,
            ))(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Bool(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpEqImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyInt as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyInt as crate::base::Downcast>::from_value(rhs)?;
            match (|a: VirPyInt, b: VirPyInt| Ok(
                a.value <= b.value,
            ))(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Bool(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpLeImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyInt as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyFloat as crate::base::Downcast>::from_value(rhs)?;
            match (|a: VirPyInt, b: VirPyFloat| Ok(
                a.value as f64 <= b.value,
            ))(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Bool(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpLeImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyFloat as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyInt as crate::base::Downcast>::from_value(rhs)?;
            match (|a: VirPyFloat, b: VirPyInt| Ok(
                a.value <= b.value as f64,
            ))(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Bool(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpLeImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyFloat as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyFloat as crate::base::Downcast>::from_value(rhs)?;
            match (|a: VirPyFloat, b: VirPyFloat| Ok(
                a.value <= b.value,
            ))(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Bool(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpLeImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyInt as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyInt as crate::base::Downcast>::from_value(rhs)?;
            match (|a: VirPyInt, b: VirPyInt| Ok(
                a.value >= b.value,
            ))(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Bool(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpGeImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyInt as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyFloat as crate::base::Downcast>::from_value(rhs)?;
            match (|a: VirPyInt, b: VirPyFloat| Ok(
                a.value as f64 >= b.value,
            ))(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Bool(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpGeImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyFloat as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyInt as crate::base::Downcast>::from_value(rhs)?;
            match (|a: VirPyFloat, b: VirPyInt| Ok(
                a.value >= b.value as f64,
            ))(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Bool(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpGeImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyFloat as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyFloat as crate::base::Downcast>::from_value(rhs)?;
            match (|a: VirPyFloat, b: VirPyFloat| Ok(
                a.value >= b.value,
            ))(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Bool(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpGeImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyInt as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyInt as crate::base::Downcast>::from_value(rhs)?;
            match (|a: VirPyInt, b: VirPyInt| Ok(
                a.value < b.value,
            ))(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Bool(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpLtImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyInt as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyFloat as crate::base::Downcast>::from_value(rhs)?;
            match (|a: VirPyInt, b: VirPyFloat| Ok(
                (a.value as f64) < b.value,
            ))(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Bool(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpLtImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyFloat as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyInt as crate::base::Downcast>::from_value(rhs)?;
            match (|a: VirPyFloat, b: VirPyInt| Ok(
                a.value < b.value as f64,
            ))(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Bool(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpLtImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyFloat as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyFloat as crate::base::Downcast>::from_value(rhs)?;
            match (|a: VirPyFloat, b: VirPyFloat| Ok(
                a.value < b.value,
            ))(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Bool(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpLtImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyInt as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyInt as crate::base::Downcast>::from_value(rhs)?;
            match (|a: VirPyInt, b: VirPyInt| Ok(
                a.value > b.value,
            ))(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Bool(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpGtImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyInt as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyFloat as crate::base::Downcast>::from_value(rhs)?;
            match (|a: VirPyInt, b: VirPyFloat| Ok(
                a.value as f64 > b.value,
            ))(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Bool(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpGtImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyFloat as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyInt as crate::base::Downcast>::from_value(rhs)?;
            match (|a: VirPyFloat, b: VirPyInt| Ok(
                a.value > b.value as f64,
            ))(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Bool(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpGtImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyFloat as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyFloat as crate::base::Downcast>::from_value(rhs)?;
            match (|a: VirPyFloat, b: VirPyFloat| Ok(
                a.value > b.value,
            ))(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Bool(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpGtImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyInt as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyInt as crate::base::Downcast>::from_value(rhs)?;
            match (|a: VirPyInt, b: VirPyInt| Ok(
                a.value != b.value,
            ))(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Bool(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpNeImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyInt as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyFloat as crate::base::Downcast>::from_value(rhs)?;
            match (|a: VirPyInt, b: VirPyFloat| Ok(
                a.value as f64 != b.value,
            ))(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Bool(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpNeImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyFloat as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyInt as crate::base::Downcast>::from_value(rhs)?;
            match (|a: VirPyFloat, b: VirPyInt| Ok(
                a.value != b.value as f64,
            ))(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Bool(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpNeImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyFloat as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyFloat as crate::base::Downcast>::from_value(rhs)?;
            match (|a: VirPyFloat, b: VirPyFloat| Ok(
                a.value != b.value,
            ))(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Bool(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpNeImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyInt as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyInt as crate::base::Downcast>::from_value(rhs)?;
            match (|a: VirPyInt, b: VirPyInt| Ok(
                VirPyInt::new(a.value << b.value),
            ))(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(crate::base::ValueContainer::new(ValueKind::Int(result), arena))
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpBslImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyInt as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyInt as crate::base::Downcast>::from_value(rhs)?;
            match (|a: VirPyInt, b: VirPyInt| Ok(
                VirPyInt::new(a.value >> b.value),
            ))(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(crate::base::ValueContainer::new(ValueKind::Int(result), arena))
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpBsrImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyInt as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyInt as crate::base::Downcast>::from_value(rhs)?;
            match (|a: VirPyInt, b: VirPyInt| Ok(
                VirPyInt::new(a.value & b.value),
            ))(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(crate::base::ValueContainer::new(ValueKind::Int(result), arena))
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpBitwiseAndImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            lhs: crate::base::Value<'ctx>,
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let lhs_val = <VirPyInt as crate::base::Downcast>::from_value(lhs)?;
            let rhs_val = <VirPyInt as crate::base::Downcast>::from_value(rhs)?;
            match (|a: VirPyInt, b: VirPyInt| Ok(
                VirPyInt::new(a.value | b.value),
            ))(lhs_val.clone(), rhs_val.clone()) {
                Ok(result) => {
                    Some(crate::base::ValueContainer::new(ValueKind::Int(result), arena))
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpBitwiseOrImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let rhs_val = <bool as crate::base::Downcast>::from_value(rhs)?;
            match (|a: bool| Ok(!a))(rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Bool(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpNotImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let rhs_val = <VirPyInt as crate::base::Downcast>::from_value(rhs)?;
            match (|a: VirPyInt| Ok(VirPyInt::new(a.value)))(rhs_val.clone()) {
                Ok(result) => {
                    Some(crate::base::ValueContainer::new(ValueKind::Int(result), arena))
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpPosImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let rhs_val = <VirPyFloat as crate::base::Downcast>::from_value(rhs)?;
            match (|a: VirPyFloat| Ok(VirPyFloat::new(a.value)))(rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Float(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpPosImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let rhs_val = <VirPyInt as crate::base::Downcast>::from_value(rhs)?;
            match (|a: VirPyInt| Ok(VirPyInt::new(-a.value)))(rhs_val.clone()) {
                Ok(result) => {
                    Some(crate::base::ValueContainer::new(ValueKind::Int(result), arena))
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpNegImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
    const _: () = {
        fn _op_impl<'ctx>(
            rhs: crate::base::Value<'ctx>,
            arena: &'ctx ::bumpalo::Bump,
        ) -> Option<crate::base::Value<'ctx>> {
            let rhs_val = <VirPyFloat as crate::base::Downcast>::from_value(rhs)?;
            match (|a: VirPyFloat| Ok(VirPyFloat::new(-a.value)))(rhs_val.clone()) {
                Ok(result) => {
                    Some(
                        crate::base::ValueContainer::new(ValueKind::Float(result), arena),
                    )
                }
                Err(err) => {
                    Some(
                        crate::base::ValueContainer::new(
                            crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ),
                    )
                }
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    crate::op::OpNegImpl {
                        function: _op_impl,
                    }
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
    };
}
mod error {
    use crate::base::{Downcast, Value};
    pub enum SandboxExecutionError {
        TimeoutError,
        ReferenceNotExistError(String),
        DivideByZeroError,
        GenericPanicRewindError,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for SandboxExecutionError {
        #[inline]
        fn clone(&self) -> SandboxExecutionError {
            match self {
                SandboxExecutionError::TimeoutError => {
                    SandboxExecutionError::TimeoutError
                }
                SandboxExecutionError::ReferenceNotExistError(__self_0) => {
                    SandboxExecutionError::ReferenceNotExistError(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                SandboxExecutionError::DivideByZeroError => {
                    SandboxExecutionError::DivideByZeroError
                }
                SandboxExecutionError::GenericPanicRewindError => {
                    SandboxExecutionError::GenericPanicRewindError
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for SandboxExecutionError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                SandboxExecutionError::TimeoutError => {
                    ::core::fmt::Formatter::write_str(f, "TimeoutError")
                }
                SandboxExecutionError::ReferenceNotExistError(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ReferenceNotExistError",
                        &__self_0,
                    )
                }
                SandboxExecutionError::DivideByZeroError => {
                    ::core::fmt::Formatter::write_str(f, "DivideByZeroError")
                }
                SandboxExecutionError::GenericPanicRewindError => {
                    ::core::fmt::Formatter::write_str(f, "GenericPanicRewindError")
                }
            }
        }
    }
    pub type Result<T> = ::core::result::Result<T, SandboxExecutionError>;
    impl<'ctx> Downcast<'ctx> for SandboxExecutionError {
        fn from_value(value: Value<'ctx>) -> Option<&'ctx Self> {
            value.as_error()
        }
    }
}
mod exec_ctx {
    use crate::base::Value;
    use crate::builtin::Mapping;
    use crate::error::SandboxExecutionError;
    use bumpalo::Bump;
    use std::cell::RefCell;
    use std::rc::Rc;
    pub type Result<T> = core::result::Result<T, SandboxExecutionError>;
    pub struct ExecutionContext<'ctx> {
        pub arena: Rc<RefCell<Bump>>,
        pub ttl: i64,
        pub mapping: Vec<Rc<RefCell<Mapping<'ctx>>>>,
    }
    #[automatically_derived]
    impl<'ctx> ::core::fmt::Debug for ExecutionContext<'ctx> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "ExecutionContext",
                "arena",
                &self.arena,
                "ttl",
                &self.ttl,
                "mapping",
                &&self.mapping,
            )
        }
    }
    #[automatically_derived]
    impl<'ctx> ::core::clone::Clone for ExecutionContext<'ctx> {
        #[inline]
        fn clone(&self) -> ExecutionContext<'ctx> {
            ExecutionContext {
                arena: ::core::clone::Clone::clone(&self.arena),
                ttl: ::core::clone::Clone::clone(&self.ttl),
                mapping: ::core::clone::Clone::clone(&self.mapping),
            }
        }
    }
    impl<'ctx> ExecutionContext<'ctx> {
        pub fn new(
            arena: Rc<RefCell<Bump>>,
            ttl: i64,
            mapping: Vec<Rc<RefCell<Mapping<'ctx>>>>,
        ) -> Self {
            Self { arena, ttl, mapping }
        }
        pub fn consume_one(&mut self) -> Result<()> {
            self.consume(1)
        }
        pub fn consume(&mut self, amount: i64) -> Result<()> {
            if amount > self.ttl {
                return Err(SandboxExecutionError::TimeoutError);
            }
            self.ttl -= amount;
            Ok(())
        }
        pub fn get(&self, name: &str) -> Result<Rc<RefCell<Value<'ctx>>>> {
            let mut r: Option<Rc<RefCell<Value<'ctx>>>> = None;
            for mapping in self.mapping.clone() {
                if mapping.borrow().mapping.contains_key(name) {
                    r = Some(mapping.borrow().mapping.get(name).unwrap().clone());
                }
            }
            match r {
                Some(v) => Ok(v),
                None => {
                    Err(SandboxExecutionError::ReferenceNotExistError(name.to_string()))
                }
            }
        }
    }
}
pub mod export {
    /// A trait for types that can be exported to a standard Rust type `T`.
    pub trait Export<T> {
        /// Performs the conversion.
        fn export(&self) -> T;
    }
}
