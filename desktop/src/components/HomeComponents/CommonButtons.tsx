import React from "react";

type Props = {
    children: React.ReactNode;
    onClick?: (e?: React.MouseEvent<HTMLButtonElement>) => void;
    className?: string;
    type?: "button" | "submit" | "reset";
    disabled?: boolean;
};

const baseClass =
    "cursor-pointer mt-2 w-full border-white text-white rounded-2xl border p-2.5 text-center transition-colors duration-200 hover:bg-white hover:text-blue-900";

export default function CommonButton({
    children,
    onClick,
    className = "",
    type = "button",
    disabled = false,
}: Props) {
    return (
        <button
            type={type}
            onClick={onClick}
            disabled={disabled}
            className={`${baseClass} ${className}`.trim()}
        >
            {children}
        </button>
    );
}
