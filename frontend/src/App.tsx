import { useState } from "react";
import { useChat } from "./useChat";

export default function App() {
  const { messages, sendMessage } = useChat();
  const [input, setInput] = useState("");

  return (
    <div style={{ padding: 20 }}>
      <h2>Peerboard</h2>

      <div style={{ marginBottom: 12 }}>
        {messages.map((m, i) => (
          <div key={i}>
            <b>{m.nickname}</b>: {m.content}
          </div>
        ))}
      </div>

      <input value={input} onChange={(e) => setInput(e.target.value)} />

      <button
        onClick={() => {
          sendMessage(input);
          setInput("");
        }}
      >
        send
      </button>
    </div>
  );
}
