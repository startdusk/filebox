import { useState } from "react";
import { Input } from "../../components/input";
import { Keyboard } from "../../components/keyboard";
import { KeyContext } from "../../context";
import { MainLayout } from "../../layouts/mainLayout";
import styles from "./PickupPage.module.css";
import { Filebox } from "../../service/request";

interface PickupPageProps {}

export const PickupPage: React.FC<PickupPageProps> = () => {
  const [currentAttempt, setCurrentAttempt] = useState("");
  const handleKey = async (key: string) => {
    const letter = key.toLowerCase();
    if (letter === "enter") {
      if (currentAttempt.length < 5) {
        return;
      }

      const data = await Filebox.getFilebox(currentAttempt);
      console.log(data);

      setCurrentAttempt("");
    } else if (letter === "backspace") {
      setCurrentAttempt(currentAttempt.slice(0, currentAttempt.length - 1));
    } else if (/^[0-9a-z]$/.test(letter)) {
      if (currentAttempt.length < 5) {
        setCurrentAttempt(currentAttempt + letter);
      }
    }
  };

  return (
    <MainLayout title="取件">
      <div className={styles.container}>
        <KeyContext.Provider value={{ handleKey }}>
          <div className={styles.input}>
            <Input attempt={currentAttempt} />
          </div>
          <Keyboard />
        </KeyContext.Provider>
      </div>
    </MainLayout>
  );
};
