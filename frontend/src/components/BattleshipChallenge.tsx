import { useEffect, useState, useRef } from "react";
import { WebSocketClient } from "@/utils/websocket";
import { SidebarNavigationSectionsSubheadingsDemo } from "./Sidebar";

export default function BattleshipPage() {
  const [nickname, setNickname] = useState("ma");
  const [localId, setLocalId] = useState<string>();
  const [connected, setConnected] = useState(false);

  // Matchmaking state
  const [peersSeeking, setPeersSeeking] = useState<string[]>([]);
  const [advertising, setAdvertising] = useState(false); // registered in rendezvous
  const [inGame, setInGame] = useState(false); // active game

  const wsClientRef = useRef<WebSocketClient | null>(null);

  // Connect websocket
  useEffect(() => {
    const wsClient = new WebSocketClient("ws://127.0.0.1:3001/ws", {
      onOpen: () => initWs(),
      onMessage: handleServerMessage,
      onClose: () => setConnected(false),
      onError: () => setConnected(false),
    });

    wsClientRef.current = wsClient;

    return () => wsClient.close();
  }, []);

  const initWs = () => {
    setConnected(true);
    wsClientRef.current?.requestLocalId();
  };

  // Handle messages from the server
  const handleServerMessage = (msg: any) => {
    console.log(msg);

    switch (msg.type) {
      case "local_id":
        setLocalId(msg.id);
        break;

      case "peers_seeking":
        break;

      case "discover_response":
        setPeersSeeking(msg.peers);
        break;

      case "game_started":
        setInGame(true);
        setAdvertising(false);
        wsClientRef.current?.unregisterRendezvous();
        break;

      case "game_ended":
        setInGame(false);
        break;

      case "challenge_propose":
        console.log(`Challenge from ${msg.from_peer_id}`);
        break;

      case "challenge_response":
        console.log(
          `Challenge response from ${msg.from_peer_id}: ${msg.accepted}`,
        );
        break;

      default:
        console.warn("Unhandled message type", msg.type);
    }
  };

  // Register/unregister from matchmaking (rendezvous)
  const toggleAdvertising = () => {
    if (!connected) return;
    if (!wsClientRef.current) return;

    if (!advertising) {
      wsClientRef.current.registerForGame(nickname);
      setAdvertising(true);
    } else {
      wsClientRef.current.unregisterRendezvous();
      setAdvertising(false);
    }
  };

  const discoverPeers = () => {
    if (!wsClientRef.current || !connected) return;
    wsClientRef.current.discover();
  };

  // Challenge a peer
  const challengePeer = (peer) => {
    if (!wsClientRef.current || inGame) return;
    wsClientRef.current.sendChallenge(peer);
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
            {connected ? `Connected as ${localId}` : "Disconnected"}
          </p>
        </div>

        {/* Nickname + advertising */}
        <div className="px-5 py-2 flex gap-2 border-b border-[var(--color-border-secondary)] shrink-0">
          <p className="text-sm text-[var(--color-text-tertiary)]">Nickname:</p>
          <input
            value={nickname}
            onChange={(e) => setNickname(e.target.value)}
            placeholder="nickname"
            className="px-2 py-1 text-xs rounded-md border border-[var(--color-border-primary)] bg-[var(--color-border-tertiary)] text-[var(--color-text-primary)]"
          />
          <button
            onClick={toggleAdvertising}
            className={`px-3 py-1 text-xs rounded-md ${
              advertising ? "bg-red-500 text-white" : "bg-green-500 text-white"
            }`}
          >
            {advertising ? "Stop Seeking" : "Seek Opponent"}
          </button>

          <button
            onClick={discoverPeers}
            className="px-3 py-1 text-xs rounded-md bg-blue-500 text-white"
          >
            Discover Peers
          </button>
        </div>

        {/* Peers seeking */}
        <div className="px-5 py-2 flex flex-wrap gap-2 border-b border-[var(--color-border-secondary)] shrink-0">
          {peersSeeking.filter((p) => p !== localId).length === 0 ? (
            <p className="text-[var(--color-text-tertiary)]">
              No peers seeking a match
            </p>
          ) : (
            peersSeeking
              .filter((p) => p !== localId)
              .map((peerId) => (
                <button
                  key={peerId}
                  onClick={() => challengePeer(peerId)}
                  className="px-2 py-1 text-xs rounded-md bg-[var(--color-fg-brand-primary)] text-[var(--color-text-primary_on-brand)] hover:bg-[var(--color-fg-brand-secondary_hover)]"
                >
                  Challenge {peerId}
                </button>
              ))
          )}
        </div>
      </main>
    </div>
  );
}
