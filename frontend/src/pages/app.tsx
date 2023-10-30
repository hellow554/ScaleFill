import "bootstrap/dist/css/bootstrap.min.css";

import { createElement, useCallback, useEffect, useReducer } from "react";
import { Container, Tab, Tabs } from "react-bootstrap";
import useWebSocket from "react-use-websocket";
import { AppContext, SharedContext } from "../helper/appContext";
import { ToastContainer } from "react-toastify";

import Home from "./home";
import Settings from "./settings";
import Exchange from "../helper/exchange";
import { Weight } from "../helper/types";
import { kilograms } from "@buge/ts-units/mass";

export function SharedCtxInit(): SharedContext {
    return {
        lastWeight: new Weight(kilograms(0)),
        targetWeight: new Weight(kilograms(0)),

        fillInProgress: false,
    };
}

const AllTabs = [
    { name: "home", title: "Start", app: Home },
    { name: "settings", title: "Einstellungen", app: Settings },
];

function jsonReducer(state: SharedContext, action: Exchange): SharedContext {
    if (action.type == "measurement") {
        return { ...state, lastWeight: new Weight(action) };
    } else if (action.type == "error") {
        throw "def";
    } else {
        console.error(action);
        throw "abc";
    }
}

function App() {
    const initialConnection = useCallback(
        (_: unknown) =>
            sendJsonMessage<Exchange>({ type: "command", cmd: "hallo" }),
        [],
    );

    const [currentCtx, setCtx] = useReducer(jsonReducer, SharedCtxInit());
    useEffect(() => {
        if (lastJsonMessage !== null) setCtx(lastJsonMessage);
    }, [lastJsonMessage]);

    const defaultActiveKey = AllTabs[0].name;

    return (
        <AppContext.Provider value={currentCtx}>
            <ToastContainer position="top-right" autoClose={5000} />
            <Container fluid className="p-0">
                <Tabs
                    justify
                    fill
                    defaultActiveKey={defaultActiveKey}
                    className="mb-4"
                >
                    {AllTabs.map((t) => (
                        <Tab eventKey={t.name} title={t.title} key={t.name}>
                            <Container fluid className="px-4">
                                {createElement(t.app)}
                            </Container>
                        </Tab>
                    ))}
                </Tabs>
            </Container>
        </AppContext.Provider>
    );
}

export default App;
