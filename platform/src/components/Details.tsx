import { useEffect, useState } from "react";
import api from "@/utils/api";

function Detail({ title, value }: { title: string; value: string }) {
  return (
    <div className="flex flex-col gap-2 rounded border-b-gray-400 border-2 p-5">
      <div className="text-lg text-blue-950">{title}</div>
      <div className="text-2xl font-bold text-blue-950">{value}</div>
    </div>
  );
}

export default function Details() {
  const [averageTime, setAverageTime] = useState<number | null>(0);
  const [averageAge, setAverageAge] = useState<number | null>(0);
  const [averageScore, setAverageScore] = useState<number | null>(0);

  useEffect(() => {
    api.players.getAverageTimeInGame().then((data) => {
      setAverageTime(data);
    });

    api.users.getAverageAge().then((data) => {
      setAverageAge(data);
    });

    api.players.getAverageScore().then((data) => {
      setAverageScore(data);
    });
  }, []);

  return (
    <div className="relative overflow-auto w-full h-full">
      <div className="flex flex-col gap-5 absolute w-full h-full p-5">
        <Detail
          title="Tiempo promedio que se esta en el juego"
          value={`${averageTime ? averageTime.toFixed(2) : 0} minutos`}
        />
        <Detail
          title="Edad promedio de los jugadores"
          value={`${averageAge ? averageAge.toFixed(0) : 0} años`}
        />
        <Detail
          title="Puntuación promedio de los jugadores"
          value={`${averageScore ? (averageScore * 100).toFixed(1) : 0} %`}
        />
      </div>
    </div>
  );
}
