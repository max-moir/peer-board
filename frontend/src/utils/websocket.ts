export interface WebSocketMessageHandler {
  onMessage: (message: string) => void;
  onOpen: () => void;
  onClose: () => void;
  onError: () => void;
}

export type WSOutgoingMessage =
  | {
      type: "message";
      topic: string;
      payload: string;
    }
  | {
      type: "ping";
    };

export class WebSocketClient {
  private ws: WebSocket | null = null;
  private handler: WebSocketMessageHandler;

  constructor(url: string, handler: WebSocketMessageHandler) {
    this.handler = handler;
    this.connect(url);
  }

  private connect(url: string) {
    this.ws = new WebSocket(url);

    this.ws.onopen = () => this.handler.onOpen();

    this.ws.onmessage = (event) => {
      this.handler.onMessage(event.data);
    };

    this.ws.onclose = () => this.handler.onClose();

    this.ws.onerror = () => this.handler.onError();
  }

  sendMessage(message: string) {
    this.send({
      type: "message",
      topic: "general",
      payload: message,
    });
  }

  send(data: WSOutgoingMessage) {
    if (this.ws?.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify(data));
    }
  }

  close() {
    this.ws?.close();
    this.ws = null;
  }

  reconnect(url: string) {
    this.close();
    this.connect(url);
  }
}
