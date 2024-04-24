import axios from "axios";
import { API_URL, setHost, removeHost } from "../utils/constants";
import { useContext, useEffect, useState } from "react";
import { AuthContext, AuthContextType } from "../contexts/AuthContext";
import { setToken, deleteToken } from "../utils/auth";
import { getConfig } from "../utils/auth";

export interface Admin {
  email: string;
}

export interface Info {
  token: string;
  admin: Admin;
}

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const auth = useProvideAuth();
  return <AuthContext.Provider value={auth}>{children}</AuthContext.Provider>;
}

export function useAuth(): AuthContextType {
  return useContext(AuthContext);
}

async function getPermissions(): Promise<Set<string>> {
  const config = await getConfig();

  return axios
    .get(`${API_URL}/permissions`, config)
    .then(({ data }: { data: string[] }) => new Set(data))
    .catch((_) => new Set());
}

export function useProvideAuth() {
  const [admin, setAdmin] = useState<Admin | null>(null);
  const [loading, setLoading] = useState(true);
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [permissions, setPermissions] = useState<Set<string> | null>(null);

  const signin = async (email: string, password: string, serverHost: string) => {
    setLoading(true);
    const config = await getConfig();

    if (serverHost != "") {
      setHost(serverHost);
    } else {
      removeHost();
    }

    axios
      .post(
        `${API_URL}/auth/signin`,
        {
          email,
          password,
        },
        config
      )
      .then(({ data }: { data: Info }) => {
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
    const config = await getConfig();

    axios
      .delete(`${API_URL}/auth/signout`, config)
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
    (async () => {
      const config = await getConfig();

      axios
        .get(`${API_URL}/auth`, config)
        .then(({ data }: { data: Admin }) => {
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
    })();
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
