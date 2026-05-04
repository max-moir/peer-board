export interface WebSocketMessageHandler {
  onMessage: (message: string) => void;
  onOpen: () => void;
  onClose: () => void;
  onError: () => void;
}

export class WebSocketClient {
  private ws: WebSocket | null = null;
  private messageHandler: WebSocketMessageHandler;

  constructor(url: string, messageHandler: WebSocketMessageHandler) {
    this.ws = new WebSocket(url);
    this.messageHandler = messageHandler;

    this.ws.onopen = () => this.messageHandler.onOpen();
    this.ws.onmessage = (event) => this.messageHandler.onMessage(event.data);
    this.ws.onclose = () => this.messageHandler.onClose();
    this.ws.onerror = () => this.messageHandler.onError();
  }

  sendMessage(message: string) {
    if (this.ws?.readyState === WebSocket.OPEN) {
      this.ws.send(message);
    }
  }

  close() {
    this.ws?.close();
  }
}
