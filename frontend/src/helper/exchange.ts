export interface Measurement {
    type: "measurement";
    value: [number, number];
    unit: "Newton" | "Kilogramm" | "Gramm" | "Pounds" | "Pieces";
}

export interface Command {
    type: "command";
    cmd: string;
}

export interface Error {
    type: "error";
    what: string;
}

type Exchange = Measurement | Command | Error;

export default Exchange;
