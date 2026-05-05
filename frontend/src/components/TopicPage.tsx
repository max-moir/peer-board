import { useEffect, useState, useRef } from "react";
import { WebSocketClient } from "@/utils/websocket";
import { useParams } from "react-router-dom";
import { SidebarNavigationSectionsSubheadingsDemo } from "./Sidebar";

export default function TopicPage() {
  const { topicName } = useParams();
  const topic = topicName ?? "None";
  const [messages, setMessages] = useState<string[]>([]);
  const [input, setInput] = useState("");
  const [connected, setConnected] = useState(false);
  const wsClientRef = useRef<WebSocketClient | null>(null);

  useEffect(() => {
    const wsClient = new WebSocketClient("ws://127.0.0.1:3001/ws", {
      onOpen: () => wsInit(),
      onMessage: (msg) => setMessages((prev) => [...prev, msg]),
      onClose: () => setConnected(false),
      onError: () => setConnected(false),
    });
    wsClientRef.current = wsClient;

    return () => {
      wsClient.close();
    };
  }, []);

  const wsInit = () => {
    console.log("here");
    setConnected(true);
  };

  return (
    <div className="flex h-screen bg-background-dark">
      <SidebarNavigationSectionsSubheadingsDemo />

      <main className="flex-1 p-5 bg-primary text-quaternary overflow-auto">
        <h2 className="text-3xl font-bold text-primary"># {topicName}</h2>
        <p className="text-lg text-secondary">
          Status: {connected ? "Connected" : "Disconnected"}
        </p>

        <div className="bg-card-light shadow-md p-4">
          <h3 className="text-xl font-semibold text-secondary">Messages</h3>
          <div className="min-h-48 border border-gray-300 p-3">
            {messages.length === 0 ? (
              <div className="text-primary">No messages yet.</div>
            ) : (
              messages.map((msg, idx) => (
                <div key={idx} className="text-primary">
                  {msg}
                </div>
              ))
            )}
          </div>
        </div>
      </main>
    </div>
  );
}
