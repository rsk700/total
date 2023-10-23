import type * as ldm from '../../leapdatamodel';
import * as valuetype from '../../valuetype';
import * as types from '..';
import { EnumBase as _EnumBase } from '../../EnumBase';
import { ValueBase as _ValueBase } from '../../ValueBase';
import { LeapError as _LeapError } from '../../LeapError';

export abstract class Option<T> extends _EnumBase {
    static _type: string = 'option';
    static _leapEnum: valuetype.LeapEnum = new valuetype.LeapEnum("option", ["t"], [new valuetype.Property("none", new valuetype.ValueType("none", [])), new valuetype.Property("some", new valuetype.ValueType("some", [new valuetype.ValueType("t", [])]))]);
}
