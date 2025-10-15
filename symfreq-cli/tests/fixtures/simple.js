import fs from "fs";
import { join } from "path";

const APP_NAME = "DummyApp";
let counter = 0;

function add(a, b) {
    return a + b;
}

const multiply = (a, b) => a * b;

async function fetchData(url) {
    try {
        const res = await fetch(url);
        return await res.json();
    } catch (err) {
        console.error("Fetch failed:", err);
    }
}

class DummyClass {
    constructor(name) {
        this.name = name;
    }

    greet() {
        return `Hello, ${this.name}!`;
    }
}

export function featureToggle(flag) {
    return flag ? "Feature Enabled" : "Feature Disabled";
}

export default class Feature {
    constructor() {
        this.enabled = true;
    }

    toggle() {
        this.enabled = !this.enabled;
        return this.enabled;
    }
}

console.log(APP_NAME, "started");
console.log("Sum test:", add(2, 3));
console.log("Multiply test:", multiply(4, 5));
console.log(new DummyClass("Tester").greet());
