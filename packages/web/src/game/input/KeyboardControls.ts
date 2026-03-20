// ---------------------------------------------------------------------------
// Keyboard mapping for @react-three/drei <KeyboardControls>
//
// Each entry maps a human-readable name to one or more key codes.
// The names are used by useKeyboardControls() to read pressed state.
// ---------------------------------------------------------------------------

export enum Controls {
  forward = "forward",     // pitch nose down  (W)
  back = "back",           // pitch nose up    (S)
  left = "left",           // roll left        (A)
  right = "right",         // roll right       (D)
  yawLeft = "yawLeft",     // yaw left         (Q)
  yawRight = "yawRight",   // yaw right        (E)
  throttleUp = "throttleUp",   // increase throttle (Shift)
  throttleDown = "throttleDown", // decrease throttle (Ctrl)
}

export type KeyMap = { name: string; keys: string[] }[];

export const keyMap: KeyMap = [
  { name: Controls.forward, keys: ["KeyW"] },
  { name: Controls.back, keys: ["KeyS"] },
  { name: Controls.left, keys: ["KeyA"] },
  { name: Controls.right, keys: ["KeyD"] },
  { name: Controls.yawLeft, keys: ["KeyQ"] },
  { name: Controls.yawRight, keys: ["KeyE"] },
  { name: Controls.throttleUp, keys: ["ShiftLeft", "ShiftRight"] },
  { name: Controls.throttleDown, keys: ["ControlLeft", "ControlRight"] },
];
