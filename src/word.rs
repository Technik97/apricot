use crate::forth::Forth;

#[derive(Clone)]
pub enum Word {
    BuiltIn(fn(&mut Forth)),
    UserDefined(Vec<String>),
}
