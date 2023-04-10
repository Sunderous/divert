mod binding;

use std::marker;

use binding::DtTileCache;

use crate::detour::NavMesh;
use crate::detour::{DivertError, DivertResult, DtStatus};

use self::binding::*;

/// Safe bindings to dtNavMesh
/// Handles life time of the dtNavMesh and will release resources when dropped
pub struct TileCache<'a> {
    handle: *mut DtTileCache,
    _phantom: marker::PhantomData<&'a DtTileCache>,
}

unsafe impl Send for TileCache<'_> {}

impl<'a> TileCache<'a> {
    pub fn update(&self, navmesh: NavMesh, mut up_to_date: bool) -> DivertResult<DtStatus> {
        let status =
            unsafe { dtTileCache_update(self.handle, 0f32, navmesh.handle, &mut up_to_date) };

        if status.is_failed() {
            return Err(DivertError::Failure(status));
        }

        Ok(status)
    }
}

impl<'a> Drop for TileCache<'a> {
    /// Frees dtNavMesh resources with dtFreeNavMesh
    fn drop(&mut self) {
        unsafe { dtTileCache_free(self.handle) }
    }
}
