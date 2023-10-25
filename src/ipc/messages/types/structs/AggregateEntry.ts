import * as ldm from '../../leapdatamodel';
import * as valuetype from '../../valuetype';
import type * as types from '..';
import { StructBase as _StructBase } from '../../StructBase';
import { LeapError as _LeapError } from '../../LeapError';
import { ValueBase as _ValueBase } from '../../ValueBase';

export class AggregateEntry extends _StructBase {
    static _type: string = 'aggregate-entry';
    static _leapStruct: valuetype.LeapStruct = new valuetype.LeapStruct("aggregate-entry", [], [new valuetype.Property("id", new valuetype.ValueType("int", [])), new valuetype.Property("name", new valuetype.ValueType("str", [])), new valuetype.Property("path", new valuetype.ValueType("str", [])), new valuetype.Property("self-size", new valuetype.ValueType("int", [])), new valuetype.Property("size", new valuetype.ValueType("int", [])), new valuetype.Property("tail-size", new valuetype.ValueType("int", [])), new valuetype.Property("self-file-count", new valuetype.ValueType("int", [])), new valuetype.Property("file-count", new valuetype.ValueType("int", [])), new valuetype.Property("tail-file-count", new valuetype.ValueType("int", [])), new valuetype.Property("self-dir-count", new valuetype.ValueType("int", [])), new valuetype.Property("dir-count", new valuetype.ValueType("int", [])), new valuetype.Property("tail-dir-count", new valuetype.ValueType("int", [])), new valuetype.Property("is-file", new valuetype.ValueType("bool", [])), new valuetype.Property("nested", new valuetype.ValueType("list", [new valuetype.ValueType("int", [])])), new valuetype.Property("parent", new valuetype.ValueType("option", [new valuetype.ValueType("int", [])]))]);
    id: number;
    name: string;
    path: string;
    selfSize: number;
    size: number;
    tailSize: number;
    selfFileCount: number;
    fileCount: number;
    tailFileCount: number;
    selfDirCount: number;
    dirCount: number;
    tailDirCount: number;
    isFile: boolean;
    nested: Array<number>;
    parent: number | null;

    constructor(id: number, name: string, path: string, selfSize: number, size: number, tailSize: number, selfFileCount: number, fileCount: number, tailFileCount: number, selfDirCount: number, dirCount: number, tailDirCount: number, isFile: boolean, nested: Array<number>, parent: number | null, ) {
        super();
        this.id = id;
        this.name = name;
        this.path = path;
        this.selfSize = selfSize;
        this.size = size;
        this.tailSize = tailSize;
        this.selfFileCount = selfFileCount;
        this.fileCount = fileCount;
        this.tailFileCount = tailFileCount;
        this.selfDirCount = selfDirCount;
        this.dirCount = dirCount;
        this.tailDirCount = tailDirCount;
        this.isFile = isFile;
        this.nested = nested;
        this.parent = parent;
    }

    _verifyAsTypeOrRaise(path: string[], valueType: valuetype.ValueType): void {
        if (valueType.name == AggregateEntry._type) {
            const structType = AggregateEntry._leapStruct.applyArgs(valueType.args);
            _ValueBase._verifyValueOrRaise(path.concat(['id']), this.id, structType.props[0].propType);
            _ValueBase._verifyValueOrRaise(path.concat(['name']), this.name, structType.props[1].propType);
            _ValueBase._verifyValueOrRaise(path.concat(['path']), this.path, structType.props[2].propType);
            _ValueBase._verifyValueOrRaise(path.concat(['selfSize']), this.selfSize, structType.props[3].propType);
            _ValueBase._verifyValueOrRaise(path.concat(['size']), this.size, structType.props[4].propType);
            _ValueBase._verifyValueOrRaise(path.concat(['tailSize']), this.tailSize, structType.props[5].propType);
            _ValueBase._verifyValueOrRaise(path.concat(['selfFileCount']), this.selfFileCount, structType.props[6].propType);
            _ValueBase._verifyValueOrRaise(path.concat(['fileCount']), this.fileCount, structType.props[7].propType);
            _ValueBase._verifyValueOrRaise(path.concat(['tailFileCount']), this.tailFileCount, structType.props[8].propType);
            _ValueBase._verifyValueOrRaise(path.concat(['selfDirCount']), this.selfDirCount, structType.props[9].propType);
            _ValueBase._verifyValueOrRaise(path.concat(['dirCount']), this.dirCount, structType.props[10].propType);
            _ValueBase._verifyValueOrRaise(path.concat(['tailDirCount']), this.tailDirCount, structType.props[11].propType);
            _ValueBase._verifyValueOrRaise(path.concat(['isFile']), this.isFile, structType.props[12].propType);
            _ValueBase._verifyValueOrRaise(path.concat(['nested']), this.nested, structType.props[13].propType);
            _ValueBase._verifyValueOrRaise(path.concat(['parent']), this.parent, structType.props[14].propType);
        } else {
            throw _ValueBase._pathError(path, `expecting ${AggregateEntry._type}`);
        }
    }

    _toLdm(valueType: valuetype.ValueType): ldm.Value {
        let value = new Map<string, ldm.Value>();
        const structType = AggregateEntry._leapStruct.applyArgs(valueType.args);
        value.set("id", _ValueBase._valueToLdm(this.id, structType.props[0].propType));
        value.set("name", _ValueBase._valueToLdm(this.name, structType.props[1].propType));
        value.set("path", _ValueBase._valueToLdm(this.path, structType.props[2].propType));
        value.set("self-size", _ValueBase._valueToLdm(this.selfSize, structType.props[3].propType));
        value.set("size", _ValueBase._valueToLdm(this.size, structType.props[4].propType));
        value.set("tail-size", _ValueBase._valueToLdm(this.tailSize, structType.props[5].propType));
        value.set("self-file-count", _ValueBase._valueToLdm(this.selfFileCount, structType.props[6].propType));
        value.set("file-count", _ValueBase._valueToLdm(this.fileCount, structType.props[7].propType));
        value.set("tail-file-count", _ValueBase._valueToLdm(this.tailFileCount, structType.props[8].propType));
        value.set("self-dir-count", _ValueBase._valueToLdm(this.selfDirCount, structType.props[9].propType));
        value.set("dir-count", _ValueBase._valueToLdm(this.dirCount, structType.props[10].propType));
        value.set("tail-dir-count", _ValueBase._valueToLdm(this.tailDirCount, structType.props[11].propType));
        value.set("is-file", _ValueBase._valueToLdm(this.isFile, structType.props[12].propType));
        value.set("nested", _ValueBase._valueToLdm(this.nested, structType.props[13].propType));
        value.set("parent", _ValueBase._valueToLdm(this.parent, structType.props[14].propType));
        return new ldm.StructValue(value, valueType);
    }
}
