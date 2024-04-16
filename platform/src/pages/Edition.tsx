import { handleKeyDown, handleEnter } from "../utils";
import { getConfig } from "../utils/auth";
import { Payment } from "@/components/AdminsTable";
import { useRef, useState, useEffect } from "react";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import AdminsTable from "@/components/AdminsTable";
import axios from "axios";
import { API_URL } from "../utils/constants";
import { Checkbox } from "@/components/ui/checkbox";
import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";

function getPermissionDiff(
  permissions: Set<string> | null,
  userPermissions: Set<string> | null
): [string, boolean][] {
  if (!permissions || !userPermissions) return [];
  const diff = new Map<string, boolean>();

  permissions.forEach((permission) => {
    diff.set(permission, userPermissions.has(permission));
  });

  return Array.from(diff);
}

function Permissions({ userInfo }: { userInfo: Payment | null }) {
  const [permissions, setPermissions] = useState<Set<string> | null>(new Set());
  const [userPermissions, setUserPermissions] = useState<Set<string> | null>(
    new Set()
  );

  useEffect(() => {
    if (!userInfo) return;

    (async () => {
      const config = await getConfig();

      axios
        .get(`${API_URL}/permissions/types/${userInfo.role_id}`, config)
        .then(({ data }) => setPermissions(new Set(data)))
        .catch((error) => console.error(error));

      axios
        .get(`${API_URL}/permissions/${userInfo.id}`, config)
        .then(({ data }) => setUserPermissions(new Set(data)))
        .catch((error) => console.error(error));
    })();
  }, [userInfo]);

  async function setPermission(permission: string, value: boolean) {
    if (!userInfo || !permissions || !userPermissions) return;
    const config = await getConfig();

    if (value) {
      userPermissions.add(permission);

      axios
      .post(
        `${API_URL}/permissions`,
        {
          id: userInfo.id,
          permission,
        },
        config
      )
      .catch((error) => console.error(error));
    } else {
      userPermissions.delete(permission);

      axios
      .delete(`${API_URL}/permissions/${userInfo.id}/${permission}`, config)
      .catch((error) => console.error(error));
    }

    setUserPermissions(new Set(userPermissions));
  }

  if (!userInfo) return null;

  return (
    <div>
      <h1 className="text-lg mb-5">
        Permisos de <span className="underline">{userInfo?.email}</span>
      </h1>
      <div className="flex gap-5 flex-wrap">
        {Array.from(getPermissionDiff(permissions, userPermissions)).map(
          ([permission, hasPermission]) => (
            <div key={permission} className="flex gap-5">
              <Checkbox
                checked={hasPermission}
                onCheckedChange={() =>
                  setPermission(permission, !hasPermission)
                }
              >
                {permission}
              </Checkbox>
              <div className="text-sm">
                <span>{permission}</span>
              </div>
            </div>
          )
        )}
      </div>
    </div>
  );
}

export default function Edition() {
  const user = useState<string | null>(null);
  const [userInfo, setUserInfo] = useState<Payment | null>(null);
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
      .post(
        `${API_URL}/auth/register`,
        {
          email,
          password,
        },
        config
      )
      .then(({ data }) => {
        emailRef.current!.value = "";
        passwordRef.current!.value = "";
        confirmRef.current!.value = "";
        setLoading(false);

        setData((prev) => [...prev, data as Payment]);
        setUserInfo(data as Payment);
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
              <Permissions userInfo={userInfo} />
              <AdminsTable
                setUserId={user[1]}
                setUserInfo={setUserInfo}
                data={data}
                setData={setData}
              />
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
