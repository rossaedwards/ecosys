import { useState } from 'react';
import FirstBoot from './boot/FirstBoot';
import TuiCockpit from './cockpit/TuiCockpit';

export default function App() {
  const [sealed, setSealed] = useState(() => localStorage.getItem('g0dm0d3-ktrl.sealed') === '1');

  if (!sealed) {
    return <FirstBoot onSealed={() => setSealed(true)} />;
  }
  return <TuiCockpit />;
}
