import useWebSocket from "react-use-websocket";
import Exchange from "./exchange";

export const WS_URL = "ws://127.0.0.1:12321";

export function CreateWebsocket() {
    const { sendJsonMessage, lastJsonMessage } = useWebSocket<Exchange>(
        WS_URL,
        {
            shouldReconnect: (_closeEvent) => true,
            share: true,
            onOpen: initialConnection,
        },
    );
}
