import { useEffect, useState } from "react";
import Stack from "react-bootstrap/Stack";
import Col from "react-bootstrap/Col";
import Row from "react-bootstrap/Row";

const VpnStatus = () => {
  const [isConnected, setIsConnected] = useState(false);

  const checkVpnStatus = () => {
    fetch("/vpn")
      .then((response) => {
        return response.json();
      })
      .then((data) => {
        setIsConnected(data.is_connected);
      });
  };

  useEffect(() => {
    checkVpnStatus();
    const interval = setInterval(() => {
      checkVpnStatus();
    }, 5000);
    return () => clearInterval(interval);
  }, []);

  return (
    <Stack
      direction="horizontal"
      className="mx-auto mb-3 justify-content-md-center"
      gap={3}
    >
      <div
        style={{
          width: "10px",
          height: "10px",
          borderRadius: "5px",
          backgroundColor: isConnected ? "green" : "orange",
        }}
      ></div>
      <div>{isConnected ? "VPN Connected" : "VPN Not Connected"}</div>
    </Stack>
  );
};

export default VpnStatus;
