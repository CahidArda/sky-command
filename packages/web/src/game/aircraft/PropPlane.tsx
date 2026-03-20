"use client";

import { useRef } from "react";
import { useFrame } from "@react-three/fiber";
import * as THREE from "three";
import { useGameStore } from "@/stores/gameStore";

// ---------------------------------------------------------------------------
// PropPlane — realistic geometric Cessna 172 Skyhawk
//
// HIGH-WING design with struts, tapered fuselage, tricycle landing gear,
// navigation lights, and spinning propeller.
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

  // --- Shared materials (avoid re-creation per mesh) ---
  const whiteFuselage = { color: "#f0f0f0", roughness: 0.4, metalness: 0.05 };
  const lightGrey = { color: "#d4d4d4", roughness: 0.45, metalness: 0.05 };
  const darkGrey = { color: "#555555", metalness: 0.3, roughness: 0.6 };
  const stripeBlue = { color: "#1a3f7a" };
  const black = { color: "#1a1a1a", roughness: 0.7, metalness: 0.1 };
  const chrome = { color: "#888888", metalness: 0.8, roughness: 0.2 };
  const glass = {
    color: "#7ab8c4",
    transparent: true,
    opacity: 0.35,
    roughness: 0.1,
    metalness: 0.1,
  };

  return (
    <group ref={groupRef}>
      {/* ================================================================
          FUSELAGE — tapered multi-section body
          Total length ~8m. Wider at cabin, narrow toward tail cone.
          Z axis = forward (negative = nose, positive = tail)
          ================================================================ */}

      {/* Nose cowling — slightly wider, houses engine */}
      <mesh position={[0, 0, -3.5]} castShadow>
        <cylinderGeometry args={[0.52, 0.58, 0.7, 12]} />
        <meshStandardMaterial {...lightGrey} />
      </mesh>
      {/* Cowling-to-firewall transition */}
      <mesh position={[0, 0, -3.05]} castShadow>
        <boxGeometry args={[1.15, 1.05, 0.2]} />
        <meshStandardMaterial {...whiteFuselage} />
      </mesh>

      {/* Forward fuselage — cabin section (widest) */}
      <mesh position={[0, 0, -2.0]} castShadow>
        <boxGeometry args={[1.18, 1.1, 1.9]} />
        <meshStandardMaterial {...whiteFuselage} />
      </mesh>

      {/* Mid fuselage — behind cabin, begins taper */}
      <mesh position={[0, 0, -0.6]} castShadow>
        <boxGeometry args={[1.1, 1.0, 0.9]} />
        <meshStandardMaterial {...whiteFuselage} />
      </mesh>

      {/* Aft fuselage — narrowing toward tail */}
      <mesh position={[0, 0.02, 0.3]} castShadow>
        <boxGeometry args={[0.95, 0.9, 0.9]} />
        <meshStandardMaterial {...whiteFuselage} />
      </mesh>

      {/* Rear fuselage taper 1 */}
      <mesh position={[0, 0.05, 1.2]} castShadow>
        <boxGeometry args={[0.75, 0.72, 0.9]} />
        <meshStandardMaterial {...whiteFuselage} />
      </mesh>

      {/* Rear fuselage taper 2 */}
      <mesh position={[0, 0.08, 2.0]} castShadow>
        <boxGeometry args={[0.55, 0.55, 0.7]} />
        <meshStandardMaterial {...whiteFuselage} />
      </mesh>

      {/* Tail cone — narrowest section */}
      <mesh position={[0, 0.1, 2.7]} castShadow>
        <boxGeometry args={[0.38, 0.4, 0.7]} />
        <meshStandardMaterial {...whiteFuselage} />
      </mesh>

      {/* Tail tip */}
      <mesh position={[0, 0.12, 3.25]} castShadow>
        <boxGeometry args={[0.22, 0.28, 0.4]} />
        <meshStandardMaterial {...whiteFuselage} />
      </mesh>

      {/* ================================================================
          FUSELAGE STRIPE — dark blue accent along each side
          ================================================================ */}
      {/* Right side stripe */}
      <mesh position={[0.6, 0.1, -1.0]}>
        <boxGeometry args={[0.02, 0.18, 5.5]} />
        <meshStandardMaterial {...stripeBlue} />
      </mesh>
      {/* Left side stripe */}
      <mesh position={[-0.6, 0.1, -1.0]}>
        <boxGeometry args={[0.02, 0.18, 5.5]} />
        <meshStandardMaterial {...stripeBlue} />
      </mesh>
      {/* Top stripe (thinner, decorative) */}
      <mesh position={[0, 0.56, -1.5]}>
        <boxGeometry args={[0.12, 0.02, 3.5]} />
        <meshStandardMaterial {...stripeBlue} />
      </mesh>

      {/* ================================================================
          CABIN / WINDSHIELD — raised transparent greenhouse
          Cessna 172 has a very tall, wrap-around windshield
          ================================================================ */}

      {/* Cabin roof structure */}
      <mesh position={[0, 0.75, -2.0]} castShadow>
        <boxGeometry args={[1.05, 0.4, 1.6]} />
        <meshStandardMaterial {...whiteFuselage} />
      </mesh>

      {/* Front windshield — angled forward */}
      <mesh position={[0, 0.65, -2.95]} rotation={[0.35, 0, 0]} castShadow>
        <boxGeometry args={[0.95, 0.5, 0.05]} />
        <meshStandardMaterial {...glass} side={THREE.DoubleSide} />
      </mesh>

      {/* Left window */}
      <mesh position={[-0.535, 0.65, -2.0]}>
        <boxGeometry args={[0.04, 0.38, 1.4]} />
        <meshStandardMaterial {...glass} side={THREE.DoubleSide} />
      </mesh>

      {/* Right window */}
      <mesh position={[0.535, 0.65, -2.0]}>
        <boxGeometry args={[0.04, 0.38, 1.4]} />
        <meshStandardMaterial {...glass} side={THREE.DoubleSide} />
      </mesh>

      {/* Rear window */}
      <mesh position={[0, 0.7, -1.1]} rotation={[-0.3, 0, 0]}>
        <boxGeometry args={[0.85, 0.35, 0.04]} />
        <meshStandardMaterial {...glass} side={THREE.DoubleSide} />
      </mesh>

      {/* ================================================================
          HIGH WING — mounted on TOP of the fuselage/cabin
          Wingspan ~11m, chord ~1.5m, slight dihedral
          This is THE defining Cessna 172 feature
          ================================================================ */}

      {/* Left wing */}
      <mesh position={[-2.95, 0.95, -1.6]} rotation={[0, 0, 0.02]} castShadow>
        <boxGeometry args={[5.1, 0.14, 1.5]} />
        <meshStandardMaterial {...lightGrey} />
      </mesh>

      {/* Right wing */}
      <mesh position={[2.95, 0.95, -1.6]} rotation={[0, 0, -0.02]} castShadow>
        <boxGeometry args={[5.1, 0.14, 1.5]} />
        <meshStandardMaterial {...lightGrey} />
      </mesh>

      {/* Wing root fairing — blends wing into fuselage top */}
      <mesh position={[0, 0.92, -1.6]} castShadow>
        <boxGeometry args={[1.2, 0.18, 1.55]} />
        <meshStandardMaterial {...lightGrey} />
      </mesh>

      {/* ================================================================
          WING STRUTS — diagonal braces from lower fuselage to wing
          Characteristic of Cessna high-wing aircraft
          ================================================================ */}

      {/* Left wing strut */}
      <mesh
        position={[-1.8, 0.2, -1.5]}
        rotation={[0, 0, 0.62]}
        castShadow
      >
        <boxGeometry args={[0.06, 1.8, 0.08]} />
        <meshStandardMaterial {...chrome} />
      </mesh>

      {/* Right wing strut */}
      <mesh
        position={[1.8, 0.2, -1.5]}
        rotation={[0, 0, -0.62]}
        castShadow
      >
        <boxGeometry args={[0.06, 1.8, 0.08]} />
        <meshStandardMaterial {...chrome} />
      </mesh>

      {/* Left rear jury strut (smaller, between main strut and wing) */}
      <mesh
        position={[-2.3, 0.5, -1.2]}
        rotation={[0, 0, 0.55]}
        castShadow
      >
        <boxGeometry args={[0.03, 0.7, 0.04]} />
        <meshStandardMaterial {...chrome} />
      </mesh>

      {/* Right rear jury strut */}
      <mesh
        position={[2.3, 0.5, -1.2]}
        rotation={[0, 0, -0.55]}
        castShadow
      >
        <boxGeometry args={[0.03, 0.7, 0.04]} />
        <meshStandardMaterial {...chrome} />
      </mesh>

      {/* ================================================================
          HORIZONTAL STABILIZER — low-mounted at the tail
          ================================================================ */}
      <mesh position={[0, 0.15, 3.3]} castShadow>
        <boxGeometry args={[3.4, 0.08, 0.9]} />
        <meshStandardMaterial {...lightGrey} />
      </mesh>

      {/* ================================================================
          VERTICAL STABILIZER with dorsal fin
          ================================================================ */}

      {/* Main vertical fin */}
      <mesh position={[0, 0.85, 3.1]} castShadow>
        <boxGeometry args={[0.08, 1.3, 1.1]} />
        <meshStandardMaterial {...lightGrey} />
      </mesh>

      {/* Dorsal fin — triangular fillet at base of vertical stab */}
      <mesh position={[0, 0.35, 2.4]} rotation={[0.4, 0, 0]} castShadow>
        <boxGeometry args={[0.06, 0.5, 0.6]} />
        <meshStandardMaterial {...lightGrey} />
      </mesh>

      {/* Rudder accent stripe */}
      <mesh position={[0.045, 0.9, 3.4]}>
        <boxGeometry args={[0.01, 0.8, 0.4]} />
        <meshStandardMaterial {...stripeBlue} />
      </mesh>

      {/* ================================================================
          ENGINE / PROPELLER
          ================================================================ */}

      {/* Engine cowling front face */}
      <mesh position={[0, 0, -3.85]} castShadow>
        <cylinderGeometry args={[0.35, 0.48, 0.05, 12]} />
        <meshStandardMaterial {...darkGrey} />
      </mesh>

      {/* Propeller spinner cone */}
      <mesh position={[0, 0, -3.95]} rotation={[Math.PI / 2, 0, 0]}>
        <coneGeometry args={[0.15, 0.3, 8]} />
        <meshStandardMaterial {...chrome} />
      </mesh>

      {/* Propeller hub */}
      <mesh position={[0, 0, -3.9]}>
        <cylinderGeometry args={[0.08, 0.08, 0.12, 8]} />
        <meshStandardMaterial {...darkGrey} />
      </mesh>

      {/* Propeller blades — spin around Z (forward axis) */}
      <group position={[0, 0, -4.0]} ref={propRef}>
        {/* Blade 1 */}
        <mesh>
          <boxGeometry args={[2.0, 0.18, 0.04]} />
          <meshStandardMaterial color="#2a2a2a" roughness={0.5} metalness={0.2} />
        </mesh>
        {/* Blade 2 */}
        <mesh rotation={[0, 0, Math.PI / 2]}>
          <boxGeometry args={[2.0, 0.18, 0.04]} />
          <meshStandardMaterial color="#2a2a2a" roughness={0.5} metalness={0.2} />
        </mesh>
      </group>

      {/* ================================================================
          TRICYCLE LANDING GEAR
          ================================================================ */}

      {/* --- Nose gear --- */}
      {/* Nose strut */}
      <mesh position={[0, -0.7, -3.0]}>
        <cylinderGeometry args={[0.04, 0.04, 0.6, 6]} />
        <meshStandardMaterial {...chrome} />
      </mesh>
      {/* Nose wheel */}
      <mesh position={[0, -1.05, -3.0]} rotation={[0, 0, Math.PI / 2]}>
        <cylinderGeometry args={[0.15, 0.15, 0.1, 12]} />
        <meshStandardMaterial {...black} />
      </mesh>
      {/* Nose wheel hub */}
      <mesh position={[0, -1.05, -3.0]} rotation={[0, 0, Math.PI / 2]}>
        <cylinderGeometry args={[0.06, 0.06, 0.12, 8]} />
        <meshStandardMaterial {...chrome} />
      </mesh>

      {/* --- Left main gear (under wing) --- */}
      {/* Left gear strut */}
      <mesh position={[-1.0, -0.75, -1.5]} rotation={[0, 0, 0.15]}>
        <cylinderGeometry args={[0.05, 0.05, 0.7, 6]} />
        <meshStandardMaterial {...chrome} />
      </mesh>
      {/* Left gear brace */}
      <mesh position={[-0.8, -0.65, -1.5]} rotation={[0, 0, -0.3]}>
        <cylinderGeometry args={[0.03, 0.03, 0.5, 6]} />
        <meshStandardMaterial {...chrome} />
      </mesh>
      {/* Left wheel */}
      <mesh position={[-1.1, -1.15, -1.5]} rotation={[0, 0, Math.PI / 2]}>
        <cylinderGeometry args={[0.2, 0.2, 0.12, 12]} />
        <meshStandardMaterial {...black} />
      </mesh>
      {/* Left wheel hub */}
      <mesh position={[-1.1, -1.15, -1.5]} rotation={[0, 0, Math.PI / 2]}>
        <cylinderGeometry args={[0.08, 0.08, 0.14, 8]} />
        <meshStandardMaterial {...chrome} />
      </mesh>
      {/* Left wheel pant (fairing) */}
      <mesh position={[-1.1, -1.15, -1.5]}>
        <boxGeometry args={[0.18, 0.3, 0.5]} />
        <meshStandardMaterial {...whiteFuselage} />
      </mesh>

      {/* --- Right main gear (under wing) --- */}
      {/* Right gear strut */}
      <mesh position={[1.0, -0.75, -1.5]} rotation={[0, 0, -0.15]}>
        <cylinderGeometry args={[0.05, 0.05, 0.7, 6]} />
        <meshStandardMaterial {...chrome} />
      </mesh>
      {/* Right gear brace */}
      <mesh position={[0.8, -0.65, -1.5]} rotation={[0, 0, 0.3]}>
        <cylinderGeometry args={[0.03, 0.03, 0.5, 6]} />
        <meshStandardMaterial {...chrome} />
      </mesh>
      {/* Right wheel */}
      <mesh position={[1.1, -1.15, -1.5]} rotation={[0, 0, Math.PI / 2]}>
        <cylinderGeometry args={[0.2, 0.2, 0.12, 12]} />
        <meshStandardMaterial {...black} />
      </mesh>
      {/* Right wheel hub */}
      <mesh position={[1.1, -1.15, -1.5]} rotation={[0, 0, Math.PI / 2]}>
        <cylinderGeometry args={[0.08, 0.08, 0.14, 8]} />
        <meshStandardMaterial {...chrome} />
      </mesh>
      {/* Right wheel pant (fairing) */}
      <mesh position={[1.1, -1.15, -1.5]}>
        <boxGeometry args={[0.18, 0.3, 0.5]} />
        <meshStandardMaterial {...whiteFuselage} />
      </mesh>

      {/* ================================================================
          NAVIGATION LIGHTS
          ================================================================ */}

      {/* Left wingtip — RED */}
      <mesh position={[-5.5, 0.95, -1.6]}>
        <sphereGeometry args={[0.06, 8, 8]} />
        <meshStandardMaterial
          color="#ff0000"
          emissive="#ff0000"
          emissiveIntensity={0.8}
        />
      </mesh>

      {/* Right wingtip — GREEN */}
      <mesh position={[5.5, 0.95, -1.6]}>
        <sphereGeometry args={[0.06, 8, 8]} />
        <meshStandardMaterial
          color="#00ff00"
          emissive="#00ff00"
          emissiveIntensity={0.8}
        />
      </mesh>

      {/* Tail beacon — WHITE */}
      <mesh position={[0, 0.14, 3.5]}>
        <sphereGeometry args={[0.04, 6, 6]} />
        <meshStandardMaterial
          color="#ffffff"
          emissive="#ffffff"
          emissiveIntensity={0.6}
        />
      </mesh>

      {/* Anti-collision beacon — RED, top of vertical stab */}
      <mesh position={[0, 1.52, 3.1]}>
        <sphereGeometry args={[0.05, 6, 6]} />
        <meshStandardMaterial
          color="#ff2200"
          emissive="#ff2200"
          emissiveIntensity={0.7}
        />
      </mesh>

      {/* ================================================================
          EXHAUST PIPE — small detail on lower cowling
          ================================================================ */}
      <mesh position={[0.25, -0.4, -3.6]} rotation={[Math.PI / 2, 0, 0]}>
        <cylinderGeometry args={[0.04, 0.04, 0.3, 6]} />
        <meshStandardMaterial {...darkGrey} />
      </mesh>

      {/* ================================================================
          PITOT TUBE — small rod under left wing
          ================================================================ */}
      <mesh position={[-3.5, 0.8, -2.1]} rotation={[Math.PI / 2, 0, 0]}>
        <cylinderGeometry args={[0.015, 0.015, 0.3, 4]} />
        <meshStandardMaterial {...chrome} />
      </mesh>

      {/* ================================================================
          ANTENNA — on top of cabin
          ================================================================ */}
      <mesh position={[0, 1.1, -1.6]}>
        <cylinderGeometry args={[0.01, 0.01, 0.35, 4]} />
        <meshStandardMaterial {...darkGrey} />
      </mesh>
    </group>
  );
}
