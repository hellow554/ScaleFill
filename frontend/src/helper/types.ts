import { grams, kilograms, Mass } from "@buge/ts-units/mass";
import { Measurement } from "./exchange";

function isMeasurement(v: Mass | Measurement): v is Measurement {
    return (
        (v as Measurement).type !== undefined &&
        (v as Measurement).type === "measurement"
    );
}

export class Weight {
    value: Mass;

    constructor(mass: Mass);
    constructor(measurement: Measurement);

    constructor(value: Mass | Measurement) {
        if (isMeasurement(value)) {
            const num = value.value[0] / value.value[1];
            if (value.unit == "Gramm") {
                this.value = grams(num);
            } else if (value.unit == "Kilogramm") {
                this.value = kilograms(num);
            } else {
                throw `Can't use value {value}`;
            }
        } else {
            this.value = value;
        }
    }

    public toString(): string {
        if (this.value.unit == grams) {
            return `${this.value.amount} ${this.value.unit.symbol}`;
        } else if (this.value.unit == kilograms) {
            return `${this.value.amount.toFixed(2)} ${this.value.unit.symbol}`;
        } else {
            throw "def";
        }
    }
}
