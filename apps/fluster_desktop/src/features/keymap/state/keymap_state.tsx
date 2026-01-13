export const keymapIds = ["toggleCommandPalette"] as const;

export type KeymapId = (typeof keymapIds)[number];

export interface KeymapItem {
    shift: boolean;
    meta: boolean;
    ctrl: boolean;
    keyCode: number;
}

export interface KeymapState extends Record<KeymapId, KeymapItem> { }
