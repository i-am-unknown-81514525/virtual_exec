use crate::base::{Downcast, Upcast, Value, ValueKind};
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Rem, Sub};
use std::rc::Rc;
use crate::error::Result;

#[derive(Debug, Clone, Copy)]
pub struct VirPyInt {
    pub value: i64,
}
impl VirPyInt {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct VirPyFloat {
    pub value: f64,
}
impl VirPyFloat {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

#[derive(Debug, Clone)]
pub struct Mapping<'ctx> {
    pub mapping: HashMap<String, Rc<RefCell<Value<'ctx>>>>,
}

#[derive(Debug, Clone)]
pub struct VirPyObject<'ctx> {
    pub mapping: Rc<RefCell<Mapping<'ctx>>>,
}

impl<'ctx> VirPyObject<'ctx> {
    pub fn new() -> Self {
        Self {
            mapping: Rc::new(RefCell::new(Mapping {
                mapping: HashMap::new(),
            })),
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

register_op_add!(VirPyInt, VirPyInt, ValueKind::Int);
register_op_add!(VirPyFloat, VirPyFloat, ValueKind::Float);
register_op_add!(VirPyInt, VirPyFloat, ValueKind::Float);
register_op_add!(VirPyFloat, VirPyInt, ValueKind::Float);


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

register_op_sub!(VirPyInt, VirPyInt, ValueKind::Int);
register_op_sub!(VirPyFloat, VirPyFloat, ValueKind::Float);
register_op_sub!(VirPyInt, VirPyFloat, ValueKind::Float);
register_op_sub!(VirPyFloat, VirPyInt, ValueKind::Float);


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

register_op_mul!(VirPyInt, VirPyInt, ValueKind::Int);
register_op_mul!(VirPyFloat, VirPyFloat, ValueKind::Float);
register_op_mul!(VirPyInt, VirPyFloat, ValueKind::Float);
register_op_mul!(VirPyFloat, VirPyInt, ValueKind::Float);

impl Div for VirPyInt {
    type Output = Result<VirPyFloat>;
    fn div(self, rhs: Self) -> Self::Output {
        if (rhs.value == 0) {
            return Err(crate::error::SandboxExecutionError::DivideByZeroError)
        }
        Ok(VirPyFloat::new((self.value as f64) / (rhs.value as f64)))
    }
}
impl Div for VirPyFloat {
    type Output = Result<Self>;
    fn div(self, rhs: Self) -> Self::Output {
        if (rhs.value == 0f64) {
            return Err(crate::error::SandboxExecutionError::DivideByZeroError)
        }
       Ok(Self::new(self.value / rhs.value))
    }
}
impl Div<VirPyInt> for VirPyFloat {
    type Output = Result<Self>;
    fn div(self, rhs: VirPyInt) -> Self::Output {
        if (rhs.value == 0) {
            return Err(crate::error::SandboxExecutionError::DivideByZeroError)
        }
        Ok(Self::new(self.value / (rhs.value as f64)))
    }
}
impl Div<VirPyFloat> for VirPyInt {
    type Output = Result<VirPyFloat>;
    fn div(self, rhs: VirPyFloat) -> Self::Output {
        if (rhs.value == 0f64) {
            return Err(crate::error::SandboxExecutionError::DivideByZeroError)
        }
        Ok(VirPyFloat::new(self.value as f64 / rhs.value))
    }
}

register_op_div!(VirPyInt, VirPyInt, ValueKind::Float);
register_op_div!(VirPyFloat, VirPyFloat, ValueKind::Float);
register_op_div!(VirPyInt, VirPyFloat, ValueKind::Float);
register_op_div!(VirPyFloat, VirPyInt, ValueKind::Float);

impl Rem for VirPyInt {
    type Output = Result<VirPyInt>;
    fn rem(self, rhs: Self) -> Self::Output {
        if (rhs.value == 0) {
            return Err(crate::error::SandboxExecutionError::DivideByZeroError)
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
            return Err(crate::error::SandboxExecutionError::DivideByZeroError)
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
            return Err(crate::error::SandboxExecutionError::DivideByZeroError)
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
            return Err(crate::error::SandboxExecutionError::DivideByZeroError)
        }
        let mut v = (self.value as f64) % (rhs.value);
        if (v < 0f64) {
            v += rhs.value;
        }
        Ok(VirPyFloat::new(v))
    }
}

register_op_moduls!(VirPyInt, VirPyInt, ValueKind::Int);
register_op_moduls!(VirPyFloat, VirPyFloat, ValueKind::Float);
register_op_moduls!(VirPyInt, VirPyFloat, ValueKind::Float);
register_op_moduls!(VirPyFloat, VirPyInt, ValueKind::Float);

register_op_eq!(bool, bool, ValueKind::Bool, |a, b| Ok(a==b));
register_op_le!(bool, bool, ValueKind::Bool, |a, b| Ok(a<=b));
register_op_lt!(bool, bool, ValueKind::Bool, |a, b| Ok(a<b));
register_op_ge!(bool, bool, ValueKind::Bool, |a, b| Ok(a>=b));
register_op_gt!(bool, bool, ValueKind::Bool, |a, b| Ok(a>b));
register_op_ne!(bool, bool, ValueKind::Bool, |a, b| Ok(a!=b));

register_op_eq!(VirPyInt, VirPyInt, ValueKind::Bool, |a: VirPyInt, b: VirPyInt| Ok(a.value == b.value));
register_op_eq!(VirPyInt, VirPyFloat, ValueKind::Bool, |a: VirPyInt, b: VirPyFloat| Ok(a.value as f64 == b.value));
register_op_eq!(VirPyFloat, VirPyInt, ValueKind::Bool, |a: VirPyFloat, b: VirPyInt| Ok(a.value  == b.value as f64));
register_op_eq!(VirPyFloat, VirPyFloat, ValueKind::Bool, |a: VirPyFloat, b: VirPyFloat| Ok(a.value  == b.value));

register_op_le!(VirPyInt, VirPyInt, ValueKind::Bool, |a: VirPyInt, b: VirPyInt| Ok(a.value <= b.value));
register_op_le!(VirPyInt, VirPyFloat, ValueKind::Bool, |a: VirPyInt, b: VirPyFloat| Ok(a.value as f64 <= b.value));
register_op_le!(VirPyFloat, VirPyInt, ValueKind::Bool, |a: VirPyFloat, b: VirPyInt| Ok(a.value <= b.value as f64));
register_op_le!(VirPyFloat, VirPyFloat, ValueKind::Bool, |a: VirPyFloat, b: VirPyFloat| Ok(a.value <= b.value));

register_op_ge!(VirPyInt, VirPyInt, ValueKind::Bool, |a: VirPyInt, b: VirPyInt| Ok(a.value >= b.value));
register_op_ge!(VirPyInt, VirPyFloat, ValueKind::Bool, |a: VirPyInt, b: VirPyFloat| Ok(a.value as f64 >= b.value));
register_op_ge!(VirPyFloat, VirPyInt, ValueKind::Bool, |a: VirPyFloat, b: VirPyInt| Ok(a.value >= b.value as f64));
register_op_ge!(VirPyFloat, VirPyFloat, ValueKind::Bool, |a: VirPyFloat, b: VirPyFloat| Ok(a.value >= b.value));

register_op_lt!(VirPyInt, VirPyInt, ValueKind::Bool, |a: VirPyInt, b: VirPyInt| Ok(a.value < b.value));
register_op_lt!(VirPyInt, VirPyFloat, ValueKind::Bool, |a: VirPyInt, b: VirPyFloat| Ok((a.value as f64) < b.value));
register_op_lt!(VirPyFloat, VirPyInt, ValueKind::Bool, |a: VirPyFloat, b: VirPyInt| Ok(a.value < b.value as f64));
register_op_lt!(VirPyFloat, VirPyFloat, ValueKind::Bool, |a: VirPyFloat, b: VirPyFloat| Ok(a.value < b.value));

register_op_gt!(VirPyInt, VirPyInt, ValueKind::Bool, |a: VirPyInt, b: VirPyInt| Ok(a.value > b.value));
register_op_gt!(VirPyInt, VirPyFloat, ValueKind::Bool, |a: VirPyInt, b: VirPyFloat| Ok(a.value as f64 > b.value));
register_op_gt!(VirPyFloat, VirPyInt, ValueKind::Bool, |a: VirPyFloat, b: VirPyInt| Ok(a.value > b.value as f64));
register_op_gt!(VirPyFloat, VirPyFloat, ValueKind::Bool, |a: VirPyFloat, b: VirPyFloat| Ok(a.value > b.value));

register_op_ne!(VirPyInt, VirPyInt, ValueKind::Bool, |a: VirPyInt, b: VirPyInt| Ok(a.value != b.value));
register_op_ne!(VirPyInt, VirPyFloat, ValueKind::Bool, |a: VirPyInt, b: VirPyFloat| Ok(a.value as f64 != b.value));
register_op_ne!(VirPyFloat, VirPyInt, ValueKind::Bool, |a: VirPyFloat, b: VirPyInt| Ok(a.value != b.value as f64));
register_op_ne!(VirPyFloat, VirPyFloat, ValueKind::Bool, |a: VirPyFloat, b: VirPyFloat| Ok(a.value != b.value));

register_op_bsl!(VirPyInt, VirPyInt, ValueKind::Int, |a: VirPyInt, b: VirPyInt| Ok(VirPyInt::new(a.value << b.value)));
register_op_bsr!(VirPyInt, VirPyInt, ValueKind::Int, |a: VirPyInt, b: VirPyInt| Ok(VirPyInt::new(a.value >> b.value)));
register_op_band!(VirPyInt, VirPyInt, ValueKind::Int, |a: VirPyInt, b: VirPyInt| Ok(VirPyInt::new(a.value & b.value)));
register_op_bor!(VirPyInt, VirPyInt, ValueKind::Int, |a: VirPyInt, b: VirPyInt| Ok(VirPyInt::new(a.value | b.value)));
// register_op_bsl!(VirPyFloat, VirPyInt, ValueKind::Float, |a: VirPyFloat, b: VirPyInt| Ok(VirPyFloat::new(a.value << b.value)));
// register_op_bsr!(VirPyFloat, VirPyInt, ValueKind::Float, |a: VirPyFloat, b: VirPyInt| Ok(VirPyFloat::new(a.value >> b.value)));
// Best attempt would be using i64::pow, but overflow would be a big issue

register_op_not!(bool, ValueKind::Bool, |a: bool| Ok(!a));
register_op_pos!(VirPyInt, ValueKind::Int, |a: VirPyInt| Ok(VirPyInt::new(a.value)));
register_op_pos!(VirPyFloat, ValueKind::Float, |a: VirPyFloat| Ok(VirPyFloat::new(a.value)));
register_op_neg!(VirPyInt, ValueKind::Int, |a: VirPyInt| Ok(VirPyInt::new(-a.value)));
register_op_neg!(VirPyFloat, ValueKind::Float, |a: VirPyFloat| Ok(VirPyFloat::new(-a.value)));