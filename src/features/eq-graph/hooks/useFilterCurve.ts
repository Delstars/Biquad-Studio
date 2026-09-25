import { useMemo } from 'react';
import { ParametricFilter } from '../../../app/store/eqStore';

// Calculate RBJ biquad coefficients (normalized)
function getCoeffs(filter: ParametricFilter, fs: number = 48000) {
  const { type, freq, gain, q } = filter;
  const w0 = 2 * Math.PI * (freq / fs);
  const alpha = Math.sin(w0) / (2 * q);
  const A = Math.pow(10, gain / 40);
  const cosW0 = Math.cos(w0);

  let b0=0, b1=0, b2=0, a0=1, a1=0, a2=0;

  switch (type) {
    case 'Peaking':
      b0 = 1 + alpha * A;
      b1 = -2 * cosW0;
      b2 = 1 - alpha * A;
      a0 = 1 + alpha / A;
      a1 = -2 * cosW0;
      a2 = 1 - alpha / A;
      break;
    case 'LowShelf': {
      const sq = 2 * Math.sqrt(A) * alpha;
      b0 = A * ((A + 1) - (A - 1) * cosW0 + sq);
      b1 = 2 * A * ((A - 1) - (A + 1) * cosW0);
      b2 = A * ((A + 1) - (A - 1) * cosW0 - sq);
      a0 = (A + 1) + (A - 1) * cosW0 + sq;
      a1 = -2 * ((A - 1) + (A + 1) * cosW0);
      a2 = (A + 1) + (A - 1) * cosW0 - sq;
      break;
    }
    case 'HighShelf': {
      const sq = 2 * Math.sqrt(A) * alpha;
      b0 = A * ((A + 1) + (A - 1) * cosW0 + sq);
      b1 = -2 * A * ((A - 1) + (A + 1) * cosW0);
      b2 = A * ((A + 1) + (A - 1) * cosW0 - sq);
      a0 = (A + 1) - (A - 1) * cosW0 + sq;
      a1 = 2 * ((A - 1) - (A + 1) * cosW0);
      a2 = (A + 1) - (A - 1) * cosW0 - sq;
      break;
    }
    case 'LowPass':
      b0 = (1 - cosW0) / 2;
      b1 = 1 - cosW0;
      b2 = (1 - cosW0) / 2;
      a0 = 1 + alpha;
      a1 = -2 * cosW0;
      a2 = 1 - alpha;
      break;
    case 'HighPass':
      b0 = (1 + cosW0) / 2;
      b1 = -(1 + cosW0);
      b2 = (1 + cosW0) / 2;
      a0 = 1 + alpha;
      a1 = -2 * cosW0;
      a2 = 1 - alpha;
      break;
  }

  return { b0: b0/a0, b1: b1/a0, b2: b2/a0, a1: a1/a0, a2: a2/a0 };
}

// Evaluate magnitude (in linear gain) of a biquad at angular frequency w
function evaluateMagnitude(c: ReturnType<typeof getCoeffs>, w: number) {
  const phi = Math.pow(Math.sin(w / 2), 2);
  const b0 = c.b0, b1 = c.b1, b2 = c.b2;
  const a1 = c.a1, a2 = c.a2;

  // Faster magnitude squared calculation for biquads
  const num = Math.pow(b0 + b1 + b2, 2) - 4 * (b0 * b1 + 4 * b0 * b2 + b1 * b2) * phi + 16 * b0 * b2 * phi * phi;
  const den = Math.pow(1 + a1 + a2, 2) - 4 * (a1 + 4 * a2 + a1 * a2) * phi + 16 * a2 * phi * phi;
  
  return Math.sqrt(num / den);
}

export function useFilterCurve(filters: ParametricFilter[], width: number, minFreq: number = 20, maxFreq: number = 20000, fs: number = 48000) {
  return useMemo(() => {
    if (width <= 0) return [];

    const activeFilters = filters.filter((f) => f.enabled);
    const coeffs = activeFilters.map(f => getCoeffs(f, fs));
    const points: { x: number, yDb: number }[] = [];

    const logMin = Math.log10(minFreq);
    const logMax = Math.log10(maxFreq);
    
    // Evaluate across pixels
    for (let x = 0; x <= width; x++) {
      // Map x to frequency
      const freq = Math.pow(10, logMin + (x / width) * (logMax - logMin));
      const w = 2 * Math.PI * (freq / fs);
      
      let totalGainLinear = 1.0;
      for (const c of coeffs) {
        totalGainLinear *= evaluateMagnitude(c, w);
      }
      
      const yDb = 20 * Math.log10(totalGainLinear || 1e-10);
      points.push({ x, yDb });
    }

    return points;
  }, [filters, width, minFreq, maxFreq, fs]);
}
