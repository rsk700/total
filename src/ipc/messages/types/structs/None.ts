import * as ldm from '../../leapdatamodel';
import * as valuetype from '../../valuetype';
import type * as types from '..';
import { StructBase as _StructBase } from '../../StructBase';
import { LeapError as _LeapError } from '../../LeapError';
import { ValueBase as _ValueBase } from '../../ValueBase';

export class None extends _StructBase {
    static _type: string = 'none';
    static _leapStruct: valuetype.LeapStruct = new valuetype.LeapStruct("none", [], []);

    constructor() {
        super();
    }

    _verifyAsTypeOrRaise(path: string[], valueType: valuetype.ValueType): void {
        if (valueType.name == None._type) {
            const structType = None._leapStruct.applyArgs(valueType.args);
        } else {
            throw _ValueBase._pathError(path, `expecting ${None._type}`);
        }
    }

    _toLdm(valueType: valuetype.ValueType): ldm.Value {
        let value = new Map<string, ldm.Value>();
        const structType = None._leapStruct.applyArgs(valueType.args);
        return new ldm.StructValue(value, valueType);
    }
}
