import { SOCKET_URL, API_URL } from "../utils/constants";
import { getConfig } from "../utils/auth";
import Socket from "../utils/socket";
import { PlatformContext } from "../contexts/Platform";
import { useEffect, useState, useContext } from "react";
import axios from "axios";
import UsersTable, { Payment } from "../components/UsersTable";
import PlayerInfo from "../components/PlayerInfo";
import Map from "../components/Map";
import Statistics from "../components/Statistics";
import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";

interface Captacion {
  visitor_count: number;
}

export default function Users() {
  const [userId, setUserId] = useState<string | null>(null);
  const [userInfo, setUserInfo] = useState<Payment | null>(null);
  const { platform } = useContext(PlatformContext);
  const [active, setActive] = useState(0);
  const [inactive, setInactive] = useState(0);

  useEffect(() => {
    (async () => {
      const config = await getConfig();
      if (!platform) return;

      Socket(
        platform ?? "",
        SOCKET_URL,
        (message: any) => {
          const captacion: Captacion = JSON.parse(message);
          setActive(captacion.visitor_count);
        },
        config.headers.token as string
      );

      axios
        .get(`${API_URL}/players/count`, config)
        .then(({ data }: { data: any }) => {
          setInactive(data);
        });
    })();
  }, [platform]);

  return (
    <div className="divide-y divide-zinc-300 h-full grid grid-rows-[auto_1fr]">
      <div className="py-5 flex px-3 pt-3 gap-10">
        <div className="flex items-baseline">
          <div className="text-green-700 font-bold text-2xl mr-3">{active}</div>
          <div className="text-gray-500 font-bold text-lg">activos</div>
        </div>
        <div className="flex items-baseline">
          <div className="text-red-700 font-bold text-2xl mr-3">
            {inactive - active}
          </div>
          <div className="text-gray-500 font-bold text-lg">inactivos</div>
        </div>
      </div>
      <div className="p-5">
        <ResizablePanelGroup
          direction="horizontal"
          className="rounded-lg border"
        >
          <ResizablePanel defaultSize={50}>
            <div className="p-5">
              <UsersTable setUserId={setUserId} setUserInfo={setUserInfo} />
            </div>
          </ResizablePanel>
          <ResizableHandle withHandle />
          <ResizablePanel defaultSize={50}>
            <ResizablePanelGroup direction="vertical">
              <ResizablePanel defaultSize={40}>
                <div>
                  <Map userInfo={userInfo} />
                </div>
              </ResizablePanel>
              <ResizableHandle withHandle />
              <ResizablePanel defaultSize={30}>
                <div className="h-full p-5">
                  <Statistics />
                </div>
              </ResizablePanel>
              <ResizableHandle withHandle />
              <ResizablePanel defaultSize={30}>
                <div>
                  <PlayerInfo userId={userId} userInfo={userInfo} />
                </div>
              </ResizablePanel>
            </ResizablePanelGroup>
          </ResizablePanel>
        </ResizablePanelGroup>
      </div>
    </div>
  );
}
