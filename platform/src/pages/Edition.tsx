import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";
import Admins from "@/components/Admins";
import Tips from "@/components/Tips";

export default function Edition() {
  return (
    <div className="h-full">
      <ResizablePanelGroup direction="horizontal" className="rounded-lg border">
        <ResizablePanel defaultSize={50}>
          <Admins />
        </ResizablePanel>
        <ResizableHandle withHandle />
        <ResizablePanel defaultSize={50}>
          <Tips />
        </ResizablePanel>
      </ResizablePanelGroup>
    </div>
  );
}
