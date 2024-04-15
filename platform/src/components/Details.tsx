import { useEffect, useState } from "react";
import { API_URL } from "@/utils/constants";
import { getConfig } from "@/utils/auth";
import axios from "axios";

function Detail({ title, value }: { title: string; value: string }) {
  return (
    <div className="flex flex-col gap-2 rounded border-b-gray-400 border-2 p-5">
      <div className="text-lg">{title}</div>
      <div className="text-2xl font-bold">{value}</div>
    </div>
  );
}

export default function Details() {
  const [averageTime, setAverageTime] = useState(0);
  const [averageAge, setAverageAge] = useState(0);

  useEffect(() => {
    (async () => {
      const config = await getConfig();

      axios.get(`${API_URL}/players/average-time`, config).then(({ data }) => {
        setAverageTime(data);
      });

      axios.get(`${API_URL}/users/average-age`, config).then(({ data }) => {
        setAverageAge(data);
      });
    })();
  }, []);

  return (
    <div className="flex flex-col p-5 gap-5">
      <Detail
        title="Tiempo promedio que se esta en el juego"
        value={`${averageTime.toFixed(2)} minutos`}
      />
      <Detail
        title="Edad promedio de los jugadores"
        value={`${averageAge.toFixed(0)} años`}
      />
    </div>
  );
}
