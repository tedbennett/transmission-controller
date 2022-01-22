import Row from "react-bootstrap/Row";
import Col from "react-bootstrap/Col";
import Form from "react-bootstrap/Form";
import Button from "react-bootstrap/Button";
import { useState } from "react";

const AddTorrent = () => {
  const [magnet, setManget] = useState("");

  const addTorrent = () => {
    fetch("/torrent", { method: "POST", body: JSON.stringify({ magnet }) });
  };

  const updateMagnet = (e) => setManget(e.target.value);

  return (
    <Col>
      <Row>
        <Form.Group className=" my-3" controlId="exampleForm.ControlTextarea1">
          <Form.Control
            as="textarea"
            rows={4}
            placeholder="Paste a magnet link here"
            style={{ backgroundColor: "#222529", color: "white" }}
            value={magnet}
            onChange={updateMagnet}
          />
        </Form.Group>
      </Row>
      <Row className="text-center  my-3">
        <Col>
          <Button variant="outline-secondary" onClick={addTorrent}>
            Add
          </Button>
        </Col>
      </Row>
    </Col>
  );
};

export default AddTorrent;
