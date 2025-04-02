import { getCurrentWindow } from "@tauri-apps/api/window";
import { invoke } from '@tauri-apps/api/core'

const Task = () => {
  const appWindow = getCurrentWindow();


  function handleMouseDown() {
    appWindow.startDragging()
  }

  function handleClose() {
    invoke('hide_task_panel')
  }
  
  return <div className="h-full w-full" onMouseDown={handleMouseDown}>
    <div className="absolute top-0 left-0 right-0 bottom-0 rounded-lg animate-glow"></div>
    <div className="absolute top-4 left-1/2 translate-x-[-50%] bg-white rounded-full px-8 py-2 cursor-pointer text-sm" onClick={handleClose}>
      关闭
    </div>
  </div>;
};

export default Task;
