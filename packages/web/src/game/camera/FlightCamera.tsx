"use client";

import { useRef } from "react";
import { useFrame, useThree } from "@react-three/fiber";
import * as THREE from "three";
import { useGameStore } from "@/stores/gameStore";

// ---------------------------------------------------------------------------
// FlightCamera — smooth chase camera that follows behind the aircraft
//
// Offset: ~15 m behind, ~5 m above (in local space).
// Uses lerp for smooth interpolation so the camera doesn't snap.
// ---------------------------------------------------------------------------

const CHASE_OFFSET = new THREE.Vector3(0, 5, 15); // local offset behind & above
const LOOK_AHEAD = new THREE.Vector3(0, 1, -20); // look-at point ahead of aircraft
const LERP_FACTOR = 3; // higher = snappier camera

export default function FlightCamera() {
  const { camera } = useThree();
  const desiredPos = useRef(new THREE.Vector3());
  const desiredLookAt = useRef(new THREE.Vector3());

  useFrame((_, delta) => {
    const { aircraft } = useGameStore.getState();
    const quat = new THREE.Quaternion().setFromEuler(aircraft.rotation);

    // Desired camera position: aircraft position + rotated offset
    const offset = CHASE_OFFSET.clone().applyQuaternion(quat);
    desiredPos.current.copy(aircraft.position).add(offset);

    // Desired look-at target: a point ahead of the aircraft
    const lookTarget = LOOK_AHEAD.clone().applyQuaternion(quat);
    desiredLookAt.current.copy(aircraft.position).add(lookTarget);

    // Smooth interpolation
    const t = 1 - Math.exp(-LERP_FACTOR * delta);
    camera.position.lerp(desiredPos.current, t);

    // Smooth look-at via slerp of the camera quaternion
    const targetQuat = new THREE.Quaternion();
    const lookMatrix = new THREE.Matrix4().lookAt(
      camera.position,
      desiredLookAt.current,
      new THREE.Vector3(0, 1, 0),
    );
    targetQuat.setFromRotationMatrix(lookMatrix);
    camera.quaternion.slerp(targetQuat, t);
  });

  return null;
}
