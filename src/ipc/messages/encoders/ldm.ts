import type { ValueBase } from '../ValueBase';
import { LeapError } from '../LeapError';
import * as ldm from '../leapdatamodel';
import * as types from '../types';

const structMapFromLdm = new Map<string, (d: ldm.StructValue) => ValueBase>([
    [
        'entry',
        function(d: ldm.StructValue): ValueBase {
            return new types.structs.Entry(
                ldmToValue(d.value.get('id')!),
                ldmToValue(d.value.get('name')!),
                ldmToValue(d.value.get('path')!),
                ldmToValue(d.value.get('self-size')!),
                ldmToValue(d.value.get('size')!),
                ldmToValue(d.value.get('self-file-count')!),
                ldmToValue(d.value.get('file-count')!),
                ldmToValue(d.value.get('self-dir-count')!),
                ldmToValue(d.value.get('dir-count')!),
                ldmToValue(d.value.get('entry-type')!),
                ldmToValue(d.value.get('parent')!),
                ldmToValue(d.value.get('entries')!),
            );
        }
    ],
    [
        'scan-progress',
        function(d: ldm.StructValue): ValueBase {
            return new types.structs.ScanProgress(
                ldmToValue(d.value.get('done-count')!),
            );
        }
    ],
    [
        'none',
        function(d: ldm.StructValue): ValueBase {
            return new types.structs.None(
            );
        }
    ],
    [
        'some',
        function(d: ldm.StructValue): ValueBase {
            return new types.structs.Some(
                ldmToValue(d.value.get('value')!),
            );
        }
    ],
]);
const variantMapFromLdm = new Map<string, Map<string, (d: ldm.EnumValue) => ValueBase>>([
    [
        'entry-type',
        new Map<string, (d: ldm.EnumValue) => ValueBase>([
            [
                'directory',
                function(d: ldm.EnumValue): ValueBase {
                    return new types.variants.EntryType.Directory(
                    );
                }
            ],
            [
                'file',
                function(d: ldm.EnumValue): ValueBase {
                    return new types.variants.EntryType.File(
                    );
                }
            ],
        ])
    ],
    [
        'scan-state',
        new Map<string, (d: ldm.EnumValue) => ValueBase>([
            [
                'ready',
                function(d: ldm.EnumValue): ValueBase {
                    return new types.variants.ScanState.Ready(
                    );
                }
            ],
            [
                'scan-progress',
                function(d: ldm.EnumValue): ValueBase {
                    return new types.variants.ScanState.ScanProgress(
                        ldmToValue(d.value.value.get('done-count')!),
                    );
                }
            ],
        ])
    ],
    [
        'option',
        new Map<string, (d: ldm.EnumValue) => ValueBase>([
            [
                'none',
                function(d: ldm.EnumValue): ValueBase {
                    return new types.variants.Option.None(
                    );
                }
            ],
            [
                'some',
                function(d: ldm.EnumValue): ValueBase {
                    return new types.variants.Option.Some(
                        ldmToValue(d.value.value.get('value')!),
                    );
                }
            ],
        ])
    ],
    [
        'result',
        new Map<string, (d: ldm.EnumValue) => ValueBase>([
            [
                'ok',
                function(d: ldm.EnumValue): ValueBase {
                    return new types.variants.Result.Ok(
                        ldmToValue(d.value.value.get('value')!),
                    );
                }
            ],
            [
                'err',
                function(d: ldm.EnumValue): ValueBase {
                    return new types.variants.Result.Err(
                        ldmToValue(d.value.value.get('value')!),
                    );
                }
            ],
        ])
    ],
]);

export function ldmToValue<T>(data: ldm.Value): T {
    if (data instanceof ldm.StringValue) {
        return data.value as unknown as T;
    } else if (data instanceof ldm.IntegerValue) {
        return data.value as unknown as T;
    } else if (data instanceof ldm.FloatValue) {
        return data.value as unknown as T;
    } else if (data instanceof ldm.BooleanValue) {
        return data.value as unknown as T;
    } else if (data instanceof ldm.ListValue) {
        return data.value.map(v => ldmToValue(v)) as unknown as T;
    } else if (data instanceof ldm.StructValue) {
        return structMapFromLdm.get(data.valueType.name)!(data) as unknown as T;
    } else if (data instanceof ldm.EnumValue) {
        if (data.valueType.name === 'option' && data.valueType.args[0].name !== 'option') {
            if (data.variant === 'none') {
                // @ts-ignore
                return null;
            } else {
                // @ts-ignore
                return variantMapFromLdm.get(data.valueType.name)!.get(data.variant)!(data).value;
            }
        } else {
            return variantMapFromLdm.get(data.valueType.name)!.get(data.variant)!(data) as unknown as T;
        }
    }
    throw new LeapError('unexpected value type');
}
