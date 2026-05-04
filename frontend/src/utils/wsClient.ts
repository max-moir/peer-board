import { WebSocketClient } from "./websocket";

export const wsClient = new WebSocketClient("ws://127.0.0.1:3001/ws", {
  onMessage: () => {},
  onOpen: () => {},
  onClose: () => {},
  onError: () => {},
});
