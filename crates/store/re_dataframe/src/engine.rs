use re_chunk::{EntityPath, TransportChunk};
use re_chunk_store::{ChunkStore, ChunkStoreHandle, ColumnDescriptor, QueryExpression};
use re_log_types::EntityPathFilter;
use re_query::{QueryCache, QueryCacheHandle};

use crate::QueryHandle;

// Used all over in docstrings.
#[allow(unused_imports)]
use re_chunk_store::ComponentColumnDescriptor;

// ---

// TODO(#3741): `arrow2` has no concept of a `RecordBatch`, so for now we just use our trustworthy
// `TransportChunk` type until we migrate to `arrow-rs`.
// `TransportChunk` maps 1:1 to `RecordBatch` so the switch (and the compatibility layer in the meantime)
// will be trivial.
pub type RecordBatch = TransportChunk;

// --- Queries ---

/// A handle to our user-facing query engine.
///
/// Cheap to clone.
///
/// See the following methods:
/// * [`QueryEngine::schema`]: get the complete schema of the recording.
/// * [`QueryEngine::query`]: execute a [`QueryExpression`] on the recording.
//
// TODO: this should be StorageEngine
#[derive(Clone)]
pub struct QueryEngine {
    store: ChunkStoreHandle,
    cache: QueryCacheHandle,
}

// TODO: explain why these exist -> multiple reasons

pub struct QueryEngineReadGuard<'a> {
    pub store: parking_lot::RwLockReadGuard<'a, ChunkStore>,
    pub cache: parking_lot::RwLockReadGuard<'a, QueryCache>,
}

pub struct QueryEngineWriteGuard<'a> {
    pub store: parking_lot::RwLockWriteGuard<'a, ChunkStore>,
    pub cache: parking_lot::RwLockWriteGuard<'a, QueryCache>,
}

impl<'a> QueryEngineWriteGuard<'a> {
    #[inline]
    pub fn downgrade(self) -> QueryEngineReadGuard<'a> {
        QueryEngineReadGuard {
            store: parking_lot::RwLockWriteGuard::downgrade(self.store),
            cache: parking_lot::RwLockWriteGuard::downgrade(self.cache),
        }
    }
}

pub struct QueryEngineArcReadGuard {
    pub store: parking_lot::ArcRwLockReadGuard<parking_lot::RawRwLock, ChunkStore>,
    pub cache: parking_lot::ArcRwLockReadGuard<parking_lot::RawRwLock, QueryCache>,
}

pub struct QueryEngineArcWriteGuard {
    pub store: parking_lot::ArcRwLockWriteGuard<parking_lot::RawRwLock, ChunkStore>,
    pub cache: parking_lot::ArcRwLockWriteGuard<parking_lot::RawRwLock, QueryCache>,
}

impl QueryEngineArcWriteGuard {
    #[inline]
    pub fn downgrade(self) -> QueryEngineArcReadGuard {
        QueryEngineArcReadGuard {
            store: parking_lot::ArcRwLockWriteGuard::downgrade(self.store),
            cache: parking_lot::ArcRwLockWriteGuard::downgrade(self.cache),
        }
    }
}

impl QueryEngine {
    #[inline]
    pub fn read(&self) -> QueryEngineReadGuard<'_> {
        QueryEngineReadGuard {
            cache: self.cache.read(),
            store: self.store.read(),
        }
    }

    #[inline]
    pub fn write(&self) -> QueryEngineWriteGuard<'_> {
        QueryEngineWriteGuard {
            cache: self.cache.write(),
            store: self.store.write(),
        }
    }

    #[inline]
    pub fn read_arc(&self) -> QueryEngineArcReadGuard {
        QueryEngineArcReadGuard {
            cache: self.cache.read_arc(),
            store: self.store.read_arc(),
        }
    }

    #[inline]
    pub fn write_arc(&self) -> QueryEngineArcWriteGuard {
        QueryEngineArcWriteGuard {
            cache: self.cache.write_arc(),
            store: self.store.write_arc(),
        }
    }
}

impl QueryEngine {
    // TODO
    pub fn new(store: ChunkStoreHandle, cache: QueryCacheHandle) -> Self {
        Self { store, cache }
    }

    /// Returns the full schema of the store.
    ///
    /// This will include a column descriptor for every timeline and every component on every
    /// entity that has been written to the store so far.
    ///
    /// The order of the columns to guaranteed to be in a specific order:
    /// * first, the time columns in lexical order (`frame_nr`, `log_time`, ...);
    /// * second, the component columns in lexical order (`Color`, `Radius, ...`).
    #[inline]
    pub fn schema(&self) -> Vec<ColumnDescriptor> {
        self.store.read().schema()
    }

    /// Returns the filtered schema for the given [`QueryExpression`].
    ///
    /// The order of the columns is guaranteed to be in a specific order:
    /// * first, the time columns in lexical order (`frame_nr`, `log_time`, ...);
    /// * second, the component columns in lexical order (`Color`, `Radius, ...`).
    #[inline]
    pub fn schema_for_query(&self, query: &QueryExpression) -> Vec<ColumnDescriptor> {
        self.store.read().schema_for_query(query)
    }

    /// Starts a new query by instantiating a [`QueryHandle`].
    #[inline]
    pub fn query(&self, query: QueryExpression) -> QueryHandle {
        QueryHandle::new(self.clone(), query)
    }

    /// Returns an iterator over all the [`EntityPath`]s present in the database.
    #[inline]
    pub fn iter_entity_paths<'a>(
        &self,
        filter: &'a EntityPathFilter,
    ) -> impl Iterator<Item = EntityPath> + 'a {
        self.store
            .read()
            .all_entities()
            .into_iter()
            .filter(|entity_path| filter.matches(entity_path))
    }
}
