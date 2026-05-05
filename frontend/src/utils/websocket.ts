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
    }
  | {
      type: "local_id_req";
    }
  | {
      type: "discover";
    }
  | {
      type: "register_for_game";
      nickname: string;
    }
  | {
      type: "unregister_for_game";
    }
  | {
      type: "send_challenge";
      peer_id: string;
    }
  | {
      type: "respond_challenge";
      peer_id: string;
      accepted: boolean;
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
    }
  | {
      type: "discover_response";
      peers: String[];
    }
  | {
      type: "challenge_propose";
      from_peer_id: string;
      nickname: string;
    }
  | {
      type: "challenge_response";
      from_peer_id: string;
      accepted: boolean;
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

    this.ws.onopen = () => this.handler.onOpen();

    this.ws.onmessage = (event) => {
      try {
        const data: WsOutgoing = JSON.parse(event.data);
        this.handler.onMessage(data);
      } catch (err) {
        console.error("Invalid WS message:", event.data);
      }
    };

    this.ws.onclose = () => this.handler.onClose();
    this.ws.onerror = () => this.handler.onError();
  }

  sendMessage(topic: string, nickname: string, content: string) {
    this.send({ type: "send_message", topic, nickname, content });
  }

  subscribe(topic: string) {
    this.send({ type: "subscribe_topic", topic });
  }

  requestLocalId() {
    this.send({ type: "local_id_req" });
  }

  unsubscribe(topic: string) {
    this.send({ type: "unsubscribe_topic", topic });
  }

  requestHistory() {
    this.send({ type: "history" });
  }

  discover() {
    console.log("discover");
    this.send({ type: "discover" });
  }

  registerForGame(nickname: string) {
    console.log("register");
    this.send({ type: "register_for_game", nickname });
  }

  unregisterRendezvous() {
    this.send({ type: "unregister_for_game" });
    console.log("uregister");
  }

  sendChallenge(peer_id: string) {
    this.send({ type: "send_challenge", peer_id: peer_id });
  }

  respondChallenge(peer_id: string, accepted: boolean) {
    this.send({ type: "respond_challenge", peer_id, accepted });
  }

  // --- Internal send helper ---
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
