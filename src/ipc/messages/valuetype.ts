import { LeapError } from './LeapError';

function getArgsMap(args: string[], appliedArgs: ValueType[]): Map<string, ValueType> {
    if (args.length !== appliedArgs.length) {
        throw new LeapError('incorrect number of applied type arguments');
    }
    let argsMap: Map<string, ValueType> = new Map<string, ValueType>();
    args.forEach((a, i) => argsMap.set(a, appliedArgs[i]));
    return argsMap;
}

export function normValueType(names: any): ValueType {
    if (names instanceof ValueType) {
        return names;
    }
    if (!(names instanceof Array)) {
        names = [names, []]
    }
    if (names.length === 1) {
        names = [names[0], []]
    }
    if (!(names[1] instanceof Array)) {
        names = [names[0], [names[1]]]
    }
    return new ValueType(names[0], names[1].map(normValueType));
}

export class ValueType {
    name: string;
    args: ValueType[];

    constructor(name: string, args: ValueType[]) {
        this.name = name;
        this.args = args;
    }

    toString(): string {
        if (this.args.length > 0) {
            var argsString = this.args.map(a => a.toString()).join(', ');
            return `${this.name}[${argsString}]`;
        }
        else {
            return `${this.name}`;
        }
    }

    clone(): ValueType {
        return new ValueType(this.name, this.args.map(a => a.clone()));
    }

    applyArgs(appliedArgs: ValueType[]): ValueType {
        const argsMap: Map<string, ValueType> = getArgsMap(this.args.map(a => a.name), appliedArgs);
        return this.applyArgsMap(argsMap);
    }

    applyArgsMap(argsMap: Map<string, ValueType>): ValueType {
        if (argsMap.has(this.name)) {
            return argsMap.get(this.name)!.clone();
        }
        else {
            return new ValueType(this.name, this.args.map(a => a.applyArgsMap(argsMap)));
        }
    }
}

export class LeapStruct {
    name: string;
    args: string[];
    props: Property[];

    constructor(name: string, args: string[], props: Property[]) {
        this.name = name;
        this.args = args;
        this.props = props;
    }

    applyArgs(appliedArgs: ValueType[]): LeapStruct {
        const argsMap: Map<string, ValueType> = getArgsMap(this.args, appliedArgs);
        let newProps: Property[] = [];
        for (const p of this.props) {
            newProps.push(p.applyArgsMap(argsMap));
        }
        return new LeapStruct(this.name, [], newProps);
    }
}

export class LeapEnum {
    name: string;
    args: string[];
    variants: Property[];

    constructor(name: string, args: string[], variants: Property[]) {
        this.name = name;
        this.args = args;
        this.variants = variants;
    }

    applyArgs(appliedArgs: ValueType[]): LeapEnum {
        const argsMap: Map<string, ValueType> = getArgsMap(this.args, appliedArgs);
        let newVariants: Property[] = [];
        for (const v of this.variants) {
            newVariants.push(v.applyArgsMap(argsMap));
        }
        return new LeapEnum(this.name, [], newVariants);
    }
}

export class LeapVariant {
    name: string;
    variant: Property;
    args: string[];
    props: Property[];

    constructor(name: string, variant: Property, args: string[], props: Property[]) {
        this.name = name;
        this.variant = variant;
        this.args = args;
        this.props = props;
    }

    applyArgs(appliedArgs: ValueType[]): LeapVariant {
        const argsMap: Map<string, ValueType> = getArgsMap(this.args, appliedArgs);
        let newProps: Property[] = [];
        for (const p of this.props) {
            newProps.push(p.applyArgsMap(argsMap))
        }
        const newVariant: Property = this.variant.applyArgsMap(argsMap);
        return new LeapVariant(this.name, newVariant, [], newProps);
    }
}

export class Property {
    name: string;
    propType: ValueType;

    constructor(name: string, propType: ValueType) {
        this.name = name;
        this.propType = propType;
    }

    applyArgsMap(argsMap: Map<string, ValueType>): Property {
        return new Property(this.name, this.propType.applyArgsMap(argsMap));
    }
}
