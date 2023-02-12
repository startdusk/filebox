import { Button } from "../button";

interface PropsType {
  letters: string;
  isLast: boolean;
}

export const KeyboardRow: React.FC<PropsType> = ({ letters, isLast }) => {
  return (
    <div>
      {isLast ? <Button buttonKey={"enter"} children={"Enter"} /> : null}
      {Array.from(letters).map((letter, index) => {
        return <Button buttonKey={letter} children={letter} key={index} />;
      })}
      {isLast ? (
        <Button buttonKey={"backspace"} children={"Backspace"} />
      ) : null}
    </div>
  );
};
