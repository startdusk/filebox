import { KeyboardRow } from "./KeyboardRow";
import styles from "./Keyboard.module.css";

export const Keyboard: React.FC = () => {
  return (
    <div id={styles.keyboard}>
      <KeyboardRow letters={"qwertyuiop"} isLast={false} />
      <KeyboardRow letters={"asdfghjkl"} isLast={false} />
      <KeyboardRow letters={"zxcvbnm"} isLast={true} />
    </div>
  );
};
