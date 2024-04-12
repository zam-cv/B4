import { useState } from "react";
import { Payment } from "@/components/AdminsTable";
import AdminsTable from "@/components/AdminsTable";
import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";

export default function Emails() {
  const stateAdminId = useState<string | null>(null);
  const stateAdminInfo = useState<Payment | null>(null);

  return (
    <div className="h-full">
      <ResizablePanelGroup direction="horizontal" className="rounded-lg border">
        <ResizablePanel defaultSize={50}>
          <div className="p-5">
            <AdminsTable
              setAdminsId={stateAdminId[0] as any}
              setAdminsInfo={stateAdminInfo[0] as any}
            />
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
