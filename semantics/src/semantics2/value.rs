use std::marker::PhantomData;
use bitvec::bitvec;
use bitvec::prelude::BitVec;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Value<'arena> {
    inner: BitVec,
    //phantom is for when/if higher perf bitvec is needed
    phantom: PhantomData<&'arena ()>,
}


impl<'arena> Value<'arena> {
    pub fn width(&self) -> usize {
        self.inner.len()
    }

    pub fn one_one() -> Self {
        Self {
            inner: bitvec![1;1],
            phantom: PhantomData::default(),
        }
    }

    pub fn new(bitvec: BitVec) -> Self {
        Self {
            inner: bitvec,
            phantom: PhantomData::default(),
        }
    }

    pub fn is_true(&self) -> bool {
        assert!(self.width() == 1);
        self == &Self::one_one()
    }
}
