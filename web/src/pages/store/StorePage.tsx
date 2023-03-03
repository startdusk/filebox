import TabContext from "@mui/lab/TabContext";
import TabList from "@mui/lab/TabList";
import TabPanel from "@mui/lab/TabPanel";
import { MainLayout } from "../../layouts/mainLayout";
import styles from "./StorePage.module.css";
import { useState } from "react";
import {
  Box,
  Tab,
  Button,
  FormControl,
  Stack,
  TextField,
  DialogContent,
  Typography,
  DialogActions,
  IconButton,
} from "@mui/material";

import { Filebox } from "../../service/request";
import { DropzoneArea } from "material-ui-dropzone";
import { MaxFileSize, assertIsAxiosError } from "../../filebox";
import { DialogHeader, FileboxDialog } from "../../components/dialog";
import CopyAllIcon from "@mui/icons-material/CopyAll";
import Tooltip from "@mui/material/Tooltip";
import { Alerts } from "../../components/alerts";

interface StorePageProps {}

export const StorePage: React.FC<StorePageProps> = () => {
  const [value, setValue] = useState("1");
  const [title, setTitle] = useState("");
  const [text, setText] = useState("");
  const [file, setFile] = useState<File | undefined>(undefined);
  const [storeDay, setStoreDay] = useState(1);
  const [open, setOpen] = useState(false);

  const [clickTitle, setClickTitle] = useState("Click to copy");

  const handleClose = () => {
    setOpen(false);
    resetForm();
    window.location.reload();
  };
  const resetForm = () => {
    setFile(undefined);
    setTitle("");
    setText("");
    setStoreDay(1);
  };
  const handleChange = (_event: React.SyntheticEvent, newValue: string) => {
    setValue(newValue);
  };

  const [filecode, setFilecode] = useState("");
  const [inputErrMsg, setInputErrMsg] = useState("");
  const handleClick = () => {
    if (title === "") {
      setInputErrMsg("请输入标题");
      setTimeout(() => {
        setInputErrMsg("");
      }, 1000);
      return;
    }
    if (value === "1" && !file) {
      setInputErrMsg("请上传文件");
      setTimeout(() => {
        setInputErrMsg("");
      }, 1000);
      return;
    }
    if (value === "2" && text === "") {
      setInputErrMsg("请输入文本");
      setTimeout(() => {
        setInputErrMsg("");
      }, 1000);
      return;
    }
    const filebox: Filebox.FileboxData = {
      name: title,
      duration_day: storeDay,
      file_type: value === "1" ? 1 : 2,
      text: text,
      file: file,
    };

    const openDialog = (filecode: string) => {
      setOpen(true);
      setFilecode(filecode);
    };

    Filebox.addFilebox(filebox)
      .then((res) => {
        openDialog(res.code);
      })
      .catch((err) => {
        assertIsAxiosError(err);
        const status = err.response?.status || 200;
        if (status === 403) {
          const data = err.response?.data as any;
          setInputErrMsg(data.message);
          setTimeout(() => {
            setInputErrMsg("");
          }, 1000);
        }
      });
  };

  return (
    <MainLayout title="寄件">
      <div className={styles.container}>
        <FormControl>
          <div className={styles["input-madal"]}>
            <Stack>
              <TextField
                placeholder="请输入标题"
                sx={{
                  color: "f5f5f5",
                  background: "f5f5f5",
                  border: "1px solid #444",
                  input: {
                    color: "white",
                  },
                }}
                label="标题"
                variant="outlined"
                value={title}
                onChange={(e) => setTitle(e.target.value)}
                autoFocus
                inputProps={{ maxLength: 30 }}
              />
            </Stack>
            <TabContext value={value}>
              <Box
                sx={{
                  borderBottom: 1,
                  borderColor: "#444",
                  display: "flex",
                  justifyContent: "space-between",
                }}
              >
                <Box>
                  <TabList onChange={handleChange}>
                    <Tab
                      label={<div style={{ color: "white" }}>文 件</div>}
                      value="1"
                    />
                    <Tab
                      label={<div style={{ color: "white" }}>文 字</div>}
                      value="2"
                    />
                  </TabList>
                </Box>
                <Box
                  sx={{
                    display: "flex",
                    alignItems: "center",
                    marginRight: "30px",
                  }}
                >
                  存放时间:
                  <TextField
                    value={storeDay}
                    onChange={(e) => {
                      if (e.target.value === "") {
                        setStoreDay(1);
                      } else if (e.target.value) {
                        let value = parseInt(e.target.value);
                        if (value > 30) {
                          value = 30;
                        }
                        setStoreDay(value);
                      }
                    }}
                    size="small"
                    placeholder="单位/天"
                    sx={{
                      width: "50px",
                      marginRight: "2px",
                      input: {
                        color: "white",
                      },
                    }}
                    inputProps={{
                      inputMode: "numeric",
                      pattern: "[0-9]*",
                    }}
                  />
                  天
                </Box>
              </Box>
              <TabPanel value="1">
                <div className={styles.dropzone}>
                  <DropzoneArea
                    filesLimit={1}
                    maxFileSize={MaxFileSize}
                    onChange={(files) => {
                      const uploadFile = files[0];
                      setFile(uploadFile);
                    }}
                    onDelete={(_) => setFile(undefined)}
                  />
                </div>
                <Button
                  sx={{
                    marginTop: "20px",
                    background: "#888",
                    width: "200px",
                  }}
                  variant="contained"
                  onClick={() => {
                    handleClick();
                  }}
                >
                  寄 件
                </Button>
              </TabPanel>
              <TabPanel value="2">
                <TextField
                  id="standard-multiline-flexible"
                  multiline
                  minRows={20}
                  maxRows={20}
                  fullWidth
                  sx={{
                    background: "#f5f5f5",
                    borderRadius: "6px",
                  }}
                  variant="standard"
                  InputProps={{
                    disableUnderline: true,
                  }}
                  value={text}
                  onChange={(e) => setText(e.target.value)}
                  inputProps={{ maxLength: 2000 }}
                />
                <Button
                  sx={{ marginTop: "20px", background: "#888", width: "200px" }}
                  variant="contained"
                  onClick={() => {
                    handleClick();
                  }}
                >
                  寄 件
                </Button>
              </TabPanel>
            </TabContext>
          </div>
        </FormControl>

        <div className={styles.alert}>
          {inputErrMsg && (
            <Alerts title="Error" severity="error">
              {inputErrMsg}
            </Alerts>
          )}
        </div>
        <FileboxDialog sx={{ zIndex: 999 }} open={open}>
          <DialogHeader id="store-dialog" onClose={handleClose}>
            寄件成功
          </DialogHeader>
          <DialogContent sx={{ minWidth: "300px" }} dividers>
            <Typography gutterBottom>
              有效期为: <strong>{storeDay}</strong> 天, 取件码为:{" "}
              <strong>{filecode}</strong>
              <Tooltip title={clickTitle}>
                <IconButton
                  color="primary"
                  aria-label="copy"
                  component="label"
                  onClick={() => {
                    setClickTitle("Copied");
                    navigator.clipboard.writeText(filecode);
                  }}
                >
                  <CopyAllIcon />
                </IconButton>
              </Tooltip>
            </Typography>
          </DialogContent>
          <DialogActions>
            <Button autoFocus onClick={handleClose}>
              确 定
            </Button>
          </DialogActions>
        </FileboxDialog>
      </div>
    </MainLayout>
  );
};
