import { useEffect, useState } from "react";
import { platform } from "@tauri-apps/plugin-os";
import { PlatformContext } from "./contexts/Platform";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import Layout from "./components/Layout";

// pages
import Login from "./pages/Login";
import Dashboard from "./pages/Dashboard";

function App() {
  const [platformName, setPlatform] = useState<string | null>(null);

  useEffect(() => {
    platform()
      .then((platformName) => {
        setPlatform(platformName);
      })
      .catch((_) => {
        setPlatform("browser");
      });
  }, [platform]);

  return (
    <PlatformContext.Provider value={{ platform: platformName }}>
      <BrowserRouter>
        <Routes>
          <Route path="/" element={<Layout />}>
            <Route index element={<Login />} />
            <Route path="dashboard" element={<Dashboard />} />
          </Route>
        </Routes>
      </BrowserRouter>
    </PlatformContext.Provider>
  );
}

export default App;
