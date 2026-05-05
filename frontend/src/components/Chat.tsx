import { useEffect, useState, useRef } from "react";
import { WebSocketClient } from "@/utils/websocket";
import { SidebarNavigationSectionsSubheadingsDemo } from "./Sidebar";

export default function Chat() {
  const [input, setInput] = useState("");
  const [messages, setMessages] = useState<string[]>([]);
  const [connected, setConnected] = useState(false);
  const wsClientRef = useRef<WebSocketClient | null>(null);

  useEffect(() => {
    const wsClient = new WebSocketClient("ws://127.0.0.1:3001/ws", {
      onOpen: () => handleStartup(),
      // onMessage: (msg) => setMessages((prev) => [...prev, msg.content]),
      onMessage: (msg) => handleResponse(msg),
      onClose: () => setConnected(false),
      onError: () => setConnected(false),
    });
    wsClientRef.current = wsClient;

    return () => {
      wsClient.close();
    };
  }, []);

  const handleStartup = () => {
    setConnected(true);
    wsClientRef.current.requestHistory("general");
  };

  const handleResponse = (msg) => {
    console.log(msg);
    if (msg.type == "message") {
      setMessages((prev) => [...prev, msg.content]);
    }
  };

  const sendMessage = () => {
    if (input.trim() && wsClientRef.current) {
      wsClientRef.current.sendMessage("general", "ma", input.trim());
      setInput("");
    }
  };

  return (
    <div className="flex h-screen bg-background-dark">
      <SidebarNavigationSectionsSubheadingsDemo />

      <main className="flex-1 p-5 bg-primary text-quaternary overflow-auto">
        <h2 className="text-2xl font-bold text-primary">General</h2>
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

        <div className="mb-4 flex gap-2 items-center">
          <input
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === "Enter") {
                sendMessage();
              }
            }}
            className="flex-1 p-2 border border-gray-300 rounded"
            placeholder="Type a message..."
          />
          <button
            onClick={sendMessage}
            className="p-2 bg-blue-500 text-white rounded"
          >
            Send
          </button>
        </div>
      </main>
    </div>
  );
}
