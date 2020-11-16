use std::iter;

pub type Marks = u16;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Subjects {
    pub maths: Marks,
    pub physics: Marks,
    pub english: Marks,
}

impl Subjects {
    pub fn marks(&self) -> impl Iterator<Item = u16> {
        let m = iter::once(self.maths);
        let p = iter::once(self.physics);
        let e = iter::once(self.english);
        m.chain(p).chain(e)
    }
}
