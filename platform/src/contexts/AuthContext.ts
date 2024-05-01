import { Admin } from '@/utils/api';
import { createContext } from 'react';

export type AuthContextType = {
  admin: Admin | null;
  loading: boolean;
  isAuthenticated: boolean;
  permissions: Set<string> | null;
  signin: (email: string, password: string, serverHost: string) => void;
  signout: () => void;
}

export const AuthContext = createContext<AuthContextType>({
  admin: null,
  loading: true,
  isAuthenticated: false,
  permissions: new Set(),
  signin: () => {},
  signout: () => {},
});