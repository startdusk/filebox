import { Link, useNavigate } from "react-router-dom";
import styles from "./Header.module.css";
import { Route, ConstantSource } from "../../constant";

interface PropsType {
  title: string;
}

export const Header: React.FC<PropsType> = ({ title }) => {
  const navigate = useNavigate();
  return (
    <div className={styles.container}>
      <button
        className={styles.button}
        onClick={() => navigate(Route.homePath)}
      >
        首页
      </button>
      <h1 className={styles.title}>{title}</h1>
      <button
        className={styles.button}
        onClick={() => {
          window.open(ConstantSource.myGithubRepo);
        }}
      >
        github
      </button>
    </div>
  );
};
