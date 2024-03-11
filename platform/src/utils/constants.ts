const SERVER_HOST = import.meta.env.VITE_APP_SERVER_HOST || "localhost";
const SERVER_PORT = import.meta.env.VITE_APP_SERVER_PORT || "4000";
const API_ROUTE = import.meta.env.VITE_APP_API_ROUTE || "/api/admin";
export const API_URL = `http://${SERVER_HOST}:${SERVER_PORT}${API_ROUTE}`;
export const SERVER_URL = `http://${SERVER_HOST}:${SERVER_PORT}`;