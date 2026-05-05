export type WsIncoming =
  | {
      type: "send_message";
      topic: string;
      nickname: string;
      content: string;
    }
  | {
      type: "subscribe_topic";
      topic: string;
    }
  | {
      type: "unsubscribe_topic";
      topic: string;
    }
  | {
      type: "history";
    };

export type WsOutgoing =
  | {
      type: "message";
      topic: string;
      sender: string;
      content: string;
      timestamp: number;
    }
  | {
      type: "history";
      topic: string;
      messages: WsMessage[];
    }
  | {
      type: "local_id";
      id: string;
    }
  | {
      type: "error";
      message: string;
    };

export type WsMessage = {
  topic: string;
  sender: string;
  content: string;
  timestamp: number;
};

export interface WebSocketMessageHandler {
  onMessage: (message: WsOutgoing) => void;
  onOpen: () => void;
  onClose: () => void;
  onError: () => void;
}

export class WebSocketClient {
  private ws: WebSocket | null = null;
  private handler: WebSocketMessageHandler;

  constructor(url: string, handler: WebSocketMessageHandler) {
    this.handler = handler;
    this.connect(url);
  }

  private connect(url: string) {
    this.ws = new WebSocket(url);

    this.ws.onopen = () => {
      this.handler.onOpen();
    };

    this.ws.onmessage = (event) => {
      try {
        const data: WsOutgoing = JSON.parse(event.data);
        this.handler.onMessage(data);
      } catch (err) {
        console.error("Invalid WS message:", event.data);
      }
    };

    this.ws.onclose = () => {
      this.handler.onClose();
    };

    this.ws.onerror = () => {
      this.handler.onError();
    };
  }

  sendMessage(topic: string, nickname: string, content: string) {
    this.send({
      type: "send_message",
      topic,
      nickname,
      content,
    });
  }

  subscribe(topic: string) {
    this.send({
      type: "subscribe_topic",
      topic,
    });
  }

  unsubscribe(topic: string) {
    this.send({
      type: "unsubscribe_topic",
      topic,
    });
  }

  requestHistory() {
    this.send({
      type: "history",
    });
  }

  private send(data: WsIncoming) {
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
