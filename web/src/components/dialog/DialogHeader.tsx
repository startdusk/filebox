import * as React from "react";
import IconButton from "@mui/material/IconButton";
import CloseIcon from "@mui/icons-material/Close";
import DialogTitle from "@mui/material/DialogTitle";

export interface PropsType {
  id: string;
  children?: React.ReactNode;
  onClose: () => void;
}

export const DialogHeader: React.FC<PropsType> = ({
  children,
  onClose,
  ...other
}) => {
  return (
    <DialogTitle sx={{ m: 0, p: 2, zIndex: 999 }} {...other}>
      {children}
      {onClose ? (
        <IconButton
          aria-label="close"
          onClick={onClose}
          sx={{
            position: "absolute",
            right: 8,
            top: 8,
            color: (theme) => theme.palette.grey[500],
          }}
        >
          <CloseIcon />
        </IconButton>
      ) : null}
    </DialogTitle>
  );
};
