import React from "react";
import Navbar from "./Components/global/Navbar";
import EndPointView from "./Components/EndPointView";
import DatabaseView from "./Components/DatabaseView";

function App() {
  const [activeWindow, setActiveWindow] = React.useState("dashboard");

  return (
    <>
      <Navbar 
        active={activeWindow} 
        onSelect={(selection) => { setActiveWindow(selection) }} />
      <div style={{ position: 'fixed', width: '100%', height: "100%", display: 'flex', flexDirection: 'row' }}>
        {activeWindow === "endpoints" && 
          <EndPointView />
        }
        {activeWindow === "database" && 
          <DatabaseView />
        }
      </div>
    </>
  );
}

export default App;
