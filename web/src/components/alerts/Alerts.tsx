import Alert from "@mui/material/Alert";
import AlertTitle from "@mui/material/AlertTitle";
import { AlertColor } from "@mui/material";

interface PropsType {
  title: string;
  severity: AlertColor;
  variant?: "standard" | "filled" | "outlined";
  children?: React.ReactNode;
}

export const Alerts: React.FC<PropsType> = ({
  title,
  severity,
  variant = "filled",
  children,
}) => {
  return (
    <Alert
      icon={false}
      variant={variant}
      severity={severity}
      style={{
        width: 300,
        display: "flex",
        justifyContent: "center",
      }}
    >
      <AlertTitle>
        <h3>{title}</h3>
      </AlertTitle>
      <h3>{children}</h3>
    </Alert>
  );
};
