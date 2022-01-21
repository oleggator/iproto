use std::sync::Arc;
use sharded_slab::{Clear, Pool, Slab};
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


// SlabEntryGuard ensures that slab entry will be deleted after guard drops
pub(crate) struct SlabEntryGuard<'a, T> {
    // entry id to remove
    idx: usize,

    // slab to remove from
    slab: &'a Slab<T>,
}

impl<'a, T> SlabEntryGuard<'a, T> {
    pub fn new(idx: usize, slab: &'a Slab<T>) -> Self {
        Self { idx, slab }
    }
}

impl<'a, E> Drop for SlabEntryGuard<'a, E> {
    fn drop(&mut self) {
        self.slab.remove(self.idx);
    }
}
