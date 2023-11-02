export function lerp(a: number, b: number, t: number): number {
    return a + (b - a) * t;
}

export function lerpc(a: number, b: number, t: number): number {
    t = Math.max(0, Math.min(1, t));
    return lerp(a, b, t);
}