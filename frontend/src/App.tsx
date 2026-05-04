import { useEffect, useRef, useState } from "react";
import { SidebarNavigationSectionDividers } from "@/components/application/app-navigation/sidebar-navigation/sidebar-section-dividers";
import { navItemsWithDividers } from "@/components/page";

export default function App() {
  const [input, setInput] = useState("");
  const [messages, setMessages] = useState<string[]>([]);
  const [connected, setConnected] = useState(false);
  const wsRef = useRef<WebSocket | null>(null);

  useEffect(() => {
    const ws = new WebSocket("ws://127.0.0.1:3001/ws");
    wsRef.current = ws;

    ws.onopen = () => setConnected(true);
    ws.onmessage = (event) => {
      setMessages((prev) => [...prev, event.data]);
    };
    ws.onclose = () => setConnected(false);
    ws.onerror = () => setConnected(false);

    return () => {
      ws.close();
    };
  }, []);

  const sendMessage = () => {
    const ws = wsRef.current;
    if (!ws || ws.readyState !== WebSocket.OPEN || !input.trim()) {
      return;
    }

    ws.send(input.trim());
    setInput("");
  };

  return (
    <div className="flex h-screen">
      <SidebarNavigationSectionDividers
        activeUrl="/"
        items={navItemsWithDividers}
      />
      <main className="flex-1 p-5 overflow-auto">
        <h2>Peerboard</h2>
        <p>Status: {connected ? "Connected" : "Disconnected"}</p>

        <div className="mb-4">
          <input
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === "Enter") {
                sendMessage();
              }
            }}
            className="w-3/4 mr-2 p-2 border rounded"
            placeholder="Type a message..."
          />
          <button
            onClick={sendMessage}
            className="p-2 bg-blue-500 text-white rounded"
          >
            Send
          </button>
        </div>

        <div>
          <h3>Messages</h3>
          <div className="min-h-48 border border-gray-300 p-3">
            {messages.length === 0 ? (
              <div className="text-gray-500">No messages yet.</div>
            ) : (
              messages.map((msg, idx) => (
                <div key={idx} className="mb-2">
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
