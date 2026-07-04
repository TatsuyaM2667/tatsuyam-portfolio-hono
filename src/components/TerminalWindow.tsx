import React from 'react';

interface TerminalWindowProps {
  children: React.ReactNode;
  title: string;
}

const TerminalWindow: React.FC<TerminalWindowProps> = ({ children, title }) => {
  return (
    <div className="terminal-window">
      <div className="terminal-header">
        <div className="dot red"></div>
        <div className="dot yellow"></div>
        <div className="dot green"></div>
        <div className="terminal-title">{title}</div>
      </div>
      <div className="terminal-content">
        {children}
      </div>
    </div>
  );
};

export default TerminalWindow;
