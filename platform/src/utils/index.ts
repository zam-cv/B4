import { setHost } from "./constants";

export const handleKeyDown = (
  e: React.KeyboardEvent<HTMLInputElement>,
  ref: React.RefObject<HTMLInputElement>
) => {
  if (e.key === "Enter") {
    e.preventDefault();
    ref.current?.focus();
  }
};

export const handleEnter = (
  e: React.KeyboardEvent<HTMLInputElement>,
  callback: Function
) => {
  if (e.key === "Enter") {
    e.preventDefault();
    callback();
  }
};

export const setInStorage = (key: string, value: string) => {
  localStorage.setItem(key, value);
}

export const getInStorage = (key: string): null | string => {
  return localStorage.getItem(key);
}

export const removeInStorage = (key: string) => {
  localStorage.removeItem(key);
}

export const initConfig = () => {
  const API_URL = getInStorage("API_URL");

  if (API_URL) {
    setHost(API_URL);
  }
}