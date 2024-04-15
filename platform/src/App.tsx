import { useEffect, useState } from "react";
import { platform } from "@tauri-apps/plugin-os";
import { PlatformContext } from "./contexts/Platform";
import { AuthProvider } from "./hooks/useAuth";
import { useAuth } from "./hooks/useAuth";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import { initChart } from "./utils/chart";
import Layout from "./components/Layout";
import { ADMIN_PERMISSIONS } from "@/utils/constants";

initChart();

// pages
import Dashboard from "./pages/Dashboard";
import Distribution from "./pages/Distribution";
import Users from "./pages/Users";
import Emails from "./pages/Emails";
import Accounts from "./pages/Accounts";
import Docs from "./pages/Docs";

export const pagePermissions = [
  {
    title: "Dashboard",
    route: "/dashboard",
    permission: ADMIN_PERMISSIONS.VIEW_DASHBOARD,
    component: Dashboard,
  },
  {
    title: "Usuarios",
    route: "/users",
    permission: ADMIN_PERMISSIONS.VIEW_USERS,
    component: Users,
  },
  {
    title: "Distribución",
    route: "/distribution",
    permission: ADMIN_PERMISSIONS.VIEW_DISTRIBUTION,
    component: Distribution,
  },
  {
    title: "Correos",
    route: "/emails",
    permission: ADMIN_PERMISSIONS.SEND_EMAILS,
    component: Emails,
  },
  {
    title: "Cuentas",
    route: "/accounts",
    permission: ADMIN_PERMISSIONS.VIEW_ACCOUNTS,
    component: Accounts,
  },
  {
    title: "Documentación",
    route: "/docs",
    permission: ADMIN_PERMISSIONS.VIEW_DOCUMENTS,
    component: Docs,
  },
];

function Pages() {
  const { permissions } = useAuth();

  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Layout />}>
          {permissions?.has(ADMIN_PERMISSIONS.VIEW_DASHBOARD) ? (
            <Route index path="/" element={<Dashboard />} />
          ) : null}
          {pagePermissions.map((page) => {
            return (
              <Route
                key={page.title}
                path={page.route}
                element={<page.component />}
              />
            );
          })}
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

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
        <Pages />
      </AuthProvider>
    </PlatformContext.Provider>
  );
}

export default App;
