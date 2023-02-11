import { Keyboard } from "../../components/keyboard";
import { KeyContext } from "../../context";
import { MainLayout } from "../../layouts/mainLayout";
import styles from "./PickupPage.module.css";

interface PickupPageProps {}

export const PickupPage: React.FC<PickupPageProps> = () => {
  const handleKey = (key: string) => {};
  const bestColors = (key: string) => {};

  return (
    <MainLayout title="取件">
      <div className={styles.container}>
        <KeyContext.Provider value={{ handleKey, bestColors }}>
          {/* <Grid history={history} currentAttempt={currentAttempt} />
           */}
          <Keyboard />
        </KeyContext.Provider>
      </div>
    </MainLayout>
  );
};
