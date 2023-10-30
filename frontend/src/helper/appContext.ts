import { createContext } from "react";
import { Weight } from "./types";

export interface SharedContext {
    lastWeight: Weight;
    targetWeight: Weight;

    fillInProgress: boolean,
}

export const AppContext = createContext<SharedContext>(null!);

