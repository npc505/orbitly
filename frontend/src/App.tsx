import { Routes, Route } from "react-router-dom";
import MobileFrame from "./components/MobileFrame";
import Login from "./pages/Login";
import Feed from "./pages/Feed";
import Register from "./pages/Register";
import Chat from "./pages/Chat";
import Profile from "./pages/Profile";
import Interests from "./pages/Interests";
import Matches from "./pages/Matches";

function App() {
  return (
    <MobileFrame>
      <Routes>
        <Route path="/" element={<Login />} />
        <Route path="/register" element={<Register />} />
        <Route path="/feed" element={<Feed />} />
        <Route path="/chat/:userId" element={<Chat />} />
        <Route path="/profile/:userId" element={<Profile />} />
        <Route path="/interests" element={<Interests />} />
        <Route path="/matches" element={<Matches />} />
      </Routes>
    </MobileFrame>
  );
}

export default App;
