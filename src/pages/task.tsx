import { getCurrentWindow } from "@tauri-apps/api/window";

const Task = () => {
  const appWindow = getCurrentWindow();


  function handleMouseDown() {
    appWindow.startDragging()
  }
  
  return <div className="h-full w-full rounded-lg animate-glow" onMouseDown={handleMouseDown}>
  </div>;
};

export default Task;
