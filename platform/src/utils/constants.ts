const SERVER_HOST = import.meta.env.VITE_APP_SERVER_HOST || "localhost";
const SERVER_PORT = import.meta.env.VITE_APP_SERVER_PORT || "4000";
export const SERVER_URL = `http://${SERVER_HOST}:${SERVER_PORT}`;