import { useCallback, useContext } from "react";
import { Button, ToggleButton } from "react-bootstrap";
import { AppContext } from "../helper/appContext";

function Home() {
    const ctx = useContext(AppContext);
    const clickButton = useCallback(() => {}, []);



    return (
        <div className="d-flex flex-column gap-4 justify-content-between">
            <div className="text-center flex-grow-1 font-monospace">
                <h1 style={{ fontSize: "10rem" }}>
                    {ctx.lastWeight?.toString()}
                </h1>
            </div>

            <ToggleButton
                id="fillButton"
                value="1"
                checked={ctx.fillInProgress}
                variant="info"
                size="lg"
                style={{ height: "150px", fontSize: "5rem" }}
                onClick={clickButton}
            >
                {ctx.fillInProgress ? "Abbrechen" : "Start"}
            </ToggleButton>
        </div>
    );
}

export default Home;
