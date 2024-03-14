import { SOCKET_URL, API_URL } from "../utils/constants";
import { getToken } from "../hooks/useAuth";
import Socket from "../utils/socket";
import { PlatformContext } from "../contexts/Platform";
import { useEffect, useState, useContext } from "react";
import axios from "axios";
import UsersTable from "../components/UsersTable";

interface Captacion {
  visitor_count: number;
}

export default function Dashboard() {
  const { platform } = useContext(PlatformContext);
  const [active, setActive] = useState(0);
  const [inactive, setInactive] = useState(0);

  useEffect(() => {
    (async () => {
      const token = await getToken();
      if (!platform) return;

      Socket(
        platform ?? "",
        SOCKET_URL,
        (message: any) => {
          const captacion: Captacion = JSON.parse(message);
          setActive(captacion.visitor_count);
        },
        token as string
      );

      axios
        .get(`${API_URL}/players/count`, {
          withCredentials: true,
          headers: {
            token,
          },
        })
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
      <div className="py-5 grid grid-cols-2 grid-rows-3 gap-5">
        <div className="row-span-3">
          <UsersTable />
        </div>
        <div className="bg-slate-300 row-span-2"></div>
        <div className="bg-slate-300"></div>
      </div>
    </div>
  );
}
