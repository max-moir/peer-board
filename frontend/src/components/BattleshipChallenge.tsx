import { useEffect, useState, useRef } from "react";
import { WebSocketClient } from "@/utils/websocket";
import { SidebarNavigationSectionsSubheadingsDemo } from "./Sidebar";

type Peer = {
  peer_id: string;
  nickname: string;
};

export default function BattleshipPage() {
  const [nickname, setNickname] = useState("ma");
  const [localId, setLocalId] = useState<string>();
  const [connected, setConnected] = useState(false);

  // Matchmaking state
  const [peers, setPeers] = useState<Peer[]>([]);
  const [available, setAvailable] = useState(false); // Am I advertising myself?
  const [inGame, setInGame] = useState(false); // Flag for active game

  const wsClientRef = useRef<WebSocketClient | null>(null);

  useEffect(() => {
    const wsClient = new WebSocketClient("ws://127.0.0.1:3001/ws", {
      onOpen: () => setConnected(true),
      onMessage: handleResponse,
      onClose: () => setConnected(false),
      onError: () => setConnected(false),
    });

    wsClientRef.current = wsClient;

    return () => wsClient.close();
  }, []);

  const handleResponse = (msg: any) => {
    if (msg.type === "local_id") setLocalId(msg.id);

    // Peer discovery response
    if (msg.type === "peer_list") {
      setPeers(msg.peers);
    }

    // Game started -> unregister automatically
    if (msg.type === "game_started") {
      setInGame(true);
      setAvailable(false);
      wsClientRef.current?.unregisterRendezvous(); // stop advertising
    }

    // Game ended -> can advertise again
    if (msg.type === "game_ended") {
      setInGame(false);
    }
  };

  // Register/unregister from rendezvous
  const toggleAvailability = () => {
    if (!wsClientRef.current) return;

    if (!available) {
      wsClientRef.current.registerRendezvous(nickname);
      setAvailable(true);
    } else {
      wsClientRef.current.unregisterRendezvous();
      setAvailable(false);
    }
  };

  // Challenge a peer
  const challengePeer = (peer: Peer) => {
    if (!wsClientRef.current || inGame) return;
    wsClientRef.current.sendChallenge(peer.peer_id);
  };

  return (
    <div className="flex h-screen overflow-hidden bg-[var(--color-bg-primary)]">
      <SidebarNavigationSectionsSubheadingsDemo />

      <main className="flex-1 flex flex-col overflow-hidden min-h-0">
        {/* Header */}
        <div className="px-5 py-3 border-b border-[var(--color-border-secondary)] shrink-0">
          <h2 className="text-xl font-semibold text-[var(--color-text-primary)]">
            Battleship Matchmaking
          </h2>
          <p className="text-sm text-[var(--color-text-tertiary)]">
            {connected ? `Connected with local id ${localId}` : "Disconnected"}
          </p>
        </div>

        {/* Nickname + availability */}
        <div className="px-5 py-2 flex gap-2 border-b border-[var(--color-border-secondary)] shrink-0">
          <p className="text-sm text-[var(--color-text-tertiary)]">Nickname:</p>
          <input
            value={nickname}
            onChange={(e) => setNickname(e.target.value)}
            placeholder="nickname"
            className="px-2 py-1 text-xs rounded-md border border-[var(--color-border-primary)] bg-[var(--color-border-tertiary)] text-[var(--color-text-primary)]"
          />
          <button
            onClick={toggleAvailability}
            className={`px-3 py-1 text-xs rounded-md ${
              available ? "bg-red-500 text-white" : "bg-green-500 text-white"
            }`}
          >
            {available ? "Stop Seeking" : "Seek Opponent"}
          </button>
        </div>

        {/* Peer list */}
        <div className="px-5 py-2 flex flex-wrap gap-2 border-b border-[var(--color-border-secondary)] shrink-0">
          {peers.length === 0 ? (
            <p className="text-[var(--color-text-tertiary)]">
              No peers available
            </p>
          ) : (
            peers.map((p) => (
              <button
                key={p.peer_id}
                onClick={() => challengePeer(p)}
                className="px-2 py-1 text-xs rounded-md bg-[var(--color-fg-brand-primary)] text-[var(--color-text-primary_on-brand)] hover:bg-[var(--color-fg-brand-secondary_hover)]"
              >
                Challenge {p.nickname}
              </button>
            ))
          )}
        </div>
      </main>
    </div>
  );
}
