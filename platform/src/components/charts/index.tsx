export default function Chart({
  title,
  children,
}: {
  title: string;
  children: React.ReactNode;
}) {
  return (
    <div className="flex flex-col w-full h-full">
      <h1 className="text-xl font-bold px-5 pt-5 text-blue-950 text-center">{title}</h1>
      <div className="flex h-full p-5 relative w-full">{children}</div>
    </div>
  );
}

export function getData(types: string[], values: [string, number][]): number[] {
  const map = new Map<string, number>();

  for (const [type, count] of values) {
    map.set(type, count);
  }

  return types.map((type) => map.get(type) ?? 0);
}
