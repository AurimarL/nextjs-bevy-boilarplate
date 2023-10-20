"use client";

import { useEffect, useState } from "react";
import init, { run_agame_app } from "../../pkg";

export default function Home() {
  const runGame = async () => {
    await init();
    run_agame_app();
  };
  const [active, setActive] = useState(false);

  useEffect(() => {
    setActive(true);
    if (active) {
      runGame();
    }
  }, [active]);

  return <main></main>;
}
