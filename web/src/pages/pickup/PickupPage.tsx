import { useState } from "react";
import { Input } from "../../components/input";
import { Keyboard } from "../../components/keyboard";
import { KeyContext } from "../../context";
import { MainLayout } from "../../layouts/mainLayout";
import styles from "./PickupPage.module.css";

interface PickupPageProps {}

export const PickupPage: React.FC<PickupPageProps> = () => {
  const [currentAttempt, setCurrentAttempt] = useState("");
  const handleKey = (key: string) => {
    const letter = key.toLowerCase();
    if (letter === "enter") {
      if (currentAttempt.length < 5) {
        return;
      }
      setCurrentAttempt("");
    } else if (letter === "backspace") {
      setCurrentAttempt(currentAttempt.slice(0, currentAttempt.length - 1));
    } else if (/^[0-9a-z]$/.test(letter)) {
      if (currentAttempt.length < 5) {
        setCurrentAttempt(currentAttempt + letter);
      }
    }
  };
  const bestColors = (key: string) => {};

  return (
    <MainLayout title="取件">
      <div className={styles.container}>
        <KeyContext.Provider value={{ handleKey, bestColors }}>
          <div className={styles.input}>
            <Input attempt={currentAttempt} />
          </div>
          <Keyboard />
        </KeyContext.Provider>
      </div>
    </MainLayout>
  );
};
