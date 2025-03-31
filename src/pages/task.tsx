import { getCurrentWindow } from "@tauri-apps/api/window";

const Task = () => {
  const appWindow = getCurrentWindow();


  function handleMouseDown() {
    appWindow.startDragging()
  }
  
  return <div className="h-full w-full border-2 border-red-500" >
    <div className="w-[100px] h-[100px] bg-blue-500 cursor-grab" onMouseDown={handleMouseDown}></div>
  </div>;
};

export default Task;
