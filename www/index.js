import * as sim from "lib-simulation-wasm";
import { Viewport } from "./app/viewport";

const viewport = new Viewport(
    document.getElementById("viewport"),
);

let simulation = new sim.Simulation(sim.Simulation.default_config());

function redraw() {

    simulation.step(0.01);

    const world = simulation.world();
    viewport.clear();
    for (const point of world.circles) {
        viewport.drawCircle(point.x, point.y, 0.01, "#ffffff");
    }

    requestAnimationFrame(redraw);
}

redraw();