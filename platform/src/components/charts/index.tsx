export default function Chart({
  title,
  children,
}: {
  title: string;
  children: React.ReactNode;
}) {
  return (
    <div className="flex flex-col">
      <h1 className="text-xl font-bold px-5 pt-5">{title}</h1>
      <div className="flex h-full p-5 relative w-full">{children}</div>
    </div>
  );
}