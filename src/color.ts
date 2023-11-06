import { clamp } from "./numbers";

export class Oklch {
    constructor(
        // 0-100
        public lightness: number,
        // 0-100
        public chroma: number,
        // 0-360, but wraps
        public hue: number,
    ) { }

    toString(): string {
        let c = 0.004 * this.chroma;
        return `oklch(${this.lightness}% ${c} ${this.hue})`;
    }

    sl(value: number): Oklch {
        return new Oklch(value, this.chroma, this.hue);
    }

    sc(value: number): Oklch {
        return new Oklch(this.lightness, value, this.hue);
    }

    sh(value: number): Oklch {
        return new Oklch(this.lightness, this.chroma, value);
    }

    dl(amount: number): Oklch {
        return new Oklch(clamp(this.lightness + amount, 0, 100), this.chroma, this.hue);
    }

    dc(amount: number): Oklch {
        return new Oklch(this.lightness, clamp(this.chroma + amount, 0, 100), this.hue);
    }

    dh(amount: number): Oklch {
        return new Oklch(this.lightness, this.chroma, this.hue + amount);
    }

    apply(diff: OkDiff): Oklch {
        return new Oklch(
            clamp(this.lightness + diff.deltaLightness, 0, 100),
            clamp(this.chroma + diff.deltaChroma, 0, 100),
            this.hue + diff.deltaHue
        )
    }
}

export class OkDiff {
    deltaLightness: number;
    deltaChroma: number;
    deltaHue: number;

    constructor(
        deltaLightness: number = 0,
        deltaChroma: number = 0,
        deltaHue: number = 0
    ) {
        this.deltaLightness = deltaLightness;
        this.deltaChroma = deltaChroma;
        this.deltaHue = deltaHue;
    }

    dl(amount: number): OkDiff {
        return new OkDiff(this.deltaLightness + amount, this.deltaChroma, this.deltaHue);
    }

    dc(amount: number): OkDiff {
        return new OkDiff(this.deltaLightness, this.deltaChroma + amount, this.deltaHue);
    }

    dh(amount: number): OkDiff {
        return new OkDiff(this.deltaLightness, this.deltaChroma, this.deltaHue + amount);
    }
}