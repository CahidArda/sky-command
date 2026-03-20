"use client";

import { useRef } from "react";
import { useFrame, useThree } from "@react-three/fiber";
import * as THREE from "three";
import { useGameStore } from "@/stores/gameStore";

// ---------------------------------------------------------------------------
// FlightCamera — chase camera locked behind the aircraft
//
// Uses a stiff lerp so the camera stays tightly behind the plane at all times.
// On the first frame it snaps to position so there's no initial drift.
// ---------------------------------------------------------------------------

const CHASE_OFFSET = new THREE.Vector3(0, 5, 15); // local: behind & above
const LOOK_AHEAD = new THREE.Vector3(0, 0, -30); // local: ahead of aircraft
const LERP_FACTOR = 12; // very stiff follow

export default function FlightCamera() {
  const { camera } = useThree();
  const initialized = useRef(false);

  useFrame((_, delta) => {
    const { aircraft } = useGameStore.getState();
    const quat = new THREE.Quaternion().setFromEuler(aircraft.rotation);

    // Desired camera position: aircraft position + rotated offset
    const offset = CHASE_OFFSET.clone().applyQuaternion(quat);
    const desiredPos = aircraft.position.clone().add(offset);

    // Desired look-at target: a point ahead of the aircraft
    const lookTarget = LOOK_AHEAD.clone().applyQuaternion(quat);
    const desiredLookAt = aircraft.position.clone().add(lookTarget);

    if (!initialized.current) {
      // Snap on first frame
      camera.position.copy(desiredPos);
      initialized.current = true;
    } else {
      // Stiff exponential lerp — stays locked behind the aircraft
      const t = 1 - Math.exp(-LERP_FACTOR * delta);
      camera.position.lerp(desiredPos, t);
    }

    // Always look at the target point ahead of the aircraft
    const lookMatrix = new THREE.Matrix4().lookAt(
      camera.position,
      desiredLookAt,
      new THREE.Vector3(0, 1, 0),
    );
    const targetQuat = new THREE.Quaternion().setFromRotationMatrix(lookMatrix);

    if (!initialized.current) {
      camera.quaternion.copy(targetQuat);
    } else {
      camera.quaternion.slerp(targetQuat, 1 - Math.exp(-LERP_FACTOR * delta));
    }
  });

  return null;
}
