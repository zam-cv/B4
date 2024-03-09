import { useContext } from "react";
import Titlebar from "./Titlebar";
import { PlatformContext } from "../contexts/Platform";

export default function Layout({ children }: { children: React.ReactNode }) {
  const { platform } = useContext(PlatformContext);

  return (
    <div>
      {platform === "macos" && <Titlebar />}
      <div className="overflow-y-auto">{children}</div>
    </div>
  );
}
