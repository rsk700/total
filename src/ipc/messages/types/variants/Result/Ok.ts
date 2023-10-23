import * as ldm from '../../../leapdatamodel';
import * as valuetype from '../../../valuetype';
import * as types from '../..';
import { LeapError as _LeapError } from '../../../LeapError';
import { ValueBase as _ValueBase } from '../../../ValueBase';


export class Ok<T,E> extends types.enums.Result<T,E> {
    static _variant: string = 'ok';
    static _leapVariant: valuetype.LeapVariant = new valuetype.LeapVariant("result", new valuetype.Property("ok", new valuetype.ValueType("some", [new valuetype.ValueType("t", [])])), ["t", "e"], [new valuetype.Property("value", new valuetype.ValueType("t", []))]);
    value: T;

    constructor(value: T, ) {
        super();
        this.value = value;
    }

    _toStruct(): types.structs.Some<T> {
        let s = new types.structs.Some<T>(
            this.value,
        );
        return s;
    }

    static  _fromStruct<T,E>(value: types.structs.Some<T>): Ok<T,E> {
        return new Ok<T,E>(
            value.value,
        );
    }

    _verifyAsTypeOrRaise(path: string[], valueType: valuetype.ValueType): void {
        if (valueType.name == Ok._type) {
            const variantType = Ok._leapVariant.applyArgs(valueType.args);
            _ValueBase._verifyValueOrRaise(path.concat(['value']), this.value, variantType.props[0].propType);
        } else {
            throw _ValueBase._pathError(path, `expecting ${Ok._type}`);
        }
    }

    _toLdm(valueType: valuetype.ValueType): ldm.Value {
        let value = new Map<string, ldm.Value>();
        const variantType = Ok._leapVariant.applyArgs(valueType.args);
        value.set("value", _ValueBase._valueToLdm(this.value, variantType.props[0].propType));
        return new ldm.EnumValue(
            "ok",
            new ldm.StructValue(value, variantType.variant.propType),
            valueType
        );
    }
}