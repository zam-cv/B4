import { useRef } from "react";

export default function ContentEdit({
  onInput,
  value,
}: {
  onInput: (ref: HTMLParagraphElement | null) => void;
  value: string;
}) {
  const ref = useRef<HTMLParagraphElement>(null);

  // Prevent the user from creating a new line when pressing enter
  function handleKeyDown(event: React.KeyboardEvent<HTMLParagraphElement>) {
    if (event.key === "Enter") {
      event.preventDefault();
    }
  }

  return (
    <p
      ref={ref}
      contentEditable
      className="outline-none"
      onInput={() => onInput(ref.current)}
      onKeyDown={handleKeyDown}
    >
      {value}
    </p>
  );
}
