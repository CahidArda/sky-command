"use client";

import { useMemo } from "react";
import * as THREE from "three";
import { TERRAIN_HALF_SIZE, TERRAIN_GRID_CELL } from "@/lib/constants";

// ---------------------------------------------------------------------------
// Terrain — ground plane with grid, trees, buildings, and runway
// ---------------------------------------------------------------------------

// Seeded pseudo-random for deterministic placement
function seededRandom(seed: number) {
  let s = seed;
  return () => {
    s = (s * 16807 + 0) % 2147483647;
    return s / 2147483647;
  };
}

export default function Terrain() {
  const gridSize = TERRAIN_HALF_SIZE * 2;
  const divisions = gridSize / TERRAIN_GRID_CELL;

  // Generate tree and building positions deterministically
  const { trees, buildings } = useMemo(() => {
    const rng = seededRandom(42);
    const t: [number, number][] = [];
    const b: [number, number, number][] = [];

    // Trees: scattered across the map
    for (let i = 0; i < 600; i++) {
      const x = (rng() - 0.5) * 8000;
      const z = (rng() - 0.5) * 8000;
      // Keep clear of runway area
      if (Math.abs(x) < 40 && Math.abs(z) < 500) continue;
      t.push([x, z]);
    }

    // Buildings: small clusters near the center
    for (let i = 0; i < 80; i++) {
      const x = (rng() - 0.5) * 3000;
      const z = (rng() - 0.5) * 3000;
      const h = 8 + rng() * 30;
      if (Math.abs(x) < 50 && Math.abs(z) < 500) continue;
      b.push([x, z, h]);
    }

    return { trees: t, buildings: b };
  }, []);

  return (
    <group>
      {/* Solid ground plane */}
      <mesh rotation={[-Math.PI / 2, 0, 0]} receiveShadow>
        <planeGeometry args={[gridSize, gridSize]} />
        <meshStandardMaterial color="#3a6b35" roughness={0.9} />
      </mesh>

      {/* Grid overlay */}
      <gridHelper
        args={[gridSize, divisions, "#2d5a27", "#2d5a27"]}
        position={[0, 0.05, 0]}
      />

      {/* Runway */}
      <Runway />

      {/* Trees (instanced for performance) */}
      <Trees positions={trees} />

      {/* Buildings */}
      <Buildings positions={buildings} />
    </group>
  );
}

// ---------------------------------------------------------------------------
// Runway with markings
// ---------------------------------------------------------------------------

function Runway() {
  return (
    <group>
      {/* Runway surface */}
      <mesh rotation={[-Math.PI / 2, 0, 0]} position={[0, 0.02, 0]} receiveShadow>
        <planeGeometry args={[30, 800]} />
        <meshStandardMaterial color="#444444" roughness={0.8} />
      </mesh>

      {/* Center line dashes */}
      {Array.from({ length: 20 }, (_, i) => (
        <mesh
          key={`cl-${i}`}
          rotation={[-Math.PI / 2, 0, 0]}
          position={[0, 0.03, -380 + i * 40]}
          receiveShadow
        >
          <planeGeometry args={[0.5, 15]} />
          <meshStandardMaterial color="#dddddd" />
        </mesh>
      ))}

      {/* Threshold markings */}
      {[-390, 390].map((z) =>
        [-8, -4, 0, 4, 8].map((x) => (
          <mesh
            key={`th-${z}-${x}`}
            rotation={[-Math.PI / 2, 0, 0]}
            position={[x, 0.03, z]}
            receiveShadow
          >
            <planeGeometry args={[1.5, 20]} />
            <meshStandardMaterial color="#dddddd" />
          </mesh>
        )),
      )}
    </group>
  );
}

// ---------------------------------------------------------------------------
// Instanced trees (cone + cylinder)
// ---------------------------------------------------------------------------

function Trees({ positions }: { positions: [number, number][] }) {
  const trunkMat = useMemo(
    () => new THREE.MeshStandardMaterial({ color: "#5a3a1a" }),
    [],
  );
  const leafMat = useMemo(
    () => new THREE.MeshStandardMaterial({ color: "#2d6b1e" }),
    [],
  );

  return (
    <group>
      {positions.map(([x, z], i) => (
        <group key={i} position={[x, 0, z]}>
          {/* Trunk */}
          <mesh position={[0, 2, 0]} castShadow>
            <cylinderGeometry args={[0.3, 0.4, 4, 6]} />
            <primitive object={trunkMat} attach="material" />
          </mesh>
          {/* Canopy */}
          <mesh position={[0, 5.5, 0]} castShadow>
            <coneGeometry args={[2.5, 5, 6]} />
            <primitive object={leafMat} attach="material" />
          </mesh>
        </group>
      ))}
    </group>
  );
}

// ---------------------------------------------------------------------------
// Buildings (simple boxes)
// ---------------------------------------------------------------------------

function Buildings({
  positions,
}: {
  positions: [number, number, number][];
}) {
  const colors = ["#888888", "#777777", "#999999", "#aaaaaa", "#666666"];

  return (
    <group>
      {positions.map(([x, z, h], i) => {
        const w = 6 + (i % 5) * 3;
        const d = 6 + ((i * 7) % 5) * 3;
        return (
          <mesh key={i} position={[x, h / 2, z]} castShadow receiveShadow>
            <boxGeometry args={[w, h, d]} />
            <meshStandardMaterial color={colors[i % colors.length]} />
          </mesh>
        );
      })}
    </group>
  );
}
