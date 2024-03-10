import { useContext } from "react";
import Titlebar from "./Titlebar";
import { PlatformContext } from "../contexts/Platform";
import { Outlet } from "react-router-dom";

export default function Layout() {
  const { platform } = useContext(PlatformContext);

  return (
    <div className="h-screen grid grid-rows-[auto_1fr] overflow-hidden">
      <div>{platform === "macos" && <Titlebar />}</div>
      <div className="overflow-y-scroll no-scrollbar h-full"><Outlet /></div>
    </div>
  );
}
