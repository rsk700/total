function hrStepped(n: number, stepSize: number, suffixes: string[]): string {
    if (n < stepSize) {
        return `${n}${suffixes[0]}`;
    }
    let step = Math.floor(Math.log(n) / Math.log(stepSize));
    n = Math.floor(n / Math.pow(stepSize, step));
    if (step < suffixes.length) {
        return `${n}${suffixes[step]}`;
    } else {
        return `${n}${suffixes[suffixes.length - 1]}`;
    }
}

// B - bytes
// K - kibibyte 1024^1
// M - mebibyte 1024^2
// G - gibibyte 1024^3
// T - tebibyte 1024^4
// P - pebibyte 1024^5
const byteSuffixes = ["B", "K", "M", "G", "T", "P"];
const byteStep = 1024;

// human readable size in bytes
export function hrByteSize(sizeBytes: number): string {
    return hrStepped(sizeBytes, byteStep, byteSuffixes);
}

// K - thousands 10^3
// M - millions 10^6
// B - billions 10^9
// T - trillions 10^12
const countSuffixes = ["", "K", "M", "B", "T"];
const countStep = 1000;

// human readable count
export function hrCount(count: number): string {
    return hrStepped(count, countStep, countSuffixes);
}