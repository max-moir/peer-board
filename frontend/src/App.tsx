import { Routes, Route } from "react-router-dom";
import Chat from "./components/Chat";
import TopicPage from "./components/TopicPage";

export default function App() {
  return (
    <Routes>
      <Route path="/" element={<Chat></Chat>} />
      <Route path="/topics/:topicName" element={<TopicPage />} />
    </Routes>
  );
}
