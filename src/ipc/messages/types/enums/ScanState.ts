import type * as ldm from '../../leapdatamodel';
import * as valuetype from '../../valuetype';
import * as types from '..';
import { EnumBase as _EnumBase } from '../../EnumBase';
import { ValueBase as _ValueBase } from '../../ValueBase';
import { LeapError as _LeapError } from '../../LeapError';

export abstract class ScanState extends _EnumBase {
    static _type: string = 'scan-state';
    static _leapEnum: valuetype.LeapEnum = new valuetype.LeapEnum("scan-state", [], [new valuetype.Property("ready", new valuetype.ValueType("none", [])), new valuetype.Property("scan-progress", new valuetype.ValueType("scan-progress", []))]);
}
