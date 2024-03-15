import { useEffect, useState } from "react";
import { platform } from "@tauri-apps/plugin-os";
import { PlatformContext } from "./contexts/Platform";
import { AuthProvider } from "./hooks/useAuth";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import { initChart } from "./utils/chart";
import Layout from "./components/Layout";

initChart();

// pages
import Dashboard from "./pages/Dashboard";
import Distribution from "./pages/Distribution";
import Emails from "./pages/Emails";

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
      <AuthProvider>
        <BrowserRouter>
          <Routes>
            <Route path="/" element={<Layout />}>
              <Route index path="/" element={<Dashboard />} />
              <Route path="/distribution" element={<Distribution />} />
              <Route path="/emails" element={<Emails />} />
              <Route path="/dashboard" element={<Dashboard />} />
            </Route>
          </Routes>
        </BrowserRouter>
      </AuthProvider>
    </PlatformContext.Provider>
  );
}

export default App;
