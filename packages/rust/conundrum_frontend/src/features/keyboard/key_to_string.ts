
const SPECIAL_KEY_MAP: Record<string, { mac: string; other: string }> = {
    Control: { mac: '⌃', other: 'Ctrl' },
    Meta: { mac: '⌘', other: 'Win' },
    Alt: { mac: '⌥', other: 'Alt' },
    Shift: { mac: '⇧', other: 'Shift' },
    Enter: { mac: '⏎', other: 'Enter' },
    Backspace: { mac: '⌫', other: 'Backspace' },
    ArrowUp: { mac: '↑', other: '↑' },
    ArrowDown: { mac: '↓', other: '↓' },
    ArrowLeft: { mac: '←', other: '←' },
    ArrowRight: { mac: '→', other: '→' },
    Escape: { mac: '⎋', other: 'Esc' },
    Tab: { mac: '⇥', other: 'Tab' },
    ' ': { mac: '␣', other: 'Space' },
};

export const keyToString = (k: string): { mac: string, other: string } => {
    if (k in SPECIAL_KEY_MAP) {
        return SPECIAL_KEY_MAP[k]
    } else {
        return {
            mac: k,
            other: k
        }
    }
}
