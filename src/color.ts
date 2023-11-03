import { clamp } from "./numbers";

export class Hsl {
    constructor(
        public h: number,
        public s: number,
        public l: number
    ) { }

    toString(): string {
        return `hsl(${this.h} ${this.s}% ${this.l}%)`;
    }

    sh(value: number): Hsl {
        return new Hsl(value, this.s, this.l);
    }

    ss(value: number): Hsl {
        return new Hsl(this.h, value, this.l);
    }

    sl(value: number): Hsl {
        return new Hsl(this.h, this.s, value);
    }

    dh(amount: number): Hsl {
        return new Hsl(this.h + amount, this.s, this.l);
    }

    ds(amount: number): Hsl {
        return new Hsl(this.h, clamp(this.s + amount, 0, 100), this.l);
    }

    dl(amount: number): Hsl {
        return new Hsl(this.h, this.s, clamp(this.l + amount, 0, 100));
    }
}