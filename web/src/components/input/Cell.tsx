import styles from "./Cell.module.css";

interface PropsType {
  attempt: string;
  index: number;
}

export const Cell: React.FC<PropsType> = ({ attempt, index }) => {
  let content;
  const hasLetter = attempt[index] !== undefined;
  if (hasLetter) {
    content = attempt[index];
  } else {
    content = <div style={{ opacity: 0 }}>X</div>;
  }
  return (
    <div className={styles.cell + " " + (hasLetter ? styles.filled : "")}>
      <div className={styles.box}>{content}</div>
    </div>
  );
};
