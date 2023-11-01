import * as ldm from '../../leapdatamodel';
import * as valuetype from '../../valuetype';
import type * as types from '..';
import { StructBase as _StructBase } from '../../StructBase';
import { LeapError as _LeapError } from '../../LeapError';
import { ValueBase as _ValueBase } from '../../ValueBase';

export class AboutApp extends _StructBase {
    static _type: string = 'about-app';
    static _leapStruct: valuetype.LeapStruct = new valuetype.LeapStruct("about-app", [], [new valuetype.Property("version", new valuetype.ValueType("str", []))]);
    version: string;

    constructor(version: string, ) {
        super();
        this.version = version;
    }

    _verifyAsTypeOrRaise(path: string[], valueType: valuetype.ValueType): void {
        if (valueType.name == AboutApp._type) {
            const structType = AboutApp._leapStruct.applyArgs(valueType.args);
            _ValueBase._verifyValueOrRaise(path.concat(['version']), this.version, structType.props[0].propType);
        } else {
            throw _ValueBase._pathError(path, `expecting ${AboutApp._type}`);
        }
    }

    _toLdm(valueType: valuetype.ValueType): ldm.Value {
        let value = new Map<string, ldm.Value>();
        const structType = AboutApp._leapStruct.applyArgs(valueType.args);
        value.set("version", _ValueBase._valueToLdm(this.version, structType.props[0].propType));
        return new ldm.StructValue(value, valueType);
    }
}
