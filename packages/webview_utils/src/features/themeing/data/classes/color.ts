
export class Color {
    hue: number;
    saturation: number;
    lightness: number;
    label: string;
    constructor(
        /** label is the css variable without the leading 2 '--' characters. */
        label: string,
        hue: number,
        saturation: number,
        lightness: number
    ) {
        this.label = label;
        this.hue = hue;
        this.saturation = saturation;
        this.lightness = lightness
    }

    toHslCssColor(): string {
        return `hsl(${this.hue}, ${this.saturation}%, ${this.lightness}%)`
    }

    toCssVariableString(): string {
        return `--${this.label}: ${this.hue} ${this.saturation}% ${this.lightness}%;`
    }
}



export abstract class ColorGroup {
    light: Color
    dark: Color
    constructor(light: Color, dark: Color) {
        this.light = light;
        this.dark = dark;
    }
}
