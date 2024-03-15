import { ComposableMap, Geographies, Geography } from "react-simple-maps";

export default function Distribution() {
  return (
    <div className="grid grid-cols-4 grid-rows-4 gap-5 h-full">
      <div className="bg-gray-300 row-span-3"></div>
      <div className="col-span-3 row-span-3 w-full h-full overflow-hidden">
        <ComposableMap>
          <Geographies geography="/features.json">
            {({ geographies }) =>
              geographies.map((geo) => (
                <Geography key={geo.rsmKey} fill="#214770" geography={geo} />
              ))
            }
          </Geographies>
        </ComposableMap>
      </div>
      <div className="bg-gray-300"></div>
      <div className="bg-gray-300 col-span-3"></div>
    </div>
  );
}
