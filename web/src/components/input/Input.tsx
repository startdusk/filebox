import { Cell } from "./Cell";

import styles from "./Input.module.css";

interface PropsType {
  attempt: string;
  length?: number;
  shaking?: boolean;
}

const defaultLength = 5;

export const Input: React.FC<PropsType> = ({
  attempt,
  length = defaultLength,
  shaking = false,
}) => {
  if (length <= 0 || length > 5) {
    length = defaultLength;
  }
  const cells = [];
  for (let i = 0; i < length; i++) {
    cells.push(<Cell key={i} index={i} attempt={attempt} />);
  }
  return <div className={shaking ? styles.shaking : ""}>{cells}</div>;
};
