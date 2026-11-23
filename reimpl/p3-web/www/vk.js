// KeyboardEvent.code → código virtual de Windows (misma tabla que p3-client/src/keys.rs).
const VK = {
  Digit0: 0x30, Digit1: 0x31, Digit2: 0x32, Digit3: 0x33, Digit4: 0x34, Digit5: 0x35, Digit6: 0x36, Digit7: 0x37, Digit8: 0x38, Digit9: 0x39,
  Numpad0: 0x60, Numpad1: 0x61, Numpad2: 0x62, Numpad3: 0x63, Numpad4: 0x64, Numpad5: 0x65, Numpad6: 0x66, Numpad7: 0x67, Numpad8: 0x68, Numpad9: 0x69,
  NumpadMultiply: 0x6A, NumpadAdd: 0x6B, NumpadSubtract: 0x6D, NumpadDecimal: 0x6E, NumpadDivide: 0x6F,
  NumpadEnter: 0x0D, Enter: 0x0D, Escape: 0x1B, Backspace: 0x08, Tab: 0x09, Space: 0x20,
  ShiftLeft: 0xA0, ShiftRight: 0xA1, ControlLeft: 0xA2, ControlRight: 0xA3, AltLeft: 0xA4, AltRight: 0xA5,
  Pause: 0x13, CapsLock: 0x14, NumLock: 0x90, ScrollLock: 0x91, PageUp: 0x21, PageDown: 0x22, End: 0x23, Home: 0x24,
  ArrowLeft: 0x25, ArrowUp: 0x26, ArrowRight: 0x27, ArrowDown: 0x28, PrintScreen: 0x2C, Insert: 0x2D, Delete: 0x2E,
  Equal: 0xBB, Minus: 0xBD, Comma: 0xBC, Period: 0xBE,
};
for (let c = 65; c <= 90; c++) VK['Key' + String.fromCharCode(c)] = c;
for (let i = 1; i <= 12; i++) VK['F' + i] = 0x6F + i;
export function vkOf(code) { return VK[code] ?? 0; }
