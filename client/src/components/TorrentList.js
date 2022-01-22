import { useEffect, useState } from "react";
import Table from "react-bootstrap/Table";
import Form from "react-bootstrap/Form";

const TorrentList = () => {
  const [torrents, setTorrents] = useState([]);

  const fetchAndUpdateTorrents = () => {
    fetch("/torrents")
      .then((response) => {
        return response.json();
      })
      .then((data) => {
        setTorrents(data.torrents);
      });
  };

  useEffect(() => {
    fetchAndUpdateTorrents();
    const interval = setInterval(() => {
      fetchAndUpdateTorrents();
    }, 1000);
    return () => clearInterval(interval);
  }, []);

  const onSelectChange = (e, id) => {
    console.log(e.target);
    const destination = e.target.value.toLowerCase();
    fetch("/torrents/move", {
      method: "PUT",
      body: JSON.stringify({ id, destination }),
    });
  };

  return (
    <Table striped bordered hover variant="dark">
      <thead>
        <tr>
          <th>Name</th>
          <th>State</th>
          <th>Downloaded</th>
          <th>Size</th>
          <th>Percentage</th>
          <th>Down Speed</th>
          <th>Up Speed</th>
          <th>ETA</th>
          <th>Destination</th>
        </tr>
      </thead>
      <tbody>
        {torrents.map((t) => {
          return (
            <tr key={t.id}>
              <td>{t.name}</td>
              <td>{t.state}</td>
              <td>{t.downloaded}</td>
              <td>{t.size}</td>
              <td>{t.percent}</td>
              <td>{t.download_speed}</td>
              <td>{t.upload_speed}</td>
              <td>{t.eta}</td>
              <td>
                <Form.Select
                  size="sm"
                  style={{ backgroundColor: "#2d3034", color: "white" }}
                  onChange={(e) => onSelectChange(e, t.id)}
                >
                  <option>TV</option>
                  <option>Movies</option>
                </Form.Select>
              </td>
            </tr>
          );
        })}
      </tbody>
    </Table>
  );
};

export default TorrentList;
