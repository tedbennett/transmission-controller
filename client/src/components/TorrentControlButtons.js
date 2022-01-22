import Button from "react-bootstrap/Button";
import Col from "react-bootstrap/Col";

const TorrentControlButtons = () => {
  const startAll = () => {
    fetch("/torrents/start", {method: "PUT"});
  };

  const stopAll = () => {
    fetch("/torrents/stop", {method: "PUT"});
  };

  return (
    <Col className="text-center">
      <Button variant="outline-secondary" className="m-1 my-3" onClick={startAll}>
        Start All
      </Button>
      <Button variant="outline-secondary" className="m-1 my-3" onClick={stopAll}>
        Stop All
      </Button>
    </Col>
  );
};

export default TorrentControlButtons;
