import { useEffect, useState } from "react";
import axios from "axios";
import { getConfig } from "../utils/auth";
import { API_URL } from "../utils/constants";

const EVENT_TYPES = ["Positive", "Negative", "Default"];

interface Statistic {
  cycle: number;
  score: number;
}

export default function History({ id }: { id: string | null }) {
  const [history, setHistory] = useState<[Statistic, [string, string][]][]>([]);

  useEffect(() => {
    (async () => {
      const config = await getConfig();
      if (!id) return;

      axios
        .get(`${API_URL}/player/${id}/history`, config)
        .then(({ data }: { data: any }) => {
          setHistory(data);
        });
    })();
  }, [id]);

  function Action(action: [string, string], index: number) {
    return (
      <li key={index} className="text-slate-700">
        {action[0]} - {action[1]}
      </li>
    );
  }

  return (
    <div className="px-5 pb-7 w-full h-full">
      <div className="relative w-full h-full">
        <div className="absolute w-full h-full overflow-auto flex flex-col gap-5">
          {history
            .sort((a, b) => a[0].cycle - b[0].cycle)
            .map(([stat, actions], index) => (
              <div
                key={index}
                className="bg-slate-100 p-3 rounded-lg flex flex-col gap-3"
              >
                <div className="text-lg font-bold">Ciclo {stat.cycle}</div>
                <div className="text-slate-700">
                  Puntaje: {stat.score.toFixed(2)}
                </div>
                <ul>
                  {EVENT_TYPES.map((type) =>
                    actions
                      .filter((a) => a[1] === type)
                      .map((action, index) => Action(action, index))
                  )}
                </ul>
              </div>
            ))}
        </div>
      </div>
    </div>
  );
}
