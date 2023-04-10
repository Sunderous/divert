// use bitflags::bitflags;

use crate::detour::{DtNavMesh, DtStatus};

pub enum DtTileCache {}

// pub enum DtTileCacheBuilder {}

#[link(name = "detour_tile_cache", kind = "static")]
extern "C" {
    pub fn dtTileCache_update(
        _self: *mut DtTileCache,
        dt: f32,
        navmesh: *mut DtNavMesh,
        up_to_date: *const bool,
    ) -> DtStatus;

    pub fn dtTileCache_free(_self: *mut DtTileCache);
}
