import { SOCKET_URL, API_URL } from "../utils/constants";
import { getToken } from "../hooks/useAuth";
import Socket from "../utils/socket";
import { PlatformContext } from "../contexts/Platform";
import { useEffect, useState, useContext } from "react";
import axios from "axios";

interface Data {
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
          const data: Data = JSON.parse(message.data);
          console.log(data);
          setActive(data.visitor_count);
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
    <div className="divide-y divide-zinc-300">
      <div className="py-5 flex px-3 pt-3 gap-10">
        <div className="flex items-baseline">
          <div className="text-green-700 font-bold text-2xl mr-3">{active}</div>
          <div className="text-gray-500 font-bold text-lg">activos</div>
        </div>
        <div className="flex items-baseline">
          <div className="text-red-700 font-bold text-2xl mr-3">{inactive}</div>
          <div className="text-gray-500 font-bold text-lg">inactivos</div>
        </div>
      </div>
      <div className="py-5">Graphics</div>
    </div>
  );
}
