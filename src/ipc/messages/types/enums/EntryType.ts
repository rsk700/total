import type * as ldm from '../../leapdatamodel';
import * as valuetype from '../../valuetype';
import * as types from '..';
import { EnumBase as _EnumBase } from '../../EnumBase';
import { ValueBase as _ValueBase } from '../../ValueBase';
import { LeapError as _LeapError } from '../../LeapError';

export abstract class EntryType extends _EnumBase {
    static _type: string = 'entry-type';
    static _leapEnum: valuetype.LeapEnum = new valuetype.LeapEnum("entry-type", [], [new valuetype.Property("directory", new valuetype.ValueType("none", [])), new valuetype.Property("file", new valuetype.ValueType("none", []))]);
}
