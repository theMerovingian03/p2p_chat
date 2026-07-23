import "./App.css";
import RegisterPage from "./pages/Register";
import LoginPage from "./pages/Login";
import GuestLoginPage from "./pages/GuestLogin";
import HomePage from "./pages/Home";
import { BrowserRouter, Route, Routes } from "react-router-dom";

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<HomePage />} />
        <Route path="/register" element={<RegisterPage />} />
        <Route path="/login" element={< LoginPage />} />
        <Route path="/guest" element={<GuestLoginPage />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
