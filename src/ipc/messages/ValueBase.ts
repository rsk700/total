import { ValueType } from './valuetype';
import { LeapError } from './LeapError';
import * as ldm from './leapdatamodel';

export abstract class ValueBase {
    abstract _verifyAsTypeOrRaise(path: string[], valueType: ValueType): void;
    abstract _toLdm(valueType: ValueType): ldm.Value;

    static _verifyOrRaise(value: any, valueType: ValueType): void {
        ValueBase._verifyValueOrRaise([], value, valueType);
    }

    static _verifyValueOrRaise(path: string[], value: any, valueType: ValueType): void {
        if (valueType.name == 'str') {
            if (!((typeof value === 'string') || (value instanceof String))) {
                throw ValueBase._pathError(path, 'expecting string');
            }
        } else if (valueType.name === 'int') {
            if (!((typeof value === 'number') || (value instanceof Number))) {
                throw ValueBase._pathError(path, 'expecting integer');
            }
        } else if (valueType.name === 'float') {
            if (!((typeof value === 'number') || (value instanceof Number))) {
                throw ValueBase._pathError(path, 'expecting float');
            }
        } else if (valueType.name === 'bool') {
            if (!((typeof value === 'boolean') || (value instanceof Boolean))) {
                throw ValueBase._pathError(path, 'expecting boolean');
            }
        } else if (valueType.name === 'list') {
            if (value instanceof Array) {
                ValueBase._verifyListOrRaise(path, value, valueType.args[0]);
            } else {
                throw ValueBase._pathError(path, 'expecting list');
            }
        } else if (valueType.name === 'option' && valueType.args[0].name !== 'option') {
            if (value !== null) {
                ValueBase._verifyValueOrRaise(path, value, valueType.args[0]);
            }
        } else if (value instanceof ValueBase) {
            value._verifyAsTypeOrRaise(path, valueType);
        }
        else {
            throw ValueBase._pathError(path, `expecting ${valueType}`);
        }
    }

    protected static _pathError(path: string[], message: string): LeapError {
        if (path.length === 0) {
            return new LeapError(message);
        } else {
            const p = path.join('.');
            return new LeapError(`${p}: ${message}`);
        }
    }

    static _verifyListOrRaise(path: string[], values: any[], elementType: ValueType): void {
        values.forEach((v, i) => ValueBase._verifyValueOrRaise(path.concat([i.toString()]), v, elementType));
    }

    static _valueToLdm(value: any, valueType: ValueType): ldm.Value {
        if (valueType.name === 'str') {
            return new ldm.StringValue(String(value));
        } else if (valueType.name === 'int') {
            return new ldm.IntegerValue(Math.trunc(value));
        } else if (valueType.name === 'float') {
            return new ldm.FloatValue(Number(value));
        } else if (valueType.name === 'bool') {
            return new ldm.BooleanValue(Boolean(value));
        } else if (valueType.name === 'list') {
            return new ldm.ListValue(
                (value as Array<any>).map(v => ValueBase._valueToLdm(v, valueType.args[0])),
                valueType
            );
        } else if (valueType.name === 'option' && valueType.args[0].name !== 'option') {
            if (value === null) {
                return new ldm.EnumValue(
                    'none',
                    new ldm.StructValue(new Map(), new ValueType('none', [])),
                    valueType
                );
            } else {
                let valuesMap = new Map();
                valuesMap.set('value', ValueBase._valueToLdm(value, valueType.args[0]));
                return new ldm.EnumValue(
                    'some',
                    new ldm.StructValue(valuesMap, valueType.args[0]),
                    valueType
                );
            }
        } else if (value instanceof ValueBase) {
            return value._toLdm(valueType);
        }
        throw new LeapError('unexpected value type');
    }
};