import { Color, ColorGroup } from "../classes/color";

export class FlusterBlue extends ColorGroup {
    constructor() {
        super(
            new Color("primary", 198.6, 88.7, 48.4),
            new Color("primary", 217.2, 912, 59.8),
        )
    }
}


export class DesktopBackgroud extends ColorGroup {
    constructor() {
        super(
            new Color("background", 0, 0, 100),
            new Color("background", 213, 39, 9),
        )
    }
}
