import * as ldm from '../../leapdatamodel';
import * as valuetype from '../../valuetype';
import type * as types from '..';
import { StructBase as _StructBase } from '../../StructBase';
import { LeapError as _LeapError } from '../../LeapError';
import { ValueBase as _ValueBase } from '../../ValueBase';

export class PathAggregate extends _StructBase {
    static _type: string = 'path-aggregate';
    static _leapStruct: valuetype.LeapStruct = new valuetype.LeapStruct("path-aggregate", [], [new valuetype.Property("entries", new valuetype.ValueType("list", [new valuetype.ValueType("entry", [])])), new valuetype.Property("tree", new valuetype.ValueType("list", [new valuetype.ValueType("list", [new valuetype.ValueType("int", [])])]))]);
    entries: Array<types.structs.Entry>;
    tree: Array<Array<number>>;

    constructor(entries: Array<types.structs.Entry>, tree: Array<Array<number>>, ) {
        super();
        this.entries = entries;
        this.tree = tree;
    }

    _verifyAsTypeOrRaise(path: string[], valueType: valuetype.ValueType): void {
        if (valueType.name == PathAggregate._type) {
            const structType = PathAggregate._leapStruct.applyArgs(valueType.args);
            _ValueBase._verifyValueOrRaise(path.concat(['entries']), this.entries, structType.props[0].propType);
            _ValueBase._verifyValueOrRaise(path.concat(['tree']), this.tree, structType.props[1].propType);
        } else {
            throw _ValueBase._pathError(path, `expecting ${PathAggregate._type}`);
        }
    }

    _toLdm(valueType: valuetype.ValueType): ldm.Value {
        let value = new Map<string, ldm.Value>();
        const structType = PathAggregate._leapStruct.applyArgs(valueType.args);
        value.set("entries", _ValueBase._valueToLdm(this.entries, structType.props[0].propType));
        value.set("tree", _ValueBase._valueToLdm(this.tree, structType.props[1].propType));
        return new ldm.StructValue(value, valueType);
    }
}
