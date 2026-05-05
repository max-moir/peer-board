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
  const [nickname, setNickname] = useState("Name");
  const [localId, setLocalId] = useState<string>();
  const [activeTopic, setActiveTopic] = useState("general");
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [connected, setConnected] = useState(false);

  const [topics, setTopics] = useState<string[]>(["general", "random"]);
  const [newTopic, setNewTopic] = useState("");
  const [filterByTopic, setFilterByTopic] = useState(false);

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

    return () => wsClient.close();
  }, []);

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages, activeTopic]);

  const handleStartup = () => {
    setConnected(true);
    wsClientRef.current?.requestHistory();

    topics.forEach((t) => wsClientRef.current?.subscribe(t));
  };

  const handleResponse = (msg: any) => {
    console.log(msg);

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
        if (prev.some((m) => m.message_id === msg.message_id)) return prev;
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

    if (!/^[a-z0-9-]+$/.test(topic)) {
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
    if (topics.length === 1) return;
    setTopics((prev) => prev.filter((t) => t !== topic));
    wsClientRef.current?.unsubscribe(topic);

    if (activeTopic === topic) setActiveTopic(topics[0]);
  };

  const MessageItem = ({ msg }: { msg: ChatMessage }) => {
    const isSelf = msg.peer_id === localId;
    return (
      <div className={`flex flex-col ${isSelf ? "items-end" : "items-start"}`}>
        <div className="text-xs text-[var(--color-text-tertiary)] mb-1">
          {filterByTopic ? "" : `#${msg.topic} • `} {msg.nickname} •{" "}
          {new Date(msg.timestamp * 1000).toLocaleTimeString()}
        </div>

        <div
          className={`px-3 py-2 rounded-lg max-w-[70%] ${
            isSelf
              ? "bg-[var(--color-fg-brand-primary)] text-[var(--color-text-primary_on-brand)]"
              : "bg-[var(--color-border-secondary)] text-[var(--color-text-primary)]"
          }`}
        >
          {msg.content}
        </div>
      </div>
    );
  };

  return (
    <div className="flex h-screen bg-[var(--color-bg-primary)]">
      <SidebarNavigationSectionsSubheadingsDemo />

      <main className="flex-1 flex flex-col overflow-hidden">
        <header className="px-6 py-4 border-b border-[var(--color-border-secondary)] shrink-0 flex justify-between items-center">
          <div>
            <h2 className="text-xl font-semibold text-[var(--color-text-primary)]">
              Bulletin Board
            </h2>
            <p className="text-sm text-[var(--color-text-tertiary)]">
              {connected ? `Connected as ${nickname}` : "Disconnected"}
            </p>
          </div>
          <div className="flex items-center gap-2">
            <p className="text-sm text-[var(--color-text-tertiary)]">
              Set nickname:
            </p>
            <input
              value={nickname}
              onChange={(e) => setNickname(e.target.value)}
              placeholder="Nickname"
              className="px-3 py-1 text-sm rounded-md border border-[var(--color-border-primary)] bg-[var(--color-border-tertiary)] text-[var(--color-text-primary)]"
            />
          </div>
        </header>

        <div className="px-6 py-3 border-b border-[var(--color-border-secondary)] flex flex-wrap items-center gap-2 bg-[var(--color-bg-secondary)]">
          <p className="text-sm text-[var(--color-text-tertiary)]">
            Select a topic to send messages in it:
          </p>

          {topics.map((t) => (
            <div
              key={t}
              className={`flex items-center gap-1 text-xs font-medium px-3 py-1 rounded-full ${
                activeTopic === t
                  ? "bg-[var(--color-fg-brand-primary)] text-[var(--color-text-primary_on-brand)]"
                  : "bg-[var(--color-border-secondary)] text-[var(--color-text-secondary)]"
              }`}
            >
              <button
                onClick={() => setActiveTopic(t)}
                className="outline-none"
              >
                #{t}
              </button>
              {topics.length > 1 && (
                <button
                  onClick={() => removeTopic(t)}
                  className="text-[var(--color-text-tertiary)] hover:text-red-400 ml-1"
                >
                  ×
                </button>
              )}
            </div>
          ))}

          <input
            value={newTopic}
            onChange={(e) => setNewTopic(e.target.value)}
            placeholder="Subscribe to topic"
            className="px-3 py-1 text-xs rounded-md border border-[var(--color-border-primary)] bg-[var(--color-border-tertiary)] text-[var(--color-text-primary)]"
          />
          <button
            onClick={addTopic}
            className="px-3 py-1 text-xs rounded-md bg-[var(--color-fg-brand-primary)] text-[var(--color-text-primary_on-brand)] hover:bg-[var(--color-fg-brand-secondary_hover)]"
          >
            Subscribe
          </button>

          <label className="ml-auto flex items-center gap-2 text-xs text-[var(--color-text-tertiary)]">
            <input
              type="checkbox"
              checked={filterByTopic}
              onChange={(e) => setFilterByTopic(e.target.checked)}
              className="w-4 h-4"
            />
            Filter by selected topic
          </label>
        </div>

        <div className="flex-1 overflow-y-auto px-6 py-4 space-y-4 bg-[var(--color-bg-secondary)]">
          {messages.filter((msg) => !filterByTopic || msg.topic === activeTopic)
            .length === 0 ? (
            <div className="text-[var(--color-text-tertiary)]">
              {filterByTopic
                ? `No messages in #${activeTopic}`
                : "No messages yet"}
            </div>
          ) : (
            messages
              .filter((msg) => !filterByTopic || msg.topic === activeTopic)
              .map((msg) => <MessageItem key={msg.message_id} msg={msg} />)
          )}
          <div ref={bottomRef} />
        </div>

        <div className="px-6 py-4 border-t border-[var(--color-border-secondary)] flex gap-3 bg-[var(--color-bg-primary)]">
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
        </div>
      </main>
    </div>
  );
}
``;
