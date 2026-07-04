import React from "react";

interface TerminalWindowProps {
  children: React.ReactNode;
  title: string;
}

const TerminalWindow: React.FC<TerminalWindowProps> = ({ children, title }) => {
  return (
    <div
      className="terminal-window"
      style={{
        position: "relative",
        // ★ トリッキーな子要素（-z-10）を使わず、親自身に直接背景とボカシを適用
        backgroundColor: "rgba(0, 0, 0, 0.3)",
        backdropFilter: "none",
        WebkitBackdropFilter: "none",

        borderRadius: "16px",
        overflow: "hidden", // ヘッダーの角丸はみ出しをここで一括カット

        // GPUレンダリングの強制
        transform: "translateZ(0)",
        WebkitTransform: "translateZ(0)",
        willChange: "backdrop-filter",
      }}
    >
      {/* ヘッダー部分 */}
      <div
        className="terminal-header"
        style={{
          // ヘッダーも少し透かす
          backgroundColor: "rgba(20, 25, 45, 0.4)",
          borderBottom: "1px solid rgba(255, 255, 255, 0.1)",
        }}
      >
        <div className="dot red"></div>
        <div className="dot yellow"></div>
        <div className="dot green"></div>
        <div className="terminal-title">{title}</div>
      </div>

      <div className="terminal-content">{children}</div>
    </div>
  );
};

export default TerminalWindow;
