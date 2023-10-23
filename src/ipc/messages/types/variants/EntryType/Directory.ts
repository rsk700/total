import * as ldm from '../../../leapdatamodel';
import * as valuetype from '../../../valuetype';
import * as types from '../..';
import { LeapError as _LeapError } from '../../../LeapError';
import { ValueBase as _ValueBase } from '../../../ValueBase';


export class Directory extends types.enums.EntryType {
    static _variant: string = 'directory';
    static _leapVariant: valuetype.LeapVariant = new valuetype.LeapVariant("entry-type", new valuetype.Property("directory", new valuetype.ValueType("none", [])), [], []);

    constructor() {
        super();
    }

    _toStruct(): types.structs.None {
        let s = new types.structs.None(
        );
        return s;
    }

    static  _fromStruct(value: types.structs.None): Directory {
        return new Directory(
        );
    }

    _verifyAsTypeOrRaise(path: string[], valueType: valuetype.ValueType): void {
        if (valueType.name == Directory._type) {
            const variantType = Directory._leapVariant.applyArgs(valueType.args);
        } else {
            throw _ValueBase._pathError(path, `expecting ${Directory._type}`);
        }
    }

    _toLdm(valueType: valuetype.ValueType): ldm.Value {
        let value = new Map<string, ldm.Value>();
        const variantType = Directory._leapVariant.applyArgs(valueType.args);
        return new ldm.EnumValue(
            "directory",
            new ldm.StructValue(value, variantType.variant.propType),
            valueType
        );
    }
}