import axios from "axios";
import { API_URL } from "../utils/constants";
import { useContext, useEffect, useState } from "react";
import { AuthContext, AuthContextType } from "../contexts/AuthContext";
import { invoke } from "@tauri-apps/api/core";

export interface Admin {
  email: string;
}

export interface Info {
  token: string;
  admin: Admin;
}

export async function getToken(): Promise<string | null> {
  try {
    return await invoke("get_token", {});
  } catch (_) {
    return null;
  }
}

export async function setToken(token: string) {
  try {
    await invoke("set_token", { token });
  } catch (_) {}
}

export async function deleteToken() {
  try {
    await invoke("delete_token", {});
  } catch (_) {}
}

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const auth = useProvideAuth();
  return <AuthContext.Provider value={auth}>{children}</AuthContext.Provider>;
}

export function useAuth(): AuthContextType {
  return useContext(AuthContext);
}

export function useProvideAuth() {
  const [admin, setAdmin] = useState<Admin | null>(null);
  const [loading, setLoading] = useState(true);
  const [isAuthenticated, setIsAuthenticated] = useState(false);

  const signin = (email: string, password: string) => {
    setLoading(true);

    axios
      .post(
        `${API_URL}/auth/signin`,
        {
          email,
          password,
        },
        { withCredentials: true }
      )
      .then(({ data }: { data: Info }) => {
        setIsAuthenticated(true);
        setAdmin(data.admin);
        setToken(data.token);
        setLoading(false);
      })
      .catch((error) => {
        console.error(error);
        setLoading(false);
      });
  };

  const signout = () => {
    axios
      .delete(`${API_URL}/auth/signout`, { withCredentials: true })
      .then((_) => {
        setIsAuthenticated(false);
        setAdmin(null);
        deleteToken();
      })
      .catch((error) => {
        console.error(error);
      });
  };

  useEffect(() => {
    (async () => {
      axios
        .get(`${API_URL}/auth`, {
          withCredentials: true,
          headers: {
            token: await getToken(),
          },
        })
        .then(({ data }: { data: Admin }) => {
          setIsAuthenticated(true);
          setAdmin(data);
          setLoading(false);
        })
        .catch((_) => {
          setIsAuthenticated(false);
          setLoading(false);
        });
    })();
  }, []);

  return {
    admin,
    loading,
    isAuthenticated,
    signin,
    signout,
  };
}
