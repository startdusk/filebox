import Box from "@mui/material/Box";
import Tab from "@mui/material/Tab";
import TabContext from "@mui/lab/TabContext";
import TabList from "@mui/lab/TabList";
import TabPanel from "@mui/lab/TabPanel";
import { MainLayout } from "../../layouts/mainLayout";
import styles from "./StorePage.module.css";
import { useState } from "react";
import { Button, Stack, TextField } from "@mui/material";

import { Filebox } from "../../service/request";

interface StorePageProps {}

export const StorePage: React.FC<StorePageProps> = () => {
  const [value, setValue] = useState("1");
  const [title, setTitle] = useState("");
  const [text, setText] = useState("");

  const handleChange = (event: React.SyntheticEvent, newValue: string) => {
    setValue(newValue);
  };

  const handleClick = () => {
    const filebox: Filebox.FileboxData = {
      name: title,
      duration_day: 7,
      text: text,
    };
    console.log(filebox);
    Filebox.addFilebox(filebox).then((res) => {
      console.log(res);
    });
  };

  return (
    <MainLayout title="寄件">
      <div className={styles.container}>
        <div className={styles["input-madal"]}>
          <Stack>
            <TextField
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
            />
          </Stack>
          <TabContext value={value}>
            <Box sx={{ borderBottom: 1, borderColor: "#444" }}>
              <TabList onChange={handleChange}>
                <Tab
                  label={<div style={{ color: "white" }}>文件</div>}
                  value="1"
                />
                <Tab
                  label={<div style={{ color: "white" }}>文字</div>}
                  value="2"
                />
              </TabList>
            </Box>
            <TabPanel value="1">文件</TabPanel>
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
      </div>
    </MainLayout>
  );
};
