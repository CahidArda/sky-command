"use client";

import { useMemo } from "react";
import * as THREE from "three";
import { TERRAIN_HALF_SIZE, TERRAIN_GRID_CELL } from "@/lib/constants";

// ---------------------------------------------------------------------------
// Terrain — flat green ground plane with a grid overlay
// ---------------------------------------------------------------------------

export default function Terrain() {
  const gridSize = TERRAIN_HALF_SIZE * 2;
  const divisions = gridSize / TERRAIN_GRID_CELL;

  return (
    <group>
      {/* Solid ground plane */}
      <mesh rotation={[-Math.PI / 2, 0, 0]} receiveShadow>
        <planeGeometry args={[gridSize, gridSize]} />
        <meshStandardMaterial color="#3a6b35" roughness={0.9} />
      </mesh>

      {/* Grid overlay — slightly above to prevent z-fighting */}
      <gridHelper
        args={[gridSize, divisions, "#2d5a27", "#2d5a27"]}
        position={[0, 0.05, 0]}
      />
    </group>
  );
}
