import { Admin } from '../hooks/useAuth';
import { createContext } from 'react';

export type AuthContextType = {
  admin: Admin | null;
  loading: boolean;
  isAuthenticated: boolean;
  signin: (email: string, password: string) => void;
  signout: () => void;
}

export const AuthContext = createContext<AuthContextType>({
  admin: null,
  loading: true,
  isAuthenticated: false,
  signin: () => {},
  signout: () => {},
});