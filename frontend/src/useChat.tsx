import { useEffect, useRef, useState } from "react";

type ServerMessage = {
  peer: string;
  nickname: string;
  content: string;
};

export function useChat() {
  const ws = useRef<WebSocket | null>(null);
  const [messages, setMessages] = useState<ServerMessage[]>([]);

  useEffect(() => {
    ws.current = new WebSocket("ws://localhost:3000/ws");

    ws.current.onmessage = (event) => {
      const msg: ServerMessage = JSON.parse(event.data);
      setMessages((prev) => [...prev, msg]);
    };

    return () => ws.current?.close();
  }, []);

  const sendMessage = (content: string) => {
    ws.current?.send(
      JSON.stringify({
        type: "chat",
        content,
        nickname: "ma",
      }),
    );
  };

  return { messages, sendMessage };
}
