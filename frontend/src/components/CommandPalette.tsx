import React, { useState } from 'react';
import { Search, Shield, Zap, Cpu, Code, HelpCircle } from 'lucide-react';

interface Props {
  isOpen: boolean;
  onClose: () => void;
  onSelectAction: (action: string) => void;
}

export const CommandPalette: React.FC<Props> = ({ isOpen, onClose, onSelectAction }) => {
  const [query, setQuery] = useState('');

  if (!isOpen) return null;

  const actions = [
    { id: 'overview', title: 'Go to Overview Dashboard', icon: Cpu },
    { id: 'graph', title: 'Open Knowledge Graph Explorer', icon: Code },
    { id: 'hotspots', title: 'Inspect Engineering Hotspots & Risks', icon: Zap },
    { id: 'security', title: 'Audit Security Surface & Data Flows', icon: Shield },
    { id: 'architecture', title: 'Explore Discovered Architecture', icon: Cpu },
    { id: 'impact', title: 'Run Change Impact Analysis', icon: Zap },
    { id: 'ask', title: 'Ask SENTRIX AI Assistant', icon: HelpCircle },
  ];

  const filtered = actions.filter((a) => a.title.toLowerCase().includes(query.toLowerCase()));

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div className="command-modal" onClick={(e) => e.stopPropagation()}>
        <input
          type="text"
          className="command-input"
          placeholder="Type a command or search symbol (e.g. auth, impact, hotspots)..."
          autoFocus
          value={query}
          onChange={(e) => setQuery(e.target.value)}
        />
        <div className="command-list">
          {filtered.map((item) => {
            const Icon = item.icon;
            return (
              <div
                key={item.id}
                className="command-item"
                onClick={() => {
                  onSelectAction(item.id);
                  onClose();
                }}
              >
                <Icon size={18} />
                <span>{item.title}</span>
              </div>
            );
          })}
        </div>
      </div>
    </div>
  );
};
