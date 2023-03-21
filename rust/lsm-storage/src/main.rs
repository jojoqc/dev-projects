//BLOCK ENCODING
impl BlockBuilder{
    /// Adds a key-value pair to the block. Returns false when the block is full.
    #[must_use]
    pub fn add(&mut self, key: &[u8], value: &[u8]) -> bool {
        unimplemented!()
    }
    /// Finalize the block.
    pub fn build(self) -> Block {}
}

impl BlockIterator {
    /// Returns the key of the current entry.
    pub fn key(&self) -> &[u8] {}
    /// Returns the value of the current entry.
    pub fn value(&self) -> &[u8] {}
    /// Move to the next key in the block.
    pub fn next(&mut self) {}
    /// Seek to the first key that >= `key`.
    pub fn seek_to_key(&mut self, key: &[u8]) {}
}

//TABLE ENCODING
impl SsTableBuilder {
    /// Adds a key-value pair to SSTable
    pub fn add(&mut self, key: &[u8], value: &[u8]) {}
    /// Builds the SSTable and writes it to the given path. No need to actually write to disk until
    /// chapter 4 block cache.
    pub fn build(self, ...) -> Result<SsTable> {}
}

impl StorageIterator for SsTableIterator {
    fn value(&self) -> &[u8] {}
    fn key(&self) -> &[u8] {}
    fn is_valid(&self) -> bool {}
    fn next(&mut self) -> Result<()> {}
}

//Mem Table and Merge iterators
/// Merge multiple iterators of the same type. If the same key occurs multiple times in some
/// iterators, prefer the one with smaller index.
pub struct MergeIterator<I: StorageIterators>{
    iters: BinaryHeap<HeapWrapper<I>>,
    current: HeapWrapper<I>,
}
/// Merges two iterators of different types into one. If the two iterators have the same key, only
/// produce the key once and prefer the entry from A.
pub struct TwoMergerIterator<A: StorageIterator, B:StorageIterator>{
    a: A,
    b: B,
}

pub struct MemTable{
    map: crossbeam_skiplist::SkipMap<Bytes, Bytes>,
}
//Storage Engine and Block Cache
impl LsmStorage {
    //Get a key from the storage. in day 7, this can be further optimized by using a bloom filter
    pub fn get(&self, key: &[u8]) -> Result<Option<Bytes>> {}
    //put a key-value pair into the storage by writing into the current memtable
    pub fn put(&self, key: &[u8], value:&[u8]) -> Result<Option> {}
    //Remove a key from the storage by writing an empty value
    pub fn delete(&self, _key: &[u8]) -> Result<Option> {}

    //Persist data to disk
    pub fn sync(&self)->Result<()>{}

    //Create an iterator over a range of keys
    pub fn scan(&self, _lower: Bound<&[u8]>, _upper: Bound<&[u8]>)-> Result<FusedIterator<LsmIterator>>{}

}

//Compaction
//Recovery
//Optimizations
