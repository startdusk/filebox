import Box from "@mui/material/Box";
import Tab from "@mui/material/Tab";
import TabContext from "@mui/lab/TabContext";
import TabList from "@mui/lab/TabList";
import TabPanel from "@mui/lab/TabPanel";
import { MainLayout } from "../../layouts/mainLayout";
import styles from "./StorePage.module.css";
import { useState } from "react";
import { InputLabel, MenuItem, Select } from "@mui/material";

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
                <Tab label="Item One" value="1" />
                <Tab label="Item Two" value="2" />
                <Tab label="Item Three" value="3" />
                <InputLabel id="label">Age</InputLabel>
                <Select labelId="label" id="select" value="20">
                  <MenuItem value="10">Ten</MenuItem>
                  <MenuItem value="20">Twenty</MenuItem>
                </Select>
              </TabList>
            </Box>
            <TabPanel value="1">Item One</TabPanel>
            <TabPanel value="2">Item Two</TabPanel>
            <TabPanel value="3">Item Three</TabPanel>
          </TabContext>
        </div>
      </div>
    </MainLayout>
  );
};
