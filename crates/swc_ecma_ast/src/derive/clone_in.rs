use crate::Ast;

pub trait CloneIn: Sized {
    type Cloned;

    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned;
}

impl<T: CloneIn> CloneIn for Vec<T> {
    type Cloned = Vec<T::Cloned>;

    fn clone_in(&self, _ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
