import * as ldm from '../../../leapdatamodel';
import * as valuetype from '../../../valuetype';
import * as types from '../..';
import { LeapError as _LeapError } from '../../../LeapError';
import { ValueBase as _ValueBase } from '../../../ValueBase';


export class Ready extends types.enums.ScanState {
    static _variant: string = 'ready';
    static _leapVariant: valuetype.LeapVariant = new valuetype.LeapVariant("scan-state", new valuetype.Property("ready", new valuetype.ValueType("none", [])), [], []);

    constructor() {
        super();
    }

    _toStruct(): types.structs.None {
        let s = new types.structs.None(
        );
        return s;
    }

    static  _fromStruct(value: types.structs.None): Ready {
        return new Ready(
        );
    }

    _verifyAsTypeOrRaise(path: string[], valueType: valuetype.ValueType): void {
        if (valueType.name == Ready._type) {
            const variantType = Ready._leapVariant.applyArgs(valueType.args);
        } else {
            throw _ValueBase._pathError(path, `expecting ${Ready._type}`);
        }
    }

    _toLdm(valueType: valuetype.ValueType): ldm.Value {
        let value = new Map<string, ldm.Value>();
        const variantType = Ready._leapVariant.applyArgs(valueType.args);
        return new ldm.EnumValue(
            "ready",
            new ldm.StructValue(value, variantType.variant.propType),
            valueType
        );
    }
}