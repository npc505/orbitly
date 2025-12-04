import { Routes, Route } from "react-router-dom";
import MobileFrame from "./components/MobileFrame";
import Login from "./pages/Login";
import Feed from "./pages/Feed";
import Register from "./pages/Register";

function App() {
  return (
    <MobileFrame>
      <Routes>
        <Route path="/" element={<Login />} />
        <Route path="/register" element={<Register />} />
        <Route path="/feed" element={<Feed />} />
      </Routes>
    </MobileFrame>
  );
}

export default App;
