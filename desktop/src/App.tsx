import "./App.css";
import RegisterPage from "./pages/Register";
import LoginPage from "./pages/Login";
import GuestLoginPage from "./pages/GuestLogin";
import ChatPage from "./pages/Chat";
import LoadingPage from "./pages/LoadingPage";
import HomePage from "./pages/Home";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import TitleBar from "./components/TitleBar";
import Footer from "./components/Footer";
import AuthComponent from "./components/AuthComponent";
import { useWebsocketStore } from "./stores/webSocketStore";
import { useDataChannelStore } from "./stores/dataChannelStore";
import { useEffect } from "react";

function App() {

  // Initialize listeners at the app level to prevent duplicate.
  const initializeDcListener = useDataChannelStore((state) => state.initializeEventListener);
  const initializeWsListener = useWebsocketStore((state) => state.initializeEventListeners);

  useEffect(() => {
    void initializeDcListener();
    void initializeWsListener();
  }, [initializeDcListener, initializeWsListener])

  return (
    <>
      <TitleBar />
      <BrowserRouter>
        <Routes>
          <Route path="/" element={<LoadingPage />} />
          <Route path="/register" element={<RegisterPage />} />
          <Route path="/login" element={< LoginPage />} />
          <Route path="/guest" element={<GuestLoginPage />} />
          <Route element={<AuthComponent />}>
            <Route path="/home" element={<HomePage />} />
            <Route path="/chat/:peerId" element={<ChatPage />} />
          </Route>
        </Routes>
      </BrowserRouter>
      <Footer />
    </>
  );
}

export default App;
