export const handleKeyDown = (
  e: React.KeyboardEvent<HTMLInputElement>,
  ref: React.RefObject<HTMLInputElement>
) => {
  if (e.key === "Enter") {
    e.preventDefault();
    ref.current?.focus();
  }
};

export const handleEnter = (
  e: React.KeyboardEvent<HTMLInputElement>,
  callback: Function
) => {
  if (e.key === "Enter") {
    e.preventDefault();
    callback();
  }
};
