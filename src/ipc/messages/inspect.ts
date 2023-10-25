import * as valuetype from './valuetype';
import type { LeapStruct, LeapEnum, LeapVariant } from './valuetype';

export const leapStructsMap = new Map<string, LeapStruct> ([
    ["entry", new valuetype.LeapStruct("entry", [], [new valuetype.Property("id", new valuetype.ValueType("int", [])), new valuetype.Property("name", new valuetype.ValueType("str", [])), new valuetype.Property("path", new valuetype.ValueType("str", [])), new valuetype.Property("self-size", new valuetype.ValueType("int", [])), new valuetype.Property("size", new valuetype.ValueType("int", [])), new valuetype.Property("self-file-count", new valuetype.ValueType("int", [])), new valuetype.Property("tail-size", new valuetype.ValueType("int", [])), new valuetype.Property("file-count", new valuetype.ValueType("int", [])), new valuetype.Property("self-dir-count", new valuetype.ValueType("int", [])), new valuetype.Property("dir-count", new valuetype.ValueType("int", [])), new valuetype.Property("is-file", new valuetype.ValueType("bool", [])), new valuetype.Property("parent", new valuetype.ValueType("option", [new valuetype.ValueType("int", [])]))])],
    ["path-aggregate", new valuetype.LeapStruct("path-aggregate", [], [new valuetype.Property("entries", new valuetype.ValueType("list", [new valuetype.ValueType("entry", [])])), new valuetype.Property("tree", new valuetype.ValueType("list", [new valuetype.ValueType("list", [new valuetype.ValueType("int", [])])]))])],
    ["scan-progress", new valuetype.LeapStruct("scan-progress", [], [new valuetype.Property("done-count", new valuetype.ValueType("int", []))])],
    ["none", new valuetype.LeapStruct("none", [], [])],
    ["some", new valuetype.LeapStruct("some", ["t"], [new valuetype.Property("value", new valuetype.ValueType("t", []))])],
]);

export const leapEnumsMap = new Map<string, LeapEnum> ([
    ["scan-state", new valuetype.LeapEnum("scan-state", [], [new valuetype.Property("ready", new valuetype.ValueType("none", [])), new valuetype.Property("scan-progress", new valuetype.ValueType("scan-progress", []))])],
    ["option", new valuetype.LeapEnum("option", ["t"], [new valuetype.Property("none", new valuetype.ValueType("none", [])), new valuetype.Property("some", new valuetype.ValueType("some", [new valuetype.ValueType("t", [])]))])],
    ["result", new valuetype.LeapEnum("result", ["t", "e"], [new valuetype.Property("ok", new valuetype.ValueType("some", [new valuetype.ValueType("t", [])])), new valuetype.Property("err", new valuetype.ValueType("some", [new valuetype.ValueType("e", [])]))])],
]);

export const leapVariantsMap = new Map<string, Map<string, LeapVariant>>([
    [
        "scan-state",
        new Map<string, LeapVariant>([
            ["ready", new valuetype.LeapVariant("scan-state", new valuetype.Property("ready", new valuetype.ValueType("none", [])), [], [])],
            ["scan-progress", new valuetype.LeapVariant("scan-state", new valuetype.Property("scan-progress", new valuetype.ValueType("scan-progress", [])), [], [new valuetype.Property("done-count", new valuetype.ValueType("int", []))])],
        ])
    ],
    [
        "option",
        new Map<string, LeapVariant>([
            ["none", new valuetype.LeapVariant("option", new valuetype.Property("none", new valuetype.ValueType("none", [])), ["t"], [])],
            ["some", new valuetype.LeapVariant("option", new valuetype.Property("some", new valuetype.ValueType("some", [new valuetype.ValueType("t", [])])), ["t"], [new valuetype.Property("value", new valuetype.ValueType("t", []))])],
        ])
    ],
    [
        "result",
        new Map<string, LeapVariant>([
            ["ok", new valuetype.LeapVariant("result", new valuetype.Property("ok", new valuetype.ValueType("some", [new valuetype.ValueType("t", [])])), ["t", "e"], [new valuetype.Property("value", new valuetype.ValueType("t", []))])],
            ["err", new valuetype.LeapVariant("result", new valuetype.Property("err", new valuetype.ValueType("some", [new valuetype.ValueType("e", [])])), ["t", "e"], [new valuetype.Property("value", new valuetype.ValueType("e", []))])],
        ])
    ],
]);