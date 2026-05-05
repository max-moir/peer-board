import { Routes, Route } from "react-router-dom";
import Chat from "./components/Chat";
import BattleshipPage from "./components/BattleshipChallenge";

export default function App() {
  return (
    <Routes>
      <Route path="/" element={<Chat></Chat>} />
      <Route path="/challenge" element={<BattleshipPage></BattleshipPage>} />
    </Routes>
  );
}
