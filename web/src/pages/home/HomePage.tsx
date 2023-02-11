import { useNavigate } from "react-router-dom";
import styles from "./HomePage.module.css";
import { RouteUtils } from "../../router";

interface HomePageProps {}

export const HomePage: React.FC<HomePageProps> = () => {
  const navigate = useNavigate();
  return (
    <div id={styles.home}>
      <h1 className={styles.title}>网络文件柜</h1>
      <div className={styles.content}>
        <div
          className={styles.button}
          onClick={() => navigate(RouteUtils.pickupPath)}
        >
          取件
        </div>
        <div
          className={styles.button}
          onClick={() => navigate(RouteUtils.storePath)}
        >
          寄件
        </div>
      </div>
    </div>
  );
};
