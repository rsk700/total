import * as ldm from '../../../leapdatamodel';
import * as valuetype from '../../../valuetype';
import * as types from '../..';
import { LeapError as _LeapError } from '../../../LeapError';
import { ValueBase as _ValueBase } from '../../../ValueBase';


export class None<T> extends types.enums.Option<T> {
    static _variant: string = 'none';
    static _leapVariant: valuetype.LeapVariant = new valuetype.LeapVariant("option", new valuetype.Property("none", new valuetype.ValueType("none", [])), ["t"], []);

    constructor() {
        super();
    }

    _toStruct(): types.structs.None {
        let s = new types.structs.None(
        );
        return s;
    }

    static  _fromStruct<T>(value: types.structs.None): None<T> {
        return new None<T>(
        );
    }

    _verifyAsTypeOrRaise(path: string[], valueType: valuetype.ValueType): void {
        if (valueType.name == None._type) {
            const variantType = None._leapVariant.applyArgs(valueType.args);
        } else {
            throw _ValueBase._pathError(path, `expecting ${None._type}`);
        }
    }

    _toLdm(valueType: valuetype.ValueType): ldm.Value {
        let value = new Map<string, ldm.Value>();
        const variantType = None._leapVariant.applyArgs(valueType.args);
        return new ldm.EnumValue(
            "none",
            new ldm.StructValue(value, variantType.variant.propType),
            valueType
        );
    }
}