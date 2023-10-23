import * as ldm from '../../leapdatamodel';
import * as valuetype from '../../valuetype';
import type * as types from '..';
import { StructBase as _StructBase } from '../../StructBase';
import { LeapError as _LeapError } from '../../LeapError';
import { ValueBase as _ValueBase } from '../../ValueBase';

export class ScanProgress extends _StructBase {
    static _type: string = 'scan-progress';
    static _leapStruct: valuetype.LeapStruct = new valuetype.LeapStruct("scan-progress", [], [new valuetype.Property("done-count", new valuetype.ValueType("int", []))]);
    doneCount: number;

    constructor(doneCount: number, ) {
        super();
        this.doneCount = doneCount;
    }

    _verifyAsTypeOrRaise(path: string[], valueType: valuetype.ValueType): void {
        if (valueType.name == ScanProgress._type) {
            const structType = ScanProgress._leapStruct.applyArgs(valueType.args);
            _ValueBase._verifyValueOrRaise(path.concat(['doneCount']), this.doneCount, structType.props[0].propType);
        } else {
            throw _ValueBase._pathError(path, `expecting ${ScanProgress._type}`);
        }
    }

    _toLdm(valueType: valuetype.ValueType): ldm.Value {
        let value = new Map<string, ldm.Value>();
        const structType = ScanProgress._leapStruct.applyArgs(valueType.args);
        value.set("done-count", _ValueBase._valueToLdm(this.doneCount, structType.props[0].propType));
        return new ldm.StructValue(value, valueType);
    }
}
