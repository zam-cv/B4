import axios from "axios";
import { useEffect, useState } from "react";
import { API_URL } from "../utils/constants";
import { getConfig } from "../utils/auth";

interface Player {
  current_cycle: number;
  current_score: number;
  current_balance: number;
  max_plots: number;
}

export default function PlayerInfo({ userId }: { userId: string | null }) {
  const [player, setPlayer] = useState<Player | null>(null);

  useEffect(() => {
    (async () => {
      if (!userId) return;

      axios
        .get(`${API_URL}/player/${userId}`, await getConfig())
        .then(({ data }) => {
          setPlayer(data);
        });
    })();
  }, [userId]);

  return (
    player && (
      <div className="grid grid-cols-2 grid-rows-2 h-full p-5">
        <div className="flex flex-col items-center justify-center">
          <h2 className="text-xl font-bold text-center pb-5">Ciclo actual</h2>
          <p>{player.current_cycle}</p>
        </div>
        <div className="flex flex-col items-center justify-center">
          <h2 className="text-xl font-bold text-center pb-5">Puntaje actual</h2>
          <p>{player.current_score}</p>
        </div>
        <div className="flex flex-col items-center justify-center">
          <h2 className="text-xl font-bold text-center pb-5">Saldo actual</h2>
          <p>{player.current_balance}</p>
        </div>
        <div className="flex flex-col items-center justify-center">
          <h2 className="text-xl font-bold text-center pb-5">Máximo numero de parcelas</h2>
          <p>{player.max_plots}</p>
        </div>
      </div>
    )
  )
}
