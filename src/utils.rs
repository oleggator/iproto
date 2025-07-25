use sharded_slab::Slab;

/// SlabEntryGuard ensures that slab entry will be deleted after guard drops
pub(crate) struct SlabEntryGuard<'a, T> {
    /// entry id to remove
    idx: usize,

    /// slab to remove from
    slab: &'a Slab<T>,
}

impl<'a, T> SlabEntryGuard<'a, T> {
    pub fn new(idx: usize, slab: &'a Slab<T>) -> Self {
        Self { idx, slab }
    }
}

impl<E> Drop for SlabEntryGuard<'_, E> {
    fn drop(&mut self) {
        self.slab.remove(self.idx);
    }
}
