import { useEffect, useState } from "react";
import { Input } from "../../components/input";
import { Keyboard } from "../../components/keyboard";
import { KeyContext } from "../../context";
import { MainLayout } from "../../layouts/mainLayout";
import styles from "./PickupPage.module.css";
import { Filebox } from "../../service/request";
import { assertIsAxiosError } from "../../filebox";
import {
  Button,
  DialogActions,
  DialogContent,
  Typography,
} from "@mui/material";
import { Alerts } from "../../components/alerts";
import { DialogHeader, FileboxDialog } from "../../components/dialog";

interface PickupPageProps {}

export const PickupPage: React.FC<PickupPageProps> = () => {
  const [currentAttempt, setCurrentAttempt] = useState("");
  const [shaking, setShaking] = useState(false);
  const [inputMiss, setInputMiss] = useState(false);
  const [inputFull, setInputFull] = useState(false);
  const [reqestErr, setReqestErr] = useState(false);
  const [filecode, setFilecode] = useState("");
  const [open, setOpen] = useState(false);
  const [text, setText] = useState("");
  const openDialog = (filecode: string, filename: string) => {
    setOpen(true);
    setFilecode(filecode);
    setText(filename);
  };
  const handleClose = () => {
    setOpen(false);
  };

  const handlePickup = async () => {
    handleClose();
    await Filebox.takeFilebox(filecode);
    setCurrentAttempt("");
    setOpen(false);
  };

  const handleKey = async (key: string) => {
    const letter = key.toLowerCase();
    if (letter === "enter") {
      if (currentAttempt.length < 5) {
        setInputMiss(true);
        setShaking(true);
        setTimeout(() => {
          setShaking(false);
          setInputMiss(false);
        }, 1000);
        return;
      }

      try {
        const res = await Filebox.getFilebox(currentAttempt);
        openDialog(currentAttempt, res.name);
      } catch (err) {
        assertIsAxiosError(err);
        if (err.response?.status === 404) {
          setShaking(true);
          setReqestErr(true);
          setTimeout(() => {
            setShaking(false);
            setReqestErr(false);
          }, 1000);
        }
        return;
      }

      setCurrentAttempt("");
    } else if (letter === "backspace") {
      setCurrentAttempt(currentAttempt.slice(0, currentAttempt.length - 1));
    } else if (/^[0-9a-z]$/.test(letter)) {
      if (currentAttempt.length < 5) {
        setCurrentAttempt(currentAttempt + letter);
      } else {
        setInputFull(true);
        setTimeout(() => {
          setInputFull(false);
        }, 1000);
      }
    }
  };
  const handleKeyDown = (e: KeyboardEvent) => {
    if (e.ctrlKey || e.metaKey || e.altKey) {
      return;
    }
    handleKey(e.key);
  };
  useEffect(() => {
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  });

  return (
    <MainLayout title="取件">
      <div className={styles.container}>
        <KeyContext.Provider value={{ handleKey }}>
          <div className={styles.input}>
            <Input attempt={currentAttempt} shaking={shaking} />
          </div>
          <Keyboard />
        </KeyContext.Provider>
        {reqestErr && (
          <Alerts title="Error" severity="error">
            口令错误 <strong>还有5次重试机会!</strong>
          </Alerts>
        )}
        {inputMiss && (
          <Alerts title="Warn" severity="warning">
            请输入5位文件口令
          </Alerts>
        )}
        {inputFull && (
          <Alerts title="Info" severity="info">
            请按ENTER
          </Alerts>
        )}
      </div>

      <FileboxDialog open={open}>
        <DialogHeader id="store-dialog" onClose={handleClose}>
          取件成功
        </DialogHeader>
        <DialogContent sx={{ minWidth: "300px" }} dividers>
          <Typography gutterBottom>{text}</Typography>
        </DialogContent>
        <DialogActions>
          <Button autoFocus onClick={handlePickup}>
            打 开
          </Button>
        </DialogActions>
      </FileboxDialog>
    </MainLayout>
  );
};
