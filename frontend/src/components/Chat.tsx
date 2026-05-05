import { useEffect, useState, useRef } from "react";
import { WebSocketClient } from "@/utils/websocket";
import { SidebarNavigationSectionsSubheadingsDemo } from "./Sidebar";

type ChatMessage = {
  sender: string;
  content: string;
  timestamp: number;
};

export default function Chat() {
  const [input, setInput] = useState("");
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [connected, setConnected] = useState(false);

  const wsClientRef = useRef<WebSocketClient | null>(null);
  const bottomRef = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    const wsClient = new WebSocketClient("ws://127.0.0.1:3001/ws", {
      onOpen: () => handleStartup(),
      onMessage: (msg) => handleResponse(msg),
      onClose: () => setConnected(false),
      onError: () => setConnected(false),
    });

    wsClientRef.current = wsClient;

    return () => {
      wsClient.close();
    };
  }, []);

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

  const handleStartup = () => {
    setConnected(true);
    wsClientRef.current?.requestHistory("general");
  };

  const handleResponse = (msg: any) => {
    console.log(msg);

    if (msg.type === "history_response") {
      setMessages(
        msg.messages.map((m: any) => ({
          sender: m.sender,
          content: m.content,
          timestamp: m.timestamp,
        })),
      );
    }

    if (msg.type === "message") {
      setMessages((prev) => {
        const exists = prev.some(
          (m) =>
            m.content === msg.content &&
            m.timestamp === msg.timestamp &&
            m.sender === msg.sender,
        );

        if (exists) return prev;

        return [
          ...prev,
          {
            sender: msg.sender,
            content: msg.content,
            timestamp: msg.timestamp,
          },
        ];
      });
    }
  };

  const sendMessage = () => {
    if (input.trim() && wsClientRef.current) {
      const optimistic = {
        sender: "ma",
        content: input.trim(),
        timestamp: Math.floor(Date.now() / 1000),
      };

      // Optimistic UI update
      setMessages((prev) => [...prev, optimistic]);

      wsClientRef.current.sendMessage("general", "ma", input.trim());
      setInput("");
    }
  };

  const MessageItem = ({
    msg,
    isSelf,
  }: {
    msg: ChatMessage;
    isSelf: boolean;
  }) => {
    return (
      <div className={`flex flex-col ${isSelf ? "items-end" : "items-start"}`}>
        <div className="text-xs text-[var(--color-text-tertiary)] mb-1">
          {msg.sender} • {new Date(msg.timestamp * 1000).toLocaleTimeString()}
        </div>

        <div
          className={`
            px-3 py-2 rounded-lg max-w-[70%]
            ${
              isSelf
                ? "bg-[var(--color-fg-brand-primary)] text-[var(--color-text-primary_on-brand)]"
                : "bg-[var(--color-border-secondary)] text-[var(--color-text-primary)]"
            }
          `}
        >
          {msg.content}
        </div>
      </div>
    );
  };

  return (
    <div className="flex h-screen overflow-hidden bg-[var(--color-bg-primary)]">
      <SidebarNavigationSectionsSubheadingsDemo />

      <main className="flex-1 flex flex-col overflow-hidden min-h-0">
        {/* Header */}
        <div className="px-5 py-3 border-b border-[var(--color-border-secondary)] shrink-0">
          <h2 className="text-xl font-semibold text-[var(--color-text-primary)]">
            # General
          </h2>
          <p className="text-sm text-[var(--color-text-tertiary)]">
            {connected ? "Connected" : "Disconnected"}
          </p>
        </div>

        {/* Messages */}
        <div className="flex-1 overflow-y-auto px-5 py-4 space-y-4 bg-[var(--color-bg-secondary)] min-h-0">
          {messages.length === 0 ? (
            <div className="text-[var(--color-text-tertiary)]">
              No messages yet.
            </div>
          ) : (
            messages.map((msg, idx) => (
              <MessageItem key={idx} msg={msg} isSelf={msg.sender === "ma"} />
            ))
          )}
          <div ref={bottomRef} />
        </div>

        {/* Input */}
        <div className="p-4 border-t border-[var(--color-border-secondary)] bg-[var(--color-bg-primary)] flex gap-2 shrink-0">
          <input
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === "Enter") sendMessage();
            }}
            placeholder="Type a message..."
            className="
              flex-1 px-3 py-2 rounded-md
              bg-[var(--color-border-tertiary)]
              text-[var(--color-text-primary)]
              placeholder:text-[var(--color-text-placeholder)]
              border border-[var(--color-border-primary)]
              focus:outline-none
            "
          />

          <button
            onClick={sendMessage}
            className="
              px-4 py-2 rounded-md
              bg-[var(--color-fg-brand-primary)]
              text-[var(--color-text-primary_on-brand)]
              hover:bg-[var(--color-fg-brand-secondary_hover)]
            "
          >
            Send
          </button>
        </div>
      </main>
    </div>
  );
}
