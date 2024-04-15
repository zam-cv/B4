import {
  ComposableMap,
  Geographies,
  Geography,
  ZoomableGroup,
} from "react-simple-maps";

export default function Distribution() {
  return (
    <div className="grid grid-cols-4 grid-rows-4 gap-5 h-full">
      <div className="bg-gray-300 row-span-3"></div>
      <div className="col-span-3 row-span-3 w-full h-full overflow-hidden">
        <div className="w-full h-full relative flex justify-center items-center">
          <div className="w-full h-full absolute flex justify-center items-center">
            <ComposableMap>
              <ZoomableGroup center={[0, 0]} zoom={1}>
                <Geographies geography="/features.json">
                  {({ geographies }) =>
                    geographies.map((geo) => (
                      <Geography
                        key={geo.rsmKey}
                        fill="#214770"
                        geography={geo}
                      />
                    ))
                  }
                </Geographies>
              </ZoomableGroup>
            </ComposableMap>
          </div>
        </div>
      </div>
      <div className="bg-gray-300"></div>
      <div className="bg-gray-300 col-span-3"></div>
    </div>
  );
}
