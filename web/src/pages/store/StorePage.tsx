import Box from "@mui/material/Box";
import Tab from "@mui/material/Tab";
import TabContext from "@mui/lab/TabContext";
import TabList from "@mui/lab/TabList";
import TabPanel from "@mui/lab/TabPanel";
import { MainLayout } from "../../layouts/mainLayout";
import styles from "./StorePage.module.css";
import { useState } from "react";
import { Button, TextField } from "@mui/material";

interface StorePageProps {}

export const StorePage: React.FC<StorePageProps> = () => {
  const [value, setValue] = useState("1");

  const handleChange = (event: React.SyntheticEvent, newValue: string) => {
    setValue(newValue);
  };

  return (
    <MainLayout title="寄件">
      <div className={styles.container}>
        <div className={styles["input-madal"]}>
          <TabContext value={value}>
            <Box sx={{ borderBottom: 1, borderColor: "#444" }}>
              <TabList
                onChange={handleChange}
                aria-label="lab API tabs example"
              >
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
                  background: "#888",
                  color: "white",
                }}
                variant="standard"
              />
              <Button
                sx={{ marginTop: "8px", background: "#888" }}
                variant="contained"
                // component="label"
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
