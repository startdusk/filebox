import { MainLayout } from "../../layouts/mainLayout";
import styles from "./StorePage.module.css";

interface StorePageProps {}

export const StorePage: React.FC<StorePageProps> = () => {
  return (
    <MainLayout title="寄件">
      <div className={styles.container}></div>
    </MainLayout>
  );
};
