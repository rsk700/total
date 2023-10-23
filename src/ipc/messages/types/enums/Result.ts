import type * as ldm from '../../leapdatamodel';
import * as valuetype from '../../valuetype';
import * as types from '..';
import { EnumBase as _EnumBase } from '../../EnumBase';
import { ValueBase as _ValueBase } from '../../ValueBase';
import { LeapError as _LeapError } from '../../LeapError';

export abstract class Result<T,E> extends _EnumBase {
    static _type: string = 'result';
    static _leapEnum: valuetype.LeapEnum = new valuetype.LeapEnum("result", ["t", "e"], [new valuetype.Property("ok", new valuetype.ValueType("some", [new valuetype.ValueType("t", [])])), new valuetype.Property("err", new valuetype.ValueType("some", [new valuetype.ValueType("e", [])]))]);
}
