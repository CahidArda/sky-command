"use client";

import { VERSION } from "@/lib/version";

// ---------------------------------------------------------------------------
// VersionBadge — "SkyCommand v0.1.0 (web)" in bottom-right corner
// ---------------------------------------------------------------------------

export default function VersionBadge() {
  return (
    <div className="absolute bottom-6 right-6">
      <span className="font-mono text-xs text-green-400/50">
        SkyCommand v{VERSION} (web)
      </span>
    </div>
  );
}
