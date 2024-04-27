import { useState, useEffect } from "react";
import { Checkbox } from "@/components/ui/checkbox";
import api, { Admin } from "@/utils/api";

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

export default function Permissions({ userInfo }: { userInfo: Admin | null }) {
  const [permissions, setPermissions] = useState<Set<string> | null>(new Set());
  const [userPermissions, setUserPermissions] = useState<Set<string> | null>(
    new Set()
  );

  useEffect(() => {
    if (!userInfo) return;

    api.permissions.getPermissionsTypes(userInfo.role_id).then((data) => {
      setPermissions(new Set(data));
    });

    api.permissions.getUserPermissions(userInfo.id).then((data) => {
      setUserPermissions(new Set(data));
    });
  }, [userInfo]);

  async function setPermission(permission: string, value: boolean) {
    if (!userInfo || !permissions || !userPermissions) return;

    if (value) {
      userPermissions.add(permission);
      api.permissions.setPermission(userInfo.id, permission);
    } else {
      userPermissions.delete(permission);
      api.permissions.deletePermission(userInfo.id, permission);
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