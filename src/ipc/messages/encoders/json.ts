import type { Verify } from './encoders';
import { VerificationResult, VerificationError } from './encoders';
import type { ValueType } from '../valuetype';
import type { ValueBase } from '../ValueBase';
import { FromJsobj, ToJsobj } from './jsobj';
import type * as ldm from '../leapdatamodel';

export class FromJson<T> implements Verify {
    fromJsobj: FromJsobj<T> | null = null;

    constructor(data: string, valueType: ValueType) {
        try {
            const obj = JSON.parse(data);
            this.fromJsobj = new FromJsobj<T>(obj, valueType);
        } catch { }
    }

    verify(upToFirstError: boolean): VerificationResult {
        if (this.fromJsobj === null) {
            return new VerificationError(['incorrect json data']);
        } else {
            return this.fromJsobj.verify(upToFirstError);
        }
    }

    toValue(): T {
        return (this.fromJsobj as FromJsobj<T>).toValue();
    }

    toLdm(): ldm.Value {
        return (this.fromJsobj as FromJsobj<T>).toLdm();
    }
}


export class ToJson {
    static fromLdm(value: ldm.Value): string {
        return JSON.stringify(ToJsobj.fromLdm(value));
    }

    static fromValue(value: ValueBase, valueType: ValueType): string {
        return this.fromLdm(value._toLdm(valueType));
    }
}