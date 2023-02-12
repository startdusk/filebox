import { KeyboardRow } from "./KeyboardRow";
import styles from "./Keyboard.module.css";

export const Keyboard: React.FC = () => {
  return (
    <div id={styles.keyboard}>
      <KeyboardRow letters={"0123456789"} isLast={false} key={1} />
      <KeyboardRow letters={"qwertyuiop"} isLast={false} key={2} />
      <KeyboardRow letters={"asdfghjkl"} isLast={false} key={3} />
      <KeyboardRow letters={"zxcvbnm"} isLast={true} key={4} />
    </div>
  );
};
