import { API_URL } from "./constants";
import { getConfig } from "./auth";
import axios from "axios";

export async function get<T>(path: string) {
  const config = await getConfig();
  return axios
    .get(`${API_URL}${path}`, config)
    .then(({ data }: { data: T }) => data);
}

export async function post<T, B>(path: string, body: B) {
  const config = await getConfig();
  return axios
    .post(`${API_URL}${path}`, body, config)
    .then(({ data }: { data: T }) => data);
}

export async function del<T>(path: string) {
  const config = await getConfig();
  return axios
    .delete(`${API_URL}${path}`, config)
    .then(({ data }: { data: T }) => data);
}

export async function put<T, B>(path: string, body: B) {
  const config = await getConfig();
  return axios
    .put(`${API_URL}${path}`, body, config)
    .then(({ data }: { data: T }) => data);
}