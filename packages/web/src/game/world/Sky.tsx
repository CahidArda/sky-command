"use client";

import { Sky as DreiSky } from "@react-three/drei";

// ---------------------------------------------------------------------------
// Sky — procedural sun-based sky from drei
// ---------------------------------------------------------------------------

export default function SkyDome() {
  return (
    <DreiSky
      distance={450000}
      sunPosition={[5000, 2000, -3000]}
      inclination={0.6}
      azimuth={0.25}
      turbidity={8}
      rayleigh={2}
      mieCoefficient={0.005}
      mieDirectionalG={0.8}
    />
  );
}
