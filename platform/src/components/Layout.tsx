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
    <div className="h-full grid grid-rows-[auto_1fr] overflow-hidden">
      <div>
        <div>{platform === "macos" && <Titlebar />}</div>
        {isAuthenticated ? <Header /> : <div></div>}
      </div>
      {isAuthenticated ? (
        <div className="p-5 overflow-auto">
          <Outlet />{" "}
        </div>
      ) : (
        <div>
          <Login />
        </div>
      )}
    </div>
  );
}
