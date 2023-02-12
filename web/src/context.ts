import { createContext } from "react";

export type KeyContextType = {
  handleKey: (key: string) => void;
};

export const KeyContext = createContext<KeyContextType | null>(null);
