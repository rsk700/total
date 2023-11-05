import * as ldm from '../../leapdatamodel';
import * as valuetype from '../../valuetype';
import type * as types from '..';
import { StructBase as _StructBase } from '../../StructBase';
import { LeapError as _LeapError } from '../../LeapError';
import { ValueBase as _ValueBase } from '../../ValueBase';

export class AggregateData extends _StructBase {
    static _type: string = 'aggregate-data';
    static _leapStruct: valuetype.LeapStruct = new valuetype.LeapStruct("aggregate-data", [], [new valuetype.Property("path", new valuetype.ValueType("str", [])), new valuetype.Property("path-top", new valuetype.ValueType("str", [])), new valuetype.Property("path-components", new valuetype.ValueType("list", [new valuetype.ValueType("path-component", [])])), new valuetype.Property("path-separator", new valuetype.ValueType("str", [])), new valuetype.Property("entries", new valuetype.ValueType("list", [new valuetype.ValueType("aggregate-entry", [])]))]);
    path: string;
    pathTop: string;
    pathComponents: Array<types.structs.PathComponent>;
    pathSeparator: string;
    entries: Array<types.structs.AggregateEntry>;

    constructor(path: string, pathTop: string, pathComponents: Array<types.structs.PathComponent>, pathSeparator: string, entries: Array<types.structs.AggregateEntry>, ) {
        super();
        this.path = path;
        this.pathTop = pathTop;
        this.pathComponents = pathComponents;
        this.pathSeparator = pathSeparator;
        this.entries = entries;
    }

    _verifyAsTypeOrRaise(path: string[], valueType: valuetype.ValueType): void {
        if (valueType.name == AggregateData._type) {
            const structType = AggregateData._leapStruct.applyArgs(valueType.args);
            _ValueBase._verifyValueOrRaise(path.concat(['path']), this.path, structType.props[0].propType);
            _ValueBase._verifyValueOrRaise(path.concat(['pathTop']), this.pathTop, structType.props[1].propType);
            _ValueBase._verifyValueOrRaise(path.concat(['pathComponents']), this.pathComponents, structType.props[2].propType);
            _ValueBase._verifyValueOrRaise(path.concat(['pathSeparator']), this.pathSeparator, structType.props[3].propType);
            _ValueBase._verifyValueOrRaise(path.concat(['entries']), this.entries, structType.props[4].propType);
        } else {
            throw _ValueBase._pathError(path, `expecting ${AggregateData._type}`);
        }
    }

    _toLdm(valueType: valuetype.ValueType): ldm.Value {
        let value = new Map<string, ldm.Value>();
        const structType = AggregateData._leapStruct.applyArgs(valueType.args);
        value.set("path", _ValueBase._valueToLdm(this.path, structType.props[0].propType));
        value.set("path-top", _ValueBase._valueToLdm(this.pathTop, structType.props[1].propType));
        value.set("path-components", _ValueBase._valueToLdm(this.pathComponents, structType.props[2].propType));
        value.set("path-separator", _ValueBase._valueToLdm(this.pathSeparator, structType.props[3].propType));
        value.set("entries", _ValueBase._valueToLdm(this.entries, structType.props[4].propType));
        return new ldm.StructValue(value, valueType);
    }
}
