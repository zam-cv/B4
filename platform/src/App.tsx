import { useEffect, useState } from "react";
import { platform } from "@tauri-apps/plugin-os";
import { PlatformContext } from "./contexts/Platform";
import Layout from "./components/Layout";

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
      <Layout>
        <div>Main</div>
      </Layout>
    </PlatformContext.Provider>
  );
}

export default App;
