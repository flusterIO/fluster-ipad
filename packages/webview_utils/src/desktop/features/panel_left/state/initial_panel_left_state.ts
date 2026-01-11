export const panelLeftIds = [
    "equations"
] as const;


export type PanelLeftId = typeof panelLeftIds[number]


export interface PanelLeftState {
    open: boolean
    showing: PanelLeftId
}
