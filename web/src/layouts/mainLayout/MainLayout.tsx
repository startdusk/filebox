import React from "react";

import { Header } from "../../components/header/Header";

interface MainLayoutProps {
  title: string;
  children?: React.ReactNode;
}

export const MainLayout: React.FC<MainLayoutProps> = ({ children, title }) => {
  return (
    <>
      <Header title={title} />
      <div>{children}</div>
    </>
  );
};
