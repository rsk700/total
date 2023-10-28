const byteStep = 1024;

// human readable size in bytes
export function hrByteSize(sizeBytes: number): string {
    if (sizeBytes < 1024) {
        return `${sizeBytes}B`;
    }
    let step = Math.floor(Math.log(sizeBytes) / Math.log(byteStep));
    let size = Math.pow(byteStep, step);
    if (step === 1) {
        return `${size}K`;
    } else if (step === 2) {
        return `${size}M`;
    } else if (step === 3) {
        return `${size}G`;
    } else if (step === 4) {
        return `${size}T`;
    } else {
        return `${size}P`;
    }
}