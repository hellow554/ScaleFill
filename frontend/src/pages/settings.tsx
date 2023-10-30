import { useState } from "react";
import {
    Accordion,
    Button,
    Dropdown,
    DropdownButton,
    FloatingLabel,
    Form,
    InputGroup,
    SplitButton,
    ToggleButton,
} from "react-bootstrap";

const ValidUnits = ["Gramm", "Kilogramm"];

function Settings() {
    const [unit, setUnit] = useState(ValidUnits[0]);
    const [permaOpen, setPermaOpen] = useState(false);

    return (
        <Accordion defaultActiveKey="0">
            <Accordion.Item eventKey="0">
                <Accordion.Header>Gewicht Einstellen</Accordion.Header>
                <Accordion.Body>
                    <InputGroup size="lg">
                        <FloatingLabel
                            controlId="floatingWeight"
                            label="Zielgewicht"
                        >
                            <Form.Control
                                type="number"
                                min={0}
                                step={0.01}
                                placeholder="xxx"
                            ></Form.Control>
                        </FloatingLabel>
                        <DropdownButton
                            variant="outline-secondary"
                            title={unit}
                            align="start"
                            style={{ width: "4rem" }}
                            onSelect={(e) => setUnit(e!)}
                        >
                            {ValidUnits.map((u) => (
                                <Dropdown.Item eventKey={u} key={u}>
                                    {u}
                                </Dropdown.Item>
                            ))}
                        </DropdownButton>
                    </InputGroup>
                </Accordion.Body>
            </Accordion.Item>
            <Accordion.Item eventKey="1">
                <Accordion.Header>Ventil Öffnen</Accordion.Header>
                <Accordion.Body>
                    <div className="d-flex flex-column">
                        <ToggleButton
                            style={{ height: "10rem", fontSize: "5rem" }}
                            size="lg"
                            id="valveOpen"
                            variant="outline-warning"
                            type="checkbox"
                            value="1"
                            checked={permaOpen}
                            onChange={(e) =>
                                setPermaOpen(e.currentTarget.checked)
                            }
                        >
                            Ventil{" "}
                            {permaOpen ? "schließen" : "dauerhaft öffnen"}
                        </ToggleButton>
                    </div>
                </Accordion.Body>
            </Accordion.Item>
        </Accordion>
    );
}

export default Settings;
