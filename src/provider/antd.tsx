import { ConfigProvider } from "antd";

export const AntdProvider = ({ children }: { children: React.ReactNode }) => {
  return (
    <ConfigProvider
      theme={{
        token: {
          // Seed Token，影响范围大
          colorPrimary: "#00b96b",
          borderRadius: 2,
          // 派生变量，影响范围小
          colorBgContainer: "#f6ffed",
        },
      }}
    >
      {children}
    </ConfigProvider>
  );
};
