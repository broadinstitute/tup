pub(crate) mod builtin;
pub(crate) mod util;

use std::rc::Rc;
use jati::trees::types::Type;
use jati::runtime::fun::Fun as JatiFun;
use jati::trees::symbols::ArgsFailure;
use crate::Error;
use crate::lang::env::Env;
use crate::lang::value::Value;

#[derive(Clone)]
pub(crate) struct FunRef {
    pub(crate) name: String,
    fun: Rc<dyn Fun>,
}

impl FunRef {
    pub(crate) fn fun(&self) -> Rc<dyn Fun> { self.fun.clone() }
}

pub(crate) trait Fun {
    fn into_fun_ref(self, name: String) -> FunRef;
    fn tpe(&self) -> Type;
    fn check_arg_types(&self, arg_types: &[Type]) -> Result<(), ArgsFailure>;
    fn call(&self, args: Vec<Value>, env: &Env) -> Result<Value, Error>;
}

impl JatiFun for FunRef {
    fn tpe(&self) -> Type { self.fun.tpe() }
}
