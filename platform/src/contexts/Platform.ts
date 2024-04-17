import { createContext } from 'react';

export type PlatformContextType = {
  platform: string | null;
}

export const PlatformContext = createContext<PlatformContextType>({
  platform: null,
});