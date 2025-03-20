import "./App.css";
import { getCurrentWindow } from "@tauri-apps/api/window";

function App() {
  const appWindow = getCurrentWindow();


  function handleMouseDown(e: React.MouseEvent<HTMLHeadingElement>) {
    if (e.buttons === 1) {
      // Primary (left) button
      e.detail === 2
        ? appWindow.toggleMaximize() // Maximize on double click
        : appWindow.startDragging(); // Else start dragging
    }
  }

  return (
    <main className="container" onMouseDown={handleMouseDown}>
    </main>
  );
}

export default App;
