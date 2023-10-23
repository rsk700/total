import type { ValueType } from './valuetype';

export abstract class Value { }

export class StringValue extends Value {
    value: string;

    constructor(value: string) {
        super();
        this.value = value;
    }
}

export class IntegerValue extends Value {
    value: number;

    constructor(value: number) {
        super();
        this.value = value;
    }
}

export class FloatValue extends Value {
    value: number;

    constructor(value: number) {
        super();
        this.value = value;
    }
}

export class BooleanValue extends Value {
    value: boolean;

    constructor(value: boolean) {
        super();
        this.value = value;
    }
}

export class ListValue extends Value {
    value: Value[];
    valueType: ValueType;

    constructor(value: Value[], valueType: ValueType) {
        super();
        this.value = value;
        this.valueType = valueType;
    }
}

export class StructValue extends Value {
    value: Map<string, Value>;
    valueType: ValueType;

    constructor(value: Map<string, Value>, valueType: ValueType) {
        super();
        this.value = value;
        this.valueType = valueType;
    }
}

export class EnumValue extends Value {
    variant: string;
    value: StructValue;
    valueType: ValueType;

    constructor(variant: string, value: StructValue, valueType: ValueType) {
        super();
        this.variant = variant;
        this.value = value;
        this.valueType = valueType;
    }
}