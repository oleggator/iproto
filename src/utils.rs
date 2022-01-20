use std::sync::Arc;
use sharded_slab::{Clear, Pool};
use sharded_slab::pool::{OwnedRef, Ref};

#[derive(Debug)]
pub(crate) struct PoolEntryBox<T: Default + Clear> {
    idx: usize,
    pool: Arc<Pool<T>>,
}

impl<T: Default + Clear> PoolEntryBox<T> {
    pub fn new(idx: usize, pool: Arc<Pool<T>>) -> Self {
        Self { idx, pool }
    }

    pub fn get_owned(&self) -> Option<OwnedRef<T>> {
        self.pool.clone().get_owned(self.idx)
    }

    pub fn get(&self) -> Option<Ref<'_, T>> {
        self.pool.get(self.idx)
    }
}

impl<T: Default + Clear> Drop for PoolEntryBox<T> {
    fn drop(&mut self) {
        self.pool.clear(self.idx);
    }
}