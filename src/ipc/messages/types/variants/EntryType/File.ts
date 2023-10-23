import * as ldm from '../../../leapdatamodel';
import * as valuetype from '../../../valuetype';
import * as types from '../..';
import { LeapError as _LeapError } from '../../../LeapError';
import { ValueBase as _ValueBase } from '../../../ValueBase';


export class File extends types.enums.EntryType {
    static _variant: string = 'file';
    static _leapVariant: valuetype.LeapVariant = new valuetype.LeapVariant("entry-type", new valuetype.Property("file", new valuetype.ValueType("none", [])), [], []);

    constructor() {
        super();
    }

    _toStruct(): types.structs.None {
        let s = new types.structs.None(
        );
        return s;
    }

    static  _fromStruct(value: types.structs.None): File {
        return new File(
        );
    }

    _verifyAsTypeOrRaise(path: string[], valueType: valuetype.ValueType): void {
        if (valueType.name == File._type) {
            const variantType = File._leapVariant.applyArgs(valueType.args);
        } else {
            throw _ValueBase._pathError(path, `expecting ${File._type}`);
        }
    }

    _toLdm(valueType: valuetype.ValueType): ldm.Value {
        let value = new Map<string, ldm.Value>();
        const variantType = File._leapVariant.applyArgs(valueType.args);
        return new ldm.EnumValue(
            "file",
            new ldm.StructValue(value, variantType.variant.propType),
            valueType
        );
    }
}