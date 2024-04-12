import axios from "axios";
import { useEffect, useState } from "react";
import { API_URL } from "../utils/constants";
import { getConfig } from "../utils/auth";
import { Payment } from "./UsersTable";

interface Player {
  current_cycle: number;
  current_score: number;
  balance_cash: number;
  balance_verqor: number;
  balance_coyote: number;
  max_plots: number;
}

function Field({ title, value }: { title: string; value: string | number }) {
  return (
    <div className="flex text-base">
      <span className="font-bold pr-3">{title}:</span> <p>{value}</p>
    </div>
  );
}

export default function PlayerInfo({
  userId,
  userInfo,
}: {
  userId: string | null;
  userInfo: Payment | null;
}) {
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
      <div className="grid grid-cols-2 h-full p-5">
        <Field title="Nombre de usuario" value={userInfo?.username ?? ""} />
        <Field title="Tipo de usuario" value={userInfo?.user_type ?? ""} />
        <Field title="Correo electrónico" value={userInfo?.email ?? ""} />
        <Field title="Genero" value={userInfo?.gender ?? ""} />
        <Field title="Edad" value={userInfo?.age ?? 0} />
        <Field title="Sistema operativo" value={userInfo?.os ?? ""} />
        <Field title="Latitud" value={userInfo?.latitude ?? 0} />
        <Field title="Longitud" value={userInfo?.longitude ?? 0} />
        <Field title="Ciclo actual" value={player.current_cycle.toString()} />
        <Field title="Puntaje actual" value={player.current_score.toString()} />
        <Field title="Saldo en efectivo" value={player.balance_cash.toString()} />
        <Field
          title="Saldo de verqor"
          value={player.balance_verqor.toString()}
        />
        <Field
          title="Saldo del coyote"
          value={player.balance_coyote.toString()}
        />
        <Field
          title="Máximo numero de parcelas"
          value={player.max_plots.toString()}
        />
      </div>
    )
  );
}
