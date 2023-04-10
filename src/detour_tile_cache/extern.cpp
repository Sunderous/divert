#include "../../recastnavigation/DetourTileCache/Include/DetourTileCache.h"
#include "../../recastnavigation/DetourTileCache/Include/DetourTileCacheBuilder.h"
#include "../../recastnavigation/Detour/Include/DetourNavMesh.h"

extern "C"
{
    dtStatus dtTileCache_update(dtTileCache* tileCache, const float dt, dtNavMesh* navmesh, bool* upToDate = 0)
    {
        return tileCache->update(dt, navmesh, upToDate);
    }

    void dtTileCache_free(dtTileCache *tileCache) {
        return dtFreeTileCache(tileCache);
    }
}