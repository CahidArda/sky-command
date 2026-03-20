"use client";

import dynamic from "next/dynamic";

// Dynamic import with SSR disabled — Three.js / WebGL cannot run on the server.
const Game = dynamic(() => import("@/game/Game"), { ssr: false });

// ---------------------------------------------------------------------------
// /game — full-screen flight simulator page
// ---------------------------------------------------------------------------

export default function GamePage() {
  return <Game />;
}
