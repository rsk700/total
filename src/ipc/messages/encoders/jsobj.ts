import type { Verify } from './encoders';
import { VerificationResult, VerificationOk, VerificationError, formatError } from './encoders';
import { ValueType } from '../valuetype';
import { LeapError } from '../LeapError';
import { ldmToValue } from './ldm';
import * as ldm from '../leapdatamodel';
import * as inspect from '../inspect';

export class FromJsobj<T> implements Verify {
    data: any;
    valueType: ValueType;

    constructor(data: any, valueType: ValueType) {
        this.data = data;
        this.valueType = valueType;
    }

    verify(upToFirstError: boolean): VerificationResult {
        if (upToFirstError) {
            for (let nextError of this._verify([], this.data, this.valueType)) {
                return new VerificationError([nextError]);
            }
        } else {
            let errors: string[] = [];
            for (let nextError of this._verify([], this.data, this.valueType)) {
                errors.push(nextError);
            }
            if (errors.length > 0) {
                return new VerificationError(errors);
            }
        }
        return new VerificationOk();
    }

    *_verify(path: string[], data: any, valueType: ValueType): Generator<string, void, unknown> {
        if (valueType.name === 'str') {
            if (typeof data !== 'string') {
                yield formatError(path, 'expecting string');
            }
        } else if (valueType.name === 'int') {
            if ((typeof data !== 'number') || (Math.trunc(data) !== data)) {
                yield formatError(path, 'expecting integer');
            }
        } else if (valueType.name === 'float') {
            if (typeof data === 'string') {
                if (data !== "+inf" && data !== "-inf" && data !== "nan") {
                    yield formatError(path, "expecting float");
                }
            } else if (typeof data !== 'number') {
                yield formatError(path, 'expecting float');
            }
        } else if (valueType.name === 'bool') {
            if (typeof data !== 'boolean') {
                yield formatError(path, 'expecting boolean');
            }
        } else if (valueType.name === 'list') {
            if (data instanceof Array) {
                yield* this._verifyList(path, data, valueType.args[0]);
            } else {
                yield formatError(path, 'expecting list');
            }
        } else if (valueType.name === 'option' && valueType.args[0].name !== 'option') {
            if (data !== null) {
                yield* this._verify(path, data, valueType.args[0]);
            }
        } else {
            if (typeof data === 'object') {
                if (inspect.leapStructsMap.has(valueType.name)) {
                    yield* this._verifyStruct(path, data, valueType);
                } else if (inspect.leapEnumsMap.has(valueType.name)) {
                    yield* this._verifyEnum(path, data, valueType);
                } else {
                    throw new LeapError('unexpected type');
                }
            } else {
                yield formatError(path, 'expecting object');
            }
        }
    }

    *_verifyList(path: string[], data: any[], elementType: ValueType): Generator<string, void, unknown> {
        for (let i = 0; i < data.length; i++) {
            yield* this._verify(path.concat([i.toString()]), data[i], elementType);
        }
    }

    *_verifyStruct(path: string[], data: any, valueType: ValueType): Generator<string, void, unknown> {
        const structType = inspect.leapStructsMap.get(valueType.name)!.applyArgs(valueType.args);
        for (let i = 0; i < structType.props.length; i++) {
            let prop = structType.props[i];
            if (typeof data[prop.name] === 'undefined') {
                yield formatError(path.concat([prop.name]), 'property is missing');
            } else {
                this._verify(path.concat([prop.name]), data[prop.name], prop.propType);
            }
        }
    }

    *_verifyEnum(path: string[], data: any, valueType: ValueType): Generator<string, void, unknown> {
        if (typeof data['_t'] === 'undefined') {
            yield formatError(path.concat(['_t']), 'variant name is missing');
            return;
        }
        if (typeof data['_t'] !== 'string') {
            yield formatError(path.concat(['_t']), 'expecting string with variant name');
            return;
        }
        if (inspect.leapVariantsMap.get(valueType.name)!.has(data['_t'])) {
            const variantType = inspect.leapVariantsMap.get(valueType.name)!.get(data['_t'])!.applyArgs(valueType.args);
            yield* this._verifyStruct(path, data, variantType.variant.propType);
        } else {
            yield formatError(path.concat(['_t']), 'unknown variant name');
        }
    }

    toValue(): T {
        const ldmValue = this.toLdm();
        return ldmToValue<T>(ldmValue);
    }

    toLdm(): ldm.Value {
        return this._toLdm(this.data, this.valueType);
    }

    _toLdm(data: any, valueType: ValueType): ldm.Value {
        if (valueType.name === 'str') {
            return new ldm.StringValue(data);
        } else if (valueType.name === 'int') {
            return new ldm.IntegerValue(data);
        } else if (valueType.name === 'float') {
            if (typeof data === 'string') {
                if (data === '+inf') {
                    return new ldm.FloatValue(Number.POSITIVE_INFINITY);
                } else if (data === '-inf') {
                    return new ldm.FloatValue(Number.NEGATIVE_INFINITY);
                } else if (data === 'nan') {
                    return new ldm.FloatValue(NaN);
                } else {
                    throw new LeapError('unexpected type');
                }
            } else if (typeof data === 'number') {
                return new ldm.FloatValue(data);
            } else {
                throw new LeapError('unexpected type');
            }
        } else if (valueType.name === 'bool') {
            return new ldm.BooleanValue(data);
        } else if (valueType.name === 'list') {
            return new ldm.ListValue(
                (data as Array<any>).map(v => this._toLdm(v, valueType.args[0])),
                valueType
            );
        } else if (valueType.name === 'option' && valueType.args[0].name !== 'option') {
            if (data === null) {
                return new ldm.EnumValue(
                    "none",
                    new ldm.StructValue(
                        new Map(),
                        new ValueType("none", [])
                    ),
                    valueType
                )
            } else {
                let props = new Map<string, ldm.Value>();
                props.set('value', this._toLdm(data, valueType.args[0]));
                return new ldm.EnumValue(
                    "some",
                    new ldm.StructValue(
                        props,
                        new ValueType("some", valueType.args)
                    ),
                    valueType
                );
            }
        } else if (inspect.leapStructsMap.has(valueType.name)) {
            return this._structToLdm(data, valueType);
        } else if (inspect.leapEnumsMap.has(valueType.name)) {
            return this._enumToLdm(data, valueType);
        } else {
            throw new LeapError('unexpected type');
        }
    }

    _structToLdm(data: any, valueType: ValueType): ldm.StructValue {
        const structType = inspect.leapStructsMap.get(valueType.name)!.applyArgs(valueType.args);
        let props = new Map<string, ldm.Value>();
        for (let i = 0; i < structType.props.length; i++) {
            let prop = structType.props[i];
            props.set(prop.name, this._toLdm(data[prop.name], prop.propType));
        }
        return new ldm.StructValue(props, valueType);
    }

    _enumToLdm(data: any, valueType: ValueType): ldm.EnumValue {
        const variantName = data['_t'];
        const variantType = inspect.leapVariantsMap.get(valueType.name)!.get(variantName)!.applyArgs(valueType.args);
        return new ldm.EnumValue(
            variantName,
            this._structToLdm(data, variantType.variant.propType),
            valueType
        );
    }
}

export class ToJsobj {

    static fromLdm(value: ldm.Value): any {
        if (value instanceof ldm.StringValue) {
            return value.value;
        } else if (value instanceof ldm.IntegerValue) {
            return value.value;
        } else if (value instanceof ldm.FloatValue) {
            return this._floatToJsobj(value.value);
        } else if (value instanceof ldm.BooleanValue) {
            return value.value;
        } else if (value instanceof ldm.ListValue) {
            return value.value.map(v => this.fromLdm(v));
        } else if (value instanceof ldm.StructValue) {
            return this._structToJsobj(value);
        } else if (value instanceof ldm.EnumValue) {
            if (value.valueType.name === 'option' && value.valueType.args[0].name !== 'option') {
                if (value.variant === 'none') {
                    return null;
                } else {
                    return this.fromLdm(value.value.value.get('value')!);
                }
            } else {
                return this._enumToJsobj(value);
            }
        }
        throw new LeapError('unexpected value type');
    }

    static _floatToJsobj(value: number): any {
        if (isFinite(value)) {
            return value;
        } else if (isNaN(value)) {
            return "nan";
        } else if (value === Number.POSITIVE_INFINITY) {
            return "+inf";
        } else if (value === Number.NEGATIVE_INFINITY) {
            return "-inf";
        }
    }

    static _structToJsobj(value: ldm.StructValue): any {
        let data: { [index: string]: any } = {};
        for (let [name, prop] of value.value) {
            data[name] = this.fromLdm(prop);
        };
        return data;
    }

    static _enumToJsobj(value: ldm.EnumValue): any {
        let data = this._structToJsobj(value.value);
        data['_t'] = value.variant;
        return data;
    }
}