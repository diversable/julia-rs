
//! Module providing wrappers for the native Julia exceptions.

use std::fmt;
use std::error;
use std::ops::Deref;
use std::ops::DerefMut;

use smallvec::SmallVec;

use sys::*;
use error::Result;
use string::IntoCString;
use super::{Ref, Symbol, Datatype};

/// Enum containing different Julia exceptions wrapped as a Ref.
#[derive(Clone)]
pub enum Exception {
    /// The parameters to a function call do not match a valid signature
    Argument(Ref),
    /// Attempt to access index out-of-bounds
    Bounds(Ref),
    /// Composite exception
    Composite(Ref),
    /// Divide by zero
    Divide(Ref),
    /// The argument is outside of the valid domain
    Domain(Ref),
    /// No more data is available from file or stream
    EOF(Ref),
    /// Generic error occurred
    Error(Ref),
    /// Type conversion cannot be done exactly
    Inexact(Ref),
    /// An error occurred when running a module's __init__
    Init(Ref),
    /// The process was stopped by a terminal interrupt (^C)
    Interrupt(Ref),
    /// The program reached an invalid exception
    InvalidState(Ref),
    /// Key doesn't exist in Associative- or Set-like object
    Key(Ref),
    /// An error occurred while include-ing, require-ing or using a file
    Load(Ref),
    /// Operation allocated too much memory
    OutOfMemory(Ref),
    /// Operation tried to write to read-only memory
    ReadOnlyMemory(Ref),
    /// Remote exception occurred
    Remote(Ref),
    /// Method with the required type signature doesn't exist
    Method(Ref),
    /// The result of an expression is too large
    Overflow(Ref),
    /// The expression couldn't be parsed as a valid Julia expression
    Parse(Ref),
    /// System call failed
    System(Ref),
    /// Type assertion failed
    Type(Ref),
    /// The item or field is not defined
    UndefRef(Ref),
    /// Symbol is not defined in current scope
    UndefVar(Ref),
    /// Byte array does not represent a valid unicode string
    Unicode(Ref),
    /// Unknown exception
    Unknown(Ref),
}

impl Exception {
    pub fn throw(&self) -> Result<()> {
        let raw = self.inner_ref().lock()?;

        unsafe {
            jl_throw(raw);
        }

        Ok(())
    }

    pub fn rethrow(&self) -> Result<()> {
        let raw = self.inner_ref().lock()?;

        unsafe {
            jl_rethrow_other(raw);
        }

        Ok(())
    }

    /// Check if an exception occurred without checking its value.
    pub fn occurred() -> bool {
        unsafe { !jl_exception_occurred().is_null() }
    }

    /// Catch an exception if it occurred. Returns None if no exception
    /// occurred.
    pub fn catch() -> Option<Exception> {
        let raw = unsafe { jl_exception_occurred() };
        unsafe {
            jl_exception_clear();
        }
        if raw.is_null() {
            None
        } else {
            Exception::with_value(Ref::new(raw)).ok()
        }
    }

    // TODO: replace comparing typename with comparing a *mut jl_datatype_t.
    /// Construct a new Exception with a wrapped Julia value.
    pub fn with_value(value: Ref) -> Result<Exception> {
        let typename = value.typename()?;
        let ex = match typename.as_str() {
            "ArgumentError" => Exception::Argument(value),
            "BoundsError" => Exception::Bounds(value),
            "CompositeException" => Exception::Composite(value),
            "DivideError" => Exception::Divide(value),
            "DomainError" => Exception::Domain(value),
            "EOFError" => Exception::EOF(value),
            "ErrorException" => Exception::Error(value),
            "InexactError" => Exception::Inexact(value),
            "InitError" => Exception::Init(value),
            "InterruptException" => Exception::Interrupt(value),
            "InvalidStateException" => Exception::InvalidState(value),
            "KeyError" => Exception::Key(value),
            "LoadError" => Exception::Load(value),
            "OutOfMemoryError" => Exception::OutOfMemory(value),
            "ReadOnlyMemoryError" => Exception::ReadOnlyMemory(value),
            "RemoteException" => Exception::Remote(value),
            "MethodError" => Exception::Method(value),
            "OverflowError" => Exception::Overflow(value),
            "ParseError" => Exception::Parse(value),
            "SystemError" => Exception::System(value),
            "TypeError" => Exception::Type(value),
            "UndefRefError" => Exception::UndefRef(value),
            "UndefVarError" => Exception::UndefVar(value),
            "UnicodeError" => Exception::Unicode(value),
            _ => Exception::Unknown(value),
        };
        Ok(ex)
    }

    /// Immutably borrows the inner value.
    pub fn inner_ref(&self) -> &Ref {
        match *self {
            Exception::Argument(ref value) => value,
            Exception::Bounds(ref value) => value,
            Exception::Composite(ref value) => value,
            Exception::Divide(ref value) => value,
            Exception::Domain(ref value) => value,
            Exception::EOF(ref value) => value,
            Exception::Error(ref value) => value,
            Exception::Inexact(ref value) => value,
            Exception::Init(ref value) => value,
            Exception::Interrupt(ref value) => value,
            Exception::InvalidState(ref value) => value,
            Exception::Key(ref value) => value,
            Exception::Load(ref value) => value,
            Exception::OutOfMemory(ref value) => value,
            Exception::ReadOnlyMemory(ref value) => value,
            Exception::Remote(ref value) => value,
            Exception::Method(ref value) => value,
            Exception::Overflow(ref value) => value,
            Exception::Parse(ref value) => value,
            Exception::System(ref value) => value,
            Exception::Type(ref value) => value,
            Exception::UndefRef(ref value) => value,
            Exception::UndefVar(ref value) => value,
            Exception::Unicode(ref value) => value,
            Exception::Unknown(ref value) => value,
        }
    }

    /// Mutably borrows the inner value.
    pub fn inner_mut(&mut self) -> &mut Ref {
        match *self {
            Exception::Argument(ref mut value) => value,
            Exception::Bounds(ref mut value) => value,
            Exception::Composite(ref mut value) => value,
            Exception::Divide(ref mut value) => value,
            Exception::Domain(ref mut value) => value,
            Exception::EOF(ref mut value) => value,
            Exception::Error(ref mut value) => value,
            Exception::Inexact(ref mut value) => value,
            Exception::Init(ref mut value) => value,
            Exception::Interrupt(ref mut value) => value,
            Exception::InvalidState(ref mut value) => value,
            Exception::Key(ref mut value) => value,
            Exception::Load(ref mut value) => value,
            Exception::OutOfMemory(ref mut value) => value,
            Exception::ReadOnlyMemory(ref mut value) => value,
            Exception::Remote(ref mut value) => value,
            Exception::Method(ref mut value) => value,
            Exception::Overflow(ref mut value) => value,
            Exception::Parse(ref mut value) => value,
            Exception::System(ref mut value) => value,
            Exception::Type(ref mut value) => value,
            Exception::UndefRef(ref mut value) => value,
            Exception::UndefVar(ref mut value) => value,
            Exception::Unicode(ref mut value) => value,
            Exception::Unknown(ref mut value) => value,
        }
    }

    /// Consumes self and returns the inner value.
    pub fn into_inner(self) -> Ref {
        match self {
            Exception::Argument(value) => value,
            Exception::Bounds(value) => value,
            Exception::Composite(value) => value,
            Exception::Divide(value) => value,
            Exception::Domain(value) => value,
            Exception::EOF(value) => value,
            Exception::Error(value) => value,
            Exception::Inexact(value) => value,
            Exception::Init(value) => value,
            Exception::Interrupt(value) => value,
            Exception::InvalidState(value) => value,
            Exception::Key(value) => value,
            Exception::Load(value) => value,
            Exception::OutOfMemory(value) => value,
            Exception::ReadOnlyMemory(value) => value,
            Exception::Remote(value) => value,
            Exception::Method(value) => value,
            Exception::Overflow(value) => value,
            Exception::Parse(value) => value,
            Exception::System(value) => value,
            Exception::Type(value) => value,
            Exception::UndefRef(value) => value,
            Exception::UndefVar(value) => value,
            Exception::Unicode(value) => value,
            Exception::Unknown(value) => value,
        }
    }
}

impl Deref for Exception {
    type Target = Ref;
    fn deref(&self) -> &Ref {
        self.inner_ref()
    }
}

impl DerefMut for Exception {
    fn deref_mut(&mut self) -> &mut Ref {
        self.inner_mut()
    }
}

impl fmt::Debug for Exception {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let typename = self.typename().map_err(|_| fmt::Error)?;
        write!(f, "{}", typename)
    }
}

// TODO
impl fmt::Display for Exception {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl error::Error for Exception {
    fn description(&self) -> &str {
        match *self {
            Exception::Argument(_) => {
                "the parameters to a function call do not match a valid signature"
            }
            Exception::Bounds(_) => "attempt to access index out-of-bounds",
            Exception::Composite(_) => "composite exception",
            Exception::Divide(_) => "divide by zero",
            Exception::Domain(_) => "the argument is outside of the valid domain",
            Exception::EOF(_) => "no more data is available from file or stream",
            Exception::Error(_) => "generic error occurred",
            Exception::Inexact(_) => "type conversion cannot be done exactly",
            Exception::Init(_) => "an error occurred when running a module's __init__ ",
            Exception::Interrupt(_) => "the process was stopped by a terminal interrupt (^C)",
            Exception::InvalidState(_) => "the program reached an invalid exception",
            Exception::Key(_) => "key doesn't exist in Associative- or Set-like object",
            Exception::Load(_) => {
                "an error occurred while include-ing, require-ing or using a file"
            }
            Exception::OutOfMemory(_) => "operation allocated too much memory",
            Exception::ReadOnlyMemory(_) => "operation tried to write to read-only memory",
            Exception::Remote(_) => "remote exception occurred",
            Exception::Method(_) => "method with the required type signature doesn't exist",
            Exception::Overflow(_) => "the result of an expression is too large",
            Exception::Parse(_) => "the expression couldn't be parsed as a valid Julia expression",
            Exception::System(_) => "system call failed",
            Exception::Type(_) => "type assertion failed",
            Exception::UndefRef(_) => "the item or field is not defined",
            Exception::UndefVar(_) => "symbol is not defined in current scope",
            Exception::Unicode(_) => "byte array does not represent a valid unicode string",
            Exception::Unknown(_) => "unknown exception",
        }
    }
}

/// Throws a generic error.
pub fn error<S: IntoCString>(string: S) {
    let string = string.into_cstring();
    let string = string.as_ptr();
    unsafe {
        jl_error(string);
    }
}

/// Throws a formatted generic error.
pub fn error_format(args: fmt::Arguments) {
    error(fmt::format(args).into_cstring());
}

/// Throws an exception with the specified Datatype and message.
pub fn exception<S: IntoCString>(ty: &Datatype, string: S) -> Result<()> {
    let ty = ty.lock()?;
    let string = string.into_cstring();
    let string = string.as_ptr();
    unsafe {
        jl_exceptionf(ty, string);
    }
    Ok(())
}

/// Throws an exception with the specified Datatype and a formatted message.
pub fn exception_format(ty: &Datatype, args: fmt::Arguments) -> Result<()> {
    exception(ty, fmt::format(args).into_cstring())
}

/// Too few arguments exception.
pub fn too_few_args<S: IntoCString>(fname: S, min: usize) {
    let fname = fname.into_cstring();
    let fname = fname.as_ptr();
    unsafe {
        jl_too_few_args(fname, min as i32);
    }
}

/// Too many arguments exception.
pub fn too_many_args<S: IntoCString>(fname: S, max: usize) {
    let fname = fname.into_cstring();
    let fname = fname.as_ptr();
    unsafe {
        jl_too_many_args(fname, max as i32);
    }
}

/// Invalid type in an expression.
pub fn type_error<S: IntoCString>(fname: S, expected: &Ref, got: &Ref) -> Result<()> {
    let fname = fname.into_cstring();
    let fname = fname.as_ptr();
    let expected = expected.lock()?;
    let got = got.lock()?;
    unsafe {
        jl_type_error(fname, expected, got);
    }
    Ok(())
}

pub fn type_error_rt<S: IntoCString>(fname: S, context: S, ty: &Ref, got: &Ref) -> Result<()> {
    let fname = fname.into_cstring();
    let fname = fname.as_ptr();
    let context = context.into_cstring();
    let context = context.as_ptr();
    let ty = ty.lock()?;
    let got = got.lock()?;
    unsafe {
        jl_type_error_rt(fname, context, ty, got);
    }
    Ok(())
}

/// No value is bound to this symbol.
pub fn undefined_var_error(var: &Symbol) -> Result<()> {
    let var = var.lock()?;
    unsafe {
        jl_undefined_var_error(var);
    }
    Ok(())
}

/// Index ouf of bound.
pub fn bounds_error(v: &Ref, idx: &Ref) -> Result<()> {
    let v = v.lock()?;
    let idx = idx.lock()?;
    unsafe {
        jl_bounds_error(v, idx);
    }
    Ok(())
}

pub fn bounds_error_v(v: &Ref, idxs: &[Ref]) -> Result<()> {
    let v = v.lock()?;
    let mut indices = SmallVec::<[*mut jl_value_t; 8]>::new();
    for i in idxs {
        indices.push(i.lock()?)
    }
    let nidxs = indices.len();
    let idxs = indices.as_mut_ptr();
    unsafe {
        jl_bounds_error_v(v, idxs, nidxs);
    }
    Ok(())
}

/// Index out of bound.
pub fn bounds_error_int(v: &Ref, i: usize) -> Result<()> {
    let v = v.lock()?;
    unsafe {
        jl_bounds_error_int(v, i);
    }
    Ok(())
}

pub fn bounds_error_tuple_int(v: &[Ref], i: usize) -> Result<()> {
    let mut vs = SmallVec::<[*mut jl_value_t; 8]>::new();
    for vi in v {
        vs.push(vi.lock()?);
    }
    let nv = vs.len();
    let v = vs.as_mut_ptr();
    unsafe {
        jl_bounds_error_tuple_int(v, nv, i);
    }
    Ok(())
}

// TODO
/*
pub fn bounds_error_unboxed_int(void *v, vt: &Ref, i: usize) -> Result<()> {
    let vt = vt.lock()?;
    unsafe {
        jl_bounds_error_unboxed_int();
    }
}
*/

pub fn bounds_error_ints(v: &Ref, idxs: &[usize]) -> Result<()> {
    let v = v.lock()?;
    let nidxs = idxs.len();
    let idxs = idxs.as_ptr() as *mut _;
    unsafe {
        jl_bounds_error_ints(v, idxs, nidxs);
    }
    Ok(())
}

/// Unexpected End of File.
pub fn eof_error() {
    unsafe {
        jl_eof_error();
    }
}
