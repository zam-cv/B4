import { invoke } from "@tauri-apps/api/core";

interface Config {
  withCredentials: boolean;
  headers: {
    token: string | null;
  };
}

export async function getToken(): Promise<string | null> {
  try {
    return await invoke("get_token", {});
  } catch (_) {
    const token = localStorage.getItem("token");
    return token;
  }
}

export async function setToken(token: string) {
  try {
    await invoke("set_token", { token });
  } catch (_) {
    localStorage.setItem("token", token);
  }
}

export async function deleteToken() {
  try {
    await invoke("delete_token", {});
  } catch (_) {
    localStorage.removeItem("token");
  }
}

export async function getConfig(): Promise<Config> {
  return {
    withCredentials: true,
    headers: {
      token: await getToken(),
    }
  }
}