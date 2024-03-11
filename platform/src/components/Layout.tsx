import { useContext } from "react";
import Titlebar from "./Titlebar";
import { PlatformContext } from "../contexts/Platform";
import { Outlet } from "react-router-dom";
import { useAuth } from "../hooks/useAuth";
import Header from "./Header";
import Login from "../pages/Login";

export default function Layout() {
  const { isAuthenticated } = useAuth();
  const { platform } = useContext(PlatformContext);

  return (
    <div className="h-screen grid grid-rows-[auto_auto_1fr] overflow-hidden">
      <div>{platform === "macos" && <Titlebar />}</div>
      {isAuthenticated ? <Header /> : <div></div>}
      <div className="overflow-y-scroll no-scrollbar h-full p-5">
        {isAuthenticated ? <Outlet /> : <Login />}
      </div>
    </div>
  );
}
