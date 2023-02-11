import { useContext } from "react";
import { KeyContext, KeyContextType } from "../../context";
import { LIGHT_GREY } from "../../color";
import styles from "./Button.module.css";

interface PropsType {
  buttonKey: string;
  children: string;
  color?: string;
}

export const Button: React.FC<PropsType> = ({
  buttonKey,
  children,
  color = LIGHT_GREY,
}) => {
  const { handleKey } = useContext(KeyContext) as KeyContextType;
  return (
    <button
      className={styles.button}
      onClick={() => {
        handleKey(buttonKey);
      }}
      style={{
        backgroundColor: color,
        borderColor: color,
      }}
    >
      {children}
    </button>
  );
};
