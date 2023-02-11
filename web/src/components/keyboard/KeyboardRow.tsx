// import { useContext } from "react";
// import { KeyContext, KeyContextType } from "../../context";
import { Button } from "../button";

interface PropsType {
  letters: string;
  isLast: boolean;
}

export const KeyboardRow: React.FC<PropsType> = ({ letters, isLast }) => {
  //   const { bestColors } = useContext(KeyContext) as KeyContextType;
  return (
    <div>
      {isLast ? <Button buttonKey={"enter"} children={"Enter"} /> : null}
      {Array.from(letters).map((letter) => {
        return (
          <Button
            buttonKey={letter}
            children={letter}
            // color={bestColors.get(letter)}
          />
        );
      })}
      {isLast ? (
        <Button buttonKey={"backspace"} children={"Backspace"} />
      ) : null}
    </div>
  );
};
