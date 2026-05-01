import { useEffect, useRef, useState } from "react";

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
    <div style={{ padding: 20, maxWidth: 640 }}>
      <h2>Peerboard</h2>
      <p>Status: {connected ? "Connected" : "Disconnected"}</p>

      <div style={{ marginBottom: 16 }}>
        <input
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === "Enter") {
              sendMessage();
            }
          }}
          style={{ width: "70%", marginRight: 8 }}
          placeholder="Type a message..."
        />
        <button onClick={sendMessage}>Send</button>
      </div>

      <div>
        <h3>Messages</h3>
        <div style={{ minHeight: 200, border: "1px solid #ddd", padding: 12 }}>
          {messages.length === 0 ? (
            <div style={{ color: "#888" }}>No messages yet.</div>
          ) : (
            messages.map((msg, idx) => (
              <div key={idx} style={{ marginBottom: 8 }}>
                {msg}
              </div>
            ))
          )}
        </div>
      </div>
    </div>
  );
}
