import * as ldm from '../../../leapdatamodel';
import * as valuetype from '../../../valuetype';
import * as types from '../..';
import { LeapError as _LeapError } from '../../../LeapError';
import { ValueBase as _ValueBase } from '../../../ValueBase';


export class Some<T> extends types.enums.Option<T> {
    static _variant: string = 'some';
    static _leapVariant: valuetype.LeapVariant = new valuetype.LeapVariant("option", new valuetype.Property("some", new valuetype.ValueType("some", [new valuetype.ValueType("t", [])])), ["t"], [new valuetype.Property("value", new valuetype.ValueType("t", []))]);
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

    static  _fromStruct<T>(value: types.structs.Some<T>): Some<T> {
        return new Some<T>(
            value.value,
        );
    }

    _verifyAsTypeOrRaise(path: string[], valueType: valuetype.ValueType): void {
        if (valueType.name == Some._type) {
            const variantType = Some._leapVariant.applyArgs(valueType.args);
            _ValueBase._verifyValueOrRaise(path.concat(['value']), this.value, variantType.props[0].propType);
        } else {
            throw _ValueBase._pathError(path, `expecting ${Some._type}`);
        }
    }

    _toLdm(valueType: valuetype.ValueType): ldm.Value {
        let value = new Map<string, ldm.Value>();
        const variantType = Some._leapVariant.applyArgs(valueType.args);
        value.set("value", _ValueBase._valueToLdm(this.value, variantType.props[0].propType));
        return new ldm.EnumValue(
            "some",
            new ldm.StructValue(value, variantType.variant.propType),
            valueType
        );
    }
}