import { useRef, useState, useEffect } from "react";
import TipsTable, { Payment } from "@/components/TipsTable";
import { Textarea } from "@/components/ui/textarea";
import { Button } from "@/components/ui/button";
import { API_URL } from "@/utils/constants";
import { getConfig } from "@/utils/auth";
import axios from "axios";

export default function Tips() {
  const [data, setData] = useState<Payment[]>([]);
  const [length, setLength] = useState(0);
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  async function createTip() {
    if (!textareaRef.current || textareaRef.current.value === "") return;

    const tip = textareaRef.current.value;
    const config = await getConfig();

    axios
      .post(
        `${API_URL}/data/tips`,
        {
          content: tip,
        },
        config
      )
      .then((response) => {
        const id = parseInt(response.data);
        const value = { id, content: tip } as Payment;
        setData([...data, value]);
        textareaRef.current!.value = "";
        setLength(0);
      })
      .catch((error) => {
        console.error(error);
      });
  }

  useEffect(() => {
    (async () => {
      const config = await getConfig();
      let users = await axios.get(`${API_URL}/data/tips`, config);
      setData(users.data);
    })();
  }, []);

  return (
    <div className="flex flex-col p-5 w-full h-full gap-5">
      <div className="flex flex-col gap-5">
        <h1 className="text-2xl font-bold text-blue-950">Crear un nuevo Tip</h1>
        <div>
          <Textarea
            ref={textareaRef}
            onChange={() => setLength(textareaRef.current?.value.length || 0)}
            placeholder="Escribe tu tip"
          />
          <p className="ml-2 mt-2 text-sm text-muted-foreground">
            {length}/500
          </p>
        </div>
        <div className="flex justify-end">
          <Button onClick={createTip} className="px-10 bg-blue-950 hover:bg-blue-800">
            Crear Tip
          </Button>
        </div>
      </div>
      <h1 className="text-2xl font-bold text-blue-950">Lista de Tips</h1>
      <div className="relative overflow-auto w-full h-full">
        <div className="absolute w-full h-full">
          <TipsTable data={data} setData={setData} />
        </div>
      </div>
    </div>
  );
}
