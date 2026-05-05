import { useEffect, useState, useRef } from "react";
import { WebSocketClient } from "@/utils/websocket";
import { SidebarNavigationSectionsSubheadingsDemo } from "./Sidebar";

type ChatMessage = {
  peer_id: string;
  nickname: string;
  content: string;
  timestamp: number;
  topic: string;
  message_id: string;
};

export default function Chat() {
  const [input, setInput] = useState("");
  const [nickname, setNickname] = useState("ma");
  const [localId, setLocalId] = useState<string>();
  const [activeTopic, setActiveTopic] = useState("general");
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [connected, setConnected] = useState(false);

  // Topics are now fully dynamic
  const [topics, setTopics] = useState<string[]>(["general", "random"]);
  const [newTopic, setNewTopic] = useState("");

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
    wsClientRef.current?.requestHistory();

    // Subscribe to all current topics
    topics.forEach((t) => {
      wsClientRef.current?.subscribe(t);
    });
  };

  const handleResponse = (msg: any) => {
    if (msg.type === "history_response") {
      setMessages(
        msg.messages.map((m: any) => ({
          peer_id: m.peer_id,
          nickname: m.nickname,
          content: m.content,
          timestamp: m.timestamp,
          topic: m.topic,
          message_id: m.message_id,
        })),
      );
    }

    if (msg.type === "local_id") {
      setLocalId(msg.id);
    }

    if (msg.type === "message") {
      setMessages((prev) => {
        const exists = prev.some((m) => m.message_id === msg.message_id);
        if (exists) return prev;
        return [...prev, msg];
      });
    }
  };

  const sendMessage = () => {
    if (!input.trim() || !wsClientRef.current) return;

    const localMessageId = `local-${Date.now()}`;

    const optimistic: ChatMessage = {
      peer_id: localId!,
      nickname,
      content: input.trim(),
      timestamp: Math.floor(Date.now() / 1000),
      topic: activeTopic,
      message_id: localMessageId,
    };

    setMessages((prev) => [...prev, optimistic]);
    wsClientRef.current.sendMessage(activeTopic, nickname, input.trim());
    setInput("");
  };

  const addTopic = () => {
    const topic = newTopic.trim();
    if (!topic) return;

    const valid = /^[a-z0-9-]+$/.test(topic);
    if (!valid) {
      alert("Topic must match [a-z0-9-]+");
      return;
    }

    if (!topics.includes(topic)) {
      setTopics((prev) => [...prev, topic]);
      wsClientRef.current?.subscribe(topic);
      setActiveTopic(topic);
    }

    setNewTopic("");
  };

  const removeTopic = (topic: string) => {
    if (topics.length === 1) return; // don't remove the last topic
    setTopics((prev) => prev.filter((t) => t !== topic));

    wsClientRef.current?.unsubscribe(topic);

    if (activeTopic === topic) {
      setActiveTopic(topics[0]); // switch to first topic
    }
  };

  const MessageItem = ({ msg }: { msg: ChatMessage }) => {
    const isSelf = msg.peer_id === localId;
    return (
      <div className={`flex flex-col ${isSelf ? "items-end" : "items-start"}`}>
        <div className="text-xs text-[var(--color-text-tertiary)] mb-1">
          #{msg.topic} • {msg.nickname} •{" "}
          {new Date(msg.timestamp * 1000).toLocaleTimeString()}
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
            Chat
          </h2>
          <p className="text-sm text-[var(--color-text-tertiary)]">
            {connected ? `Connected with local id ${localId}` : "Disconnected"}
          </p>
        </div>

        {/* Controls: nickname */}
        <div className="px-5 py-2 flex gap-2 border-b border-[var(--color-border-secondary)] shrink-0">
          <p className="text-sm text-[var(--color-text-tertiary)]">Nickname:</p>
          <input
            value={nickname}
            onChange={(e) => setNickname(e.target.value)}
            placeholder="nickname"
            className="px-2 py-1 text-xs rounded-md border border-[var(--color-border-primary)] bg-[var(--color-border-tertiary)] text-[var(--color-text-primary)]"
          />
        </div>

        {/* Topics display + add/remove */}
        <div className="px-5 py-2 flex flex-wrap gap-2 border-b border-[var(--color-border-secondary)] shrink-0">
          {topics.map((t) => (
            <div
              key={t}
              className="flex items-center gap-1 text-xs px-2 py-1 rounded-full bg-[var(--color-border-secondary)] text-[var(--color-text-secondary)]"
            >
              #{t}
              <button
                onClick={() => removeTopic(t)}
                className="text-[var(--color-text-tertiary)] hover:text-red-400"
              >
                ×
              </button>
            </div>
          ))}

          <input
            value={newTopic}
            onChange={(e) => setNewTopic(e.target.value)}
            placeholder="new-topic"
            className="px-2 py-1 text-xs rounded-md border border-[var(--color-border-primary)] bg-[var(--color-border-tertiary)] text-[var(--color-text-primary)]"
          />
          <button
            onClick={addTopic}
            className="px-2 py-1 text-xs rounded-md bg-[var(--color-fg-brand-primary)] text-[var(--color-text-primary_on-brand)] hover:bg-[var(--color-fg-brand-secondary_hover)]"
          >
            Add
          </button>
        </div>

        {/* Messages */}
        <div className="flex-1 overflow-y-auto px-5 py-4 space-y-4 bg-[var(--color-bg-secondary)] min-h-0">
          {messages.length === 0 ? (
            <div className="text-[var(--color-text-tertiary)]">
              No messages yet.
            </div>
          ) : (
            messages.map((msg) => (
              <MessageItem key={msg.message_id} msg={msg} />
            ))
          )}
          <div ref={bottomRef} />
        </div>

        {/* Input + topic selector */}
        <div className="p-4 border-t border-[var(--color-border-secondary)] bg-[var(--color-bg-primary)] flex gap-2 shrink-0">
          <input
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === "Enter") sendMessage();
            }}
            placeholder={`Message #${activeTopic}`}
            className="flex-1 px-3 py-2 rounded-md bg-[var(--color-border-tertiary)] text-[var(--color-text-primary)] placeholder:text-[var(--color-text-placeholder)] border border-[var(--color-border-primary)] focus:outline-none"
          />

          <button
            onClick={sendMessage}
            className="px-4 py-2 rounded-md bg-[var(--color-fg-brand-primary)] text-[var(--color-text-primary_on-brand)] hover:bg-[var(--color-fg-brand-secondary_hover)]"
          >
            Send
          </button>

          <select
            value={activeTopic}
            onChange={(e) => setActiveTopic(e.target.value)}
            className="px-2 py-1 text-xs rounded-md border border-[var(--color-border-primary)] bg-[var(--color-border-tertiary)] text-[var(--color-text-primary)]"
          >
            {topics.map((t) => (
              <option key={t} value={t}>
                #{t}
              </option>
            ))}
          </select>
        </div>
      </main>
    </div>
  );
}
