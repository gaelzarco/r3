import { ButtonHTMLAttributes } from "react";

interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {}

export default function Button({ ...props }: ButtonProps) {
  return (
    <button
      className={`px-5 py-2 min-w-[100px] text-white bg-neutral-900 border-black border-solid backdrop-blur-lg rounded-full hover:text-black hover:bg-neutral-100 hover:border-white-solid transition-all ${props.className}`}
      {...props}
    >
      {props.children}
    </button>
  );
}
