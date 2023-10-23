import * as ldm from '../../../leapdatamodel';
import * as valuetype from '../../../valuetype';
import * as types from '../..';
import { LeapError as _LeapError } from '../../../LeapError';
import { ValueBase as _ValueBase } from '../../../ValueBase';


export class ScanProgress extends types.enums.ScanState {
    static _variant: string = 'scan-progress';
    static _leapVariant: valuetype.LeapVariant = new valuetype.LeapVariant("scan-state", new valuetype.Property("scan-progress", new valuetype.ValueType("scan-progress", [])), [], [new valuetype.Property("done-count", new valuetype.ValueType("int", []))]);
    doneCount: number;

    constructor(doneCount: number, ) {
        super();
        this.doneCount = doneCount;
    }

    _toStruct(): types.structs.ScanProgress {
        let s = new types.structs.ScanProgress(
            this.doneCount,
        );
        return s;
    }

    static  _fromStruct(value: types.structs.ScanProgress): ScanProgress {
        return new ScanProgress(
            value.doneCount,
        );
    }

    _verifyAsTypeOrRaise(path: string[], valueType: valuetype.ValueType): void {
        if (valueType.name == ScanProgress._type) {
            const variantType = ScanProgress._leapVariant.applyArgs(valueType.args);
            _ValueBase._verifyValueOrRaise(path.concat(['doneCount']), this.doneCount, variantType.props[0].propType);
        } else {
            throw _ValueBase._pathError(path, `expecting ${ScanProgress._type}`);
        }
    }

    _toLdm(valueType: valuetype.ValueType): ldm.Value {
        let value = new Map<string, ldm.Value>();
        const variantType = ScanProgress._leapVariant.applyArgs(valueType.args);
        value.set("done-count", _ValueBase._valueToLdm(this.doneCount, variantType.props[0].propType));
        return new ldm.EnumValue(
            "scan-progress",
            new ldm.StructValue(value, variantType.variant.propType),
            valueType
        );
    }
}