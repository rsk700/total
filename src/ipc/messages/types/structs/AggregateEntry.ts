import * as ldm from '../../leapdatamodel';
import * as valuetype from '../../valuetype';
import type * as types from '..';
import { StructBase as _StructBase } from '../../StructBase';
import { LeapError as _LeapError } from '../../LeapError';
import { ValueBase as _ValueBase } from '../../ValueBase';

export class AggregateEntry extends _StructBase {
    static _type: string = 'aggregate-entry';
    static _leapStruct: valuetype.LeapStruct = new valuetype.LeapStruct("aggregate-entry", [], [new valuetype.Property("global-id", new valuetype.ValueType("int", [])), new valuetype.Property("local-id", new valuetype.ValueType("int", [])), new valuetype.Property("name", new valuetype.ValueType("str", [])), new valuetype.Property("name-hash", new valuetype.ValueType("int", [])), new valuetype.Property("path", new valuetype.ValueType("str", [])), new valuetype.Property("self-size", new valuetype.ValueType("int", [])), new valuetype.Property("size", new valuetype.ValueType("int", [])), new valuetype.Property("tail-size", new valuetype.ValueType("int", [])), new valuetype.Property("self-file-count", new valuetype.ValueType("int", [])), new valuetype.Property("file-count", new valuetype.ValueType("int", [])), new valuetype.Property("tail-file-count", new valuetype.ValueType("int", [])), new valuetype.Property("self-dir-count", new valuetype.ValueType("int", [])), new valuetype.Property("dir-count", new valuetype.ValueType("int", [])), new valuetype.Property("tail-dir-count", new valuetype.ValueType("int", [])), new valuetype.Property("is-file", new valuetype.ValueType("bool", [])), new valuetype.Property("global-parent", new valuetype.ValueType("option", [new valuetype.ValueType("int", [])])), new valuetype.Property("local-parent", new valuetype.ValueType("option", [new valuetype.ValueType("int", [])])), new valuetype.Property("nested", new valuetype.ValueType("list", [new valuetype.ValueType("int", [])]))]);
    globalId: number;
    localId: number;
    name: string;
    nameHash: number;
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
    globalParent: number | null;
    localParent: number | null;
    nested: Array<number>;

    constructor(globalId: number, localId: number, name: string, nameHash: number, path: string, selfSize: number, size: number, tailSize: number, selfFileCount: number, fileCount: number, tailFileCount: number, selfDirCount: number, dirCount: number, tailDirCount: number, isFile: boolean, globalParent: number | null, localParent: number | null, nested: Array<number>, ) {
        super();
        this.globalId = globalId;
        this.localId = localId;
        this.name = name;
        this.nameHash = nameHash;
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
        this.globalParent = globalParent;
        this.localParent = localParent;
        this.nested = nested;
    }

    _verifyAsTypeOrRaise(path: string[], valueType: valuetype.ValueType): void {
        if (valueType.name == AggregateEntry._type) {
            const structType = AggregateEntry._leapStruct.applyArgs(valueType.args);
            _ValueBase._verifyValueOrRaise(path.concat(['globalId']), this.globalId, structType.props[0].propType);
            _ValueBase._verifyValueOrRaise(path.concat(['localId']), this.localId, structType.props[1].propType);
            _ValueBase._verifyValueOrRaise(path.concat(['name']), this.name, structType.props[2].propType);
            _ValueBase._verifyValueOrRaise(path.concat(['nameHash']), this.nameHash, structType.props[3].propType);
            _ValueBase._verifyValueOrRaise(path.concat(['path']), this.path, structType.props[4].propType);
            _ValueBase._verifyValueOrRaise(path.concat(['selfSize']), this.selfSize, structType.props[5].propType);
            _ValueBase._verifyValueOrRaise(path.concat(['size']), this.size, structType.props[6].propType);
            _ValueBase._verifyValueOrRaise(path.concat(['tailSize']), this.tailSize, structType.props[7].propType);
            _ValueBase._verifyValueOrRaise(path.concat(['selfFileCount']), this.selfFileCount, structType.props[8].propType);
            _ValueBase._verifyValueOrRaise(path.concat(['fileCount']), this.fileCount, structType.props[9].propType);
            _ValueBase._verifyValueOrRaise(path.concat(['tailFileCount']), this.tailFileCount, structType.props[10].propType);
            _ValueBase._verifyValueOrRaise(path.concat(['selfDirCount']), this.selfDirCount, structType.props[11].propType);
            _ValueBase._verifyValueOrRaise(path.concat(['dirCount']), this.dirCount, structType.props[12].propType);
            _ValueBase._verifyValueOrRaise(path.concat(['tailDirCount']), this.tailDirCount, structType.props[13].propType);
            _ValueBase._verifyValueOrRaise(path.concat(['isFile']), this.isFile, structType.props[14].propType);
            _ValueBase._verifyValueOrRaise(path.concat(['globalParent']), this.globalParent, structType.props[15].propType);
            _ValueBase._verifyValueOrRaise(path.concat(['localParent']), this.localParent, structType.props[16].propType);
            _ValueBase._verifyValueOrRaise(path.concat(['nested']), this.nested, structType.props[17].propType);
        } else {
            throw _ValueBase._pathError(path, `expecting ${AggregateEntry._type}`);
        }
    }

    _toLdm(valueType: valuetype.ValueType): ldm.Value {
        let value = new Map<string, ldm.Value>();
        const structType = AggregateEntry._leapStruct.applyArgs(valueType.args);
        value.set("global-id", _ValueBase._valueToLdm(this.globalId, structType.props[0].propType));
        value.set("local-id", _ValueBase._valueToLdm(this.localId, structType.props[1].propType));
        value.set("name", _ValueBase._valueToLdm(this.name, structType.props[2].propType));
        value.set("name-hash", _ValueBase._valueToLdm(this.nameHash, structType.props[3].propType));
        value.set("path", _ValueBase._valueToLdm(this.path, structType.props[4].propType));
        value.set("self-size", _ValueBase._valueToLdm(this.selfSize, structType.props[5].propType));
        value.set("size", _ValueBase._valueToLdm(this.size, structType.props[6].propType));
        value.set("tail-size", _ValueBase._valueToLdm(this.tailSize, structType.props[7].propType));
        value.set("self-file-count", _ValueBase._valueToLdm(this.selfFileCount, structType.props[8].propType));
        value.set("file-count", _ValueBase._valueToLdm(this.fileCount, structType.props[9].propType));
        value.set("tail-file-count", _ValueBase._valueToLdm(this.tailFileCount, structType.props[10].propType));
        value.set("self-dir-count", _ValueBase._valueToLdm(this.selfDirCount, structType.props[11].propType));
        value.set("dir-count", _ValueBase._valueToLdm(this.dirCount, structType.props[12].propType));
        value.set("tail-dir-count", _ValueBase._valueToLdm(this.tailDirCount, structType.props[13].propType));
        value.set("is-file", _ValueBase._valueToLdm(this.isFile, structType.props[14].propType));
        value.set("global-parent", _ValueBase._valueToLdm(this.globalParent, structType.props[15].propType));
        value.set("local-parent", _ValueBase._valueToLdm(this.localParent, structType.props[16].propType));
        value.set("nested", _ValueBase._valueToLdm(this.nested, structType.props[17].propType));
        return new ldm.StructValue(value, valueType);
    }
}
