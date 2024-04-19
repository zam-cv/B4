import { useEffect } from "react";
import axios from "axios";
import { getConfig } from "../utils/auth";
import { API_URL } from "../utils/constants";

export default function History({ id }: { id: string | null }) {
  useEffect(() => {
    (async () => {
      const config = await getConfig();
      if (!id) return;

      axios
        .get(`${API_URL}/player/${id}/history`, config)
        .then(({ data }: { data: any }) => {
          console.log(data);
        });
    })();
  }, [id]);

  return <div>{id}</div>;
}
