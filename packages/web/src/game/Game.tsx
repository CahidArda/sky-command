"use client";

import { Canvas } from "@react-three/fiber";
import { KeyboardControls } from "@react-three/drei";
import { keyMap } from "./input/KeyboardControls";
import Terrain from "./world/Terrain";
import SkyDome from "./world/Sky";
import PropPlane from "./aircraft/PropPlane";
import GameLoop from "./GameLoop";
import FlightCamera from "./camera/FlightCamera";
import HUD from "./hud/HUD";

// ---------------------------------------------------------------------------
// Game — top-level component that assembles the 3D scene + HTML overlay
//
// Structure:
//   KeyboardControls (drei provider — must wrap Canvas)
//     Canvas          (R3F — everything 3D lives here)
//       Sky, Terrain, PropPlane, GameLoop, FlightCamera, Lights
//     HUD             (HTML overlay — sibling to Canvas, NOT inside it)
// ---------------------------------------------------------------------------

export default function Game() {
  return (
    <KeyboardControls map={keyMap}>
      <div className="relative h-screen w-screen overflow-hidden bg-black">
        <Canvas
          shadows
          camera={{
            fov: 65,
            near: 0.5,
            far: 100_000,
            position: [0, 1005, 15],
          }}
          gl={{ antialias: true }}
        >
          {/* Lighting */}
          <ambientLight intensity={0.4} />
          <directionalLight
            position={[500, 800, -300]}
            intensity={1.2}
            castShadow
            shadow-mapSize-width={2048}
            shadow-mapSize-height={2048}
            shadow-camera-far={5000}
            shadow-camera-left={-500}
            shadow-camera-right={500}
            shadow-camera-top={500}
            shadow-camera-bottom={-500}
          />

          {/* World */}
          <SkyDome />
          <Terrain />

          {/* Aircraft */}
          <PropPlane />

          {/* Systems */}
          <GameLoop />
          <FlightCamera />

          {/* Fog for depth cue */}
          <fog attach="fog" args={["#87CEEB", 2000, 10000]} />
        </Canvas>

        {/* HTML HUD overlay (outside Canvas) */}
        <HUD />
      </div>
    </KeyboardControls>
  );
}
