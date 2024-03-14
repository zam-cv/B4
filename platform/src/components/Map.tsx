import {
  ComposableMap,
  Geographies,
  Geography,
  Marker,
} from "react-simple-maps";
import { TransformWrapper, TransformComponent } from "react-zoom-pan-pinch";
import { Payment } from "./UsersTable";

export default function Map({ userInfo }: { userInfo: Payment | null }) {
  if (!userInfo) return <div></div>;

  return (
    <TransformWrapper
      centerOnInit={true}
      initialScale={2}
      limitToBounds={false}
    >
      <TransformComponent wrapperClass="w-full h-full">
        <ComposableMap className="w-full h-full">
          <Geographies geography="/features.json">
            {({ geographies }) =>
              geographies.map((geo) => (
                <Geography key={geo.rsmKey} fill="#214770" geography={geo} />
              ))
            }
          </Geographies>
          <Marker coordinates={[userInfo.longitude, userInfo.latitude]}>
            <circle r={3} fill="#F00" />
          </Marker>
        </ComposableMap>
      </TransformComponent>
    </TransformWrapper>
  );
}
