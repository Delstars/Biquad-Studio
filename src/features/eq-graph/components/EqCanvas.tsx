import React, { useRef, useEffect, useState } from 'react';
import { useEqStore } from '../../../app/store/eqStore';
import { useFilterCurve } from '../hooks/useFilterCurve';

const MIN_FREQ = 20;
const MAX_FREQ = 20000;
const MIN_DB = -24;
const MAX_DB = 24;

export const EqCanvas: React.FC = () => {
  const containerRef = useRef<HTMLDivElement>(null);
  const [dimensions, setDimensions] = useState({ width: 0, height: 0 });
  const filters = useEqStore(state => state.filters);
  const updateFilter = useEqStore(state => state.updateFilter);

  // Measure container
  useEffect(() => {
    const observer = new ResizeObserver((entries) => {
      if (entries[0]) {
        const { width, height } = entries[0].contentRect;
        setDimensions({ width, height });
      }
    });
    if (containerRef.current) observer.observe(containerRef.current);
    return () => observer.disconnect();
  }, []);

  const { width, height } = dimensions;
  const curvePoints = useFilterCurve(filters, width, MIN_FREQ, MAX_FREQ);

  // Map frequency to X coordinate
  const freqToX = (freq: number) => {
    const logMin = Math.log10(MIN_FREQ);
    const logMax = Math.log10(MAX_FREQ);
    return ((Math.log10(freq) - logMin) / (logMax - logMin)) * width;
  };

  // Map X coordinate to frequency
  const xToFreq = (x: number) => {
    const logMin = Math.log10(MIN_FREQ);
    const logMax = Math.log10(MAX_FREQ);
    return Math.pow(10, logMin + (x / width) * (logMax - logMin));
  };

  // Map dB to Y coordinate
  const dbToY = (db: number) => {
    return height - ((db - MIN_DB) / (MAX_DB - MIN_DB)) * height;
  };

  // Map Y coordinate to dB
  const yToDb = (y: number) => {
    return MAX_DB - (y / height) * (MAX_DB - MIN_DB);
  };

  // SVG Path generation
  const pathData = curvePoints.length > 0 
    ? `M 0 ${dbToY(curvePoints[0].yDb)} ` + 
      curvePoints.slice(1).map(p => `L ${p.x} ${dbToY(p.yDb)}`).join(' ')
    : '';

  // Drag interaction state
  const [draggingId, setDraggingId] = useState<string | null>(null);

  const handlePointerDown = (id: string) => (e: React.PointerEvent) => {
    (e.target as Element).setPointerCapture(e.pointerId);
    setDraggingId(id);
    e.stopPropagation();
  };

  const handlePointerMove = (e: React.PointerEvent) => {
    if (!draggingId || !containerRef.current) return;
    
    const rect = containerRef.current.getBoundingClientRect();
    const x = Math.max(0, Math.min(e.clientX - rect.left, width));
    const y = Math.max(0, Math.min(e.clientY - rect.top, height));

    const newFreq = Math.round(xToFreq(x));
    const newGain = Math.round(yToDb(y) * 10) / 10; // 1 decimal place

    updateFilter(draggingId, { freq: newFreq, gain: newGain });
  };

  const handlePointerUp = (e: React.PointerEvent) => {
    if (draggingId) {
      (e.target as Element).releasePointerCapture(e.pointerId);
      setDraggingId(null);
    }
  };

  // Scroll to adjust Q factor
  const handleWheel = (id: string) => (e: React.WheelEvent) => {
    e.preventDefault();
    const filter = filters.find(f => f.id === id);
    if (!filter) return;

    // Scroll up = narrower Q, scroll down = wider Q
    const delta = e.deltaY > 0 ? 0.9 : 1.1;
    const newQ = Math.max(0.1, Math.min(filter.q * delta, 20.0));
    updateFilter(id, { q: newQ });
  };

  return (
    <div className="relative w-full h-64 bg-slate-900 rounded-lg overflow-hidden border border-slate-800" ref={containerRef}>
      <svg className="absolute inset-0 w-full h-full pointer-events-none">
        {/* Zero line */}
        <line x1="0" y1={dbToY(0)} x2={width} y2={dbToY(0)} stroke="#334155" strokeWidth="1" strokeDasharray="4 4" />
        
        {/* The frequency response curve */}
        <path d={pathData} fill="none" stroke="#3b82f6" strokeWidth="2" className="drop-shadow-lg" />
      </svg>

      {/* Interactive filter nodes */}
      {filters.map((f, i) => {
        if (!f.enabled) return null;
        const cx = freqToX(f.freq);
        const cy = dbToY(f.gain);

        return (
          <div
            key={f.id}
            className={`absolute w-4 h-4 -ml-2 -mt-2 rounded-full border-2 bg-slate-800 cursor-move transition-transform ${draggingId === f.id ? 'scale-125 border-white' : 'border-blue-500 hover:border-blue-300'}`}
            style={{ left: cx, top: cy, touchAction: 'none' }}
            onPointerDown={handlePointerDown(f.id)}
            onPointerMove={handlePointerMove}
            onPointerUp={handlePointerUp}
            onWheel={handleWheel(f.id)}
            title={`Band ${i+1}: ${f.freq} Hz, ${f.gain} dB, Q: ${f.q.toFixed(2)}`}
          />
        );
      })}
      
      {/* Readouts */}
      <div className="absolute bottom-2 right-2 text-xs text-slate-500 pointer-events-none">
        Scroll over nodes to adjust Q
      </div>
    </div>
  );
};
