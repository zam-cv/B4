import { SOCKET_URL } from "../utils/constants";
import { getConfig } from "../utils/auth";
import Socket from "../utils/socket";
import { PlatformContext } from "../contexts/Platform";
import { useEffect, useState, useContext } from "react";
import UsersTable from "../components/UsersTable";
import PlayerInfo from "../components/PlayerInfo";
import Map from "../components/Map";
import Statistics from "../components/Statistics";
import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import History from "@/components/History";
import api, { User } from "@/utils/api";

interface Captacion {
  visitor_count: number;
}

export default function Users() {
  const [userId, setUserId] = useState<string | null>(null);
  const [userInfo, setUserInfo] = useState<User | null>(null);
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

      api.players.getCountPlayers().then((data) => {
        setInactive(data);
      })
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
      <div className="pt-5">
        <ResizablePanelGroup
          direction="horizontal"
          className="rounded-lg border"
        >
          <ResizablePanel defaultSize={55}>
            <div className="p-5 h-full">
              <div className="h-full w-full relative">
                <div className="absolute h-full w-full overflow-auto">
                  <UsersTable setUserId={setUserId} setUserInfo={setUserInfo} />
                </div>
              </div>
            </div>
          </ResizablePanel>
          <ResizableHandle withHandle />
          <ResizablePanel defaultSize={45}>
            <Tabs
              defaultValue="info"
              className="w-full h-full grid grid-rows-[auto_1fr]"
            >
              <div className="p-3">
                <TabsList className="grid grid-cols-3">
                  <TabsTrigger value="info">
                    Información del usuario
                  </TabsTrigger>
                  <TabsTrigger value="cycles">Recuento de ciclos</TabsTrigger>
                  <TabsTrigger value="location">Ubicación</TabsTrigger>
                </TabsList>
              </div>
              <div>
                <TabsContent className="w-full h-full" value="info">
                  <ResizablePanelGroup
                    className="w-full h-full"
                    direction="vertical"
                  >
                    <ResizablePanel defaultSize={50}>
                      <div className="px-5 pb-5 h-full w-full">
                        <div className="h-full w-full relative">
                          <div className="h-full w-full absolute">
                            <Statistics userId={userId} />
                          </div>
                        </div>
                      </div>
                    </ResizablePanel>
                    <ResizableHandle withHandle />
                    <ResizablePanel defaultSize={50}>
                      <div className="w-full h-full overflow-auto">
                        <PlayerInfo userId={userId} userInfo={userInfo} />
                      </div>
                    </ResizablePanel>
                  </ResizablePanelGroup>
                </TabsContent>
                <TabsContent className="w-full h-full" value="cycles">
                  <History id={userId} />
                </TabsContent>
                <TabsContent className="w-full h-full" value="location">
                  <div className="relative w-full h-full">
                    <div className="absolute w-full h-full flex justify-center items-center">
                      <Map userInfo={userInfo} />
                    </div>
                  </div>
                </TabsContent>
              </div>
            </Tabs>
          </ResizablePanel>
        </ResizablePanelGroup>
      </div>
    </div>
  );
}
