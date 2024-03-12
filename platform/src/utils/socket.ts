import * as WebSocketTauri from "@tauri-apps/plugin-websocket";

export default function Socket(
  plataform: string,
  url: string,
  event: (message: string) => void,
  token?: string
) {
  if (plataform === "browser") {
    const socket = new WebSocket(url);
    socket.onmessage = (message) => {
      event(message.data);
    };
  } else if (token) {
    WebSocketTauri.default
      .connect(url, {
        headers: {
          token,
        },
      })
      .then((ws) => {
        ws.addListener((message) => {
          event(message.data?.toString() || "");
        });
      });
  }
}
