import TorrentList from "./components/TorrentList";
import AddTorrent from "./components/AddTorrent";
import Container from "react-bootstrap/Container";
import Row from "react-bootstrap/Row";
import Col from "react-bootstrap/Col";
import TorrentControlButtons from "./components/TorrentControlButtons";
import "./App.css";
import VpnStatus from "./components/VpnStatus";
const App = () => {
  return (
    <div className="App">
      <Container>
        <Col />
        <Col>
          <Row className="my-3">
            <h1 className="text-center">Torrents</h1>
          </Row>
          <Row>
            <VpnStatus />
          </Row>
          <Row>
            <TorrentList />
          </Row>
          <Row>
            <TorrentControlButtons />
          </Row>
          <Row>
            <AddTorrent />
          </Row>
        </Col>
        <Col />
      </Container>
    </div>
  );
};

export default App;
