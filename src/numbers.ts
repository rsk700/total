export function clamp(value: number, minValue: number, maxValue: number): number {
    return Math.max(minValue, Math.min(maxValue, value));
}

export function lerp(a: number, b: number, t: number): number {
    return a + (b - a) * t;
}

export function lerpc(a: number, b: number, t: number): number {
    t = clamp(t, 0, 1);
    return lerp(a, b, t);
}