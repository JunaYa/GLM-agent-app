import { Route } from "react-router-dom";
import { Routes } from "react-router-dom";
import "./App.css";
import Home from "./pages/home";
import RootLayout from "./layout";
import Task from "./pages/task";
import Index from "./pages/index";

export default function App() {
  return (
    <Routes>
      <Route path="/" element={<RootLayout />}>
        <Route index element={<Index />} />
        <Route path="index" element={<Index />} />
        <Route path="home" element={<Home />} />
        <Route path="task" element={<Task />} />

        {/* Using path="*"" means "match anything", so this route
              acts like a catch-all for URLs that we don't have explicit
              routes for. */}
        {/* <Route path="*" element={<NoMatch />} /> */}
      </Route>
    </Routes>
  );
}

