import { getCurrentWindow } from "@tauri-apps/api/window";

function Home() {
  const appWindow = getCurrentWindow();


  function handleMouseDown() {
    appWindow.startDragging()
  }

  return (
    <main className="container" onMouseDown={handleMouseDown}>
    </main>
  );
}

export default Home;
