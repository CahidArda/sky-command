// The version string is derived from package.json at build time via Next.js
// bundler (resolveJsonModule). Importing the entire file and extracting the
// field keeps things simple and tree-shakeable.

import pkg from "../../package.json";

export const VERSION: string = pkg.version;
