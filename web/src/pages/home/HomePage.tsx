import { useNavigate } from "react-router-dom";
import styles from "./HomePage.module.css";
import { Route } from "../../constant";

interface HomePageProps {}

export const HomePage: React.FC<HomePageProps> = () => {
  const navigate = useNavigate();
  return (
    <div id={styles.home}>
      <h1 className={styles.title}>网络文件柜</h1>
      <div className={styles.content}>
        <div
          className={styles.button}
          onClick={() => navigate(Route.pickupPath)}
        >
          取件
        </div>
        <div
          className={styles.button}
          onClick={() => navigate(Route.storePath)}
        >
          寄件
        </div>
      </div>
    </div>
  );
};
