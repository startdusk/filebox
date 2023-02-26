import Box from "@mui/material/Box";
import Tab from "@mui/material/Tab";
import TabContext from "@mui/lab/TabContext";
import TabList from "@mui/lab/TabList";
import TabPanel from "@mui/lab/TabPanel";
import { MainLayout } from "../../layouts/mainLayout";
import styles from "./StorePage.module.css";
import { useState } from "react";
import { Button, FormControl, Stack, TextField } from "@mui/material";

import { Filebox } from "../../service/request";
import { DropzoneArea } from "material-ui-dropzone";
import { MaxFileSize } from "../../filebox";

interface StorePageProps {}

export const StorePage: React.FC<StorePageProps> = () => {
  const [value, setValue] = useState("1");
  const [title, setTitle] = useState("");
  const [text, setText] = useState("");
  const [file, setFile] = useState<File | undefined>(undefined);
  const [storeDay, setStoreDay] = useState(1);

  const handleChange = (_event: React.SyntheticEvent, newValue: string) => {
    setValue(newValue);
  };

  const handleClick = () => {
    const filebox: Filebox.FileboxData = {
      name: title,
      duration_day: storeDay,
      file_type: value === "1" ? 1 : 2,
      text: text,
      file: file,
    };

    Filebox.addFilebox(filebox).then((res) => {
      console.log(res);
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
                        setStoreDay(0);
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
                  sx={{ marginTop: "8px", background: "#888", width: "200px" }}
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
                />
                <Button
                  sx={{ marginTop: "8px", background: "#888", width: "200px" }}
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
      </div>
    </MainLayout>
  );
};
