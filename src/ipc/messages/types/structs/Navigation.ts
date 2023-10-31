import * as ldm from '../../leapdatamodel';
import * as valuetype from '../../valuetype';
import type * as types from '..';
import { StructBase as _StructBase } from '../../StructBase';
import { LeapError as _LeapError } from '../../LeapError';
import { ValueBase as _ValueBase } from '../../ValueBase';

export class Navigation extends _StructBase {
    static _type: string = 'navigation';
    static _leapStruct: valuetype.LeapStruct = new valuetype.LeapStruct("navigation", [], [new valuetype.Property("global-id", new valuetype.ValueType("int", [])), new valuetype.Property("path", new valuetype.ValueType("str", []))]);
    globalId: number;
    path: string;

    constructor(globalId: number, path: string, ) {
        super();
        this.globalId = globalId;
        this.path = path;
    }

    _verifyAsTypeOrRaise(path: string[], valueType: valuetype.ValueType): void {
        if (valueType.name == Navigation._type) {
            const structType = Navigation._leapStruct.applyArgs(valueType.args);
            _ValueBase._verifyValueOrRaise(path.concat(['globalId']), this.globalId, structType.props[0].propType);
            _ValueBase._verifyValueOrRaise(path.concat(['path']), this.path, structType.props[1].propType);
        } else {
            throw _ValueBase._pathError(path, `expecting ${Navigation._type}`);
        }
    }

    _toLdm(valueType: valuetype.ValueType): ldm.Value {
        let value = new Map<string, ldm.Value>();
        const structType = Navigation._leapStruct.applyArgs(valueType.args);
        value.set("global-id", _ValueBase._valueToLdm(this.globalId, structType.props[0].propType));
        value.set("path", _ValueBase._valueToLdm(this.path, structType.props[1].propType));
        return new ldm.StructValue(value, valueType);
    }
}
