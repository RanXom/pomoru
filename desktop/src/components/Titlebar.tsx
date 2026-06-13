import { getCurrentWindow } from "@tauri-apps/api/window";
import logoUrl from "../assets/logo.svg";

interface TitlebarProps {
  onToggleSidebar: () => void;
  isSidebarOpen: boolean;
}

export default function Titlebar({ onToggleSidebar, isSidebarOpen }: TitlebarProps) {
  const appWindow = getCurrentWindow();

  return (
    <div data-tauri-drag-region className="titlebar" id="titlebar">
      <div className="titlebar-left">
        <button
          className={`titlebar-btn toggle-sidebar ${isSidebarOpen ? "open" : ""}`}
          onClick={onToggleSidebar}
          aria-label={isSidebarOpen ? "Close sidebar" : "Open sidebar"}
          title="Toggle Tasks"
        >
          <svg className="menu-icon-svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round">
            <line className="line-top" x1="3" y1="6" x2="21" y2="6" />
            <line className="line-middle" x1="3" y1="12" x2="21" y2="12" />
            <line className="line-bottom" x1="3" y1="18" x2="21" y2="18" />
          </svg>
        </button>
      </div>
      <div data-tauri-drag-region className="titlebar-center">
        <img src={logoUrl} alt="Pomoru" className="titlebar-logo" />
      </div>
      <div className="titlebar-right">
        <button
          className="titlebar-btn minimize"
          onClick={() => appWindow.minimize()}
          aria-label="Minimize"
          title="Minimize"
        >
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round">
            <line x1="4" y1="12" x2="20" y2="12" />
          </svg>
        </button>
        <button
          className="titlebar-btn close"
          onClick={() => appWindow.close()}
          aria-label="Close"
          title="Close"
        >
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round">
            <line x1="18" y1="6" x2="6" y2="18" />
            <line x1="6" y1="6" x2="18" y2="18" />
          </svg>
        </button>
      </div>
    </div>
  );
}
