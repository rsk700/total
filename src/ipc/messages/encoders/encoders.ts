export function formatError(path: string[], message: string): string {
    if (path.length === 0) {
        return message;
    } else {
        const p = path.join('.');
        return `${p}: ${message}`;
    }
}

export interface Verify {
    verify(upToFirstError: boolean): VerificationResult;
}

export abstract class VerificationResult { };

export class VerificationOk implements VerificationResult { }

export class VerificationError implements VerificationResult {
    errors: string[];

    constructor(errors: string[]) {
        this.errors = errors;
    }
}
