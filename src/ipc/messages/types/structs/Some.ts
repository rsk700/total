import * as ldm from '../../leapdatamodel';
import * as valuetype from '../../valuetype';
import type * as types from '..';
import { StructBase as _StructBase } from '../../StructBase';
import { LeapError as _LeapError } from '../../LeapError';
import { ValueBase as _ValueBase } from '../../ValueBase';

export class Some<T> extends _StructBase {
    static _type: string = 'some';
    static _leapStruct: valuetype.LeapStruct = new valuetype.LeapStruct("some", ["t"], [new valuetype.Property("value", new valuetype.ValueType("t", []))]);
    value: T;

    constructor(value: T, ) {
        super();
        this.value = value;
    }

    _verifyAsTypeOrRaise(path: string[], valueType: valuetype.ValueType): void {
        if (valueType.name == Some._type) {
            const structType = Some._leapStruct.applyArgs(valueType.args);
            _ValueBase._verifyValueOrRaise(path.concat(['value']), this.value, structType.props[0].propType);
        } else {
            throw _ValueBase._pathError(path, `expecting ${Some._type}`);
        }
    }

    _toLdm(valueType: valuetype.ValueType): ldm.Value {
        let value = new Map<string, ldm.Value>();
        const structType = Some._leapStruct.applyArgs(valueType.args);
        value.set("value", _ValueBase._valueToLdm(this.value, structType.props[0].propType));
        return new ldm.StructValue(value, valueType);
    }
}
