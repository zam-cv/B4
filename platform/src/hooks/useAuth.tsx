import { setHost, removeHost } from "../utils/constants";
import { useContext, useEffect, useState } from "react";
import { AuthContext, AuthContextType } from "../contexts/AuthContext";
import { setToken, deleteToken } from "../utils/auth";
import api, { Admin } from "@/utils/api";

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const auth = useProvideAuth();
  return <AuthContext.Provider value={auth}>{children}</AuthContext.Provider>;
}

export function useAuth(): AuthContextType {
  return useContext(AuthContext);
}

async function getPermissions(): Promise<Set<string>> {
  return new Set(await api.permissions.getPermissions());
}

export function useProvideAuth() {
  const [admin, setAdmin] = useState<Admin | null>(null);
  const [loading, setLoading] = useState(true);
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [permissions, setPermissions] = useState<Set<string> | null>(null);

  const signin = async (
    email: string,
    password: string,
    serverHost: string
  ) => {
    setLoading(true);

    if (serverHost != "") {
      setHost(serverHost);
    } else {
      removeHost();
    }

    api.auth
      .signin(email, password)
      .then((data) => {
        setIsAuthenticated(true);
        setAdmin(data.admin);
        setToken(data.token);
        getPermissions().then((permissions) => setPermissions(permissions));
        setLoading(false);
      })
      .catch((error) => {
        console.error(error);
        setLoading(false);
      });
  };

  const signout = async () => {
    api.auth
      .signout()
      .then((_) => {
        setIsAuthenticated(false);
        setAdmin(null);
        setPermissions(null);
        deleteToken();
        removeHost();
      })
      .catch((error) => {
        console.error(error);
        removeHost();
      });
  };

  useEffect(() => {
    api.auth
      .authenticate()
      .then((data) => {
        setIsAuthenticated(true);
        setAdmin(data);
        getPermissions().then((permissions) => setPermissions(permissions));
        setLoading(false);
      })
      .catch((_) => {
        setIsAuthenticated(false);
        setLoading(false);
        removeHost();
      });
  }, []);

  return {
    admin,
    loading,
    isAuthenticated,
    signin,
    signout,
    permissions,
  };
}
