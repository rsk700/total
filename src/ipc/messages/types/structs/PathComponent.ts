import * as ldm from '../../leapdatamodel';
import * as valuetype from '../../valuetype';
import type * as types from '..';
import { StructBase as _StructBase } from '../../StructBase';
import { LeapError as _LeapError } from '../../LeapError';
import { ValueBase as _ValueBase } from '../../ValueBase';

export class PathComponent extends _StructBase {
    static _type: string = 'path-component';
    static _leapStruct: valuetype.LeapStruct = new valuetype.LeapStruct("path-component", [], [new valuetype.Property("path", new valuetype.ValueType("str", [])), new valuetype.Property("name", new valuetype.ValueType("str", []))]);
    path: string;
    name: string;

    constructor(path: string, name: string, ) {
        super();
        this.path = path;
        this.name = name;
    }

    _verifyAsTypeOrRaise(path: string[], valueType: valuetype.ValueType): void {
        if (valueType.name == PathComponent._type) {
            const structType = PathComponent._leapStruct.applyArgs(valueType.args);
            _ValueBase._verifyValueOrRaise(path.concat(['path']), this.path, structType.props[0].propType);
            _ValueBase._verifyValueOrRaise(path.concat(['name']), this.name, structType.props[1].propType);
        } else {
            throw _ValueBase._pathError(path, `expecting ${PathComponent._type}`);
        }
    }

    _toLdm(valueType: valuetype.ValueType): ldm.Value {
        let value = new Map<string, ldm.Value>();
        const structType = PathComponent._leapStruct.applyArgs(valueType.args);
        value.set("path", _ValueBase._valueToLdm(this.path, structType.props[0].propType));
        value.set("name", _ValueBase._valueToLdm(this.name, structType.props[1].propType));
        return new ldm.StructValue(value, valueType);
    }
}
