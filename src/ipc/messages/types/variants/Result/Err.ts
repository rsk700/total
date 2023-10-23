import * as ldm from '../../../leapdatamodel';
import * as valuetype from '../../../valuetype';
import * as types from '../..';
import { LeapError as _LeapError } from '../../../LeapError';
import { ValueBase as _ValueBase } from '../../../ValueBase';


export class Err<T,E> extends types.enums.Result<T,E> {
    static _variant: string = 'err';
    static _leapVariant: valuetype.LeapVariant = new valuetype.LeapVariant("result", new valuetype.Property("err", new valuetype.ValueType("some", [new valuetype.ValueType("e", [])])), ["t", "e"], [new valuetype.Property("value", new valuetype.ValueType("e", []))]);
    value: E;

    constructor(value: E, ) {
        super();
        this.value = value;
    }

    _toStruct(): types.structs.Some<E> {
        let s = new types.structs.Some<E>(
            this.value,
        );
        return s;
    }

    static  _fromStruct<T,E>(value: types.structs.Some<E>): Err<T,E> {
        return new Err<T,E>(
            value.value,
        );
    }

    _verifyAsTypeOrRaise(path: string[], valueType: valuetype.ValueType): void {
        if (valueType.name == Err._type) {
            const variantType = Err._leapVariant.applyArgs(valueType.args);
            _ValueBase._verifyValueOrRaise(path.concat(['value']), this.value, variantType.props[0].propType);
        } else {
            throw _ValueBase._pathError(path, `expecting ${Err._type}`);
        }
    }

    _toLdm(valueType: valuetype.ValueType): ldm.Value {
        let value = new Map<string, ldm.Value>();
        const variantType = Err._leapVariant.applyArgs(valueType.args);
        value.set("value", _ValueBase._valueToLdm(this.value, variantType.props[0].propType));
        return new ldm.EnumValue(
            "err",
            new ldm.StructValue(value, variantType.variant.propType),
            valueType
        );
    }
}