type UnitCategory = "length" | "weight";

interface Unit {
    name: string;
    prefix: string;
    category: UnitCategory;
    scalar: number;
}

const units: Unit[] = [
    { name: "meter", prefix: "m", category: "length", scalar: 1 },
    { name: "centimeter", prefix: "cm", category: "length", scalar: 0.01 },
    { name: "kilometer", prefix: "km", category: "length", scalar: 1000 },
    { name: "kilogram", prefix: "kg", category: "weight", scalar: 1 },
    { name: "gram", prefix: "g", category: "weight", scalar: 0.001 },
];

function convert(value: number, from: string, to: string): number | undefined {
    const fromUnit = units.find((u) => u.prefix === from || u.name === from);

    const toUnit = units.find((u) => u.prefix === to || u.name === to);

    if (!fromUnit || !toUnit) return undefined;
    if (fromUnit.category !== toUnit.category) return undefined;

    const valueInBase = value * fromUnit.scalar;
    return valueInBase / toUnit.scalar;
}
