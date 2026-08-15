import "./App.css";
import RegisterPage from "./pages/Register";
import LoginPage from "./pages/Login";
import GuestLoginPage from "./pages/GuestLogin";
import LoadingPage from "./pages/LoadingPage";
import HomePage from "./pages/Home";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import TitleBar from "./components/TitleBar";
import Footer from "./components/Footer";
import AuthComponent from "./components/AuthComponent";

function App() {
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
          </Route>
        </Routes>
      </BrowserRouter>
      <Footer />
    </>
  );
}

export default App;
