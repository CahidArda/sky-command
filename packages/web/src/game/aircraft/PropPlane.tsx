"use client";

import { useRef } from "react";
import { useFrame } from "@react-three/fiber";
import * as THREE from "three";
import { useGameStore } from "@/stores/gameStore";

// ---------------------------------------------------------------------------
// PropPlane — simple geometric mesh representing a Cessna 172
//
// Composed of boxes and a cone: fuselage, wings, tail, propeller.
// Position & rotation are driven entirely by the game store.
// ---------------------------------------------------------------------------

export default function PropPlane() {
  const groupRef = useRef<THREE.Group>(null!);
  const propRef = useRef<THREE.Mesh>(null!);

  useFrame(() => {
    const { aircraft } = useGameStore.getState();

    // Sync group transform with physics state
    groupRef.current.position.copy(aircraft.position);
    groupRef.current.rotation.copy(aircraft.rotation);

    // Spin propeller based on throttle (visual only)
    if (propRef.current) {
      propRef.current.rotation.z += aircraft.throttle * 0.8;
    }
  });

  return (
    <group ref={groupRef}>
      {/* Fuselage */}
      <mesh castShadow>
        <boxGeometry args={[1.2, 1.0, 6.0]} />
        <meshStandardMaterial color="#e8e8e8" />
      </mesh>

      {/* Cockpit / windshield */}
      <mesh position={[0, 0.5, -1.5]} castShadow>
        <boxGeometry args={[1.0, 0.6, 1.5]} />
        <meshStandardMaterial color="#4488cc" transparent opacity={0.6} />
      </mesh>

      {/* Wings */}
      <mesh position={[0, -0.1, 0]} castShadow>
        <boxGeometry args={[11.0, 0.15, 1.6]} />
        <meshStandardMaterial color="#d0d0d0" />
      </mesh>

      {/* Horizontal stabilizer (tail wing) */}
      <mesh position={[0, 0.1, 3.2]} castShadow>
        <boxGeometry args={[3.6, 0.1, 0.8]} />
        <meshStandardMaterial color="#d0d0d0" />
      </mesh>

      {/* Vertical stabilizer (tail fin) */}
      <mesh position={[0, 0.8, 3.2]} castShadow>
        <boxGeometry args={[0.1, 1.4, 0.9]} />
        <meshStandardMaterial color="#d0d0d0" />
      </mesh>

      {/* Blue accent stripe */}
      <mesh position={[0, 0.51, 0]}>
        <boxGeometry args={[1.22, 0.02, 6.02]} />
        <meshStandardMaterial color="#2255aa" />
      </mesh>

      {/* Propeller hub */}
      <mesh position={[0, 0, -3.2]}>
        <cylinderGeometry args={[0.15, 0.15, 0.3, 8]} />
        <meshStandardMaterial color="#444444" />
      </mesh>

      {/* Propeller blades — spin around Z (forward axis) */}
      <group position={[0, 0, -3.4]} ref={propRef}>
        <mesh>
          <boxGeometry args={[2.4, 0.2, 0.05]} />
          <meshStandardMaterial color="#333333" />
        </mesh>
        <mesh rotation={[0, 0, Math.PI / 2]}>
          <boxGeometry args={[2.4, 0.2, 0.05]} />
          <meshStandardMaterial color="#333333" />
        </mesh>
      </group>

      {/* Landing gear struts (visual only) */}
      <mesh position={[-1.2, -0.9, -0.8]}>
        <cylinderGeometry args={[0.05, 0.05, 0.8, 6]} />
        <meshStandardMaterial color="#555555" />
      </mesh>
      <mesh position={[1.2, -0.9, -0.8]}>
        <cylinderGeometry args={[0.05, 0.05, 0.8, 6]} />
        <meshStandardMaterial color="#555555" />
      </mesh>
      {/* Wheels */}
      <mesh position={[-1.2, -1.35, -0.8]} rotation={[0, 0, Math.PI / 2]}>
        <cylinderGeometry args={[0.2, 0.2, 0.15, 12]} />
        <meshStandardMaterial color="#222222" />
      </mesh>
      <mesh position={[1.2, -1.35, -0.8]} rotation={[0, 0, Math.PI / 2]}>
        <cylinderGeometry args={[0.2, 0.2, 0.15, 12]} />
        <meshStandardMaterial color="#222222" />
      </mesh>
    </group>
  );
}
