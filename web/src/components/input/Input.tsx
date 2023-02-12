import { Cell } from "./Cell";

interface PropsType {
  attempt: string;
  length?: number;
}

const defaultLength = 5;

export const Input: React.FC<PropsType> = ({
  attempt,
  length = defaultLength,
}) => {
  if (length <= 0 || length > 5) {
    length = defaultLength;
  }
  const cells = [];
  for (let i = 0; i < length; i++) {
    cells.push(<Cell key={i} index={i} attempt={attempt} />);
  }
  return <div>{cells}</div>;
};
