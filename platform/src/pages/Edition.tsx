import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import Admins from "@/components/Admins";
import Tips from "@/components/Tips";
import CropsTypes from "@/components/CropsTypes";
import Events from "@/components/Events";

export default function Edition() {
  return (
    <div className="h-full">
      <ResizablePanelGroup direction="horizontal" className="rounded-lg border">
        <ResizablePanel defaultSize={50}>
          <Admins />
        </ResizablePanel>
        <ResizableHandle withHandle />
        <ResizablePanel defaultSize={50}>
          <Tabs
            defaultValue="tips"
            className="w-full h-full grid grid-rows-[auto_1fr]"
          >
            <div className="p-3">
              <TabsList className="grid grid-cols-3">
                <TabsTrigger value="tips">Tips</TabsTrigger>
                <TabsTrigger value="crops">Cultivos</TabsTrigger>
                <TabsTrigger value="events">Eventos</TabsTrigger>
              </TabsList>
            </div>
            <div>
              <TabsContent className="w-full h-full" value="tips">
                <Tips />
              </TabsContent>
              <TabsContent className="w-full h-full" value="crops">
                <CropsTypes />
              </TabsContent>
              <TabsContent className="w-full h-full m-0" value="events">
                <Events />
              </TabsContent>
            </div>
          </Tabs>
        </ResizablePanel>
      </ResizablePanelGroup>
    </div>
  );
}
