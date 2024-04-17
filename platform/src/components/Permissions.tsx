import { getConfig } from "../utils/auth";
import { Payment } from "@/components/AdminsTable";
import { useState, useEffect } from "react";
import axios from "axios";
import { API_URL } from "../utils/constants";
import { Checkbox } from "@/components/ui/checkbox";

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

export default function Permissions({ userInfo }: { userInfo: Payment | null }) {
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