import { handleKeyDown, handleEnter } from "../utils";
import { useRef, useState } from "react";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import AdminsTable from "@/components/AdminsTable";
import Permissions from "@/components/Permissions";
import api, { Admin } from "@/utils/api";

export default function Edition() {
  const user = useState<string | null>(null);
  const [userInfo, setUserInfo] = useState<Admin | null>(null);
  const [data, setData] = useState<Admin[]>([]);
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

    api.auth.registerAdmin(email, password).then((data) => {
      emailRef.current!.value = "";
      passwordRef.current!.value = "";
      confirmRef.current!.value = "";
      setLoading(false);

      setData((prev) => [...prev, data]);
      setUserInfo(data);
    }).catch((error) => {
      console.error(error);
      setLoading(false);
    });
  }

  return (
    <div className="grid p-5 gap-10">
      <div className="flex flex-col gap-5">
        <h1 className="text-2xl font-bold text-blue-950">Registrar un nuevo administrador</h1>
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
            <Button className="bg-blue-950 hover:bg-blue-800" disabled>Registrando...</Button>
          ) : (
            <Button className="bg-blue-950 hover:bg-blue-800" onClick={register}>Registrar</Button>
          )}
        </div>
      </div>
      <div className="flex flex-col gap-5">
        <h1 className="text-2xl font-bold text-blue-950">Lista de administradores</h1>
        <Permissions userInfo={userInfo} />
        <AdminsTable
          setUserId={user[1]}
          setUserInfo={setUserInfo}
          data={data}
          setData={setData}
        />
      </div>
    </div>
  );
}
