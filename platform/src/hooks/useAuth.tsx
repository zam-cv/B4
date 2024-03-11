import axios from "axios";
import { API_URL } from "../utils/constants";
import { useContext, useEffect, useState } from "react";
import { AuthContext, AuthContextType } from "../contexts/AuthContext";

export interface Admin {
  email: string;
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
      .then((_) => {
        setIsAuthenticated(true);
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
      })
      .catch((error) => {
        console.error(error);
      });
  };

  useEffect(() => {
    axios
      .get(`${API_URL}/auth`, { withCredentials: true })
      .then(({ data: Admin }) => {
        setIsAuthenticated(true);
        setAdmin(Admin);
        setLoading(false);
      })
      .catch((_) => {
        setIsAuthenticated(false);
        setLoading(false);
      });

    console.log("Auth provider mounted");
  }, []);

  return {
    admin,
    loading,
    isAuthenticated,
    signin,
    signout,
  };
}
