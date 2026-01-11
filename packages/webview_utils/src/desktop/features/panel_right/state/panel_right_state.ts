export const panelRightIds = [
    "equations"
] as const;


export type PanelRightId = typeof panelRightIds[number]


export interface PanelRightState {
    open: boolean
    showing: PanelRightId
}
