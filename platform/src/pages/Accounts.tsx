import { handleKeyDown, handleEnter } from "../utils";
import { getConfig } from "../utils/auth";
import { Payment } from "@/components/AdminsTable";
import { useRef, useState } from "react";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import AdminsTable from "@/components/AdminsTable";
import axios from "axios";
import { API_URL } from "../utils/constants";

import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";

export default function Accounts() {
  const [data, setData] = useState<Payment[]>([]);
  const [loading, setLoading] = useState(false);
  const emailRef = useRef<HTMLInputElement>(null);
  const passwordRef = useRef<HTMLInputElement>(null);
  const confirmRef = useRef<HTMLInputElement>(null);

  async function register() {
    const email = emailRef.current?.value;
    const password = passwordRef.current?.value;
    const confirm = confirmRef.current?.value;

    if (!email || !password || !confirm || password !== confirm) return;
    setLoading(true);

    const config = await getConfig();

    axios
      .post(`${API_URL}/auth/register`, {
        email,
        password,
      }, config)
      .then(({ data }) => {
        emailRef.current!.value = "";
        passwordRef.current!.value = "";
        confirmRef.current!.value = "";
        setLoading(false);

        let id = data as string;
        let payment: Payment = { id, email };
        setData((prev) => [...prev, payment]);
      })
      .catch((error) => {
        console.error(error);
        setLoading(false);
      });
  }

  return (
    <div className="h-full">
      <ResizablePanelGroup direction="horizontal" className="rounded-lg border">
        <ResizablePanel defaultSize={50}>
          <div className="grid p-5 gap-10">
            <div className="flex flex-col gap-5">
              <h1 className="text-2xl font-bold">
                Registrar un nuevo administrador
              </h1>
              <Input
                type="email"
                ref={emailRef}
                placeholder="Correo electrónico"
                onKeyDown={(e) => handleKeyDown(e, passwordRef)}
              />
              <div className="flex gap-5">
                <Input
                  type="password"
                  ref={passwordRef}
                  onKeyDown={(e) => handleKeyDown(e, confirmRef)}
                  placeholder="Contraseña"
                />
                <Input
                  type="password"
                  ref={confirmRef}
                  onKeyDown={(e) => handleEnter(e, register)}
                  placeholder="Confirmar contraseña"
                />
                {loading ? (
                  <Button disabled>Registrando...</Button>
                ) : (
                  <Button onClick={register}>Registrar</Button>
                )}
              </div>
            </div>
            <div className="flex flex-col gap-5">
              <h1 className="text-2xl font-bold">Lista de administradores</h1>
              <AdminsTable data={data} setData={setData} />
            </div>
          </div>
        </ResizablePanel>
        <ResizableHandle withHandle />
        <ResizablePanel defaultSize={50}>
          <div>Edit</div>
        </ResizablePanel>
      </ResizablePanelGroup>
    </div>
  );
}
